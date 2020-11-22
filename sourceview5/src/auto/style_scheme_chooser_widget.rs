// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StyleSchemeChooser;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct StyleSchemeChooserWidget(Object<ffi::GtkSourceStyleSchemeChooserWidget, ffi::GtkSourceStyleSchemeChooserWidgetClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, StyleSchemeChooser;

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_widget_get_type(),
    }
}

impl StyleSchemeChooserWidget {
    pub fn new() -> StyleSchemeChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_style_scheme_chooser_widget_new())
                .unsafe_cast()
        }
    }
}

impl Default for StyleSchemeChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STYLE_SCHEME_CHOOSER_WIDGET: Option<&StyleSchemeChooserWidget> = None;

impl fmt::Display for StyleSchemeChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleSchemeChooserWidget")
    }
}
