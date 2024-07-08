use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn binary_eval(binary_pair: Pair<Rule>, assignments: &HashMap<&str, bool>, binary_op: fn(bool, bool) -> bool) -> Option<bool> {
    let mut pair_iter = binary_pair.into_inner();
    let first_term = evaluate_pair(pair_iter.next().unwrap(), assignments);
    let second_term = evaluate_pair(pair_iter.next().unwrap(), assignments);

    if let (Some(first_result), Some(second_result)) = (first_term, second_term) {
        Some(binary_op(first_result, second_result))
    } else {
        None
    }
}

fn evaluate_pair(pair: Pair<Rule>, assignments: &HashMap<&str, bool>) -> Option<bool> {
    match pair.as_rule() {
        Rule::negation => {
            if let Some(result) = evaluate_pair(pair.into_inner().next().unwrap(), assignments) {
                Some(!result)
            } else {
                None
            }
        }
        Rule::conjunction => {
            binary_eval(pair, assignments, std::ops::BitAnd::bitand)
        }
        Rule::disjunction => {
            binary_eval(pair, assignments, std::ops::BitOr::bitor)
        }
        Rule::material_conditional => {
            binary_eval(pair, assignments, |p, q| !p || q)
        }
        Rule::material_biconditional => {
            binary_eval(pair, assignments, |p, q| (p && q) || (!p && !q))
        }
        Rule::sentence_letter => {
            if let Some(value) = assignments.get(pair.as_str()) {
                Some(*value)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "(A > B)");

    if let Ok(pairs) = pairs_result {
        let assignments = HashMap::from([("A", false), ("B", false)]);
        let sentence = pairs.as_str(); // grab the string

        for p in pairs.into_iter() {
            if let Some(evaluation) = evaluate_pair(p, &assignments) {
                println!("evaluation of |{}|: {}", sentence, evaluation);
            } else {
                println!(
                    "cannot evaluate |{}| with assignments: {:#?}",
                    sentence, assignments
                );
            }
        }
    } else {
        println!("error parsing string: {}", pairs_result.err().unwrap());
    }
}
