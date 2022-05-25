#[cfg(test)]
use super::*;

#[test]
fn compute_commitments_works() {
    // initialize backend, choosing between GPU and CPU
    init_backend(Backend::GPU);

    // generate input table
    let data1 = vec![2000, 7500, 5000, 1500];
    let data2 = vec![5000, 0, 400000, 10, 0, 0];
    let data3 = vec![2000 + 5000, 7500 + 0, 5000 + 400000, 1500 + 10];

    let mut table: Vec<Sequence> = Vec::new();
    
    table.push(Sequence::Bytes16(&data1));
    table.push(Sequence::Bytes32(&data2));
    table.push(Sequence::Bytes64(&data3));

    let commit1 = Commitment::from_slice(
        &([
            4,105,58,131,59,69,150,106,
            120,137,32,225,175,244,82,115,
            216,180,206,150,21,250,240,98,
            251,192,146,244,54,169,199,97
        ] as [u8; 32])
    );

    let mut commitments = vec![commit1; table.len()];
    
    compute_commitments(& mut commitments[..], &table);

    let commit2 = Commitment::from_slice(
        &([
            2,254,178,195,198,238,44,156,
            24,29,88,196,37,63,157,50,
            236,159,61,49,153,181,79,126,
            55,188,67,1,228,248,72,51
        ] as [u8; 32])
    );

    let commit3 = Commitment::from_slice(
        &([
            30,237,163,234,252,111,45,133,
            235,227,21,117,229,188,88,149,
            240,109,205,90,6,130,199,152,
            5,221,57,231,168,9,141,122
        ] as [u8; 32])
    );

    // verify if commitment results are correct
    assert_eq!(commitments[0], commit1);
    assert_eq!(commitments[1], commit2);
    assert_eq!(commitments[2], commit3);
}
