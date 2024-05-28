use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::value::function::Function;
use crate::interpreter::value::function::FunctionShape;
use crate::interpreter::value::Value;
use crate::interpreter::context::Context;
use crate::interpreter::module::Module;
use crate::interpreter::InterpreterResult;
use crate::interpreter::kwargs::Kwargs;




fn stdlib_mpsc_channel_shape() -> FunctionShape {
    FunctionShape::new(vec![])
}

fn stdlib_mpsc_channel(context: &mut Context, _: Vec<Value>, _: Kwargs) -> HelperResult<Value> {
    let (tx, rx): (std::sync::mpsc::Sender<Value>, std::sync::mpsc::Receiver<Value>)  = std::sync::mpsc::channel();
    let tx = Box::new(tx);
    let rx = Box::new(rx);
    let tx = Value::new_rust_value(tx, context);
    let rx = Value::new_rust_value(rx, context);
    let pair = Value::new_pair(tx, rx, context);
    Ok(pair)
}


fn stdlib_mpsc_send_shape() -> FunctionShape {
    FunctionShape::new(vec!["channel".to_string(), "value".to_string()])
}

fn stdlib_mpsc_send(context: &mut Context, args: Vec<Value>, kwargs: Kwargs) -> HelperResult<Value> {
    let (tx, value) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	if let Some(value) = kwargs.get("value") {
	    (args[0].clone(), value.clone())
	} else {
	    return Err(Box::new(Exception::new(&vec!["mpsc","send"], "value is not provided", context)));
	}
    } else if args.len() == 0 {
	if let Some(channel) = kwargs.get("channel") {
	    if let Some(value) = kwargs.get("value") {
		(channel.clone(), value.clone())
	    } else {
		return Err(Box::new(Exception::new(&vec!["mpsc","send"], "value is not provided", context)));
	    }
	} else {
	    return Err(Box::new(Exception::new(&vec!["mpsc","send"], "channel is not provided", context)));
	}
    } else {
	return Err(Box::new(Exception::new(&vec!["mpsc","send"], "invalid arguments", context)));
    };
    let tx = tx.get_rust_value(context)?;
    let tx = tx.downcast_ref::<std::sync::mpsc::Sender<Value>>().expect("tx is not a Sender");
    tx.send(value).expect("send error");
    Ok(Value::new_nil())
}

fn stdlib_mpsc_receive_shape() -> FunctionShape {
    FunctionShape::new(vec!["channel".to_string()])
}

fn stdlib_mpsc_receive(context: &mut Context, args: Vec<Value>, kwargs: Kwargs) -> HelperResult<Value> {
    let channel = if args.len() == 1 {
	args[0].clone()
    } else if args.len() == 0 {
	if let Some(channel) = kwargs.get("channel") {
	    channel.clone()
	} else {
	    return Err(Box::new(Exception::new(&vec!["mpsc","receive"], "channel is not provided", context)));
	}
    } else {
	return Err(Box::new(Exception::new(&vec!["mpsc","receive"], "invalid arguments", context)));
    };
    let channel = channel.get_rust_value(context)?;
    let channel = channel.downcast_ref::<std::sync::mpsc::Receiver<Value>>().expect("channel is not a Receiver");
    let value = channel.recv().expect("receive error");
    Ok(value)
}

fn stdlib_mpsc_try_receive_shape() -> FunctionShape {
    FunctionShape::new(vec!["channel".to_string()])
}

fn stdlib_mpsc_try_receive(context: &mut Context, args: Vec<Value>, kwargs: Kwargs) -> HelperResult<Value> {
    let channel = if args.len() == 1 {
	args[0].clone()
    } else if args.len() == 0 {
	if let Some(channel) = kwargs.get("channel") {
	    channel.clone()
	} else {
	    return Err(Box::new(Exception::new(&vec!["mpsc","receive"], "channel is not provided", context)));
	}
    } else {
	return Err(Box::new(Exception::new(&vec!["mpsc","receive"], "invalid arguments", context)));
    };
    let channel = channel.get_rust_value(context)?;
    let channel = channel.downcast_ref::<std::sync::mpsc::Receiver<Value>>().expect("channel is not a Receiver");
    match channel.try_recv() {
	Ok(value) => Ok(value),
	Err(std::sync::mpsc::TryRecvError::Empty) => Ok(Value::new_nil()),
	Err(std::sync::mpsc::TryRecvError::Disconnected) => Err(Box::new(Exception::new(&vec!["mpsc","try-receive"], "channel is disconnected", context))),
	}
}

pub fn get_sync_library(context: &mut Context) -> Module {
    let submodules = HashMap::new();
    context.push_frame(None);

    context.define("mpsc-channel", Value::new_function(Function::Native(stdlib_mpsc_channel, stdlib_mpsc_channel_shape()), context));
    context.define("mpsc-send", Value::new_function(Function::Native(stdlib_mpsc_send, stdlib_mpsc_send_shape()), context));
    context.define("mpsc-receive", Value::new_function(Function::Native(stdlib_mpsc_receive, stdlib_mpsc_receive_shape()), context));
    context.define("mpsc-try-receive", Value::new_function(Function::Native(stdlib_mpsc_try_receive, stdlib_mpsc_try_receive_shape()), context));
    

    let frame = context.pop_frame().expect("pop error");

    Module::new_loaded(submodules, frame)
}

