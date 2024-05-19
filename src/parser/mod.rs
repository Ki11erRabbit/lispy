use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct File {
    body: Vec<Sexpr>,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    String(String),
    Integer(String),
    Float(f64),
    Boolean(bool),
    Symbol(Vec<String>),
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
	pub(crate) rule symbol() -> String
	    = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '*' | '/' | '+' | '<' | '>' | '=' | '?' | '!' | '$' | '%' | '&' | ':' | '^' | '~' | '@' | '#' | '|' | '\\' | '\'']+) {?
		if i64::from_str(s).is_ok() || u64::from_str(s).is_ok() || f64::from_str(s).is_ok() {
		    Err("not symbol but a number")
                } else if s == "#t" || s == "#f" {
		    Err("not symbol but a boolean")
		} else if s.chars().nth(0) == Some(':') {
		    Err("not symbol but a keyword")
		} else if s.chars().nth(0) == Some('"') {
		    Err("not symbol but a string")
		} else if s.chars().nth(0) == Some('\'') {
		    Err("not symbol but a quoted list")
		} else {
	            Ok(s.to_string())
		}
	    }
	pub(crate) rule scoped_symbol() -> Vec<String>
	    = s:symbol() ++ ['.'] { s }
	pub(crate) rule keyword() -> String
	    = ":" s:symbol() { s }
	pub(crate) rule string() -> String
	    = "\"" s:$([^'"']+) "\"" { s.to_string() }
	pub(crate) rule integer() -> String
	    = s:$(['-' | '+']?['0'..='9']+) { s.to_string() }
	pub(crate) rule float() -> f64
	    = s:$(['-' | '+']?['0'..='9']+ ['.'] ['0'..='9']+) { s.parse().unwrap() }
	pub(crate) rule boolean() -> bool
	    = "#t" { true }
	    / "#f" { false }
	pub(crate) rule atom() -> Atom
	    = f:(float()) { Atom::Float(f) }
		 / s:(string()) { Atom::String(s) }
		 / i:(integer()) { Atom::Integer(i) }
	         / k:(keyword()) { Atom::Keyword(k) }
	/ b:(boolean()) { Atom::Boolean(b) }
	/ s:(scoped_symbol()) { Atom::Symbol(s) }
	pub(crate) rule paren_list() -> Vec<Sexpr>
	    = ['('] [' '|'\t'|'\n'|'\r']* a:sexpr() ** ([' '|'\t'|'\n'|'\r']*) [')'] { a.into_iter().map(|a| a).collect() }
	pub(crate) rule bracket_list() -> Vec<Sexpr>
	    = ['['] [' '|'\t'|'\n'|'\r']* a:sexpr() ** ([' '|'\t'|'\n'|'\r']*) [']'] { a.into_iter().map(|a| a).collect() }
	pub(crate) rule brace_list() -> Vec<Sexpr>
	    = ['{'] [' '|'\t'|'\n'|'\r']* a:sexpr() ** ([' '|'\t'|'\n'|'\r']*) ['}'] { a.into_iter().map(|a| a).collect() }
	pub(crate) rule list() -> Vec<Sexpr>
	    = paren_list() / bracket_list() / brace_list()
	pub(crate) rule quoted_list() -> Vec<Sexpr>
	    = ['\''] l:list() { l }
	pub rule sexpr() -> Sexpr
	    = a:atom() { Sexpr::Atom(a) }
	    / l:list() { Sexpr::List(l) }
	    / q:quoted_list() { Sexpr::QuotedList(q) }
	    
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol() {
	assert_eq!(parser::symbol("abc"), Ok("abc".to_string()));
    }

    #[test]
    fn test_scoped_symbol() {
	assert_eq!(parser::scoped_symbol("abc.def"), Ok(vec!["abc".to_string(), "def".to_string()]));
    }

    #[test]
    fn test_keyword() {
	assert_eq!(parser::keyword(":abc"), Ok("abc".to_string()));
    }

    #[test]
    fn test_string() {
	assert_eq!(parser::string("\"abc\""), Ok("abc".to_string()));
    }

    #[test]
    fn test_integer() {
	assert_eq!(parser::integer("123"), Ok("123".to_string()));
    }

    #[test]
    fn test_float() {
	assert_eq!(parser::float("123.456"), Ok(123.456));
    }

    #[test]
    fn test_boolean() {
	assert_eq!(parser::boolean("#t"), Ok(true));
	assert_eq!(parser::boolean("#f"), Ok(false));
    }

    #[test]
    fn test_atom() {
	assert_eq!(parser::atom("123"), Ok(Atom::Integer("123".to_string())));
	assert_eq!(parser::atom("123.456"), Ok(Atom::Float(123.456)));
	assert_eq!(parser::atom("\"abc\""), Ok(Atom::String("abc".to_string())));
	assert_eq!(parser::atom("#t"), Ok(Atom::Boolean(true)));
	assert_eq!(parser::atom("#f"), Ok(Atom::Boolean(false)));
	assert_eq!(parser::atom("abc"), Ok(Atom::Symbol(vec!["abc".to_string()])));
    }

    #[test]
    fn test_paren_list() {
	assert_eq!(parser::paren_list("(123 456)"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }

    #[test]
    fn test_bracket_list() {
	assert_eq!(parser::bracket_list("[123 456]"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }

    #[test]
    fn test_brace_list() {
	assert_eq!(parser::brace_list("{123 456}"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }

    #[test]
    fn test_list() {
	assert_eq!(parser::list("(123 456)"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
	assert_eq!(parser::list("[123 456]"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
	assert_eq!(parser::list("{123 456}"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }

    #[test]
    fn test_quoted_list() {
	assert_eq!(parser::quoted_list("'(123 456)"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }


}
