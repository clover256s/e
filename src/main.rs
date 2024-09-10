use ediotr::Editor;

mod ediotr;
mod terminal;
mod text_buffer;
mod text_view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Editor::new()?.run()?;
    Ok(())
}
