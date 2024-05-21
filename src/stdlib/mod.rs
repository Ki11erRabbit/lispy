
pub mod thread;

use std::io::Write;
use std::collections::HashMap;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;

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

fn stdlib_plus(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let mut sum = 0.0;
	for arg in args.iter() {
	    if arg.is_float() {
		sum += arg.get_float(context)?;
	    } else {
		sum += arg.get_integer(context)?.to_f64();
	    }
	}
	match keyword_args.get("x") {
	    Some(value) => {
		if value.is_float() {
		    sum += value.get_float(context)?;
		} else {
		    sum += value.get_integer(context)?.to_f64();
		}
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		if value.is_float() {
		    sum += value.get_float(context)?;
		} else {
		    sum += value.get_integer(context)?.to_f64();
		}
	    }
	    None => {}
	}
	Ok(Value::new_float(sum))
    } else {
	let mut sum = Integer::new();
	for arg in args.iter() {
	    sum += arg.get_integer(context)?;
	}
	match keyword_args.get("x") {
	    Some(value) => {
		sum += value.get_integer(context)?;
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		sum += value.get_integer(context)?;
	    }
	    None => {}
	}
	Ok(Value::new_integer_from_integer(sum))
    }
}

fn stdlib_sub_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_sub(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let difference = if args.len() == 1 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    }; 
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    part1 - part2
	} else if args.len() == 2 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    };
	    let part2 = if args[1].is_float() {
		args[1].get_float(context)?
	    } else {
		args[1].get_integer(context)?.to_f64()
	    };
	    part1 - part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    part1 - part2
	};
	Ok(Value::new_float(difference))
    } else {
	let difference = if args.len() == 1 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    part1 - part2
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = args[1].get_integer(context)?;
	    part1 - part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
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

fn stdlib_mul(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let mut sum = 1.0;
	for arg in args.iter() {
	    if arg.is_float() {
		sum *= arg.get_float(context)?;
	    } else {
		sum *= arg.get_integer(context)?.to_f64();
	    }
	}
	match keyword_args.get("x") {
	    Some(value) => {
		if value.is_float() {
		    sum *= value.get_float(context)?;
		} else {
		    sum *= value.get_integer(context)?.to_f64();
		}
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		if value.is_float() {
		    sum *= value.get_float(context)?;
		} else {
		    sum *= value.get_integer(context)?.to_f64();
		}
	    }
	    None => {}
	}
	Ok(Value::new_float(sum))
    } else {
	let mut sum = Integer::from(1);
	for arg in args.iter() {
	    sum *= arg.get_integer(context)?;
	}
	match keyword_args.get("x") {
	    Some(value) => {
		sum *= value.get_integer(context)?;
	    }
	    None => {}
	}
	match keyword_args.get("y") {
	    Some(value) => {
		sum *= value.get_integer(context)?;
	    }
	    None => {}
	}
	Ok(Value::new_integer_from_integer(sum))
    }
}

fn stdlib_div_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_div(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let difference = if args.len() == 1 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    }; 
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1 / part2
	} else if args.len() == 2 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    };
	    let part2 = if args[1].is_float() {
		args[1].get_float(context)?
	    } else {
		args[1].get_integer(context)?.to_f64()
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1 / part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1 / part2
	};
	Ok(Value::new_float(difference))
    } else {
	let difference = if args.len() == 1 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1.to_f64() / part2.to_f64()
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = args[1].get_integer(context)?;
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1.to_f64() / part2.to_f64()
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["/"], "division by zero", context)));
	    }
	    part1.to_f64() / part2.to_f64()
	};
	Ok(Value::new_float(difference))
    }
}

fn stdlib_floor_div_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_floor_div(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    let float_exists = check_for_floats(&args, &keyword_args);

    if float_exists {
	let difference = if args.len() == 1 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    }; 
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	} else if args.len() == 2 {
	    let part1 = if args[0].is_float() {
		args[0].get_float(context)?
	    } else {
		args[0].get_integer(context)?.to_f64()
	    };
	    let part2 = if args[1].is_float() {
		args[1].get_float(context)?
	    } else {
		args[1].get_integer(context)?.to_f64()
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    if value.is_float() {
			value.get_float(context)?
		    } else {
			value.get_integer(context)?.to_f64()
		    }
		}
		None => unreachable!(),
	    };
	    if part2 == 0.0 {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	};
	Ok(Value::new_float(difference))
    } else {
	let difference = if args.len() == 1 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = args[1].get_integer(context)?;
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	} else {
	    let part1 = match keyword_args.get("x") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    let part2 = match keyword_args.get("y") {
		Some(value) => {
		    value.get_integer(context)?
		}
		None => unreachable!(),
	    };
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["//"], "division by zero", context)));
	    }
	    part1 / part2
	};
	Ok(Value::new_integer_from_integer(Integer::from(difference)))
    }
}


macro_rules! numeric_equality_check {
    ($name:ident, $op:tt, $str:expr) => {
fn $name(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 2 {
	if args[0].is_integer() && args[1].is_integer() {
	    let x = args[0].get_integer(context)?;
	    let y = args[1].get_integer(context)?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_float() && args[1].is_float() {
	    let x = args[0].get_float(context)?;
	    let y = args[1].get_float(context)?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_integer() && args[1].is_float() {
	    let x = args[0].get_integer(context)?.to_f64();
	    let y = args[1].get_float(context)?;
	    return Ok(Value::new_boolean(x $op y));
	} else if args[0].is_float() && args[1].is_integer() {
	    let x = args[0].get_float(context)?;
	    let y = args[1].get_integer(context)?.to_f64();
	    return Ok(Value::new_boolean(x $op y));
	} else {
	    return Err(Box::new(Exception::new(&vec![$str], "arguments must be numbers", context)));
	}
    } else if args.len() == 1 {
	if args[0].is_integer() {
	    let x = args[0].get_integer(context)?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_integer() {
			return Ok(Value::new_boolean(x $op value.get_integer(context)?));
		    } else if value.is_float() {
			return Ok(Value::new_boolean(x.to_f64() $op value.get_float(context)?));
		    } else {
			return Err(Box::new(Exception::new(&vec![$str], "arguments must be numbers", context)));
		    }
		}
		None => {
		    return Err(Box::new(Exception::new(&vec![$str], "missing argument y", context)));
		}
	    }
	} else if args[0].is_float() {
	    let x = args[0].get_float(context)?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_integer() { 
			return Ok(Value::new_boolean(x $op value.get_integer(context)?.to_f64()));
		    } else if value.is_float() {
			return Ok(Value::new_boolean(x $op value.get_float(context)?));
		    } else {
			return Err(Box::new(Exception::new(&vec![$str], "arguments must be numbers", context)));
		    }
		}
		None => {
		    return Err(Box::new(Exception::new(&vec![$str], "missing argument y", context)));
		}
	    }
	} else {
	    let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec![$str], "missing argument x", context)))?;
	    let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec![$str], "missing argument y", context)))?;
	    if x.is_integer() && y.is_integer() {
		let x = x.get_integer(context)?;
		let y = y.get_integer(context)?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_float() && y.is_float() {
		let x = x.get_float(context)?;
		let y = y.get_float(context)?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_integer() && y.is_float() {
		let x = x.get_integer(context)?.to_f64();
		let y = y.get_float(context)?;
		return Ok(Value::new_boolean(x $op y));
	    } else if x.is_float() && y.is_integer() {
		let x = x.get_float(context)?;
		let y = y.get_integer(context)?.to_f64();
		return Ok(Value::new_boolean(x $op y));
	    } else {
		return Err(Box::new(Exception::new(&vec![$str], "arguments must be numbers", context)));
	    }
	}

    }
    Err(Box::new(Exception::new(&vec![$str], "wrong number of arguments", context)))
	
}
    }
}

fn stdlib_greater_than_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_greater_than, >, ">");

fn stdlib_less_than_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_less_than, <, "<");

fn stdlib_greater_than_or_equal_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_greater_than_or_equal, >=, ">=");

fn stdlib_less_than_or_equal_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_less_than_or_equal, <=, "<=");

fn stdlib_equal_to_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

numeric_equality_check!(stdlib_equal, ==, "=");



fn stdlib_display_shape() -> FunctionShape {
    FunctionShape::new(vec!["str".to_string()])
} 

fn stdlib_display(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {

    if args.len() != 1 {
	if keyword_args.get("str").unwrap().is_string() {
	    let string = keyword_args.get("str").unwrap().get_string(context)?;
	    print!("{}", string);
        std::io::stdout().flush().unwrap();
	} else {
	    return Err(Box::new(Exception::new(&vec!["display"], "argument must be a string", context)));
	}
    } else {
	if args[0].is_string() {
	    let string = args[0].get_string(context)?;
	    print!("{}", string);
        std::io::stdout().flush().unwrap();
	} else {
	    return Err(Box::new(Exception::new(&vec!["display"], "argument must be a string", context)));
	}
    }

    Ok(Value::new_nil())
}

fn stdlib_or_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_or(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 2 {
	if args[0].is_boolean() && args[1].is_boolean() {
	    let x = args[0].get_boolean(context)?;
	    let y = args[1].get_boolean(context)?;
	    return Ok(Value::new_boolean(x || y));
	} else {
	    return Err(Box::new(Exception::new(&vec!["or"], "arguments must be booleans", context)));
	}
    } else if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean(context)?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_boolean() { 
			return Ok(Value::new_boolean(x || value.get_boolean(context)?));
		    } else {
			return Err(Box::new(Exception::new(&vec!["or"], "arguments must be booleans", context)));
		    }
		}
		None => {
		    return Err(Box::new(Exception::new(&vec!["or"], "missing argument y", context)));
		}
	    }
	} else {
	    let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["or"], "missing argument x", context)))?;
	    let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["or"], "missing argument y", context)))?;
	    if x.is_boolean() && y.is_boolean() {
		let x = x.get_boolean(context)?;
		let y = y.get_boolean(context)?;
		return Ok(Value::new_boolean(x || y));
	    } else {
		return Err(Box::new(Exception::new(&vec!["or"], "arguments must be booleans", context)));
	    }
	}
    }
    return Err(Box::new(Exception::new(&vec!["or"], "wrong number of arguments", context)));
}

fn stdlib_and_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_and(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 2 {
	if args[0].is_boolean() && args[1].is_boolean() {
	    let x = args[0].get_boolean(context)?;
	    let y = args[1].get_boolean(context)?;
	    return Ok(Value::new_boolean(x && y));
	} else {
	    return Err(Box::new(Exception::new(&vec!["and"], "arguments must be booleans", context)));
	}
    } else if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean(context)?;
	    match keyword_args.get("y") {
		Some(value) => {
		    if value.is_boolean() { 
			return Ok(Value::new_boolean(x && value.get_boolean(context)?));
		    } else {
			return Err(Box::new(Exception::new(&vec!["and"], "arguments must be booleans", context)));
		    }
		}
		None => {
		    return Err(Box::new(Exception::new(&vec!["and"], "missing argument y", context)));
		}
	    }
	} else {
	    let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["and"], "missing argument x", context)))?;
	    let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["and"], "missing argument y", context)))?;
	    if x.is_boolean() && y.is_boolean() {
		let x = x.get_boolean(context)?;
		let y = y.get_boolean(context)?;
		return Ok(Value::new_boolean(x && y));
	    } else {
		return Err(Box::new(Exception::new(&vec!["and"], "arguments must be booleans", context)));
	    }
	}

    }
    return Err(Box::new(Exception::new(&vec!["and"], "wrong number of arguments", context)));
}

fn stdlib_not_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_not(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	if args[0].is_boolean() {
	    let x = args[0].get_boolean(context)?;
	    return Ok(Value::new_boolean(!x));
	} else {
	    return Err(Box::new(Exception::new(&vec!["not"], "argument must be a boolean", context)));
	}
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["not"], "missing argument x", context)))?;
	if x.is_boolean() {
	    let x = x.get_boolean(context)?;
	    return Ok(Value::new_boolean(!x));
	} else {
	    todo!("error");
	}
    }
}

fn stdlib_car_shape() -> FunctionShape {
	FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_car(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	if args[0].is_pair() {
	    let pair = args[0].get_pair(context)?;
	    let (car, _) = pair;
	    return Ok(car.clone());
	} else {
	    return Err(Box::new(Exception::new(&vec!["car"], "argument must be a list", context)));
	}
    } else {
	let x = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["car"], "list not bound", context)))?;
	if x.is_pair() {
	    let list = x.get_pair(context)?;
	    let (car, _) = list;
	    return Ok(car.clone());
	} else {
	    return Err(Box::new(Exception::new(&vec!["car"], "argument must be a list", context)));
	}
    }
}

fn stdlib_cdr_shape() -> FunctionShape {
	FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_cdr(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	if args[0].is_pair() {
	    let pair = args[0].get_pair(context)?;
	    let (_, cdr) = pair;
	    return Ok(cdr.clone());
	} else {
	    return Err(Box::new(Exception::new(&vec!["cdr"], "argument must be a list", context)));
	}
    } else {
	let x = keyword_args.get("list").unwrap();
	if x.is_pair() {
	    let list = x.get_pair(context)?;
	    let (_, cdr) = list;
	    return Ok(cdr.clone());
	} else {
	    return Err(Box::new(Exception::new(&vec!["cdr"], "argument must be a list", context)));
	}
    }
}

fn stdlib_is_integer_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_integer(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_integer()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["integer?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_integer()));
    }
}

fn stdlib_is_float_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_float(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_float()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["float?".to_string()], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_integer()));
    }
}

fn stdlib_is_boolean_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_boolean(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_boolean()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["bool?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_boolean()));
    }
}

fn stdlib_is_symbol_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_symbol(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_symbol()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["symbol?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_symbol()));
    }
}

fn stdlib_is_null_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_null(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_nil()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["null?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_nil()));
    }
}

fn stdlib_is_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_string(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_string()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["string?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_string()));
    }
}

fn stdlib_is_procedure_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_procedure(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_function()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["procedure?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_function()));
    }
}

fn stdlib_is_pair_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_pair(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_pair()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["pair?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_pair()));
    }
}

fn stdlib_is_vector_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_vector(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_vector()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["vector?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_vector()));
    }
}



fn stdlib_sleep_shape() -> FunctionShape {
    FunctionShape::new(vec!["seconds".to_string()])
}

fn stdlib_sleep(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
    if args[0].is_integer() {
	let x = args[0].get_integer(context)?;
	std::thread::sleep(std::time::Duration::from_secs(x.to_u64().unwrap()));
	return Ok(Value::new_nil());
    } else {
	return Err(Box::new(Exception::new(&vec!["sleep"], "argument must be an integer", context)));
    }
    } else {
	let x = keyword_args.get("seconds").unwrap();
	if x.is_integer() {
	    let x = x.get_integer(context)?;
	    std::thread::sleep(std::time::Duration::from_secs(x.to_u64().unwrap()));
	    return Ok(Value::new_nil());
	} else {
	    return Err(Box::new(Exception::new(&vec!["sleep"], "argument must be an integer", context)));
	}
    }
}

fn stdlib_exit_shape() -> FunctionShape {
	FunctionShape::new(vec!["code".to_string()])
}

fn stdlib_exit(context: &mut Context, args: Vec<Value>, keyword_args: HashMap<String, Value>) -> HelperResult<Value> {
    if args.len() == 1 {
	if args[0].is_integer() {
	    let x = args[0].get_integer(context)?;
	    std::process::exit(x.to_i32().unwrap());
	} else {
	    return Err(Box::new(Exception::new(&vec!["exit"], "argument must be an integer", context)));
	}
    } else {
    let x = keyword_args.get("code").unwrap();
	if x.is_integer() {
	    let x = x.get_integer(context)?;
	    std::process::exit(x.to_i32().unwrap());
	} else {
	    return Err(Box::new(Exception::new(&vec!["exit"], "argument must be an integer", context)));
	}
    }
}

pub fn get_stdlib(context: &mut Context) -> ContextFrame {
    let mut bindings = HashMap::new();

    bindings.insert("+".to_string(), Value::new_function(Function::Native(stdlib_plus, stdlib_plus_shape()), context));
    bindings.insert("-".to_string(), Value::new_function(Function::Native(stdlib_sub, stdlib_sub_shape()), context));
    bindings.insert("*".to_string(), Value::new_function(Function::Native(stdlib_mul, stdlib_mul_shape()), context));
    bindings.insert("/".to_string(), Value::new_function(Function::Native(stdlib_div, stdlib_div_shape()), context));
    bindings.insert("//".to_string(), Value::new_function(Function::Native(stdlib_floor_div, stdlib_floor_div_shape()), context));
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
    bindings.insert("exit".to_string(), Value::new_function(Function::Native(stdlib_exit, stdlib_exit_shape()), context));
    bindings.insert("car".to_string(), Value::new_function(Function::Native(stdlib_car, stdlib_car_shape()), context));
    bindings.insert("cdr".to_string(), Value::new_function(Function::Native(stdlib_cdr, stdlib_cdr_shape()), context));
    bindings.insert("integer?".to_string(), Value::new_function(Function::Native(stdlib_is_integer, stdlib_is_integer_shape()), context));
    bindings.insert("float?".to_string(), Value::new_function(Function::Native(stdlib_is_float, stdlib_is_float_shape()), context));
    bindings.insert("boolean?".to_string(), Value::new_function(Function::Native(stdlib_is_boolean, stdlib_is_boolean_shape()), context));
    bindings.insert("symbol?".to_string(), Value::new_function(Function::Native(stdlib_is_symbol, stdlib_is_symbol_shape()), context));
    bindings.insert("integer?".to_string(), Value::new_function(Function::Native(stdlib_is_integer, stdlib_is_integer_shape()), context));
    bindings.insert("string?".to_string(), Value::new_function(Function::Native(stdlib_is_string, stdlib_is_string_shape()), context));
    bindings.insert("procedure?".to_string(), Value::new_function(Function::Native(stdlib_is_procedure, stdlib_is_procedure_shape()), context));
    bindings.insert("pair?".to_string(), Value::new_function(Function::Native(stdlib_is_pair, stdlib_is_pair_shape()), context));
    bindings.insert("vector?".to_string(), Value::new_function(Function::Native(stdlib_is_vector, stdlib_is_vector_shape()), context));
    bindings.insert("null?".to_string(), Value::new_function(Function::Native(stdlib_is_null, stdlib_is_null_shape()), context));

    ContextFrame::new_with_bindings(bindings)
}
