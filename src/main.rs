use pest::Parser;
use pest_derive::Parser;

use crate::truth_table::truth_table;

mod evaluation;
mod truth_table;
mod truth_tree;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "((~ B ⊃ C) & (A ≡ B))");

    if let Ok(pairs) = pairs_result {
        println!("truth table:");

        for line in truth_table(pairs) {
            println!("{}", line);
        }
    } else {
        println!("error parsing string: {}", pairs_result.unwrap_err());
    }
}
