use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

use crate::evaluation::evaluate_pair;

mod evaluation;
mod truth_table;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "(A > B)");

    if let Ok(pairs) = pairs_result {
        let assignments = HashMap::from([("A", false), ("B", false)]);
        let sentence = pairs.as_str();
        let sentence_pair = pairs.clone().into_iter().next().unwrap();

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

        // println!("truth table:");
        // print_truth_table(pairs);
    } else {
        println!("error parsing string: {}", pairs_result.err().unwrap());
    }
}
