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

generate_syscalls! {
    EXIT = 1
    READ = 3
    WRITE = 4
}

impl Syscall {
    pub fn handle(&self, m: &mut Machine) -> Result<(), String> {
        match self {
            Self::EXIT => m.halt = true,
            Self::WRITE => {
                if m.debug {
                    print!("| WRITE ");
                }
                let mode: u8 = m.fetch()?;
                let start: usize = m.fetch()?;
                let end: usize = m.fetch()?;
                if mode == 1 {
                    if m.debug {
                        println!("STDOUT");
                    }
                    let data = &m.data[start..start + end as usize];
                    /* if let Some(&0) = data.last() {
                        let output = std::str::from_utf8(&data[..data.len() - 1]).map_err(|e| e.to_string())?;
                        print!("{}", output);
                    } else {
                        return Err("Data is not null-terminated".to_string());
                    } */

                    // FIXME: Output should be escaped properly and print the data properly
                    //        See also, preprocessor/pp.rs to see if the data is being layed
                    //        correctly
                    let output =
                        std::str::from_utf8(&data[..data.len()]).map_err(|e| e.to_string())?;

                    print!("{}", output);
                } else {
                    return Err(format!("Unsupported mode: {}", mode));
                }
            }
            _ => return Err(format!("{:?} syscall not implemented", self)),
        }
        Ok(())
    }
}
