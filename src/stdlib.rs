use std::collections::HashMap;

use rug::Integer;

use crate::interpreter::{value::{Value, FunctionShape, Function}, context::ContextFrame};

fn check_for_floats(args: &Vec<Value>, keyword_args: &HashMap<String, Value>) -> bool {
    let mut floats_exist = false;
    for arg in args.iter() {
	if arg.is_float() {
	    floats_exist = true;
	    break;
	}
    }
    if !floats_exist {
	for (_, value) in keyword_args.iter() {
	    if value.is_float() {
		floats_exist = true;
		break;
	    }
	}
    }
    floats_exist
}

fn stdlib_plus_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_plus(args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let mut sum = 0.0;
	for arg in args.iter() {
	    if arg.is_float() {
		sum += arg.get_float()?;
	    } else {
		sum += arg.get_integer()?.to_f64();
	    }
	}
	match keyword_args.get("x") {
	    Some(value) => {
		if value.is_float() {
		    sum += value.get_float()?;
		} else {
		    sum += value.get_integer()?.to_f64();
		}
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		if value.is_float() {
		    sum += value.get_float()?;
		} else {
		    sum += value.get_integer()?.to_f64();
		}
	    }
	    None => {}
	}
	Ok(Value::new_float(sum))
    } else {
	let mut sum = Integer::new();
	for arg in args.iter() {
	    sum += arg.get_integer()?;
	}
	match keyword_args.get("x") {
	    Some(value) => {
		sum += value.get_integer()?;
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		sum += value.get_integer()?;
	    }
	    None => {}
	}
	Ok(Value::new_integer_from_integer(sum))
    }
}

fn stdlib_sub_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_sub(args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let difference = if args.len() == 1 {
	    let part1 = if args[0].is_float() {
		args[0].get_float()?
	    } else {
		args[0].get_integer()?.to_f64()
	    }; 
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    part1 - part2
	} else if args.len() == 2 {
	    let part1 = if args[0].is_float() {
		args[0].get_float()?
	    } else {
		args[0].get_integer()?.to_f64()
	    };
	    let part2 = if args[1].is_float() {
		args[1].get_float()?
	    } else {
		args[1].get_integer()?.to_f64()
	    };
	    part1 - part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    part1 - part2
	};
	Ok(Value::new_float(difference))
    } else {
	let difference = if args.len() == 1 {
	    let part1 = args[0].get_integer()?;
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    part1 - part2
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer()?;
	    let part2 = args[1].get_integer()?;
	    part1 - part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    part1 - part2
	};
	Ok(Value::new_integer_from_integer(Integer::from(difference)))
    }
}

fn stdlib_mul_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_mul(args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let mut sum = 1.0;
	for arg in args.iter() {
	    if arg.is_float() {
		sum *= arg.get_float()?;
	    } else {
		sum *= arg.get_integer()?.to_f64();
	    }
	}
	match keyword_args.get("x") {
	    Some(value) => {
		if value.is_float() {
		    sum *= value.get_float()?;
		} else {
		    sum *= value.get_integer()?.to_f64();
		}
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		if value.is_float() {
		    sum *= value.get_float()?;
		} else {
		    sum *= value.get_integer()?.to_f64();
		}
	    }
	    None => {}
	}
	Ok(Value::new_float(sum))
    } else {
	let mut sum = Integer::from(1);
	for arg in args.iter() {
	    sum *= arg.get_integer()?;
	}
	match keyword_args.get("x") {
	    Some(value) => {
		sum *= value.get_integer()?;
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		sum *= value.get_integer()?;
	    }
	    None => {}
	}
	Ok(Value::new_integer_from_integer(sum))
    }
}

fn stdlib_div_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_div(args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let difference = if args.len() == 1 {
	    let part1 = if args[0].is_float() {
		args[0].get_float()?
	    } else {
		args[0].get_integer()?.to_f64()
	    }; 
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		todo!("error");
	    }
	    part1 / part2
	} else if args.len() == 2 {
	    let part1 = if args[0].is_float() {
		args[0].get_float()?
	    } else {
		args[0].get_integer()?.to_f64()
	    };
	    let part2 = if args[1].is_float() {
		args[1].get_float()?
	    } else {
		args[1].get_integer()?.to_f64()
	    };
	    if part2 == 0.0 {
		todo!("error");
	    }
	    part1 / part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float()?
		    } else {
			value.get_integer()?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		todo!("error");
	    }
	    part1 / part2
	};
	Ok(Value::new_float(difference))
    } else {
	let difference = if args.len() == 1 {
	    let part1 = args[0].get_integer()?;
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		todo!("error");
	    }
	    part1 / part2
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer()?;
	    let part2 = args[1].get_integer()?;
	    if part2.is_zero() {
		todo!("error");
	    }
	    part1 / part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer()?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		todo!("error");
	    }
	    part1 / part2
	};
	Ok(Value::new_integer_from_integer(Integer::from(difference)))
    }
}

fn stdlib_display_shape() -> FunctionShape {
    FunctionShape::new(vec!["str".to_string()])
} 

fn stdlib_display(args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {

    if args.len() != 1 {
	if keyword_args.get("str").unwrap().is_string() {
	    let string = keyword_args.get("str").unwrap().get_string()?;
	    print!("{}", string);
	} else {
	    todo!("error");
	}
    } else {
	if args[0].is_string() {
	    let string = args[0].get_string()?;
	    print!("{}", string);
	} else {
	    todo!("error");
	}
    }

    Ok(Value::new_nil())
}


pub fn get_stdlib() -> ContextFrame {
    let mut bindings = HashMap::new();

    bindings.insert("+".to_string(), Value::new_function(Function::Native(stdlib_plus, stdlib_plus_shape())));
    bindings.insert("-".to_string(), Value::new_function(Function::Native(stdlib_sub, stdlib_sub_shape())));
    bindings.insert("*".to_string(), Value::new_function(Function::Native(stdlib_mul, stdlib_mul_shape())));
    bindings.insert("/".to_string(), Value::new_function(Function::Native(stdlib_div, stdlib_div_shape())));
    bindings.insert("display".to_string(), Value::new_function(Function::Native(stdlib_display, stdlib_display_shape())));
    ContextFrame::new_with_bindings(bindings)
}
