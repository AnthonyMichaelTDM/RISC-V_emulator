# Traditional Matrix Multiply program
		.data
matrix_a:
		.word   1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12
		.word  13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
		.word  25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36
		.word  37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48
		.word  49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60
		.word  61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72
		.word  73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84
		.word  85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96
		.word  97, 98, 99,100,101,102,103,104,105,106,107,108
		.word 109,110,111,112,113,114,115,116,117,118,119,120
		.word 121,122,123,124,125,126,127,128,129,130,131,132
		.word 133,134,135,136,137,138,139,140,141,142,143,144

matrix_b:
		.word 133,134,135,136,137,138,139,140,141,142,143,144
		.word 121,122,123,124,125,126,127,128,129,130,131,132
		.word 109,110,111,112,113,114,115,116,117,118,119,120
		.word  97, 98, 99,100,101,102,103,104,105,106,107,108
		.word  85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96
		.word  73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84
		.word  61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72
		.word  49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60
		.word  37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48
		.word  25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36
		.word  13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
		.word   1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12

matrix_c:
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
		.word   0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0

n:		.word 12

nline:  	.string "\n"			#Define new line string
space:		.string " "
msga: 		.string "Matrix A is: \n"
msgb: 		.string "Matrix B is: \n"
msgc: 		.string "Matrix C=A*B is: \n"
		.text
		.globl main
main:

		la	s1, n
		lw	s1, 0(s1)
		la	s2, matrix_a
		la	s3, matrix_b
		la	s4, matrix_c

		la	a0, msga
		la 	a1, matrix_a
		jal	PRINT_MAT 
		la	a0, msgb
		la 	a1, matrix_b
		jal	PRINT_MAT 

# Your CODE HERE
# our registers:
# s1 = n
# s2 = matrix_a
# s3 = matrix_b
# s4 = matrix_c
# t0 = i
# t1 = j
# t2 = k
# t3-6: for arithmetic operations
# 
# Recall: address of element (i,j) In a 2D array A is 
# "base address of the matrix + (i * N + j) * 4"


# Naive matrix multiplication (27566)

# for (i=0; i < N; i++)
		li t0, 0
ISTART:
		bge t0, s1, IEND
#   for (j=0; j< N; j++)
		li t1, 0
JSTART:
		bge t1, s1, JEND
#   	for (k=0; k<N; k++)
		li t2, 0
# 		calc address of C[i][j]
		mul t6, t0, s1 # t4 = N * i
		add t6, t6, t1 # t4 = N * i + j
		slli t6, t6, 2 # t4 = t4 * 4 to make byte offset
		add t6, t6, s4 # t4 = matrix starting address + t4
# 		load C[i][j] to t5
		lw t5, 0(t6) # load C[i][j] to t5
KSTART:
		bge t2, s1, KEND
#			C[i][j] = C[i][j] + A[i,k]*B[k][j];

		# load A[i][k] to t3
		mul t3, t0, s1 # t3 = N * i
		add t3, t3, t2 # t3 = N * i + k
		slli t3, t3, 2 # t3 = t3 * 4 to make byte offset
		add t3, t3, s2 # t3 = matrix starting address + t3
		lw t3, 0(t3) # load A[i][k] to t3

		# load B[k][j] to t4
		mul t4, t2, s1 # t4 = N * k
		add t4, t4, t1 # t4 = N * k + j
		slli t4, t4, 2 # t4 = t4 * 4 to make byte offset
		add t4, t4, s3 # t4 = matrix starting address + t4
		lw t4, 0(t4) # load B[k][j] to t4

		# multiply A[i][k] and B[k][j]
		mul t3, t3, t4
		# add the result to C[i][j]
		add t5, t5, t3 # add the result to C[i][j]

		# increment k
		addi t2, t2, 1
		j KSTART
KEND:
		# store C[i][j] to matrix_c
		sw t5, 0(t6)
		addi t1, t1, 1
		j JSTART
JEND:
		addi t0, t0, 1
		j ISTART
IEND:


# End CODE

		la	a0, msgc
		la 	a1, matrix_c
		jal	PRINT_MAT 

#   Exit
		li	 a7,10
    		ecall


PRINT_MAT:	li	a7,4
		ecall
		addi 	a2,x0,0	
PL4:		bge	a2,s1,PL1
		addi 	a3,x0,0
PL3:		bge	a3,s1,PL2

		lw	a0,0(a1)
		li	a7,1
		ecall
		la	a0,space
		li	a7,4
		ecall
		addi 	a1,a1,4
		addi 	a3,a3,1
		jal 	PL3

PL2:		addi	a2,a2,1
		la	a0,nline
		li	a7,4
		ecall
		jal	PL4
PL1:		jr	ra
