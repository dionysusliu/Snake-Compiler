use crate::asm::instrs_to_string;
use crate::asm::{Arg32, Arg64, BinArgs, Instr, MemRef, MovArgs, Reg, Reg32};
use crate::syntax::{Exp, FunDecl, ImmExp, Prim, SeqExp, SeqProg, SurfProg};

use std::collections::{HashMap, HashSet};
// use std::fmt::format;



type ErrorCode = u64;
static ARITH_ERROR: ErrorCode = 0;
static CMP_ERROR:   ErrorCode = 1;
static IF_ERROR:    ErrorCode = 2;
static LOGIC_ERROR: ErrorCode = 3;
static OVFL_ERROR:  ErrorCode = 4;

struct SnakeVal(u64);
static TAG_MASK: u64 = 0x00_00_00_00_00_00_00_01;
static NOT_MUSK: u64 = 0x80_00_00_00_00_00_00_00;
static SNAKE_TRU: SnakeVal = SnakeVal(0xFF_FF_FF_FF_FF_FF_FF_FF);
static SNAKE_FLS: SnakeVal = SnakeVal(0x7F_FF_FF_FF_FF_FF_FF_FF);

static MAX_SNAKE_INT: i64 = i64::MAX >> 1;
static MIN_SNAKE_INT: i64 = i64::MIN >> 1;


#[derive(Debug, PartialEq, Eq)]
pub enum CompileErr<Span> {
    UnboundVariable {
        unbound: String,
        location: Span,
    },
    UndefinedFunction {
        undefined: String,
        location: Span,
    },
    // The Span here is the Span of the let-expression that has the two duplicated bindings
    DuplicateBinding {
        duplicated_name: String,
        location: Span,
    },

    Overflow {
        num: i64,
        location: Span,
    },

    DuplicateFunName {
        duplicated_name: String,
        location: Span, // the location of the 2nd function
    },

    DuplicateArgName {
        duplicated_name: String,
        location: Span,
    },

    FunctionUsedAsValue {
        function_name: String,
        location: Span,
    },

    ValueUsedAsFunction {
        variable_name: String,
        location: Span,
    },

    FunctionCalledWrongArity {
        function_name: String,
        correct_arity: usize,
        arity_used: usize,
        location: Span, // location of the function *call*
    },
}

// ********************************************************************
//
//                         check prog & tag
//
// ********************************************************************

pub fn check_prog<Span>(p: &SurfProg<Span>) -> Result<(), CompileErr<Span>>
where
    Span: Clone,
{
    fn check_prog_helper<Span>(e: &SurfProg<Span>, mut var_env: HashSet<String>, mut fun_env: HashMap<String, Vec<String>>) -> Result<(), CompileErr<Span>> 
    where Span: Clone{
        match e {
            Exp::Num(n, span) => {
                if *n > MAX_SNAKE_INT || *n < MIN_SNAKE_INT {
                    return Err(CompileErr::Overflow { num: *n, location: span.clone()}) 
                } else {
                    return Ok(())
                }
            },

            Exp::Bool(_, _) => return Ok(()),

            Exp::Var(var_name, span) => {
                if var_env.contains(var_name) {
                    return Ok(())
                } else {
                    if fun_env.contains_key(var_name){
                        return Err(CompileErr::FunctionUsedAsValue { function_name: var_name.clone(), location: span.clone() });
                    } else {
                        return Err(CompileErr::UnboundVariable{unbound: var_name.clone(), location: span.clone()});
                    }
                }
            },

            Exp::Prim(_, exp_vec, _) => {
                for exp in exp_vec.iter() {
                    check_prog_helper(exp, var_env.clone(), fun_env.clone())?;
                }
                return Ok(())
            },

            Exp::Let{bindings, body, ann} => {
                let mut this_env = HashSet::new();
                for (var_name, exp) in bindings.iter() {
                    let insert_flag: bool = this_env.insert(var_name.clone());
                    if insert_flag == false {
                        return Err(CompileErr::DuplicateBinding{duplicated_name: var_name.clone(), location: ann.clone()});
                    }
                    check_prog_helper(exp, var_env.clone(), fun_env.clone())?;
                    var_env.insert(var_name.clone());
                }
                check_prog_helper(body, var_env.clone(), fun_env.clone())
            },

            Exp::If{cond, thn, els, ann: _} => {
                check_prog_helper(cond, var_env.clone(), fun_env.clone())?;
                check_prog_helper(thn, var_env.clone(), fun_env.clone())?;
                check_prog_helper(els, var_env.clone(), fun_env.clone())?;
                return Ok(())
            },

            Exp::FunDefs { decls, body, ann: _ } => {
                let mut seen_fun_names: HashSet<String> = HashSet::new();
                for fun_decl in decls.iter() {
                    // 1. check DuplicateFunName
                    let funname_insert_flag = seen_fun_names.insert(fun_decl.name.clone());
                    if funname_insert_flag == false {
                        return Err(CompileErr::DuplicateFunName { duplicated_name: fun_decl.name.clone(), location: fun_decl.ann.clone() })
                    }
                    // 2. check dupicate arg names 
                    let mut seen_arg_names: HashSet<String> = HashSet::new();
                    for arg_name in fun_decl.parameters.iter() {
                        let argname_insert_flag = seen_arg_names.insert(arg_name.clone());
                        if argname_insert_flag == false {
                            return Err(CompileErr::DuplicateArgName { duplicated_name: arg_name.clone(), location: fun_decl.ann.clone() })
                        }
                    }
                    // declaration is well-formed, add to global env
                    fun_env.insert(fun_decl.name.clone(), fun_decl.parameters.clone());
                }

                // the function bodies and body_expr should be checked AFTER all fun decl is registered in global fun_env
                for fun_decl in decls {
                    let mut env_for_this_body = var_env.clone();
                    for arg_name in fun_decl.parameters.iter() {
                        env_for_this_body.insert(arg_name.clone());
                    }
                    check_prog_helper(&fun_decl.body, env_for_this_body.clone(), fun_env.clone())?; // check function body
                }
                check_prog_helper(body, var_env.clone(), fun_env.clone())?;
                Ok({})
            },

            Exp::Call(func_name, args, ann) => {
                // check whether there is a local variable with same name
                if var_env.contains(func_name) == true {
                    return Err(CompileErr::ValueUsedAsFunction { variable_name: func_name.clone(), location: ann.clone() });
                }
                // check whether the name is defined in fun_env
                // if defined, check the args arity
                match fun_env.get(func_name) {
                    None => {
                        return Err(CompileErr::UndefinedFunction { undefined: func_name.clone(), location: ann.clone() });
                    },
                    Some(defined_args) => {
                        if args.len() != defined_args.len() {
                            return Err(CompileErr::FunctionCalledWrongArity { function_name: func_name.clone(), correct_arity: defined_args.len(), arity_used: args.len(), location: ann.clone() });
                        }
                    },
                }
                Ok({})
            },

            Exp::InternalTailCall(_func_name, _args, _ann) => {
                panic!("InternalTailCall shouldn't occur at check_prog stage")
            },

            Exp::ExternalCall { fun_name: _, args: _, is_tail: _, ann: _ } => {
                panic!("ExternalCall shouldn't occur at check_prog stage")
            }
        }
    }

    let var_env: HashSet<String> = HashSet::new();
    let fun_env: HashMap<String, Vec<String>> = HashMap::new();
    check_prog_helper(p, var_env, fun_env)
}


// ********************************************************************
//
//                              uniquify
//
// ********************************************************************

fn uniquify(e: &Exp<u32>) -> Exp<()> {
    fn uniquify_helper(e: &Exp<u32>, mut var_name_mapping: HashMap<String, String>, mut fun_name_mapping: HashMap<String, String>) -> Exp<()> {
        match e {
            Exp::Num(val, _ann) => Exp::Num(val.clone(), ()),

            Exp::Bool(bool, _ann) => Exp::Bool(bool.clone(), ()),

            Exp::Var(var_name, _ann) => Exp::Var(var_name_mapping.get(var_name).unwrap().clone(), ()),

            Exp::Prim(op, exprs, _ann) => {
                let mut uniquified_exprs: Vec<Box<Exp<()>>> = vec![];
                for expr in exprs.iter() {
                    uniquified_exprs.push(Box::new(uniquify_helper(expr, var_name_mapping.clone(), fun_name_mapping.clone())));
                }
                Exp::Prim(op.clone(), uniquified_exprs, ())
            },

            Exp::Let { bindings, body, ann } => {
                // process the bindings
                let mut new_bindings: Vec<(String, Exp<()>)> = vec![];
                for (var_name, expr) in bindings.iter() {
                    let new_expr = uniquify_helper(expr, var_name_mapping.clone(), fun_name_mapping.clone()); //// before renaming, process its expr
                    let new_var_name = format!("{}#{}", var_name.clone(), ann.clone()); //// for each name, rename it with tag
                    var_name_mapping.insert(var_name.clone(), new_var_name.clone()); //// update name mapping
                    new_bindings.push((new_var_name, new_expr));
                }
                // process the body
                let new_body = uniquify_helper(body, var_name_mapping.clone(), fun_name_mapping.clone());

                Exp::Let{
                    bindings: new_bindings,
                    body: Box::new(new_body),
                    ann: ()
                }
            },

            Exp::If { cond, thn, els, ann: _ } => {
                let new_cond = uniquify_helper(cond, var_name_mapping.clone(), fun_name_mapping.clone());
                let new_thn  = uniquify_helper(thn, var_name_mapping.clone(), fun_name_mapping.clone());
                let new_els  = uniquify_helper(els, var_name_mapping.clone(), fun_name_mapping.clone());

                Exp::If { cond: Box::new(new_cond), thn: Box::new(new_thn), els: Box::new(new_els), ann: () }
            },

            Exp::FunDefs { decls, body, ann } => {
                // process the decls
                let mut new_decls: Vec<FunDecl<Exp<()>, ()>> = vec![];


                // first pass, register / update all the decl names
                for decl in decls.iter() {
                    // get the new function name
                    let new_fun_name = format!("{}#{}", decl.name.clone(), ann.clone()); // new function name
                    fun_name_mapping.insert(decl.name.clone(), new_fun_name.clone()); // need to do before uniquifying the body, since a local function may refer to it
                }

                // second pass: update all the decl bodies
                for decl in decls.iter() {
                    // new parameter names
                    let mut new_parameters: Vec<String> = vec![];

                    // new var_name_mapping for function body
                    let mut this_decl_var_name_mapping = var_name_mapping.clone();
                    for param_name in decl.parameters.iter() {
                        let new_param_name = format!("{}#{}", param_name.clone(), ann.clone());
                        this_decl_var_name_mapping.insert(param_name.clone(), new_param_name.clone());
                        new_parameters.push(new_param_name.clone());
                    }
                    
                    // new function body
                    let new_body = uniquify_helper(&decl.body, this_decl_var_name_mapping.clone(), fun_name_mapping.clone());
                    // new declaration
                    let new_decl: FunDecl<Exp<()>, ()> = FunDecl{
                        name: fun_name_mapping.get(&decl.name).unwrap().clone(),
                        parameters: new_parameters,
                        body: new_body,
                        ann: ()
                    };
                    new_decls.push(new_decl);
                }

                
                

                // process the body
                let new_def_body = uniquify_helper(body, var_name_mapping.clone(), fun_name_mapping.clone());

                Exp::FunDefs { decls: new_decls, body: Box::new(new_def_body), ann: () }
            },

            Exp::Call(fun_name, args, _ann) => {
                // replace function name
                let new_fun_name = match fun_name_mapping.get(fun_name) {
                    Some(new_name) => new_name.clone(),
                    None => panic!("name_to_query: {}, env: {:?}", fun_name, fun_name_mapping)
                };
                // replace parameter names
                let mut new_args: Vec<Exp<()>> = vec![];
                for arg in args.iter() {
                    new_args.push(uniquify_helper(arg, var_name_mapping.clone(), fun_name_mapping.clone()));
                }

                Exp::Call(new_fun_name.clone(), new_args, ())
            }

            Exp::InternalTailCall(_func_name, _args, _ann) => {
                panic!("InternalTailCall shouldn't occur at uniquifying stage")
            },

            Exp::ExternalCall { fun_name: _, args: _, is_tail: _, ann: _ } => {
                panic!("ExternalCall shouldn't occur at uniquifying stage")
            }
        }
    }

    let var_name_mapping: HashMap<String, String> = HashMap::new();
    let fun_name_mapping: HashMap<String, String> = HashMap::new();
    uniquify_helper(e, var_name_mapping, fun_name_mapping)
}


// ********************************************************************
//
//                           lambda lifting
//
// ********************************************************************

// Identify which functions should be lifted to the top level
fn should_lift<Ann>(p: &Exp<Ann>) -> HashSet<String> {
    fn should_lift_helper<Ann>(p: &Exp<Ann>, is_tail: bool) -> HashSet<String> {
        match p {
            Exp::Num(_, _) => HashSet::new(),

            Exp::Bool(_, _) => HashSet::new(),

            Exp::Var(_, _) => HashSet::new(),

            Exp::Prim(_, exprs, _) => {
                let mut calls_to_lift = HashSet::new();
                // operands are never in tail position
                for expr in exprs.iter() {
                    calls_to_lift.extend(should_lift_helper(expr, false));
                }

                calls_to_lift
            },

            Exp::Let { bindings, body, ann: _ } => {
                let mut calls_to_lift = HashSet::new();
                // If a let-binding is in tail position, then
                // (a) its body is in tail position. 
                // notice that Let also create a local scope
                calls_to_lift.extend(should_lift_helper(body, is_tail));
                // (b) the bindings themselves are not.
                for (_var_name, expr) in bindings.iter() {
                    calls_to_lift.extend(should_lift_helper(expr, false));
                }

                calls_to_lift
            },

            Exp::If { cond, thn, els, ann: _ } => {
                let mut calls_to_lift = HashSet::new();
                // If a conditional is in tail position, then 
                // (a) its branches are in tail position.
                calls_to_lift.extend(should_lift_helper(thn, is_tail));
                calls_to_lift.extend(should_lift_helper(els, is_tail));
                // (b) the condition itself is not.
                calls_to_lift.extend(should_lift_helper(cond, false));

                calls_to_lift
            },

            Exp::FunDefs { decls, body, ann: _ } => {
                let mut calls_to_lift = HashSet::new();
                // process each declaration
                // the body of function definition is in tail position
                for decl in decls.iter() {
                    calls_to_lift.extend(should_lift_helper(&decl.body, true));
                }
                // process the function body
                calls_to_lift.extend(should_lift_helper(body, is_tail));

                calls_to_lift
            },

            Exp::Call(fun_name, arg_exprs, _ann) => {
                let mut calls_to_lift = HashSet::new();
                // this call is a tail call only if caller is in tail position
                if !is_tail { // we only lift a local non-tail call
                    calls_to_lift.insert(fun_name.clone());
                }

                for expr in arg_exprs {
                    calls_to_lift.extend(should_lift_helper(expr, false));
                }

                calls_to_lift
            }, 

            Exp::InternalTailCall(_, _, _) => {
                panic!("InternalTailCall shouldn't occur at should_lift stage")
            },

            Exp::ExternalCall { fun_name: _, args: _, is_tail: _, ann: _ } => {
                panic!("ExternalCall shouldn't occur at should_lift stage")
            }
        }
    }

    fn lift_other_fun_in_scope<Ann>(p: &Exp<Ann>, fun_in_scope: HashSet<String>, lifted_fun: &HashSet<String>) -> HashSet<String> {
        match p {
            Exp::Num(_, _) | Exp::Bool(_, _) | Exp::Var(_, _) |
            Exp::InternalTailCall(_, _, _) | Exp::ExternalCall { fun_name: _, args: _, is_tail: _, ann: _ } => lifted_fun.clone(),
            
            Exp::Prim(_, exprs, _) => {
                let mut calls_to_lift = lifted_fun.clone();
                // operands are never in tail position
                for expr in exprs.iter() {
                    calls_to_lift.extend(lift_other_fun_in_scope(expr, fun_in_scope.clone(), lifted_fun));
                }

                calls_to_lift
            },

            Exp::Let { bindings, body, ann: _ } => {
                let mut calls_to_lift = lifted_fun.clone();
                // If a let-binding is in tail position, then
                // (a) its body is in tail position. 
                // notice that Let also create a local scope
                calls_to_lift.extend(lift_other_fun_in_scope(body, fun_in_scope.clone(), lifted_fun));
                // (b) the bindings themselves are not.
                for (_var_name, expr) in bindings.iter() {
                    calls_to_lift.extend(lift_other_fun_in_scope(expr, fun_in_scope.clone(), lifted_fun));
                }

                calls_to_lift
            },

            Exp::If { cond, thn, els, ann: _ } => {
                let mut calls_to_lift = lifted_fun.clone();
                // If a conditional is in tail position, then 
                // (a) its branches are in tail position.
                calls_to_lift.extend(lift_other_fun_in_scope(thn, fun_in_scope.clone(), lifted_fun));
                calls_to_lift.extend(lift_other_fun_in_scope(els, fun_in_scope.clone(), lifted_fun));
                // (b) the condition itself is not.
                calls_to_lift.extend(lift_other_fun_in_scope(cond, fun_in_scope.clone(), lifted_fun));

                calls_to_lift
            },

            Exp::FunDefs { decls, body, ann: _ } => {
                let mut calls_to_lift = lifted_fun.clone();
                let mut fun_in_this_scope: HashSet<String> = fun_in_scope.clone();
                // process each declaration
                // first pass, add all functions (which are mutually exclusive) in this scope
                for decl in decls {
                    fun_in_this_scope.insert(decl.name.clone());
                }

                // the body of function definition is in tail position
                for decl in decls.iter() {
                    calls_to_lift.extend(lift_other_fun_in_scope(&decl.body, fun_in_this_scope.clone(), lifted_fun));
                    if calls_to_lift.contains(&decl.name) {
                        calls_to_lift.extend(fun_in_this_scope.clone()); // all functions in the scope are also to lift
                    }
                }

                // process the function body
                calls_to_lift.extend(lift_other_fun_in_scope(body, fun_in_this_scope.clone(), lifted_fun));

                calls_to_lift
            },

            Exp::Call(_, arg_exprs, _) => {
                let mut calls_to_lift = lifted_fun.clone();
                for expr in arg_exprs {
                    calls_to_lift.extend(lift_other_fun_in_scope(expr, fun_in_scope.clone(), lifted_fun));
                }

                calls_to_lift
            }
        }
    }

    let fun_non_tail = should_lift_helper(p, true);
    let fun_to_lift = lift_other_fun_in_scope(p, HashSet::new(), &fun_non_tail);
    
    let res = fun_to_lift;
    res
}

// Lift some functions to global definitions
// Convert Call's to InternalTailCall / ExternalCall
fn lambda_lift<Ann>(p: &Exp<Ann>) -> (Vec<FunDecl<Exp<()>, ()>>, Exp<()>) {

    // this function complete three jobs
    // (0) extend the parameter lists of all declarations in original AST
    // (1) seperate all declarations (including InternalTailCalls) and main_body_expr
    // (2) replace Call with InternalTailCalls / ExternalCalls (but leave the parameter list be for this stage)
    fn extend_declaration_and_lift<Ann>(p: &Exp<Ann>, env: HashSet<String>, fun_to_lift: &HashSet<String>, is_tail: bool) -> (Vec<FunDecl<Exp<()>, ()>>, Exp<()>) {
        match p {
            Exp::FunDefs { decls, body, ann: _ } => {
                //* we still need to return ALL func decls in the expr, 
                // so that after this pass we know updated parameter list of ALL of them */ 
                let mut all_decls = vec![]; 

                let mut decls_not_lifted = vec![]; // decls not to list (manually add to main_body_expr before return)

                // process the declarations
                for decl in decls {
                    // capture variables in this scope, for lifting the inside bodies
                    let mut new_env = env.clone();
                    new_env.extend(decl.parameters.clone().into_iter());

                    // lift the inside bodies
                    let (decls_inside, expr_body) = extend_declaration_and_lift(&decl.body, new_env.clone(), fun_to_lift, true); // function body is in tail position

                    // extend parameter list of this declaration
                    let captured_vars: Vec<String> = env.clone().into_iter().collect();
                    let new_parameters = [captured_vars.clone(), decl.parameters.clone()].concat();

                    // construct the new declaration
                    let new_decl = FunDecl {
                        name: decl.name.clone(),
                        parameters: new_parameters,
                        body: expr_body,
                        ann: (),
                    };

                    if !fun_to_lift.contains(&decl.name) { // record it if it is not lifted (add to main_body_expr later)
                        decls_not_lifted.push(new_decl.clone());
                    }

                    all_decls.push(new_decl); // update the decls to return                        
                        

                    all_decls.extend(decls_inside);
                }

                // panic!("count of decls_not_lifted: {}");

                // process the body
                let (decls_in_body, main_body_expr) = extend_declaration_and_lift(body, env.clone(), fun_to_lift, is_tail); // parse the body 
                all_decls.extend(decls_in_body); // update the decls to return

                // construct returned expr
                // (need more discussion)
                // if there are func not to be lifted in the def sequence, 
                // the returned main_body_expr' should be a new FunDefs containing (1) not-to-lift funcs (2) original main_body_expr
                if decls_not_lifted.len() > 0 {

                    let new_main_body_expr = Exp::FunDefs {
                        decls: decls_not_lifted, 
                        body: Box::new(main_body_expr), 
                        ann: () 
                    };

                    (all_decls, new_main_body_expr)
                } else {

                    (all_decls, main_body_expr)
                }
            },

            Exp::Let { bindings, body, ann: _ } => {
                let mut all_decls = vec![];
                let mut cur_env = env.clone();
                let mut new_bindings = vec![];

                // update processsed decls and cur_env
                for (var, expr) in bindings {
                    let (decls_inside, expr_body) = extend_declaration_and_lift(expr, cur_env.clone(), fun_to_lift, false); // bindings are never in tail position
                    all_decls.extend(decls_inside); // extend decls
                    cur_env.insert(var.clone()); // extend cur_env
                    new_bindings.push((var.clone(), expr_body.clone()));
                }

                // process the body of Let and extract decls in the body
                let (body_decls, body_expr) = extend_declaration_and_lift(body, cur_env.clone(), fun_to_lift, is_tail);
                all_decls.extend(body_decls);

                (all_decls, Exp::Let {
                     bindings: new_bindings, 
                     body: Box::new(body_expr), 
                     ann: () 
                })
            },

            Exp::Num(val, _ann) => (vec![], Exp::Num(*val, ())),
            Exp::Bool(val, _ann) => (vec![], Exp::Bool(*val, ())),
            Exp::Var(var, _ann) => (vec![], Exp::Var(var.clone(), ())),

            Exp::Prim(op, exprs, _ann) => {
                let mut all_decls = vec![];
                let mut new_exprs = vec![];

                for expr in exprs {
                    let (expr_decls, expr_body) = extend_declaration_and_lift(expr, env.clone(), fun_to_lift, false); // operands are never in tail position
                    all_decls.extend(expr_decls);
                    new_exprs.push(Box::new(expr_body));
                }

                (all_decls, Exp::Prim(op.clone(), new_exprs, ()))
            }

            Exp::If { cond, thn, els, ann: _ } => {
                let mut all_decls = vec![];

                // process three exprs
                let (cond_decls, cond_body) = extend_declaration_and_lift(cond, env.clone(), fun_to_lift, false); // condition is never in tail position
                all_decls.extend(cond_decls);
                let (thn_decls, thn_body) = extend_declaration_and_lift(thn, env.clone(), fun_to_lift, is_tail);
                all_decls.extend(thn_decls);
                let (els_decls, els_body) = extend_declaration_and_lift(els, env.clone(), fun_to_lift, is_tail);
                all_decls.extend(els_decls);

                (all_decls, Exp::If { 
                    cond: Box::new(cond_body), 
                    thn:  Box::new(thn_body), 
                    els:  Box::new(els_body), 
                    ann: () 
                })
            },

            Exp::Call(fun_name, arg_exprs, _ann) => {
                let mut all_decls = vec![];
                let mut new_arg_exprs = vec![];

                for expr in arg_exprs {
                    let (expr_decls, expr_body) = extend_declaration_and_lift(expr, env.clone(), fun_to_lift, false);
                    all_decls.extend(expr_decls);
                    new_arg_exprs.push(expr_body);
                }

                if fun_to_lift.contains(fun_name) { // external call
                    (all_decls, 
                        
                     Exp::ExternalCall { 
                        fun_name: fun_name.clone(), 
                        args: new_arg_exprs, 
                        is_tail: is_tail, 
                        ann: () }
                    )
                } else { // internal tail call
                    (all_decls,

                     Exp::InternalTailCall(
                        fun_name.clone(), 
                        new_arg_exprs, 
                        ())
                    )
                }
            },

            Exp::InternalTailCall(_, _, _) => {
                panic!("InternalTailCall shouldn't occur at extend_parameters stage")
            },

            Exp::ExternalCall { fun_name: _, args: _, is_tail: _, ann: _ } => {
                panic!("ExternalCall shouldn't occur at extend_parameters stage")
            }
        }
    }
    
    // generate the lifted global FunDefs
    fn generate_global_decls(all_decls: Vec<FunDecl<Exp<()>, ()>>, fun_to_lift: &HashSet<String>) -> Vec<FunDecl<Exp<()>, ()>>{
        let mut decls_to_lift = vec![];
        for decl in all_decls {
            if fun_to_lift.contains(&decl.name) {
                decls_to_lift.push(decl.clone());
            }
        }

        decls_to_lift
    }

    // get the mapping from function name to its parameter (name) list
    fn fun_decls_to_hashmap(all_decls: &Vec<FunDecl<Exp<()>, ()>>) -> HashMap<String, Vec<Exp<()>>> {
        let mut map = HashMap::new();

        for decl in all_decls.iter() {
            let args: Vec<Exp<()>> = decl.parameters.iter().map(|param_name| Exp::Var(param_name.clone(), ())).collect();
            map.insert(decl.name.clone(), args);
        }

        map
    }

    // extend all Calls' parameter list: [<added free variables>, <original argument expressions>].concat()
    fn extend_fun_calls(exp: &Exp<()>, name_param_mapping: &HashMap<String, Vec<Exp<()>>>) -> Exp<()> {
        match exp {
            Exp::Num(_, _) | Exp::Bool(_, _) | Exp::Var(_, _) => exp.clone(),

            Exp::Prim(op, exprs, ann) => {
                let mut new_exprs = vec![];
                for expr in exprs {
                    new_exprs.push(Box::new(extend_fun_calls(expr, name_param_mapping)));
                }

                Exp::Prim(op.clone(), new_exprs, ann.clone())
            },

            Exp::Let { bindings, body, ann: _ } => {
                let new_bindings = bindings.iter().map(|(var_name, bind_expr)| (var_name.clone(), extend_fun_calls(bind_expr, name_param_mapping))).collect();
                let new_body = extend_fun_calls(body, name_param_mapping);
                
                Exp::Let { 
                    bindings: new_bindings, 
                    body: Box::new(new_body), 
                    ann: () 
                }
            }

            Exp::If { cond, thn, els, ann } => {
                Exp::If { 
                    cond: Box::new(extend_fun_calls(cond, name_param_mapping)), 
                    thn: Box::new(extend_fun_calls(thn, name_param_mapping)), 
                    els: Box::new(extend_fun_calls(els, name_param_mapping)),
                    ann: ann.clone() 
                }
            },

            Exp::FunDefs { decls, body, ann } => {
                let mut new_decls = vec![];

                for decl in decls {
                    new_decls.push(FunDecl {
                        name: decl.name.clone(),
                        parameters: decl.parameters.clone(),
                        body: extend_fun_calls(&decl.body, name_param_mapping),
                        ann: ann.clone(),
                    })
                }

                Exp::FunDefs { 
                    decls: new_decls, 
                    body: Box::new(extend_fun_calls(body, name_param_mapping)), 
                    ann: ann.clone(),
                }
            },

            Exp::Call(_fun_name, _arg_exprs, _ann) => {
                panic!("Call shouldn't occur at extend_fun_calls stage")
            },

            Exp::InternalTailCall(fun_name, arg_exprs, ann) => {
                let extended_arg_exprs: Vec<Exp<()>> = arg_exprs.iter().map(|expr| {
                    extend_fun_calls(expr, name_param_mapping)
                }).collect();

                let var_name_list = match name_param_mapping.get(fun_name) {
                    Some(list) => list,
                    None => {
                        panic!("{} is not found in name_param_mapping, which of length {}", fun_name, name_param_mapping.len())
                    }
                };
                let params_to_extend = var_name_list[0..(var_name_list.len() - extended_arg_exprs.len())].to_vec();
                let extended_call_param_list = [params_to_extend, extended_arg_exprs.clone()].concat();
                assert_eq!(var_name_list.len(), extended_call_param_list.len());

                Exp::InternalTailCall(
                    fun_name.clone(), 
                    extended_call_param_list, 
                    ann.clone()
                )
            },

            Exp::ExternalCall { fun_name, args: arg_exprs, is_tail, ann: _ } => {
                let extended_arg_exprs: Vec<Exp<()>> = arg_exprs.iter().map(|expr| {
                    extend_fun_calls(expr, name_param_mapping)
                }).collect();

                let var_name_list = match name_param_mapping.get(fun_name) {
                    Some(list) => list,
                    None => panic!("{} is not found in name_param_mappingwhich of length {}", fun_name, name_param_mapping.len())
                };
                let params_to_extend = var_name_list[0..(var_name_list.len() - extended_arg_exprs.len())].to_vec();
                let extended_call_param_list = [params_to_extend, extended_arg_exprs.clone()].concat();
                assert_eq!(var_name_list.len(), extended_call_param_list.len());

                Exp::ExternalCall { 
                    fun_name: fun_name.clone(), 
                    args: extended_call_param_list, 
                    is_tail: *is_tail, 
                    ann: () 
                }
            }
        }
    }

    fn extend_fun_decls(decls: &Vec<FunDecl<Exp<()>, ()>>, name_param_mapping: &HashMap<String, Vec<Exp<()>>>) -> Vec<FunDecl<Exp<()>, ()>> {
        decls.iter().map(|decl| {
            // extended param list
            let var_name_list = match name_param_mapping.get(&decl.name) {
                Some(list) => list,
                None => panic!("{} is not found in name_param_mappingwhich of length {}", &decl.name, name_param_mapping.len())
            };
            let params_exprs_to_extend = var_name_list[0..(var_name_list.len() - decl.parameters.len())].to_vec();
            let params_var_to_extend: Vec<String> = params_exprs_to_extend.iter().map(|expr| {
                match expr {
                    Exp::Var(var, _) => var.clone(),
                    _ => panic!("params_exprs_to_extend should contain only Exp::Var!\n{:#?}", params_exprs_to_extend)
                }
            }).collect();
            let extended_call_param_list = [params_var_to_extend, decl.parameters.clone()].concat();
            assert_eq!(var_name_list.len(), extended_call_param_list.len());

            // extended body
            let extended_body = extend_fun_calls(&decl.body, name_param_mapping);


            FunDecl {
                name: decl.name.clone(),
                parameters: extended_call_param_list, // new param list
                body: extended_body,
                ann: decl.ann,
            }
        }).collect()
    }

    // seperate decls and main_body
    let fun_to_lift = should_lift(p); // extract all func_name to lift
    let (unextended_all_decls, unextended_main_body) = extend_declaration_and_lift(p, HashSet::new(), &fun_to_lift, true); 

    // extend the call parameters 
    let name_param_mapping = fun_decls_to_hashmap(&unextended_all_decls);
    // panic!("length of mapping: {}, count of all_decls: {}, count of fun_to_lift: {}",
    //     name_param_mapping.len(), unextended_all_decls.len(), fun_to_lift.len());
    let extended_main_body = extend_fun_calls(&unextended_main_body, &name_param_mapping);
    let extended_all_decls = extend_fun_decls(&unextended_all_decls, &name_param_mapping);
    let extended_global_decls = generate_global_decls(extended_all_decls.clone(), &fun_to_lift);

    (extended_global_decls, extended_main_body)

}


// ********************************************************************
//
//                              taggings 
//
// ********************************************************************

fn tag_exp<Ann>(e: &Exp<Ann>, tag: &mut u32) -> Exp<u32> {
    *tag = *tag + 1;
    let cur_tag: u32 = tag.clone();
    match e {
        Exp::Num(n, _) => return Exp::Num(*n, cur_tag),

        Exp::Bool(b, _) => return Exp::Bool(*b, cur_tag),

        Exp::Var(v, _) => return Exp::Var(v.clone(), cur_tag),

        Exp::Prim(prim, exp_vec, _) => {
            let mut tagged_exp_vec: Vec<Box<Exp<u32>>> = Vec::new();
            for exp in exp_vec.iter() {
                tagged_exp_vec.push(Box::new(tag_exp(exp, tag)));
            }
            return Exp::Prim(*prim, tagged_exp_vec, cur_tag)
        },

        Exp::Let{bindings, body, ..} => {
            let mut tagged_bindings: Vec<(String, Exp<u32>)> = Vec::new();
            for (var_name, exp) in bindings.iter() {
                tagged_bindings.push((var_name.clone(), tag_exp(exp, tag)));
            }
            return Exp::Let{
                bindings: tagged_bindings, 
                body: Box::new(tag_exp(body, tag)), 
                ann: cur_tag
            }
        },

        Exp::If{cond, thn, els, ..} => {
            return Exp::If{
                cond: Box::new(tag_exp(cond, tag)),
                thn: Box::new(tag_exp(thn, tag)),
                els: Box::new(tag_exp(els, tag)),
                ann: cur_tag
            }
        },

        Exp::FunDefs { decls, body, ann: _ } => {
            let new_decls = decls.iter().map(|decl| {
                let tagged_body = tag_exp(&decl.body, tag);
                FunDecl {
                    name: decl.name.clone(),
                    parameters: decl.parameters.clone(),
                    body: tagged_body,
                    ann: cur_tag,
                }
            }).collect();

            Exp::FunDefs { 
                decls: new_decls, 
                body: Box::new(tag_exp(body, tag)), 
                ann: cur_tag
            }
        },

        Exp::Call(fun_name, arg_exprs, _ann) => {
            let new_arg_exprs = arg_exprs.iter().map(|expr| tag_exp(expr, tag)).collect();

            Exp::Call(fun_name.clone(), new_arg_exprs, cur_tag)
        },

        Exp::InternalTailCall(fun_name, arg_exprs, _ann) => {
            let new_arg_exprs = arg_exprs.iter().map(|expr| tag_exp(expr, tag)).collect();

            Exp::InternalTailCall(fun_name.clone(), new_arg_exprs, cur_tag)
        },

        Exp::ExternalCall { fun_name, args, is_tail, ann: _ } => {
            let new_args = args.iter().map(|expr| tag_exp(expr, tag)).collect();

            Exp::ExternalCall { 
                fun_name: fun_name.clone(), 
                args: new_args, 
                is_tail: *is_tail, 
                ann: cur_tag, 
            }
        }
    }
}

fn tag_fundecl(defs: &Vec<FunDecl<Exp<()>, ()>>, tag: &mut u32) -> Vec<FunDecl<Exp<u32>, u32>> {
    defs.iter().map(|decl| {
        FunDecl {
            name: decl.name.clone(),
            parameters: decl.parameters.clone(),
            body: tag_exp(&decl.body, tag),
            ann: tag.clone(),
        }
    }).collect()
}


fn tag_prog(defs: &Vec<FunDecl<Exp<()>, ()>>, main: Exp<()>) -> (Vec<FunDecl<Exp<u32>, u32>>, Exp<u32>) {
    let mut tag: u32 = 0;
    let tagged_defs = tag_fundecl(defs, &mut tag);
    let tagged_main = tag_exp(&main, &mut tag);
    (tagged_defs, tagged_main)
}


fn sequentialize(e: &Exp<u32>) -> SeqExp<()> {
    match e {
        Exp::Num(i, _) => SeqExp::Imm(ImmExp::Num(*i), ()),
        Exp::Bool(b, _) => SeqExp::Imm(ImmExp::Bool(*b), ()),
        Exp::Var(x, _) => SeqExp::Imm(ImmExp::Var(x.clone()), ()),
        Exp::Prim(cur_prim, exp_vec, cur_tag) => {
            match cur_prim {
                Prim::Add1 | Prim::Sub1 | Prim::Not | Prim::Print | Prim::IsBool | Prim::IsNum => {
                    let seq_exp = sequentialize(exp_vec[0].as_ref());
                    let cur_name = format!("#prim1_{}", cur_tag);
                    return SeqExp::Let{ // let #prim1_tag = seq_exp in Prim1(#prim1_tag)
                        var: cur_name.clone(), 
                        bound_exp: Box::new(seq_exp),
                        body: Box::new(SeqExp::Prim(cur_prim.clone(), vec![ImmExp::Var(cur_name)], ())),
                        ann: ()
                    }
                } ,   
                Prim::Add | Prim::Sub | Prim::Mul | Prim::And | Prim::Or | Prim::Lt | Prim::Gt 
                | Prim::Le | Prim::Ge | Prim::Eq | Prim::Neq => {
                    let seq_exp_1 = sequentialize(exp_vec[0].as_ref());
                    let seq_exp_2 = sequentialize(exp_vec[1].as_ref());
                    let name_1 = format!("#prim2_1_{}", cur_tag);
                    let name_2 = format!("#prim2_2_{}", cur_tag);
                    return SeqExp::Let{
                        var: name_1.clone(),
                        bound_exp: Box::new(seq_exp_1),
                        body: Box::new(SeqExp::Let{
                            var: name_2.clone(),
                            bound_exp: Box::new(seq_exp_2),
                            body: Box::new(SeqExp::Prim(cur_prim.clone(), vec![ImmExp::Var(name_1), ImmExp::Var(name_2)], ())),
                            ann: ()
                        }),
                        ann: ()
                    }
                }
            }
        },
        Exp::Let { bindings, body, ann: _ } => {
            let mut current_body = sequentialize(body);
            for (var_name, var_exp) in bindings.iter().rev() {
                let var_exp_seq = sequentialize(var_exp);
                current_body = SeqExp::Let {
                    var: var_name.clone(),
                    bound_exp: Box::new(var_exp_seq),
                    body: Box::new(current_body),
                    ann: (),
                };
            }
            return current_body;
        },
        Exp::If { cond, thn, els, ann } => {
            let seq_exp_1 = sequentialize(cond);
            let seq_exp_2 = sequentialize(thn);
            let seq_exp_3 = sequentialize(els);
            let cond_name = format!("#if_cond_{}", ann);
            return SeqExp::Let{ // let x1 = se1 in if x1: se2 else: se3
                var: cond_name.clone(),
                bound_exp: Box::new(seq_exp_1),
                body: Box::new(SeqExp::If{
                    cond: ImmExp::Var(cond_name),
                    thn: Box::new(seq_exp_2),
                    els: Box::new(seq_exp_3),
                    ann: ()
                }),
                ann: ()
            }
        },
        Exp::FunDefs { decls, body, ann: _ } => {
            let mut tagged_decl: Vec<FunDecl<SeqExp<()>, ()>> = Vec::new();
            for decl in decls.iter() {
                tagged_decl.push(FunDecl { 
                    name: decl.name.clone(), 
                    parameters: decl.parameters.clone(), 
                    body: sequentialize(&decl.body), 
                    ann: () 
                })
            }
            return SeqExp::FunDefs { 
                decls: tagged_decl, 
                body: Box::new(sequentialize(&body)), 
                ann: () 
            }
        },
        Exp::Call(_s, _exp_vec, _ann) => {
            panic!("should be either internal or external call here");
        },
        Exp::InternalTailCall(s, exp_vec, ann) => {
            let mut seq_args: Vec<ImmExp> = Vec::new();
            let mut bindings: Vec<(String, Exp<u32>)> = Vec::new();
            for (i, arg) in exp_vec.iter().enumerate() {
                let arg_name = format!("#function_{}_arg_{}", ann, i);
                seq_args.push(ImmExp::Var(arg_name.clone()));
                bindings.push((arg_name.clone(), arg.clone()));
            }
            let mut current_body = SeqExp::InternalTailCall(s.clone(), seq_args.clone(), ());
            for (var_name, var_exp) in bindings.iter().rev() {
                let var_exp_seq = sequentialize(var_exp);
                current_body = SeqExp::Let {
                    var: var_name.clone(),
                    bound_exp: Box::new(var_exp_seq),
                    body: Box::new(current_body),
                    ann: (),
                };
            }
            return current_body
        },
        Exp::ExternalCall { fun_name, args, is_tail, ann } => {
            let mut seq_args: Vec<ImmExp> = Vec::new();
            let mut bindings: Vec<(String, Exp<u32>)> = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                let arg_name = format!("#function_{}_arg_{}", ann, i);
                seq_args.push(ImmExp::Var(arg_name.clone()));
                bindings.push((arg_name.clone(), arg.clone()));
            }
            let mut current_body = SeqExp::ExternalCall { 
                fun_name: fun_name.clone(), 
                args: seq_args.clone(), 
                is_tail: is_tail.clone(), 
                ann: () 
            };
            for (var_name, var_exp) in bindings.iter().rev() {
                let var_exp_seq = sequentialize(var_exp);
                current_body = SeqExp::Let {
                    var: var_name.clone(),
                    bound_exp: Box::new(var_exp_seq),
                    body: Box::new(current_body),
                    ann: (),
                };
            }
            return current_body
        }
    }
}

fn seq_prog(decls: &[FunDecl<Exp<u32>, u32>], p: &Exp<u32>) -> SeqProg<()> {
    let mut seq_funs: Vec<FunDecl<SeqExp<()>, ()>> = Vec::new();
    for decl in decls.iter() {
        seq_funs.push(FunDecl{
            name: decl.name.clone(),
            parameters: decl.parameters.clone(),
            body: sequentialize(&decl.body),
            ann: ()
        })
    }
    return SeqProg{
        funs: seq_funs,
        main: sequentialize(&p),
        ann: ()
    }
}

fn tag_seq_exp(seq_e: &SeqExp<()>, tag: &mut u32) -> SeqExp<u32> {
    *tag = *tag + 1;
    let cur_tag: u32 = tag.clone();
    match seq_e {
        SeqExp::Imm(i, _) => return SeqExp::Imm(i.clone(), cur_tag),
        SeqExp::Prim(prim, s_e_vec, _) => return SeqExp::Prim(*prim, s_e_vec.clone(), cur_tag),
        SeqExp::Let{var, bound_exp, body, ..} => {
            return SeqExp::Let{
                var: var.clone(),
                bound_exp: Box::new(tag_seq_exp(bound_exp, tag)),
                body: Box::new(tag_seq_exp(body, tag)),
                ann: cur_tag
            }
        },
        SeqExp::If{cond, thn, els, ..} => {
            return SeqExp::If{
                cond: cond.clone(),
                thn: Box::new(tag_seq_exp(thn, tag)),
                els: Box::new(tag_seq_exp(els, tag)),
                ann: cur_tag
            }
        },
        SeqExp::FunDefs { decls, body, ann: _ } => {
            let mut tagged_decls: Vec<FunDecl<SeqExp<u32>, u32>> = Vec::new();
            for decl in decls.iter() {
                let decl_tag: u32 = tag.clone();
                tagged_decls.push(FunDecl { 
                    name: decl.name.clone(), 
                    parameters: decl.parameters.clone(), 
                    body: tag_seq_exp(&decl.body, tag), 
                    ann: decl_tag
                })
            }
            return SeqExp::FunDefs { 
                decls: tagged_decls, 
                body: Box::new(tag_seq_exp(body, tag)), 
                ann: cur_tag
            };
        },
        SeqExp::InternalTailCall(s, i_exp_vec, _) => {
            return SeqExp::InternalTailCall(s.clone(), i_exp_vec.clone(), cur_tag);
        },
        SeqExp::ExternalCall { fun_name, args, is_tail, ann: _ } => {
            return SeqExp::ExternalCall { fun_name: fun_name.clone(), args: args.clone(), is_tail: is_tail.clone(), ann: cur_tag };
        }
    }
}

fn tag_sprog(p: &SeqProg<()>) -> SeqProg<u32> {
    // pub funs: Vec<FunDecl<SeqExp<Ann>, Ann>>,
    // pub main: SeqExp<Ann>,
    // pub ann: Ann,
    let mut tag: u32 = 1;
    let mut new_funs: Vec<FunDecl<SeqExp<u32>, u32>> = Vec::new();
    for fun in p.funs.iter() {
        let cur_tag = tag.clone();
        new_funs.push(FunDecl { 
            name: fun.name.clone(), 
            parameters: fun.parameters.clone(), 
            body: tag_seq_exp(&fun.body, &mut tag), 
            ann: cur_tag
        })
    }

    return SeqProg{funs: new_funs, main: tag_seq_exp(&p.main, &mut tag), ann: 0};
}


fn get_offset(var: &String, env: &HashMap<String, i32>) -> i32 {
    match env.get(var) {
        Some(offset) => return *offset,
        None => {
            panic!("Variable {} should be in scope, content of current env is {:?}", var.clone(), env)
        }
    }
}

fn compile_imm_to_arg(i: &ImmExp, env: &HashMap<String, i32>) -> Arg64 {
    match i {
        ImmExp::Num(n) => return Arg64::Signed(*n << 1),
        ImmExp::Bool(b) => {
            if *b {return Arg64::Unsigned(SNAKE_TRU.0)} else {return Arg64::Unsigned(SNAKE_FLS.0)}
        },
        ImmExp::Var(x) => {
            let addr = get_offset(x, env);
            return Arg64::Mem(MemRef{reg: Reg::Rsp, offset: addr})
        }
    }
}

fn runtime_overflow_check() -> Vec<Instr> {
    let mut is: Vec<Instr> = Vec::new();
    is.push(Instr::Comment(String::from("Check overflow")));
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rdi, Arg64::Unsigned(OVFL_ERROR))));
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rsi, Arg64::Reg(Reg::Rax))));
    is.push(Instr::Jo(String::from("snake_err")));
    return is
}

fn check_type_num(reg: Reg, err_code: ErrorCode) -> Vec<Instr> {
    let mut is: Vec<Instr> = Vec::new();
    is.push(Instr::Comment(String::from("Check Whether Num")));
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rdi, Arg64::Unsigned(err_code))));       // first par of snake_err(): err_code
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rsi, Arg64::Reg(reg))));                 // second par of snake_err(): snakeval
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rbx, Arg64::Unsigned(TAG_MASK))));       // mov Rbx, TAG_MASK
    is.push(Instr::Test(BinArgs::ToReg(Reg::Rbx, Arg32::Reg(reg))));                // test Rbx, (reg)
    is.push(Instr::Jnz(String::from("snake_err")));                                 // jnz snake_err
    return is
}

fn check_type_bool(reg: Reg, err_code: ErrorCode) -> Vec<Instr> {
    let mut is: Vec<Instr> = Vec::new();
    is.push(Instr::Comment(String::from("Check Whether Bool")));
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rdi, Arg64::Unsigned(err_code))));       // mov Rdi, LOGIC_ERROR
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rsi, Arg64::Reg(reg))));                 // mov Rsi, (reg)
    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rbx, Arg64::Unsigned(TAG_MASK))));       // mov Rbx, TAG_MASK
    is.push(Instr::Test(BinArgs::ToReg(Reg::Rbx, Arg32::Reg(reg))));                // test Rbx, (reg)
    is.push(Instr::Jz(String::from("snake_err")));                                  // jz snake_err
    return is
}

fn runtime_prim1_check(reg: Reg, p: &Prim) -> Vec<Instr> {
    match p {
        Prim::Add1 | Prim::Sub1 => return check_type_num(reg.clone(), ARITH_ERROR),
        Prim::Not               => return check_type_bool(reg.clone(), LOGIC_ERROR),
        _ => return Vec::new()
    }
}

fn runtime_prim2_check(reg: Reg, p: &Prim) -> Vec<Instr> {
    match p {
        Prim::Add | Prim::Sub | Prim::Mul                       => return check_type_num(reg.clone(), ARITH_ERROR),
        Prim::And | Prim::Or                                    => return check_type_bool(reg.clone(), LOGIC_ERROR),
        Prim::Lt | Prim::Gt | Prim::Le | Prim::Ge               => return check_type_num(reg.clone(), CMP_ERROR),
        _ => return Vec::new()
    }
}

fn runtime_if_check(reg: Reg) -> Vec<Instr> {
    return check_type_bool(reg.clone(), IF_ERROR)
}

fn compile_prim1_to_instr(p: &Prim, space: &i32) -> Vec<Instr> {
    let mut is: Vec<Instr> = Vec::new();
    match p {
        Prim::Add1 => {
            is.push(Instr::Comment(String::from("Add1")));
            is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1 << 1))));
            is.extend(runtime_overflow_check());                                        // overflow check
        },
        Prim::Sub1 => {
            is.push(Instr::Comment(String::from("Sub1")));
            is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1 << 1))));
            is.extend(runtime_overflow_check());                                        // overflow check
        },
        Prim::Not => {
            // let NOT_MUSK: u64 = 0x80_00_00_00_00_00_00_00;
            is.push(Instr::Comment(String::from("Not")));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(NOT_MUSK))));
            is.push(Instr::Xor(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
        },
        Prim::Print => {
            is.push(Instr::Comment(String::from("Print")));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::Rdi, Arg64::Reg(Reg::Rax))));            // mov Rdi, (reg)
            is.push(Instr::Sub(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(*space + 8))));     // sub Rsp, (space + 8)
            is.push(Instr::Call(String::from("print_snake_val")));                          // call print_snake_val
            is.push(Instr::Add(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(*space + 8))));     // add Rsp, (space + 8)
        },
        Prim::IsNum => {
            is.push(Instr::Comment(String::from("IsNum")));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));       // mov R10, TAG_MASK

            is.push(Instr::And(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));            // and Rax, R10
            is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(63))));             // shl Rax, 63

            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(SNAKE_TRU.0))));      // mov R10, SNAKE_TRU
            is.push(Instr::Xor(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));            // xor Rax, R10
        },
        Prim::IsBool => {
            is.push(Instr::Comment(String::from("IsBool")));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));       // mov R10, TAG_MASK

            is.push(Instr::And(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));            // and Rax, R10
            is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(63))));             // shl Rax, 63

            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(SNAKE_FLS.0))));      // mov R10, SNAKE_FLS
            is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));             // or Rax, R10
        },
        _ => panic!("unexpected situation: expect Prim1")
    }
    return is
}

fn compile_prim2_to_instr(p: &Prim, ann: &u32) -> Vec<Instr> {
    let mut is: Vec<Instr> = Vec::new();
    match p {
        Prim::Add => {
            is.push(Instr::Comment(String::from("Add")));
            is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
            is.extend(runtime_overflow_check());                                        // overflow check
        },
        Prim::Sub => {
            is.push(Instr::Comment(String::from("Sub")));
            is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
            is.extend(runtime_overflow_check());                                        // overflow check
        },
        Prim::Mul => {
            is.push(Instr::Comment(String::from("Mul")));
            is.push(Instr::Sar(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));             
            is.push(Instr::IMul(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
            is.extend(runtime_overflow_check());                                        // overflow check 
        },
        Prim::And => {
            is.push(Instr::And(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
        },
        Prim::Or => {
            is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
        },
        _ => {
            let cur_str: String = {match p {
                Prim::Lt => format!("less_than#{}", ann),
                Prim::Gt => format!("greater_than#{}", ann),
                Prim::Le => format!("less_equal#{}", ann),
                Prim::Ge => format!("greater_equal#{}", ann),
                Prim::Eq => format!("equal#{}", ann),
                Prim::Neq => format!("unequal#{}", ann),
                _ => panic!("unexpected situation: expect comparison")
            }};
            is.push(Instr::Comment(String::from("Compare")));
            is.push(Instr::Cmp(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));        // cmp rax, r10
            is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Unsigned(SNAKE_TRU.0))));  // mov rax, SNAKE_TRU
            is.push(match p {                                                           // (cond_jump) (cur_label)
                Prim::Lt => Instr::Jl(cur_str.clone()),
                Prim::Gt => Instr::Jg(cur_str.clone()),
                Prim::Le => Instr::Jle(cur_str.clone()),
                Prim::Ge => Instr::Jge(cur_str.clone()),
                Prim::Eq => Instr::Je(cur_str.clone()),
                Prim::Neq => Instr::Jne(cur_str.clone()),
                _ => panic!("unexpected situation: expect comparison")
            });
            is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Unsigned(SNAKE_FLS.0))));  // mov rax, SNAKE_FLS
            is.push(Instr::Label(cur_str));                                             // (cur_label)
        }
    }
    return is
}

fn space_needed_helper(e: &SeqExp<u32>) -> i32 {
    let mut var_num = 0;
    match e {
        SeqExp::Imm(..) | SeqExp::Prim(..) | SeqExp::InternalTailCall(..) | SeqExp::ExternalCall{..} => {},
        SeqExp::Let{var: _, bound_exp, body, ann: _} => {
            var_num = std::cmp::max(space_needed_helper(bound_exp), 1 + space_needed_helper(body));
        },
        SeqExp::If{cond: _, thn, els, ann: _} => {
            var_num = std::cmp::max(space_needed_helper(thn), space_needed_helper(els));
        },
        SeqExp::FunDefs{decls, body, ann: _} => {
            let mut max_space = 0;
            for decl in decls.iter() {
                max_space = std::cmp::max(max_space, space_needed_helper(&decl.body));
            }
            var_num = max_space + space_needed_helper(body);
        }
    }
    return var_num
}

fn space_needed(e: &SeqExp<u32>, arg_num: i32) -> i32 {
    let var_num = space_needed_helper(e) + arg_num;
    if var_num % 2 == 0 {
        return  8 * (var_num + 1)
    } else {
        return 8 * var_num
    }
}

fn compile_to_instrs_help(e: &SeqExp<u32>, mut env: HashMap<String, i32>, space: &i32, env_size: i32) -> Vec<Instr> {
    match e {
        SeqExp::Imm(i_exp, _ann) => {
            return vec![Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(i_exp, &env)))]
        },
        SeqExp::Prim(prim, i_exp_vec, ann) => {
            match prim {
                Prim::Add1 | Prim::Sub1 | Prim::Not | Prim::Print | Prim::IsNum | Prim::IsBool => {
                    let mut is: Vec<Instr> = Vec::new();
                    is.push(Instr::Comment(String::from("Prim1")));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(&i_exp_vec[0], &env))));
                    is.extend(runtime_prim1_check(Reg::Rax, prim));
                    is.extend(compile_prim1_to_instr(prim, space));
                    return is
                },
                Prim::Add | Prim::Sub | Prim::Mul | Prim::And | Prim::Or | Prim::Lt | Prim::Gt | Prim::Le | Prim::Ge | Prim::Eq | Prim::Neq => {
                    let mut is: Vec<Instr> = Vec::new();
                    is.push(Instr::Comment(String::from("Prim2")));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(&i_exp_vec[0], &env))));
                    is.extend(runtime_prim2_check(Reg::Rax, prim));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, compile_imm_to_arg(&i_exp_vec[1], &env))));
                    is.extend(runtime_prim2_check(Reg::R10, prim));
                    is.extend(compile_prim2_to_instr(prim, ann));
                    return is
                }
            }
        },
        SeqExp::Let{var, bound_exp, body, ann: _} => {
            let mut is = compile_to_instrs_help(&bound_exp, env.clone(), space, env_size);
            is.push(Instr::Comment(format!("Let var: {}", var)));
            env.insert(var.clone(), -8 * (env.len() as i32 + 1));

            let new_offset = get_offset(var, &env);
            is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg: Reg::Rsp, offset: new_offset}, Reg32::Reg(Reg::Rax))));

            is.extend(compile_to_instrs_help(&body, env.clone(), space, env_size+1));

            return is
        },

        SeqExp::If{cond, thn, els, ann} => {
            let else_lab = format!("if_false#{}", *ann);
            let done_lab = format!("done#{}", *ann);

            let mut is: Vec<Instr> = Vec::new();
            is.push(Instr::Comment(String::from("If")));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(cond, &env))));    // mov Rax, eval(cond)
            is.extend(runtime_if_check(Reg::Rax));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(SNAKE_FLS.0))));        // mov R10, SNAKE_FLS
            is.push(Instr::Cmp(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));                // cmp rax, r10

            is.push(Instr::Je(else_lab.clone()));                                               // Je if_false#{ann}
            is.extend(compile_to_instrs_help(&thn, env.clone(), space, env_size));            // eval(thn)
            is.push(Instr::Jmp(done_lab.clone()));                                              // Jmp done#{ann}
            is.push(Instr::Label(else_lab.clone()));   // if_false#{ann}       
            is.extend(compile_to_instrs_help(&els, env.clone(), space, env_size));            // eval(els)
            is.push(Instr::Label(done_lab.clone()));                                            // done#{ann}

            return is
        },

        SeqExp::FunDefs {decls, body, ann} => {
            // panic!("env_len:{}, env_size:{}", env.len(), env_size);
            let mut is: Vec<Instr> = Vec::new();
            is.push(Instr::Comment(format!("FunDefs{}_body", ann)));
            is.extend(compile_to_instrs_help(&body, env.clone(), space, env_size));
            is.push(Instr::Ret);

            is.push(Instr::Comment(format!("FunDefs{}_decls", ann)));

            for decl in decls {
                is.push(Instr::Label(decl.name.clone()));

                // get the env of this scope: (free variables from outside + func params inside)
                let mut this_env = env.clone();
                for var_name in decl.parameters.iter() {
                    if !this_env.contains_key(var_name) { // only update variables not in old env keys
                        this_env.insert(var_name.clone(), -8*(this_env.len() as i32 + 1));
                    }
                }

                is.extend(compile_to_instrs_help(&decl.body, this_env.clone(), space, this_env.len() as i32));
                is.push(Instr::Ret);
            }

        
            return is
        },

        SeqExp::InternalTailCall(s, i_exp_vec, _ann) => {
            let mut is: Vec<Instr> = Vec::new();
            is.push(Instr::Comment(String::from("InCall")));

            let mut count: i32 = 16;
            for arg in i_exp_vec.iter() {
                is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(&arg, &env))));
                is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg: Reg::Rsp, offset: -space - count}, Reg32::Reg(Reg::Rax))));
                count += 8;
            }
            let mut arg_idx: i32 = 8;
            for _ in i_exp_vec.iter() {
                is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg: Reg::Rsp, offset: -space - arg_idx - 8}))));
                is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg: Reg::Rsp, offset: -arg_idx}, Reg32::Reg(Reg::Rax))));
                arg_idx += 8;
            }
            is.push(Instr::Jmp(s.clone()));

            return is
        },
        SeqExp::ExternalCall{fun_name, args, is_tail, ann: _} => {
            let mut is: Vec<Instr> = Vec::new();
            is.push(Instr::Comment(String::from("ExCall")));

            // if fun_name.clone() == String::from("div#21") && *is_tail {
            //     panic!("{:#?}\n{:#?}", args, env);
            // }

            let mut count: i32 = 16;
            for arg in args.iter() {
                is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, compile_imm_to_arg(&arg, &env))));
                is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg: Reg::Rsp, offset: -space - count}, Reg32::Reg(Reg::Rax))));
                count += 8;
            }

            if *is_tail {
                let mut arg_idx: i32 = 8;
                for _ in args.iter() {
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg: Reg::Rsp, offset: -space - arg_idx - 8}))));
                    is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg: Reg::Rsp, offset: -arg_idx}, Reg32::Reg(Reg::Rax))));
                    arg_idx += 8;
                }
                is.push(Instr::Jmp(fun_name.clone()));
            } else {
                is.push(Instr::Sub(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(*space))));
                is.push(Instr::Call(fun_name.clone()));
                is.push(Instr::Add(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(*space))));
            }

            return is
        }
    }
}

fn compile_to_instrs(p: &SeqProg<u32>) -> Vec<Instr> {
    let mut instrs: Vec<Instr> = Vec::new();

    // start by calling main
    instrs.push(Instr::Call(String::from("main")));
    instrs.push(Instr::Ret);

    // asm for the main body (entry point)
    instrs.push(Instr::Label(String::from("main")));
    instrs.extend(compile_to_instrs_help(
        &p.main,
        HashMap::new(),
        &space_needed(&p.main, 0),
        0
    ));
    instrs.push(Instr::Ret);


    instrs.push(Instr::Comment(String::from("Global FunDecls")));
    // asm for the global function definition
    for fun in p.funs.iter() {
        instrs.push(Instr::Label(fun.name.clone()));
        let mut env: HashMap<String, i32> = HashMap::new();
        for (i, arg) in fun.parameters.iter().enumerate() {
            env.insert(arg.clone(), -8 * (i as i32 + 1));
        }
        let num_pars = fun.parameters.len() as i32;
        instrs.extend(compile_to_instrs_help(
            &fun.body,
            env,
            &space_needed(&fun.body, num_pars),
            num_pars
        ));
        instrs.push(Instr::Ret);
    }
    instrs.push(Instr::Label(String::from("snake_err")));
    instrs.push(Instr::Call(String::from("snake_error")));

    return instrs
}


// ********************************************************************
//
//                        compile to string
//
// ********************************************************************
pub fn compile_to_string<Span1>(p: &Exp<Span1>) -> Result<String, CompileErr<Span1>>
where Span1: Clone,
{
    check_prog(p)?;

    let uniq_p = uniquify(&tag_exp(p, &mut 0));

    let (defs, main) = lambda_lift(&uniq_p);

    let (t_defs, t_main) = tag_prog(&defs, main.clone());

    let seq_p = tag_sprog(&seq_prog(&t_defs, &t_main));

    // panic!("{:#?}", seq_p);


    Ok(format!(
        "\
section .text
        global start_here
        extern snake_error
        extern print_snake_val
start_here:
{}
",
        instrs_to_string(&compile_to_instrs(&seq_p))
    ))
}