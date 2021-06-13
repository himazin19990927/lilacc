use ast::expr::Expr;
use lexer::Lexer;
use parser::lilac;
use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    loop {
        print!(">> ");
        let _ = stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.as_str().trim() {
            "quit" | ":q" => break,
            input => println!("{:#?}", parse(input)),
        }
    }

    Ok(())
}

fn parse(input: &str) -> Expr {
    let lexer = Lexer::new(input);
    lilac::ExprParser::new().parse(lexer).unwrap()
}
