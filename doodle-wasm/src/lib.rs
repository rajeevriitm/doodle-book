// use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
// use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
const CANVAS_WIDTH: u32 = 500;
const CANVAS_HEIGHT: u32 = 400;
#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Drawing {
    points: Rc<RefCell<Vec<Vec<[i32; 2]>>>>,
}
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let drawing = Drawing::default();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    let width = CANVAS_WIDTH;
    let height = CANVAS_HEIGHT;
    canvas.set_width(width);
    canvas.set_height(height);
    canvas.style().set_property("border", "solid")?;
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
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let points =
                serde_json::to_string::<Vec<Vec<[i32; 2]>>>(&points_clone.borrow()).unwrap();
            web_sys::console::log_1(&points.clone().into());
            input.set_value(&points);
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
        let input = document
            .get_element_by_id("points")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()?;
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let canvas = context.canvas().unwrap();
            context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            *points_clone.borrow_mut() = vec![];
            input.set_value("");
        });
        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let points_clone = drawing.points.clone();
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            // web_sys::console::log_1(1.into());
            // web_sys::console::log_1(&points_clone.into());
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
            points_clone
                .borrow_mut()
                .push(vec![[event.offset_x(), event.offset_y()]]);
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
