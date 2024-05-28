
use crate::interpreter::HelperResult;
use crate::interpreter::bytecode::RawBytecode;

use crate::interpreter::bytecode::Bytecode;
use crate::interpreter::{context::Context, Exception};
use super::Value;
use super::function::{Function, FunctionShape};

pub struct Struct {
    name_index: usize,
    members: Box<[Value]>,
}

impl Struct {
    pub fn new(name_index: usize, members: Box<[Value]>) -> Self {
	Struct {
	    name_index,
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

    pub fn create_functions(module_name: &Vec<String>, name: &Vec<String>, member_names: Vec<Vec<String>>, context: &mut Context) {
	let type_name = module_name.iter().chain(name.iter()).map(|s| s.clone()).collect();
	context.get_or_create_type_symbol(&type_name);
	let constructor_shape = FunctionShape::new(member_names.iter().map(|v| v.join(".")).collect());
	let mut constructor_bytecode = member_names.iter().map(|s| vec![
	    Bytecode::new(RawBytecode::PushSymbol(s.clone()), 0, 0),
	    Bytecode::new(RawBytecode::Load, 0, 0),
	]).rev().flatten().collect::<Vec<Bytecode>>();
	constructor_bytecode.push(Bytecode::new(RawBytecode::PushSymbol(type_name.clone()), 0, 0));
	constructor_bytecode.push(Bytecode::new(RawBytecode::MakeStruct(member_names.len()), 0, 0));
	constructor_bytecode.push(Bytecode::new(RawBytecode::Return, 0, 0));
	let constructor = Function::Bytecode(member_names.iter().map(|v| v.join(".")).collect(), constructor_bytecode, constructor_shape);
	context.define(&name[0], Value::new_function(constructor, context));

	let mut accessor_shapes = Vec::new();
	let mut accessor_member_names = member_names.iter().map(|m| (*m).clone()).collect::<Vec<Vec<String>>>();
	for member in accessor_member_names.iter_mut() {
	    let new_end = name.last().cloned().unwrap() + "-" + member.last_mut().unwrap();
	    *member.last_mut().unwrap() = new_end;
	    accessor_shapes.push(FunctionShape::new(vec![name[0].clone()]));
	}

	for (i, member) in accessor_member_names.iter().enumerate() {
	    let accessor_bytecode = vec![
		Bytecode::new(RawBytecode::PushSymbol(name.clone()), 0, 0),
		Bytecode::new(RawBytecode::Load, 0, 0),
		Bytecode::new(RawBytecode::PushInteger(i.to_string()), 0, 0),
		Bytecode::new(RawBytecode::StructAccess, 0, 0),
		Bytecode::new(RawBytecode::Return, 0, 0),
	    ];
	    let accessor = Function::Bytecode(vec![name[0].clone()], accessor_bytecode, accessor_shapes[i].clone());
	    context.define(&member[0], Value::new_function(accessor, context));
	}

	let mut setter_shapes = Vec::new();
	let mut setter_member_names = member_names.iter().map(|m| (*m).clone()).collect::<Vec<Vec<String>>>();
	for member in setter_member_names.iter_mut() {
	    let new_end = name.last().cloned().unwrap() + "-" + member.last_mut().unwrap() + "-set!";
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
		Bytecode::new(RawBytecode::StructStore, 0, 0),
		Bytecode::new(RawBytecode::Return, 0, 0),
	    ];
	    let setter = Function::Bytecode(vec![name[0].clone(), "value".to_string()], setter_bytecode, setter_shapes[i].clone());
	    context.define(&member[0], Value::new_function(setter, context));
	} 
    }
}
