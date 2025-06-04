use std::env;
use std::fs;
use std::os::unix;
use std::process::exit;
use toml::Value;

use crate::Mapping;

pub fn read_filemap(mapping_filename: &str) -> Vec<Mapping> {
    let toml_string = match fs::read_to_string(mapping_filename) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!(
                "Error reading \"{}\". Check it exists and is in the same directory as easydot. {}",
                mapping_filename, e
            );
            exit(1);
        }
    };
    let toml_data: Value = match toml_string.parse::<Value>() {
        Ok(td) => td,
        Err(e) => {
            eprintln!(
                "Error parsing TOML data in \"{}\". Ensure that it is valid and follows the example from the GitHub repo. {}",
                mapping_filename, e,
            );
            exit(1);
        }
    };

    let mut mappings: Vec<Mapping> = Vec::new();

    // if toml_data is a Table
    if let Value::Table(top_level) = toml_data {
        for (table_name, table_value) in top_level {
            // if the top table's data (value) is a table
            if let Value::Table(inner_table) = table_value {
                // extract the two values in the innermost table
                let source = inner_table.get("source");
                let dest = inner_table.get("dest");

                // if source and dest and both strings, move forward calling
                // them src and dst
                if let (Some(Value::String(src)), Some(Value::String(dst))) = (source, dest) {
                    let mapping = Mapping {
                        name: table_name,
                        source: src.clone(),
                        dest: dst.clone(),
                    };
                    mappings.push(mapping);
                } else {
                    eprintln!(
                        "Error: table \"{}\" is missing a source or dest or they are of an incorrect type (should be strings).",
                        table_name
                    );
                    exit(1);
                }
            }
        }
    }

    mappings
}

pub fn create_mapping(mapping: Mapping) {
    let pwd = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(_) => {
            eprintln!("Error getting current directory!");
            exit(1);
        }
    };

    let absolute_source = format!("{}/dotfiles/{}", pwd.display(), mapping.source);
    let absolute_destination = handle_home(&mapping.dest);

    println!("----- {} -----", mapping.name);
    println!("{} -> {}", absolute_source, absolute_destination);

    match unix::fs::symlink(absolute_source, absolute_destination) {
        Ok(_) => println!("success."),
        Err(e) => {
            eprintln!(
                "error mapping \"{}\"! please check this table and try again.",
                mapping.name
            );
            eprintln!("{}", e);
            exit(1);
        }
    };
}

fn handle_home(path: &String) -> String {
    let home_dir = match env::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("Error getting home directory.");
            exit(1);
        }
    };

    let home_dir = match home_dir.to_str() {
        Some(h) => h,
        None => {
            eprintln!("Error converting home directory from PathBuf to &str.");
            exit(1);
        }
    };

    let path = path.replace("~", home_dir);
    return path;
}
