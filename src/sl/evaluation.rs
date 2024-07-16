use std::collections::HashMap;

use pest::iterators::Pair;

use crate::sl::Rule;

pub(crate) fn evaluate_pair(pair: &Pair<Rule>, assignments: &HashMap<&str, bool>) -> Option<bool> {
    match pair.as_rule() {
        Rule::sentence => evaluate_pair(&pair.clone().into_inner().next().unwrap(), assignments),
        Rule::negation => evaluate_pair(&pair.clone().into_inner().next().unwrap(), assignments)
            .map(|result| !result),
        Rule::conjunction => binary_eval(&pair.clone(), assignments, std::ops::BitAnd::bitand),
        Rule::disjunction => binary_eval(&pair.clone(), assignments, std::ops::BitOr::bitor),
        Rule::material_conditional => binary_eval(&pair.clone(), assignments, |p, q| !p || q),
        Rule::material_biconditional => {
            binary_eval(&pair.clone(), assignments, |p, q| (p && q) || (!p && !q))
        }
        Rule::sentence_letter => assignments.get(pair.as_str()).copied(),
        _ => None,
    }
}

fn binary_eval(
    binary_pair: &Pair<Rule>,
    assignments: &HashMap<&str, bool>,
    binary_op: fn(bool, bool) -> bool,
) -> Option<bool> {
    let mut pair_iter = binary_pair.clone().into_inner();
    let first_term = evaluate_pair(&pair_iter.next().unwrap(), assignments);
    let second_term = evaluate_pair(&pair_iter.next().unwrap(), assignments);

    if let (Some(first_result), Some(second_result)) = (first_term, second_term) {
        Some(binary_op(first_result, second_result))
    } else {
        None
    }
}
