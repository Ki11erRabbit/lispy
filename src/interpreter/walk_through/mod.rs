use crate::parser::{File, Sexpr, Atom};
use super::Exception;
use super::context::Context;
use super::value::{Value, Function, FunctionShape};
use super::InterpreterResult;

pub fn run(file: File, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {

    for sexpr in file {
	walk_through(&sexpr, context)?;
	//println!("{:?}", value);
	if crate::gc::is_gc_on() {
	    println!("gc");
	    context.garbage_collect();
	}
    }
    Ok(())
}




fn walk_through(sexpr: &Sexpr, context: &mut Context) -> InterpreterResult {
    match sexpr {
	Sexpr::Atom(atom) => {
	    match atom {
		Atom::String(s) => {
		    Ok(Some(Value::new_string(s, context)))
		}
		Atom::Integer(i) => {
		    Ok(Some(Value::new_integer(&i)))
		}
		Atom::Float(f) => {
		    Ok(Some(Value::new_float(*f)))
		}
		Atom::Boolean(b) => {
		    Ok(Some(Value::new_boolean(*b)))
		}
		Atom::Symbol(s) => {
		    match context.get(&s) {
			Some(value) => Ok(Some(value.clone())),
			None => Err(Box::new(Exception::new(s.clone(), "not bound".to_string()))),
		    }
		}
		Atom::Keyword(_) => {
		    Err(Box::new(Exception::new(Vec::new(), "keyword not allowed here".to_string())))
		}
		Atom::Char(c) => {
		    Ok(Some(Value::new_char(*c)))
		}
		Atom::Null => {
		    Ok(Some(Value::new_nil()))
		}
	    }
	},
	Sexpr::QuotedList(list) => {
	    let mut output = Vec::new();
	    for sexpr in list {
		match walk_through(sexpr, context)? {
		    Some(value) => {
			output.push(value);
		    }
		    None => {
			return Err(Box::new(Exception::new(Vec::new(), "expression didn't result in a value".to_string())));
		    }
		}
	    }
	    let mut pair = Value::new_pair(output.pop().unwrap(), Value::new_nil(), context);
	    for value in output.iter().rev() {
		pair = Value::new_pair(value.clone(), pair, context);
	    }
	    Ok(Some(pair))
	},
	Sexpr::VectorList(list) => {
	    let mut output = Vec::new();
	    for sexpr in list {
		match walk_through(sexpr, context)? {
		    Some(value) => {
			output.push(value);
		    }
		    None => {
			return Err(Box::new(Exception::new(Vec::new(), "expression didn't result in a value".to_string())));
		    }
		}
	    }
	    Ok(Some(Value::new_vector(output, context)))
	}
	Sexpr::List(list) => {
	    walk_through_list(list, context)
	}
    }
}

fn walk_through_list(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    if list.is_empty() {
	return Ok(None);
    }
    if let Sexpr::Atom(Atom::Symbol(s)) = &list[0] {
	match s[0].as_str() {
	    "define" => walk_through_define(list, context),
	    "lambda" => walk_through_lambda(list, context),
	    "if" => walk_through_if(list, context),
	    "set!" => walk_through_set(list, context),
	    "let" => walk_through_let(list, context),
	    "begin" => walk_through_begin(list, context),
	    "import" => walk_through_import(list, context),
	    "module" => walk_through_module(list, context),
	    "try" => walk_through_try(list, context),
	    "error" => walk_through_error(list, context),
	    _ => walk_through_call(list, context),
	}
    } else {
	Err(Box::new(Exception::new(Vec::new(), "unreachable".to_string())))
    }
}

fn walk_through_define(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(name)), value] => {
	    let value = walk_through(value, context)?;
	    match value {
		Some(value) => {
		    context.define(&name[0], value);
		    Ok(None)
		}
		None => {
		    Err(Box::new(Exception::new(vec!["define".to_string()], "expression didn't result in a value".to_string())))
		}
	    }
	}
	[_, Sexpr::List(header), body] => {
	    let name = match &header[0] {
		Sexpr::Atom(Atom::Symbol(s)) => &s[0],
		_ => return Err(Box::new(Exception::new(vec!["define".to_string()], "not a symbol".to_string())))
	    };
	    let args = header.iter().skip(1).map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => Ok(s[0].clone()),
		_ => Err(Box::new(Exception::new(vec!["define".to_string()], "not a symbol".to_string()))),
	    }).collect::<Vec<Result<String, Box<Exception>>>>();
	    let args = args.into_iter().collect::<Result<Vec<String>, Box<Exception>>>()?;

	    let function = Function::Tree(args.clone(), body.clone(), context.copy_frame(), FunctionShape::new(args));
	    let function = Value::new_function(function, context);
	    context.define(&name, function);
	    Ok(None)
	},
	_ => Err(Box::new(Exception::new(vec!["define".to_string()], "unusual syntax".to_string()))),
    }
}

fn walk_through_lambda(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::List(header), body] => {
	    let args = header.iter().map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => Ok(s[0].clone()),
		_ => Err(Box::new(Exception::new(vec!["lambda".to_string()], "not a symbol".to_string()))),
	    }).collect::<Vec<Result<String, Box<Exception>>>>();
	    let args = args.into_iter().collect::<Result<Vec<String>, Box<Exception>>>()?;

	    let function = Function::Tree(args.clone(), body.clone(), context.copy_frame(), FunctionShape::new(args));
	    Ok(Some(Value::new_function(function, context)))
	},
	_ => Err(Box::new(Exception::new(vec!["lambda".to_string()], "unusual syntax".to_string()))),
    }
}

fn walk_through_if(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, condition, consequent, alternate] => {
	    let condition = walk_through(condition, context)?;
	    match condition {
		Some(value) => {
		    if value.get_boolean()? {
			walk_through(consequent, context)
		    } else {
			walk_through(alternate, context)
		    }
		}
		_ => Err(Box::new(Exception::new(vec!["if".to_string()], "expression didn't result in a value".to_string())))
	    }
	},
	_ => Err(Box::new(Exception::new(vec!["if".to_string()], "unusual syntax".to_string()))),
    }
}

fn walk_through_set(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(name)), value] => {
	    let value = walk_through(value, context)?;
	    match value {
		Some(value) => {
		    context.define(&name[0], value);
		    Ok(None)
		}
		None => {
		    Err(Box::new(Exception::new(vec!["set!".to_string()], "expression didn't result in a value".to_string())))
		}
	    }
	}
	_ => Err(Box::new(Exception::new(vec!["set!".to_string()], "unusual syntax".to_string()))),
    }
}

fn walk_through_let(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::List(bindings), body] => {
	    context.push_frame(None);
	    for binding in bindings {
		match binding {
		    Sexpr::List(sets) => {
			match sets.as_slice() {
			    [Sexpr::Atom(Atom::Symbol(name)), value] => {
				let value = walk_through(value, context)?;
				match value {
				    Some(value) => {
					context.define(&name[0], value);
				    }
				    None => {
					return Err(Box::new(Exception::new(vec!["let".to_string()], "expression didn't result in a value".to_string())));
				    }
				}
			    }
			    _ => return Err(Box::new(Exception::new(vec!["let".to_string()], "unusual syntax".to_string()))),
			}
		    }
		    _ => return Err(Box::new(Exception::new(vec!["let".to_string()], "unusual syntax".to_string()))),
		}
	    }
	    let value = walk_through(body, context);
	    context.pop_frame();
	    value
	}
	_ => Err(Box::new(Exception::new(vec!["let".to_string()], "unusual syntax".to_string())))
    }
}

fn walk_through_begin(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    let mut output = None;
    for sexpr in list.iter().skip(1) {
	output = walk_through(sexpr, context)?;
    }
    Ok(output)
}

fn walk_through_import(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::String(path))] => {
	    todo!("import");
	}
	_ => Err(Box::new(Exception::new(vec!["import".to_string()], "unusual syntax".to_string())))
    }
}

fn walk_through_module(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::String(ref path))] => {
	    todo!("module");
	}
	_ => Err(Box::new(Exception::new(vec!["module".to_string()], "unusual syntax".to_string())))
    }
}

fn walk_through_try(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, body, handlers] => {
	    let value = walk_through(body, context);
	    match value {
		Ok(value) => return Ok(value),
		Err(e) => {
		    let Sexpr::List(handlers) = handlers else {
			return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())));
		    };
		    for handler in handlers {
			if let Sexpr::List(handler) = handler {
			    if handler.len() != 2 {
				return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())));
			    }
			    let Sexpr::List(clause) = &handler[0] else {
				return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())));
			    };
			    let Sexpr::Atom(Atom::Symbol(keyword)) = &clause[0] else {
				return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())));
			    };
			    
			    match keyword[0].as_str() {
				"catch" => {},
				_ => return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string()))),
			    }
			    let Sexpr::Atom(Atom::Symbol(who)) = &clause[1] else {
				return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())));
			    };
					
			    match handler.as_slice() {
				[_, body] => {
				    if e.get_who() == who {
					return walk_through(body, context);
				    }
				},
				_ => return Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())))
			    }
			}
		    }
		    Err(e)
		}
	    }
	}
	_ => Err(Box::new(Exception::new(vec!["try".to_string()], "unusual syntax".to_string())))
    }
}

fn walk_through_error(list: &Vec<Sexpr>, _: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(who)), Sexpr::Atom(Atom::String(message))] => {
	    Err(Box::new(Exception::new(who.clone(), message.clone())))
	}
	_ => Err(Box::new(Exception::new(vec!["error".to_string()], "unusual syntax".to_string()))),
    }
}

fn walk_through_call(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    if let Sexpr::Atom(Atom::Symbol(name)) = &list[0] {
	let function = match context.get(&name) {
	    Some(f) => f.get_function()?.clone(),
	    None => return Err(Box::new(Exception::new(name.clone(), "not bound".to_string())))
	};

	match function {
	    Function::Tree(fun_args, body, frame, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = std::collections::HashMap::new();
		let mut iterator = list.iter().skip(1);
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match walk_through(value, context)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(vec![name[0].clone()], "expression didn't result in a value".to_string())));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(vec![name[0].clone()], "unusual syntax".to_string())));
			    }
			}
			s => {
			    match walk_through(s, context)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(vec![name[0].clone()], "expression didn't result in a value".to_string())));
				}
			    }
			}
		    }
		}

		shape.check(&name, &args, &keyword_args)?;

		context.push_frame(Some(frame.clone()));

		for (arg, value) in fun_args.iter().zip(args.iter()) {
		    context.define(arg, value.clone());
		}
		for (arg, value) in keyword_args.iter() {
		    context.define(arg, value.clone());
		}
		
		let value = walk_through(&body, context);
		context.pop_frame();
		value
	    },
	    Function::Native(f, shape) => {
		let mut args = Vec::new();
		let mut keyword_args = std::collections::HashMap::new();
		let mut iterator = list.iter().skip(1);
		while let Some(sexpr) = iterator.next() {
		    match sexpr {
			Sexpr::Atom(Atom::Keyword(k)) => {
			    if let Some(value) = iterator.next() {
				match walk_through(value, context)? {
				    Some(value) => {
					keyword_args.insert(k.clone(), value);
				    }
				    None => {
					return Err(Box::new(Exception::new(vec![name[0].clone()], "expression didn't result in a value".to_string())));
				    }
				}
			    } else {
				return Err(Box::new(Exception::new(vec![name[0].clone()], "unusual syntax".to_string())));
			    }
			}
			s => {
			    match walk_through(s, context)? {
				Some(value) => {
				    args.push(value);
				}
				None => {
				    return Err(Box::new(Exception::new(vec![name[0].clone()], "expression didn't result in a value".to_string())));
				}
			    }
			}
		    }
		}

		shape.check(&name, &args, &keyword_args)?;
		
		Ok(Some(f(context, args, keyword_args)?))
	    }
	}
    } else {
	Err(Box::new(Exception::new(Vec::new(), "unreachable".to_string())))
    }
}
