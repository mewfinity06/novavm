
macro_rules! generate_opcodes {
    ($($name:ident = $v:expr)*) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        #[repr(u8)]
        pub enum OpCode {
            $(
                $name = $v,
            )*
        }
        impl From<u8> for OpCode {
            fn from(value: u8) -> Self {
                match value {
                    $(
                        $v => Self::$name,
                    )*
                    _ => panic!()
                }
            }
        }
    };
}

generate_opcodes!{
    HALT = 0x00
    NOP = 0x01

    ADD = 0x50
    SUB = 0x51
    MUL = 0x52
    DIV = 0x53

    PUSH = 0x60
    POP = 0x61
}