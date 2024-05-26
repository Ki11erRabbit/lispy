use std::io::Read;
use std::io::Write;
use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionShape;
use crate::interpreter::value::Value;
use crate::interpreter::context::Context;
use crate::interpreter::module::Module;
use crate::interpreter::kwargs::Kwargs;


fn stdlib_open_shape() -> FunctionShape {
    FunctionShape::new(vec!["filename".to_string(), "mode".to_string()])
}

fn stdlib_open(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let filename = if let Some(filename) = args.get(0) {
	filename.get_string(context)?.clone()
    } else if let Some(filename) = keyword_args.get("filename") {
	filename.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","open"], "filename is not provided", context)));
    };

    let mode = if let Some(mode) = args.get(1) {
	mode.get_string(context)?.clone()
    } else if let Some(mode) = keyword_args.get("mode") {
	mode.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","open"], "mode is not provided", context)));
    };
    let mut write = false;
    let mut create = false;
    let mut append = false;
    for c in mode.chars() {
	match c {
	    'w' => write = true,
	    'r' => {},
	    'a' => append = true,
	    '+' => {},
	    'x' => create = true,
	    _ => return Err(Box::new(Exception::new(&vec!["file","open"], "invalid mode", context))),
	}
    }

    let file = std::fs::OpenOptions::new().write(write).create(create).append(append).open(filename).expect("file open error");
    let file = Box::new(Some(file));
    let file = Value::new_rust_value(file, context);
    Ok(file)
}

fn stdlib_read_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["file".to_string()])
}

fn stdlib_read_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let mut file = if let Some(file) = args.get(0) {
	file.clone()
    } else if let Some(file) = keyword_args.get("file") {
	file.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","read-string"], "file is not provided", context)));
    };

    let file = file.get_rust_value_mut(context)?;
    let file = file.downcast_mut::<Option<std::fs::File>>().ok_or(Box::new(Exception::new(&vec!["file","read-string"], "file is not a file", context)))?;
    let file = file.as_mut().ok_or(Box::new(Exception::new(&vec!["file","read-string"], "file is closed", context)))?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| Box::new(Exception::new(&vec!["file","read-string"], "file read error", context)))?;
    Ok(Value::new_string(&content, context))
}

fn stdlib_write_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["file".to_string(), "content".to_string()])
}

fn stdlib_write_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let mut file = if let Some(file) = args.get(0) {
	file.clone()
    } else if let Some(file) = keyword_args.get("file") {
	file.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","write-string"], "file is not provided", context)));
    };

    let content = if let Some(content) = args.get(1) {
	content.get_string(context)?.clone()
    } else if let Some(content) = keyword_args.get("content") {
	content.get_string(context)?.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","write-string"], "content is not provided", context)));
    };

    let file = file.get_rust_value_mut(context)?;
    let file = file.downcast_mut::<Option<std::fs::File>>().ok_or(Box::new(Exception::new(&vec!["file","write-string"], "file is not a file", context)))?;
    let file = file.as_mut().ok_or(Box::new(Exception::new(&vec!["file","write-string"], "file is closed", context)))?;
    file.write_all(content.as_bytes()).map_err(|_| Box::new(Exception::new(&vec!["file","write-string"], "file write error", context)))?;
    Ok(Value::new_nil())
}

fn stdlib_close_shape() -> FunctionShape {
    FunctionShape::new(vec!["file".to_string()])
}

fn stdlib_close(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let mut file = if let Some(file) = args.get(0) {
	file.clone()
    } else if let Some(file) = keyword_args.get("file") {
	file.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["file","close"], "file is not provided", context)));
    };

    let file = file.get_rust_value_mut(context)?;
    let file = file.downcast_mut::<Option<std::fs::File>>().ok_or(Box::new(Exception::new(&vec!["file","close"], "file is not a file", context)))?;
    let _ = file.take().ok_or(Box::new(Exception::new(&vec!["file","close"], "file is closed", context)))?;

    Ok(Value::new_nil())
}

pub fn get_file_library(context: &mut Context) -> Module {
    let submodules = HashMap::new();
    context.push_frame(None);

    context.define("open", Value::new_function(Function::Native(stdlib_open, stdlib_open_shape()), context));
    context.define("read-string", Value::new_function(Function::Native(stdlib_read_string, stdlib_read_string_shape()), context));
    context.define("write-string", Value::new_function(Function::Native(stdlib_write_string, stdlib_write_string_shape()), context));
    context.define("close", Value::new_function(Function::Native(stdlib_close, stdlib_close_shape()), context));
		   
    let frame = context.pop_frame().expect("pop error");
    
    Module::new_loaded(submodules, frame)
}
