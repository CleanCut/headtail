use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use tempfile::NamedTempFile;

fn number_of_input_lines() -> usize {
    let f = BufReader::new(File::open("tests/files/input.txt").unwrap());
    f.lines().count()
}

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
            assert!(stdout.contains("FILENAME"));
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

#[test]
fn head_length_exceeds_file_length() {
    let expected_lines = number_of_input_lines();
    let head_lines = 64;
    assert!(expected_lines < head_lines, "Expected input file to have fewer than {} lines (otherwise this test doesn't test anything)", head_lines);

    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .arg("-H")
        .arg(head_lines.to_string())
        .arg("-T")
        .arg("0")
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let num_lines = stdout.lines().count();
            assert_eq!(num_lines, expected_lines);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn tail_length_exceeds_file_length() {
    let expected_lines = number_of_input_lines();
    let tail_lines = 64;
    assert!(expected_lines < tail_lines, "Expected input file to have fewer than {} lines (otherwise this test doesn't test anything)", tail_lines);

    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .arg("-H")
        .arg("0")
        .arg("-T")
        .arg(tail_lines.to_string())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let num_lines = stdout.lines().count();
            assert_eq!(num_lines, expected_lines);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn overlapping_head_and_tail() {
    let expected_lines = number_of_input_lines();
    let head_lines = 20;
    let tail_lines = 20;
    assert!(expected_lines < head_lines + tail_lines, "Expected input file to have fewer than {} lines (otherwise this test doesn't test anything)", head_lines + tail_lines);

    match Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg("tests/files/input.txt")
        .arg("-H")
        .arg(head_lines.to_string())
        .arg("-T")
        .arg(tail_lines.to_string())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let num_lines = stdout.lines().count();
            assert_eq!(num_lines, expected_lines);
        }
        Err(e) => println!("Error: {}", e),
    }
}

// TODO: Add test for -f/--follow

#[test]
fn follow_detects_recreation() -> Result<()> {
    let wait_duration = Duration::from_millis(100); // 4 times higher than minimum required for my machine - cleancut
    let first_file_contents = "first file\n";
    let second_file_contents = "second file\n";

    // create a temporary file
    let just_for_name_file = NamedTempFile::new()?;
    let tmpfilename = just_for_name_file.path().to_owned();
    drop(just_for_name_file);
    // give filesystem time to really delete the file
    std::thread::sleep(wait_duration);

    let mut tmpfile = File::create(&tmpfilename)?;
    write!(tmpfile, "{}", first_file_contents)?;
    tmpfile.flush();
    drop(tmpfile);

    // give filesystem time to write file contents and close file
    std::thread::sleep(wait_duration);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_headtail"))
        .arg(&tmpfilename)
        .arg("--follow")
        .stdout(Stdio::piped())
        .spawn()?;

    // Give headtail sufficient time to open the file and read it
    std::thread::sleep(wait_duration);

    // give filesystem time to really delete the file
    std::fs::remove_file(&tmpfilename)?;

    std::thread::sleep(wait_duration);

    let mut newfile = File::create(&tmpfilename)?;
    write!(newfile, "{}", second_file_contents)?;
    newfile.flush();
    drop(newfile);

    // give filesystem time to write file contents and close file
    std::thread::sleep(wait_duration);

    cmd.kill()?;

    match cmd.wait_with_output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let mut combined = first_file_contents.to_owned();
            combined.push_str(second_file_contents);
            assert_eq!(combined, stdout);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
