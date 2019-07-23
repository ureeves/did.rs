
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(unused_imports)]
use pest::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DIDParser;

#[test]
fn malele() {
    let did_str = "did:method0:0123456789";
    let pairs = DIDParser::parse(Rule::did, did_str).unwrap();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::method_name => println!("did: {}", inner_pair.as_str()),
                Rule::fragment => println!("Fragment:   {}", inner_pair.as_str()),
                _ => {}
            };
        }
    }
}
