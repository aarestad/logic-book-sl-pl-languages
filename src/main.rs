use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sl-grammar.pest"]
struct SLGrammarParser;

fn process_pair(pair: Pair<Rule>, indent: usize) {
    let indentation = " ".repeat(indent);

    match pair.as_rule() {
        Rule::sentence => {}
        Rule::negation => {
            println!("{}negation", indentation)
        }
        Rule::conjunction => {
            println!("{}conjunction", indentation)
        }
        Rule::disjunction => {
            println!("{}disjunction", indentation)
        }
        Rule::material_conditional => {
            println!("{}material conditional", indentation)
        }
        Rule::material_biconditional => {
            println!("{}material biconditional", indentation)
        }
        Rule::sentence_letter => {
            println!("{}{}", indentation, pair.as_str());
        }
        _ => {

        }
    }


    for p in pair.into_inner() {
        process_pair(p, indent + 2);
    }
}

fn main() {
    let pairs_result = SLGrammarParser::parse(Rule::sentence, "~(A & (B v (C = D)))");

    if let Ok(pairs) = pairs_result {
        for p in pairs {
            process_pair(p, 0);
        }
    } else {
        println!("{}", pairs_result.err().unwrap());
    }
}
