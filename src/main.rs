mod token;

use anyhow::bail;
use token::Token;

use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    let mut tokens = Vec::<Token>::new();

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut content = file_contents.chars();
            while let Some(ch) = content.next() {
                match ch {
                    ch if ch.is_whitespace() => continue,
                    _ => {
                        tokens.push(Token::try_from(ch)?);
                    }
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }

    tokens.push(Token::end_of_file());

    for token in tokens {
        println!("{}", token.to_string())
    }

    Ok(())
}
