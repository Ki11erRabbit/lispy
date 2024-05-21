use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::value::Function;
use crate::interpreter::value::FunctionShape;
use crate::interpreter::value::Value;
use crate::interpreter::context::Context;
use crate::interpreter::module::Module;


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
        function.protect();
        function.call(&vec!["<procedure>".to_string()], &vec![], &mut new_context).expect("call error");
    });

    let handle = Box::new(handle);

    let output = Value::new_rust_value(handle, context);

    Ok(output)
}




pub fn get_thread_library(context: &mut Context) -> Module {
    let submodules = HashMap::new();
    context.push_frame(None);

    context.define("spawn", Value::new_function(Function::Native(stdlib_spawn, stdlib_spawn_shape()), context));





    let frame = context.pop_frame().expect("pop error");

    Module::new_loaded(submodules, frame)
}
