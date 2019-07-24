use std::convert::{TryFrom};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused_imports)]
use pest::Parser;
use pest::iterators::Pairs;
use pest::error::Error as PestError;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DIDParser;

// Parsed decentralized identifier
#[derive(Debug)]
pub struct DID {
    pub method: String,
    pub specific_id: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>
}

pub struct ParserError(PestError<Rule>);

impl DID {
    pub fn parse<'a, T>(input: T) -> Result<Self, ParserError>
        where T: Into<&'a str> {
            let input_str = input.into();
            let pairs_res = DIDParser::parse(Rule::did_url, input_str);

            match pairs_res {
                Ok(pairs) => return Ok(pairs_to_did(pairs)),
                Err(err)  => return Err(ParserError(err))
            }
    }
}

impl Into<String> for DID {
     fn into(self) -> String {
        let mut out: String = String::from("did:");

        out = out + self.method.as_str() + ":";
        match self.specific_id {
            Some(id) => { out = out + id.as_str() },
            None => {},
        }
        out = out + "/";
        match self.query {
            Some(q) => { out = out + "?" + q.as_str() },
            None => {},
        }
        match self.fragment {
            Some(f) => { out = out + "#" + f.as_str() },
            None => {},
        }

        out
     }
}

impl TryFrom<&str> for DID {
    type Error = ParserError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DID::parse(value)
    }
}

fn pairs_to_did(pairs: Pairs<Rule>) -> DID {

    let mut method_name_str: &str = "";
    let mut method_specific_id_str: &str = "";
    let mut query_str: &str = "";
    let mut fragment_str: &str = "";

    for did_url in pairs {
        for pair in did_url.into_inner() {
            match pair.as_rule() {
                Rule::did => {
                    println!("did: {}", pair.as_str());
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::method_name => {
                                method_name_str = inner_pair.as_str()
                            },
                            Rule::method_specific_id => {
                                method_specific_id_str = inner_pair.as_str()
                            },
                            _ => unreachable!("should not get here")
                        }
                    }
                },
                Rule::param => {}, // TODO: handle parameters
                Rule::query => {
                    query_str = pair.as_str();
                },
                Rule::fragment => {
                    fragment_str = pair.as_str();
                },
                _ => panic!("unexpected token, should have failed parse")
            }
        }
    }

    DID {
        method: String::from(method_name_str),
        specific_id:
            if method_specific_id_str.len() == 0 { None }
            else { Some(String::from(method_specific_id_str)) },
        query:
            if query_str.len() == 0 { None }
            else { Some(String::from(query_str)) },
        fragment:
            if fragment_str.len() == 0 { None }
            else { Some(String::from(fragment_str)) }
    }
}
