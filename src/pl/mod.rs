use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pl/pl-grammar.pest"]
pub(crate) struct PLGrammarParser;

#[derive(strum_macros::Display)]
pub(crate) enum Quantifier {
    #[strum(to_string = "∀")]
    Universal,
    #[strum(to_string = "∃")]
    Existential,
}

pub(crate) struct PLFormula {}