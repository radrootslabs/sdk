mod check;
mod fs;
mod generate;
mod manifest;
mod package_matrix;
mod ts;

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let args = args.into_iter().collect::<Vec<_>>();
    match args.as_slice() {
        [command, target] if command == "generate" && target == "ts" => generate::generate_ts(),
        [command] if command == "check" => check::check(),
        [] => Err(usage()),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: cargo xtask generate ts | cargo xtask check".to_owned()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn accepts_generate_ts() {
        assert!(run(["generate".to_owned(), "ts".to_owned()]).is_ok());
    }

    #[test]
    fn accepts_check() {
        assert!(run(["check".to_owned()]).is_ok());
    }

    #[test]
    fn rejects_unknown_command() {
        assert!(run(["generate".to_owned(), "swift".to_owned()]).is_err());
    }
}
