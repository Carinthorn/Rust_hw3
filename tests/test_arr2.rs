use std::fs;

use assert_cmd::Command; 

type Outcome = Result<(), Box<dyn std::error::Error>>;
////neg, blank, str, 0, 4
#[test]
fn arr2_blank_input(){
    let mut cmd = Command::cargo_bin("arrow2").unwrap();
    cmd.assert().failure().stdout("Enter a numeric value");
}


#[test]
fn arr2_str_input(){
    let mut cmd = Command::cargo_bin("arrow2").unwrap();
    cmd.arg("str").assert().failure().stdout("Enter a numeric value");
    
}

#[test]
fn arr2_neg_input(){
    run(&"-3", "tests/expected/arr2_input.txt").expect("Test failed");
}


#[test]
fn arr2_zero_input(){
    let mut cmd =  Command::cargo_bin("arrow2").unwrap();
    cmd.arg("0").assert().success().stdout(""); 
}

#[test]
fn arr2_input(){
    run(&"3", "tests/expected/arr2_input.txt").expect("Test failed");
}


fn run(arg: &str, file: &str) -> Outcome {
    let expected = fs::read_to_string(file)?;
    Command::cargo_bin("arrow2")?.arg(arg).assert().success().stdout(expected);
    Ok(())
}