use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).map(|name| *name)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut names = Vec::new();

        for chunk in rna.chars().collect::<Vec<_>>().chunks(3) {
            let codon = chunk.iter().collect::<String>();

            if let Some(name) = self.name_for(&codon) {
                if name == "stop codon" {
                    break;
                }

                names.push(name);
            } else {
                return None;
            }
        }

        Some(names)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.iter().cloned().collect(),
    }
}
