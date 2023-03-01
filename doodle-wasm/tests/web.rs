// extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
const CANVAS_FORM: &'static str = include_str!("canvas_form.html");
const DRAWING_LIST: &'static str = include_str!("drawings_list.html");
#[wasm_bindgen_test]
fn canvas_form_load_canvas() {
    doodle_canvas::set_panic_hook();
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let canvas_div = document.create_element("div").unwrap();
    canvas_div.set_inner_html(CANVAS_FORM);
    body.append_child(&canvas_div).unwrap();
    let canvas = document.get_element_by_id("drawing-canvas");
    assert!(canvas.is_none());
    doodle_canvas::create_canvas_form().unwrap();
    let canvas = document.get_element_by_id("drawing-canvas");
    assert!(canvas.is_some());
}
#[wasm_bindgen_test]
fn doodle_canvas_drawn() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let canvas_div = document.create_element("div").unwrap();
    canvas_div.set_inner_html(DRAWING_LIST);
    body.append_child(&canvas_div).unwrap();
    let class = "drawing-list-item".to_string();
    doodle_canvas::create_canvas_drawing(class.clone()).unwrap();
    let drawings_list = document.get_elements_by_class_name(&class);
    assert_eq!(drawings_list.length(), 1);
    let item = drawings_list.get_with_index(0).unwrap();
    let canvas = item.get_elements_by_class_name("wasm-added-canvas");
    assert_eq!(canvas.length(), 1);
    // let canvas = canvas.get_with_index(0).unwrap();
    let class = "wasm-user-div".to_string();
    doodle_canvas::create_canvas_drawing(class.clone()).unwrap();
    let drawings_list = document.get_elements_by_class_name(&class);
    assert_eq!(drawings_list.length(), 1);
    let item = drawings_list.get_with_index(0).unwrap();
    let canvas = item.get_elements_by_class_name("wasm-added-canvas");
    assert_eq!(canvas.length(), 1);
}
// fn create_button(id: &str, document: &web_sys::Document) -> web_sys::Element {
//     let button = document.create_element("button").unwrap();
//     button.set_id(id);
//     button
// }
