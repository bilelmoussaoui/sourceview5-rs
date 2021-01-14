// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::SnippetChunk;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
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
    pub struct Snippet(Object<ffi::GtkSourceSnippet, ffi::GtkSourceSnippetClass>);

    match fn {
        get_type => || ffi::gtk_source_snippet_get_type(),
    }
}

impl Snippet {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_new")]
    pub fn new(trigger: Option<&str>, language_id: Option<&str>) -> Snippet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_snippet_new(
                trigger.to_glib_none().0,
                language_id.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_add_chunk")]
    pub fn add_chunk(&self, chunk: &SnippetChunk) {
        unsafe {
            ffi::gtk_source_snippet_add_chunk(self.to_glib_none().0, chunk.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_copy")]
    pub fn copy(&self) -> Option<Snippet> {
        unsafe { from_glib_full(ffi::gtk_source_snippet_copy(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_context")]
    pub fn get_context(&self) -> Option<SnippetContext> {
        unsafe { from_glib_none(ffi::gtk_source_snippet_get_context(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_description")]
    pub fn get_description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_get_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_focus_position")]
    pub fn get_focus_position(&self) -> i32 {
        unsafe { ffi::gtk_source_snippet_get_focus_position(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_language_id")]
    pub fn get_language_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_get_language_id(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_n_chunks")]
    pub fn get_n_chunks(&self) -> u32 {
        unsafe { ffi::gtk_source_snippet_get_n_chunks(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_name")]
    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_source_snippet_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_nth_chunk")]
    pub fn get_nth_chunk(&self, nth: u32) -> Option<SnippetChunk> {
        unsafe {
            from_glib_none(ffi::gtk_source_snippet_get_nth_chunk(
                self.to_glib_none().0,
                nth,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_get_trigger")]
    pub fn get_trigger(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_source_snippet_get_trigger(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_set_description")]
    pub fn set_description(&self, description: &str) {
        unsafe {
            ffi::gtk_source_snippet_set_description(
                self.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_set_language_id")]
    pub fn set_language_id(&self, language_id: &str) {
        unsafe {
            ffi::gtk_source_snippet_set_language_id(
                self.to_glib_none().0,
                language_id.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_set_name")]
    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_source_snippet_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_snippet_set_trigger")]
    pub fn set_trigger(&self, trigger: &str) {
        unsafe {
            ffi::gtk_source_snippet_set_trigger(self.to_glib_none().0, trigger.to_glib_none().0);
        }
    }

    pub fn get_property_buffer(&self) -> Option<gtk::TextBuffer> {
        unsafe {
            let mut value = glib::Value::from_type(<gtk::TextBuffer as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"buffer\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffer` getter")
        }
    }

    pub fn get_property_description(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"description\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `description` getter")
        }
    }

    pub fn set_property_description(&self, description: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"description\0".as_ptr() as *const _,
                glib::Value::from(description).to_glib_none().0,
            );
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

    pub fn get_property_language_id(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"language-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `language-id` getter")
        }
    }

    pub fn set_property_language_id(&self, language_id: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"language-id\0".as_ptr() as *const _,
                glib::Value::from(language_id).to_glib_none().0,
            );
        }
    }

    pub fn get_property_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name` getter")
        }
    }

    pub fn set_property_name(&self, name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"name\0".as_ptr() as *const _,
                glib::Value::from(name).to_glib_none().0,
            );
        }
    }

    pub fn get_property_trigger(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"trigger\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `trigger` getter")
        }
    }

    pub fn set_property_trigger(&self, trigger: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"trigger\0".as_ptr() as *const _,
                glib::Value::from(trigger).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_buffer_notify<F: Fn(&Snippet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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
                b"notify::buffer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_description_notify<F: Fn(&Snippet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_focus_position_notify<F: Fn(&Snippet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_position_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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

    pub fn connect_property_language_id_notify<F: Fn(&Snippet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_id_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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
                b"notify::language-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_name_notify<F: Fn(&Snippet) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_trigger_notify<F: Fn(&Snippet) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_trigger_trampoline<F: Fn(&Snippet) + 'static>(
            this: *mut ffi::GtkSourceSnippet,
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
                b"notify::trigger\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_trigger_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct SnippetBuilder {
    description: Option<String>,
    language_id: Option<String>,
    name: Option<String>,
    trigger: Option<String>,
}

impl SnippetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Snippet {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref description) = self.description {
            properties.push(("description", description));
        }
        if let Some(ref language_id) = self.language_id {
            properties.push(("language-id", language_id));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref trigger) = self.trigger {
            properties.push(("trigger", trigger));
        }
        let ret = glib::Object::new::<Snippet>(&properties).expect("object new");
        ret
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn language_id(mut self, language_id: &str) -> Self {
        self.language_id = Some(language_id.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn trigger(mut self, trigger: &str) -> Self {
        self.trigger = Some(trigger.to_string());
        self
    }
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Snippet")
    }
}
