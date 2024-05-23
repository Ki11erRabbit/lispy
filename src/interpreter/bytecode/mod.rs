mod virtual_machine;

use crate::interpreter::InterpreterResult;

use super::context::Context;

#[derive(Debug, PartialEq, Clone)]
pub struct Bytecode {
    raw: RawBytecode,
    line: usize,
    column: usize
}

impl Bytecode {
    pub fn new(raw: RawBytecode, line: usize, column: usize) -> Self {
	Bytecode {
	    raw,
	    line,
	    column
	}
    }

    pub fn get_raw(&self) -> &RawBytecode {
	&self.raw
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RawBytecode {
    PushString(String),
    PushInteger(String),
    PushFloat(f64),
    PushBoolean(bool),
    PushSymbol(Vec<String>),
    PushChar(char),
    Pop,
    Store,
    Load,
    BindKeyword,
    Call(usize),
    Return,
    MakeStruct(usize),
    StructAccess,
    StructStore,
    MakeEnum(usize),
    EnumAccess,
    EnumStore,

}


pub fn run(bytecode: &[Bytecode], context: &mut Context) -> InterpreterResult {
    let mut vm = virtual_machine::VirtualMachine::new(bytecode);
    vm.run(context)
}
