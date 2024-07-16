use pest::Parser;

mod sl;
mod pl;

fn main() {
    let top =
        sl::SLGrammarParser::parse(sl::Rule::top, "((∼B ⊃ C) ∧ (A ≡ B))");

    if let Ok(mut pairs) = top {
        println!("truth table:");

        for line in sl::truth_table::truth_table(pairs.next().unwrap().into_inner()) {
            println!("{}", line);
        }
    } else {
        println!("error parsing SL string: {}", top.unwrap_err());
    }

    let top2 = pl::PLGrammarParser::parse(pl::Rule::sentence, "B");

    if let Ok(pairs) = top2 {
        println!("{}", pairs.as_str());
    } else {
        let err = top2.unwrap_err();
        println!("error parsing PL string: {}", err);
        println!("{:#?}", err);
    }
}
