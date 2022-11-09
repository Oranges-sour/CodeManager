pub fn run_command(command: &str, args: &Vec<&str>) -> Result<String, String> {
    use std::process::Command;

    let mut binding = Command::new(&command);
    let cmd = binding.args(&args[..]);

    match cmd.output() {
        Err(e) => Err(e.to_string()),
        Ok(t) => {
            if !t.status.success() {
                match String::from_utf8(t.stderr) {
                    Err(e) => return Err(e.to_string()),
                    Ok(t) => return Err(t),
                }
            }

            match String::from_utf8(t.stdout) {
                Err(e) => Err(e.to_string()),
                Ok(t) => Ok(t),
            }
        }
    }
}
