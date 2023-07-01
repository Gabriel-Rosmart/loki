use loki::{
    fetching::indexer::{Indexer, TermMapThroughDocuments},
    searching::searcher::Searcher,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let query: &str = &args[2];
    let assets_dir = &args[1];

    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str(&format!("/{assets_dir}"));

    let mut term_map = TermMapThroughDocuments::new();
    Indexer::index_directory(&path, &mut term_map);

    Searcher::search_term(query, &term_map);
}
