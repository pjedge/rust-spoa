# rust-spoa

This crate is a Rust wrapper and interface to the [SPOA](https://github.com/rvaser/spoa) (simd-accelerated partial order alignment) library.
This library allows the efficient generation of a consensus sequence from a set of DNA or protein sequences.

If you use this crate, please cite the original authors of SPOA:

[Vaser, R., Sović, I., Nagarajan, N. and Šikić, M., 2017. Fast and accurate de novo genome assembly from long uncorrected reads. Genome research, 27(5), pp.737-746.](https://genome.cshlp.org/content/27/5/737)

To use this crate, add the following to your ```Cargo.toml```:
```
[dependencies]
rust-spoa = "*"
```
And add this to your crate root:
```
extern crate rust_spoa;
```

For description of the API, see [the documentation](https://docs.rs/rust-spoa/0.1.0/rust_spoa/):
Example usage:
```
extern crate rust_spoa;

use rust_spoa::poa_consensus;

fn main() {
    let mut seqs = vec![];

    // generated each string by adding small tweaks to the expected consensus "AATGCCCGTT"
    // convert sequences to Vec<u8>
    for seq in ["ATTGCCCGTT",
        "AATGCCGTT",
        "AATGCCCGAT",
        "AACGCCCGTC",
        "AGTGCTCGTT",
        "AATGCTCGTT"].iter() {
        seqs.push((*seq).bytes().map(|x|{x as u8}).collect::<Vec<u8>>());
    }

    // length of consensus vec is an upper bound on the output consensus length
    let consensus_max_len = 20;
    let mut consensus: Vec<u8> = vec![0u8; consensus_max_len];

    // generate consensus sequence
    poa_consensus(&seqs, &mut consensus, 1, 5, -4, -3, -1);

    let expected = "AATGCCCGTT".to_string().into_bytes();
    assert_eq!(consensus, expected);
}
```
