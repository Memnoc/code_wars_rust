// HEADER: My solution
fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

// HEADER: kind of like this more verbose approach
fn alternate_dna_to_rna(dna: &str) -> String {
    dna.chars()
        .map(|x| match x {
            'T' => 'U',
            _ => x,
        })
        .collect()
}

// TODO: Create a function that converts DNA to RNA.
// The lenght is arbitrary, the string can be empty.
fn main() {
    println!("=== DNA to RNA conversion ===");
    let conversion_one = "GCAT";
    let result_one = dna_to_rna(conversion_one);
    println!("Converted string is: {} ", result_one);
}

#[cfg(test)]
mod test {
    use super::dna_to_rna;

    #[test]
    fn test_dna_to_rna() {
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna(""), "");
    }
}
