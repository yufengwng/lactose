use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

use tfproc::test_suite;

const TF_EXE_PATH: &str = "./target/debug/tofu";

test_suite!(suite, "./tests/suite");

fn test_script<P>(path: P)
where P: AsRef<Path> {
    let expect = parse(&path);
    let actual = run(&path);
    assert_eq!(expect, actual);
}

fn parse<P>(path: P) -> String
where P: AsRef<Path> {
    let file = File::open(path).expect("cannot open file");
    let reader = io::BufReader::new(file);
    let mut expected = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(idx) = line.find("#=> ") {
            let output = line.split_at(idx + 4).1;
            expected.push_str(output);
            expected.push('\n');
        }
    }
    expected
}

fn run<P>(path: P) -> String
where P: AsRef<Path> {
    let output = Command::new(TF_EXE_PATH)
        .arg(path.as_ref())
        .output()
        .expect("failed to run script");
    String::from_utf8_lossy(&output.stdout).into_owned()
}
