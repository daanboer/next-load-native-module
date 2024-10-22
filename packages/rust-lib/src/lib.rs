#[macro_use]
extern crate napi_derive;

#[napi]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
