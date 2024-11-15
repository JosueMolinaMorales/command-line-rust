use clap::{Arg, Command};
fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Josue Molina Morales")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();
    let mut text = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    if !matches.contains_id("omit_newline") {
        text.push('\n')
    }
    println!("{}", text)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn dies_no_args() {
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
    }

    #[test]
    fn runs() {
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.arg("hello").assert().success();
    }
}
