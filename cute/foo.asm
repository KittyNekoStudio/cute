    global print_uint32

print_uint32:
    mov rax, rdi

    mov  ecx, 0xa
    push rcx
    mov  rsi, rsp
    sub  rsp, 16

    .toascii_digit:
    xor  edx, edx
    div  ecx
    add  edx, '0'
    dec  rsi
    mov [rsi], dl

    test rax, rax
    jnz  .toascii_digit


    mov  rax, 1
    mov  rdi, 1
    lea  edx, [rsp+16 + 1]
    sub  edx, esi
    syscall
    add  rsp, 24
    ret

section .text
    global _start

_start:
    mov  rdi, 4
    add  rdi, 9
    call print_uint32
    mov  rdi, 32
    sub  rdi, 4
    call print_uint32
    mov  rax, 4
    mov  rdx, 72
    mul  rdx
    mov  rdi, rax 
    call print_uint32
    mov  rax, 60
    mov  rdi, 0
    syscall
