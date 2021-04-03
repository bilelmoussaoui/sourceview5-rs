use gtk::prelude::*;
use sourceview5::prelude::*;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("SourceView5 + Rust"));
    window.set_default_size(500, 500);

    let buffer = sourceview5::Buffer::new(None);
    buffer.set_highlight_syntax(true);
    if let Some(ref language) = sourceview5::LanguageManager::new().get_language("rust") {
        buffer.set_language(Some(language));
    }
    if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().get_scheme("solarized-light") {
        buffer.set_style_scheme(Some(scheme));
    }
    buffer.set_text("println!(\"hello world\"");

    let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let view = sourceview5::View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
    view.set_show_line_numbers(true);
    view.set_highlight_current_line(true);
    view.set_tab_width(4);
    view.set_hexpand(true);
    container.append(&view);
    let map = sourceview5::Map::new();
    map.set_view(&view);
    container.append(&map);

    window.set_child(Some(&container));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.bilelmoussaoui.sourceview5-example"),
        Default::default(),
    )
    .expect("Initialization failed...");
    application.connect_activate(build_ui);

    application.run(&args().collect::<Vec<_>>());
}
