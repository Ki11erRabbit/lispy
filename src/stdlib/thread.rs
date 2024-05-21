use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionShape;
use crate::interpreter::value::Value;
use crate::interpreter::context::Context;
use crate::interpreter::module::Module;
use crate::interpreter::InterpreterResult;


fn stdlib_spawn_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string()])
}

fn stdlib_spawn(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let function = if let Some(function) = args.get(0) {
	function.clone()
    } else if let Some(function) = keyword_args.get("function") {
	function.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["thread","spawn"], "function is not provided", context)));
    };

    let mut new_context = context.clone();
    new_context.push_frame(Some(context.copy_frame_at(0)));

    let handle = std::thread::spawn(move || {
	function.protect();
        let mut new_context = new_context;
        let function = function.get_function(&new_context).expect("function is not a function");
        function.call(&vec!["<procedure>".to_string()], &vec![], &mut new_context)
    });

    let handle = Box::new(Some(handle));

    let output = Value::new_rust_value(handle, context);

    Ok(output)
}

fn stdlib_join_shape() -> FunctionShape {
    FunctionShape::new(vec!["thread".to_string()])
}

fn stdlib_join(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let mut handle = if let Some(handle) = args.get(0) {
	handle.clone()
    } else if let Some(handle) = keyword_args.get("handle") {
	handle.clone()
    } else {
	return Err(Box::new(Exception::new(&vec!["thread","join"], "thread handle was not provided", context)));
    };

    let handle = handle.get_rust_value_mut(context)?;

    let mut handle = handle.downcast_mut::<Box<Option<std::thread::JoinHandle<InterpreterResult>>>>();
    let handle = handle.as_mut().expect("downcast error");

    Ok(handle.take().unwrap().join().expect("join error")?.unwrap())
}



pub fn get_thread_library(context: &mut Context) -> Module {
    let submodules = HashMap::new();
    context.push_frame(None);

    context.define("spawn", Value::new_function(Function::Native(stdlib_spawn, stdlib_spawn_shape()), context));
    context.define("join", Value::new_function(Function::Native(stdlib_join, stdlib_join_shape()), context));





    let frame = context.pop_frame().expect("pop error");

    Module::new_loaded(submodules, frame)
}
