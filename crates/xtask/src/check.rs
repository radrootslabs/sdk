use std::{fs, path::Path};

use crate::{
    fs::workspace_root,
    package_matrix::{FORBIDDEN_PACKAGE_NAMES, package_specs, validate_package_matrix},
};

pub fn check() -> Result<(), String> {
    validate_package_matrix()?;
    let root = workspace_root()?;
    check_forbidden_packages(&root)?;
    for spec in package_specs() {
        let package_dir = root.join(spec.package_dir);
        let package_json_path = package_dir.join("package.json");
        let index_path = package_dir.join("src/index.ts");
        check_package_json(&package_json_path, spec.package_name)?;
        if !index_path.is_file() {
            return Err(format!("missing package index: {}", index_path.display()));
        }
    }
    Ok(())
}

fn check_forbidden_packages(root: &Path) -> Result<(), String> {
    for forbidden in FORBIDDEN_PACKAGE_NAMES {
        let package_leaf = forbidden.trim_start_matches("@radroots/").to_owned();
        let forbidden_dir = root.join("packages").join(package_leaf);
        if forbidden_dir.exists() {
            return Err(format!(
                "forbidden package directory exists: {}",
                forbidden_dir.display()
            ));
        }
    }
    Ok(())
}

fn check_package_json(path: &Path, expected_name: &str) -> Result<(), String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let json = serde_json::from_str::<serde_json::Value>(&raw)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    let actual_name = json
        .get("name")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("package.json missing name: {}", path.display()))?;
    if actual_name != expected_name {
        return Err(format!(
            "package name mismatch in {}: expected {expected_name}, found {actual_name}",
            path.display()
        ));
    }
    let private = json
        .get("private")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if !private {
        return Err(format!("package must be private: {}", path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn package_skeleton_is_valid() {
        check().expect("package skeleton validates");
    }
}
