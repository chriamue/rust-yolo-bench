use log::Level;
use rust_yolo_bench::App;

fn main() {
    let _ = console_log::init_with_level(Level::Debug);
    yew::Renderer::<App>::new().render();
}
