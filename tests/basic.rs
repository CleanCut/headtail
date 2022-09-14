use std::process::Command;

#[test]
fn help() {
    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("-h")
        .output()
    {
        Ok(output) => {
            // debug_output(&output);
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("USAGE"));
            assert!(stdout.contains("headtail"));
            assert!(stdout.contains("filename"));
            assert!(stdout.contains("-H"));
            assert!(stdout.contains("-T"));
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn argless() {
    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .output()
    {
        Ok(output) => {
            // debug_output(&output);
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("one\n"));
            assert!(stdout.contains("ten\n"));
            assert!(!stdout.contains("eleven\n"));
            assert!(!stdout.contains("twenty\n"));
            assert!(stdout.contains("thirty\n"));
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn head() {
    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .arg("-H")
        .arg("3")
        .arg("-T")
        .arg("0")
        .output()
    {
        Ok(output) => {
            // debug_output(&output);
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("one\n"));
            assert!(stdout.contains("two\n"));
            assert!(stdout.contains("three\n"));
            assert!(!stdout.contains("four\n"));
            assert!(!stdout.contains("twenty\n"));
            assert!(!stdout.contains("thirty\n"));
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn tail() {
    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .arg("-H")
        .arg("0")
        .arg("-T")
        .arg("3")
        .output()
    {
        Ok(output) => {
            // debug_output(&output);
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("twenty-eight\n"));
            assert!(stdout.contains("twenty-nine\n"));
            assert!(stdout.contains("thirty\n"));
            assert!(!stdout.contains("twenty-seven\n"));
            assert!(!stdout.contains("one\n"));
            assert!(!stdout.contains("ten\n"));
        }
        Err(e) => println!("Error: {}", e),
    }
}

// TODO: Add test for -f/--follow
