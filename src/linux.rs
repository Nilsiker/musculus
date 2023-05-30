#[cfg(target_os = "linux")]
pub fn get_double_click_value_linux() -> Result<u32, &'static str> {
    use x11::xlib;

    match std::env::var_os("WAYLAND_DISPLAY") {
        Some(_) => {
            Ok(0)
        }
        None => {
            let display_name = std::env::var("DISPLAY").unwrap_or_else(|_| "".to_string());
            let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr() as *const i8) };

            if display.is_null() {
                return Err("Failed to open X11 display");
            }

            let mut accel_numerator_return = 0;
            let mut accel_denominator_return = 0;
            let mut threshold_return = 0;

            unsafe {
                xlib::XGetPointerControl(
                    display,
                    &mut accel_numerator_return,
                    &mut accel_denominator_return,
                    &mut threshold_return,
                );
            }

            println!("Double click threshold: {} ms", threshold_return);

            unsafe { xlib::XCloseDisplay(display) };

            Ok(threshold_return as u32)
        }
    }
}
