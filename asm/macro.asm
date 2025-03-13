; test_macro_simple.asm
!define_macro MY_VALUE 0x01
!define_macro MY_REGISTER A


ADD !MY_REGISTER !MY_VALUE 0x01  ; Should expand to MOV A, 0xABCD
HALT
