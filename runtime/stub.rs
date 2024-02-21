#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct SnakeVal(u64);

static BOOL_TAG: u64 = 0x00_00_00_00_00_00_00_01;
static SNAKE_TRU: SnakeVal = SnakeVal(0xFF_FF_FF_FF_FF_FF_FF_FF);
static SNAKE_FLS: SnakeVal = SnakeVal(0x7F_FF_FF_FF_FF_FF_FF_FF);

#[link(name = "compiled_code", kind = "static")]
extern "sysv64" {

    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    #[link_name = "\x01start_here"]
    fn start_here() -> SnakeVal;
}

// reinterprets the bytes of an unsigned number to a signed number
fn unsigned_to_signed(x: u64) -> i64 {
    i64::from_le_bytes(x.to_le_bytes())
}

fn sprint_snake_val(x: SnakeVal) -> String {
    if x.0 & BOOL_TAG == 0 {
        // it's a number
        format!("{}", unsigned_to_signed(x.0) >> 1)
    } else if x == SNAKE_TRU {
        String::from("true")
    } else if x == SNAKE_FLS {
        String::from("false")
    } else {
        format!("Invalid snake value 0x{:x}", x.0)
    }
}

#[export_name = "\x01print_snake_val"]
extern "sysv64" fn print_snake_val(v: SnakeVal) -> SnakeVal {
    println!("{}", sprint_snake_val(v.clone()));
    return v
}

type ErrorCode = u64;
static ARITH_ERROR: ErrorCode = 0;
static CMP_ERROR:   ErrorCode = 1;
static IF_ERROR:    ErrorCode = 2;
static LOGIC_ERROR: ErrorCode = 3;
static OVFL_ERROR:  ErrorCode = 4;

#[export_name = "\x01snake_error"]
extern "sysv64" fn snake_error(err_code: ErrorCode, v: SnakeVal) {
    if err_code == ARITH_ERROR {
        eprintln!("arithmetic expected a number but got a boolean {}", sprint_snake_val(v));
    } else if err_code == CMP_ERROR {
        eprintln!("comparison expected a number but got a boolean {}", sprint_snake_val(v));
    } else if err_code == IF_ERROR {
        eprintln!("if expected a boolean but got a number {}", sprint_snake_val(v));
    } else if err_code == LOGIC_ERROR {
        eprintln!("logic expected a boolean but got a number {}", sprint_snake_val(v));
    } else if err_code == OVFL_ERROR {
        eprintln!("overflow");    
    } else {
        eprintln!("I apologize to you, dear user. I made a bug. Here's a snake value: {}", sprint_snake_val(v));
    }
    std::process::exit(1);
}

fn main() {
    let output = unsafe { start_here() };
    println!("{}", sprint_snake_val(output));
}
