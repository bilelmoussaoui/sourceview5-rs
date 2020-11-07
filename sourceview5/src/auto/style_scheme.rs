// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Style;

glib_wrapper! {
    pub struct StyleScheme(Object<gtk_source_sys::GtkSourceStyleScheme, gtk_source_sys::GtkSourceStyleSchemeClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_style_scheme_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct StyleSchemeBuilder {
    id: Option<String>,
}

impl StyleSchemeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> StyleScheme {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref id) = self.id {
            properties.push(("id", id));
        }
        let ret = glib::Object::new(StyleScheme::static_type(), &properties)
            .expect("object new")
            .downcast::<StyleScheme>()
            .expect("downcast");
        ret
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
}

pub const NONE_STYLE_SCHEME: Option<&StyleScheme> = None;

pub trait StyleSchemeExt: 'static {
    fn get_authors(&self) -> Vec<GString>;

    fn get_description(&self) -> Option<GString>;

    fn get_filename(&self) -> Option<GString>;

    fn get_id(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_style(&self, style_id: &str) -> Option<Style>;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleScheme>> StyleSchemeExt for O {
    fn get_authors(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                gtk_source_sys::gtk_source_style_scheme_get_authors(self.as_ref().to_glib_none().0),
            )
        }
    }

    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_style_scheme_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filename(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_style_scheme_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_style_scheme_get_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_style_scheme_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_style(&self, style_id: &str) -> Option<Style> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_style_scheme_get_style(
                self.as_ref().to_glib_none().0,
                style_id.to_glib_none().0,
            ))
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceStyleScheme,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleScheme>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceStyleScheme,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleScheme>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filename\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceStyleScheme,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleScheme>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleScheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleScheme")
    }
}
