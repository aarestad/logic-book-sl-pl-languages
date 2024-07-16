use std::collections::{BTreeSet, HashMap};

use itertools::{iproduct, zip_eq};
use pest::iterators::{Pair, Pairs};

use crate::sl::evaluation::evaluate_pair;
use crate::sl::Rule;

pub(crate) fn truth_table(mut top: Pairs<Rule>) -> Vec<String> {
    let sentence = top.next().unwrap();
    let sentence_str = sentence.as_str();

    // we want to keep the atoms sorted alphabetically
    let mut atoms = BTreeSet::new();
    find_atoms_recurse(sentence.clone().into_inner().next().unwrap(), &mut atoms);

    let mut truth_table = header(&atoms, sentence_str);

    // TODO macro?
    match &atoms.len() {
        0 => (),

        1 => {
            for a in [true, false] {
                truth_table.push(evaluate_row(atoms.iter(), [a], &sentence));
            }
        }

        2 => {
            for (a, b) in iproduct!([true, false], [true, false]) {
                truth_table.push(evaluate_row(atoms.iter(), [a, b], &sentence));
            }
        }

        3 => {
            for (a, b, c) in iproduct!([true, false], [true, false], [true, false]) {
                truth_table.push(evaluate_row(atoms.iter(), [a, b, c], &sentence));
            }
        }

        4 => {
            for (a, b, c, d) in
                iproduct!([true, false], [true, false], [true, false], [true, false])
            {
                truth_table.push(evaluate_row(atoms.iter(), [a, b, c, d], &sentence));
            }
        }

        5 => {
            for (a, b, c, d, e) in iproduct!(
                [true, false],
                [true, false],
                [true, false],
                [true, false],
                [true, false]
            ) {
                truth_table.push(evaluate_row(atoms.iter(), [a, b, c, d, e], &sentence));
            }
        }

        _ => panic!("too may atoms; {}", atoms.len()),
    }

    truth_table
}

fn evaluate_row<'a, const N: usize>(
    atoms: impl Iterator<Item = &'a String>,
    atom_values: [bool; N],
    sentence: &Pair<Rule>,
) -> String {
    let mut assignments = HashMap::new();

    for (atom, assignment) in zip_eq(atoms, atom_values) {
        assignments.insert(atom.as_str(), assignment);
    }

    let mut row_components = vec![];

    for value in atom_values {
        row_components.push(if value { "T " } else { "F " });
    }

    row_components.push("| ");
    let evaluation_option = evaluate_pair(sentence, &assignments);

    if let Some(evaluation) = evaluation_option {
        row_components.push(if evaluation { "T" } else { "F" });
    } else {
        row_components.push("?");
    }

    row_components.join("")
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
    let header_len = atoms.len() * 2 + sentence_str.len() - 2;

    vec![
        format!("{} | {}", itertools::join(atoms, " "), sentence_str),
        format!("{}", "-".repeat(header_len)),
    ]
}
