use std::{env, fs, path::Path, process::Command};

use crate::{
    fs::workspace_root,
    package_matrix::{WasmPackageSpec, validate_package_matrix, wasm_package_specs},
};

pub fn generate(args: &[String]) -> Result<(), String> {
    validate_package_matrix()?;
    let specs = selected_specs(args)?;
    let root = workspace_root()?;
    for spec in specs {
        let dist_dir = root.join(spec.package_dir).join("dist");
        if dist_dir.exists() {
            fs::remove_dir_all(&dist_dir)
                .map_err(|error| format!("failed to remove {}: {error}", dist_dir.display()))?;
        }
        let mut command = Command::new("wasm-pack");
        command
            .current_dir(&root)
            .arg("build")
            .arg(spec.crate_dir)
            .arg("--release")
            .arg("--target")
            .arg("web")
            .arg("--out-dir")
            .arg(spec.out_dir)
            .arg("--out-name")
            .arg(spec.out_name)
            .arg("--no-pack");
        if let Some(rustc) = rustup_tool("rustc") {
            if let Some(parent) = Path::new(&rustc).parent() {
                prepend_path(&mut command, parent);
            }
            command.env("RUSTC", rustc);
        }
        if let Some(cargo) = rustup_tool("cargo") {
            command.env("CARGO", cargo);
        }
        let status = command
            .status()
            .map_err(|error| format!("failed to start wasm-pack for {}: {error}", spec.key))?;
        if !status.success() {
            return Err(format!("wasm-pack failed for {}", spec.key));
        }
        println!("generated wasm package {}", spec.package_name);
    }
    Ok(())
}

fn selected_specs(args: &[String]) -> Result<Vec<WasmPackageSpec>, String> {
    match args {
        [] => Ok(wasm_package_specs().to_vec()),
        [flag, key] if flag == "--package" => wasm_package_specs()
            .iter()
            .copied()
            .find(|spec| spec.key == key)
            .map(|spec| vec![spec])
            .ok_or_else(|| format!("unknown wasm package: {key}")),
        _ => Err("usage: cargo xtask generate wasm [--package <key>]".to_owned()),
    }
}

fn rustup_tool(name: &str) -> Option<String> {
    let output = Command::new("rustup")
        .arg("which")
        .arg(name)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8(output.stdout).ok()?;
    let trimmed = path.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

fn prepend_path(command: &mut Command, prefix: &Path) {
    let existing = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![prefix.to_path_buf()];
    paths.extend(env::split_paths(&existing));
    if let Ok(joined) = env::join_paths(paths) {
        command.env("PATH", joined);
    }
}

#[cfg(test)]
mod tests {
    use super::{rustup_tool, selected_specs};

    #[test]
    fn selects_all_specs_by_default() {
        assert_eq!(selected_specs(&[]).expect("all specs").len(), 3);
    }

    #[test]
    fn selects_one_spec_by_key() {
        let specs = selected_specs(&["--package".to_owned(), "replica_db".to_owned()])
            .expect("replica db spec");
        assert_eq!(specs[0].package_name, "@radroots/replica-db-wasm");
    }

    #[test]
    fn rejects_unknown_spec_key() {
        assert!(selected_specs(&["--package".to_owned(), "missing".to_owned()]).is_err());
    }

    #[test]
    fn rustup_tool_resolution_is_non_panicking() {
        let _ = rustup_tool("rustc");
    }
}
