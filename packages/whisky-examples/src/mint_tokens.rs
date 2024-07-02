use whisky::{
    builder::{IMeshTxBuilder, MeshTxBuilder, WData, WRedeemer},
    csl::JsError,
    model::{Asset, Budget, ProvidedScriptSource, UTxO},
};

pub async fn mint_tokens(
    to_mint_asset: Asset,
    redeemer: &str,
    script: ProvidedScriptSource,
    my_address: &str,
    inputs: Vec<UTxO>,
    collateral: UTxO,
) -> Result<String, JsError> {
    let mut mesh = MeshTxBuilder::new_core();

    mesh.mint_plutus_script_v2()
        .mint(
            to_mint_asset.quantity_u64(),
            &to_mint_asset.policy(),
            &to_mint_asset.name(),
        )
        .minting_script(&script.script_cbor, script.language_version)
        .mint_redeemer_value(WRedeemer {
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
