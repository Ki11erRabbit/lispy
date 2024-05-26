use crate::interpreter::HelperResult;
use crate::interpreter::bytecode::RawBytecode;

use crate::interpreter::bytecode::Bytecode;
use crate::interpreter::{context::Context, Exception};
use super::Value;
use super::function::{Function, FunctionShape};

pub struct Enum {
    name_index: usize,
    variant_index: usize,
    members: Box<[Value]>,
}

impl Enum {
    pub fn new(name_index: usize, variant_index: usize, members: Box<[Value]>) -> Self {
	Enum {
	    name_index,
	    variant_index,
	    members,
	}
    }

    pub fn get_member(&self, index: usize, context: &Context) -> HelperResult<&Value> {
	let empty: Vec<&str> = Vec::new();
	if index >= self.members.len() {
	    Err(Box::new(Exception::new(&empty, "index out of bounds", context)))
	} else {
	    Ok(&self.members[index])
	}
    }

    pub fn set_member(&mut self, index: usize, value: Value, context: &Context) -> HelperResult<()> {
	let empty: Vec<&str> = Vec::new();
	if index >= self.members.len() {
	    Err(Box::new(Exception::new(&empty, "index out of bounds", context)))
	} else {
	    self.members[index] = value;
	    Ok(())
	}
    }

    pub fn get_name_index(&self) -> usize {
	self.name_index
    }

    pub fn get_variant_index(&self) -> usize {
	self.variant_index
    }

    pub fn get_members(&self) -> &Box<[Value]> {
	&self.members
    }

    pub fn create_functions(module_name: &Vec<String>, name: &Vec<String>, variants: &Vec<Vec<String>>, member_names: Vec<Vec<Vec<String>>>, context: &mut Context) {
	if variants.len() != member_names.len() {
	    panic!("variants and member_names must have the same length");
	}
	let type_name = module_name.iter().chain(name.iter()).map(|s| s.clone()).collect();
	context.get_or_create_type_symbol_enum(&type_name);
	for (variant, member_names) in variants.iter().zip(member_names.iter()) {
	    let variant_name: Vec<String> = module_name.iter().chain(variant.iter()).map(|s| s.clone()).collect();
	    context.get_or_create_type_symbol_enum(&variant);
	    let constructor_shape = FunctionShape::new(member_names.iter().map(|v| v.join(".")).collect());
	    let mut constructor_bytecode = member_names.iter().map(|s| vec![
		Bytecode::new(RawBytecode::PushSymbol(s.clone()), 0, 0),
		Bytecode::new(RawBytecode::Load, 0, 0),
	    ]).flatten().collect::<Vec<Bytecode>>();
	    constructor_bytecode.push(Bytecode::new(RawBytecode::PushSymbol(variant_name.clone()), 0, 0));
	    constructor_bytecode.push(Bytecode::new(RawBytecode::PushSymbol(type_name.clone()), 0, 0));
	    constructor_bytecode.push(Bytecode::new(RawBytecode::MakeEnum(member_names.len()), 0, 0));
	    constructor_bytecode.push(Bytecode::new(RawBytecode::Return, 0, 0));
	    let constructor = Function::Bytecode(member_names.iter().map(|v| v.join(".")).collect(), constructor_bytecode, constructor_shape);

	    let constructor_name = name.last().cloned().unwrap() + "-" + &variant.last().cloned().unwrap();
	    
	    
	    context.define(&constructor_name, Value::new_function(constructor, context));
	}



	for (x, member_names) in member_names.iter().enumerate() {
	    let mut accessor_shapes = Vec::new();
	    let mut accessor_member_names = member_names.iter().map(|m| (*m).clone()).collect::<Vec<Vec<String>>>();
	    for member in accessor_member_names.iter_mut() {
		let new_end = name.last().cloned().unwrap() + "-" + &variants[x].join(".") + "-" + member.last_mut().unwrap();
		*member.last_mut().unwrap() = new_end;
		accessor_shapes.push(FunctionShape::new(vec![name[0].clone()]));
	    }

	    for (i, member) in accessor_member_names.iter().enumerate() {
		let accessor_bytecode = vec![
		    Bytecode::new(RawBytecode::PushSymbol(name.clone()), 0, 0),
		    Bytecode::new(RawBytecode::Load, 0, 0),
		    Bytecode::new(RawBytecode::PushInteger(i.to_string()), 0, 0),
		    Bytecode::new(RawBytecode::EnumAccess, 0, 0),
		    Bytecode::new(RawBytecode::Return, 0, 0),
		];
		let accessor = Function::Bytecode(vec![name[0].clone()], accessor_bytecode, accessor_shapes[i].clone());
		context.define(&member[0], Value::new_function(accessor, context));
	    }

	    let mut setter_shapes = Vec::new();
	    let mut setter_member_names = member_names.iter().map(|m| (*m).clone()).collect::<Vec<Vec<String>>>();
	    for member in setter_member_names.iter_mut() {
		let new_end = name.last().cloned().unwrap() + "-" + &variants[x].join(".") + "-" + member.last_mut().unwrap() + "-set!";
		*member.last_mut().unwrap() = new_end;
		setter_shapes.push(FunctionShape::new(vec![name[0].clone(), "value".to_string()]));
	    }

	    for (i, member) in setter_member_names.iter().enumerate() {
		let setter_bytecode = vec![
		    Bytecode::new(RawBytecode::PushSymbol(name.clone()), 0, 0),
		    Bytecode::new(RawBytecode::Load, 0, 0),
		    Bytecode::new(RawBytecode::PushSymbol(vec!["value".to_string()]), 0, 0),
		    Bytecode::new(RawBytecode::Load, 0, 0),
		    Bytecode::new(RawBytecode::PushInteger(i.to_string()), 0, 0),
		    Bytecode::new(RawBytecode::EnumStore, 0, 0),
		    Bytecode::new(RawBytecode::Return, 0, 0),
		];
		let setter = Function::Bytecode(vec![name[0].clone(), "value".to_string()], setter_bytecode, setter_shapes[i].clone());
		context.define(&member[0], Value::new_function(setter, context));
	    }
	}
    }

}
