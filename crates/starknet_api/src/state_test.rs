use super::StateDiff;
use crate::{
    shash, ClassHash, ContractAddress, ContractClass, ContractNonce, DeclaredContract,
    DeployedContract, Nonce, StarkHash, StorageDiff,
};

#[test]
fn test_sorted() {
    let hash0 = shash!("0x0");
    let hash1 = shash!("0x1");

    let dep_contract_0 =
        DeployedContract { address: ContractAddress(hash0), class_hash: ClassHash(hash0) };
    let dep_contract_1 =
        DeployedContract { address: ContractAddress(hash1), class_hash: ClassHash(hash1) };

    let storage_diff_0 = StorageDiff { address: ContractAddress(hash0), storage_entries: vec![] };
    let storage_diff_1 = StorageDiff { address: ContractAddress(hash1), storage_entries: vec![] };

    let dec_contract_0 =
        DeclaredContract { class_hash: ClassHash(hash0), contract_class: ContractClass::default() };
    let dec_contract_1 =
        DeclaredContract { class_hash: ClassHash(hash1), contract_class: ContractClass::default() };

    let nonce_0 = ContractNonce { contract_address: ContractAddress(hash0), nonce: Nonce(hash0) };
    let nonce_1 = ContractNonce { contract_address: ContractAddress(hash1), nonce: Nonce(hash1) };

    let unsorted_deployed_contracts = vec![dep_contract_1.clone(), dep_contract_0.clone()];
    let unsorted_storage_diffs = vec![storage_diff_1.clone(), storage_diff_0.clone()];
    let unsorted_declared_contracts = vec![dec_contract_1.clone(), dec_contract_0.clone()];
    let unsorted_nonces = vec![nonce_1.clone(), nonce_0.clone()];

    let state_diff = StateDiff::new(
        unsorted_deployed_contracts,
        unsorted_storage_diffs,
        unsorted_declared_contracts,
        unsorted_nonces,
    );

    let sorted_deployed_contracts = vec![dep_contract_0, dep_contract_1];
    let sorted_storage_diffs = vec![storage_diff_0, storage_diff_1];
    let sorted_declared_contracts = vec![dec_contract_0, dec_contract_1];
    let sorted_nonces = vec![nonce_0, nonce_1];

    assert_eq!(
        state_diff.destruct(),
        (sorted_deployed_contracts, sorted_storage_diffs, sorted_declared_contracts, sorted_nonces)
    );
}
