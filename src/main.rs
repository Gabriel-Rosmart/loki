use loki::{
    fetching::{
        indexer::{Indexer, TfIdfModel},
        Storage,
    },
    searching::searcher::Searcher,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let query: &str = &args[2];
    let assets_dir = &args[1];

    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str(&format!("/{assets_dir}"));

    let mut tf_idf_model = TfIdfModel::new();

    Indexer::index_directory(&path, &mut tf_idf_model);

    Storage::save_model_to_disk(&tf_idf_model);

    Searcher::search_term(query, &tf_idf_model);
}
