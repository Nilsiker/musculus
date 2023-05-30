#[cfg(target_os = "windows")]
use winapi::um::winuser::GetDoubleClickTime;

#[cfg(target_os = "windows")]
fn get_double_click_value_windows() -> u32 {
    unsafe { GetDoubleClickTime() }
}