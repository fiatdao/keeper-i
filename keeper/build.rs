use ethers::contract::Abigen;

// TODO: Figure out how to write the rerun-if-changed script properly
fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed=./abis/*.json");
    bindgen("Codex").unwrap();
    bindgen("NoLossCollateralAuction").unwrap();
    bindgen("Collybus").unwrap();
    bindgen("Limes").unwrap();
    bindgen("IVault").unwrap();
    bindgen("IMulticall2").unwrap();
}

#[allow(dead_code)]
fn bindgen(fname: &str) -> Result<(), anyhow::Error> {
    let bindings = Abigen::new(fname, format!("./abis/{}.json", fname))
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generate bindings");

    bindings.write_to_file(format!("./src/bindings/{}.rs", fname.to_lowercase()))
}
