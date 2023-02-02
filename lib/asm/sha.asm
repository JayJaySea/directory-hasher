;
; Library with procedures to perform buffer calculation for sha-1 algorithm
; Author: Jacek Cytera
; Version: 1.3
;
; Exported procedures:
; 	init_asm: Initializes array that is needed in calculations for following procedure
;	compute_buffer_values_asm: Computes a, b, c, d, e buffer values for sha-1 algorithm
; 
; Version log:
; 1.0 - made initial compute_buffer_values_asm procedure drafts
; 1.1 - made naive implementation of sha-1 buffers calculation algorithm
; 1.2 - added init_asm, which is needed for branchless solution
; 1.3 - transformed compute_buffer_values_asm to use that solution (using array instead of jump instructions)

.data
	; four constants used for buffers calculation in sha-1 algorithm
	k_0 dd 05A827999h
	k_1 dd 06ED9EBA1h
	k_2 dd 08F1BBCDCh
	k_3 dd 0CA62C1D6h

	; declaration of f array, later filled in init_asm with procedure addresses
	f dq 80 dup (0)

.code


; This procedure initializes 80 quad word array for later calculations
; Array is filled with addresses of f_n procedures:
;
; Indexes 0 to 19: address of f_0
; Indexes 20 to 39: address of f_1
; Indexes 40 to 69: address of f_2
; Indexes 60 to 79: address of f_3
; 
; Later, in compute_buffer_values_asm, this array is used to easily
; call f_n procedures from within loop
;
; It is vital to call this procedure before calling compute_buffer_values_asm.
;
; No arguments or return value
; Does not destroy any registers
init_asm proc export
	push rdi
	push rax
	push rcx

	lea rcx, qword ptr[f]			; loading address of procedure array (f) to rcx
	mov rdi, 0				; initializing loop counter

	lea rax, qword ptr[f_0]			; loading address of current procedure (f_0) to rax
	vpbroadcastq ymm0, rax			; filling ymm0 register with address of current procedure | *f_0 | *f_0 | *f_0 | *f_0 |
	loop_0:
		vmovdqu ymmword ptr [rcx], ymm0	; copying proper procedure address to array as a vector (4 times at once)
		add rcx, 32			; adding to f_0 procedure address number 32, which is size of ymm register in bytes,
                                        	; to address next memory place to fill

		inc rdi				; incrementing loop counter
		cmp rdi, 5			; checking if there were already 5 iterations
	jb loop_0				; jumping to .loop_0 if there weren't

	lea rax, qword ptr[f_1]			; loading address of current procedure (f_1) to rax
	vpbroadcastq ymm0, rax              	; filling ymm0 register with address of current procedure | *f_1 | *f_1 | *f_1 | *f_1 |
	loop_1:                                                                                                                    
		vmovdqu ymmword ptr [rcx], ymm0	; copying proper procedure address to array as a vector (4 times at once)
		add rcx, 32			; adding to f_0 procedure address number 32, which is size of ymm register in bytes,
						; to address next memory place to fill

		inc rdi				; incrementing loop counter
		cmp rdi, 10			; checking if there were already 10 iterations
	jb loop_1				; jumping to .loop_1 if there weren't

	lea rax, qword ptr[f_2]			; loading address of current procedure (f_2) to rax
	vpbroadcastq ymm0, rax                  ; filling ymm0 register with address of current procedure | *f_2 | *f_2 | *f_2 | *f_2 |
	loop_2:                                                                                                                    
		vmovdqu ymmword ptr [rcx], ymm0	; copying proper procedure address to array as a vector (4 times at once)
		add rcx, 32			; adding to f_0 procedure address number 32, which is size of ymm register in bytes,
                                	        ; to address next memory place to fill

		inc rdi				; incrementing loop counter
		cmp rdi, 15			; checking if there were already 15 iterations
	jb loop_2				; jumping to .loop_2 if there weren't

	lea rax, qword ptr[f_3]			; loading address of current procedure (f_3) to rax
	vpbroadcastq ymm0, rax                  ; filling ymm0 register with address of current procedure | *f_3 | *f_3 | *f_3 | *f_3 |
	loop_3:                                                                                                                    
		vmovdqu ymmword ptr [rcx], ymm0 ; copying proper procedure address to array as a vector (4 times at once)
		add rcx, 32			; adding to f_0 procedure address number 32, which is size of ymm register in bytes,
                           	                ; to address next memory place to fill

		inc rdi				; incrementing loop counter
		cmp rdi, 20			; checking if there were already 20 iterations
	jb loop_3				; jumping to .loop_3 if there weren't

	pop rcx
	pop rax
	pop rdi

	ret
init_asm endp

; This procedure calculates the values of buffers (a, b, c, d, e) for one iteration (processing of 512-bit chunk) of sha-1 algorithm
; It needs a pointer to previously created array consisting of 80 word-sized values, and a pointer to an array of 5 double-word-sized values
; (called later buffers array) that will store values calculated as the result of this part of algorithm.
; 
; rcx - pointer to words array
; rdx - pointer to buffers array (array will be modified by this procedure)
;
; No return value
; Does not destroy any registers
compute_buffer_values_asm proc export
	push rbx				; saving previous register values
	push rsi
	push rdi
	push rcx
	push rdx
	push r10

	mov r10, rcx				; saving pointer to words array in r10
	xor rdi, rdi				; settin loop counter to 0
	xor eax, eax				; cleaning up eax register for later calculations
	mov rsi, rdx
	lea rdx, qword ptr [f]

	main_loop:					; for(int i = 0; i < 80; i++)
		call qword ptr [rdx + rdi*8]		; calling different functions for index in ranges 0..19, 20..39, 40..59, 60..79
							; so that tmp = f[i]()

		mov ebx, dword ptr [rsi]		; copying "a" to ebx
		rol ebx, 5   				; rotating "a" left by 5
		add eax, ebx				; tmp += rol(a, 5)
		add eax, dword ptr [r10 + rdi*4] 	; tmp += words[i]
		add eax, dword ptr [rsi + 16] 		; tmp += e

		mov ebx, dword ptr [rsi + 12]
		mov dword ptr [rsi + 16], ebx 		; e = d

		mov ebx, dword ptr [rsi + 8]
		mov dword ptr [rsi + 12], ebx 		; d = c

		mov ebx, dword ptr [rsi + 4]
		rol ebx, 30
		mov dword ptr [rsi + 8], ebx  		; c = rol(b, 30)

		mov ebx, dword ptr [rsi]
		mov dword ptr [rsi + 4], ebx  		; b = a

		mov dword ptr [rsi], eax 	  	; a = tmp

		inc rdi					; incrementing loop counter
		cmp rdi, 80				; checking if there were already 80 iterations
	jb main_loop					; jumping to .loop if there weren't

	pop r10
	pop rdx
	pop rcx
	pop rdi
	pop rsi
	pop rbx						; restoring previous register values
	ret
compute_buffer_values_asm endp


; These procedures are pre-defined functions used in buffers calculations
; Following f_n procedures operate solely by modifying eax register,
;
; They do not destroy any registers
; Return value in eax
;
; return ((b&c) | ((~b)&d)) + k[0];
f_0 proc
	push rbx			; saving rbx register value

	mov eax, dword ptr [rsi + 4]	; eax = b
	and eax, dword ptr [rsi + 8]	; eax = b&c
	mov ebx, dword ptr [rsi + 4]	; ebx = b
	not ebx				; ebx = ~b
	and ebx, dword ptr [rsi + 12]	; ebx = (~b)&d
	or  eax, ebx			; eax = b&c | (~b)&d
	add eax, dword ptr [k_0]	; eax += k[0]

	pop rbx
	ret
f_0 endp

; return (b^c^d) + k[1];
f_1 proc
	push rbx

	mov eax, dword ptr [rsi + 4]	; eax = b
	xor eax, dword ptr [rsi + 8]	; eax = b^c
	xor eax, dword ptr [rsi + 12]	; eax = b^c^d
	add eax, dword ptr [k_1]	; eax += k[1]

	pop rbx
	ret
f_1 endp

; return ((b&c) | (b&d) | (c&d)) + k[2];
f_2 proc
	push rbx

	mov eax, dword ptr [rsi + 4]	; eax = b
	and eax, dword ptr [rsi + 8]	; eax = b&c

	mov ebx, dword ptr [rsi + 4]	; ebx = b
	and ebx, dword ptr [rsi + 12]	; ebx = b&d

	or  eax, ebx			; eax = b&c | b&d

	mov ebx, dword ptr [rsi + 8]	; ebx = c
	and ebx, dword ptr [rsi + 12]	; ebx = c&d

	or  eax, ebx			; eax = b&c | b&d | c&d
	add eax, dword ptr [k_2]	; eax += k[2]

	pop rbx
	ret
f_2 endp

; return (b^c^d) + k[3];
f_3 proc
	push rbx

	mov eax, dword ptr [rsi + 4]	; eax = b
	xor eax, dword ptr [rsi + 8]    ; eax = b^c
	xor eax, dword ptr [rsi + 12]   ; eax = b^c^d
	add eax, dword ptr [k_3]   	; eax += k[3]

	pop rbx
	ret
f_3 endp
END
