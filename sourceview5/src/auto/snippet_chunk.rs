// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SnippetContext;
use glib::object::Cast;
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
    pub struct SnippetChunk(Object<ffi::GtkSourceSnippetChunk, ffi::GtkSourceSnippetChunkClass>);

    match fn {
        get_type => || ffi::gtk_source_snippet_chunk_get_type(),
    }
}

impl SnippetChunk {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_new")]
    pub fn new() -> SnippetChunk {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_source_snippet_chunk_new()) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_copy")]
    pub fn copy(&self) -> Option<SnippetChunk> {
        unsafe { from_glib_full(ffi::gtk_source_snippet_chunk_copy(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_get_context")]
    pub fn get_context(&self) -> Option<SnippetContext> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_chunk_get_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_get_focus_position")]
    pub fn get_focus_position(&self) -> i32 {
        unsafe { ffi::gtk_source_snippet_chunk_get_focus_position(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_get_spec")]
    pub fn get_spec(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_chunk_get_spec(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_get_text")]
    pub fn get_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_chunk_get_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_get_text_set")]
    pub fn get_text_set(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_snippet_chunk_get_text_set(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_chunk_get_tooltip_text")]
    pub fn get_tooltip_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_chunk_get_tooltip_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_snippet_chunk_set_context")]
    pub fn set_context(&self, context: &SnippetContext) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_context(
                self.to_glib_none().0,
                context.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_set_focus_position")]
    pub fn set_focus_position(&self, focus_position: i32) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_focus_position(self.to_glib_none().0, focus_position);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_set_spec")]
    pub fn set_spec(&self, spec: &str) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_spec(self.to_glib_none().0, spec.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_set_text")]
    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_chunk_set_text_set")]
    pub fn set_text_set(&self, text_set: bool) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_text_set(self.to_glib_none().0, text_set.to_glib());
        }
    }

    #[doc(alias = "gtk_source_snippet_chunk_set_tooltip_text")]
    pub fn set_tooltip_text(&self, tooltip_text: &str) {
        unsafe {
            ffi::gtk_source_snippet_chunk_set_tooltip_text(
                self.to_glib_none().0,
                tooltip_text.to_glib_none().0,
            );
        }
    }

    pub fn get_property_context(&self) -> Option<SnippetContext> {
        unsafe {
            let mut value = glib::Value::from_type(<SnippetContext as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"context\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `context` getter")
        }
    }

    pub fn get_property_focus_position(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"focus-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `focus-position` getter")
                .unwrap()
        }
    }

    pub fn set_property_focus_position(&self, focus_position: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"focus-position\0".as_ptr() as *const _,
                glib::Value::from(&focus_position).to_glib_none().0,
            );
        }
    }

    pub fn get_property_spec(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"spec\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `spec` getter")
        }
    }

    pub fn set_property_spec(&self, spec: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"spec\0".as_ptr() as *const _,
                glib::Value::from(spec).to_glib_none().0,
            );
        }
    }

    pub fn get_property_text(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text\0".as_ptr() as *const _,
                glib::Value::from(text).to_glib_none().0,
            );
        }
    }

    pub fn get_property_text_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text-set` getter")
                .unwrap()
        }
    }

    pub fn set_property_text_set(&self, text_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"text-set\0".as_ptr() as *const _,
                glib::Value::from(&text_set).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_context_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_context_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_context_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_focus_position_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_position_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::focus-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_spec_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_spec_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::spec\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spec_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_text_set_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_set_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::text-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_tooltip_text_notify<F: Fn(&SnippetChunk) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_text_trampoline<F: Fn(&SnippetChunk) + 'static>(
            this: *mut ffi::GtkSourceSnippetChunk,
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
                b"notify::tooltip-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tooltip_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
impl Default for SnippetChunk {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct SnippetChunkBuilder {
    context: Option<SnippetContext>,
    focus_position: Option<i32>,
    spec: Option<String>,
    text: Option<String>,
    text_set: Option<bool>,
    tooltip_text: Option<String>,
}

impl SnippetChunkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SnippetChunk {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref context) = self.context {
            properties.push(("context", context));
        }
        if let Some(ref focus_position) = self.focus_position {
            properties.push(("focus-position", focus_position));
        }
        if let Some(ref spec) = self.spec {
            properties.push(("spec", spec));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref text_set) = self.text_set {
            properties.push(("text-set", text_set));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        let ret = glib::Object::new::<SnippetChunk>(&properties).expect("object new");
        ret
    }

    pub fn context(mut self, context: &SnippetContext) -> Self {
        self.context = Some(context.clone());
        self
    }

    pub fn focus_position(mut self, focus_position: i32) -> Self {
        self.focus_position = Some(focus_position);
        self
    }

    pub fn spec(mut self, spec: &str) -> Self {
        self.spec = Some(spec.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn text_set(mut self, text_set: bool) -> Self {
        self.text_set = Some(text_set);
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }
}

impl fmt::Display for SnippetChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SnippetChunk")
    }
}