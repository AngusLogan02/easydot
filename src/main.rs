use toml::Value;
use std::fs;
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

                if let (Some(Value::String(src)), Some(Value::String(dst))) =
                    (source, dest)
                {
                    let mapping = Mapping {
                        name: table_name,
                        source: src.clone(),
                        dest: dst.clone(),
                    };
                    mappings.push(mapping);
                }
                else {
                    eprintln!(
                        "Error: table \"{}\" is missing a source or dest.",
                        table_name
                    );
                    exit(1);
                }
            }
        }
    }

    return mappings;
}

fn main() {
    let filemap_filename = "filemap.toml";

    let toml_string = fs::read_to_string(filemap_filename)
        .expect("Unable to read filemap.toml");

    let value: Value = toml_string.parse::<Value>().expect("Error parsing.");

    let mappings: Vec<Mapping> = get_mappings(value);
    
    for mapping in mappings {
        println!("name: {}, source: {}, dest: {}",
            mapping.name, mapping.source, mapping.dest)
    }
}
