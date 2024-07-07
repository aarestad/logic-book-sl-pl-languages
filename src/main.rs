use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn evaluate_pair(pair: Pair<Rule>, assignments: &HashMap<&str, bool>) -> Option<bool> {
    match pair.as_rule() {
        Rule::negation => {
            let mut inner = pair.clone().into_inner();
            let negation_target = inner.nth(1).unwrap().into_inner().next().unwrap();
            let evaluation = evaluate_pair(negation_target, assignments);

            if let Some(result) = evaluation {
                Some(!result)
            } else {
                None
            }
        }
        Rule::sentence_letter => {
            let letter = pair.as_str();

            if assignments.contains_key(letter) {
                Some(*assignments.get(letter).unwrap())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "~A");

    if let Ok(pairs) = pairs_result {
        let assignments = HashMap::from([("B", false)]);
        let sentence = pairs.as_str();

        for p in pairs.into_iter() {
            if let Some(evaluation) =
                evaluate_pair(p.into_inner().next().unwrap(), &assignments)
            {
                println!("evaluation: {}", evaluation);
            } else {
                println!(
                    "cannot evaluate |{}| with assignments: {:#?}",
                    sentence,
                    assignments
                );
            }
        }
    } else {
        println!("error parsing string: {}", pairs_result.err().unwrap());
    }
}
