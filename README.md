# Snake



## 1. Introduction

Snake is an advanced programming language that builds upon the foundation of Rust type system and lalrpop library, introducing enhanced features and capabilities. In addition to supporting floating point as a new data type, it also offers a comprehensive suite of auxiliary numerical operations, significantly broadening its computational scope and efficiency.



## 2. Concrete Syntax

The concrete syntax of Snake is very similar to Diamond, except now it accepts floating point literal as expressions. Snake also introduce division between numbers. 

\<exprs\>: ...

​	| *FLOAT NUMBER*

​	| \<expr\> `/` \<expr\>

### 2.1 Examples

```snake
# basic operations 
let pi = 3.1415926 in 
let a = 1.2 in 
pi + a 			# output is 4.3415926
```

```
# operations between int and float
let a = 3 in 
let b = 4.0 in 
let PRINT_1 = print(a + b) in  # 7.0
let PRINT_2 = print(a - b) in  # -1.0
let PRINT_3 = print(a * b) in  # 12.0
print(a / b) 									 # 0.75
```



## 3. Abstract Syntax and Semantics

### 3.1 AST

The only symbol update of Snake from Diamondback operation set is the division `/`.  However, `+`, `-`, `*` now supports both integers and floating point numbers. 

### 3.2 New Assembly Codes

Since x86 architecture performs floating number calculations using a novel set of registers and operations called x87 extension, it is necessary to add new registers and operations in `asm.rs`. 

#### 3.2.1 Registers

x87 store floating point numbers on an eight-slot register array, and access them as a stack, naming from `st0` to `st7`. Since the support calculation for Snake are all binary and we don't perform register allocation optimization, `st0` and `st1` are enough for the language.

```rust
pub enum Reg {
  // ... previous cases
  St0,
  St1,
}
```

#### 3.2.2 Operations

##### 3.2.2.1 Data Transfer

Since x86 forbids direct byte movements between common registers and floating point registers, it's necessary to have operations transferring bytes between memory and floating point registers:

- `fld qword [a] `  load fp from meory into st0
- `fild qword [a]` load an integer from memory into st0, and convert to 64-bt fp
- `fst qword [mem]|[st_i]` copy the value at st0 into [a memory location] | [a stack register]
- `fstp qword [mem]|[st_i]` pop the value at st0 into [a memory location] | [a stack register]

##### 3.2.2.2 Arithmetic 

Snake utilize x86 floating point arithmetic opeartions for calculation

Here are the updates in `asm.rs`:

```rust
pub enum Instr {
  ... // previous cases
  
  // mem to reg
  Fld(MovArgs),
  Fild(MovArgs),
  
  // reg to mem
  Fst(MovArgs),
  Fstp(MovArgs),
  
  // addition
  Fadd(BinArgs),
  Faddp(BinArgs),
  Fiadd(BinArgs),
  
  // subtraction
  Fsub(BinArgs),
  Fsubp(BinArgs),
  Fisub(BinArgs),
  
  // multiplication
  Fmul(BinArgs),
  Fmulp(BinArgs),
  Fimul(BinArgs),
  
  // division
  Fdiv(BinArgs),
  Fdivp(BinArgs),
  Fidiv(BinArgs),
  
}
```

### 3.3 Data representations

Snake supports three data types: **integers**, **booleans** and **floating point numbers**. Each type is represented in **64** bits. The last two bits for each data are reserved as tag bits. 

#### 3.3.1 floating pointing number

![Snake_fp](./pics/Snake_fp.jpeg)

The representation of floating point numbers in Snake ressembles the 64-bit floating point numbers in IEEE-754 standard, except the last two bits are saved for tagging. For printing, floating point numbers are depicted in scientific notation in command line. We choose `10` as last two tagging bits

#### 3.3.2 Integer

The only change in integer representation is that it requires two tagging bits. We choose `00` as tagging bits

- Range of Snake integers is $[-2^{61}, 2^{61}-1]$

#### 3.3.3 Boolean

Representation for booleans are identical to Diamondback, specifically: 

```rust
SNAKE_TRUE  = 0xFF_FF_FF_FF_FF_FF_FF_FF
SNAKE_FALSE = 0x7F_FF_FF_FF_FF_FF_FF_FF
```



## 4. Transformations

### 4.1 Dynamic type conversion for arithmetic operations

Since sequentialized expressions treat every data as immediate and shadow their types, to support hybrid arithmetic between floating points and integers, these operations requires additional type check and branching at runtime. Specifically, for `+`, `-`, `*`, integers must be treated as floating points except another operand is also an integer. 

```assembly
.section data
	#bufA dq 0.0
	#bufB dq 0.0


.section text
// ... previous commands

// for a + b
// suppose a in Rax, b in R10

// check both operands are numbers
	type_check_is_num(a) ... // primitive to check whether an imm is number
	type_check_is_num(b) ...

// runtime type check
	type_check_is_float(a) ... // primitive to check whether an imm is float
	jz A_FLOAT
A_INT:
	type_check_is_float(b) ...
	jz A_INT_B_FLOAT
A_INT_B_INT:
	untag_int(Rax) ... // primitive to untag a int
	untag_int(R10)
	add Rax R10
	tag_int(Rax) ... // primitive to tag a int
	jmp END

A_FLOAT:
	type_check_is_float(b) ...
	jz A_FLOAT_B_FLOAT
A_FLOAT_B_INT:
	untag_float(Rax) ... // primitive to untag a float
	untag_int(R10)...
	mov bufA Rax
	mov bufB R10
	fld qword [bufA]
	fiadd qword [bufB]
	fstp qword [bufA]
	mov Rax [bufA]
	tag_float(Rax) ... // primitive to tag a float
	jmp END

A_INT_B_FLOAT:
	untag_int(Rax) ...
	untag_float(R10) ...
	mov bufA Rax
	mov bufB R10
	fld qword [bufB]
	fiadd qword [bufA]
	fstp qword [bufA]
	mov Rax [bufA]
	tag_float(Rax) ...
	jmp END

A_FLOAT_B_FLOAT:
	untag_float(Rax) ...	
	untag_float(R10) ...
	mov bufA Rax
	mov bufB R10
	fld qword [bufA]
	fiadd qword [bufB]
	fstp qword [bufA]
	mov Rax [bufA]
	tag_float(Rax) ...
	jmp END

END:
// ... following commands
```

As a sidemark, `-` and `*` operations have a similar framework, while for `/`, we convert both operands into floating points despite their types. For` type_check_is_num()`, we move the data into x86 registers; and for moving a, b to FPU registers, we first move them to a buffer, and then load from buffer with `fld`.

## 5. Errors

### 5.1 Compile Errors

1. `UnboundVariable`: use a variable that is not in scope

2. `UndefinedFunction`: use a function that is not in scope

3. `DuplicateBinding`: two identical variable binding names occur in the same `let` binding

4. `IntegerOverflow` : an integer constant is too large

5. `DuplicateFunName`: two mutually recursive functions have same names

6. `DuplicateArgName`: two argument shares names in a function argument list

7. `FunctionUsedAsValue`: An identifier `x` used in a value position with no corresponding let declaration but where there is a function declaration defining `x`

8. `ValueUsedAsFunction`: apply a function `f` but there is a local variable named `f`

9. `FunctionCalledWrongArity`: a function call with wrong number of arguments

   

### 5.2 Runtime Errors

1. If an arithmetic operation takes a boolean operand, should raise error with message containing `arithmetic expected a number but got a boolean`
2. if a comparison operation takes a boolean operand, should raise error with message containing `comparison expected an integer but got a boolean`
3. if a comparison operation takes a floating point number operand, should raise error with message containing `comparison expected an integer but got a floating point`
4. if an `if` operation takes a non-boolean operand, should raise error with message containing `if expected a boolean but got a number`
5. if a logical operation takes a non-boolean operand, should raise error with message containing `if logic expected a boolean but got a number`
6. If an arithmetic operation runs into overflow, should raise error with message containing `overflow`
7. If denominator of `/` is 0 or 0.0, should raise error with message containing `divided by zero`

