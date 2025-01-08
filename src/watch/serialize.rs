use std::{fs::{self, read_to_string}, path::PathBuf};

use crate::parser::{self, parse::{ChildType, Recipe, RecipePath, ScriptType}};

use super::{ActionType, ProjectQueue};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use log::error;

pub fn build(project: &ProjectQueue, recipe: Recipe) -> String {
    let mut serialized = String::new();
    for (path, action) in project.clone().queue {
        match action {
            ActionType::Create => {
                serialized.push_str("{C,");
                serialized.push_str(&qualify_recipe_path(path, &recipe));
                serialized.push_str(",\"");

                serialized.push_str("\"}");
            },
            ActionType::Path => {
                serialized.push('{');
                serialized.push('P');
                serialized.push('}');
            },
            ActionType::Remove => {
                serialized.push('{');
                serialized.push('R');
                serialized.push('}');
            }
        }
    }

    return serialized;
}

fn qualify_recipe_path(path: PathBuf, recipe: &Recipe) -> String {
    let path = fs::canonicalize(&path).expect("Expected path to resolve");
    let meta = std::fs::metadata(&path).expect("Failed to fetch metadata for path!");

    if meta.is_dir() { 
        return "3".to_string();    
    };

    let mut path_fit = Vec::new();
    let mut fit_length = 0;

    for entry in &recipe.entries {
        let entry_path = fs::canonicalize(recipe.options.active_directory.join(&entry.path)).expect("Expected path to resolve");

        if !entry_path.exists() {
            error!("The directory specified for recipe '{}' does not exist", path.display());
        }

        let dir_length = entry_path.to_str().unwrap().len();
        if path.starts_with(entry_path) && fit_length < dir_length {
            fit_length = dir_length;
            path_fit = entry.clone().target;
        } 
    }


    let mut type_fit = ScriptType::ServerScript;
    let mut fit_length = 0;

    for association in &recipe.associations {
        let association_path = fs::canonicalize(recipe.options.active_directory.join(&association.path)).expect("Expected path to resolve");

        if !association_path.exists() {
            error!("The directory specified for association '{}' does not exist", path.display());
        }

        let dir_length = association_path.to_str().unwrap().len();
        if path.starts_with(association_path) && fit_length < dir_length {
            fit_length = dir_length;
            type_fit = association.script_type;
        } 
    }

    let mut file_information = match type_fit {
        ScriptType::LocalScript => "2",
        ScriptType::ModuleScript => "1",
        ScriptType::ServerScript => "0"
    }.to_owned();

    file_information.push(',');

    let file_content = read_file(path, &recipe);
    let file_target = process_path(path_fit);

    file_information.push_str(&URL_SAFE.encode(file_content));
    file_information.push(',');
    file_information.push_str(&URL_SAFE.encode(file_target));

    return file_information;
}

fn read_file(path: PathBuf, recipe: &Recipe) -> String {
    let contents = read_to_string(&path).expect("Expected file contents");
    if recipe.options.preprocess_text && path.ends_with(".txt") {
        return parser::preprocess::process_text(&contents, &recipe.options);
    }

    return contents;
}

fn process_path(path: Vec<RecipePath>) -> String {
    let mut target = "".to_owned();
    for child in path {
        target.push_str(match child.child_type {
            ChildType::CreateChild => "!",
            ChildType::FindChild => ".",
            ChildType::Service => "#",
            ChildType::WaitChild => ":"
        });

        target.push_str(&URL_SAFE.encode(child.path));
    }

    return target;
}