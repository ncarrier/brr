use std::path::Path;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let p = Path::new(&args[0]).file_name();
    let s = p.expect("base name can't be None").to_str().expect("really?");
    match s {
        "false" => return ExitCode::FAILURE,
        "true" => return ExitCode::SUCCESS,
        _ => return ExitCode::FAILURE
    }
}
