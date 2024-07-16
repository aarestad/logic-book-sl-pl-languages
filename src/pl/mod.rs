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

mod test {
    use pest::Parser;

    use crate::pl::{PLGrammarParser, Rule};

    #[test]
    fn pl_non_letter() {
        let result = PLGrammarParser::parse(Rule::top, "💥");

        assert!(result.is_err());
    }

    #[test]
    fn pl_sentence_letter_with_optional_subscript() {
        for l in 'A'..='Z' {
            let s: String = vec![l].into_iter().collect();
            assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());

            for sub in '\u{2081}'..='\u{2089}' {
                let s: String = vec![l, sub].into_iter().collect();
                assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());
            }
        }
    }

    #[test]
    fn pl_predicate_without_primes_without_subscripts() {
        for l in 'A'..='Z' {
            let sentence_letter = vec![l];

            for n in 1..=10 {
                for t in 'a'..='z' {
                    // TODO test with subscripts on terms
                    let mut str_with_terms = sentence_letter.clone();

                    for _ in 1..=n {
                        str_with_terms.push(t);
                    }

                    let s: String = str_with_terms.into_iter().collect();
                    assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());
                }
            }
        }
    }

    #[test]
    fn sentence_letter_too_many_subscripts() {
        assert!(PLGrammarParser::parse(Rule::top, "A\u{2081}\u{2082}\u{2083}").is_err());
    }

    #[test]
    fn pl_predicate_without_primes_with_subscripts() {
        for l in 'A'..='Z' {
            let sentence_letter = vec![l];

            for sub in '\u{2081}'..='\u{2089}' {
                let mut sentence_letter_with_sub = sentence_letter.clone();
                sentence_letter_with_sub.push(sub);

                for n in 1..=10 {
                    for t in 'a'..='z' {
                        // TODO test with subscripts on terms
                        let mut str_with_terms = sentence_letter_with_sub.clone();

                        for _ in 1..=n {
                            str_with_terms.push(t);
                        }

                        let s: String = str_with_terms.into_iter().collect();
                        assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());
                    }
                }
            }
        }
    }

    #[test]
    fn pl_predicate_with_primes_bad_number_of_terms() {
        let too_few_result = PLGrammarParser::parse(Rule::top, "A′");
        assert!(too_few_result.is_err());

        // TODO these are not errors yet
        // let too_few_nonzero_result = PLGrammarParser::parse(Rule::top, "A′′a");
        // assert!(too_few_nonzero_result.is_err());

        // let too_many_result = PLGrammarParser::parse(Rule::top, "A′aa");
        // assert!(too_many_result.is_err());
    }

    #[test]
    fn pl_predicate_with_primes_without_subscripts() {
        for l in 'A'..='Z' {
            let sentence_letter = vec![l];

            for n in 1..=10 {
                let mut str_with_primes = sentence_letter.clone();

                for _ in 1..=n {
                    str_with_primes.push('′');
                }

                for t in 'a'..='z' {
                    // TODO test with subscripts on terms
                    let mut str_with_terms = str_with_primes.clone();

                    for _ in 1..=n {
                        str_with_terms.push(t);
                    }

                    let s: String = str_with_terms.into_iter().collect();
                    assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());
                }
            }
        }
    }

    #[test]
    fn pl_predicate_with_primes_with_subscripts() {
        for l in 'A'..='Z' {
            let sentence_letter = vec![l];

            for sub in '\u{2081}'..='\u{2089}' {
                let mut sentence_letter_with_sub = sentence_letter.clone();
                sentence_letter_with_sub.push(sub);

                for n in 1..=10 {
                    let mut str_with_primes = sentence_letter_with_sub.clone();

                    for _ in 1..=n {
                        str_with_primes.push('′');
                    }

                    for t in 'a'..='z' {
                        // TODO test with subscripts on terms
                        let mut str_with_terms = str_with_primes.clone();

                        for _ in 1..=n {
                            str_with_terms.push(t);
                        }

                        let s: String = str_with_terms.into_iter().collect();
                        assert!(PLGrammarParser::parse(Rule::top, s.as_str()).is_ok());
                    }
                }
            }
        }
    }
}
