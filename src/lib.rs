#[cfg(target_os = "windows")]
use winapi::um::winuser::GetDoubleClickTime;

pub fn get_mouse_double_click_time() -> u32 {
    #[cfg(target_os = "windows")]
    return get_double_click_value_windows();

    #[cfg(target_os = "macos")]
    return get_double_click_value_macos();

    #[cfg(target_os = "linux")]
    return get_double_click_value_linux();
}

#[cfg(target_os = "windows")]
fn get_double_click_value_windows() -> u32 {
    unsafe { GetDoubleClickTime() }
}

#[cfg(target_os = "macos")]
fn get_double_click_value_macos() -> u32 {
    let accessibility_domain =
        unsafe { CFStringGetCStringPtr(kCFAllocatorDefault, kCFPreferencesAnyApplication) };

    if accessibility_domain.is_null() {
        return Err("Failed to get accessibility domain");
    }

    let key = CFString::from_static_string("com.apple.mouse.doubleClickThreshold");

    let double_click_speed_ref = unsafe {
        CFDictionaryGetValue(
            kCFPreferencesCurrentApplication,
            key.as_concrete_TypeRef() as CFDictionaryRef,
        )
    };

    if double_click_speed_ref.is_null() {
        return Err("Failed to get double click speed");
    }

    let double_click_speed = unsafe {
        let mut speed: CFNumberRef = ptr::null_mut();
        CFNumberGetValue(
            double_click_speed_ref as CFNumberRef,
            CFNumberType_kCFNumberDoubleType,
            &mut speed,
        );

        speed
    };

    let double_click_speed_value = unsafe {
        let mut speed: f64 = 0.0;
        CFNumberGetValue(
            double_click_speed as CFNumberRef,
            CFNumberType_kCFNumberDoubleType,
            &mut speed,
        );

        speed
    };

    Ok(double_click_speed_value)
}

#[cfg(target_os = "linux")]
fn get_double_click_value_linux() -> u32 {
    fn main() {
        match env::var_os("WAYLAND_DISPLAY") {
            Some(_) => {
                println!("WAYLAND!");
                todo!()
            }
            None => {
                let display_name = std::env::var("DISPLAY").unwrap_or_else(|_| "".to_string());
                let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr() as *const i8) };

                if display.is_null() {
                    println!("Failed to open X11 display");
                    return;
                }

                let mut control = xlib::XPointerControl {
                    do_not_propagate_mask: 0,
                    throttle_threshold: 0,
                    acceleration_numerator: 0,
                    acceleration_denominator: 0,
                };

                unsafe { xlib::XGetPointerControl(display, &mut control) };

                let double_click_time_ms = control.throttle_threshold;
                println!("Double click threshold: {} ms", double_click_time_ms);

                unsafe { xlib::XCloseDisplay(display) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_mouse_double_click() {
        get_mouse_double_click_time();
    }
}
