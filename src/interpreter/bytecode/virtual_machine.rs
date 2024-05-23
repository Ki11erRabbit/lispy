use std::collections::HashMap;

use crate::interpreter::value::Struct;
use crate::interpreter::{bytecode::Bytecode, value::Value, context::Context};
use crate::interpreter::InterpreterResult;
use crate::interpreter::bytecode::RawBytecode;
use crate::interpreter::Exception;









pub struct VirtualMachine<'a> {
    instructions: &'a [Bytecode],
    pc: usize,
    stack: Vec<Value>,
    keywords: Option<HashMap<String, Value>>, 
}

impl<'a> VirtualMachine<'a> {
    pub fn new(instructions: &'a [Bytecode]) -> Self {
	VirtualMachine {
	    instructions,
	    pc: 0,
	    stack: Vec::new(),
	    keywords: None,
	}
    }

    pub fn run(&mut self, context: &mut Context) -> InterpreterResult {
	while self.pc < self.instructions.len() {
	    
	    if crate::gc::is_gc_on() {
		context.garbage_collect_vm(&mut self.stack);
	    }
	    
	    let instruction = &self.instructions[self.pc];
	    match instruction.get_raw() {
		RawBytecode::PushString(s) => {
		    self.stack.push(Value::new_string(s, context));
		}
		RawBytecode::PushInteger(i) => {
		    self.stack.push(Value::new_integer(i));
		}
		RawBytecode::PushFloat(f) => {
		    self.stack.push(Value::new_float(*f));
		}
		RawBytecode::PushBoolean(b) => {
		    self.stack.push(Value::new_boolean(*b));
		}
		RawBytecode::PushSymbol(s) => {
		    self.stack.push(Value::new_symbol(s.clone(), context));
		}
		RawBytecode::PushChar(c) => {
		    self.stack.push(Value::new_char(*c));
		}
		RawBytecode::Pop => {
		    self.stack.pop();
		}
		RawBytecode::Store => {
		    let symbol = self.stack.pop().expect("stack is empty");
		    let value = self.stack.pop().expect("stack is empty");
		    let symbol = symbol.get_symbol(context)?;
		    context.bind(symbol, value);
		}
		RawBytecode::Load => {
		    let symbol = self.stack.pop().expect("stack is empty");
		    let symbol = symbol.get_symbol(context)?;
		    let Some(value) = context.get(symbol) else {
			return Err(Box::new(Exception::new(symbol, "symbol not found", context)));
		    };
		    self.stack.push(value);
		}
		RawBytecode::BindKeyword => {
		    let value = self.stack.pop().expect("stack is empty");
		    let keyword = self.stack.pop().expect("stack is empty");
		    if let Some(ref mut keywords) = self.keywords {
			keywords.insert(keyword.get_symbol(context)?.last().unwrap().clone(), value);
		    } else {
			let mut keywords = HashMap::new();
			keywords.insert(keyword.get_symbol(context)?.last().unwrap().clone(), value);
			self.keywords = Some(keywords);
		    }
		}
		RawBytecode::Call(arg_count) => {
		    let function = self.stack.pop().expect("stack is empty");
		    let function = function.get_function(context)?;
		    let kwarg = if let Some(keywords) = self.keywords.take() {
			keywords
		    } else {
			HashMap::new()
		    };
		    let mut args = Vec::new();
		    for _ in 0..*arg_count {
			args.push(self.stack.pop().expect("stack is empty"));
		    }
		    //TODO add function name to call so that we can get the function name in the exception
		    let result = function.call_from_bytecode(&vec![], args, kwarg, context)?;
		    if let Some(result) = result {
			self.stack.push(result);
		    }
		}
		RawBytecode::Return => {
		    return Ok(self.stack.pop());
		}
		RawBytecode::MakeStruct(field_count) => {
		    let name = self.stack.pop().expect("stack is empty");
		    let name = name.get_symbol(context)?;
		    let mut fields = Vec::new();
		    for _ in 0..*field_count {
			fields.push(self.stack.pop().expect("stack is empty"));
		    }

		    let structure = Struct::new(context.get_or_create_type_symbol(name), fields.into_boxed_slice());
		    self.stack.push(Value::new_struct(structure, context));
		}
		RawBytecode::StructAccess => {
		    let index = self.stack.pop().expect("stack is empty");
		    let index = index.get_integer(context)?;
		    let structure = self.stack.pop().expect("stack is empty");
		    let structure = structure.get_struct(context)?;
		    self.stack.push(structure.get_member(index.to_u64().unwrap() as usize, context)?.clone());
		}
		    
	    }
	}
	Ok(None)
    }
}

