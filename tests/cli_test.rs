use std::{env, error::Error, path::PathBuf};

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "testmate";

struct Test {
    cwd: PathBuf,
    file: PathBuf,
    test_dir: Option<String>,
}

fn prepare_test(file_name: String, test_dir: Option<String>) -> Result<Test, Box<dyn Error>> {
    let cwd = env::current_dir()?;

    let test = Test {
        cwd: cwd.clone(),
        file: PathBuf::from(format!("{}/src/{}", &cwd.to_str().unwrap(), file_name)),
        test_dir,
    };

    Ok(test)
}

#[test]
fn test1() -> TestResult {
    let test = prepare_test("file1.rs".to_string(), Some("tests/expected".to_string()))?;

    let expected = format!(
        "{}/{}/{}_test.rs",
        &test.cwd.to_string_lossy(),
        &test.test_dir.clone().unwrap().to_string(),
        &test.file.file_stem().unwrap().to_string_lossy()
    );

    Command::cargo_bin(PRG)?
        .write_stdin(test.file.to_str().unwrap())
        .args(&["-d", &test.test_dir.unwrap().to_string()])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn test2() -> TestResult {
    let test = prepare_test("File2.rs".to_string(), Some("tests/expected".to_string()))?;

    let expected = format!(
        "{}/{}/{}.spec.rs",
        &test.cwd.to_string_lossy(),
        &test.test_dir.clone().unwrap().to_string(),
        &test.file.file_stem().unwrap().to_string_lossy()
    );

    Command::cargo_bin(PRG)?
        .write_stdin(test.file.to_str().unwrap())
        .args(&["-d", &test.test_dir.unwrap().to_string()])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn test3() -> TestResult {
    let test = prepare_test("File3.rs".to_string(), Some("tests/expected".to_string()))?;

    let expected = format!(
        "{}/{}/{}_Test.rs",
        &test.cwd.to_string_lossy(),
        &test.test_dir.clone().unwrap().to_string(),
        &test.file.file_stem().unwrap().to_string_lossy()
    );

    Command::cargo_bin(PRG)?
        .write_stdin(test.file.to_str().unwrap())
        .args(&["-d", &test.test_dir.unwrap().to_string()])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn test4() -> TestResult {
    let test = prepare_test("File4.rs".to_string(), Some("tests/expected".to_string()))?;

    let expected = format!(
        "{}/{}/{}Test.rs",
        &test.cwd.to_string_lossy(),
        &test.test_dir.clone().unwrap().to_string(),
        &test.file.file_stem().unwrap().to_string_lossy()
    );

    Command::cargo_bin(PRG)?
        .write_stdin(test.file.to_str().unwrap())
        .args(&["-d", &test.test_dir.unwrap().to_string()])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
