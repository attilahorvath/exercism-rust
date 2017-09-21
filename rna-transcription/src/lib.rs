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
        self.0.chars().map(|c| match c {
            'G' => Ok('C'),
            'C' => Ok('G'),
            'T' => Ok('A'),
            'A' => Ok('U'),
            _ => Err("Invalid DNA strand")
        }).collect::<Result<String, &'static str>>().map(RibonucleicAcid)
    }
}
