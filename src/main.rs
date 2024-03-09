use std::path::Path;
use std::env;
use std::process::ExitCode;
use brr::applets::false_applet;
use brr::applets::true_applet;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let p = Path::new(&args[0]).file_name();
    let s = p.expect("base name can't be None").to_str().expect("really?");
    match s {
        "false" => return false_applet::false_main(),
        "true" => return true_applet::true_main(),
        _ => return ExitCode::FAILURE
    }
}
