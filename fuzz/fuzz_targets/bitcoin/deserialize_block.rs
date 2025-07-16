use bitcoin::block::{self, Block, BlockCheckedExt as _};
use honggfuzz::fuzz;

fn do_test(data: &[u8]) {
    let block_result: Result<bitcoin::block::Block, _> =
        bitcoin::consensus::encode::deserialize(data);

    match block_result {
        Err(_) => {}
        Ok(block) => {
            let ser = bitcoin::consensus::encode::serialize(&block);
            assert_eq!(&ser[..], data);

            // Manually call all compute functions with unchecked block data.
            let (header, transactions) = block.into_parts();
            block::compute_merkle_root(&transactions);
            block::compute_witness_commitment(&transactions, &[]); // TODO: Is empty slice ok?
            block::compute_witness_root(&transactions);

            if let Ok(block) = Block::new_checked(header, transactions) {
                let _ = block.bip34_block_height();
                block.block_hash();
                block.weight();
            }
        }
    }
}

fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(all(test, fuzzing))]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00", &mut a);
        super::do_test(&a);
    }
}
