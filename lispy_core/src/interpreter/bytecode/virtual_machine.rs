
use crate::interpreter::value::{r#struct::Struct, r#enum::Enum};
use crate::interpreter::{bytecode::Bytecode, value::Value, context::Context};
use crate::interpreter::InterpreterResult;
use crate::interpreter::bytecode::RawBytecode;
use crate::interpreter::Exception;
use crate::interpreter::kwargs::Kwargs;









pub struct VirtualMachine<'a> {
    instructions: &'a [Bytecode],
    pc: usize,
    stack: Vec<Value>,
    keywords: Option<Kwargs>, 
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

    pub fn run(&mut self, context: &mut Context, module_name: &Vec<String>) -> InterpreterResult {
	while self.pc < self.instructions.len() {
	    
	    if crate::gc::is_gc_on() {
		context.garbage_collect_vm(&mut self.stack);
	    }
	    
	    let instruction = &self.instructions[self.pc];
	    match instruction.get_raw() {
		RawBytecode::PushString(s) => {
		    self.stack.push(Value::new_string(s, context));
		    self.pc += 1;
		}
		RawBytecode::PushInteger(i) => {
		    self.stack.push(Value::new_integer(i));
		    self.pc += 1;
		}
		RawBytecode::PushFloat(f) => {
		    self.stack.push(Value::new_float(*f));
		    self.pc += 1;
		}
		RawBytecode::PushBoolean(b) => {
		    self.stack.push(Value::new_boolean(*b));
		    self.pc += 1;
		}
		RawBytecode::PushSymbol(s) => {
		    self.stack.push(Value::new_symbol(s.clone(), context));
		    self.pc += 1;
		}
		RawBytecode::PushChar(c) => {
		    self.stack.push(Value::new_char(*c));
		    self.pc += 1;
		}
		RawBytecode::Pop => {
		    self.stack.pop();
		    self.pc += 1;
		}
		RawBytecode::Store => {
		    let symbol = self.stack.pop().expect("stack is empty");
		    let value = self.stack.pop().expect("stack is empty");
		    let symbol = symbol.get_symbol(context)?;
		    context.bind(symbol, value);
		    self.pc += 1;
		}
		RawBytecode::Load => {
		    let symbol = self.stack.pop().expect("stack is empty");
		    let symbol = symbol.get_symbol(context)?;
		    let Some(value) = context.get(symbol, module_name) else {
			return Err(Box::new(Exception::new(symbol, "symbol not found", context)));
		    };
		    self.stack.push(value);
		    self.pc += 1;
		}
		RawBytecode::BindKeyword => {
		    let value = self.stack.pop().expect("stack is empty");
		    let keyword = self.stack.pop().expect("stack is empty");
		    if let Some(ref mut keywords) = self.keywords {
			keywords.insert(keyword.get_symbol(context)?.last().unwrap().clone(), value);
		    } else {
			let mut keywords = Kwargs::new();
			keywords.insert(keyword.get_symbol(context)?.last().unwrap().clone(), value);
			self.keywords = Some(keywords);
		    }
		    self.pc += 1;
		}
		RawBytecode::Call(arg_count) => {
		    let function = self.stack.pop().expect("stack is empty");
		    let function = function.get_function(context)?;
		    let kwarg = if let Some(keywords) = self.keywords.take() {
			keywords
		    } else {
			Kwargs::new()
		    };
		    let mut args = Vec::new();
		    for _ in 0..*arg_count {
			args.push(self.stack.pop().expect("stack is empty"));
		    }
		    //TODO add function name to call so that we can get the function name in the exception
		    let result = function.call_from_bytecode(&vec![], args, kwarg, context, module_name)?;
		    if let Some(result) = result {
			self.stack.push(result);
		    }
		    self.pc += 1;
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
		    self.pc += 1;
		}
		RawBytecode::StructAccess => {
		    let index = self.stack.pop().expect("stack is empty");
		    let index = index.get_integer(context)?;
		    let structure = self.stack.pop().expect("stack is empty");
		    let structure = structure.get_struct(context)?;
		    self.stack.push(structure.get_member(index.to_u64().unwrap() as usize, context)?.clone());
		    self.pc += 1;
		}
		RawBytecode::StructStore => {
		    let index = self.stack.pop().expect("stack is empty");
		    let index = index.get_integer(context)?;
		    let value = self.stack.pop().expect("stack is empty");
		    let mut structure = self.stack.pop().expect("stack is empty");
		    let structure = structure.get_struct_mut(context)?;
		    structure.set_member(index.to_u64().unwrap() as usize, value.clone(), context)?;
		    self.pc += 1;
		}
		RawBytecode::MakeEnum(field_count) => {
		    let name = self.stack.pop().expect("stack is empty");
		    let variant = self.stack.pop().expect("stack is empty");
		    let variant = variant.get_symbol(context)?;
		    let name = name.get_symbol(context)?;
		    let mut fields = Vec::new();
		    for _ in 0..*field_count {
			fields.push(self.stack.pop().expect("stack is empty"));
		    }

		    let enumeration = Enum::new(context.get_or_create_type_symbol(name), context.get_or_create_type_symbol(variant), fields.into_boxed_slice());
		    self.stack.push(Value::new_enum(enumeration, context));
		    self.pc += 1;
		},
		RawBytecode::EnumAccess => {
		    let index = self.stack.pop().expect("stack is empty");
		    let index = index.get_integer(context)?;
		    let enumeration = self.stack.pop().expect("stack is empty");
		    let enumeration = enumeration.get_enum(context)?;
		    self.stack.push(enumeration.get_member(index.to_u64().unwrap() as usize, context)?.clone());
		    self.pc += 1;
		},
		RawBytecode::EnumStore => {
		    let index = self.stack.pop().expect("stack is empty");
		    let index = index.get_integer(context)?;
		    let value = self.stack.pop().expect("stack is empty");
		    let mut enumeration = self.stack.pop().expect("stack is empty");
		    let enumeration = enumeration.get_enum_mut(context)?;
		    enumeration.set_member(index.to_u64().unwrap() as usize, value.clone(), context)?;
		    self.pc += 1;
		},
	    }
	}
	Ok(None)
    }
}

