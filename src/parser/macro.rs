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
	    Sexpr::List(l) => Macro::expand_body_list(l, variables, bindings, redirect_bindings, level),
	    _ => {
		Some(body.clone())
	    },
	}
    }

    fn expand_body_list<'a>(list: &'a Vec<Sexpr>,
		   variables: &Vec<Vec<String>>,
		   bindings: &MacroContext,
		   redirect_bindings: &mut HashMap<&'a Vec<String>, Sexpr>,
			    level: usize) -> Option<Sexpr> {
	if let Sexpr::Atom(Atom::Symbol(s)) = &list[0] {
	    match s[0].as_str() {
		"define" => {
		    let mut new_list = Vec::new();
		    new_list.push(Sexpr::Atom(Atom::Symbol(s.clone())));
		    new_list.push(list[1].clone());
		    let expanded = Macro::expand_body(&list[2], variables, bindings, redirect_bindings, level);
		    new_list.push(expanded?);
		    Some(Sexpr::List(new_list))
		},
		"let" => {
		    let let_bindings = &list[1];
		    let Sexpr::List(let_bindings) = let_bindings else {
			return None;
		    };
		    let body = &list[2];
		    let mut new_bindings = Vec::new();
		    for binding in let_bindings {
			if let Sexpr::List(l) = binding {
			    let mut new_binding = Vec::new();
			    let Sexpr::Atom(Atom::Symbol(s)) = &l[0] else {
				return None;
			    };
			    if bindings.is_bound(s) {
				let mangled_name = s.iter().map(|s| format!("{}-{}", s, level)).collect::<Vec<String>>();
				redirect_bindings.insert(s, Sexpr::Atom(Atom::Symbol(mangled_name.clone())));
				new_binding.push(Sexpr::Atom(Atom::Symbol(mangled_name)));
			    } else {
				new_binding.push(Sexpr::Atom(Atom::Symbol(s.clone())));
			    }
			    new_binding.push(Macro::expand_body(&l[1], variables, bindings, redirect_bindings, level)?);
			    new_bindings.push(Sexpr::List(new_binding));
			} else {
			    return None;
			}
		    }
		    let expanded_body = Macro::expand_body(body, variables, bindings, redirect_bindings, level)?;
		    let mut new_list = Vec::new();
		    new_list.push(Sexpr::Atom(Atom::Symbol(s.clone())));
		    new_list.push(Sexpr::List(new_bindings));
		    new_list.push(expanded_body);
		    Some(Sexpr::List(new_list))
		},
		"try" => {
		    todo!();
		},
		_ => {
		    let mut new_list = Vec::new();
		    new_list.push(Sexpr::Atom(Atom::Symbol(s.clone())));
		    for sexpr in list.iter().skip(1) {
			let expanded = Macro::expand_body(sexpr, variables, bindings, redirect_bindings, level);
			new_list.push(expanded?);
		    }
		    Some(Sexpr::List(new_list))
		},
	    }
	} else {
	    None
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

struct MacroContext {
    bindings: Vec<HashSet<Vec<String>>>,
}

impl MacroContext {
    pub fn new() -> Self {
	MacroContext {
	    bindings: vec![HashSet::new()],
	}
    }

    pub fn bind(&mut self, variable: Vec<String>) {
	self.bindings.last_mut().unwrap().insert(variable);
    }

    pub fn is_bound(&self, variable: &Vec<String>) -> bool {
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

fn expand_real<'a>(bindings: &mut MacroContext, source: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Vec<Sexpr> {
    let mut output = Vec::new();
    for sexpr in source.iter() {
	match expand_real_single(bindings, sexpr, macros) {
	    Some(sexpr) => {
		output.push(sexpr);
	    },
	    None => {},
	}
    }
    output
}

fn expand_real_single<'a> (bindings: &mut MacroContext, sexpr: &'a Sexpr, macros: &mut HashSet<Macro>) -> Option<Sexpr> {
    match sexpr {
	Sexpr::List(l) => {
	    expand_list(bindings, l, macros)
	},
	x => {
	    Some(x.clone())
	},
    }
}

fn expand_list<'a>(bindings: &mut MacroContext, list: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Option<Sexpr> {
    if list.is_empty() {
	//return;
    }
    if let Sexpr::Atom(Atom::Symbol(s)) = &list[0] {
	match s[0].as_str() {
	    "define" => {
		let body = expand_define(bindings, list, macros);
		let new_list = vec![Sexpr::Atom(Atom::Symbol(vec!["define".to_string()])), list[1].clone(), body];
		Some(Sexpr::List(new_list))
	    },
	    "define-syntax-rule" => expand_define_syntax_rule(bindings, list, macros),
	    "let" => {
		//println!("{:?}", list);
		match list.as_slice() {
		    [_, Sexpr::List(let_bindings), body] => {
			bindings.push();
			let mut new_bindings = Vec::new();
			for binding in let_bindings {
			    match binding {
				Sexpr::List(sets) => {
				    match sets.as_slice() {
					[Sexpr::Atom(Atom::Symbol(s)), body] => {
					    bindings.bind(s.clone());
					    if let Some(expanded) = expand_real_single(bindings, body, macros) {
						new_bindings.push(Sexpr::List(vec![Sexpr::Atom(Atom::Symbol(s.clone())), expanded]));
					    }
					},
					_ => todo!("make an error"),
				    }
				},
				_ => todo!("make an error"),
			    }
			}
			let new_list = if let Some(expanded_body) = expand_real_single(bindings, body, macros) {
			    vec![Sexpr::Atom(Atom::Symbol(vec!["let".to_string()])), Sexpr::List(new_bindings), expanded_body]
			} else {
			    vec![Sexpr::Atom(Atom::Symbol(vec!["let".to_string()])), Sexpr::List(new_bindings), body.clone()]
			};
			//println!("{:?}", new_list);
			let out = Sexpr::List(new_list);
			bindings.pop();
			Some(out)
		    },
		    _ => todo!("make an error"),
		}
	    },
	    "match" => {
		//println!("{:?}", list);
		match list.as_slice() {
		    [_, value, cases @ ..] => {
			let expanded_value = expand_real_single(bindings, value, macros);
			let mut new_cases = Vec::new();
			for case in cases {
			    bindings.push();
			    let Sexpr::List(case) = case else {
				todo!("make an error");
			    };
			    match case.as_slice() {
				[Sexpr::List(clause), body] => {
				    let clause_iter = clause.iter().skip(1);
				    for clause in clause_iter {
					let Sexpr::Atom(Atom::Symbol(s)) = clause else {
					    todo!("make an error");
					};
					bindings.bind(s.clone());
				    }
				    let expanded_body = expand_real_single(bindings, body, macros);
				    new_cases.push(Sexpr::List(vec![Sexpr::List(clause.clone()), expanded_body?]));
				},
				[Sexpr::Atom(Atom::Symbol(s)), body] => {
				    match s[0].as_str() {
					"else" => {
					    let expanded_body = expand_real_single(bindings, body, macros);
					    new_cases.push(Sexpr::List(vec![Sexpr::Atom(Atom::Symbol(s.clone())), expanded_body?]));
					},
					_ => {
					    bindings.bind(s.clone());
					    let expanded_body = expand_real_single(bindings, body, macros);
					    new_cases.push(Sexpr::List(vec![Sexpr::Atom(Atom::Symbol(s.clone())), expanded_body?]));
					}
				    }
				},
				[Sexpr::Atom(_), body] => {
				    let expanded_body = expand_real_single(bindings, body, macros);
				    new_cases.push(Sexpr::List(vec![case[0].clone(), expanded_body?]));
				}, 
				[Sexpr::QuotedList(_), _] => {
				    todo!("quoted list in match");
				},
				[Sexpr::VectorList(_), _] => {
				    todo!("vector list in match");
				}, 

				_ => todo!("make an error"),
			    }
			    bindings.pop();
			}
			let mut out = vec![list[0].clone(), expanded_value?];
			out.extend(new_cases);
			//println!("{:?}", out);
			Some(Sexpr::List(out))
		    },
		    _ => todo!("make an error"),
		}
	    }
	    "try" => {
		match list.as_slice() {
		    [_, body, handlers @ ..] => {
			let mut new_handlers = Vec::new();
			let expanded_body = expand_real_single(bindings, body, macros);
			for handler in handlers {
			    bindings.push();
			    let Sexpr::List(handler) = handler else {
				todo!("make an error");
			    };
			    match handler.as_slice() {
				[Sexpr::List(clause), body] => {
				    let Sexpr::Atom(Atom::Symbol(s)) = &clause[0] else {
					todo!("make an error");
				    };
				    bindings.bind(s.clone());
				    let expanded_body = expand_real_single(bindings, body, macros);
				    new_handlers.push(Sexpr::List(vec![Sexpr::List(clause.clone()), expanded_body?]));
				},
				_ => todo!("make an error"),
			    }
			    bindings.pop();
			}
			let mut out = vec![list[0].clone(), expanded_body?];
			out.extend(new_handlers);
			Some(Sexpr::List(out))
		    },
		    _ => todo!("make an error"),
		}
	    },
	    _ => try_expand_macro(bindings, list, macros),
	}
    } else {
	return Some(Sexpr::List(list.clone()));
    }
}

fn expand_define<'a>(bindings: &mut MacroContext, list: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Sexpr {
    match list.as_slice() {
	[_, Sexpr::Atom(Atom::Symbol(s)), body] => {
	    bindings.push();
	    let out = match body {
		Sexpr::Atom(_) => body.clone(),
		Sexpr::List(l) => {
		    let out = expand_real(bindings, l, macros);
		    Sexpr::List(out)
		},
		_ => body.clone(),
	    }; 
	    bindings.pop();
	    bindings.bind(s.clone());
	    out
	},
	[_, Sexpr::List(l), body] => {
	    let Sexpr::Atom(Atom::Symbol(s)) = &l[0] else {
		todo!("make an error");
	    };
	    bindings.bind(s.clone());
	    bindings.push();
	    for sexpr in l.iter().skip(1) {
		match sexpr {
		    Sexpr::Atom(Atom::Symbol(s)) => {
			bindings.bind(s.clone());
		    },
		    _ => {},
		}
	    }
	    let out = match body {
		Sexpr::Atom(_) => body.clone(),
		Sexpr::List(l) => {
		    let out = expand_real(bindings, l, macros);
		    Sexpr::List(out)
		},
		_ => body.clone(),
	    };
	    bindings.pop();
	    return out;
	},
	_ => todo!("make an error"),
    }
}

fn expand_define_syntax_rule<'a>(bindings: &mut MacroContext, list: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Option<Sexpr> {
    match list.as_slice() {
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
    None
}

fn try_expand_macro<'a>(bindings: &'a mut MacroContext, list: &'a Vec<Sexpr>, macros: &mut HashSet<Macro>) -> Option<Sexpr> {
    for macro_ in macros.iter() {
	let out = macro_.expand(bindings, list.clone());
	if let Some(out) = out {
	    return Some(out);
	}
    }
    Some(Sexpr::List(list.clone()))
}
