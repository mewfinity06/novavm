macro_rules! generate_syscalls {
    ($($name:ident = $v:expr)*) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        #[repr(u8)]
        pub enum Syscall {
            $(
                $name = $v,
            )*
        }
        impl TryFrom<u8> for Syscall {
            type Error = String;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $v => Ok(Self::$name),
                    )*
                    _ => Err(format!("{} is not a valid syscall", value))
                }
            }
        }
        impl TryFrom<&str> for Syscall {
            type Error = String;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
               match value {
                    $(
                        stringify!($name) => Ok(Self::$name),
                    )*
                    _ => Err(format!("{} is not a valid syscall", value))
               }
            }
        }
    };
}

generate_syscalls!{
    EXIT = 1
}