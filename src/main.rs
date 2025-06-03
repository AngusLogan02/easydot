use toml::Value;

use std::env;
use std::fs;
use std::os::unix;
use std::process::exit;

struct Mapping {
    name: String,
    source: String,
    dest: String,
}

fn get_mappings(parsed_toml: Value) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = Vec::new();
    if let Value::Table(top_level) = parsed_toml {
        for (table_name, table_value) in top_level {
            if let Value::Table(inner_table) = table_value {
                // Extract the two values
                let source = inner_table.get("source");
                let dest = inner_table.get("dest");

                if let (Some(Value::String(src)), Some(Value::String(dst))) = (source, dest) {
                    let mapping = Mapping {
                        name: table_name,
                        source: src.clone(),
                        dest: dst.clone(),
                    };
                    mappings.push(mapping);
                } else {
                    eprintln!(
                        "Error: table \"{}\" is missing a source or dest or \
                        they are of an incorrect type (should be strings).",
                        table_name
                    );
                    exit(1);
                }
            }
        }
    }

    mappings
}

fn main() {
    let pwd = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(_) => {
            eprintln!("Error getting current directory!");
            exit(1);
        }
    };

    let filemap_filename = "filemap.toml";

    let toml_string = fs::read_to_string(filemap_filename).expect("Unable to read filemap.toml");

    let value: Value = toml_string.parse::<Value>().expect("Error parsing.");

    let mappings: Vec<Mapping> = get_mappings(value);

    for mapping in mappings {
        let absolute_source = format!("{}/dotfiles/{}", pwd.display(), mapping.source);
        println!("----- {} -----", mapping.name);
        println!("{} -> {}", absolute_source, mapping.dest);
        match unix::fs::symlink(absolute_source, mapping.dest) {
            Ok(_) => println!("Success."),
            Err(e) => {
                eprintln!(
                    "Error mapping \"{}\"! Please check this table and try again.",
                    mapping.name
                );
                eprintln!("{}", e);
                exit(1);
            }
        };
    }
}
