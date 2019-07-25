//! Implements a parser for decentralized identifiers.
//!
//! Given the fact that the
//! [spec](https://w3c-ccg.github.io/did-spec/#generic-did-syntax) is still in
//! draft this module implements the parser using
//! [pest](https://github.com/pest-parser/pest), leading to less than ideal
//! - but still quite ok - performance. This allows for quicker changes
//! adjusting to the spec.
//!
//! When the spec is out of draft stage the parsing backend will be
//! reimplemented using something like [nom](https://github.com/Geal/nom).
//! However, the public interface should remain the same.
//!
//! # Examples
//! ```
//! use did::DID;
//! let d = DID::parse()
//! ```
use std::fmt;
use std::string::ToString;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DIDParser;

/// Decentralized identifier as specified in the
/// [spec](https://w3c-ccg.github.io/did-spec/#generic-did-syntax).
pub struct DID {
    /// [DID method](https://w3c-ccg.github.io/did-spec/#dfn-did-method)
    pub method: String,

    /// The multiple segments of the method specific ID (parts separated by
    /// ':'). At least one String must be here.
    pub id_segments: Vec<String>,

    /// [DID Parameters](https://w3c-ccg.github.io/did-spec/#generic-did-parameter-names)
    /// are separated by ';'. Has an optional value. Optional
    pub params: Vec<(String, Option<String>)>,

    /// [DID Path](https://w3c-ccg.github.io/did-spec/#dfn-did-path) segments.
    /// The parts separated by '/'. Optional.
    pub path_segments: Vec<String>,

    /// [DID Query](https://w3c-ccg.github.io/did-spec/#dfn-did-query).
    /// Optional.
    pub query: Option<String>,

    /// [DID Fragment](https://w3c-ccg.github.io/did-spec/#dfn-did-fragment).
    /// Optional.
    pub fragment: Option<String>
}

impl DID {
    /// Parses a `DID` from anything that implements the `std::string::ToString`
    /// trait.
    pub fn parse<T>(input: T) -> Result<Self, String>
        where T: ToString {
            let input_str = input.to_string();
            let pairs_res = DIDParser::parse(Rule::did, &*input_str);

            match pairs_res {
                Ok(pairs) => return Ok(pairs_to_parsed(pairs)),
                Err(err)  => return Err(err.to_string())
            }
    }
}

impl fmt::Display for DID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = String::from("did:");

        out = out + self.method.as_str();

        if self.id_segments.is_empty() {
            out = out + ":";
        } else {
            for id in self.id_segments.iter() {
                out = out + ":" + id;
            }
        }

        for param in &self.params {
            out = out + ";" + param.0.as_str();
            match &param.1 {
                Some(value) => out = out + "=" + value.as_str(),
                None => {}
            }
        }

        for seg in &self.path_segments {
            out = out + "/" + seg.as_str();
        }

        match &self.query {
            Some(q) => out = out + "?" + q.as_str(),
            None => {}
        }

        match &self.fragment {
            Some(f) => out = out + "#" + f.as_str(),
            None => {}
        }

        write!(f, "{}", out)
    }
}

// implements the parsing logic.
fn pairs_to_parsed(pairs: Pairs<Rule>) -> DID {

    let mut method: String = "".to_string();
    let mut id_segments: Vec<String> = Vec::new();
    let mut params: Vec<(String, Option<String>)> = Vec::new();
    let mut path_segments: Vec<String> = Vec::new();
    let mut query: Option<String> = None;
    let mut fragment: Option<String> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::method => {
                method = pair.as_str().to_string();
            }
            Rule::id_segment => {
                id_segments.push(pair.as_str().to_string());
            }
            Rule::param => {
                let mut inner = pair.into_inner();
                let name = inner.next().expect("parameter with no name");

                match inner.next() {
                    Some(value) => {
                        params.push((
                            name.as_str().to_string(),
                            Some(value.as_str().to_string())
                        ));
                    },
                    None => {
                        params.push((
                            name.as_str().to_string(),
                            None
                        ));
                    }
                }
            }
            Rule::path_segment => {
                path_segments.push(pair.as_str().to_string());
            }
            Rule::query => {
                query = Some(pair.as_str().to_string());
            }
            Rule::fragment => {
                fragment = Some(pair.as_str().to_string());
            }
            _ => unreachable!("unexpected inner rule in 'did'")
        }
    }

    DID {
        method,
        id_segments,
        params,
        path_segments,
        query,
        fragment
    }
}
