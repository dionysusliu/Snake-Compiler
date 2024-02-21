section .text
        global start_here
        extern snake_error
        extern print_snake_val
start_here:
        call main
        ret
main:
        mov rax, 12
;;; Let
        mov [rsp + -8], rax
;;; FunDefs103_body
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -16], rax
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -24], rax
;;; InCall
        mov rax, [rsp + -16]
        mov [rsp + -88], rax
        mov rax, [rsp + -24]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp collatz#66
        ret
;;; FunDefs103_decls
when_odd#66:
        mov rax, 186
;;; Let
        mov [rsp + -24], rax
;;; Prim1
        mov rax, [rsp + -24]
;;; Print
        mov rdi, rax
        sub rsp, 80
        call print_snake_val
        add rsp, 80
;;; Let
        mov [rsp + -24], rax
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -32], rax
        mov rax, 6
;;; Let
        mov [rsp + -40], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -48], rax
;;; Prim2
        mov rax, [rsp + -40]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jnz snake_err
        mov r10, [rsp + -48]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jnz snake_err
;;; Mul
        sar rax, 0x00000001
        imul rax, r10
;;; Check overflow
        mov rdi, 0x0000000000000004
        mov rsi, rax
        jo snake_err
;;; Let
        mov [rsp + -40], rax
        mov rax, 2
;;; Let
        mov [rsp + -48], rax
;;; Prim2
        mov rax, [rsp + -40]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jnz snake_err
        mov r10, [rsp + -48]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jnz snake_err
;;; Add
        add rax, r10
;;; Check overflow
        mov rdi, 0x0000000000000004
        mov rsi, rax
        jo snake_err
;;; Let
        mov [rsp + -40], rax
;;; InCall
        mov rax, [rsp + -32]
        mov [rsp + -88], rax
        mov rax, [rsp + -40]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp collatz#66
        ret
when_even#66:
        mov rax, 188
;;; Let
        mov [rsp + -24], rax
;;; Prim1
        mov rax, [rsp + -24]
;;; Print
        mov rdi, rax
        sub rsp, 80
        call print_snake_val
        add rsp, 80
;;; Let
        mov [rsp + -24], rax
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -32], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -40], rax
        mov rax, 4
;;; Let
        mov [rsp + -48], rax
        mov rax, 0
;;; Let
        mov [rsp + -56], rax
;;; ExCall
        mov rax, [rsp + -40]
        mov [rsp + -88], rax
        mov rax, [rsp + -48]
        mov [rsp + -96], rax
        mov rax, [rsp + -56]
        mov [rsp + -104], rax
        sub rsp, 72
        call div#21
        add rsp, 72
;;; Let
        mov [rsp + -40], rax
;;; InCall
        mov rax, [rsp + -32]
        mov [rsp + -88], rax
        mov rax, [rsp + -40]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp collatz#66
        ret
collatz#66:
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -24], rax
;;; Prim1
        mov rax, [rsp + -24]
;;; Print
        mov rdi, rax
        sub rsp, 80
        call print_snake_val
        add rsp, 80
;;; Let
        mov [rsp + -24], rax
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -32], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -40], rax
;;; ExCall
        mov rax, [rsp + -32]
        mov [rsp + -88], rax
        mov rax, [rsp + -40]
        mov [rsp + -96], rax
        sub rsp, 72
        call base_case#47
        add rsp, 72
;;; Let
        mov [rsp + -32], rax
;;; If
        mov rax, [rsp + -32]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#146
        mov rax, 0
        jmp done#146
if_false#146:
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -40], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -48], rax
;;; ExCall
        mov rax, [rsp + -40]
        mov [rsp + -88], rax
        mov rax, [rsp + -48]
        mov [rsp + -96], rax
        sub rsp, 72
        call is_even#3
        add rsp, 72
;;; Let
        mov [rsp + -40], rax
;;; If
        mov rax, [rsp + -40]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#154
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -48], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -56], rax
;;; InCall
        mov rax, [rsp + -48]
        mov [rsp + -88], rax
        mov rax, [rsp + -56]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp when_even#66
        jmp done#154
if_false#154:
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -48], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -56], rax
;;; InCall
        mov rax, [rsp + -48]
        mov [rsp + -88], rax
        mov rax, [rsp + -56]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp when_odd#66
done#154:
done#146:
        ret
        ret
;;; Global FunDecls
is_even#3:
        mov rax, 180
;;; Let
        mov [rsp + -24], rax
;;; Prim1
        mov rax, [rsp + -24]
;;; Print
        mov rdi, rax
        sub rsp, 80
        call print_snake_val
        add rsp, 80
;;; Let
        mov [rsp + -24], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -32], rax
        mov rax, 0
;;; Let
        mov [rsp + -40], rax
;;; Prim2
        mov rax, [rsp + -32]
        mov r10, [rsp + -40]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#11
        mov rax, 0x7fffffffffffffff
equal#11:
;;; Let
        mov [rsp + -32], rax
;;; If
        mov rax, [rsp + -32]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#12
        mov rax, 0xffffffffffffffff
        jmp done#12
if_false#12:
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -40], rax
        mov rax, 2
;;; Let
        mov [rsp + -48], rax
;;; Prim2
        mov rax, [rsp + -40]
        mov r10, [rsp + -48]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#19
        mov rax, 0x7fffffffffffffff
equal#19:
;;; Let
        mov [rsp + -40], rax
;;; If
        mov rax, [rsp + -40]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#20
        mov rax, 0x7fffffffffffffff
        jmp done#20
if_false#20:
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -48], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -56], rax
        mov rax, 4
;;; Let
        mov [rsp + -64], rax
;;; Prim2
        mov rax, [rsp + -56]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jnz snake_err
        mov r10, [rsp + -64]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jnz snake_err
;;; Sub
        sub rax, r10
;;; Check overflow
        mov rdi, 0x0000000000000004
        mov rsi, rax
        jo snake_err
;;; Let
        mov [rsp + -56], rax
;;; ExCall
        mov rax, [rsp + -48]
        mov [rsp + -88], rax
        mov rax, [rsp + -56]
        mov [rsp + -96], rax
        mov rax, [rsp + -88]
        mov [rsp + -8], rax
        mov rax, [rsp + -96]
        mov [rsp + -16], rax
        jmp is_even#3
done#20:
done#12:
        ret
div#21:
        mov rax, 182
;;; Let
        mov [rsp + -40], rax
;;; Prim1
        mov rax, [rsp + -40]
;;; Print
        mov rdi, rax
        sub rsp, 128
        call print_snake_val
        add rsp, 128
;;; Let
        mov [rsp + -40], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -48], rax
;;; Prim1
        mov rax, [rsp + -48]
;;; Print
        mov rdi, rax
        sub rsp, 128
        call print_snake_val
        add rsp, 128
;;; Let
        mov [rsp + -48], rax
        mov rax, [rsp + -24]
;;; Let
        mov [rsp + -56], rax
;;; Prim1
        mov rax, [rsp + -56]
;;; Print
        mov rdi, rax
        sub rsp, 128
        call print_snake_val
        add rsp, 128
;;; Let
        mov [rsp + -56], rax
        mov rax, [rsp + -32]
;;; Let
        mov [rsp + -64], rax
;;; Prim1
        mov rax, [rsp + -64]
;;; Print
        mov rdi, rax
        sub rsp, 128
        call print_snake_val
        add rsp, 128
;;; Let
        mov [rsp + -64], rax
        mov rax, [rsp + -24]
;;; Let
        mov [rsp + -72], rax
        mov rax, [rsp + -32]
;;; Let
        mov [rsp + -80], rax
;;; Prim2
        mov rax, [rsp + -72]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jnz snake_err
        mov r10, [rsp + -80]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jnz snake_err
;;; Mul
        sar rax, 0x00000001
        imul rax, r10
;;; Check overflow
        mov rdi, 0x0000000000000004
        mov rsi, rax
        jo snake_err
;;; Let
        mov [rsp + -72], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -80], rax
;;; Prim2
        mov rax, [rsp + -72]
        mov r10, [rsp + -80]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#56
        mov rax, 0x7fffffffffffffff
equal#56:
;;; Let
        mov [rsp + -72], rax
;;; If
        mov rax, [rsp + -72]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#57
        mov rax, [rsp + -32]
        jmp done#57
if_false#57:
        mov rax, [rsp + -8]
;;; Let
        mov [rsp + -80], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -88], rax
        mov rax, [rsp + -24]
;;; Let
        mov [rsp + -96], rax
        mov rax, [rsp + -32]
;;; Let
        mov [rsp + -104], rax
        mov rax, 2
;;; Let
        mov [rsp + -112], rax
;;; Prim2
        mov rax, [rsp + -104]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jnz snake_err
        mov r10, [rsp + -112]
;;; Check Whether Num
        mov rdi, 0x0000000000000000
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jnz snake_err
;;; Add
        add rax, r10
;;; Check overflow
        mov rdi, 0x0000000000000004
        mov rsi, rax
        jo snake_err
;;; Let
        mov [rsp + -104], rax
;;; ExCall
        mov rax, [rsp + -80]
        mov [rsp + -136], rax
        mov rax, [rsp + -88]
        mov [rsp + -144], rax
        mov rax, [rsp + -96]
        mov [rsp + -152], rax
        mov rax, [rsp + -104]
        mov [rsp + -160], rax
        mov rax, [rsp + -136]
        mov [rsp + -8], rax
        mov rax, [rsp + -144]
        mov [rsp + -16], rax
        mov rax, [rsp + -152]
        mov [rsp + -24], rax
        mov rax, [rsp + -160]
        mov [rsp + -32], rax
        jmp div#21
done#57:
        ret
base_case#47:
        mov rax, 184
;;; Let
        mov [rsp + -24], rax
;;; Prim1
        mov rax, [rsp + -24]
;;; Print
        mov rdi, rax
        sub rsp, 64
        call print_snake_val
        add rsp, 64
;;; Let
        mov [rsp + -24], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -32], rax
        mov rax, 2
;;; Let
        mov [rsp + -40], rax
;;; Prim2
        mov rax, [rsp + -32]
        mov r10, [rsp + -40]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#82
        mov rax, 0x7fffffffffffffff
equal#82:
;;; Let
        mov [rsp + -32], rax
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -40], rax
        mov rax, 4
;;; Let
        mov [rsp + -48], rax
;;; Prim2
        mov rax, [rsp + -40]
        mov r10, [rsp + -48]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#88
        mov rax, 0x7fffffffffffffff
equal#88:
;;; Let
        mov [rsp + -40], rax
;;; Prim2
        mov rax, [rsp + -32]
;;; Check Whether Bool
        mov rdi, 0x0000000000000003
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, [rsp + -40]
;;; Check Whether Bool
        mov rdi, 0x0000000000000003
        mov rsi, r10
        mov rbx, 0x0000000000000001
        test rbx, r10
        jz snake_err
        or rax, r10
;;; Let
        mov [rsp + -32], rax
;;; If
        mov rax, [rsp + -32]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#90
        mov rax, 0xffffffffffffffff
        jmp done#90
if_false#90:
        mov rax, [rsp + -16]
;;; Let
        mov [rsp + -40], rax
        mov rax, 8
;;; Let
        mov [rsp + -48], rax
;;; Prim2
        mov rax, [rsp + -40]
        mov r10, [rsp + -48]
;;; Compare
        cmp rax, r10
        mov rax, 0xffffffffffffffff
        je equal#97
        mov rax, 0x7fffffffffffffff
equal#97:
;;; Let
        mov [rsp + -40], rax
;;; If
        mov rax, [rsp + -40]
;;; Check Whether Bool
        mov rdi, 0x0000000000000002
        mov rsi, rax
        mov rbx, 0x0000000000000001
        test rbx, rax
        jz snake_err
        mov r10, 0x7fffffffffffffff
        cmp rax, r10
        je if_false#98
        mov rax, 0xffffffffffffffff
        jmp done#98
if_false#98:
        mov rax, 0x7fffffffffffffff
done#98:
done#90:
        ret
snake_err:
        call snake_error

