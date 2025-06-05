mod utils;

fn main() {
    let filemap_filename = "filemap.toml";

    let mappings: Vec<utils::Mapping> = utils::read_filemap(filemap_filename);

    for mapping in mappings {
        utils::create_mapping(mapping);
    }

    println!("-----");
    println!("Done!");
}
