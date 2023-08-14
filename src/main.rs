fn main() {
    let cargo_toml = include_str!("Cargo.toml");
    let document: toml_edit::Document = cargo_toml.parse().unwrap();
    let workspace = document.get("workspace").unwrap();
    let dependencies = workspace.get("dependencies").unwrap();
    let arrow = dependencies.get("arrow").unwrap();

    let other_cargo_toml = r#"
[dependencies]
polars-error = { version = "0.32.0", path = "../polars-error" }

arrow = { workspace = true }
atoi = { workspace = true, optional = true }
    "#;

    let mut doc2: toml_edit::Document = other_cargo_toml.parse().unwrap();
    let doc2_deps = doc2
        .get_mut("dependencies")
        .unwrap()
        .as_table_mut()
        .unwrap();
    doc2_deps["arrow"] = arrow.clone();
    println!("{}", doc2.to_string());
}
