use crate::Register;
use crate::opcode::OpCode;

pub trait Fetch<'a> : TryFrom<u8> {}

impl<'a> Fetch<'a> for Register {}

impl TryFrom<u8> for Register {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::M),
            4 => Ok(Self::SP),
            5 => Ok(Self::PC),
            6 => Ok(Self::FLAGS),
            _ => Err(format!("{} is not a valid register", value)),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "M" => Ok(Self::M),
            "SP" => Ok(Self::SP),
            "PC" => Ok(Self::PC),
            "FLAGS" => Ok(Self::FLAGS),
            _ => Err(format!("{} is not a valid register", value))
        }
    }
}

impl<'a> Fetch<'a> for OpCode {}

impl<'a> Fetch<'a> for u8 {}
impl<'a> Fetch<'a> for u16 {}