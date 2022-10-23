use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("motivator")?;
    cmd.arg("-f").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid JSON file."));

    Ok(())
}

#[test]
fn tag_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("motivator")?;
    cmd.arg("-f").arg("quotes.json")
        .arg("-t").arg("");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Selected Tag returned no matching quotes."));

    Ok(())
}

#[test]
fn check_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("temp.json")?;
    file.write_str("")?;

    let mut cmd = Command::cargo_bin("motivator")?;
    cmd.arg("-f").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid JSON file."));

    Ok(())
}

#[test]
fn check_json_format() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("temp.json")?;
    // missing ending " for 'quotes'
    file.write_str(r#"{
                  "quotes: [
                    {
                      "quote": "Mindset is everything",
                      "author": "",
                      "tags": ["Motivational", "Positivity"]
                    }
                  ]
                }
                "#)?;

    let mut cmd = Command::cargo_bin("motivator")?;
    cmd.arg("-f").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid JSON file."));

    Ok(())
}
