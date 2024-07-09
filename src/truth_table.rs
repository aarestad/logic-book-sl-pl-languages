use std::collections::BTreeSet;

use pest::iterators::{Pair, Pairs};

use crate::Rule;

pub(crate) fn truth_table(top: &mut Pairs<Rule>) -> Vec<String> {
    let sentence = top.next().unwrap();
    let sentence_str = sentence.as_str();

    // we want to keep the atoms sorted alphabetically
    let mut atoms = BTreeSet::new();
    find_atoms_recurse(sentence.into_inner().next().unwrap(), &mut atoms);

    if atoms.len() > 5 {
        panic!("too many atoms to print a truth table: {}", atoms.len());
    }

    let mut truth_table = header(&atoms, sentence_str);

    // for each unique assignment, print the row
    truth_table
}

fn find_atoms_recurse(pair: Pair<Rule>, atoms: &mut BTreeSet<String>) {
    match pair.as_rule() {
        Rule::sentence_letter => {
            atoms.insert(pair.as_str().into());
        }
        _ => {
            for p in pair.into_inner() {
                find_atoms_recurse(p, atoms);
            }
        }
    }
}

fn header(atoms: &BTreeSet<String>, sentence_str: &str) -> Vec<String> {
    let header_len = atoms.len() * 2 + sentence_str.len() + 2;

    vec![
        format!("{} | {}", itertools::join(atoms, " "), sentence_str),
        format!("{}", "-".repeat(header_len)),
    ]
}
