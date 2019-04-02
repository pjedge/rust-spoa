
//! This crate is a Rust wrapper and interface to the [SPOA](https://github.com/rvaser/spoa) (simd-accelerated partial order alignment) library.
//! This library allows the efficient generation of a consensus sequence from a set of DNA or protein sequences.
//!
//! If you use this crate, please cite the original authors of SPOA:
//!
//! [Vaser, R., Sović, I., Nagarajan, N. and Šikić, M., 2017. Fast and accurate de novo genome assembly from long uncorrected reads. Genome research, 27(5), pp.737-746.](https://genome.cshlp.org/content/27/5/737)

extern "C" {
    fn poa_func(
        seqs: *const *const u8,
        num_seqs: i32,
        consensus: *const u8,
        consensus_len: i32,
        alignment_type: i32, // 0 = local, 1 = global, 2 = gapped
        match_score: i32,
        mismatch_score: i32,
        gap_open: i32,
        gap_extend: i32,
    ) -> u32;
}

/// Generates a consensus sequence from a list of sequences.
/// # Arguments
///
/// * `seqs` - a vector holding the sequences (each as a vector of u8) to form a consensus from
/// * `consensus_max_len` - The upper bound for the output consensus length. If the output consensus sequence is longer than this value, it will be truncated to this length. Setting a large value uses more memory and runtime, since a buffer of this size is allocated internally.
/// * `alignment_type` - alignment mode: 0 = local, 1 = global, 2 = gapped
/// * `match_score` - the match score for alignment
/// * `mismatch_score` - the mismatch score for alignment
/// * `gap_open` - the gap open score for alignment
/// * `gap_extend` - the gap extend score for alignment
///
/// # Returns
/// * returns the consensus of the input sequences as a vector of u8
///
/// # Examples
///
/// ```
///     use rust_spoa::poa_consensus;
///
///     fn test_dna_consensus() {
///        let mut seqs = vec![];
///
///        // generated each string by adding small tweaks to the expected consensus "AATGCCCGTT"
///        // convert sequences to Vec<u8>
///        for seq in ["ATTGCCCGTT",
///            "AATGCCGTT",
///            "AATGCCCGAT",
///            "AACGCCCGTC",
///            "AGTGCTCGTT",
///            "AATGCTCGTT"].iter() {
///            seqs.push((*seq).bytes().map(|x|{x as u8}).collect::<Vec<u8>>());
///        }
///
///        // generate consensus sequence
///        let consensus = poa_consensus(&seqs, 20, 1, 5, -4, -3, -1);
///
///        let expected = "AATGCCCGTT".to_string().into_bytes();
///        assert_eq!(consensus, expected);
///    }
/// ```

pub fn poa_consensus(
    seqs: &Vec<Vec<u8>>,
    consensus_max_length: usize,
    alignment_type: i32,
    match_score: i32,
    mismatch_score: i32,
    gap_open: i32,
    gap_extend: i32
) -> Vec<u8> {

    let mut consensus: Vec<u8> = vec![0; consensus_max_length];

    let num_seqs = seqs.len() as i32;
    let consensus_len = consensus.len() as i32;

    let mut seq_ptrs: Vec<*const u8> = Vec::with_capacity(seqs.len());

    for seq in seqs {
        seq_ptrs.push(seq.as_ptr());
    }

    unsafe {

        let len = poa_func(
            seq_ptrs.as_ptr(),
            num_seqs,
            consensus.as_ptr(),
            consensus_len,
            alignment_type,
            match_score,
            mismatch_score,
            gap_open,
            gap_extend
        );

        consensus.truncate(len as usize);
    }


    consensus
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_consensus() {
        let mut seqs = vec![];

        // generated each string by adding small tweaks to the expected consensus "AATGCCCGTT"
        for seq in ["ATTGCCCGTT",
            "AATGCCGTT",
            "AATGCCCGAT",
            "AACGCCCGTC",
            "AGTGCTCGTT",
            "AATGCTCGTT"].iter() {
            seqs.push((*seq).bytes().map(|x|{x as u8}).collect::<Vec<u8>>());
        }

        let consensus = poa_consensus(&seqs, 20, 1, 5, -4, -3, -1);

        let expected = "AATGCCCGTT".to_string().into_bytes();
        assert_eq!(consensus, expected);
    }


    #[test]
    fn test_protein_consensus() {
        let mut seqs = vec![];
        // expect consensus "FNLKPSWDDCQ"
        for seq in ["FNLKESWDDCQ".to_string(),
            "FNLKPSWDCQ".to_string(),
            "FNLKSPSWDDCQ".to_string(),
            "FNLKASWCQ".to_string(),
            "FLKPSWDDCQ".to_string(),
            "FNLKPSWDADCQ".to_string()].iter() {
            seqs.push(seq.chars().into_iter().map(|x|{x as u8}).collect::<Vec<u8>>());
        }

        let consensus = poa_consensus(&seqs, 20, 1, 5, -4, -3, -1);

        let expected = "FNLKPSWDDCQ".to_string().into_bytes();
        assert_eq!(consensus, expected);

    }
}
