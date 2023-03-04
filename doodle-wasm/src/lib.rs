// use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
// use std::ops::Deref;
use js_sys::Array;
// use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlElement;
mod default_drawing;
const CANVAS_WIDTH_RATIO: f32 = 1.0;
const CANVAS_WIDTH_HEIGHT_RATIO: f32 = 0.75;
const DEFAULT_DRAWING: &str = default_drawing::DEFAULT_DRAWING;
#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Drawing {
    points: Rc<RefCell<Vec<Vec<[i32; 2]>>>>,
}
#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
#[wasm_bindgen]
pub fn create_canvas_drawing(class: String) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let drawings_list = document.get_elements_by_class_name(&class);
    // let drawings_list = document.get_elements_by_class_name("wasm-canvas-div");
    let mut item_dimension = None;
    for list in Array::from(&drawings_list).iter() {
        let list_elem = list.dyn_into::<HtmlElement>().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_attribute("class", "wasm-added-canvas").unwrap();
        match item_dimension {
            Some((width, height)) => {
                set_canvas_dimension(&canvas, width, height);
            }
            None => {
                let width = list_elem.offset_width() as u32;
                let height = ((width as f32) * CANVAS_WIDTH_HEIGHT_RATIO) as u32;
                item_dimension = Some((width, height));
                set_canvas_dimension(&canvas, width, height)
            }
        }
        let drawing_data = list_elem.dataset();
        let points_vec =
            serde_json::from_str::<Vec<Vec<[i32; 2]>>>(&drawing_data.get("points").unwrap())
                .unwrap_or(serde_json::from_str::<Vec<Vec<[i32; 2]>>>(DEFAULT_DRAWING).unwrap());
        let originnal_canvas_width = drawing_data
            .get("width")
            .unwrap()
            .parse::<f64>()
            .unwrap_or(482.0);

        list_elem
            .dyn_into::<Element>()
            .unwrap()
            .append_child(&canvas)?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let ratio = item_dimension.unwrap().0 as f64 / originnal_canvas_width;
        for points in points_vec.iter() {
            context.begin_path();
            let mut points_iter = points.iter();
            if let Some(first_point) = points_iter.next() {
                let first_point = get_point_transormed(first_point, ratio);
                context.move_to(first_point.0, first_point.1);
                for point in points_iter {
                    let point = get_point_transormed(point, ratio);
                    context.line_to(point.0, point.1);
                    context.stroke();
                    context.begin_path();
                    context.move_to(point.0, point.1);
                }
            }
        }

        // let width =
    }
    Ok(())
}
fn get_point_transormed(array: &[i32; 2], ratio: f64) -> (f64, f64) {
    (array[0] as f64 * ratio, array[1] as f64 * ratio)
}
fn set_canvas_dimension(canvas: &HtmlCanvasElement, width: u32, height: u32) {
    canvas.set_width(width);
    canvas.set_height(height);
}
#[wasm_bindgen]
pub fn create_canvas_form() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_div = document.get_element_by_id("canvas-form-div").unwrap();
    let drawing = Drawing::default();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_attribute("id", "drawing-canvas").unwrap();
    canvas_div.append_child(&canvas)?;
    let width = canvas_div
        .dyn_into::<web_sys::HtmlElement>()?
        .offset_width() as f32;
    // web_sys::console::log_1(&width.into()) as f32;
    let canvas_width = (width * CANVAS_WIDTH_RATIO) as u32;
    canvas.set_width(canvas_width);
    let canvas_height = (canvas_width as f32 * CANVAS_WIDTH_HEIGHT_RATIO) as u32;
    canvas.set_height(canvas_height);
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    {
        let points_clone = drawing.points.clone();

        let button = document.get_element_by_id("canvas-button").unwrap();
        let input = document
            .get_element_by_id("points")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()?;
        let width_input = document
            .get_element_by_id("canvas-form-width")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            let points =
                serde_json::to_string::<Vec<Vec<[i32; 2]>>>(&points_clone.borrow()).unwrap();
            web_sys::console::log_1(&points.clone().into());
            input.set_value(&points);
            width_input.set_value(&canvas_width.to_string());
            // web_sys::console::log_1(&points.into());
            input.form().unwrap().submit().expect("form");
        });
        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let points_clone = drawing.points.clone();
        let context = context.clone();

        let button = document.get_element_by_id("canvas-clear").unwrap();
        // let input = document
        //     .get_element_by_id("points")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlInputElement>()?;
        let submit_button = document
            .get_element_by_id("canvas-button")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            let canvas = context.canvas().unwrap();
            context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            *points_clone.borrow_mut() = vec![];
            // input.set_value("[]");
            submit_button.set_disabled(true);
        });
        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let points_clone = drawing.points.clone();
        let context = context.clone();
        let pressed = pressed.clone();
        let submit_button = document
            .get_element_by_id("canvas-button")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            // web_sys::console::log_1(1.into());
            // web_sys::console::log_1(&points_clone.into());
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
            points_clone
                .borrow_mut()
                .push(vec![[event.offset_x(), event.offset_y()]]);
            submit_button.set_disabled(false);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let points_clone = drawing.points.clone();
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                points_clone
                    .borrow_mut()
                    .last_mut()
                    .unwrap()
                    .push([event.offset_x(), event.offset_y()]);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
