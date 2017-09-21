#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(rna: &str) -> RibonucleicAcid {
        RibonucleicAcid(rna.into())
    }
}

#[derive(PartialEq, Debug)]
pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(dna.into())
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, &'static str> {
        let mut valid = true;

        let rna = self.0.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => { valid = false; ' ' }
        }).collect();

        if valid {
            Ok(RibonucleicAcid(rna))
        } else {
            Err("Invalid DNA strand")
        }
    }
}
