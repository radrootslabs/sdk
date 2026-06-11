use crate::{
    manifest::{manifest_file_name, package_manifest},
    package_matrix::{package_specs, validate_package_matrix},
    ts::{generated_constants_file, generated_header, generated_types_file, normalize_lf},
};

pub fn generate_ts() -> Result<(), String> {
    validate_package_matrix()?;
    let header = normalize_lf(generated_header());
    for spec in package_specs() {
        let manifest = package_manifest(*spec);
        println!(
            "planned TypeScript generation for {} with {}, {}, {}, and {}",
            manifest["package"],
            generated_types_file(),
            generated_constants_file(),
            manifest_file_name(),
            header.lines().next().unwrap_or_default()
        );
    }
    Ok(())
}
