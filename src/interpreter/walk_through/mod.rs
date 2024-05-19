use crate::parser::{File, Sexpr, Atom};
use super::context::Context;
use super::value::{Value, Function};

fn unbox<T>(value: Box<[T]>) -> Box<[T]> {
	value
}

pub fn run(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();

    for sexpr in file {
	let value = walk_through(&sexpr, &mut context)?;
	//println!("{:?}", value);
    }
    Ok(())
}




fn walk_through(sexpr: &Sexpr, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match sexpr {
	Sexpr::Atom(atom) => {
	    match atom {
		Atom::String(s) => {
		    Ok(Some(Value::new_string(s)))
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
			None => todo!("error"),
		    }
		}
		Atom::Keyword(k) => {
		    todo!("keyword error")
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
			todo!("error");
		    }
		}
	    }
	    Ok(Some(Value::new_list(output)))
	},
	Sexpr::List(list) => {
	    walk_through_list(list, context)
	}
    }
}

fn walk_through_list(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
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
	    _ => walk_through_call(list, context),
	}
    } else {
	todo!("error");
    }
}

fn walk_through_define(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(name)), value] => {
	    let value = walk_through(value, context)?;
	    match value {
		Some(value) => {
		    context.define(&name[0], value);
		    Ok(None)
		}
		None => {
		    todo!("error");
		}
	    }
	}
	[_, Sexpr::List(header), body] => {
	    let name = match &header[0] {
		Sexpr::Atom(Atom::Symbol(s)) => &s[0],
		_ => todo!("error"),
	    };
	    let args = header.iter().skip(1).map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => s[0].clone(),
		_ => todo!("error"),
	    }).collect::<Vec<String>>();

	    let function = Function::Tree(args, body.clone(), context.copy_frame());
	    context.define(&name, Value::new_function(function));
	    Ok(None)
	},
	_ => todo!("error"),
    }
}

fn walk_through_lambda(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match list.as_slice() {
	[_, Sexpr::List(header), body] => {
	    let args = header.iter().map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => s[0].clone(),
		_ => todo!("error"),
	    }).collect::<Vec<String>>();

	    let function = Function::Tree(args, body.clone(), context.copy_frame());
	    Ok(Some(Value::new_function(function)))
	},
	_ => todo!("error"),
    }
}

fn walk_through_if(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
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
		_ => todo!("error"),
	    }
	},
	_ => todo!("error"),
    }
}

fn walk_through_set(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(name)), value] => {
	    let value = walk_through(value, context)?;
	    match value {
		Some(value) => {
		    context.define(&name[0], value);
		    Ok(None)
		}
		None => {
		    todo!("error");
		}
	    }
	}
	_ => todo!("error"),
    }
}

fn walk_through_let(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
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
					todo!("error");
				    }
				}
			    }
			    _ => todo!("error"),
			}
		    }
		    _ => todo!("error"),
		}
	    }
	    let value = walk_through(body, context);
	    context.pop_frame();
	    value
	}
	_ => todo!("error"),
    }
}

fn walk_through_begin(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let mut output = None;
    for sexpr in list.iter().skip(1) {
	output = walk_through(sexpr, context)?;
    }
    Ok(output)
}

fn walk_through_import(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::String(path))] => {
	    todo!("import");
	}
	_ => todo!("error"),
    }
}

fn walk_through_module(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::String(ref path))] => {
	    todo!("module");
	}
	_ => todo!("error"),
    }
}

fn walk_through_call(list: &Vec<Sexpr>, context: &mut Context) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    if let Sexpr::Atom(Atom::Symbol(name)) = &list[0] {
	let function = match context.get(&name) {
	    Some(f) => f.get_function()?.clone(),
	    None => todo!("error"),
	};

	match function {
	    Function::Tree(args, body, frame) => {
		context.push_frame(Some(frame.clone()));
		for (arg, value) in args.iter().zip(list.iter().skip(1)) {
		    let value = walk_through(value, context)?;
		    match value {
			Some(value) => {
			    context.define(arg, value);
			}
			None => {
			    todo!("error");
			}
		    }
		}
		let value = walk_through(&body, context);
		context.pop_frame();
		value
	    }
	}
    } else {
	todo!("error");
    }
}
