use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, strand: &str) -> Result<usize, &'static str> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err("Invalid nucleotide");
    }

    if strand.chars().any(|c| !NUCLEOTIDES.contains(&c)) {
        return Err("Invalid DNA strand");
    }

    Ok(strand.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char, usize>, &'static str> {
    NUCLEOTIDES
        .iter()
        .map(|&n| count(n, strand).map(|c| (n, c)))
        .collect()
}
