use wasm_bindgen::prelude::wasm_bindgen;

use crate::atoms::tag::RgTag;
use crate::base::CustomElementCreator;

pub mod atoms;
pub mod molecules;
pub mod organisms;

pub mod base;

#[wasm_bindgen(start)]
fn run() {
  RgTag::define_element();
}
