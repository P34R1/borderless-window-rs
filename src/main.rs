const MESSAGE: &str = "Which program would you like to borderlessly fullscreen?";

fn main() -> Result<(), inquire::InquireError> {
    let all_windows = borderless_window_rs::read_all_windows();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let target_program = args.join(" ");

    let target_program = if target_program.is_empty() {
        inquire::Select::new(MESSAGE, all_windows.keys().collect::<Vec<_>>())
            .prompt()?
            .to_string()
    } else {
        target_program
    };

    if let Some(&hwnd) = all_windows.get(&target_program) {
        unsafe { borderless_window_rs::set_borderless_fullscreen(hwnd) }
    }

    Ok(())
}
