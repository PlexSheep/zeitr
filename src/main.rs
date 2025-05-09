use std::process::ExitCode;

mod cli;
#[cfg(feature = "gui")]
mod gui;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if gui_requested(&args) {
        #[cfg(feature = "gui")]
        return gui::gui(&args);
        #[cfg(not(feature = "gui"))]
        unimplemented!(
            "This version of {} was compiled without GUI support",
            env!("CARGO_PKG_NAME")
        )
    }

    cli::cli(&args)
}

fn gui_requested(args: &[String]) -> bool {
    if args.len() > 1 {
        for arg in args {
            if arg == "--gui" {
                return true;
            }
            if arg.starts_with("-") && arg.chars().nth(1) != Some('-') && arg.contains('g') {
                return true;
            }
        }
    }
    false
}
