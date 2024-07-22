use std::process::exit;

use log::error;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Number(i64),
    String(String),
    True, False,
    Equal, Arrow,
    DoubleColon, Colon,
    Dot, Comma,
    LeftBracket, RightBracket,
    LeftBrace, RightBrace,
    Bang
}

const KEYWORDS: [&str; 21] = [
    "workspace", "Workspace", "Players", "Lighting",
    "MaterialService", "NetworkClient", "ReplicatedFirst",
    "ReplicatedStorage", "ServerScriptService", "ServerStorage",
    "StarterGui", "StarterPack", "StarterPlayer", "StarterPlayerScripts",
    "StarterCharacterScripts", "Teams", "SoundService", "TextChatService",
    "LocalScript", "Script", "ModuleScript"
];

pub fn init(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut characters = input.chars().peekable();

    let mut line = 1;

    while let Some(character) = characters.next() {
        match character {
            '\n' => line += 1,
            character if character.is_whitespace() => continue,
            '{' => tokens.push(Token::LeftBrace), '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket), ']' => tokens.push(Token::RightBracket),
            '=' => tokens.push(Token::Equal), '#' => {
                while let Some(&character) = characters.peek() {
                    if character == '\n' {
                        break;
                    }

                    characters.next();
                }
            },
            '.' => tokens.push(Token::Dot), ':' => {
                if let Some(&':') = characters.peek() {
                    characters.next();
                    tokens.push(Token::DoubleColon);
                } else {
                    tokens.push(Token::Colon);
                }
            },
            ',' => tokens.push(Token::Comma), '!' => tokens.push(Token::Bang),
            'a'..='z' | 'A'..='Z' | '_' | '-' => {
                if character == '-' && characters.peek() == Some(&'>') {
                    characters.next();
                    tokens.push(Token::Arrow);
                    continue;
                }

                let mut identifier = character.to_string();

                while let Some(&character) = characters.peek() {
                    match character {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
                            identifier.push(character);
                            characters.next();
                        },
                        _ => break
                    }
                }

                tokens.push(match identifier.as_str() {
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => if KEYWORDS.contains(&identifier.as_str()) {
                        Token::Keyword(identifier)
                    } else {
                        Token::Identifier(identifier)
                    }
                });
            },
            '0'..='9' => {
                let mut number = character.to_string();

                while let Some(&character) = characters.peek() {
                    match character {
                        '0'..='9' => {
                            number.push(character);
                            characters.next();
                        },
                        _ => break
                    }
                }

                tokens.push(Token::Number(number.parse().unwrap()));
            },
            '"' => {
                let mut string = String::new();
                let mut escaped = false;

                while let Some(character) = characters.next() {
                    match character {
                        '"' if !escaped => break,
                        '\\' if !escaped => escaped = true,
                        _ => {
                            escaped = false;
                            string.push(character)
                        }
                    }
                }

                tokens.push(Token::String(string));
            },
            _ => {
                error!("Unexpected \"{}\" when parsing wakefile on line \x1b[93m{}\x1b[0m:", character, line);
                error!("\x1b[93m{} \x1b[90m| \x1b[0m{}", line, input.lines().nth(line - 1).unwrap());
                exit(1);
            }
        }
    }

    return tokens;
}