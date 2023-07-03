use super::{Fetcher, Reader};
use crate::parsing::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Maps each term with how many times appears in a single document
type FrequencyMap = HashMap<String, usize>;

// Maps each document name with a FrequencyMap and how many distinct term has the document
type TermFrequencyPerDocumentMap = HashMap<String, DocumentMap>;

// Maps how many times appears each term across all files
type TermFrequencyAcrossDocumentsMap = HashMap<String, usize>;

#[derive(Default, Serialize, Deserialize)]
pub struct DocumentMap {
    pub frequency_map: FrequencyMap,
    pub total_terms: usize,
}

#[derive(Default, Serialize, Deserialize)]
pub struct TfIdfModel {
    pub term_frequency_per_document: TermFrequencyPerDocumentMap,
    pub term_frequency_across_documents: TermFrequencyAcrossDocumentsMap,
}

impl TfIdfModel {
    pub fn new() -> Self {
        Self {
            term_frequency_per_document: TermFrequencyPerDocumentMap::new(),
            term_frequency_across_documents: TermFrequencyAcrossDocumentsMap::new(),
        }
    }
}

pub struct Indexer;

impl Indexer {
    pub fn term_frequency(
        term: &str,
        document_term_frequencies: &FrequencyMap,
        document_entries: usize,
    ) -> f32 {
        *document_term_frequencies.get(term).unwrap_or(&0) as f32 / document_entries as f32
    }

    pub fn inverse_document_frequency(
        term: &str,
        term_frequency_across_documents_cache: &TermFrequencyAcrossDocumentsMap,
    ) -> f32 {
        let total_documents = term_frequency_across_documents_cache.len() as f32;
        let term_frequency_across_documents = term_frequency_across_documents_cache
            .get(term)
            .cloned()
            .unwrap_or(1) as f32;

        (total_documents / term_frequency_across_documents).log10()
    }

    pub fn index_directory(dirpath: &str, tf_idf_model: &mut TfIdfModel) {
        let dir_entries = Fetcher::fetch_directory(dirpath);
        let total_entries = dir_entries.len();

        for (index, file) in dir_entries.into_iter().enumerate() {
            print!(
                "Indexing... {:.2}%\r",
                (index as f32 / total_entries as f32) * 100.0
            );

            let file_content = Reader::read_file(&file);

            if file_content.is_err() {
                continue;
            }

            let file_content = file_content.unwrap();

            let document_map = Parser::index(file_content.chars().collect::<Vec<char>>());

            for term in document_map.frequency_map.keys() {
                tf_idf_model
                    .term_frequency_across_documents
                    .entry(term.to_string())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }

            tf_idf_model.term_frequency_per_document.insert(
                file.clone().into_os_string().into_string().unwrap(),
                document_map,
            );
        }
    }
}
