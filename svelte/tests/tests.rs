extern crate diff;

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

fn slurp<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

macro_rules! test {
    ( $name:ident $( , $args:expr )* ) => {
        #[test]
        fn $name() {
            let output = Command::new("cargo")
                .arg("run")
                .arg("--")
                $(
                    .arg($args)
                )*
                .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests"))
                .output()
                .unwrap();

            assert!(
                output.status.success(),
                "should have run `svelete` OK\n\n\
                 ============================== stdout ==============================\n\n\
                 {}\n\n\
                 ============================== stderr ==============================\n\n\
                 {}\n\n",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );

            let expected_path = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/expectations/",
                stringify!($name)
            );

            // Ignore errors. The diffing will provide a better diagnostic report.
            let expected = slurp(expected_path).unwrap_or(vec![]);

            if output.stdout != expected {
                let mut cmd = "svelte".to_string();
                $(
                    cmd.push(' ');
                    cmd.push_str($args);
                )*
                println!("\n`{}` did not have the expected output!\n", cmd);

                println!("--- {}", expected_path);
                println!("+++ actually generated by `{}`", cmd);
                let expected = String::from_utf8_lossy(&expected);
                let actual = String::from_utf8_lossy(&output.stdout);
                for diff in diff::lines(&expected, &actual) {
                    match diff {
                        diff::Result::Left(l) => println!("-{}", l),
                        diff::Result::Both(l, _) => println!(" {}", l),
                        diff::Result::Right(r) => println!("+{}", r),
                    }
                }
                panic!();
            }
        }
    }
}

test!(
    top_wee_alloc,
    "top",
    "-n",
    "10",
    "./fixtures/wee_alloc.wasm"
);
test!(top_mappings, "top", "-n", "10", "./fixtures/mappings.wasm");

test!(
    top_retained_wee_alloc,
    "top",
    "-n",
    "10",
    "-s",
    "retained",
    "./fixtures/wee_alloc.wasm"
);
test!(
    top_retained_mappings,
    "top",
    "-n",
    "10",
    "-s",
    "retained",
    "./fixtures/mappings.wasm"
);

test!(
    dominators_wee_alloc,
    "dominators",
    "./fixtures/wee_alloc.wasm"
);

test!(
    paths_wee_alloc,
    "paths",
    "./fixtures/wee_alloc.wasm",
    "wee_alloc::alloc_first_fit::h9a72de3af77ef93f",
    "hello",
    "goodbye"
);
