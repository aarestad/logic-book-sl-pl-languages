use pest::iterators::Pairs;

use crate::Rule;

pub(crate) fn print_truth_table(sentence: Pairs<Rule>) {
    let _atoms = atoms_for_sentence(sentence);
    todo!()
}

fn atoms_for_sentence(_sentence: Pairs<Rule>) -> Vec<&str> {
    let _atoms: Vec<&str> = vec![];

    // find_atoms_recurse(sentence.)


    todo!()
}