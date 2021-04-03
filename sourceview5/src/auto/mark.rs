// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct Mark(Object<ffi::GtkSourceMark, ffi::GtkSourceMarkClass>) @extends gtk::TextMark;

    match fn {
        get_type => || ffi::gtk_source_mark_get_type(),
    }
}

impl Mark {
    #[doc(alias = "gtk_source_mark_new")]
    pub fn new(name: &str, category: &str) -> Mark {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_mark_new(
                name.to_glib_none().0,
                category.to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct MarkBuilder {
    category: Option<String>,
    left_gravity: Option<bool>,
    name: Option<String>,
}

impl MarkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Mark {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref category) = self.category {
            properties.push(("category", category));
        }
        if let Some(ref left_gravity) = self.left_gravity {
            properties.push(("left-gravity", left_gravity));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        let ret = glib::Object::new::<Mark>(&properties).expect("object new");
        ret
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    pub fn left_gravity(mut self, left_gravity: bool) -> Self {
        self.left_gravity = Some(left_gravity);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

pub const NONE_MARK: Option<&Mark> = None;

pub trait MarkExt: 'static {
    #[doc(alias = "gtk_source_mark_get_category")]
    fn get_category(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_mark_next")]
    fn next(&self, category: Option<&str>) -> Option<Mark>;

    #[doc(alias = "gtk_source_mark_prev")]
    fn prev(&self, category: &str) -> Option<Mark>;
}

impl<O: IsA<Mark>> MarkExt for O {
    fn get_category(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_get_category(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next(&self, category: Option<&str>) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_next(
                self.as_ref().to_glib_none().0,
                category.to_glib_none().0,
            ))
        }
    }

    fn prev(&self, category: &str) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_prev(
                self.as_ref().to_glib_none().0,
                category.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Mark")
    }
}
