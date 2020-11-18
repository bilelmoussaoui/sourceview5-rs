extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate gio;
extern crate gio_sys;
extern crate libc;
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate gtk_sys;
extern crate pango;
extern crate sourceview5_sys as gtk_source_sys;
#[macro_use]
extern crate bitflags;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GtkSourceView may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using GtkSourceView.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
#[allow(unused_doc_comments)]
#[allow(unused_imports)]
mod auto;
pub use auto::*;

mod gutter_lines;
pub use gutter_lines::*;
mod view;
pub use view::*;

pub mod prelude;
