use druid::*
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};

fn build_ui() -> impl Widget<()> {
    let mut col = Flex::column();

    col.add_child(LabelText::new("Hello, Rust World!").center(), 1.0);
    col.add_child(Button::new("Click me!", |_ctx, _data, _env| {
        println!("Button clicked!");
    }).center(), 1.0);

    col
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("A Simple Rust Desktop App")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())?
;    Ok(())
}