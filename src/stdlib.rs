use std::collections::HashMap;

use rug::Integer;

use crate::interpreter::{value::{Value, FunctionShape, Function}, context::{ContextFrame, Context}};

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

fn stdlib_plus(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
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

fn stdlib_sub(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
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

fn stdlib_mul(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
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

fn stdlib_div(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
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

macro_rules! numeric_equality_check {
    ($name:ident, $op:tt) => {
fn $name(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    if args.len() == 2 {
	if args[0].is_integer() && args[1].is_integer() {
	    let x = args[0].get_integer()?;
	    let y = args[1].get_integer()?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_float() && args[1].is_float() {
	    let x = args[0].get_float()?;
	    let y = args[1].get_float()?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_integer() && args[1].is_float() {
	    let x = args[0].get_integer()?.to_f64();
	    let y = args[1].get_float()?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_float() && args[1].is_integer() {
	    let x = args[0].get_float()?;
	    let y = args[1].get_integer()?.to_f64();
	    return Ok(Value::new_boolean(x $op y));
	} else {
	    todo!("error");
	}
    } else if args.len() == 1 {
	if args[0].is_integer() {
	    let x = args[0].get_integer()?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_integer() {
			return Ok(Value::new_boolean(x $op value.get_integer()?));
		    } else if value.is_float() {
			return Ok(Value::new_boolean(x.to_f64() $op value.get_float()?));
		    } else {
			todo!("error");
		    }
		}
		None => {
		    todo!("error");
		}
	    }
	} else if args[0].is_float() {
	    let x = args[0].get_float()?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_integer() { 
			return Ok(Value::new_boolean(x $op value.get_integer()?.to_f64()));
		    } else if value.is_float() {
			return Ok(Value::new_boolean(x $op value.get_float()?));
		    } else {
			todo!("error");
		    }
		}
		None => {
		    todo!("error");
		}
	    }
	} else {
	    let x = keyword_args.get("x").unwrap();
	    let y = keyword_args.get("y").unwrap();
	    if x.is_integer() && y.is_integer() {
		let x = x.get_integer()?;
		let y = y.get_integer()?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_float() && y.is_float() {
		let x = x.get_float()?;
		let y = y.get_float()?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_integer() && y.is_float() {
		let x = x.get_integer()?.to_f64();
		let y = y.get_float()?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_float() && y.is_integer() {
		let x = x.get_float()?;
		let y = y.get_integer()?.to_f64();
		return Ok(Value::new_boolean(x $op y));
	    } else {
		todo!("error");
	    }
	}

    }
    todo!("error");
	
}
    }
}

fn stdlib_greater_than_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_greater_than, >);

fn stdlib_less_than_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_less_than, <);

fn stdlib_greater_than_or_equal_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_greater_than_or_equal, >=);

fn stdlib_less_than_or_equal_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_less_than_or_equal, <=);

fn stdlib_equal_to_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_equal, ==);



fn stdlib_display_shape() -> FunctionShape {
    FunctionShape::new(vec!["str".to_string()])
} 

fn stdlib_display(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {

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

fn stdlib_or_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_or(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
	if args.len() == 2 {
	if args[0].is_boolean() && args[1].is_boolean() {
	    let x = args[0].get_boolean()?;
	    let y = args[1].get_boolean()?;
	    return Ok(Value::new_boolean(x || y));
	} else {
	    todo!("error");
	}
	} else if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean()?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_boolean() { 
			return Ok(Value::new_boolean(x || value.get_boolean()?));
		    } else {
			todo!("error");
		    }
		}
		None => {
		    todo!("error");
		}
	    }
	} else {
	    let x = keyword_args.get("x").unwrap();
	    let y = keyword_args.get("y").unwrap();
	    if x.is_boolean() && y.is_boolean() {
		let x = x.get_boolean()?;
		let y = y.get_boolean()?;
		return Ok(Value::new_boolean(x || y));
	    } else {
		todo!("error");
	    }
	}

	}
	todo!("error");
}

fn stdlib_and_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_and(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
	if args.len() == 2 {
	if args[0].is_boolean() && args[1].is_boolean() {
	    let x = args[0].get_boolean()?;
	    let y = args[1].get_boolean()?;
	    return Ok(Value::new_boolean(x && y));
	} else {
	    todo!("error");
	}
	} else if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean()?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_boolean() { 
			return Ok(Value::new_boolean(x && value.get_boolean()?));
		    } else {
			todo!("error");
		    }
		}
		None => {
		    todo!("error");
		}
	    }
	} else {
	    let x = keyword_args.get("x").unwrap();
	    let y = keyword_args.get("y").unwrap();
	    if x.is_boolean() && y.is_boolean() {
		let x = x.get_boolean()?;
		let y = y.get_boolean()?;
		return Ok(Value::new_boolean(x && y));
	    } else {
		todo!("error");
	    }
	}

	}
	todo!("error");
}

fn stdlib_not_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_not(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
	if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean()?;
	    return Ok(Value::new_boolean(!x));
	} else {
	    todo!("error");
	}
	} else {
	    let x = keyword_args.get("x").unwrap();
	    if x.is_boolean() {
		let x = x.get_boolean()?;
		return Ok(Value::new_boolean(!x));
	    } else {
		todo!("error");
	    }
	}
}

fn stdlib_sleep_shape() -> FunctionShape {
    FunctionShape::new(vec!["seconds".to_string()])
}

fn stdlib_sleep(_: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> Result<Value, Box<dyn std::error::Error>> {
    if args.len() == 1 {
    if args[0].is_integer() {
	let x = args[0].get_integer()?;
	std::thread::sleep(std::time::Duration::from_secs(x.to_u64().unwrap()));
	return Ok(Value::new_nil());
    } else {
	todo!("error");
    }
    } else {
	let x = keyword_args.get("seconds").unwrap();
	if x.is_integer() {
	    let x = x.get_integer()?;
	    std::thread::sleep(std::time::Duration::from_secs(x.to_u64().unwrap()));
	    return Ok(Value::new_nil());
	} else {
	    todo!("error");
	}
    }
}

pub fn get_stdlib(context: &mut Context) -> ContextFrame {
    let mut bindings = HashMap::new();

    bindings.insert("+".to_string(), Value::new_function(Function::Native(stdlib_plus, stdlib_plus_shape()), context));
    bindings.insert("-".to_string(), Value::new_function(Function::Native(stdlib_sub, stdlib_sub_shape()), context));
    bindings.insert("*".to_string(), Value::new_function(Function::Native(stdlib_mul, stdlib_mul_shape()), context));
    bindings.insert("/".to_string(), Value::new_function(Function::Native(stdlib_div, stdlib_div_shape()), context));
    bindings.insert(">".to_string(), Value::new_function(Function::Native(stdlib_greater_than, stdlib_greater_than_shape()), context));
    bindings.insert("<".to_string(), Value::new_function(Function::Native(stdlib_less_than, stdlib_less_than_shape()), context));
    bindings.insert(">=".to_string(), Value::new_function(Function::Native(stdlib_greater_than_or_equal, stdlib_greater_than_or_equal_shape()), context));
    bindings.insert("<=".to_string(), Value::new_function(Function::Native(stdlib_less_than_or_equal, stdlib_less_than_or_equal_shape()), context));
    bindings.insert("=".to_string(), Value::new_function(Function::Native(stdlib_equal, stdlib_equal_to_shape()), context));
    bindings.insert("display".to_string(), Value::new_function(Function::Native(stdlib_display, stdlib_display_shape()), context));
    bindings.insert("or".to_string(), Value::new_function(Function::Native(stdlib_or, stdlib_or_shape()), context));
    bindings.insert("and".to_string(), Value::new_function(Function::Native(stdlib_and, stdlib_and_shape()), context));
    bindings.insert("not".to_string(), Value::new_function(Function::Native(stdlib_not, stdlib_not_shape()), context));
    bindings.insert("sleep".to_string(), Value::new_function(Function::Native(stdlib_sleep, stdlib_sleep_shape()), context));
    ContextFrame::new_with_bindings(bindings)
}
