use assert_cmd::Command;

// cargo test --test test_score

#[test]
fn score_blank_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.assert().failure().stdout("Enter a numeric score value");
}

#[test]
fn score_str_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("fjdaklf").assert().failure().stdout("Enter a numeric score value");
}

#[test]
fn score_more_than_100_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("130").assert().success().stdout("Invalid score");
}

#[test]
fn score_less_than_0_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("-5").assert().success().stdout("Invalid score");
}

#[test]
fn score_0_49_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("44").assert().success().stdout("Failed with F");
}

#[test]
fn score_50_60_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("55").assert().success().stdout("D");
}

#[test]
fn score_61_70_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("66").assert().success().stdout("C");
}

#[test]
fn score_71_80_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("77").assert().success().stdout("B");
}

#[test]
fn score_81_94_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("88").assert().success().stdout("A");
}

#[test]
fn score_95_100_input(){
    let mut cmd = Command::cargo_bin("score").unwrap();
    cmd.arg("99").assert().success().stdout("Excellent with A+");
}

