use crate::Register;
use crate::opcode::OpCode;

pub trait Fetch : From<u8> {}

impl Fetch for Register {}
impl Fetch for OpCode {}
impl Fetch for u8 {}
impl Fetch for u16 {}