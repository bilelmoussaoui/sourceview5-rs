// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Mark;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct MarkAttributes(Object<ffi::GtkSourceMarkAttributes, ffi::GtkSourceMarkAttributesClass>);

    match fn {
        get_type => || ffi::gtk_source_mark_attributes_get_type(),
    }
}

impl MarkAttributes {
    #[doc(alias = "gtk_source_mark_attributes_new")]
    pub fn new() -> MarkAttributes {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_mark_attributes_new()) }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_background")]
    pub fn get_background(&self) -> Option<gdk::RGBA> {
        unsafe {
            let mut background = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_source_mark_attributes_get_background(
                self.to_glib_none().0,
                background.to_glib_none_mut().0,
            ));
            if ret {
                Some(background)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_gicon")]
    pub fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_gicon(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_icon_name")]
    pub fn get_icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_icon_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_pixbuf")]
    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_get_pixbuf(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_tooltip_markup")]
    pub fn get_tooltip_markup<P: IsA<Mark>>(&self, mark: &P) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_markup(
                self.to_glib_none().0,
                mark.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_get_tooltip_text")]
    pub fn get_tooltip_text<P: IsA<Mark>>(&self, mark: &P) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_attributes_get_tooltip_text(
                self.to_glib_none().0,
                mark.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_render_icon")]
    pub fn render_icon<P: IsA<gtk::Widget>>(
        &self,
        widget: &P,
        size: i32,
    ) -> Option<gdk::Paintable> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_attributes_render_icon(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                size,
            ))
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_background")]
    pub fn set_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_background(
                self.to_glib_none().0,
                background.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_gicon")]
    pub fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: &P) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_gicon(
                self.to_glib_none().0,
                gicon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_mark_attributes_set_pixbuf")]
    pub fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_source_mark_attributes_set_pixbuf(
                self.to_glib_none().0,
                pixbuf.to_glib_none().0,
            );
        }
    }

    pub fn connect_query_tooltip_markup<F: Fn(&MarkAttributes, &Mark) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_markup_trampoline<
            F: Fn(&MarkAttributes, &Mark) -> String + 'static,
        >(
            this: *mut ffi::GtkSourceMarkAttributes,
            mark: *mut ffi::GtkSourceMark,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mark)).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_tooltip_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_query_tooltip_text<F: Fn(&MarkAttributes, &Mark) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn query_tooltip_text_trampoline<
            F: Fn(&MarkAttributes, &Mark) -> String + 'static,
        >(
            this: *mut ffi::GtkSourceMarkAttributes,
            mark: *mut ffi::GtkSourceMark,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mark)).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-tooltip-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_tooltip_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_background_notify<F: Fn(&MarkAttributes) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_gicon_notify<F: Fn(&MarkAttributes) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_icon_name_notify<F: Fn(&MarkAttributes) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_pixbuf_notify<F: Fn(&MarkAttributes) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<F: Fn(&MarkAttributes) + 'static>(
            this: *mut ffi::GtkSourceMarkAttributes,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for MarkAttributes {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct MarkAttributesBuilder {
    background: Option<gdk::RGBA>,
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
}

impl MarkAttributesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MarkAttributes {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref background) = self.background {
            properties.push(("background", background));
        }
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        let ret = glib::Object::new::<MarkAttributes>(&properties).expect("object new");
        ret
    }

    pub fn background(mut self, background: &gdk::RGBA) -> Self {
        self.background = Some(background.clone());
        self
    }

    pub fn gicon<P: IsA<gio::Icon>>(mut self, gicon: &P) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }
}

impl fmt::Display for MarkAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MarkAttributes")
    }
}
