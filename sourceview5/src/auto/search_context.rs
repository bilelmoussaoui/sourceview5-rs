// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buffer;
use crate::SearchSettings;
use crate::Style;
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
use std::mem;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct SearchContext(Object<ffi::GtkSourceSearchContext, ffi::GtkSourceSearchContextClass>);

    match fn {
        get_type => || ffi::gtk_source_search_context_get_type(),
    }
}

impl SearchContext {
    #[doc(alias = "gtk_source_search_context_new")]
    pub fn new<P: IsA<Buffer>, Q: IsA<SearchSettings>>(
        buffer: &P,
        settings: Option<&Q>,
    ) -> SearchContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_new(
                buffer.as_ref().to_glib_none().0,
                settings.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_backward")]
    pub fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_source_search_context_backward(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            let has_wrapped_around = has_wrapped_around.assume_init();
            if ret {
                Some((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_backward_async")]
    pub fn backward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn backward_async_trampoline<
            Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let _ = ffi::gtk_source_search_context_backward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
                &mut error,
            );
            let has_wrapped_around = has_wrapped_around.assume_init();
            let result = if error.is_null() {
                Ok((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = backward_async_trampoline::<Q>;
        unsafe {
            ffi::gtk_source_search_context_backward_async(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn backward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>,
                > + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.backward_async(&iter, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gtk_source_search_context_forward")]
    pub fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_source_search_context_forward(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            let has_wrapped_around = has_wrapped_around.assume_init();
            if ret {
                Some((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_forward_async")]
    pub fn forward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn forward_async_trampoline<
            Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let _ = ffi::gtk_source_search_context_forward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
                &mut error,
            );
            let has_wrapped_around = has_wrapped_around.assume_init();
            let result = if error.is_null() {
                Ok((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = forward_async_trampoline::<Q>;
        unsafe {
            ffi::gtk_source_search_context_forward_async(
                self.to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn forward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>,
                > + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.forward_async(&iter, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gtk_source_search_context_get_buffer")]
    pub fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_buffer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_highlight")]
    pub fn get_highlight(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_context_get_highlight(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_match_style")]
    pub fn get_match_style(&self) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_match_style(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_occurrence_position")]
    pub fn get_occurrence_position(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
    ) -> i32 {
        unsafe {
            ffi::gtk_source_search_context_get_occurrence_position(
                self.to_glib_none().0,
                match_start.to_glib_none().0,
                match_end.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gtk_source_search_context_get_occurrences_count")]
    pub fn get_occurrences_count(&self) -> i32 {
        unsafe { ffi::gtk_source_search_context_get_occurrences_count(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_source_search_context_get_regex_error")]
    pub fn get_regex_error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_full(ffi::gtk_source_search_context_get_regex_error(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_get_settings")]
    pub fn get_settings(&self) -> Option<SearchSettings> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_context_get_settings(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_search_context_replace")]
    pub fn replace(
        &self,
        match_start: &mut gtk::TextIter,
        match_end: &mut gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_search_context_replace(
                self.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_replace_all")]
    pub fn replace_all(&self, replace: &str) -> Result<(), glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_search_context_replace_all(
                self.to_glib_none().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_source_search_context_set_highlight")]
    pub fn set_highlight(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_search_context_set_highlight(
                self.to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    #[doc(alias = "gtk_source_search_context_set_match_style")]
    pub fn set_match_style(&self, match_style: Option<&Style>) {
        unsafe {
            ffi::gtk_source_search_context_set_match_style(
                self.to_glib_none().0,
                match_style.to_glib_none().0,
            );
        }
    }

    pub fn connect_property_highlight_notify<F: Fn(&SearchContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
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
                b"notify::highlight\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_match_style_notify<F: Fn(&SearchContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_match_style_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
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
                b"notify::match-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_match_style_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_occurrences_count_notify<F: Fn(&SearchContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_occurrences_count_trampoline<
            F: Fn(&SearchContext) + 'static,
        >(
            this: *mut ffi::GtkSourceSearchContext,
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
                b"notify::occurrences-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_occurrences_count_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_regex_error_notify<F: Fn(&SearchContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_regex_error_trampoline<F: Fn(&SearchContext) + 'static>(
            this: *mut ffi::GtkSourceSearchContext,
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
                b"notify::regex-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_regex_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct SearchContextBuilder {
    buffer: Option<Buffer>,
    highlight: Option<bool>,
    match_style: Option<Style>,
    settings: Option<SearchSettings>,
}

impl SearchContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SearchContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref highlight) = self.highlight {
            properties.push(("highlight", highlight));
        }
        if let Some(ref match_style) = self.match_style {
            properties.push(("match-style", match_style));
        }
        if let Some(ref settings) = self.settings {
            properties.push(("settings", settings));
        }
        let ret = glib::Object::new::<SearchContext>(&properties).expect("object new");
        ret
    }

    pub fn buffer<P: IsA<Buffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }

    pub fn match_style(mut self, match_style: &Style) -> Self {
        self.match_style = Some(match_style.clone());
        self
    }

    pub fn settings<P: IsA<SearchSettings>>(mut self, settings: &P) -> Self {
        self.settings = Some(settings.clone().upcast());
        self
    }
}

impl fmt::Display for SearchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SearchContext")
    }
}