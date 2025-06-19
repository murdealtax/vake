use log::error;

use super::parse::Recipe;

pub fn init(recipe: &Recipe) {
    let options = &recipe.options;
    let entries = &recipe.entries;
    let associations = &recipe.associations;

    let active_directory = options.active_directory.as_path();
    let entry_file = active_directory.join(options.entry_name.as_str());

    let mut recipe_errors = 0;

    if !active_directory.exists() {
        error!(
            "The active directory '{:?}' does not exist",
            options
                .active_directory
                .as_os_str()
                .to_str()
                .expect("Expected a specified directory")
        );
        recipe_errors += 1;
    }

    // if !entry_file.exists() {
    //     error!("The specified entry file '{}' does not exist", entry_file.display());
    //     recipe_errors += 1;
    // }

    for entry in entries {
        let path = active_directory.join(entry.path.clone());

        if !path.exists() {
            error!(
                "The directory specified for recipe '{}' does not exist",
                path.display()
            );
            recipe_errors += 1;
        }
    }

    for association in associations {
        let path = active_directory.join(association.path.clone());

        if !path.exists() {
            error!(
                "The directory specified for association '{}' does not exist",
                path.display()
            );
            recipe_errors += 1;
        }
    }

    if recipe_errors > 0 {
        error!(
            "Recipe failed with {} error(s) when attempting to build from directory '{}'",
            recipe_errors,
            options
                .active_directory
                .as_os_str()
                .to_str()
                .expect("Expected a specified directory")
        );
        std::process::exit(1);
    }
}
