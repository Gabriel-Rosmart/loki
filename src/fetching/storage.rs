use super::indexer::TfIdfModel;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

pub struct Storage;

impl Storage {
    const INDEX_FILE: &'static str = "index.cbor";

    pub fn save_model_to_disk(tf_idf_model: &TfIdfModel) {
        let index_file = File::create(Self::INDEX_FILE).unwrap();
        let _r = ciborium::into_writer(&tf_idf_model, BufWriter::new(index_file));
    }

    pub fn load_model_from_disk() -> TfIdfModel {
        let index_file = File::open(Self::INDEX_FILE).unwrap();
        let model: TfIdfModel = ciborium::from_reader(BufReader::new(index_file)).unwrap();
        model
    }
}
