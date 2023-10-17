//  タウリで共有するステート
#[derive(Default)]
pub struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
