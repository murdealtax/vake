use super::parse::RecipeOptions;

pub fn process_text(text: &str, options: &RecipeOptions) -> String {
    
    let longest_escape = string_escape(text);
    let mut block = String::from("return ");

    if options.preprocess_pretty {
        block.push_str("(");
    }

    block.push('[');
    for _ in 0..longest_escape {
        block.push('=');
    }
    block.push('[');

    if options.preprocess_pretty {
        block.push('\n');
    }

    block.push_str(text);

    if options.preprocess_pretty {
        block.push('\n');
    }

    block.push(']');
    for _ in 0..longest_escape {
        block.push('=');
    }
    block.push(']');

    if options.preprocess_pretty {
        block.push_str("):sub(1, -2)");
    }

    return block;

}

fn string_escape(text: &str) -> i32 {
    let mut longest = 0;
    let mut characters = text.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            ']' => {
                let mut size = 1;
                while let Some(next) = characters.peek() {
                    if next == &'=' {
                        size += 1;
                        characters.next();
                    } else {
                        break;
                    }
                }

                if size >= longest {
                    longest = size;
                }
            },
            _ => continue,
        }
    }

    return longest;
}