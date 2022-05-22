use glob::glob;
// use log::{error, info};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::from_utf8;

// Compares two test outputs line by line, ignoring trailing whitespace.
fn compare_lenient(left: &str, right: &str) -> bool {
    left.lines()
        .map(|s| s.trim())
        .zip(right.lines().map(|s| s.trim()))
        .all(|(l, r)| l == r)
}

fn test_single_case(
    exec_path: &PathBuf,
    input_path: &PathBuf,
    exp_output_path: &PathBuf,
) -> std::io::Result<bool> {
    println!("Testing case {}:", input_path.to_str().unwrap());
    let output = Command::new(exec_path)
        .stdin(Stdio::from(File::open(input_path)?))
        .output()?;
    let output_str = from_utf8(output.stdout.as_slice()).unwrap();
    let mut exp_str = String::new();
    match File::open(exp_output_path) {
        Ok(exp_file) => {
            let mut buf_reader = BufReader::new(exp_file);
            buf_reader.read_to_string(&mut exp_str)?;
        }
        Err(err) => {
            eprintln!("Can't open {}: {}", exp_output_path.to_str().unwrap(), err);
            return Err(err);
        }
    }
    let res = compare_lenient(output_str, &exp_str);
    if res {
        println!("PASSED");
    } else {
        println!("FAILED");
        println!("Input:");
        let mut buf_reader = BufReader::new(File::open(input_path)?);
        let mut inp_str = String::new();
        buf_reader.read_to_string(&mut inp_str)?;
        println!("{}", inp_str);
        println!("Output:");
        println!("{}", output_str);
        println!("Expected:");
        println!("{}", exp_str);
    }
    Ok(res)
}

pub fn test_run(exec_path: PathBuf, short_name: &str) -> std::io::Result<()> {
    let mut passed = 0;
    let mut total = 0;
    let mut errors = 0;
    for entry in glob(&format!("{}-*.in", short_name)).unwrap() {
        match entry {
            Ok(input_path) => {
                total += 1;
                let mut output_path = input_path.clone();
                output_path.set_extension("out");
                match test_single_case(&exec_path, &input_path, &output_path) {
                    Ok(good) => {
                        if good {
                            passed += 1;
                        }
                    }
                    Err(err) => {
                        errors += 1;
                        eprintln!("Error processing case: {}", err);
                    }
                }
            }
            Err(err) => {
                return Err(err.into_error());
            }
        }
    }
    println!("Passed {}/{} cases.", passed, total);
    if errors > 0 {
        println!("Error processing {} cases.", errors);
    }
    if passed == total && total > 0 && errors == 0 {
        println!("Congratulations! All tests passed.");
    }
    Ok(())
}
