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
        println!("error parsing string: {}", top.unwrap_err());
    }

    let top2 = pl::PLGrammarParser::parse(pl::Rule::top, "((∼B ⊃ C) ∧ (A ≡ B))");
    println!("{:#?}", top2);
}
