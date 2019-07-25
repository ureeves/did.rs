use std::fmt;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DIDParser;

// Parsed decentralized identifier
pub struct DID {
    pub method: String,

    pub id_segments: Vec<String>,
    pub params: Vec<(String, Option<String>)>,

    pub path_segments: Vec<String>,

    pub query: Option<String>,
    pub fragment: Option<String>
}

impl DID {
    pub fn parse<'a, T>(input: T) -> Result<Self, String>
        where T: fmt::Display {
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

        if !self.params.is_empty() {
            out = out + ";";
            for param in &self.params {
                out = out + ";" + param.0.as_str();
                match &param.1 {
                    Some(value) => out = out + "=" + value.as_str(),
                    None => {}
                }
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
