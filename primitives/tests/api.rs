// SPDX-License-Identifier: CC0-1.0

//! Test the API surface of `primitives`.
//!
//! The point of these tests are to check the API surface as opposed to test the API functionality.
//!
//! ref: <https://rust-lang.github.io/api-guidelines/about.html>

#![allow(dead_code)]
#![allow(unused_imports)]
// No benefit in running this test without features enabled.
#![cfg(feature = "alloc")]
#![cfg(feature = "hex")]
#![cfg(feature = "arbitrary")]

use arbitrary::Arbitrary;
use bitcoin_primitives::block::{Checked, Unchecked};
use bitcoin_primitives::script::{self, ScriptHash, WScriptHash};
use bitcoin_primitives::{
    absolute, block, pow, relative, transaction, witness, OutPoint, RedeemScript,
    RedeemScriptBuf, ScriptPubKey, ScriptPubKeyBuf, ScriptSig, ScriptSigBuf, Sequence, TapScript,
    TapScriptBuf, Transaction, TxIn, TxOut, Txid, Witness, WitnessScript, WitnessScriptBuf, Wtxid,
    BlockHash, TxMerkleNode, WitnessMerkleNode, WitnessCommitment,
};
use hashes::sha256t;

/// A struct that includes all public non-error enums.
#[derive(Debug)] // All public types implement Debug (C-DEBUG).
struct Enums {
    a: block::Checked, // Empty enums are not constructable.
    b: block::Unchecked,
    c: absolute::LockTime,
    d: relative::LockTime,
}

/// A struct that includes all public non-error structs.
#[derive(Debug)] // All public types implement Debug (C-DEBUG).
struct Structs<'a> {
    a: block::Block<Checked>,
    b: block::Block<Unchecked>,
    c: block::Header,
    d: block::Version,
    e: BlockHash,
    f: block::WitnessCommitment,
    g: TxMerkleNode,
    h: WitnessMerkleNode,
    i: pow::CompactTarget,
    j1: &'a RedeemScript,
    j2: &'a ScriptPubKey,
    j3: &'a ScriptSig,
    j4: &'a TapScript,
    j5: &'a WitnessScript,
    k: ScriptHash,
    l: WScriptHash,
    m1: RedeemScriptBuf,
    m2: ScriptPubKeyBuf,
    m3: ScriptSigBuf,
    m4: TapScriptBuf,
    m5: WitnessScriptBuf,
    n: Sequence,
    o: Transaction,
    p: TxIn,
    q: TxOut,
    r: OutPoint,
    s: Txid,
    t: Wtxid,
    u: transaction::Version,
    v: Witness,
    // w: witness::Iter<'a>,
}

static REDEEM_SCRIPT: RedeemScriptBuf = RedeemScriptBuf::new();
static SCRIPT_SIG: ScriptSigBuf = ScriptSigBuf::new();
static SCRIPT_PUB_KEY: ScriptPubKeyBuf = ScriptPubKeyBuf::new();
static TAP_SCRIPT: TapScriptBuf = TapScriptBuf::new();
static WITNESS_SCRIPT: WitnessScriptBuf = WitnessScriptBuf::new();
static BYTES: [u8; 32] = [0x00; 32];

/// Public structs that derive common traits.
// C-COMMON-TRAITS excluding `Debug`, `Default`, `Display`, `Ord`, `PartialOrd`, `Hash`.
#[derive(Clone, PartialEq, Eq)]
struct CommonTraits {
    a: block::Block<Checked>,
    b: block::Block<Unchecked>,
    c: block::Header,
    d: block::Version,
    e: BlockHash,
    f: WitnessCommitment,
    g: TxMerkleNode,
    h: WitnessMerkleNode,
    i: pow::CompactTarget,
    // j: &'a Script,
    k: ScriptHash,
    l: WScriptHash,
    m1: RedeemScriptBuf,
    m2: ScriptPubKeyBuf,
    m3: ScriptSigBuf,
    m4: TapScriptBuf,
    m5: WitnessScriptBuf,
    n: Sequence,
    o: Transaction,
    p: TxIn,
    q: TxOut,
    r: OutPoint,
    s: Txid,
    t: Wtxid,
    u: transaction::Version,
    v: Witness,
    // w: witness::Iter<'a>,
}

/// A struct that includes all types that implement `Clone`.
#[derive(Clone)] // C-COMMON-TRAITS: `Clone`
struct Clone<'a> {
    a: block::Block<Checked>,
    b: block::Block<Unchecked>,
    c: block::Header,
    d: block::Version,
    e: BlockHash,
    f: WitnessCommitment,
    g: TxMerkleNode,
    h: WitnessMerkleNode,
    i: pow::CompactTarget,
    // j: &'a Script,
    k: ScriptHash,
    l: WScriptHash,
    m1: RedeemScriptBuf,
    m2: ScriptPubKeyBuf,
    m3: ScriptSigBuf,
    m4: TapScriptBuf,
    m5: WitnessScriptBuf,
    n: Sequence,
    o: Transaction,
    p: TxIn,
    q: TxOut,
    r: OutPoint,
    s: Txid,
    t: Wtxid,
    u: transaction::Version,
    v: Witness,
    w: witness::Iter<'a>,
}

/// Public structs that derive common traits.
// C-COMMON-TRAITS excluding `Clone`, `Debug`, `Default`, and `Display`
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ord {
    // a: block::Block<Checked>,
    // b: block::Block<Unchecked>,
    c: block::Header,
    d: block::Version,
    e: BlockHash,
    f: WitnessCommitment,
    g: TxMerkleNode,
    h: WitnessMerkleNode,
    i: pow::CompactTarget,
    // j: &'a Script,  // Doesn't implement `Clone`.
    k: ScriptHash,
    l: WScriptHash,
    m1: RedeemScriptBuf,
    m2: ScriptPubKeyBuf,
    m3: ScriptSigBuf,
    m4: TapScriptBuf,
    m5: WitnessScriptBuf,
    n: Sequence,
    o: Transaction,
    p: TxIn,
    q: TxOut,
    r: OutPoint,
    s: Txid,
    t: Wtxid,
    u: transaction::Version,
    v: Witness,
    // w: witness::Iter<'a>,
}

/// A struct that includes all types that implement `Default`.
#[derive(Default, Debug, PartialEq, Eq)] // C-COMMON-TRAITS: `Default` (others just so we can test).
struct Default {
    a: block::Version,
    b1: &'static RedeemScript,
    b2: &'static ScriptPubKey,
    b3: &'static ScriptSig,
    b4: &'static TapScript,
    b5: &'static WitnessScript,
    c1: RedeemScriptBuf,
    c2: ScriptPubKeyBuf,
    c3: ScriptSigBuf,
    c4: TapScriptBuf,
    c5: WitnessScriptBuf,
    d: Sequence,
    e: Witness,
}

/// A struct that includes all public error types.
// These derives are the policy of `rust-bitcoin` not Rust API guidelines.
#[derive(Debug, Clone, PartialEq, Eq)] // All public types implement Debug (C-DEBUG).
struct Errors {
    a: transaction::ParseOutPointError,
    b: relative::error::DisabledLockTimeError,
    c: relative::error::IsSatisfiedByError,
    d: relative::error::IsSatisfiedByHeightError,
    e: relative::error::IsSatisfiedByTimeError,
    f: script::RedeemScriptSizeError,
    g: script::WitnessScriptSizeError,
}

#[test]
fn api_can_use_units_modules_from_crate_root() {
    use bitcoin_primitives::{amount, block, fee_rate, locktime, weight};
}

#[test]
fn api_can_use_units_types_from_crate_root() {
    use bitcoin_primitives::{Amount, BlockHeight, BlockInterval, FeeRate, SignedAmount, Weight};
}

#[test]
fn api_can_use_all_units_types_from_module_amount() {
    use bitcoin_primitives::amount::{
        Amount, Denomination, Display, OutOfRangeError, ParseAmountError, ParseDenominationError,
        ParseError, SignedAmount,
    };
}

#[test]
fn api_can_use_all_units_types_from_module_amount_error() {
    use bitcoin_primitives::amount::error::{
        InputTooLargeError, InvalidCharacterError, MissingDenominationError, MissingDigitsError,
        OutOfRangeError, ParseAmountError, ParseDenominationError, ParseError,
        PossiblyConfusingDenominationError, TooPreciseError, UnknownDenominationError,
    };
}

#[test]
fn api_can_use_modules_from_crate_root() {
    use bitcoin_primitives::{
        block, locktime, pow, script, sequence, transaction, witness,
    };
}

#[test]
fn api_can_use_types_from_crate_root() {
    use bitcoin_primitives::{
        Block, BlockHash, BlockHeader, BlockVersion, CompactTarget, OutPoint, ScriptPubKey,
        ScriptPubKeyBuf, ScriptSig, ScriptSigBuf, Sequence, Transaction, TransactionVersion, TxIn,
        TxMerkleNode, TxOut, Txid, Witness, WitnessCommitment, WitnessMerkleNode, Wtxid,
    };
}

#[test]
fn api_can_use_all_types_from_module_locktime() {
    use bitcoin_primitives::locktime::relative::error::{
        DisabledLockTimeError, InvalidHeightError, InvalidTimeError,
    };
    use bitcoin_primitives::locktime::relative::LockTime;
    use bitcoin_primitives::locktime::{absolute, relative};
}

#[test]
fn api_can_use_all_types_from_module_script() {
    use bitcoin_primitives::script::{
        RedeemScriptSizeError, ScriptHash, ScriptPubKey, ScriptPubKeyBuf, ScriptSig, ScriptSigBuf,
        WScriptHash, WitnessScriptSizeError,
    };
}

// `Debug` representation is never empty (C-DEBUG-NONEMPTY).
#[test]
fn api_all_non_error_types_have_non_empty_debug() {
    macro_rules! check_debug {
        ($($t:expr);* $(;)?) => {
            $(
                let debug = format!("{:?}", $t);
                assert!(!debug.is_empty());
            )*
        }
    }

    // All the enums.
    check_debug! {
        absolute::LockTime::ZERO;
        relative::LockTime::ZERO
    };

    // We abuse `Arbitrary` here to get a quick and dirty instance.
    let ab: [u8; 32] = [0xab; 32];
    let mut u = arbitrary::Unstructured::new(&ab);
    let transaction = Transaction::arbitrary(&mut u).unwrap();

    // All the structs.
    check_debug! {
        block::Block::<Unchecked>::arbitrary(&mut u).unwrap().assume_checked(None);
        block::Block::<Unchecked>::arbitrary(&mut u).unwrap();
        block::Header::arbitrary(&mut u).unwrap();
        block::Version::arbitrary(&mut u).unwrap();
        BlockHash::from_byte_array(BYTES);
        block::WitnessCommitment::from_byte_array(BYTES);
        TxMerkleNode::from_byte_array(BYTES);
        WitnessMerkleNode::from_byte_array(BYTES);
        pow::CompactTarget::from_consensus(0x1d00_ffff);
        REDEEM_SCRIPT.as_script();
        SCRIPT_SIG.as_script();
        SCRIPT_PUB_KEY.as_script();
        TAP_SCRIPT.as_script();
        WITNESS_SCRIPT.as_script();
        ScriptHash::from_script(&REDEEM_SCRIPT).unwrap();
        WScriptHash::from_script(&WITNESS_SCRIPT).unwrap();
        REDEEM_SCRIPT.clone();
        SCRIPT_SIG.clone();
        SCRIPT_PUB_KEY.clone();
        TAP_SCRIPT.clone();
        WITNESS_SCRIPT.clone();
        Sequence::arbitrary(&mut u).unwrap();
        Transaction::arbitrary(&mut u).unwrap();
        TxIn::arbitrary(&mut u).unwrap();
        TxOut::arbitrary(&mut u).unwrap();
        OutPoint::arbitrary(&mut u).unwrap();
        transaction.compute_txid();
        transaction.compute_wtxid();
        transaction.version;
        Witness::arbitrary(&mut u).unwrap();
        // ad: witness::Iter<'a>,
    };
}

#[test]
fn all_types_implement_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    //  Types are `Send` and `Sync` where possible (C-SEND-SYNC).
    assert_send::<Structs>();
    assert_sync::<Structs>();
    assert_send::<Enums>();
    assert_sync::<Enums>();

    // Error types should implement the Send and Sync traits (C-GOOD-ERR).
    assert_send::<Errors>();
    assert_sync::<Errors>();
}

#[test]
fn regression_default() {
    let got: Default = Default::default();
    let want = Default {
        a: block::Version::NO_SOFT_FORK_SIGNALLING,
        b1: RedeemScript::from_bytes(&[]),
        b2: ScriptPubKey::from_bytes(&[]),
        b3: ScriptSig::from_bytes(&[]),
        b4: TapScript::from_bytes(&[]),
        b5: WitnessScript::from_bytes(&[]),
        c1: RedeemScriptBuf::from_bytes(Vec::new()),
        c2: ScriptPubKeyBuf::from_bytes(Vec::new()),
        c3: ScriptSigBuf::from_bytes(Vec::new()),
        c4: TapScriptBuf::from_bytes(Vec::new()),
        c5: WitnessScriptBuf::from_bytes(Vec::new()),
        d: Sequence::MAX,
        e: Witness::new(),
    };
    assert_eq!(got, want);
}

#[test]
// The only trait in this crate is `block::Validation` and it is not dyn compatible.
fn dyn_compatible() {}
