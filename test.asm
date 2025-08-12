section .bss
mem resb 30000
section .text
global _start
_start:
    mov rbx, mem
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
loop_start_0:
    cmp byte [rbx], 0
    je loop_end_0
    add rbx, 1
    inc byte [rbx]
    add rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    add rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    add rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    sub rbx, 1
    sub rbx, 1
    sub rbx, 1
    sub rbx, 1
    dec byte [rbx]
    cmp byte [rbx], 0
    jne loop_start_0
loop_end_0:
    add rbx, 1
    add rbx, 1
    add rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    add rbx, 1
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    sub rbx, 1
    sub rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    add rbx, 1
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    add rbx, 1
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    inc byte [rbx]
    inc byte [rbx]
    inc byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    dec byte [rbx]
    mov rax, 1
    mov rdi, 1
    mov rsi, rbx
    mov rdx, 1
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall
