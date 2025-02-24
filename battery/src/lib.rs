use std::process::Command;

use regex::Regex;

pub fn get_shell_output(cmd: &str) -> Result<String, String> {
    if !cfg!(target_os = "windows") {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd.to_string())
            .output()
            .expect("failed to execute process");
        match std::str::from_utf8(&output.stdout) {
            Ok(output_string) => Ok(output_string
                .lines()
                .take(1)
                .collect::<String>()
                .to_string()),
            Err(_) => Err("cannot parse stdout to string".to_string()),
        }
    } else {
        Err("windows not supported".to_string())
    }
}

pub fn get_correct_percentage(acpi_output: &str) -> Result<u8, ()> {
    let re = Regex::new(r"[^0-9]([0-9]|[1-9][0-9]|[1-9][0-9]{2})%").unwrap();
    for matched in re.find_iter(acpi_output) {
        let match_str = matched.as_str();
        let match_str = &match_str[1..(match_str.len() - 1)];
        let perc = match_str.parse::<u8>().unwrap();
        if perc != 0 {
            return Ok(perc);
        }
    }
    Err(())
}

pub fn get_charging_status(acpi_output: &str) -> bool {
    let re = Regex::new(r"[dD]ischarging").unwrap();
    for _ in re.find_iter(acpi_output) {
        return false;
    }
    true
}
