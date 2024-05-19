use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct File {
    body: Vec<Sexpr>,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Symbol(String),
    Keyword(String),
}

#[derive(Debug, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    QuotedList(Vec<Sexpr>),
    
}

peg::parser!{
    grammar parser() for str {
	rule symbol() -> String
	    = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '*' | '/' | '+' | '<' | '>' | '=' | '?' | '!' | '.' | '$' | '%' | '&' | ':' | '^' | '~' | '@' | '#' | '|' | '\\' | '\'']+) {?
		if i64::from_str(s).is_ok() || u64::from_str(s).is_ok() || f64::from_str(s).is_ok() {
		    Err("not symbol but a number")
                } else if s == "#t" || s == "#f" {
		    Err("not symbol but a boolean")
		} else {
	            Ok(s.to_string())
		}
	    }
	rule keyword() -> String
	    = ":" s:symbol() { s }
	rule string() -> String
	    = "\"" s:$([^'"']+) "\"" { s.to_string() }
	rule integer() -> i64
	    = s:$(['-' | '+']?['0'..='9']+) { s.parse().unwrap() }
	rule unsigned_integer() -> u64
	    = s:$(['0'..='9']+) { s.parse().unwrap() }
	rule float() -> f64
	    = s:$(['-' | '+']?['0'..='9']+ (['.'] ['0'..='9']+)) { s.parse().unwrap() }
	rule boolean() -> bool
	    = "#t" { true }
	    / "#f" { false }
	rule atom() -> Atom
	    = s:(string()) { Atom::String(s) }
		 / i:(integer()) { Atom::Integer(i) }
		 / u:(unsigned_integer()) { Atom::UnsignedInteger(u) }
		 / f:(float()) { Atom::Float(f) }
		 / b:(boolean()) { Atom::Boolean(b) }
		 / s:(symbol()) { Atom::Symbol(s) }
	/ k:(keyword()) { Atom::Keyword(k) }
	rule paren_list() -> Vec<Sexpr>
	    = ['('] [' '|'\t'|'\n'|'\r']* a:atom() ** ([' '|'\t'|'\n'|'\r']*) [')'] { a.into_iter().map(|a| Sexpr::Atom(a)).collect() }
	rule bracket_list() -> Vec<Sexpr>
	    = ['['] [' '|'\t'|'\n'|'\r']* a:atom() ** ([' '|'\t'|'\n'|'\r']*) [']'] { a.into_iter().map(|a| Sexpr::Atom(a)).collect() }
	rule brace_list() -> Vec<Sexpr>
	    = ['{'] [' '|'\t'|'\n'|'\r']* a:atom() ** ([' '|'\t'|'\n'|'\r']*) ['}'] { a.into_iter().map(|a| Sexpr::Atom(a)).collect() }
	rule list() -> Vec<Sexpr>
	    = paren_list() / bracket_list() / brace_list()
	rule quoted_list() -> Vec<Sexpr>
	    = ['\''] l:list() { l }
	pub rule sexpr() -> Sexpr
	    = a:atom() { Sexpr::Atom(a) }
	    / l:list() { Sexpr::List(l) }
	    / q:quoted_list() { Sexpr::QuotedList(q) }
	    
    }
}
