// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::StyleSchemeChooser;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct StyleSchemeChooserButton(Object<ffi::GtkSourceStyleSchemeChooserButton, ffi::GtkSourceStyleSchemeChooserButtonClass>) @extends gtk::Button, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable, StyleSchemeChooser;

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_button_get_type(),
    }
}

impl StyleSchemeChooserButton {
    #[doc(alias = "gtk_source_style_scheme_chooser_button_new")]
    pub fn new() -> StyleSchemeChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_style_scheme_chooser_button_new())
                .unsafe_cast()
        }
    }
}

impl Default for StyleSchemeChooserButton {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_STYLE_SCHEME_CHOOSER_BUTTON: Option<&StyleSchemeChooserButton> = None;

impl fmt::Display for StyleSchemeChooserButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleSchemeChooserButton")
    }
}
