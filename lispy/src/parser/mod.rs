pub mod r#macro;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{str::FromStr, cell::RefCell};

use self::r#macro::Macro;

#[derive(Debug, PartialEq)]
pub struct File {
    index: RefCell<usize>,
    body: Vec<Sexpr>,
}

impl File {
    pub fn new(body: Vec<Sexpr>) -> Self {
	File {
	    index: RefCell::new(0),
	    body,
	}
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	for sexpr in self.body.iter() {
	    write!(f, "{}\n", sexpr)?;
	}
	Ok(())
    }
}

impl Iterator for File {
    type Item = Sexpr;

    fn next(&mut self) -> Option<Self::Item> {
	let index = *self.index.borrow();
	if index < self.body.len() {
	    *self.index.borrow_mut() += 1;
	    Some(self.body[index].clone())
	} else {
	    None
	}
    }
}

pub struct ProtoFile {
    body: Vec<FileObject>,
}

pub enum FileObject {
    Sexpr(Sexpr),
    Comment,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    String(String),
    Integer(String),
    Float(f64),
    Boolean(bool),
    Symbol(Vec<String>),
    Keyword(String),
    Char(char),
    QuotedSymbol(Vec<String>),
    Null,
    Placeholder,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    Atom::String(s) => write!(f, "\"{}\"", s),
	    Atom::Integer(i) => write!(f, "{}", *i),
	    Atom::Float(fl) => write!(f, "{}", *fl),
	    Atom::Boolean(b) => write!(f, "#{}", if *b { "t" } else { "f" }),
	    Atom::Symbol(s) => {
		for (i, s) in s.iter().enumerate() {
		    if i == 0 {
			write!(f, "{}", s)?;
		    } else {
			write!(f, ".{}", s)?;
		    }
		}
		Ok(())
	    },
	    Atom::Keyword(k) => write!(f, ":{}", k),
	    Atom::Char(c) => write!(f, "#\\{}", c),
	    Atom::QuotedSymbol(s) => {
		write!(f, "'")?;
		for (i, s) in s.iter().enumerate() {
		    if i == 0 {
			write!(f, "{}", s)?;
		    } else {
			write!(f, ".{}", s)?;
		    }
		}
		Ok(())
	    },
	    Atom::Null => write!(f, "nil"),
	    Atom::Placeholder => write!(f, "..."),
	}
    }
}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
	match self {
	    Atom::String(s) => s.hash(state),
	    Atom::Integer(i) => i.hash(state),
	    Atom::Float(f) => f.to_string().hash(state),
	    Atom::Boolean(b) => b.hash(state),
	    Atom::Symbol(s) => s.hash(state),
	    Atom::Keyword(k) => k.hash(state),
	    Atom::Char(c) => c.hash(state),
	    Atom::QuotedSymbol(s) => s.hash(state),
	    Atom::Null => "null".hash(state),
	    Atom::Placeholder => "placeholder".hash(state),
	}
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    QuotedList(Vec<Sexpr>),
    VectorList(Vec<Sexpr>),
    
}

impl std::fmt::Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    Sexpr::Atom(a) => write!(f, "{}", a),
	    Sexpr::List(l) => {
		write!(f, "(")?;
		for sexpr in l.iter() {
		    write!(f, "{} ", sexpr)?;
		}
		write!(f, ")")
	    },
	    Sexpr::QuotedList(l) => {
		write!(f, "'(")?;
		for sexpr in l.iter() {
		    write!(f, "{} ", sexpr)?;
		}
		write!(f, ")")
	    },
	    Sexpr::VectorList(l) => {
		write!(f, "#(")?;
		for sexpr in l.iter() {
		    write!(f, "{} ", sexpr)?;
		}
		write!(f, ")")
	    },
	}
	}
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
		    match s {
			"nil" => Err("not symbol but null"),
			"#|" => Err("not symbol but comment"),
			_ => Ok(s.to_string()),
		    }
		}
	    }
	pub(crate) rule scoped_symbol() -> Vec<String>
	    = s:symbol() ++ ['.'] { s }
	pub(crate) rule quoted_symbol() -> Vec<String>
	    = ['\''] s:scoped_symbol() { s }
	pub(crate) rule keyword() -> String
	    = ":" s:symbol() { s }
	pub(crate) rule string() -> String
	    = "\"" s:$(([^'"'] / "\\\"")+) "\"" {?
		let mut buffer = String::new();
		let mut in_escape = false;
		for c in s.chars() {
		    if in_escape {
			match c {
			    '\\' => buffer.push('\\'),
			    'n' => buffer.push('\n'),
			    'r' => buffer.push('\r'),
			    't' => buffer.push('\t'),
			    '"' => buffer.push('"'),
			    _ => { return Err("Bad escape sequence"); },
			}
			in_escape = false;
			continue;
		    }
		    if c == '\\' {
			in_escape = true;
		    } else {
			buffer.push(c)
		    }
		}
		Ok(buffer)
	    }
	pub(crate) rule integer() -> String
	    = s:$(['-' | '+']?['0'..='9']+) { s.to_string() }
	pub(crate) rule float() -> f64
	    = s:$(['-' | '+']?['0'..='9']+ ['.'] ['0'..='9']+) { s.parse().unwrap() }
	pub(crate) rule boolean() -> bool
	    = "#t" { true }
	    / "#f" { false }
	pub(crate) rule character() -> char
	    = "#\\space" { ' ' }
	/ "#\\newline" { '\n' }
	/ "#\\tab" { '\t' }
	/ "#\\return" { '\r' }
	/ "#\\null" { '\0' }
	/ "#\\" c:$([^' ' | '\n' | '\r' | '\0']) { c.chars().nth(0).unwrap() }
	pub(crate) rule atom() -> Atom
	    = f:(float()) { Atom::Float(f) }
	/ c:(character()) { Atom::Char(c) }
		 / s:(string()) { Atom::String(s) }
		 / i:(integer()) { Atom::Integer(i) }
	         / k:(keyword()) { Atom::Keyword(k) }
	/ b:(boolean()) { Atom::Boolean(b) }
	/ "nil" { Atom::Null }
	/ "..." { Atom::Placeholder }
	/ q:(quoted_symbol()) { Atom::QuotedSymbol(q) }
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
	pub(crate) rule vector_list() -> Vec<Sexpr>
	    = ['#'] l:list() { l }
	pub(crate) rule sexpr() -> Sexpr
	    = q:quoted_list() { Sexpr::QuotedList(q) }
	    / v:vector_list() { Sexpr::VectorList(v) }
	    / a:atom() { Sexpr::Atom(a) }
	/ l:list() { Sexpr::List(l) }
	rule comment() -> FileObject
	    = ";" [^'\n']* ['\n'] { FileObject::Comment }
	/ "#;" sexpr() { FileObject::Comment }
	rule file_sexpr() -> FileObject
	= c:comment() { c }
	    / s:sexpr() { FileObject::Sexpr(s) }
	pub rule file() -> ProtoFile
	    = [' '|'\t'|'\n'|'\r']* b:(file_sexpr() ** ([' '|'\t'|'\n'|'\r']*)) [' '|'\t'|'\n'|'\r']* { ProtoFile { body: b } }
    }
}

pub fn parse(input: &str, macros: &mut HashSet<Macro>) -> Result<File, peg::error::ParseError<peg::str::LineCol>> {
    parser::file(input).map(|f| {

	let file = File::new(f.body.into_iter().filter_map(|fo| {
	    match fo {
		FileObject::Sexpr(s) => Some(s),
		FileObject::Comment => None,
	    }
	}).collect());
    //file
	let File { body, .. } = file;
	File::new(r#macro::expand(body, macros))
    })
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

    #[test]
    fn test_vector_list() {
	assert_eq!(parser::vector_list("#(123 456)"), Ok(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))]));
    }

    #[test]
    fn test_file() {
	assert_eq!(parser::file("(123 456)"), Ok(File::new(vec![Sexpr::List(vec![Sexpr::Atom(Atom::Integer("123".to_string())), Sexpr::Atom(Atom::Integer("456".to_string()))])])));
	}

}
