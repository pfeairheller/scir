use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scir")?;

    cmd.arg("saidify").arg("--file").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.json")?;
    file.write_str("{
  \"v\": \"KERI10JSON00023c_\",
  \"t\": \"rot\",
  \"d\": \"\",
  \"i\": \"EIL2dvwm6lYAsyKKtzxIEFm51gSfwe3IIZSx8kI8ve7_\",
  \"s\": \"2\",
  \"p\": \"EODgCVSGS9S8ZaOr89HKDP_Zll21C8zbUBjbBU1HjGEk\",
  \"kt\": [
    \"1/2\",
    \"1/2\",
    \"1/2\"
  ],
  \"k\": [
    \"DHqJ2DNmypwMKelWXLgl3V-9pDRcOenM5Wf03O1xx1Ri\",
    \"DEIISiMvtnaPTpMHkoGs4d0JdbwjreW53OUBfMedLUaF\",
    \"DDQFJ_uXcZum_DY6NNTtI5UrTEQo6PRWEANpn6hVtfyQ\"
  ],
  \"nt\": [
    \"1/2\",
    \"1/2\",
    \"1/2\"
  ],
  \"n\": [
    \"EJsp5uWsQOsioYA16kbCZW9HPMr0rEaU4NUvfm6QTYd2\",
    \"EFxT53mK2-1sAnh8VcLEL1HowQp0t84dfIWRaju5Ef61\",
    \"EETqITKVCCpOS6aDPiZFJOSWll2i39xaFQkfAYsG18I_\"
  ],
  \"bt\": \"0\",
  \"br\": [],
  \"ba\": [],
  \"a\": []
}")?;

    let mut cmd = Command::cargo_bin("scir")?;
    cmd.arg("saidify").arg("--file").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ELKSLVpbV9eH3xk2xBqH3fSgOmWTbUoBuE2JsLl0lu2L"));

    Ok(())
}