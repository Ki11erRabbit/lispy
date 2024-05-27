pub mod thread;
pub mod file;
pub mod network;
pub mod sync;

use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use std::io::BufRead;
use crate::interpreter::Exception;
use crate::interpreter::HelperResult;
use crate::interpreter::kwargs::Kwargs;


use rug::Integer;

use crate::interpreter::{value::{Value, function::{FunctionShape, Function}}, context::{ContextFrame, Context}};

fn check_for_floats(args: &Vec<Value>, keyword_args: &Kwargs) -> bool {
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

fn stdlib_plus(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_sub(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_mul(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_div(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_floor_div(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_modulo_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
} 

fn stdlib_modulo(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
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
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
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
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
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
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
	} else if args.len() == 2 {
	    let part1 = args[0].get_integer(context)?;
	    let part2 = args[1].get_integer(context)?;
	    if part2.is_zero() {
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
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
		return Err(Box::new(Exception::new(&vec!["modulo"], "division by zero", context)));
	    }
	    part1 % part2
	};
	Ok(Value::new_integer_from_integer(Integer::from(difference)))
    }
}

macro_rules! numeric_equality_check {
    ($name:ident, $op:tt, $str:expr) => {
fn $name(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_display(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {

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

fn stdlib_or(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_and(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_not(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_car(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_cdr(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_integer(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_float(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_boolean(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_symbol(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_null(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_nil()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["nil?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_nil()));
    }
}

fn stdlib_is_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_procedure(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_pair(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_vector(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_vector()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["vector?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_vector()));
    }
}

fn stdlib_is_char_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string()])
}

fn stdlib_is_char(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    if args.len() == 1 {
	return Ok(Value::new_boolean(args[0].is_char()));
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["char?"], "expected x to be bound", context)))?;
	return Ok(Value::new_boolean(x.is_char()));
    }
}


fn stdlib_append_shape() -> FunctionShape {
    FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_append(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (x, y) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let x = args[0].clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["append"], "missing argument y", context)))?.clone();
	(x, y)
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["append"], "missing argument x", context)))?.clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["append"], "missing argument y", context)))?.clone();
	(x, y)
	};

    if x.is_vector() && y.is_vector() {
	stdlib_vector_append(context, vec![x, y], Kwargs::new())
    } else if x.is_pair() && y.is_pair() {
	stdlib_list_append(context, vec![x, y], Kwargs::new())
    } else if x.is_string() && y.is_string() {
	stdlib_string_append(context, vec![x, y], Kwargs::new())
    } else if x.is_symbol() && y.is_symbol() {
	stdlib_symbol_append(context, vec![x, y], Kwargs::new())
    } else {
	return Err(Box::new(Exception::new(&vec!["append"], "arguments must be vectors, strings, pairs, or symbols", context)));
    }
}

fn stdlib_vector_append_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_vector_append(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (mut x, y) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let x = args[0].clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["vector-append"], "missing argument y", context)))?.clone();
	(x, y)
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["vector-append"], "missing argument x", context)))?.clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["vector-append"], "missing argument y", context)))?.clone();
	(x, y)
    };

    if x.is_vector() && y.is_vector() {
	let x_ref = x.get_vector_mut(context)?;
	let y = y.get_vector(context)?;
	x_ref.append(&mut y.clone());
	return Ok(x);
    } else {
	return Err(Box::new(Exception::new(&vec!["vector-append"], "arguments must be vectors", context)));
    }
}

fn stdlib_list_append_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_list_append(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (mut x, y) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let x = args[0].clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["list-append"], "missing argument y", context)))?.clone();
	(x, y)
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["list-append"], "missing argument x", context)))?.clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["list-append"], "missing argument y", context)))?.clone();
	(x, y)
    };

    if x.is_pair() && y.is_pair() {
	let original_x = x.clone();
	let x = x.get_pair_mut(context)?;
	let mut x_ref = x;
	while !x_ref.1.is_nil() {
	    x_ref = x_ref.1.get_pair_mut(context)?;
	}
	*x_ref.1 = y.clone();
	return Ok(original_x);
    } else {
	return Err(Box::new(Exception::new(&vec!["list-append"], "arguments must be pairs", context)));
    }
}

fn stdlib_string_append_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_string_append(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (x, y) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let x = args[0].clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["string-append"], "missing argument y", context)))?.clone();
	(x, y)
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["string-append"], "missing argument x", context)))?.clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["string-append"], "missing argument y", context)))?.clone();
	(x, y)
    };

    if x.is_string() && y.is_string() {
	let x = x.get_string(context)?;
	let y = y.get_string(context)?;
	let out = format!("{}{}", x, y);
	return Ok(Value::new_string_from_string(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["string-append"], "arguments must be strings", context)));
    }
}

fn stdlib_symbol_append_shape() -> FunctionShape {
	FunctionShape::new(vec!["x".to_string(), "y".to_string()])
}

fn stdlib_symbol_append(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (x, y) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let x = args[0].clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["symbol-append"], "missing argument y", context)))?.clone();
	(x, y)
    } else {
	let x = keyword_args.get("x").ok_or(Box::new(Exception::new(&vec!["symbol-append"], "missing argument x", context)))?.clone();
	let y = keyword_args.get("y").ok_or(Box::new(Exception::new(&vec!["symbol-append"], "missing argument y", context)))?.clone();
	(x, y)
    };

    if x.is_symbol() && y.is_symbol() {
	let x = x.get_symbol(context)?;
	let y = y.get_symbol(context)?;
	let mut out = Vec::from_iter(x.iter().cloned().chain(y.iter().cloned()));
	for y_i in y.iter() {
	    out.push(y_i.clone());
	}

	return Ok(Value::new_symbol(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["symbol-append"], "arguments must be symbols", context)));
    }
}

fn stdlib_read_char_shape() -> FunctionShape {
    FunctionShape::new(vec![])
}

fn stdlib_read_char(context: &mut Context, _: Vec<Value>, _: Kwargs) -> HelperResult<Value> {
    let mut buffer = [0; 1];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_exact(&mut buffer).map_err(|e| Box::new(Exception::new(&vec!["read-char"], &format!("{}", e), context)))?;
    let c = buffer[0] as char;
    Ok(Value::new_char(c))
}

fn stdlib_read_line_shape() -> FunctionShape {
	FunctionShape::new(vec![])
}

fn stdlib_read_line(context: &mut Context, _: Vec<Value>, _: Kwargs) -> HelperResult<Value> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer).map_err(|e| Box::new(Exception::new(&vec!["read-line"], &format!("{}", e), context)))?;
    Ok(Value::new_string_from_string(buffer, context))
}

fn stdlib_read_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["amount".to_string()])
}

fn stdlib_read_string(context: &mut Context, args: Vec<Value>, kargs: Kwargs) -> HelperResult<Value> {
    let amount = if args.len() == 1 {
	if args[0].is_integer() {
	    args[0].get_integer(context)?.to_u64().unwrap()
	} else {
	    return Err(Box::new(Exception::new(&vec!["read-string"], "argument must be an integer", context)));
	}
    } else {
	if kargs.get("amount").unwrap().is_integer() {
	    kargs.get("amount").unwrap().get_integer(context)?.to_u64().unwrap()
	} else {
	    return Err(Box::new(Exception::new(&vec!["read-string"], "argument must be an integer", context)));
	}
    };

    let mut buffer = vec![0; amount as usize];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_exact(&mut buffer).map_err(|e| Box::new(Exception::new(&vec!["read-string"], &format!("{}", e), context)))?;
    let s = String::from_utf8(buffer).map_err(|e| Box::new(Exception::new(&vec!["read-string"], &format!("{}", e), context)))?;
    Ok(Value::new_string_from_string(s, context))
}

fn stdlib_list_ref_shape() -> FunctionShape {
    FunctionShape::new(vec!["list".to_string(), "index".to_string()])
}

fn stdlib_list_ref(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (list, index) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let list = args[0].clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["list-ref"], "missing argument index", context)))?.clone();
	(list, index)
    } else {
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list-ref"], "missing argument list", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["list-ref"], "missing argument index", context)))?.clone();
	(list, index)
    };

    let list = list.get_pair(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    let mut current = list;
    for _ in 0..index {
	current = current.1.get_pair(context)?;
    }
    Ok(current.0.clone())
}

fn stdlib_vector_ref_shape() -> FunctionShape {
    FunctionShape::new(vec!["vector".to_string(), "index".to_string()])
}

fn stdlib_vector_ref(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (vector, index) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let vector = args[0].clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["vector-ref"], "missing argument index", context)))?.clone();
	(vector, index)
    } else {
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector-ref"], "missing argument vector", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["vector-ref"], "missing argument index", context)))?.clone();
	(vector, index)
    };

    let vector = vector.get_vector(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    Ok(vector[index as usize].clone())
}

fn stdlib_string_ref_shape() -> FunctionShape {
	FunctionShape::new(vec!["string".to_string(), "index".to_string()])
}

fn stdlib_string_ref(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (string, index) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let string = args[0].clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["string-ref"], "missing argument index", context)))?.clone();
	(string, index)
    } else {
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string-ref"], "missing argument string", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["string-ref"], "missing argument index", context)))?.clone();
	(string, index)
    };

    let string = string.get_string(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    let c = string.chars().nth(index as usize).ok_or(Box::new(Exception::new(&vec!["string-ref"], "index out of bounds", context)))?;
    Ok(Value::new_char(c))
}

fn stdlib_vector_set_shape() -> FunctionShape {
    FunctionShape::new(vec!["vector".to_string(), "index".to_string(), "value".to_string()])
}

fn stdlib_vector_set(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (mut vector, index, value) = if args.len() == 3 {
	(args[0].clone(), args[1].clone(), args[2].clone())
    } else if args.len() == 2 {
	let vector = args[0].clone();
	let index = args[1].clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["vector-set!"], "missing argument value", context)))?.clone();
	(vector, index, value)
    } else {
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector-set!"], "missing argument vector", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["vector-set!"], "missing argument index", context)))?.clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["vector-set!"], "missing argument value", context)))?.clone();
	(vector, index, value)
    };

    let vector = vector.get_vector_mut(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    vector[index as usize] = value.clone();
    Ok(Value::new_nil())
}

fn stdlib_string_set_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string(), "index".to_string(), "value".to_string()])
}

fn stdlib_string_set(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (mut string, index, value) = if args.len() == 3 {
	(args[0].clone(), args[1].clone(), args[2].clone())
    } else if args.len() == 2 {
	let string = args[0].clone();
	let index = args[1].clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["string-set!"], "missing argument value", context)))?.clone();
	(string, index, value)
    } else {
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string-set!"], "missing argument string", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["string-set!"], "missing argument index", context)))?.clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["string-set!"], "missing argument value", context)))?.clone();
	(string, index, value)
    };

    let string = string.get_string_mut(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    let value = value.get_char(context)?.to_string();
    let mut new_string = String::new();
    for (i, c) in string.chars().enumerate() {
    if i == index as usize {
	new_string.push_str(&value);
    } else {
	new_string.push(c);
    }
    }
    *string = new_string;
    Ok(Value::new_nil())
}

fn stdlib_list_set_shape() -> FunctionShape {
    FunctionShape::new(vec!["list".to_string(), "index".to_string(), "value".to_string()])
}

fn stdlib_list_set(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (mut list, index, value) = if args.len() == 3 {
	(args[0].clone(), args[1].clone(), args[2].clone())
    } else if args.len() == 2 {
	let list = args[0].clone();
	let index = args[1].clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["list-set!"], "missing argument value", context)))?.clone();
	(list, index, value)
    } else {
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list-set!"], "missing argument list", context)))?.clone();
	let index = keyword_args.get("index").ok_or(Box::new(Exception::new(&vec!["list-set!"], "missing argument index", context)))?.clone();
	let value = keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["list-set!"], "missing argument value", context)))?.clone();
	(list, index, value)
    };

    let list = list.get_pair_mut(context)?;
    let index = index.get_integer(context)?.to_u64().unwrap();
    let mut current = list;
    for _ in 0..index {
	current = current.1.get_pair_mut(context)?;
    }
    *current.0 = value.clone();
    Ok(Value::new_nil())
}

fn stdlib_length_shape() -> FunctionShape {
    FunctionShape::new(vec!["data".to_string()])
}

fn stdlib_length(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let data = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("data").ok_or(Box::new(Exception::new(&vec!["length"], "missing argument data", context)))?.clone()
    };

    if data.is_string() {
	return stdlib_string_length(context, args, keyword_args);
    } else if data.is_vector() {
	return stdlib_vector_length(context, args, keyword_args);
    } else if data.is_pair() {
	return stdlib_list_length(context, args, keyword_args);
    } else {
	todo!("make it so that we can do a dynamic lookup of the function to call");
	//return Err(Box::new(Exception::new(&vec!["length"], "argument must be a string, vector, or list", context)));
    }
}

fn stdlib_string_length_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_length(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string-length"], "missing argument string", context)))?.clone()
    };

    if string.is_string() {
	let string = string.get_string(context)?;
	return Ok(Value::new_integer_from_usize(string.chars().count()));
    } else {
	return Err(Box::new(Exception::new(&vec!["string-length"], "argument must be a string", context)));
    }
}

fn stdlib_vector_length_shape() -> FunctionShape {
	FunctionShape::new(vec!["vector".to_string()])
}

fn stdlib_vector_length(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let vector = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector-length"], "missing argument vector", context)))?.clone()
    };

    if vector.is_vector() {
	let vector = vector.get_vector(context)?;
	return Ok(Value::new_integer_from_usize(vector.len()));
    } else {
	return Err(Box::new(Exception::new(&vec!["vector-length"], "argument must be a vector", context)));
    }
}

fn stdlib_list_length_shape() -> FunctionShape {
	FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_list_length(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let list = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list-length"], "missing argument list", context)))?.clone()
    };

    if list.is_pair() {
	let mut current = list.get_pair(context)?;
	let mut len = 0;
	while !current.1.is_nil() {
	    len += 1;
	    current = current.1.get_pair(context)?;
	}
	return Ok(Value::new_integer_from_usize(len));
    } else {
	return Err(Box::new(Exception::new(&vec!["list-length"], "argument must be a list", context)));
    }
}

fn stdlib_reverse_shape() -> FunctionShape {
    FunctionShape::new(vec!["data".to_string()])
}

fn stdlib_reverse(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let data = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("data").ok_or(Box::new(Exception::new(&vec!["reverse"], "missing argument data", context)))?.clone()
    };

    if data.is_string() {
	return stdlib_string_reverse(context, args, keyword_args);
    } else if data.is_vector() {
	return stdlib_vector_reverse(context, args, keyword_args);
    } else if data.is_pair() {
	return stdlib_list_reverse(context, args, keyword_args);
    } else {
	todo!("make it so that we can do a dynamic lookup of the function to call");
	//return Err(Box::new(Exception::new(&vec!["reverse"], "argument must be a string, vector, or list", context)));
    }
}

fn stdlib_list_reverse_shape() -> FunctionShape {
    FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_list_reverse(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let list = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list-reverse"], "missing argument list", context)))?.clone()
    };

    if list.is_pair() {
	let mut current = list.get_pair(context)?;
	let mut reversed = Value::new_nil();
	while !current.1.is_nil() {
	    reversed = Value::new_pair(current.0.clone(), reversed, context);
	    current = current.1.get_pair(context)?;
	}
	reversed = Value::new_pair(current.0.clone(), reversed, context);
	return Ok(reversed);
    } else {
	return Err(Box::new(Exception::new(&vec!["list-reverse"], "argument must be a list", context)));
    }
}

fn stdlib_string_reverse_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_reverse(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string-reverse"], "missing argument string", context)))?.clone()
    };

    if string.is_string() {
	let string = string.get_string(context)?;
	let reversed = string.chars().rev().collect::<String>();
	return Ok(Value::new_string_from_string(reversed, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["string-reverse"], "argument must be a string", context)));
    }
}

fn stdlib_vector_reverse_shape() -> FunctionShape {
    FunctionShape::new(vec!["vector".to_string()])
}

fn stdlib_vector_reverse(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let vector = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector-reverse"], "missing argument vector", context)))?.clone()
    };

    if vector.is_vector() {
	let vector = vector.get_vector(context)?;
	let reversed = vector.iter().rev().cloned().collect::<Vec<Value>>();
	return Ok(Value::new_vector(reversed, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["vector-reverse"], "argument must be a vector", context)));
    }
}
    
fn stdlib_type_name_shape() -> FunctionShape {
	FunctionShape::new(vec!["value".to_string()])
}

fn stdlib_type_name(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let value = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("value").ok_or(Box::new(Exception::new(&vec!["type-name"], "missing argument value", context)))?.clone()
    };

    let index = value.get_type_index();

    Ok(context.get_type_symbol(index))
}

fn stdlib_integer_to_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["integer".to_string()])
}

fn stdlib_integer_to_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let integer = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("integer").ok_or(Box::new(Exception::new(&vec!["integer->string"], "missing argument integer", context)))?.clone()
    };

    if integer.is_integer() {
    let integer = integer.get_integer(context)?;
	return Ok(Value::new_string_from_string(integer.to_string(), context));
    } else {
	return Err(Box::new(Exception::new(&vec!["integer->string"], "argument must be an integer", context)));
    }
}

fn stdlib_string_to_integer_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_integer(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string->integer"], "missing argument string", context)))?.clone()
    };

    if string.is_string() {
	let string = string.get_string(context)?;
	let integer = string.parse::<rug::Integer>().map_err(|e| Box::new(Exception::new(&vec!["string->integer"], &format!("{}", e), context)))?;
	return Ok(Value::new_integer_from_integer(integer));
    } else {
	return Err(Box::new(Exception::new(&vec!["string->integer"], "argument must be a string", context)));
    }
}

fn stdlib_list_to_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_list_to_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let list = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list->string"], "missing argument list", context)))?.clone()
    };

    if list.is_pair() {
	let mut current = list.get_pair(context)?;
	let mut out = String::new();
	while !current.1.is_nil() {
	    if current.0.is_char() {
		out.push(current.0.get_char(context)?);
	    } else {
		return Err(Box::new(Exception::new(&vec!["list->string"], "list must contain only characters", context)));
	    }
	    current = current.1.get_pair(context)?;
	}
	if current.0.is_char() {
	    out.push(current.0.get_char(context)?);
	} else {
	    return Err(Box::new(Exception::new(&vec!["list->string"], "list must contain only characters", context)));
	}
	return Ok(Value::new_string_from_string(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["list->string"], "argument must be a list", context)));
    }
}

fn stdlib_string_to_list_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_list(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string->list"], "missing argument string", context)))?.clone()
    };

    if string.is_string() {
	let string = string.get_string(context)?;
	let mut out = Value::new_nil();
	for c in string.chars().rev() {
	    out = Value::new_pair(Value::new_char(c), out, context);
	}
	return Ok(out);
    } else {
	return Err(Box::new(Exception::new(&vec!["string->list"], "argument must be a string", context)));
    }
}

fn stdlib_vector_to_list_shape() -> FunctionShape {
    FunctionShape::new(vec!["vector".to_string()])
}

fn stdlib_vector_to_list(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let vector = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector->list"], "missing argument vector", context)))?.clone()
    };

    if vector.is_vector() {
	let vector = vector.get_vector(context)?;
	let mut out = Value::new_nil();
	for v in vector.iter().rev() {
	    out = Value::new_pair(v.clone(), out, context);
	}
	return Ok(out);
    } else {
	return Err(Box::new(Exception::new(&vec!["vector->list"], "argument must be a vector", context)));
    }
}

fn stdlib_list_to_vector_shape() -> FunctionShape {
    FunctionShape::new(vec!["list".to_string()])
}

fn stdlib_list_to_vector(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let list = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["list->vector"], "missing argument list", context)))?.clone()
    };

    if list.is_pair() {
	let mut current = list.get_pair(context)?;
	let mut out = Vec::new();
	while !current.1.is_nil() {
	    out.push(current.0.clone());
	    current = current.1.get_pair(context)?;
	}
	out.push(current.0.clone());
	return Ok(Value::new_vector(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["list->vector"], "argument must be a list", context)));
    }
}

fn stdlib_vector_to_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["vector".to_string()])
}

fn stdlib_vector_to_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let vector = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["vector->string"], "missing argument vector", context)))?.clone()
    };

    if vector.is_vector() {
	let vector = vector.get_vector(context)?;
	let mut out = String::new();
	for v in vector.iter().rev() {
	    if v.is_char() {
		out.push(v.get_char(context)?);
	    } else {
		return Err(Box::new(Exception::new(&vec!["vector->string"], "vector must contain only characters", context)));
	    }
	}
	return Ok(Value::new_string_from_string(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["vector->string"], "argument must be a vector", context)));
    }
}

fn stdlib_string_to_vector_shape() -> FunctionShape {
    FunctionShape::new(vec!["string".to_string()])
}

fn stdlib_string_to_vector(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let string = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["string->vector"], "missing argument string", context)))?.clone()
    };

    if string.is_string() {
	let string = string.get_string(context)?;
	let mut out = Vec::new();
	for c in string.chars().rev() {
	    out.push(Value::new_char(c));
	}
	return Ok(Value::new_vector(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["string->vector"], "argument must be a string", context)));
    }
}

fn stdlib_integer_to_float_shape() -> FunctionShape {
    FunctionShape::new(vec!["integer".to_string()])
}

fn stdlib_integer_to_float(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let integer = if args.len() == 1 {
	args[0].clone()
    } else {
	keyword_args.get("integer").ok_or(Box::new(Exception::new(&vec!["integer->float"], "missing argument integer", context)))?.clone()
    };

    if integer.is_integer() {
	let integer = integer.get_integer(context)?;
	let float = integer.to_f64();
	return Ok(Value::new_float(float));
    } else {
	return Err(Box::new(Exception::new(&vec!["integer->float"], "argument must be an integer", context)));
    }
}

fn stdlib_map_list_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "list".to_string()])
}

fn stdlib_map_list(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, list) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["map-list"], "missing argument list", context)))?.clone();
	(function, list)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["map-list"], "missing argument function", context)))?.clone();
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["map-list"], "missing argument list", context)))?.clone();
	(function, list)
    };

    if function.is_function() && list.is_pair() {
	let function = function.get_function(context)?;
	let mut current = list.get_pair(context)?;
	let mut out = Value::new_nil();
	while !current.1.is_nil() {
	    let result = function.call_raw(vec![current.0.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	    out = Value::new_pair(result, out, context);
	    current = current.1.get_pair(context)?;
	}
	let result = function.call_raw(vec![current.0.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	out = Value::new_pair(result, out, context);
	return Ok(out);
    } else {
	return Err(Box::new(Exception::new(&vec!["map-list"], "first argument must be a function and second argument must be a list", context)));
    }
}

fn stdlib_map_vector_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "vector".to_string()])
}

fn stdlib_map_vector(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, vector) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["map-vector"], "missing argument vector", context)))?.clone();
	(function, vector)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["map-vector"], "missing argument function", context)))?.clone();
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["map-vector"], "missing argument vector", context)))?.clone();
	(function, vector)
    };

    if function.is_function() && vector.is_vector() {
	let function = function.get_function(context)?;
	let vector = vector.get_vector(context)?;
	let mut out = Vec::new();
	for v in vector.iter() {
	    let result = function.call_raw(vec![v.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	    out.push(result);
	}
	return Ok(Value::new_vector(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["map-vector"], "first argument must be a function and second argument must be a vector", context)));
    }
}

fn stdlib_map_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "string".to_string()])
}

fn stdlib_map_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, string) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["map-string"], "missing argument string", context)))?.clone();
	(function, string)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["map-string"], "missing argument function", context)))?.clone();
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["map-string"], "missing argument string", context)))?.clone();
	(function, string)
    };

    if function.is_function() && string.is_string() {
	let function = function.get_function(context)?;
	let string = string.get_string(context)?;
	let mut out = String::new();
	for c in string.chars() {
	    let result = function.call_raw(vec![Value::new_char(c)], Kwargs::new(), context, &vec![])?.unwrap();
	    if result.is_char() {
		out.push(result.get_char(context)?);
	    } else {
		return Err(Box::new(Exception::new(&vec!["map-string"], "function must return a character", context)));
	    }
	}
	return Ok(Value::new_string_from_string(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["map-string"], "first argument must be a function and second argument must be a string", context)));
    }
}

fn stdlib_filter_list_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "list".to_string()])
}

fn stdlib_filter_list(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, list) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["filter-list"], "missing argument list", context)))?.clone();
	(function, list)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["filter-list"], "missing argument function", context)))?.clone();
	let list = keyword_args.get("list").ok_or(Box::new(Exception::new(&vec!["filter-list"], "missing argument list", context)))?.clone();
	(function, list)
    };

    if function.is_function() && list.is_pair() {
	let function = function.get_function(context)?;
	let mut current = list.get_pair(context)?;
	let mut out = Value::new_nil();
	while !current.1.is_nil() {
	    let result = function.call_raw(vec![current.0.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	    if result.is_boolean() && result.get_boolean(context)? {
		out = Value::new_pair(current.0.clone(), out, context);
	    }
	    current = current.1.get_pair(context)?;
	}
	let result = function.call_raw(vec![current.0.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	if result.is_boolean() && result.get_boolean(context)? {
	    out = Value::new_pair(current.0.clone(), out, context);
	}
	return Ok(out);
    } else {
	return Err(Box::new(Exception::new(&vec!["filter-list"], "first argument must be a function and second argument must be a list", context)));
    }
}

fn stdlib_filter_vector_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "vector".to_string()])
}

fn stdlib_filter_vector(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, vector) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["filter-vector"], "missing argument vector", context)))?.clone();
	(function, vector)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["filter-vector"], "missing argument function", context)))?.clone();
	let vector = keyword_args.get("vector").ok_or(Box::new(Exception::new(&vec!["filter-vector"], "missing argument vector", context)))?.clone();
	(function, vector)
    };

    if function.is_function() && vector.is_vector() {
	let function = function.get_function(context)?;
	let vector = vector.get_vector(context)?;
	let mut out = Vec::new();
	for v in vector.iter() {
	    let result = function.call_raw(vec![v.clone()], Kwargs::new(), context, &vec![])?.unwrap();
	    if result.is_boolean() && result.get_boolean(context)? {
		out.push(v.clone());
	    }
	}
	return Ok(Value::new_vector(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["filter-vector"], "first argument must be a function and second argument must be a vector", context)));
    }
}

fn stdlib_filter_string_shape() -> FunctionShape {
    FunctionShape::new(vec!["function".to_string(), "string".to_string()])
}

fn stdlib_filter_string(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    let (function, string) = if args.len() == 2 {
	(args[0].clone(), args[1].clone())
    } else if args.len() == 1 {
	let function = args[0].clone();
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["filter-string"], "missing argument string", context)))?.clone();
	(function, string)
    } else {
	let function = keyword_args.get("function").ok_or(Box::new(Exception::new(&vec!["filter-string"], "missing argument function", context)))?.clone();
	let string = keyword_args.get("string").ok_or(Box::new(Exception::new(&vec!["filter-string"], "missing argument string", context)))?.clone();
	(function, string)
    };

    if function.is_function() && string.is_string() {
	let function = function.get_function(context)?;
	let string = string.get_string(context)?;
	let mut out = String::new();
	for c in string.chars() {
	    let result = function.call_raw(vec![Value::new_char(c)], Kwargs::new(), context, &vec![])?.unwrap();
	    if result.is_boolean() && result.get_boolean(context)? {
		out.push(c);
	    }
	}
	return Ok(Value::new_string_from_string(out, context));
    } else {
	return Err(Box::new(Exception::new(&vec!["filter-string"], "first argument must be a function and second argument must be a string", context)));
    }
}


fn stdlib_sleep_shape() -> FunctionShape {
    FunctionShape::new(vec!["seconds".to_string()])
}

fn stdlib_sleep(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_exit(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
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

fn stdlib_is_bound_shape() -> FunctionShape {
    FunctionShape::new(vec!["name".to_string()])
}

fn stdlib_is_bound(context: &mut Context, args: Vec<Value>, keyword_args: Kwargs) -> HelperResult<Value> {
    if args.len() == 1 {
	if args[0].is_symbol() {
	    let x = args[0].get_symbol(context)?;
	    return Ok(Value::new_boolean(context.is_bound(x)));
	} else {
	    return Err(Box::new(Exception::new(&vec!["bound?"], "argument must be a symbol", context)));
	}
    } else {
	let x = keyword_args.get("name").unwrap();
	if x.is_symbol() {
	    let x = x.get_symbol(context)?;
	    return Ok(Value::new_boolean(context.is_bound(x)));
	} else {
	    return Err(Box::new(Exception::new(&vec!["bound?"], "argument must be a symbol", context)));
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
    bindings.insert("modulo".to_string(), Value::new_function(Function::Native(stdlib_modulo, stdlib_modulo_shape()), context));
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
    bindings.insert("nil?".to_string(), Value::new_function(Function::Native(stdlib_is_null, stdlib_is_null_shape()), context));
    bindings.insert("char?".to_string(), Value::new_function(Function::Native(stdlib_is_char, stdlib_is_char_shape()), context));
    bindings.insert("append".to_string(), Value::new_function(Function::Native(stdlib_append, stdlib_append_shape()), context));
    bindings.insert("vector-append".to_string(), Value::new_function(Function::Native(stdlib_vector_append, stdlib_vector_append_shape()), context));
    bindings.insert("string-append".to_string(), Value::new_function(Function::Native(stdlib_string_append, stdlib_string_append_shape()), context));
    bindings.insert("list-append".to_string(), Value::new_function(Function::Native(stdlib_list_append, stdlib_list_append_shape()), context));
    bindings.insert("symbol-append".to_string(), Value::new_function(Function::Native(stdlib_symbol_append, stdlib_symbol_append_shape()), context));
    bindings.insert("read-char".to_string(), Value::new_function(Function::Native(stdlib_read_char, stdlib_read_char_shape()), context));
    bindings.insert("read-line".to_string(), Value::new_function(Function::Native(stdlib_read_line, stdlib_read_line_shape()), context));
    bindings.insert("read-string".to_string(), Value::new_function(Function::Native(stdlib_read_string, stdlib_read_string_shape()), context));
    bindings.insert("list-ref".to_string(), Value::new_function(Function::Native(stdlib_list_ref, stdlib_list_ref_shape()), context));
    bindings.insert("vector-ref".to_string(), Value::new_function(Function::Native(stdlib_vector_ref, stdlib_vector_ref_shape()), context));
    bindings.insert("string-ref".to_string(), Value::new_function(Function::Native(stdlib_string_ref, stdlib_string_ref_shape()), context));
    bindings.insert("list-set!".to_string(), Value::new_function(Function::Native(stdlib_list_set, stdlib_list_set_shape()), context));
    bindings.insert("vector-set!".to_string(), Value::new_function(Function::Native(stdlib_vector_set, stdlib_vector_set_shape()), context));
    bindings.insert("string-set!".to_string(), Value::new_function(Function::Native(stdlib_string_set, stdlib_string_set_shape()), context));
    bindings.insert("list-length".to_string(), Value::new_function(Function::Native(stdlib_list_length, stdlib_list_length_shape()), context));
    bindings.insert("vector-length".to_string(), Value::new_function(Function::Native(stdlib_vector_length, stdlib_vector_length_shape()), context));
    bindings.insert("string-length".to_string(), Value::new_function(Function::Native(stdlib_string_length, stdlib_string_length_shape()), context));
    bindings.insert("length".to_string(), Value::new_function(Function::Native(stdlib_length, stdlib_length_shape()), context));
    bindings.insert("list-reverse".to_string(), Value::new_function(Function::Native(stdlib_list_reverse, stdlib_list_reverse_shape()), context));
    bindings.insert("vector-reverse".to_string(), Value::new_function(Function::Native(stdlib_vector_reverse, stdlib_vector_reverse_shape()), context));
    bindings.insert("string-reverse".to_string(), Value::new_function(Function::Native(stdlib_string_reverse, stdlib_string_reverse_shape()), context));
    bindings.insert("reverse".to_string(), Value::new_function(Function::Native(stdlib_reverse, stdlib_reverse_shape()), context));
    bindings.insert("type-name".to_string(), Value::new_function(Function::Native(stdlib_type_name, stdlib_type_name_shape()), context));
    bindings.insert("bound?".to_string(), Value::new_function(Function::Native(stdlib_is_bound, stdlib_is_bound_shape()), context));
    bindings.insert("integer->string".to_string(), Value::new_function(Function::Native(stdlib_integer_to_string, stdlib_integer_to_string_shape()), context));
    bindings.insert("string->integer".to_string(), Value::new_function(Function::Native(stdlib_string_to_integer, stdlib_string_to_integer_shape()), context));
    bindings.insert("list->string".to_string(), Value::new_function(Function::Native(stdlib_list_to_string, stdlib_list_to_string_shape()), context));
    bindings.insert("string->list".to_string(), Value::new_function(Function::Native(stdlib_string_to_list, stdlib_string_to_list_shape()), context));
    bindings.insert("vector->list".to_string(), Value::new_function(Function::Native(stdlib_vector_to_list, stdlib_vector_to_list_shape()), context));
    bindings.insert("list->vector".to_string(), Value::new_function(Function::Native(stdlib_list_to_vector, stdlib_list_to_vector_shape()), context));
    bindings.insert("vector->string".to_string(), Value::new_function(Function::Native(stdlib_vector_to_string, stdlib_vector_to_string_shape()), context));
    bindings.insert("string->vector".to_string(), Value::new_function(Function::Native(stdlib_string_to_vector, stdlib_string_to_vector_shape()), context));
    bindings.insert("integer->float".to_string(), Value::new_function(Function::Native(stdlib_integer_to_float, stdlib_integer_to_float_shape()), context));
    bindings.insert("map-list".to_string(), Value::new_function(Function::Native(stdlib_map_list, stdlib_map_list_shape()), context));
    bindings.insert("map-vector".to_string(), Value::new_function(Function::Native(stdlib_map_vector, stdlib_map_vector_shape()), context));
    bindings.insert("map-string".to_string(), Value::new_function(Function::Native(stdlib_map_string, stdlib_map_string_shape()), context));
    bindings.insert("filter-list".to_string(), Value::new_function(Function::Native(stdlib_filter_list, stdlib_filter_list_shape()), context));
    bindings.insert("filter-vector".to_string(), Value::new_function(Function::Native(stdlib_filter_vector, stdlib_filter_vector_shape()), context));
    bindings.insert("filter-string".to_string(), Value::new_function(Function::Native(stdlib_filter_string, stdlib_filter_string_shape()), context));
    
    




    ContextFrame::new_with_bindings(bindings)
}
