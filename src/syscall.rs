use crate::Machine;

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
        impl Syscall {
            pub fn required_args_count(&self) -> u8 {
                match self {
                    $(
                        Self::$name => ($v - 1) as u8,
                    )*
                }
            }
        }
    };
}

generate_syscalls!{
    EXIT = 1
    READ = 3
    WRITE = 4
}

impl Syscall {
    pub fn handle(&self, m: &mut Machine) -> Result<(), String> {
        let args_needed = self.required_args_count() as usize;
        match self {
            Self::EXIT => m.halt = true,
            Self::WRITE => {
                // SYSCALL WRITE 1
                let _mode: u8 = m.fetch()?;
                unimplemented!("Must have data section")
            }
            _ => return Err(format!("{:?} syscall not implemented", self)),
        }
        Ok(())
    }
}