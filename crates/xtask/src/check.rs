use std::{fs, path::Path};

use crate::{
    fs::workspace_root,
    output::package_outputs,
    package_matrix::{
        FORBIDDEN_PACKAGE_NAMES, package_specs, validate_package_matrix, wasm_package_specs,
    },
};

pub fn check() -> Result<(), String> {
    validate_package_matrix()?;
    let root = workspace_root()?;
    check_forbidden_packages(&root)?;
    check_binding_crate_sources(&root)?;
    for spec in package_specs() {
        let package_dir = root.join(spec.package_dir);
        let package_json_path = package_dir.join("package.json");
        let index_path = package_dir.join("src/index.ts");
        check_package_json(&package_json_path, spec.package_name)?;
        if !index_path.is_file() {
            return Err(format!("missing package index: {}", index_path.display()));
        }
    }
    for spec in wasm_package_specs() {
        let package_dir = root.join(spec.package_dir);
        check_package_json(&package_dir.join("package.json"), spec.package_name)?;
    }
    for output in package_outputs() {
        for expected in output.files() {
            let path = root
                .join(output.spec.package_dir)
                .join(expected.relative_path);
            let actual = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            if actual != expected.contents {
                return Err(format!("stale generated output: {}", path.display()));
            }
        }
    }
    Ok(())
}

fn check_binding_crate_sources(root: &Path) -> Result<(), String> {
    for spec in package_specs() {
        let crate_src_dir = root.join(spec.crate_dir).join("src");
        let typescript_dir = crate_src_dir.join("typescript");
        if typescript_dir.exists() {
            return Err(format!(
                "forbidden crate TypeScript source directory exists: {}",
                typescript_dir.display()
            ));
        }
        check_no_typescript_files(&crate_src_dir)?;
    }
    for spec in wasm_package_specs() {
        check_no_typescript_files(&root.join(spec.crate_dir).join("src"))?;
    }
    Ok(())
}

fn check_no_typescript_files(dir: &Path) -> Result<(), String> {
    for entry in
        fs::read_dir(dir).map_err(|error| format!("failed to read {}: {error}", dir.display()))?
    {
        let entry =
            entry.map_err(|error| format!("failed to read {} entry: {error}", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            check_no_typescript_files(&path)?;
        } else if file_type.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("ts")
        {
            return Err(format!(
                "forbidden crate TypeScript source file exists: {}",
                path.display()
            ));
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
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{check, check_binding_crate_sources, check_no_typescript_files};

    #[test]
    fn package_skeleton_is_valid() {
        check().expect("package skeleton validates");
    }

    #[test]
    fn rejects_crate_typescript_directories() {
        let root = test_root("typescript_dir");
        let typescript_dir = root
            .join("crates")
            .join("core_bindings")
            .join("src")
            .join("typescript");
        fs::create_dir_all(&typescript_dir).expect("create forbidden directory");

        let error = check_binding_crate_sources(&root).expect_err("forbidden directory rejected");

        assert!(error.contains("forbidden crate TypeScript source directory"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_crate_typescript_files() {
        let root = test_root("typescript_file");
        let src_dir = root.join("crates/core_bindings/src");
        fs::create_dir_all(&src_dir).expect("create crate source directory");
        fs::write(src_dir.join("types.ts"), "export type A = string;\n")
            .expect("write forbidden file");

        let error = check_no_typescript_files(&src_dir).expect_err("forbidden file rejected");

        assert!(error.contains("forbidden crate TypeScript source file"));
        let _ = fs::remove_dir_all(root);
    }

    fn test_root(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "radroots_sdk_xtask_check_{name}_{}_{}",
            std::process::id(),
            stamp
        ));
        let _ = fs::remove_dir_all(&root);
        root
    }
}
