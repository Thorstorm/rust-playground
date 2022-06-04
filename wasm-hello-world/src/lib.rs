extern crate wasm_bindgen;
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{window, HtmlDivElement, HtmlElement, console};

// import 'window.alert'
#[wasm_bindgen]
extern {
    fn alert(s:&str);
}

struct Game {
    pub width: usize,
    pub height: usize
}
impl Game {
    pub fn new() -> Self {
        Self {
            width: 50,
            height: 50
        }
    }
}

fn draw_div_squares() {

    let document = window().unwrap_throw().document().unwrap_throw();

    let root_container = document
      .get_element_by_id("root")
      .unwrap_throw()
      .dyn_into::<HtmlElement>()
      .unwrap_throw();


    // root_container.set_inner_html("");

    let game = Game::new();
    let width = game.width;
    let height = game.height;

    root_container
      .style()
      .set_property("display", "inline-grid")
      .unwrap_throw();
    root_container
      .style()
      .set_property(
        "grid-template",
        &format!("repeat({}, auto) / repeat({}, auto)", height, width),
      )
      .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let _pos = (x, y);
            let field_element = document
            .create_element("div")
            .unwrap_throw()
            .dyn_into::<HtmlDivElement>()
            .unwrap_throw();

            field_element.set_class_name("field");

            // field_element.set_inner_text({
            // if pos == game.food {
            //     "🍎"
            // } else if game.snake.get(0) == Some(&pos) {
            //     "❇️"
            // } else if game.snake.contains(&pos) {
            //     "🟩"
            // } else {
            //     " "
            // }
            // });

            root_container.append_child(&field_element).unwrap_throw();
        }
    }
}

// Export a 'helloworld' function
#[wasm_bindgen]
pub fn helloworld(_name: &str) {
    // alert(&format!("Hello World:{}", name));

    // use std::time::Instant;
    let start = js_sys::Date::now();

    draw_div_squares();

    let finish = js_sys::Date::now();
    let elapsed = finish - start;

    console::log_1(&elapsed.into());
}