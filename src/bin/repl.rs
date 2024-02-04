use std::io::{self, stdin, stdout, Write};

use monkey::lexer::lexer::{Lexer, Token};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    print!(">> ");
    stdout.flush()?;
    stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut lexer = Lexer::new(line);

            while let Ok(tok) = lexer.next_token() {
                println!("{}", tok);
                if tok == Token::Eof {
                    break;
                }
            }
            print!(">> ");
            stdout.flush().unwrap();
        }
    });
    Ok(())
}
