mod utils;

struct Mapping {
    name: String,
    source: String,
    dest: String,
}

fn main() {
    let filemap_filename = "filemap.toml";

    let mappings: Vec<Mapping> = utils::read_filemap(filemap_filename);

    for mapping in mappings {
        utils::create_mapping(mapping);
    }
}
