use std::collections::HashMap;
use std::collections::HashSet;

use crate::parser::{Sexpr, Atom};



pub struct Macro {
    header: Vec<Sexpr>,
    body: Sexpr,
    variables: Vec<Vec<String>>,
    bound_positions: HashSet<(usize, usize)>,
}

impl Macro {
    pub fn new(header: Vec<Sexpr>, body: Sexpr) -> Self {
	let mut symbol_positions = HashMap::new();
	let mut possible_variables = Vec::new();
	Macro::get_possible_variables(&header, &mut possible_variables, &mut symbol_positions, 0);
	let mut variables = HashSet::new();
	Macro::find_variable_positions(&body, &mut possible_variables, &mut variables);

	let mut variable_positions = Vec::new();
	let mut bound_positions = HashSet::new();
	for variable in variables.iter() {
	    if possible_variables.contains(variable) {
		variable_positions.push(possible_variables.iter().position(|v| v == variable).unwrap());
		bound_positions.insert(symbol_positions.get(variable).unwrap().clone());
	    }
	}

	let mut variables = Vec::new();
	for i in variable_positions {
	    variables.push(possible_variables[i].clone());
	}

	Ok(Macro {
	    header,
	    body,
	    variables,
	    bound_positions,
	})
    }

    fn get_possible_variables(header: &Vec<Sexpr>, variables: &mut Vec<&Vec<String>>, bound_positions: &mut HashMap<&Vec<String>, (usize, usize)>, level: usize) {
	for (i, sexpr) in header.iter().enumerate() {
	    match sexpr {
		Sexpr::Atom(Atom::Symbol(s)) => {
		    if bound_positions.contains_key(s) {
			todo!("make an error");
		    }
		    variables.push(s);
		    bound_positions.insert(s, (level, i));
		},
		Sexpr::List(l) => {
		    Macro::get_possible_variables(l, variables, bound_positions, level + 1);
		},
		_ => {},
	    }
	}
    }

    fn find_variable_positions(body: &Sexpr, possible_variables: &mut Vec<&Vec<String>>, variables: &mut HashSet<&Vec<String>>) {
	match body {
	    Sexpr::Atom(Atom::Symbol(s)) => {
		variables.insert(s);
	    },
	    Sexpr::List(l) => {
		for sexpr in l {
		    Macro::find_variable_positions(sexpr, possible_variables, variables);
		}
	    },
	    _ => {},
	}
    }

    fn expand(&self, bindings: &MacroContext, list: Vec<Sexpr>) -> Option<Sexpr> {
    }
    
    fn expand_sub(&self, bindings: &mut MacroContext, source: &Vec<Sexpr>, macros: &HashSet<Macro>, level: usize) {
    }
}


struct MacroContext<'a> {
    bindings: Vec<HashSet<&'a Vec<String>>>,
}

impl<'a> MacroContext<'a> {
    pub fn new() -> Self {
	MacroContext {
	    bindings: vec![HashSet::new()],
	}
    }

    pub fn bind(&mut self, variable: &'a Vec<String>) {
	self.bindings.last_mut().unwrap().insert(variable);
    }

    pub fn unbind(&mut self, variable: &'a Vec<String>) {
	self.bindings.last_mut().unwrap().remove(variable);
    }

    pub fn is_bound(&self, variable: &'a Vec<String>) -> bool {
	self.bindings.iter().any(|b| b.contains(variable))
    }

    pub fn push(&mut self) {
	self.bindings.push(HashSet::new());
    }

    pub fn pop(&mut self) {
	self.bindings.pop();
    }
}

pub fn expand(source: Vec<Sexpr>, macros: &HashSet<Macro>) -> Vec<Sexpr> {
    let bindings = MacroContext::new();
    expand_real(&mut bindings, source, macros)
}

fn expand_real(bindings: &mut MacroContext, mut source: Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Vec<Sexpr> {
    let mut output = Vec::new();
    for sexpr in source {
	expand_real_single(bindings, sexpr, macros, level, &mut output);
    }
}

fn expand_real_single(bindings: &mut MacroContext, sexpr: Sexpr, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    match sexpr {
	Sexpr::List(l) => {
	    expand_list(bindings, l, macros, level, output);
	},
	x => {
	    output.push(x);
	},
    }
}

fn expand_list(bindings: &mut MacroContext, list: Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    if list.is_empty() {
	return;
    }
    if let Sexpr::Atom(Atom::Symbol(s)) = &list[0] {
	match s[0].as_str() {
	    "define" => {
		let body = expand_define(bindings, list, macros, output);
		list[2] = body;
		output.push(Sexpr::List(list));
	    },
	    "define-syntax-rule" => expand_define_syntax_rule(bindings, list, macros, output),
	    _ => try_expand_macro(bindings, list, macros, level, output),
	}
    }
}

fn expand_define(bindings: &mut MacroContext, list: Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Sexpr {
    match *list.into_boxed_slice() {
	[_, Sexpr::Atom(Atom::Symbol(s)), body] => {
	    bindings.push();
	    let out = match body {
		Sexpr::Atom(_) => body,
		Sexpr::List(_) => {
		    expand_real(bindings, body, macros);
		    Sexpr::List(out)
		},
	    }; 
	    bindings.pop();
	    bindings.bind(s);
	    out
	},
	[_, Sexpr::List(l), body] => {
	    bindings.bind(&l[0]);
	    bindings.push();
	    for sexpr in l.iter().skip(1) {
		match sexpr {
		    Sexpr::Atom(Atom::Symbol(s)) => {
			bindings.bind(s);
		    },
		    _ => todo!("make an error"),
		}
	    }
	    let out = match body {
		Sexpr::Atom(_) => body,
		Sexpr::List(_) => {
		    expand_real(bindings, body, macros);
		    Sexpr::List(out)
		},
	    };
	    bindings.pop();
	    return out;
	},
	_ => todo!("make an error"),
    }
}

fn expand_define_syntax_rule(bindings: &mut MacroContext, list: Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    match *list.into_boxed_slice() {
	[_, Sexpr::List(header), body] => {
	    let new_body = expand_real(bindings, body, macros);
	    let macro = Macro::new(header, new_body);
	    macros.insert(macro);
	},
	_ => todo!("make an error"),
    }
}

fn try_expand_macro(bindings: &mut MacroContext, list: Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    for macro in macros.iter() {
	let output = macro.expand(bindings, list);
	if let Some(output) = output {
	    output.push(output);
	    return;
	}
    }
    output.push(Sexpr::List(list));
}

