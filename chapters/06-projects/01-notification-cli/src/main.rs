fn main() -> std::process::ExitCode {
    match notification_cli::run(std::env::args().skip(1)) {
        Ok(output) => {
            print!("{output}");
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::from(2)
        }
    }
}
