use pest_derive::Parser;

pub(crate) mod evaluation;
pub(crate) mod truth_table;
pub(crate) mod truth_tree;

#[derive(Parser)]
#[grammar = "sl/sl-grammar.pest"]
pub(crate) struct SLGrammarParser;
