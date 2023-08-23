SRC = main.c
TARGET = main
GCC = gcc
FLAGS = -o
LIBDIR = -L ./target/debug
LIB = -lcall_from_c

${TARGET} :
	${GCC} ${SRC} ${FLAGS} ${TARGET} ${LIBDIR} ${LIB}

clean : 
	-rm ${TARGET}