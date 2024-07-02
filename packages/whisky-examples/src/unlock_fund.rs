use whisky::{
    builder::{IMeshTxBuilder, MeshTxBuilder, WData, WRedeemer},
    csl::JsError,
    model::{Budget, UTxO},
};

pub async fn unlock_fund(
    script_utxo: UTxO,
    redeemer: &str,
    my_address: &str,
    inputs: Vec<UTxO>,
    collateral: UTxO,
) -> Result<String, JsError> {
    let mut mesh = MeshTxBuilder::new_core();
    mesh.spending_plutus_script_v2()
        .tx_in(
            &script_utxo.input.tx_hash,
            script_utxo.input.output_index,
            script_utxo.output.amount,
            &script_utxo.output.address,
        )
        .tx_in_inline_datum_present()
        // .tx_in_datum_value(datum here) or provide datum value
        .tx_in_redeemer_value(WRedeemer {
            data: WData::JSON(redeemer.to_string()),
            ex_units: Budget { mem: 0, steps: 0 },
        })
        .change_address(my_address)
        .tx_in_collateral(
            &collateral.input.tx_hash,
            collateral.input.output_index,
            collateral.output.amount,
            &collateral.output.address,
        )
        .select_utxos_from(inputs.clone(), 5000000)
        .complete(None)
        .await?;

    Ok(mesh.tx_hex())
}
