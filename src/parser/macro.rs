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
    pub fn new(header: Vec<Sexpr>, body: Sexpr) -> Result<Self, String> {
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
}


struct MacroContext<'a> {
    bindings: HashSet<&'a Vec<String>>
}

impl<'a> MacroContext<'a> {
    pub fn new() -> Self {
	MacroContext {
	    bindings: HashSet::new(),
	}
    }

    pub fn bind(&mut self, variable: &'a Vec<String>) {
	self.bindings.insert(variable);
    }

    pub fn unbind(&mut self, variable: &'a Vec<String>) {
	self.bindings.remove(variable);
    }

    pub fn is_bound(&self, variable: &'a Vec<String>) -> bool {
	self.bindings.contains(variable)
    }
}

pub fn expand(source: Vec<Sexpr>, macros: &HashSet<Macro>) -> Vec<Sexpr> {
    let bindings = MacroContext::new();
    expand_real(&mut bindings, source, macros)
}

fn expand_real(bindings: &mut MacroContext, mut source: Vec<Sexpr>, macros: &HashSet<Macro>) -> Vec<Sexpr> {
    let level = 0;
}

fn expand_sub(bindings: &mut MacroContext, source: &Vec<Sexpr>, macros: &HashSet<Macro>, level: usize, output: &mut Vec<Sexpr>) {
}
