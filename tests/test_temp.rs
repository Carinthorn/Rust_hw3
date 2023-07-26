use assert_cmd::Command;
use std::fs;

type Outcome = Result<(), Box<dyn std::error::Error>>;

#[test]
fn temp_blank_input(){
    let mut cmd = Command::cargo_bin("temp").unwrap();
    cmd.assert().success().stdout("Input: <start_f> <end_f> <delta>");
}

#[test]
fn temp_str_input(){
    let mut cmd = Command::cargo_bin("temp").unwrap();
    cmd.arg("str").arg("0").arg("20").assert().failure().stdout("Enter a numeric value for f_start");
    cmd.arg("300").arg("str").arg("20").assert().failure().stdout("Enter a numeric value for f_start");
    cmd.arg("300").arg("0").arg("str").assert().failure().stdout("Enter a numeric value for f_start");
}


#[test] 
fn temp_start_less_than_end(){
    run(&["0", "10", "2"], "tests/expected/temp_start_less_than_end.txt").expect("Test failed");
} 


#[test] 
fn temp_start_more_than_end(){
    run(&["10", "0", "2"], "tests/expected/temp_start_more_than_end.txt").expect("Test failed");
} 


#[test] 
fn temp_indivisible_delta(){
    run(&["0", "10", "3"], "tests/expected/temp_indivisible_delta.txt").expect("Test failed");
}

#[test] 
fn temp_negative_delta(){
    run(&["0", "10", "-3"], "tests/expected/temp_negative_delta.txt").expect("Test failed");
}

fn run(args: &[&str], file: &str) -> Outcome {
    let expected = fs::read_to_string(file)?;
    Command::cargo_bin("temp")?.args(args).assert().success().stdout(expected);
    Ok(())
}