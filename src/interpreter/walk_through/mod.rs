use crate::parser::{File, Sexpr, Atom};
use super::Exception;
use super::context::Context;
use super::module::Module;
use super::value::{Value, Function, FunctionShape};
use super::InterpreterResult;

pub fn run(file: File, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {

    for sexpr in file {
	match walk_through(&sexpr, context) {
	    Err(e) => {
		println!("{}", e);
		break;
	    }
	    _ => {},
	}
	if crate::gc::is_gc_on() {
	    context.garbage_collect();
	}
    }
    Ok(())
}




pub fn walk_through(sexpr: &Sexpr, context: &mut Context) -> InterpreterResult {
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
                        None => Err(Box::new(Exception::new(s, "not bound", context))),
                    }
                }
                Atom::QuotedSymbol(s) => {
                    Ok(Some(Value::new_symbol(s.clone(), context)))
                }
                Atom::Keyword(_) => {
                    let empty: Vec<&str> = Vec::new();
                    Err(Box::new(Exception::new(&empty, "keyword not allowed here", context)))
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
                    let empty: Vec<&str> = Vec::new();
                    return Err(Box::new(Exception::new(&empty, "expression didn't result in a value", context)));
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
                        let empty: Vec<&str> = Vec::new();
                        return Err(Box::new(Exception::new(&empty, "expression didn't result in a value", context)));
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
	    let empty: Vec<&str> = Vec::new();
	    Err(Box::new(Exception::new(&empty, "unreachable", context)))
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
		    Err(Box::new(Exception::new(&vec!["define"], "expression didn't result in a value", context)))
		}
	    }
	}
	[_, Sexpr::List(header), body] => {
	    let name = match &header[0] {
		Sexpr::Atom(Atom::Symbol(s)) => &s[0],
		_ => return Err(Box::new(Exception::new(&vec!["define"], "not a symbol", context)))
	    };
	    let args = header.iter().skip(1).map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => Ok(s[0].clone()),
		_ => Err(Box::new(Exception::new(&vec!["define"], "not a symbol", context))),
	    }).collect::<Vec<Result<String, Box<Exception>>>>();
	    let args = args.into_iter().collect::<Result<Vec<String>, Box<Exception>>>()?;

	    let function = Function::Tree(args.clone(), body.clone(), context.copy_frame(), FunctionShape::new(args));
	    let function = Value::new_function(function, context);
	    context.define(&name, function);
	    Ok(None)
	},
	_ => Err(Box::new(Exception::new(&vec!["define"], "unusual syntax", context))),
    }
}

fn walk_through_lambda(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, Sexpr::List(header), body] => {
	    let args = header.iter().map(|sexpr| match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => Ok(s[0].clone()),
		_ => Err(Box::new(Exception::new(&vec!["lambda"], "not a symbol", context))),
	    }).collect::<Vec<Result<String, Box<Exception>>>>();
	    let args = args.into_iter().collect::<Result<Vec<String>, Box<Exception>>>()?;

	    let function = Function::Tree(args.clone(), body.clone(), context.copy_frame(), FunctionShape::new(args));
	    Ok(Some(Value::new_function(function, context)))
	},
	_ => Err(Box::new(Exception::new(&vec!["lambda"], "unusual syntax", context))),
    }
}

fn walk_through_if(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, condition, consequent, alternate] => {
	    let condition = walk_through(condition, context)?;
	    match condition {
		Some(value) => {
		    if value.get_boolean(context)? {
			walk_through(consequent, context)
		    } else {
			walk_through(alternate, context)
		    }
		}
		_ => Err(Box::new(Exception::new(&vec!["if"], "expression didn't result in a value", context)))
	    }
	},
	_ => Err(Box::new(Exception::new(&vec!["if"], "unusual syntax", context))),
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
		    Err(Box::new(Exception::new(&vec!["set!"], "expression didn't result in a value", context)))
		}
	    }
	}
	_ => Err(Box::new(Exception::new(&vec!["set!"], "unusual syntax", context))),
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
					return Err(Box::new(Exception::new(&vec!["let"], "expression didn't result in a value", context)));
				    }
				}
			    }
			    _ => return Err(Box::new(Exception::new(&vec!["let"], "unusual syntax", context))),
			}
		    }
		    _ => return Err(Box::new(Exception::new(&vec!["let"], "unusual syntax", context))),
		}
	    }
	    let value = walk_through(body, context);
	    context.pop_frame();
	    value
	}
	_ => Err(Box::new(Exception::new(&vec!["let"], "unusual syntax", context)))
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
	[_, path, name] => {
	    let path = walk_through(path, context)?.ok_or(Box::new(Exception::new(&vec!["import"], "not a string", context)))?;
	    let path = path.get_string(context)?;
	    let name = walk_through(name, context)?.ok_or(Box::new(Exception::new(&vec!["import"], "not a symbol", context)))?;
	    let name = name.get_symbol(context)?;

	    let module = Module::new(path);
	    let name = if name.len() > 1 {
		return Err(Box::new(Exception::new(&vec!["import"], "symbol must be singular", context)));
	    } else {
		&name[0]
	    };

	    context.add_module(&name, module);
	    Ok(None)
	}
	_ => Err(Box::new(Exception::new(&vec!["import"], "unusual syntax", context)))
    }
}

fn walk_through_module(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, name, ..] => {
	    let name = walk_through(name, context)?.ok_or(Box::new(Exception::new(&vec!["module"], "not a symbol", context)))?;
	    let name = name.get_symbol(context)?;

	    let name = if name.len() > 1 {
		    return Err(Box::new(Exception::new(&vec!["import"], "symbol must be singular", context)));
        } else {
		&name[0]
	    };

	    let mut new_context = context.clone();
	    new_context.push_frame(None);

	    let list = list.as_slice()[2..].to_vec();
	    run(File::new(list), &mut new_context).map_err(|_| Box::new(Exception::new(&vec!["module"], "error while loading module", context)))?;

	    let module = Module::new_from_context(new_context);

	    context.add_module(&name, module);
	    Ok(None)
	}
	_ => Err(Box::new(Exception::new(&vec!["module"], "unusual syntax", context)))
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
			return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)));
		    };
		    for handler in handlers {
			if let Sexpr::List(handler) = handler {
			    if handler.len() != 2 {
				return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)));
			    }
			    let Sexpr::List(clause) = &handler[0] else {
				return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)));
			    };
			    let Sexpr::Atom(Atom::Symbol(keyword)) = &clause[0] else {
				return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)));
			    };
			    
			    match keyword[0].as_str() {
				"catch" => {},
				_ => return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context))),
			    }
			    let who = walk_through(&clause[1], context)?.ok_or(Box::new(Exception::new(&vec!["try"], "not a symbol", context)))?;
			    let who = who.get_symbol(context)?;
					
			    match handler.as_slice() {
				[_, body] => {
				    if e.get_who(context) == who {
					return walk_through(body, context);
				    }
				},
				_ => return Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)))
			    }
			}
		    }
		    Err(e)
		}
	    }
	}
	_ => Err(Box::new(Exception::new(&vec!["try"], "unusual syntax", context)))
    }
}

fn walk_through_error(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    match list.as_slice() {
	[_, who, Sexpr::Atom(Atom::String(message))] => {
	    let who = walk_through(who, context)?.ok_or(Box::new(Exception::new(&vec!["error"], "not a symbol", context)))?;
	    let who = who.get_symbol(context)?;

	    Err(Box::new(Exception::new(&who, &message, context)))
	}
	_ => Err(Box::new(Exception::new(&vec!["error"], "unusual syntax", context))),
    }
}

fn walk_through_call(list: &Vec<Sexpr>, context: &mut Context) -> InterpreterResult {
    if let Sexpr::Atom(Atom::Symbol(name)) = &list[0] {
        let function = match context.get(&name) {
            Some(f) => {
                f.get_function(context)?.clone()
            },
            None => return Err(Box::new(Exception::new(&name, "not bound", context)))
        };

        function.call(&name, list, context)

    } else {
        let empty: Vec<&str> = Vec::new();
        Err(Box::new(Exception::new(&empty, "unreachable", context)))
    }
}
