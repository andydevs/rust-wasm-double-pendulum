use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Get document
    let document = web_sys::window()
        .ok_or(JsValue::from_str("Could not find window object!"))?
        .document()
        .ok_or(JsValue::from_str("Could not find webpage document!"))?;

    // Simple arithmetic demo
    let a = 8;
    let b = 10;
    let c = a + b;

    // Create root element
    let root = document.create_element("div")?;
    root.set_attribute("style", "font-family:Arial,sans-serif;padding:24px;")?;

    // Create html substring
    let mut subhtml = String::new();
    subhtml.push_str("<h1>Rust WASM Double Pendulum</h1>\n");
    subhtml
        .push_str("<p>Webpack + HtmlWebpackPlugin demo. Replace this with your app entry.</p>\n");
    subhtml.push_str(&format!("<p>{} + {} = {}</p>", a, b, c));

    // Set substring in root
    root.set_inner_html(&subhtml);

    // Add root to document body
    document
        .body()
        .ok_or(JsValue::from("Could not find document body!"))?
        .append_child(&root)?;

    Ok(())
}
