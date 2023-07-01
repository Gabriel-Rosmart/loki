use std::collections::HashMap;

pub struct Indexer;

type FrequencyMap = HashMap<String, usize>;
type DocumentIndexMap = HashMap<String, HashMap<String, usize>>;

impl Indexer {
    pub fn term_frequency(term: &str, document_frequencies: &FrequencyMap) -> f32 {
        *document_frequencies.get(term).unwrap_or(&0) as f32
            / document_frequencies
                .iter()
                .map(|(_, freq)| *freq)
                .sum::<usize>() as f32
    }

    pub fn inverse_document_frequency(term: &str, documents_frequencies: &DocumentIndexMap) -> f32 {
        let total_documents = documents_frequencies.len() as f32;
        let term_frequency_across_documents = documents_frequencies
            .values()
            .filter(|term_frequency| term_frequency.contains_key(term))
            .count()
            .max(1) as f32;

        (total_documents / term_frequency_across_documents).log10()
    }
}
