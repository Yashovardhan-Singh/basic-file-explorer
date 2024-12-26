BITS 64

global sys_get_cwd

section .text
sys_get_cwd:
    mov rax, 79             ; so easy to patch this and make a virus a lol
    syscall
    ret
