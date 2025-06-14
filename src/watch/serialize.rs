use std::{
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf,
    process::exit,
};

use crate::parser::{
    self,
    parse::{CaseType, ChildType, Recipe, RecipePath, ScriptType},
};

use super::{ActionType, ProjectQueue};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use log::error;

struct Response {
    pub script_type: Option<ScriptType>,
    pub content: String,
    pub name: PathBuf,
    pub path: Vec<RecipePath>,
}

impl Response {
    pub fn new() -> Self {
        return Self {
            script_type: None,
            content: String::new(),
            name: PathBuf::new(),
            path: Vec::new(),
        };
    }

    pub fn set_type(&mut self, script_type: ScriptType) {
        self.script_type = Some(script_type);
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_name(&mut self, name: PathBuf) {
        self.name = name;
    }

    pub fn set_path(&mut self, path: Vec<RecipePath>) {
        self.path = path;
    }

    pub fn serialize(&self, recipe: &Recipe) -> String {
        let mut serialized = String::new();

        serialized.push('{');

        // Add the path and name
        let mut path = String::new();
        path.push_str(&process_path(&self.path));

        let components: Vec<_> = self.name.components().collect();

        for component in &components[..components.len() - 1] {
            path.push('!');
            path.push_str(&fix_name(
                component.as_os_str().to_str().expect("Invalid path"),
                recipe.options.case_type.clone(),
                recipe.options.case_exceptions.clone(),
                recipe.options.case_abbreviations,
            ));
        }

        serialized.push_str(&URL_SAFE.encode(&path));

        serialized.push(':');
        serialized.push_str(
            &URL_SAFE.encode(&fix_name(
                self.name
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .expect("Expected file name!"),
                recipe.options.case_type.clone(),
                recipe.options.case_exceptions.clone(),
                recipe.options.case_abbreviations,
            )),
        );

        serialized.push(',');

        // Add the script type
        serialized.push_str(match self.script_type {
            Some(ScriptType::LocalScript) => "LocalScript",
            Some(ScriptType::ModuleScript) => "ModuleScript",
            Some(ScriptType::ServerScript) => "Script",
            None => "Folder",
        });

        serialized.push(',');
        serialized.push_str(&URL_SAFE.encode(&self.content));

        serialized.push('}');

        return serialized;
    }
}

pub fn build(project: &mut ProjectQueue, recipe: Recipe) -> String {
    let mut serialized = String::new();
    for (path, action) in project.clone().queue {
        match action {
            ActionType::Create => {
                serialized.push_str(
                    qualify_recipe_path(path, &recipe)
                        .serialize(&recipe)
                        .as_str(),
                );
            }
            _ => {
                return String::from("^");
            }
        }
    }

    return serialized;
}

fn qualify_recipe_path(path: PathBuf, recipe: &Recipe) -> Response {
    let mut response = Response::new();
    let path = fs::canonicalize(&path).expect("Expected path to resolve");
    let meta = std::fs::metadata(&path).expect("Failed to fetch metadata for path!");

    if meta.is_dir() {
        return response;
    };

    let mut path_fit = Vec::new();
    let mut fit_length = 0;
    let mut short_path = PathBuf::new();

    for entry in &recipe.entries {
        let entry_path = fs::canonicalize(recipe.options.active_directory.join(&entry.path))
            .expect("Expected path to resolve");

        if !entry_path.exists() {
            error!(
                "The directory specified for recipe '{}' does not exist",
                path.display()
            );
        }

        let dir_length = entry_path.to_str().unwrap().len();
        if path.starts_with(entry_path.clone()) && fit_length < dir_length {
            fit_length = dir_length;
            path_fit = entry.clone().target;

            short_path = path
                .strip_prefix(entry_path)
                .expect("Failed to trim path!")
                .to_path_buf();
        }
    }

    let mut type_fit = ScriptType::ServerScript;
    let mut fit_length = 0;

    for association in &recipe.associations {
        let association_path =
            fs::canonicalize(recipe.options.active_directory.join(&association.path))
                .expect("Expected path to resolve");

        if !association_path.exists() {
            error!(
                "The directory specified for association '{}' does not exist",
                path.display()
            );
        }

        let dir_length = association_path.to_str().unwrap().len();
        if path.starts_with(&association_path) && fit_length < dir_length {
            fit_length = dir_length;
            type_fit = association.script_type;
        }
    }

    if short_path.as_os_str().is_empty() {
        if path
            .strip_prefix(
                fs::canonicalize(&recipe.options.active_directory)
                    .unwrap()
                    .as_path(),
            )
            .expect("Expected file to be in active directory!")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            == recipe.options.entry_name
        {
            response.set_type(ScriptType::ServerScript);
            response.set_name(
                path.strip_prefix(
                    fs::canonicalize(&recipe.options.active_directory)
                        .unwrap()
                        .as_path(),
                )
                .expect("Expected file to be in active directory!")
                .to_path_buf(),
            );

            let file_content = read_file(path.clone(), &recipe);
            response.set_content(file_content);
            response.set_path(vec![RecipePath {
                path: "ServerScriptService".to_owned(),
                child_type: ChildType::Service,
            }]);

            return response;
        }

        error!(
            "Unhandled file '{}', please add an association to your configuration.",
            path.display()
        );

        exit(1);
    }

    response.set_type(type_fit);
    response.set_name(short_path);

    let file_content = read_file(path.clone(), &recipe);
    response.set_content(file_content);
    response.set_path(path_fit);

    return response;
}

fn read_file(path: PathBuf, recipe: &Recipe) -> String {
    let contents = read_to_string(&path).expect("Expected file contents");
    if let Some(extension) = path.extension() {
        if recipe.options.preprocess_text && extension == OsStr::new("txt") {
            return parser::preprocess::process_text(&contents, &recipe.options);
        }
    }

    return contents;
}

fn process_path(path: &Vec<RecipePath>) -> String {
    let mut target = "".to_owned();
    for child in path {
        target.push_str(match child.child_type {
            ChildType::CreateChild => "!",
            ChildType::FindChild => ".",
            ChildType::Service => "#",
            ChildType::WaitChild => ":",
        });

        target.push_str(&child.path);
    }

    return target;
}

fn fix_name(
    name: &str,
    case_type: CaseType,
    case_exceptions: Vec<String>,
    case_abbr: bool,
) -> String {
    if case_exceptions.contains(&name.to_owned()) {
        return name.to_owned();
    };

    // Assume the original name uses Underscore_case (with any capitalization) to delimit words
    let parts: Vec<&str> = name.split('_').collect();

    return match case_type {
        CaseType::Camel => {
            let mut iter = parts.into_iter();
            let first = iter.next().unwrap_or("").to_lowercase();
            let rest: String = iter.map(|s| capitalize(s, case_abbr)).collect();
            format!("{}{}", first, rest)
        }
        CaseType::Pascal => parts.iter().map(|s| capitalize(s, case_abbr)).collect(),
        CaseType::Snake => parts
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join("_"),
        CaseType::Kebab => parts
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join("-"),
    };
}

fn capitalize(s: &str, case_abbr: bool) -> String {
    if case_abbr && s.chars().all(|c| c.is_ascii_uppercase()) {
        return s.to_uppercase(); // Keep abbreviation all caps
    }

    let mut chars = s.chars();
    match chars.next() {
        Some(first) => {
            first.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase()
        }
        None => String::new(),
    }
}
