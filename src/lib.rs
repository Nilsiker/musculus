use linux::get_double_click_value_linux;

mod linux;
mod macos;
mod windows;

pub fn get_mouse_double_click_time() -> Result<u32, &'static str> {
    // TODO use anyhow
    #[cfg(target_os = "windows")]
    return get_double_click_value_windows();

    #[cfg(target_os = "macos")]
    return get_double_click_value_macos();

    #[cfg(target_os = "linux")]
    return get_double_click_value_linux();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_mouse_double_click() {
        let time = get_mouse_double_click_time();
        match time {
            Ok(time) => println!("{time}"),
            Err(msg) => println!("{msg}"),
        }
    }
}
