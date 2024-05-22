use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::parser::{Sexpr, Atom};

pub struct Macro {
    header: Vec<Sexpr>,
    body: Sexpr,
    variables: Vec<Vec<String>>,
    //bound_positions: HashSet<(usize, usize)>,
}

impl Macro {
    pub fn new(header: Vec<Sexpr>, body: Sexpr) -> Self {
	let mut symbol_positions = HashMap::new();
	let mut possible_variables = Vec::new();
	Macro::get_possible_variables(&header, &mut possible_variables, &mut symbol_positions, 0);
	let mut variables = HashSet::new();
	Macro::find_variable_positions(&body, &mut possible_variables, &mut variables);

	let mut variable_positions = Vec::new();
	//let mut bound_positions = HashSet::new();
	for variable in variables.iter() {
	    if possible_variables.contains(variable) {
		variable_positions.push(possible_variables.iter().position(|v| v == variable).unwrap());
		//bound_positions.insert(symbol_positions.get(variable).unwrap().clone());
	    }
	}

	let mut variables = Vec::new();
	for i in variable_positions {
	    variables.push(possible_variables[i].clone());
	}

	Macro {
	    header,
	    body,
	    variables,
	    //bound_positions,
	}
    }

    fn get_possible_variables<'a>(header: &'a Vec<Sexpr>, variables: &mut Vec<&'a Vec<String>>, bound_positions: &mut HashMap<&'a Vec<String>, (usize, usize)>, level: usize) {
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

    fn find_variable_positions<'a>(body: &'a Sexpr, possible_variables: &mut Vec<&'a Vec<String>>, variables: &mut HashSet<&'a Vec<String>>) {
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
	let mut redirect_bindings = HashMap::new();
	let level = 0;
	Macro::bind_header(&self.header, &self.variables, &bindings, &list, level, &mut redirect_bindings)?;

	Macro::expand_body(&self.body, &self.variables, &bindings, &mut redirect_bindings, level) 
    }
    
    fn bind_header<'a>(header: &'a Vec<Sexpr>,
		  variables: &Vec<Vec<String>>,
		  bindings: &MacroContext,
		  source: &'a Vec<Sexpr>,
		  level: usize,
		  redirect_bindings: &mut HashMap<&'a Vec<String>, Sexpr>) -> Option<()> {
	if header.len() != source.len() {
	    return None;
	}
	for (header, source) in header.iter().zip(source.iter()) {
	    match (header, &source) {
		(Sexpr::List(l1), Sexpr::List(l2)) => {
		    if let Some(()) = Macro::bind_header(l1, variables, bindings, l2, level + 1, redirect_bindings) {
			continue;
		    } else {
			return None;
		    }
		},
		(Sexpr::Atom(Atom::Symbol(s1)), Sexpr::Atom(Atom::Symbol(s2))) => {
		    if bindings.is_bound(&s2) && variables.contains(s1) {
			if redirect_bindings.contains_key(&s2) {
			    let mangled_name = s2.iter().map(|s| format!("{}-{}", s, level)).collect();
			    redirect_bindings.insert(&s2, Sexpr::Atom(Atom::Symbol(mangled_name))); 
			}
			redirect_bindings.insert(s1, source.clone());
		    } else if s1 != s2 {
			return None;
		    }
		},
		(Sexpr::Atom(Atom::Symbol(s1)), x) => {
		    if variables.contains(s1) {
			redirect_bindings.insert(s1, (*x).clone());
			return Some(());
		    } else {
			return None;
		    }
		}
		_ => {
		    return None;
		}
	    }
	}
	Some(())
    }

    fn expand_body<'a>(body: &'a Sexpr,
	      variables: &Vec<Vec<String>>,
	      bindings: &MacroContext,
	      redirect_bindings: &mut HashMap<&'a Vec<String>, Sexpr>,
              level: usize) -> Option<Sexpr> {
	match body {
	    Sexpr::Atom(Atom::Symbol(s)) => {
		if let Some(x) = redirect_bindings.get(s) {
		    Some(x.clone())
		} else {
		    if bindings.is_bound(s) {
			let mangled_name = s.iter().map(|s| format!("{}-{}", s, level)).collect::<Vec<String>>();
			redirect_bindings.insert(s, Sexpr::Atom(Atom::Symbol(mangled_name.clone())));
			Some(Sexpr::Atom(Atom::Symbol(mangled_name)))
		    } else {
			Some(Sexpr::Atom(Atom::Symbol(s.clone())))
		    }
		}
	    },
	    Sexpr::List(l) => {
		let mut output = Vec::new();
		for sexpr in l {
		    if let Some(x) = Macro::expand_body(sexpr, variables, bindings, redirect_bindings, level + 1) {
			output.push(x);
		    } else {
			return None;
		    }
		}
		Some(Sexpr::List(output))
	    },
	    _ => {
		Some(body.clone())
	    },
	}
    }

}

impl PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
	self.header == other.header
    }
}

impl Hash for Macro {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.header.hash(state);
    }
}

impl Eq for Macro {
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

pub fn expand(mut source: Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Vec<Sexpr> {
    let mut bindings = MacroContext::new();
    expand_real(&mut bindings, &mut source, macros)
}

fn expand_real<'a>(bindings: &mut MacroContext<'a>, source: &'a mut Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Vec<Sexpr> {
    let mut output = Vec::new();
    for sexpr in source.iter_mut() {
	expand_real_single(bindings, sexpr, macros, &mut output);
    }
    output
}

fn expand_real_single<'a> (bindings: &mut MacroContext<'a>, sexpr: &'a mut Sexpr, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    match sexpr {
	Sexpr::List(ref mut l) => {
	    expand_list(bindings, l, macros, output);
	},
	x => {
	    output.push(x.clone());
	},
    }
}

fn expand_list<'a>(bindings: &mut MacroContext<'a>, list: &'a mut Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    if list.is_empty() {
	return;
    }
    if let Sexpr::Atom(Atom::Symbol(s)) = &list[0] {
	match s[0].as_str() {
	    "define" => {
		let body = expand_define(bindings, list, macros);
		let new_list = vec![Sexpr::Atom(Atom::Symbol(vec!["define".to_string()])), list[1].clone(), body];
		output.push(Sexpr::List(new_list));
	    },
	    "define-syntax-rule" => expand_define_syntax_rule(bindings, list, macros, output),
	    _ => try_expand_macro(bindings, list, macros, output),
	}
    }
}

fn expand_define<'a>(bindings: &mut MacroContext<'a>, list: &'a mut Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Sexpr {
    match list.as_mut_slice() {
	[_, Sexpr::Atom(Atom::Symbol(s)), body] => {
	    bindings.push();
	    let out = match body {
		Sexpr::Atom(_) => body.clone(),
		Sexpr::List(l) => {
		    let out = expand_real(bindings, l, macros);
		    Sexpr::List(out)
		},
		_ => todo!("make an error"),
	    }; 
	    bindings.pop();
	    bindings.bind(&s);
	    out
	},
	[_, Sexpr::List(l), body] => {
	    let Sexpr::Atom(Atom::Symbol(s)) = &l[0] else {
		todo!("make an error");
	    };
	    bindings.bind(s);
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
		Sexpr::Atom(_) => body.clone(),
		Sexpr::List(l) => {
		    let out = expand_real(bindings, l, macros);
		    Sexpr::List(out)
		},
		_ => todo!("make an error"),
	    };
	    bindings.pop();
	    return out;
	},
	_ => todo!("make an error"),
    }
}

fn expand_define_syntax_rule<'a>(bindings: &mut MacroContext<'a>, list: &'a mut Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    match list.as_mut_slice() {
	[_, Sexpr::List(header), body] => {
	    let out = match body {
		Sexpr::List(l) => {
		    let out = expand_real(bindings, l, macros);

		    Sexpr::List(out)
		},
		Sexpr::Atom(_) => body.clone(),
		_ => todo!("make an error"),
	    };
	    let macro_ = Macro::new(header.clone(), out);
	    macros.insert(macro_);
	},
	_ => todo!("make an error"),
    }
}

fn try_expand_macro<'a>(bindings: &'a mut MacroContext, list: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>, output: &mut Vec<Sexpr>) {
    for macro_ in macros.iter() {
	let out = macro_.expand(bindings, list.clone());
	if let Some(out) = out {
	    output.push(out);
	    return;
	}
    }
    output.push(Sexpr::List(list.clone()));
}
