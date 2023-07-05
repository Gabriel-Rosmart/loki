use std::collections::HashMap;

// Maps each term with how many times appears in a single document
type FrequencyMap = HashMap<String, usize>;

// Maps how many times appears each term across all files
type TermFrequencyAcrossDocumentsMap = HashMap<String, usize>;

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
