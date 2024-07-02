use whisky::{
    builder::{IMeshTxBuilder, MeshTxBuilder},
    csl::JsError,
    model::UTxO,
};

pub fn delegate_stake(
    stake_key_hash: &str,
    pool_id: &str, // In the form of 'poolxxxxxx'
    my_address: &str,
    inputs: Vec<UTxO>,
) -> Result<String, JsError> {
    let mut mesh = MeshTxBuilder::new_core();
    mesh.register_stake_certificate(stake_key_hash)
        .delegate_stake_certificate(stake_key_hash, pool_id)
        .change_address(my_address)
        .select_utxos_from(inputs.clone(), 5000000)
        .complete_sync(None)?;

    Ok(mesh.tx_hex())
}
