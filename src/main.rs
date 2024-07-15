use pest::Parser;

mod sl;

fn main() {
    let pairs_result = sl::SLGrammarParser::parse(sl::Rule::sentence, "((\u{223C} B ⊃ C) \u{2227} (A ≡ B))");

    if let Ok(pairs) = pairs_result {
        println!("truth table:");

        for line in sl::truth_table::truth_table(pairs) {
            println!("{}", line);
        }
    } else {
        println!("error parsing string: {}", pairs_result.unwrap_err());
    }
}
