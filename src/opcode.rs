macro_rules! generate_opcodes {
    ($($name:ident = $v:expr)*) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        #[repr(u8)]
        pub enum OpCode {
            $(
                $name = $v,
            )*
        }
        impl TryFrom<u8> for OpCode {
            type Error = String;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $v => Ok(Self::$name),
                    )*
                    _ => Err(format!("{} is not a valid opcode", value))
                }
            }
        }
        impl TryFrom<&str> for OpCode {
            type Error = String;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
               match value {
                    $(
                        stringify!($name) => Ok(Self::$name),
                    )*
                    _ => Err(format!("{} is not a valid opcode", value))
               }
            }
        }
    };
}

generate_opcodes! {
    HALT = 0x00
    NOP = 0x01

    ADD = 0x50
    SUB = 0x51
    MUL = 0x52
    DIV = 0x53

    PUSH = 0x60
    POP = 0x61
    SWAP = 0x62
}
