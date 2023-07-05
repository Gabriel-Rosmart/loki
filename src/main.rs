use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use loki::{
    fetching::{
        indexer::{Indexer, TfIdfModel},
        Storage,
    },
    searching::searcher::Searcher,
};

use tiny_http::{Header, Method, Response, Server};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let assets_dir = &args[1];

    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str(&format!("/{assets_dir}"));

    let tf_idf_model = Arc::new(Mutex::new(TfIdfModel::new()));

    let indexing_model = Arc::clone(&tf_idf_model);
    std::thread::spawn(move || loop {
        let model = Arc::clone(&indexing_model);

        Indexer::index_directory(&path, model);
        std::thread::sleep(Duration::from_secs(3));
    });

    let server = Server::http("127.0.0.1:8000").unwrap();

    println!("Server started on port 8000");

    for mut request in server.incoming_requests() {
        match (request.method(), request.url()) {
            (Method::Post, "/") => {
                let mut buffer = Vec::new();
                request.as_reader().read_to_end(&mut buffer).unwrap();
                let body = std::str::from_utf8(&buffer).unwrap();

                let search_result = Searcher::search_term(&body, Arc::clone(&tf_idf_model));
                let search_result = search_result.iter().take(10).collect::<Vec<_>>();

                request
                    .respond(
                        Response::from_string(serde_json::to_string(&search_result).unwrap())
                            .with_header(
                                Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                                    .unwrap(),
                            ),
                    )
                    .unwrap()
            }
            _ => request
                .respond(Response::from_string("GET OUT OF HERE"))
                .unwrap(),
        };
    }
}
