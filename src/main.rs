mod lang;

use lang::lexer::Lexer;

fn main() {
    let code: &str = "СУММА РАВНО 2 ПЛЮС 2";

    let mut lexer = Lexer::new(code.to_string());
    lexer.analyse();
}
