use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix;
use std::path::Path;
use std::process::exit;
use toml::Value;

pub struct Mapping {
    name: String,
    source: String,
    dest: String,
}

pub fn read_filemap(mapping_filename: &str) -> Vec<Mapping> {
    // define toml_string, if the type returned by read_to_string matches Err, then quit
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
    // parse the toml string into a nested toml structure, with the generic "Value" at the top
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
                    // src, dest must be cloned as the Mapping type is looking for String
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

pub fn create_mapping(mapping: Mapping, delete: bool) {
    let pwd = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(_) => {
            eprintln!("Error getting current directory!");
            exit(1);
        }
    };

    // format! is essentially a string builder here.
    let absolute_source = format!("{}/dotfiles/{}", pwd.display(), mapping.source);
    // resolve a '~' in the destination to get the absolute destination
    let absolute_destination = handle_home(&mapping.dest);

    println!("----- {} -----", mapping.name);

    let absolute_source_metadata = match fs::metadata(&absolute_source) {
        Ok(md) => md,
        Err(e) => {
            eprintln!("Error getting metadata for \"{}\": {}", absolute_source, e);
            exit(1);
        }
    };

    // if the source file is a directory and the destination folder already exists
    if absolute_source_metadata.file_type().is_dir() && Path::new(&absolute_destination).exists() {
        // get a list of files (in the form of an iterator) in the source
        if let Ok(source_files) = fs::read_dir(&absolute_source) {
            // printout to the user that the destination already exists so we're going
            // to map the contents of the source as children of that destination
            println!(
                "{} already exists, linking the files and folders within.",
                absolute_destination
            );

            for f in source_files {
                if let Ok(f) = f {
                    let filename = match f.file_name().into_string() {
                        Ok(filename) => filename,
                        Err(_) => {
                            eprintln!("Error getting filename into string.");
                            exit(1);
                        }
                    };
                    let absolute_destination = format!("{}/{}", absolute_destination, filename);
                    let absolute_source = format!("{}/{}", absolute_source, filename);
                    // if we're not running the -r option, link the file
                    if !delete {
                        symlink(&absolute_source, &absolute_destination);
                    } else {
                        restore(&absolute_destination);
                    }
                };
                println!();
            }
        }
    } else {
        if !delete {
            symlink(&absolute_source, &absolute_destination);
        } else {
            restore(&absolute_destination);
        }
        println!();
    };
}

fn symlink(source: &String, dest: &String) -> bool {
    if Path::new(dest).exists() {
        println!("A file/folder already exists at {}", &dest);
        println!(
            "Do you want to create a backup of the already existing file/folder and then create the link?"
        );
        let backup;
        loop {
            let mut choice = String::new();
            print!("Create backup? (y/n): ");
            io::stdout().flush().unwrap();
            match io::stdin().read_line(&mut choice) {
                Ok(_) => {
                    let choice = choice.trim().to_lowercase();
                    if choice == "y" {
                        backup = true;
                        break;
                    } else if choice == "n" {
                        backup = false;
                        break;
                    } else {
                        println!("entered {}", choice);
                        println!("Please enter either 'y' or 'n'.");
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        if backup {
            let backup_success = create_backup(&dest);
            if !backup_success {
                eprintln!("Error creating backup! Skipping file/folder!");
                return true;
            }
        } else {
            println!("Ok. Skipping file/folder.");
            return true;
        }
    }

    println!("creating link: {} -> {}", source, dest);
    match unix::fs::symlink(source, dest) {
        Ok(_) => println!("success."),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    return true;
}

fn restore(target: &String) -> bool {
    let p = Path::new(&target);
    if p.exists() {
        match fs::remove_file(&p) {
            Ok(_) => println!("Successfully deleted {}", &p.display()),
            Err(e) => {
                eprintln!("Error deleting {}: {}", &p.display(), e);
                return false;
            }
        }
    } else {
        println!("{} does not exist. Skipping.", &p.display());
    }

    let mut backup_path = p.to_path_buf();
    let extension: String = match backup_path.extension() {
        Some(e) => format!("{}.edbackup", e.display()),
        None => String::from("edbackup"),
    };

    backup_path.set_extension(extension);
    if backup_path.exists() {
        println!("Found backup: {}", backup_path.display());
        match fs::rename(backup_path, p) {
            Ok(_) => println!("Successfully restored backup!"),
            Err(e) => eprintln!("Error restoring backup: {}", e),
        };
    }
    return true;
}

fn handle_home(path: &String) -> String {
    if !path.contains('~') {
        return path.to_string();
    }

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

fn create_backup(path: &String) -> bool {
    let mut backup_filename: String = path.clone();
    backup_filename.push_str(".edbackup");

    let backup_path = Path::new(&backup_filename);

    match fs::rename(path, backup_path) {
        Ok(_) => println!("Successfully created backup file."),
        Err(e) => {
            eprintln!("Error creating backup file: {}", e);
            return false;
        }
    };

    return true;
}
