mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Delete the symlinks created at the destinations
    #[arg(short, long, default_value_t = false)]
    restore: bool,
}

fn main() {
    let args = Args::parse();

    let filemap_filename = "filemap.toml";

    let mappings: Vec<utils::Mapping> = utils::read_filemap(filemap_filename);

    for mapping in mappings {
        utils::create_mapping(mapping, args.restore);
    }

    println!("-----");
    println!("Done!");
}
