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