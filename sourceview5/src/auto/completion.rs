// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::Buffer;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::CompletionProvider;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::View;
use glib::object::Cast;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use glib::object::IsA;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct Completion(Object<ffi::GtkSourceCompletion, ffi::GtkSourceCompletionClass>);

    match fn {
        get_type => || ffi::gtk_source_completion_get_type(),
    }
}

impl Completion {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_add_provider")]
    pub fn add_provider<P: IsA<CompletionProvider>>(&self, provider: &P) {
        unsafe {
            ffi::gtk_source_completion_add_provider(
                self.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_completion_block_interactive")]
    pub fn block_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_block_interactive(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_get_buffer")]
    pub fn get_buffer(&self) -> Option<Buffer> {
        unsafe { from_glib_none(ffi::gtk_source_completion_get_buffer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_source_completion_get_page_size")]
    pub fn get_page_size(&self) -> u32 {
        unsafe { ffi::gtk_source_completion_get_page_size(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_get_view")]
    pub fn get_view(&self) -> Option<View> {
        unsafe { from_glib_none(ffi::gtk_source_completion_get_view(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_hide")]
    pub fn hide(&self) {
        unsafe {
            ffi::gtk_source_completion_hide(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_remove_provider")]
    pub fn remove_provider<P: IsA<CompletionProvider>>(&self, provider: &P) {
        unsafe {
            ffi::gtk_source_completion_remove_provider(
                self.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_source_completion_set_page_size")]
    pub fn set_page_size(&self, page_size: u32) {
        unsafe {
            ffi::gtk_source_completion_set_page_size(self.to_glib_none().0, page_size);
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_show")]
    pub fn show(&self) {
        unsafe {
            ffi::gtk_source_completion_show(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_source_completion_unblock_interactive")]
    pub fn unblock_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_unblock_interactive(self.to_glib_none().0);
        }
    }

    pub fn get_property_remember_info_visibility(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"remember-info-visibility\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `remember-info-visibility` getter")
                .unwrap()
        }
    }

    pub fn set_property_remember_info_visibility(&self, remember_info_visibility: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"remember-info-visibility\0".as_ptr() as *const _,
                glib::Value::from(&remember_info_visibility)
                    .to_glib_none()
                    .0,
            );
        }
    }

    pub fn get_property_select_on_show(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"select-on-show\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `select-on-show` getter")
                .unwrap()
        }
    }

    pub fn set_property_select_on_show(&self, select_on_show: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"select-on-show\0".as_ptr() as *const _,
                glib::Value::from(&select_on_show).to_glib_none().0,
            );
        }
    }

    pub fn get_property_show_icons(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"show-icons\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `show-icons` getter")
                .unwrap()
        }
    }

    pub fn set_property_show_icons(&self, show_icons: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"show-icons\0".as_ptr() as *const _,
                glib::Value::from(&show_icons).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_fuzzy_highlight")]
    pub fn fuzzy_highlight(haystack: &str, casefold_query: &str) -> Option<pango::AttrList> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_completion_fuzzy_highlight(
                haystack.to_glib_none().0,
                casefold_query.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_fuzzy_match")]
    pub fn fuzzy_match(haystack: Option<&str>, casefold_needle: &str) -> Option<u32> {
        assert_initialized_main_thread!();
        unsafe {
            let mut priority = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_source_completion_fuzzy_match(
                haystack.to_glib_none().0,
                casefold_needle.to_glib_none().0,
                priority.as_mut_ptr(),
            ));
            let priority = priority.assume_init();
            if ret {
                Some(priority)
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_hide<F: Fn(&Completion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hide_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"hide\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    hide_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn emit_hide(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("hide", &[])
                .unwrap()
        };
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_provider_added<F: Fn(&Completion, &CompletionProvider) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_added_trampoline<
            F: Fn(&Completion, &CompletionProvider) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            provider: *mut ffi::GtkSourceCompletionProvider,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(provider))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_provider_removed<F: Fn(&Completion, &CompletionProvider) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_removed_trampoline<
            F: Fn(&Completion, &CompletionProvider) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
            provider: *mut ffi::GtkSourceCompletionProvider,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(provider))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_show<F: Fn(&Completion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn emit_show(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("show", &[])
                .unwrap()
        };
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_property_buffer_notify<F: Fn(&Completion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
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

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn connect_property_page_size_notify<F: Fn(&Completion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_size_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
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
                b"notify::page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_remember_info_visibility_notify<F: Fn(&Completion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remember_info_visibility_trampoline<
            F: Fn(&Completion) + 'static,
        >(
            this: *mut ffi::GtkSourceCompletion,
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
                b"notify::remember-info-visibility\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remember_info_visibility_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_select_on_show_notify<F: Fn(&Completion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_on_show_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
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
                b"notify::select-on-show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_on_show_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_icons_notify<F: Fn(&Completion) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_icons_trampoline<F: Fn(&Completion) + 'static>(
            this: *mut ffi::GtkSourceCompletion,
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
                b"notify::show-icons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_icons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct CompletionBuilder {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    page_size: Option<u32>,
    remember_info_visibility: Option<bool>,
    select_on_show: Option<bool>,
    show_icons: Option<bool>,
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    view: Option<View>,
}

impl CompletionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Completion {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v5_0", feature = "dox"))]
        if let Some(ref page_size) = self.page_size {
            properties.push(("page-size", page_size));
        }
        if let Some(ref remember_info_visibility) = self.remember_info_visibility {
            properties.push(("remember-info-visibility", remember_info_visibility));
        }
        if let Some(ref select_on_show) = self.select_on_show {
            properties.push(("select-on-show", select_on_show));
        }
        if let Some(ref show_icons) = self.show_icons {
            properties.push(("show-icons", show_icons));
        }
        #[cfg(any(feature = "v5_0", feature = "dox"))]
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        let ret = glib::Object::new::<Completion>(&properties).expect("object new");
        ret
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn remember_info_visibility(mut self, remember_info_visibility: bool) -> Self {
        self.remember_info_visibility = Some(remember_info_visibility);
        self
    }

    pub fn select_on_show(mut self, select_on_show: bool) -> Self {
        self.select_on_show = Some(select_on_show);
        self
    }

    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = Some(show_icons);
        self
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn view<P: IsA<View>>(mut self, view: &P) -> Self {
        self.view = Some(view.clone().upcast());
        self
    }
}

impl fmt::Display for Completion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Completion")
    }
}
