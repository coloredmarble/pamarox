; consider using 2 inc/dec instead of [thing + index] (not sure which one is faster)

adios: ret

ret_nuh:
    mov rax, 0
    ret

global shady_byte_deref
shady_byte_deref:
    mov al, byte [rdi]

; doesn't work for some reason. i need to study the abi further
global shady_ptr_inc
shady_ptr_inc:
    inc rdi

; rdi = str
; rsi = c
; forward
global strchr
strchr:
    cmp byte [rdi], sil
    je .ret
    cmp byte [rdi], 0
    je ret_nuh
    inc rdi
    jmp strchr
.ret:
    mov rax, rdi
    ret

; checks if 2 strings are equal.
; if not equal. return s1[i] - s2[i]
; else 0 if reached end
; rdi = s1
; rsi = s2
; al = tbyte s1
; r8b = tbyte s2
; rdx = index
; 1 instruction for inc rdx, 2 for both strs
; https://github.com/Paul-Marie/minilibc/blob/master/source/strcmp.asm
; forward
global strcmp
strcmp:
    mov al, byte [rdi + rdx]
    test al,al
    jz ret_nuh
    mov r8b, byte [rsi + rdx]
    test r8b, r8b
    jz ret_nuh
    cmp al, r8b
    jne .ret
    inc rdx
    jmp strcmp
.ret:
    sub al, r8b
    ; no REX.W prefix or whatever the fuck it is
    ; https://www.felixcloutier.com/x86/movsx:movsxd
    movsx eax, al ; instruction default size is i32
    ret

; rdi = str
; rax = len
; forward
global strlen
strlen:
    xor rax, rax ; clear first (its not 0 for some reason)
    xor r8, r8 ; use as tmp byte (cant test 2 mem addr)
.loop: 
    mov r8b, byte [rdi + rax]
    test byte [rdi + rax], r8b
    jz adios
    inc rax
    jmp .loop

; strcmp but with n celing
; rdi = s1
; rsi = s2
; al = tbyte s1
; r8b = tbyte s2
; rdx = index
; backwards
strncmp:
global strncmp
    test rdx,rdx
    jz ret_nuh
    mov al, byte [rdi + rdx]
    test al, al
    jz ret_nuh
    mov r8b, byte [rsi + rdx]
    test r8b, r8b
    jz ret_nuh
    cmp al, r8b
    jne .ret
    dec rdx
    jmp strncmp
.ret:
    sub al, r8b
    movsx eax, al ; movsx results in i32
    ret


