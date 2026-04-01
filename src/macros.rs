//! Utility macros for the WebAssembly environment.

/// Logs a formatted message to the browser's JavaScript console.
///
/// Accepts the same format string syntax as [`format!`] / [`println!`].
/// Internally calls `web_sys::console::log_1` with the formatted string.
///
/// # Examples
/// ```ignore
/// console_log!("Hello, world!");
/// console_log!("Value: {}", 42);
/// console_log!("Multiple: {} and {}", "hello", "goodbye");
/// ```
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}
