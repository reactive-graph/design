use rs_web_component::Component;
use std::cell::{Cell, RefCell};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use web_sys::ShadowRoot;
use web_sys::ShadowRootInit;
use web_sys::ShadowRootMode;

use crate::base::CustomElementCreator;
use crate::base::RootVal;
use crate::base::ThisVal;

pub struct RgTag {
    root: RootVal,
    this: ThisVal,
    ty: RefCell<String>,
    text: RefCell<String>,
}

impl CustomElementCreator for RgTag {
    fn tag_name() -> String {
        "rg-tag".to_string()
    }

    fn create_component() -> Box<dyn Component> {
        Box::new(Self {
            root: RootVal::None,
            this: ThisVal::None,
            ty: RefCell::new(String::new()),
            text: RefCell::new(String::new()),
        })
    }
}

impl Component for RgTag {
    fn init(&mut self, this: HtmlElement) {
        self.this = ThisVal::Value(this);
    }

    fn observed_attributes(&self) -> Vec<String> {
        return vec!["ty".to_string()];
    }

    fn attribute_changed_callback(&self, _name: String, _old_value: JsValue, _new_value: wasm_bindgen::JsValue) {
        web_sys::console::log_1(&format!("{_name}").into());
        web_sys::console::log_1(&"attribute_changed_callback 1".into());
        web_sys::console::log_1((&_old_value).into());
        web_sys::console::log_1((&_new_value).into());
      // if _old_value.is_undefined() {
      //   // self.render();
      //   return;
      // }
      //   if _old_value != _new_value {
      //       match _name.as_str() {
      //           "ty" => {
      //             let ty = self.get_this().get_attribute("ty").unwrap_or(String::new());
      //               *self.ty.borrow_mut() = ty;
      //           }
      //           _ => {
      //               return;
      //           }
      //       }
      //       self.render();
      //       // web_sys::console::log_1(&"attribute_changed_callback 2".into());
      //       // let x = self.render();
      //       // web_sys::console::log_1(&"attribute_changed_callback 3".into());
      //       //   self.get_root().set_inner_html(x.as_str());
      //       // web_sys::console::log_1(&"attribute_changed_callback 4".into());
      //   }
      //   web_sys::console::log_1(&"attribute_changed_callback 5".into());
    }

    fn connected_callback(&mut self) {
        self.root = RootVal::Value(self.get_this().attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open)).unwrap());
        self.ty = RefCell::new(self.get_this().get_attribute("ty").unwrap_or(String::new()));
      web_sys::console::log_1(&format!("ty: {}", &self.ty.borrow()).into());
        self.text = RefCell::new(self.get_root().inner_html());
      web_sys::console::log_1(&format!("text: {}", &self.text.borrow()).into());
        // let text = ;
        // self.text = if text.is_empty() { Cell::new(None) } else { Cell::new(Some(text)) };
        // self.get_root().set_inner_html(self.render().as_str())
        self.render();
    }

    fn disconnected_callback(&self) {}
}

impl RgTag {
    fn render(&self) {
        web_sys::console::log_1(&"render1".into());
      let ty = self.ty.borrow().clone(); // clone().unwrap_or("".to_string());
      let text = self.text.borrow().clone(); // .unwrap_or("".to_string());

        let inner_html = format!(
            "<span class=\"tag {ty}\">{text}</span>",
        )
        .to_string();
        web_sys::console::log_1(&"render2".into());
        self.get_root().set_inner_html(inner_html.as_str());
      web_sys::console::log_1(&"rendered".into());
    }

    fn get_root(&self) -> &ShadowRoot {
        return match &self.root {
            RootVal::Value(root) => &root,
            RootVal::None => panic!("not a root!"),
        };
    }

    fn get_this(&self) -> &HtmlElement {
        match &self.this {
            ThisVal::Value(val) => val,
            ThisVal::None => panic!("not an HtmlElement"),
        }
    }
}
