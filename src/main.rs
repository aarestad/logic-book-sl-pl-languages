use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

use crate::evaluation::evaluate_pair;
use crate::truth_table::truth_table;

mod evaluation;
mod truth_table;
mod truth_tree;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "(A > B)");

    if let Ok(pairs) = pairs_result {
        let assignments = HashMap::from([("A", false), ("B", false)]);
        let sentence = pairs.as_str();
        let sentence_pair = pairs.clone().next().unwrap();

        if let Some(evaluation) = evaluate_pair(sentence_pair, &assignments) {
            println!(
                "evaluation of |{}|: {}; assignments: {:#?}",
                sentence, evaluation, assignments
            );
        } else {
            println!(
                "cannot evaluate |{}| with assignments: {:#?}",
                sentence, assignments
            );
        }

        println!("truth table:");
        for line in truth_table(&mut pairs.clone()) {
            println!("{}", line);
        }
    } else {
        println!("error parsing string: {}", pairs_result.unwrap_err());
    }
}
