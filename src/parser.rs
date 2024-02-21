// auto-generated: "lalrpop 0.19.8"
// sha3: aa5bdd41184737f2c0377cc3b3937308ed06f345a9108ed0ddc4ffa9f567b0c1
use std::str::FromStr;
use crate::syntax::{FunDecl, SurfProg, SurfFunDecl, Exp, Prim};
use crate::span::Span1;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Exp {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use std::str::FromStr;
    use crate::syntax::{FunDecl, SurfProg, SurfFunDecl, Exp, Prim};
    use crate::span::Span1;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Exp<Span1>),
        Variant2(alloc::vec::Vec<Exp<Span1>>),
        Variant3(String),
        Variant4(alloc::vec::Vec<String>),
        Variant5((SurfFunDecl<Span1>, &'input str)),
        Variant6(alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>),
        Variant7(usize),
        Variant8(Vec<(String, Exp<Span1>)>),
        Variant9(bool),
        Variant10(Box<Exp<Span1>>),
        Variant11(Prim),
        Variant12(Vec<Exp<Span1>>),
        Variant13(Vec<String>),
        Variant14(core::option::Option<Exp<Span1>>),
        Variant15(SurfFunDecl<Span1>),
        Variant16(core::option::Option<String>),
        Variant17(i64),
        Variant18(SurfProg<Span1>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 56, 0, -81, 0, 0, -81, 0, -81, 0, 0, 0, 0, 0, 0, 0, -81, 0, -81, 0, 0, -81, 0, 0, 0, 0, 0, 0, 57, 0, 0,
        // State 3
        0, 59, -36, 0, -36, 0, 0, -36, 0, -36, 60, 61, 0, 62, 63, 64, 0, -36, 0, -36, 0, 0, -36, 0, 0, 0, 0, 0, 0, -36, 0, 0,
        // State 4
        0, -82, -82, 0, -82, 0, 65, -82, 66, -82, -82, -82, 0, -82, -82, -82, 0, -82, 0, -82, 0, 0, -82, 0, 0, 0, 0, 0, 0, -82, 0, 0,
        // State 5
        0, -78, -78, 0, -78, 67, -78, -78, -78, -78, -78, -78, 0, -78, -78, -78, 0, -78, 0, -78, 0, 0, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0,
        // State 6
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 48, 49, 0, 50, 51, 52, 0, 53, 54,
        // State 7
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 9
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 11
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 48, 49, 0, 50, 51, 52, 0, 53, 54,
        // State 12
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 13
        7, 0, 0, 8, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 14
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 48, 49, 0, 50, 51, 52, 0, 53, 54,
        // State 15
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 48, 49, 0, 50, 51, 52, 0, 53, 54,
        // State 16
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 47, 0, 0, 48, 49, 0, 50, 51, 52, 0, 53, 54,
        // State 17
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 18
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 19
        7, 0, 0, 8, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 20
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 21
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 23
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 24
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 25
        0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54,
        // State 26
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 27
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 28
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 9, 0, 47, 10, 0, 48, 49, 11, 50, 51, 52, 0, 53, 54,
        // State 29
        0, -69, -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, 0, -69, 0, -69, 0, 0, -69, 0, 0, 0, 0, 0, 0, -69, 0, 0,
        // State 30
        0, 0, 0, 0, -48, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, -48, 0, -48, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -19, -19, 0, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19, -19, -19, 0, -19, 0, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0, -19, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, -47, 0, 0, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, -47, 0, -47, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, -45, 0, 0, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, -45, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -18, -18, 14, -18, -18, -18, -18, -18, -18, -18, -18, 0, -18, -18, -18, 0, -18, 0, -18, 0, 0, -18, 0, 0, 0, 0, 0, 0, -18, 0, 0,
        // State 37
        0, 0, 0, 0, -46, 0, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, -46, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, -24, 0, 0, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, -24, 0, -24, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, -63, 0, -63, -63, -63, 0, -63, 0, -63, 0, 0, -63, 0, 0, 0, 0, 0, 0, -63, 0, 0,
        // State 40
        0, -20, -20, 0, -20, -20, -20, -20, -20, -20, -20, -20, 0, -20, -20, -20, 0, -20, 0, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0,
        // State 41
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -61, -61, 0, -61, 0, -61, -61, -61, -61, -61, -61, 0, -61, -61, -61, 0, -61, 0, -61, 0, 0, -61, 0, 0, 0, 0, 0, 0, -61, 0, 0,
        // State 43
        0, 0, 0, 0, -67, 0, 0, -67, 0, -67, 0, 0, 0, 0, 0, 0, 0, -67, 0, -67, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, -59, -59, 0, -59, 0, 0, -59, 0, -59, -59, -59, 0, -59, -59, -59, 0, -59, 0, -59, 0, 0, -59, 0, 0, 0, 0, 0, 0, -59, 0, 0,
        // State 45
        0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, 0, -28, -28, -28, 0, -28, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, -28, 0, 0,
        // State 47
        0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, 0, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, 0, -27, 0, 0,
        // State 52
        0, -70, -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, 0, -70, 0, -70, 0, 0, -70, 0, 0, 0, 0, 0, 0, -70, 0, 0,
        // State 53
        0, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, 0, 0, 0, -55, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        -65, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, -65, 0, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65,
        // State 56
        -66, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, -66, 0, 0, -66, -66, 0, -66, -66, -66, 0, -66, -66,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        -35, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35,
        // State 59
        -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, 0, 0, -30, -30, 0, -30, -30, -30, 0, -30, -30,
        // State 60
        -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, 0, 0, -31, -31, 0, -31, -31, -31, 0, -31, -31,
        // State 61
        -34, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, -34, 0, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34,
        // State 62
        -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, 0, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32,
        // State 63
        -33, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, 0, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33,
        // State 64
        -71, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, -71, 0, 0, -71, -71, 0, -71, -71, -71, 0, -71, -71,
        // State 65
        -72, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, -72, 0, 0, -72, -72, 0, -72, -72, -72, 0, -72, -72,
        // State 66
        -83, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, -83, 0, 0, -83, -83, 0, -83, -83, -83, 0, -83, -83,
        // State 67
        0, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, 0, -68, 0, -68, 0, 0, -68, 0, 0, 0, 0, 0, 0, -68, 0, 0,
        // State 68
        0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, -80, 0, 0, -80, 0, -80, 0, 0, 0, 0, 0, 0, 0, -80, 0, -80, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, -51, 0, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, -51, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, -29, 0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, -29, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, -37, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, -58, -58, 0, -58, 0, 0, -58, 0, -58, -58, -58, 0, -58, -58, -58, 0, -58, 0, -58, 0, 0, -58, 0, 0, 0, 0, 0, 0, -58, 0, 0,
        // State 80
        0, -60, -60, 0, -60, 0, -60, -60, -60, -60, -60, -60, 0, -60, -60, -60, 0, -60, 0, -60, 0, 0, -60, 0, 0, 0, 0, 0, 0, -60, 0, 0,
        // State 81
        0, -62, -62, 0, -62, -62, -62, -62, -62, -62, -62, -62, 0, -62, -62, -62, 0, -62, 0, -62, 0, 0, -62, 0, 0, 0, 0, 0, 0, -62, 0, 0,
        // State 82
        0, 0, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, -22, -22, 0, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, -22, -22, 0, -22, 0, -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0,
        // State 84
        0, 0, 0, 0, -52, 0, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, -39, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, 0, -23, 0, -23, 0, 0, -23, 0, 0, 0, 0, 0, 0, -23, 0, 0,
        // State 87
        -4, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, -4, 0, -4, -4, 0, -4, -4, -4, -4, -4, -4, 0, -4, -4,
        // State 88
        0, -21, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21, 0, -21, -21, -21, 0, -21, 0, -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0,
        // State 89
        0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, -41, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, -64, 0, 0, -64, 0, -64, 0, 0, 0, 0, 0, 0, 0, -64, 0, -64, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        -5, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, -5, 0, -5, -5, 0, -5, -5, -5, -5, -5, -5, 0, -5, -5,
        // State 96
        0, 0, 0, 0, -43, 0, 0, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10,
        // State 101
        0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, -54, 0, 0, -54, 0, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 32 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -81,
        // State 3
        -36,
        // State 4
        -82,
        // State 5
        -78,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -69,
        // State 30
        -48,
        // State 31
        -19,
        // State 32
        -84,
        // State 33
        -47,
        // State 34
        0,
        // State 35
        -45,
        // State 36
        -18,
        // State 37
        -46,
        // State 38
        -24,
        // State 39
        -63,
        // State 40
        -20,
        // State 41
        0,
        // State 42
        -61,
        // State 43
        -67,
        // State 44
        -59,
        // State 45
        0,
        // State 46
        -28,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -27,
        // State 52
        -70,
        // State 53
        -55,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        -68,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        -80,
        // State 75
        -51,
        // State 76
        -29,
        // State 77
        0,
        // State 78
        0,
        // State 79
        -58,
        // State 80
        -60,
        // State 81
        -62,
        // State 82
        0,
        // State 83
        -22,
        // State 84
        -52,
        // State 85
        0,
        // State 86
        -23,
        // State 87
        0,
        // State 88
        -21,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        -64,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        -54,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 19,
            5 => 25,
            8 => 1,
            11 => match state {
                6 => 67,
                _ => 29,
            },
            12 => 30,
            13 => 71,
            14 => 31,
            15 => match state {
                18 => 84,
                _ => 75,
            },
            16 => 14,
            17 => 2,
            18 => 77,
            19 => 89,
            20 => match state {
                0 => 32,
                7 => 68,
                9 => 70,
                13 => 78,
                17 => 82,
                19 => 85,
                21 => 91,
                23 => 93,
                24 => 94,
                26 => 101,
                27 => 102,
                28 => 103,
                _ => 76,
            },
            22 => 33,
            23 => match state {
                1 => 54,
                _ => 34,
            },
            24 => 35,
            25 => match state {
                8 => 69,
                10 => 72,
                20 => 90,
                22 => 92,
                25 => 96,
                _ => 36,
            },
            27 => 3,
            28 => 4,
            29 => 5,
            30 => 37,
            31 => 11,
            32 => 38,
            33 => match state {
                16 => 81,
                _ => 39,
            },
            34 => 40,
            35 => 15,
            36 => 41,
            37 => match state {
                15 => 80,
                _ => 42,
            },
            39 => match state {
                11 => 74,
                _ => 43,
            },
            40 => match state {
                14 => 79,
                _ => 44,
            },
            41 => 16,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""add1""###,
            r###""and""###,
            r###""def""###,
            r###""else""###,
            r###""false""###,
            r###""if""###,
            r###""in""###,
            r###""isbool""###,
            r###""isnum""###,
            r###""let""###,
            r###""print""###,
            r###""sub1""###,
            r###""true""###,
            r###""||""###,
            r###"r#"[+-]?[0-9]+"#"###,
            r###"r#"[a-zA-Z][a-zA-Z0-9_]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Exp<Span1>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 32 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(23, _) if true => Some(21),
            Token(24, _) if true => Some(22),
            Token(25, _) if true => Some(23),
            Token(26, _) if true => Some(24),
            Token(27, _) if true => Some(25),
            Token(28, _) if true => Some(26),
            Token(29, _) if true => Some(27),
            Token(30, _) if true => Some(28),
            Token(31, _) if true => Some(29),
            Token(0, _) if true => Some(30),
            Token(1, _) if true => Some(31),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExpParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExpParser {
        pub fn new() -> ExpParser {
            let __builder = super::__intern_token::new_builder();
            ExpParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Exp<Span1>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Exp<Span1>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                // __Exp = Exp => ActionFn(1);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            84 => {
                __reduce84(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (SurfFunDecl<Span1>, &'input str), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Exp<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Prim, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SurfFunDecl<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SurfProg<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(String, Exp<Span1>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",") = Exp, "," => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")* =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")* = (<Exp> ",")+ => ActionFn(72);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")+ = Exp, "," => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")+ = (<Exp> ",")+, Exp, "," => ActionFn(79);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",") = Id, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")* =  => ActionFn(66);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action66::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")* = (<Id> ",")+ => ActionFn(67);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")+ = Id, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")+ = (<Id> ",")+, Id, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and") = FunDecl, "and" => ActionFn(60);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action60::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")* =  => ActionFn(58);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action58::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")* = (FunDecl "and")+ => ActionFn(59);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")+ = FunDecl, "and" => ActionFn(86);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action86::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")+ = (FunDecl "and")+, FunDecl, "and" => ActionFn(87);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action87::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action61::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Id => ActionFn(105);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Bool => ActionFn(106);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Num => ActionFn(107);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action107::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Prim1, "(", Exp, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = "(", Exp, ")" => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Id, "(", Comma<Exp>, ")" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOps = LogExp => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bindings = Id, "=", Exp => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bindings = Bindings, ",", Id, "=", Exp => ActionFn(41);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (5, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Boxed<Exp> = Exp => ActionFn(57);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "<" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "<=" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = ">" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = ">=" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "==" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "!=" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CmpExp = LAssoc<Cmp, Sum> => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = Exp => ActionFn(120);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> =  => ActionFn(121);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action121::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = (<Exp> ",")+, Exp => ActionFn(122);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action122::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = (<Exp> ",")+ => ActionFn(123);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action123::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = Id => ActionFn(124);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action124::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = (<Id> ",")+, Id => ActionFn(126);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action126::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = (<Id> ",")+ => ActionFn(127);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = IF => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = LET => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = FUNDEFS => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = BinOps => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp? = Exp => ActionFn(69);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp? =  => ActionFn(70);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action70::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FUNDEFS = FunDecl, "in", Boxed<Exp> => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FUNDEFS = (FunDecl "and")+, FunDecl, "in", Boxed<Exp> => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 22)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunDecl = "def", Id, "(", Comma<Id>, ")", ":", Exp => ActionFn(112);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant13(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action112::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IF = "if", Exp, ":", Exp, "else", ":", Exp => ActionFn(113);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action113::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (7, 24)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id? = Id => ActionFn(64);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id? =  => ActionFn(65);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action65::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Cmp, Sum> = LAssoc<Cmp, Sum>, Cmp, Sum => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Cmp, Sum> = Sum => ActionFn(52);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<PlusMinus, Product> = LAssoc<PlusMinus, Product>, PlusMinus, Product => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 28)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<PlusMinus, Product> = Product => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Times, NotExp> = LAssoc<Times, NotExp>, Times, NotExp => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 29)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Times, NotExp> = NotExp => ActionFn(48);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LET = "let", Bindings, "in", Exp => ActionFn(117);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action117::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 30)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Log = "&&" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Log = "||" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogExp = RAssoc<Log, CmpExp> => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NotExp = "!", BaseExp => ActionFn(118);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action118::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NotExp = BaseExp => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[+-]?[0-9]+"# => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PlusMinus = "+" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PlusMinus = "-" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "add1" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "sub1" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "print" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "isnum" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "isbool" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Product = LAssoc<Times, NotExp> => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prog = Exp => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RAssoc<Log, CmpExp> = CmpExp, Log, RAssoc<Log, CmpExp> => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 39)
    }
    pub(crate) fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RAssoc<Log, CmpExp> = CmpExp => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Sum = LAssoc<PlusMinus, Product> => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Times = "*" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce84<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __Prog = Prog => ActionFn(0);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
}
pub use self::__parse__Exp::ExpParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Prog {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use std::str::FromStr;
    use crate::syntax::{FunDecl, SurfProg, SurfFunDecl, Exp, Prim};
    use crate::span::Span1;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Exp<Span1>),
        Variant2(alloc::vec::Vec<Exp<Span1>>),
        Variant3(String),
        Variant4(alloc::vec::Vec<String>),
        Variant5((SurfFunDecl<Span1>, &'input str)),
        Variant6(alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>),
        Variant7(usize),
        Variant8(Vec<(String, Exp<Span1>)>),
        Variant9(bool),
        Variant10(Box<Exp<Span1>>),
        Variant11(Prim),
        Variant12(Vec<Exp<Span1>>),
        Variant13(Vec<String>),
        Variant14(core::option::Option<Exp<Span1>>),
        Variant15(SurfFunDecl<Span1>),
        Variant16(core::option::Option<String>),
        Variant17(i64),
        Variant18(SurfProg<Span1>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 57, 0, -81, 0, 0, -81, 0, -81, 0, 0, 0, 0, 0, 0, 0, -81, 0, -81, 0, 0, -81, 0, 0, 0, 0, 0, 0, 58, 0, 0,
        // State 3
        0, 60, -36, 0, -36, 0, 0, -36, 0, -36, 61, 62, 0, 63, 64, 65, 0, -36, 0, -36, 0, 0, -36, 0, 0, 0, 0, 0, 0, -36, 0, 0,
        // State 4
        0, -82, -82, 0, -82, 0, 66, -82, 67, -82, -82, -82, 0, -82, -82, -82, 0, -82, 0, -82, 0, 0, -82, 0, 0, 0, 0, 0, 0, -82, 0, 0,
        // State 5
        0, -78, -78, 0, -78, 68, -78, -78, -78, -78, -78, -78, 0, -78, -78, -78, 0, -78, 0, -78, 0, 0, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0,
        // State 6
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 49, 50, 0, 51, 52, 53, 0, 54, 55,
        // State 7
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
        // State 9
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
        // State 11
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 49, 50, 0, 51, 52, 53, 0, 54, 55,
        // State 12
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 13
        7, 0, 0, 8, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 14
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 49, 50, 0, 51, 52, 53, 0, 54, 55,
        // State 15
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 49, 50, 0, 51, 52, 53, 0, 54, 55,
        // State 16
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 48, 0, 0, 49, 50, 0, 51, 52, 53, 0, 54, 55,
        // State 17
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 18
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 19
        7, 0, 0, 8, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 20
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
        // State 21
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
        // State 23
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 24
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 25
        0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55,
        // State 26
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 27
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 28
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 9, 0, 48, 10, 0, 49, 50, 11, 51, 52, 53, 0, 54, 55,
        // State 29
        0, -69, -69, 0, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, 0, -69, 0, -69, 0, 0, -69, 0, 0, 0, 0, 0, 0, -69, 0, 0,
        // State 30
        0, 0, 0, 0, -48, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, -48, 0, -48, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -19, -19, 0, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19, -19, -19, 0, -19, 0, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0, -19, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, -47, 0, 0, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, -47, 0, -47, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, -45, 0, 0, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, -45, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -18, -18, 14, -18, -18, -18, -18, -18, -18, -18, -18, 0, -18, -18, -18, 0, -18, 0, -18, 0, 0, -18, 0, 0, 0, 0, 0, 0, -18, 0, 0,
        // State 37
        0, 0, 0, 0, -46, 0, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, -46, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, -24, 0, 0, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, -24, 0, -24, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, -63, 0, -63, -63, -63, 0, -63, 0, -63, 0, 0, -63, 0, 0, 0, 0, 0, 0, -63, 0, 0,
        // State 40
        0, -20, -20, 0, -20, -20, -20, -20, -20, -20, -20, -20, 0, -20, -20, -20, 0, -20, 0, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0,
        // State 41
        0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -61, -61, 0, -61, 0, -61, -61, -61, -61, -61, -61, 0, -61, -61, -61, 0, -61, 0, -61, 0, 0, -61, 0, 0, 0, 0, 0, 0, -61, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -67, 0, 0, -67, 0, -67, 0, 0, 0, 0, 0, 0, 0, -67, 0, -67, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -59, -59, 0, -59, 0, 0, -59, 0, -59, -59, -59, 0, -59, -59, -59, 0, -59, 0, -59, 0, 0, -59, 0, 0, 0, 0, 0, 0, -59, 0, 0,
        // State 46
        0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, -28, 0, -28, -28, -28, 0, -28, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, -28, 0, 0,
        // State 48
        0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, 0, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, 0, -27, 0, 0,
        // State 53
        0, -70, -70, 0, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, 0, -70, 0, -70, 0, 0, -70, 0, 0, 0, 0, 0, 0, -70, 0, 0,
        // State 54
        0, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, -55, 0, -55, 0, -55, 0, 0, -55, 0, 0, 0, 0, 0, 0, -55, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        -65, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, -65, 0, 0, -65, -65, 0, -65, -65, -65, 0, -65, -65,
        // State 57
        -66, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, -66, 0, 0, -66, -66, 0, -66, -66, -66, 0, -66, -66,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        -35, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, -35, -35, 0, -35, -35, -35, 0, -35, -35,
        // State 60
        -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, 0, 0, -30, -30, 0, -30, -30, -30, 0, -30, -30,
        // State 61
        -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, 0, 0, -31, -31, 0, -31, -31, -31, 0, -31, -31,
        // State 62
        -34, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, -34, 0, 0, -34, -34, 0, -34, -34, -34, 0, -34, -34,
        // State 63
        -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, 0, 0, -32, -32, 0, -32, -32, -32, 0, -32, -32,
        // State 64
        -33, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, 0, 0, -33, -33, 0, -33, -33, -33, 0, -33, -33,
        // State 65
        -71, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, -71, 0, 0, -71, -71, 0, -71, -71, -71, 0, -71, -71,
        // State 66
        -72, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, -72, 0, 0, -72, -72, 0, -72, -72, -72, 0, -72, -72,
        // State 67
        -83, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, -83, 0, 0, -83, -83, 0, -83, -83, -83, 0, -83, -83,
        // State 68
        0, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, -68, 0, -68, -68, -68, 0, -68, 0, -68, 0, 0, -68, 0, 0, 0, 0, 0, 0, -68, 0, 0,
        // State 69
        0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, -80, 0, 0, -80, 0, -80, 0, 0, 0, 0, 0, 0, 0, -80, 0, -80, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, -51, 0, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, -51, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, -29, 0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, -29, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, -37, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, -58, -58, 0, -58, 0, 0, -58, 0, -58, -58, -58, 0, -58, -58, -58, 0, -58, 0, -58, 0, 0, -58, 0, 0, 0, 0, 0, 0, -58, 0, 0,
        // State 81
        0, -60, -60, 0, -60, 0, -60, -60, -60, -60, -60, -60, 0, -60, -60, -60, 0, -60, 0, -60, 0, 0, -60, 0, 0, 0, 0, 0, 0, -60, 0, 0,
        // State 82
        0, -62, -62, 0, -62, -62, -62, -62, -62, -62, -62, -62, 0, -62, -62, -62, 0, -62, 0, -62, 0, 0, -62, 0, 0, 0, 0, 0, 0, -62, 0, 0,
        // State 83
        0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, -22, -22, 0, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, -22, -22, 0, -22, 0, -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0,
        // State 85
        0, 0, 0, 0, -52, 0, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, -39, 0, 0, 97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, 0, -23, 0, -23, 0, 0, -23, 0, 0, 0, 0, 0, 0, -23, 0, 0,
        // State 88
        -4, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, -4, 0, -4, -4, 0, -4, -4, -4, -4, -4, -4, 0, -4, -4,
        // State 89
        0, -21, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21, 0, -21, -21, -21, 0, -21, 0, -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0,
        // State 90
        0, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, -41, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, -64, 0, 0, -64, 0, -64, 0, 0, 0, 0, 0, 0, 0, -64, 0, -64, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        -5, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, -5, 0, -5, -5, 0, -5, -5, -5, -5, -5, -5, 0, -5, -5,
        // State 97
        0, 0, 0, 0, -43, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10,
        // State 102
        0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, -54, 0, 0, -54, 0, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, -54, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 32 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -81,
        // State 3
        -36,
        // State 4
        -82,
        // State 5
        -78,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -69,
        // State 30
        -48,
        // State 31
        -19,
        // State 32
        -79,
        // State 33
        -47,
        // State 34
        0,
        // State 35
        -45,
        // State 36
        -18,
        // State 37
        -46,
        // State 38
        -24,
        // State 39
        -63,
        // State 40
        -20,
        // State 41
        0,
        // State 42
        -61,
        // State 43
        -85,
        // State 44
        -67,
        // State 45
        -59,
        // State 46
        0,
        // State 47
        -28,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        -27,
        // State 53
        -70,
        // State 54
        -55,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -68,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        -80,
        // State 76
        -51,
        // State 77
        -29,
        // State 78
        0,
        // State 79
        0,
        // State 80
        -58,
        // State 81
        -60,
        // State 82
        -62,
        // State 83
        0,
        // State 84
        -22,
        // State 85
        -52,
        // State 86
        0,
        // State 87
        -23,
        // State 88
        0,
        // State 89
        -21,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        -64,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        -54,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 19,
            5 => 25,
            8 => 1,
            11 => match state {
                6 => 68,
                _ => 29,
            },
            12 => 30,
            13 => 72,
            14 => 31,
            15 => match state {
                18 => 85,
                _ => 76,
            },
            16 => 14,
            17 => 2,
            18 => 78,
            19 => 90,
            20 => match state {
                0 => 32,
                7 => 69,
                9 => 71,
                13 => 79,
                17 => 83,
                19 => 86,
                21 => 92,
                23 => 94,
                24 => 95,
                26 => 102,
                27 => 103,
                28 => 104,
                _ => 77,
            },
            22 => 33,
            23 => match state {
                1 => 55,
                _ => 34,
            },
            24 => 35,
            25 => match state {
                8 => 70,
                10 => 73,
                20 => 91,
                22 => 93,
                25 => 97,
                _ => 36,
            },
            27 => 3,
            28 => 4,
            29 => 5,
            30 => 37,
            31 => 11,
            32 => 38,
            33 => match state {
                16 => 82,
                _ => 39,
            },
            34 => 40,
            35 => 15,
            36 => 41,
            37 => match state {
                15 => 81,
                _ => 42,
            },
            38 => 43,
            39 => match state {
                11 => 75,
                _ => 44,
            },
            40 => match state {
                14 => 80,
                _ => 45,
            },
            41 => 16,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""add1""###,
            r###""and""###,
            r###""def""###,
            r###""else""###,
            r###""false""###,
            r###""if""###,
            r###""in""###,
            r###""isbool""###,
            r###""isnum""###,
            r###""let""###,
            r###""print""###,
            r###""sub1""###,
            r###""true""###,
            r###""||""###,
            r###"r#"[+-]?[0-9]+"#"###,
            r###"r#"[a-zA-Z][a-zA-Z0-9_]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = SurfProg<Span1>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 32 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(23, _) if true => Some(21),
            Token(24, _) if true => Some(22),
            Token(25, _) if true => Some(23),
            Token(26, _) if true => Some(24),
            Token(27, _) if true => Some(25),
            Token(28, _) if true => Some(26),
            Token(29, _) if true => Some(27),
            Token(30, _) if true => Some(28),
            Token(31, _) if true => Some(29),
            Token(0, _) if true => Some(30),
            Token(1, _) if true => Some(31),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgParser {
        pub fn new() -> ProgParser {
            let __builder = super::__intern_token::new_builder();
            ProgParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<SurfProg<Span1>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<SurfProg<Span1>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                // __Prog = Prog => ActionFn(0);
                let __sym0 = __pop_Variant18(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (SurfFunDecl<Span1>, &'input str), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Exp<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Prim, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SurfFunDecl<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SurfProg<Span1>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(String, Exp<Span1>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Exp<Span1>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",") = Exp, "," => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")* =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")* = (<Exp> ",")+ => ActionFn(72);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")+ = Exp, "," => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Exp> ",")+ = (<Exp> ",")+, Exp, "," => ActionFn(79);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action79::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",") = Id, "," => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")* =  => ActionFn(66);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action66::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")* = (<Id> ",")+ => ActionFn(67);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")+ = Id, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Id> ",")+ = (<Id> ",")+, Id, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and") = FunDecl, "and" => ActionFn(60);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action60::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")* =  => ActionFn(58);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action58::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")* = (FunDecl "and")+ => ActionFn(59);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action59::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")+ = FunDecl, "and" => ActionFn(86);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action86::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (FunDecl "and")+ = (FunDecl "and")+, FunDecl, "and" => ActionFn(87);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action87::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action61::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(56);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action56::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Id => ActionFn(105);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action105::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Bool => ActionFn(106);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action106::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Num => ActionFn(107);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action107::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Prim1, "(", Exp, ")" => ActionFn(108);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action108::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = "(", Exp, ")" => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BaseExp = Id, "(", Comma<Exp>, ")" => ActionFn(109);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action109::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BinOps = LogExp => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bindings = Id, "=", Exp => ActionFn(40);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bindings = Bindings, ",", Id, "=", Exp => ActionFn(41);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (5, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Boxed<Exp> = Exp => ActionFn(57);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "<" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "<=" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = ">" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = ">=" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "==" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Cmp = "!=" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CmpExp = LAssoc<Cmp, Sum> => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = Exp => ActionFn(120);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> =  => ActionFn(121);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action121::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = (<Exp> ",")+, Exp => ActionFn(122);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action122::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Exp> = (<Exp> ",")+ => ActionFn(123);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action123::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = Id => ActionFn(124);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action124::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = (<Id> ",")+, Id => ActionFn(126);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action126::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Id> = (<Id> ",")+ => ActionFn(127);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action127::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = IF => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = LET => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = FUNDEFS => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp = BinOps => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp? = Exp => ActionFn(69);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Exp? =  => ActionFn(70);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action70::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FUNDEFS = FunDecl, "in", Boxed<Exp> => ActionFn(110);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action110::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 22)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FUNDEFS = (FunDecl "and")+, FunDecl, "in", Boxed<Exp> => ActionFn(111);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action111::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 22)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunDecl = "def", Id, "(", Comma<Id>, ")", ":", Exp => ActionFn(112);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant13(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action112::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IF = "if", Exp, ":", Exp, "else", ":", Exp => ActionFn(113);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action113::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (7, 24)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id? = Id => ActionFn(64);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Id? =  => ActionFn(65);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action65::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Cmp, Sum> = LAssoc<Cmp, Sum>, Cmp, Sum => ActionFn(114);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action114::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Cmp, Sum> = Sum => ActionFn(52);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<PlusMinus, Product> = LAssoc<PlusMinus, Product>, PlusMinus, Product => ActionFn(115);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action115::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 28)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<PlusMinus, Product> = Product => ActionFn(50);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Times, NotExp> = LAssoc<Times, NotExp>, Times, NotExp => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action116::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 29)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LAssoc<Times, NotExp> = NotExp => ActionFn(48);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LET = "let", Bindings, "in", Exp => ActionFn(117);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action117::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 30)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Log = "&&" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Log = "||" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogExp = RAssoc<Log, CmpExp> => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NotExp = "!", BaseExp => ActionFn(118);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action118::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NotExp = BaseExp => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[+-]?[0-9]+"# => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PlusMinus = "+" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PlusMinus = "-" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "add1" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "sub1" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "print" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "isnum" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prim1 = "isbool" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Product = LAssoc<Times, NotExp> => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prog = Exp => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RAssoc<Log, CmpExp> = CmpExp, Log, RAssoc<Log, CmpExp> => ActionFn(119);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action119::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 39)
    }
    pub(crate) fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // RAssoc<Log, CmpExp> = CmpExp => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Sum = LAssoc<PlusMinus, Product> => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Times = "*" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce83<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __Exp = Exp => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 42)
    }
}
pub use self::__parse__Prog::ProgParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::syntax::{FunDecl, SurfProg, SurfFunDecl, Exp, Prim};
    use crate::span::Span1;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([\\+\\-]?[0-9]+)", false),
            ("^([A-Za-z][0-9A-Z_a-z]*)", false),
            ("^(!)", false),
            ("^(!=)", false),
            ("^(\\&\\&)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(,)", false),
            ("^(\\-)", false),
            ("^(:)", false),
            ("^(<)", false),
            ("^(<=)", false),
            ("^(=)", false),
            ("^(==)", false),
            ("^(>)", false),
            ("^(>=)", false),
            ("^(add1)", false),
            ("^(and)", false),
            ("^(def)", false),
            ("^(else)", false),
            ("^(false)", false),
            ("^(if)", false),
            ("^(in)", false),
            ("^(isbool)", false),
            ("^(isnum)", false),
            ("^(let)", false),
            ("^(print)", false),
            ("^(sub1)", false),
            ("^(true)", false),
            ("^(\\|\\|)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SurfProg<Span1>, usize),
) -> SurfProg<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> SurfProg<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, funs, _): (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    (_, last, _): (usize, SurfFunDecl<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Box<Exp<Span1>>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    {
      let mut decls: Vec<SurfFunDecl<Span1>> = funs.into_iter().map(|(decl, _)| decl).collect();
      decls.push(last);
      Exp::FunDefs { decls, body, ann: Span1 { start_ix: l, end_ix: r} }
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, parameters, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> SurfFunDecl<Span1>
{
    FunDecl { name, parameters, body, ann: Span1 { start_ix: l, end_ix: r}  }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, cond, _): (usize, Exp<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, thn, _): (usize, Exp<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, els, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::If { cond: Box::new(cond), thn: Box::new(thn), els: Box::new(els), ann: Span1 { start_ix: l, end_ix: r }}
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, bindings, _): (usize, Vec<(String, Exp<Span1>)>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Let { bindings, body: Box::new(body), ann: Span1 { start_ix: l, end_ix: r }}
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Add
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Sub
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Mul
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::And
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Or
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Lt
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Le
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Gt
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Ge
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Eq
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Neq
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(Prim::Not, vec![Box::new(e)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Var(x, Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, b, _): (usize, bool, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Bool(b, Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, n, _): (usize, i64, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Num(n, Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, p, _): (usize, Prim, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(p, vec![Box::new(e)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Exp<Span1>
{
    e
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, es, _): (usize, Vec<Exp<Span1>>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Call(x, es, Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Add1
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Sub1
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::Print
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::IsNum
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Prim
{
    Prim::IsBool
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, x, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
) -> Vec<(String, Exp<Span1>)>
{
    vec![(x, e)]
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, mut bs, _): (usize, Vec<(String, Exp<Span1>)>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, x, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
) -> Vec<(String, Exp<Span1>)>
{
    {
        bs.push((x, e));
        bs
    }
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i64
{
    i64::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    String::from(s)
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    false
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Exp<Span1>>, usize),
    (_, last, _): (usize, core::option::Option<Exp<Span1>>, usize),
) -> Vec<Exp<Span1>>
{
    {
        match last {
            None => { },
            Some(t) => { v.push(t); }
        };
        v
    }
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, e1, _): (usize, Exp<Span1>, usize),
    (_, op, _): (usize, Prim, usize),
    (_, e2, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(op, vec![Box::new(e1), Box::new(e2)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, e1, _): (usize, Exp<Span1>, usize),
    (_, op, _): (usize, Prim, usize),
    (_, e2, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(op, vec![Box::new(e1), Box::new(e2)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, e1, _): (usize, Exp<Span1>, usize),
    (_, op, _): (usize, Prim, usize),
    (_, e2, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(op, vec![Box::new(e1), Box::new(e2)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, e1, _): (usize, Exp<Span1>, usize),
    (_, op, _): (usize, Prim, usize),
    (_, e2, _): (usize, Exp<Span1>, usize),
    (_, r, _): (usize, usize, usize),
) -> Exp<Span1>
{
    Exp::Prim(op, vec![Box::new(e1), Box::new(e2)], Span1 { start_ix: l, end_ix: r })
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, last, _): (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    {
        match last {
            None => { },
            Some(t) => { v.push(t); }
        };
        v
    }
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Exp<Span1>, usize),
) -> Box<Exp<Span1>>
{
    Box::new(t)
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    v
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SurfFunDecl<Span1>, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (SurfFunDecl<Span1>, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (SurfFunDecl<Span1>, &'input str), usize),
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    (_, e, _): (usize, (SurfFunDecl<Span1>, &'input str), usize),
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> core::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<String>
{
    None
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> core::option::Option<Exp<Span1>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Exp<Span1>>
{
    None
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Exp<Span1>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Exp<Span1>>, usize),
) -> alloc::vec::Vec<Exp<Span1>>
{
    v
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Exp<Span1>
{
    __0
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Exp<Span1>, usize),
) -> alloc::vec::Vec<Exp<Span1>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Exp<Span1>>, usize),
    (_, e, _): (usize, Exp<Span1>, usize),
) -> alloc::vec::Vec<Exp<Span1>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Exp<Span1>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action73(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Exp<Span1>>, usize),
    __1: (usize, Exp<Span1>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Exp<Span1>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action73(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Exp<Span1>>, usize),
) -> Vec<Exp<Span1>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action71(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Exp<Span1>>, usize),
    __1: (usize, core::option::Option<Exp<Span1>>, usize),
) -> Vec<Exp<Span1>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action72(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action68(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action68(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action66(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action67(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
>(
    input: &'input str,
    __0: (usize, SurfFunDecl<Span1>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action60(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action87<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    __1: (usize, SurfFunDecl<Span1>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action60(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
>(
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, SurfFunDecl<Span1>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Box<Exp<Span1>>, usize),
    __4: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action58(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
>(
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    __2: (usize, SurfFunDecl<Span1>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Box<Exp<Span1>>, usize),
    __5: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action59(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __temp0,
        __2,
        __3,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
>(
    input: &'input str,
    __0: (usize, bool, usize),
    __1: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action92<
    'input,
>(
    input: &'input str,
    __0: (usize, i64, usize),
    __1: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action93<
    'input,
>(
    input: &'input str,
    __0: (usize, Prim, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action94<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Exp<Span1>>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action95<
    'input,
>(
    input: &'input str,
    __0: (usize, SurfFunDecl<Span1>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Box<Exp<Span1>>, usize),
    __3: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action96<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    __1: (usize, SurfFunDecl<Span1>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Box<Exp<Span1>>, usize),
    __4: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action97<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Exp<Span1>, usize),
    __7: (usize, usize, usize),
) -> SurfFunDecl<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action98<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Exp<Span1>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Exp<Span1>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Exp<Span1>, usize),
    __7: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
fn __action99<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action100<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<(String, Exp<Span1>)>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Exp<Span1>, usize),
    __4: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action103<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Exp<Span1>, usize),
    __2: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action104<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, usize, usize),
) -> Exp<Span1>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action105<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Exp<Span1>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action106<
    'input,
>(
    input: &'input str,
    __0: (usize, bool, usize),
) -> Exp<Span1>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'input,
>(
    input: &'input str,
    __0: (usize, i64, usize),
) -> Exp<Span1>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'input,
>(
    input: &'input str,
    __0: (usize, Prim, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Exp<Span1>, usize),
    __3: (usize, &'input str, usize),
) -> Exp<Span1>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<Exp<Span1>>, usize),
    __3: (usize, &'input str, usize),
) -> Exp<Span1>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'input,
>(
    input: &'input str,
    __0: (usize, SurfFunDecl<Span1>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Box<Exp<Span1>>, usize),
) -> Exp<Span1>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<(SurfFunDecl<Span1>, &'input str)>, usize),
    __1: (usize, SurfFunDecl<Span1>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Box<Exp<Span1>>, usize),
) -> Exp<Span1>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Exp<Span1>, usize),
) -> SurfFunDecl<Span1>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action97(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Exp<Span1>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Exp<Span1>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<(String, Exp<Span1>)>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
    __1: (usize, Prim, usize),
    __2: (usize, Exp<Span1>, usize),
) -> Exp<Span1>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
    'input,
>(
    input: &'input str,
    __0: (usize, Exp<Span1>, usize),
) -> Vec<Exp<Span1>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action69(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Exp<Span1>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action70(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action122<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Exp<Span1>>, usize),
    __1: (usize, Exp<Span1>, usize),
) -> Vec<Exp<Span1>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action69(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action123<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Exp<Span1>>, usize),
) -> Vec<Exp<Span1>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action70(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action124<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action64(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action65(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action126<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action64(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action127<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action65(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
