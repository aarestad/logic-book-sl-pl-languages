use std::collections::BTreeSet;

use pest::iterators::{Pair, Pairs};

use crate::Rule;

pub(crate) fn print_truth_table(top: &mut Pairs<Rule>) {
    let sentence = top.next().unwrap();
    let sentence_str = sentence.as_str();

    let mut atoms = BTreeSet::new();
    find_atoms_recurse(sentence.into_inner().next().unwrap(), &mut atoms);

    if atoms.len() > 5 {
        panic!("too many atoms to print a truth table: {}", atoms.len());
    }

    print_header(&atoms, sentence_str);
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

fn print_header(atoms: &BTreeSet<String>, sentence_str: &str) {
    let header_len = atoms.len() * 2 + sentence_str.len() + 2;

    println!("{} | {}", itertools::join(atoms, " "), sentence_str);
    println!("{}", "-".repeat(header_len));
}