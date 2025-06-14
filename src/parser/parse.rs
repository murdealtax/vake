use std::{path::PathBuf, process::exit};
use crate::parser::lex::Token;
use log::{error, debug};

const OPTION_NAMES: [&str; 8] = [
    "active_directory",
    "case_type",
    "case_abbreviations",
    "case_exceptions",
    "entry_name",
    "preprocess_text",
    "preprocess_pretty",
    "preserve_folders"
];

#[derive(Debug, Clone)]
pub struct RecipeOptions {
    pub active_directory: PathBuf,
    pub case_type: CaseType,
    pub case_abbreviations: bool,
    pub case_exceptions: Vec<String>,
    pub entry_name: String,
    pub preprocess_text: bool,
    pub preprocess_pretty: bool,
    pub preserve_folders: bool,
    pub cc: Option<String>,
    pub cflags: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CaseType {
    Pascal,
    Camel,
    Snake,
    Kebab
}

#[derive(Debug, Clone, Copy)]
pub enum ScriptType {
    LocalScript,
    ServerScript,
    ModuleScript
}

#[derive(Debug, Clone)]
pub enum ChildType {
    WaitChild,
    FindChild,
    CreateChild,
    Service
}

#[derive(Debug, Clone)]
pub struct RecipeAssociation {
    pub path: PathBuf,
    pub script_type: ScriptType
}

#[derive(Debug, Clone)]
pub struct RecipePath {
    pub path: String,
    pub child_type: ChildType
}

#[derive(Debug, Clone)]
pub struct RecipeEntry {
    pub path: PathBuf,
    pub target: Vec<RecipePath>
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub options: RecipeOptions,
    pub associations: Vec<RecipeAssociation>,
    pub entries: Vec<RecipeEntry>
}

macro_rules !expect {
    ($iterator:ident, $token:pat) => {
        match $iterator.next() {
            Some($token) => {},
            Some(token) => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected token {:?} in wakefile", token);
                exit(1);
            }
            _ => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected end of file in wakefile");
                exit(1);
            }
        }
    };
}

macro_rules !expect_value {
    ($iterator:ident, $token_type:ident) => {
        match $iterator.next() {
            Some(Token::$token_type(name)) => Some(name),
            Some(token) => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected token {:?} in wakefile", token);
                exit(1);
            }
            _ => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected end of file in wakefile");
                exit(1);
            }
        }.unwrap()
    };
}

macro_rules !expect_boolean {
    ($iterator:ident) => {
        match $iterator.next() {
            Some(Token::True) => true,
            Some(Token::False) => false,
            Some(token) => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected token {:?} in wakefile", token);
                exit(1);
            }
            _ => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected end of file in wakefile");
                exit(1);
            }
        }
    };
}

pub fn init(tokens: Vec<Token>) -> Recipe {
    debug!("Parsing wakefile with {} tokens", tokens.len());

    let mut iterator = tokens.into_iter().peekable();
    let default_options = RecipeOptions {
        active_directory: PathBuf::from("src"),
        case_type: CaseType::Pascal,
        case_abbreviations: false,
        case_exceptions: Vec::new(),
        entry_name: String::from("main.lua"),
        preprocess_text: true,
        preprocess_pretty: false,
        preserve_folders: false,
        cc: Option::None,
        cflags: Option::None,
    };

    let mut recipe = Recipe {
        options: default_options,
        associations: Vec::new(),
        entries: Vec::new()
    };

    while let Some(token) = iterator.next() {
        match token {
            Token::Colon => {
                let option = expect_value!(iterator, Identifier);

                if !OPTION_NAMES.contains(&option.as_str()) {
                    error!("Aborted due to malformed wakefile body");
                    error!("Unexpected option {:?} in wakefile", option);
                    exit(1);
                }

                expect!(iterator, Token::Equal);

                handle_option(&mut recipe, option, &mut iterator);
            },
            Token::Identifier(name) => {
                let path = build_path(name, &mut iterator);

                match iterator.peek() {
                    Some(Token::Arrow) => handle_entry(&mut recipe, path, &mut iterator),
                    Some(Token::DoubleColon) => handle_association(&mut recipe, path, &mut iterator),
                    _ => {
                        error!("Aborted due to malformed wakefile body");
                        error!("Unexpected token {:?} in wakefile", iterator.peek());
                        exit(1);
                    }
                }
            },
            Token::Slash => {
                let path = build_path(String::new(), &mut iterator);

                match iterator.peek() {
                    Some(Token::Arrow) => handle_entry(&mut recipe, path, &mut iterator),
                    Some(Token::DoubleColon) => handle_association(&mut recipe, path, &mut iterator),
                    _ => {
                        error!("Aborted due to malformed wakefile body");
                        error!("Unexpected token {:?} in wakefile", iterator.peek());
                        exit(1);
                    }
                }
            }
            _ => {
                error!("Aborted due to malformed wakefile body");
                error!("Unexpected token {:?} in wakefile", token);
                exit(1);
            }
        }
    }

    return recipe;
}

fn handle_option(recipe: &mut Recipe, option: String, iterator: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) {
    debug!("Parsing option {}", option);

    match option.as_str() {
        "active_directory" => {
            let value = expect_value!(iterator, String);
            recipe.options.active_directory = PathBuf::from(value);
        },
        "case_type" => {
            let value = expect_value!(iterator, Identifier);

            match value.as_str() {
                "pascal" => recipe.options.case_type = CaseType::Pascal,
                "camel" => recipe.options.case_type = CaseType::Camel,
                "snake" => recipe.options.case_type = CaseType::Snake,
                "kebab" => recipe.options.case_type = CaseType::Kebab,
                _ => {
                    error!("Aborted due to malformed wakefile options");
                    error!("Unexpected case type {:?} in wakefile", value);
                    exit(1);
                }
            }
        },
        "case_abbreviations" => {
            let value = expect_boolean!(iterator);
            recipe.options.case_abbreviations = value;
        },
        "case_exceptions" => {
            let mut exceptions = Vec::new();

            expect!(iterator, Token::LeftBracket);

            loop {
                match iterator.peek() {
                    Some(Token::String(name)) => {
                        exceptions.push(name.clone());
                        iterator.next();
                    },
                    _ => {
                        expect!(iterator, Token::RightBracket);
                        break;
                    }
                }

                match iterator.peek() {
                    Some(Token::Comma) => {
                        iterator.next();
                    },
                    Some(Token::RightBracket) => {
                        iterator.next();
                        break;
                    },
                    _ => {
                        error!("Aborted due to malformed wakefile options");
                        error!("Unexpected token {:?} in wakefile", iterator.peek());
                        exit(1);
                    }
                }
            }

            recipe.options.case_exceptions = exceptions;
        },
        "entry_name" => {
            let value = expect_value!(iterator, String);
            recipe.options.entry_name = value;
        },
        "preprocess_text" => {
            let value = expect_boolean!(iterator);
            recipe.options.preprocess_text = value;
        },
        "preprocess_pretty" => {
            let value = expect_boolean!(iterator);
            recipe.options.preprocess_pretty = value;
        },
        "preserve_folders" => {
            let value = expect_boolean!(iterator);
            recipe.options.preserve_folders = value;
        },
        "cc" => {
            let value = expect_value!(iterator, String);
            recipe.options.cc = Some(value);
        },
        "cflags" => {
            let value = expect_value!(iterator, String);
            recipe.options.cflags = Some(value);
        },
        _ => {
            error!("Aborted due to malformed wakefile options");
            error!("Unexpected option {:?} in wakefile", option);
            exit(1);
        }
    }
}

fn handle_association(recipe: &mut Recipe, path: PathBuf, iterator: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) {
    iterator.next();

    let association = RecipeAssociation {
        path: path,
        script_type: match expect_value!(iterator, Keyword).as_str() {
            "LocalScript" => ScriptType::LocalScript,
            "ServerScript" => ScriptType::ServerScript,
            "ModuleScript" => ScriptType::ModuleScript,
            _ => {
                error!("Aborted due to malformed wakefile association");
                error!("Unexpected script type {:?} in wakefile", iterator.peek());
                exit(1);
            }
        }
    };

    recipe.associations.push(association);
}

fn handle_entry(recipe: &mut Recipe, path: PathBuf, iterator: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) {
    iterator.next();

    let entry = RecipeEntry {
        path: path,
        target: build_tree(iterator)
    };

    recipe.entries.push(entry);
}

fn build_path(name: String, iterator: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(name);

    loop {
        match iterator.peek() {
            Some(Token::Arrow) | Some(Token::DoubleColon) => break,
            Some(Token::Dot) => {
                iterator.next();
            },
            Some(Token::Identifier(name)) => {
                path.push(name);
                iterator.next();
            },
            _ => {
                error!("Aborted due to malformed wakefile path");
                error!("Unexpected token {:?} in wakefile", iterator.peek());
                exit(1);
            }
        }
    }

    return path;
}

fn build_tree(iterator: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Vec<RecipePath> {
    let mut tree: Vec<RecipePath> = vec![
        RecipePath {
            path: expect_value!(iterator, Keyword),
            child_type: ChildType::Service
        }
    ];

    loop {
        match iterator.peek() {
            Some(Token::Colon) => {
                iterator.next();
                tree.push(RecipePath {
                    path: expect_value!(iterator, Identifier),
                    child_type: ChildType::WaitChild
                })
            },
            Some(Token::Bang) => {
                iterator.next();
                tree.push(RecipePath {
                    path: expect_value!(iterator, Identifier),
                    child_type: ChildType::CreateChild
                })
            },
            Some(Token::Dot) => {
                iterator.next();
                tree.push(RecipePath {
                    path: expect_value!(iterator, Identifier),
                    child_type: ChildType::FindChild
                })
            },
            _ => break
        }
    }

    return tree;
}