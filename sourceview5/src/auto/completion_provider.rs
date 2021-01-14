// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CompletionCell;
use crate::CompletionContext;
use crate::CompletionProposal;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct CompletionProvider(Interface<ffi::GtkSourceCompletionProvider>);

    match fn {
        get_type => || ffi::gtk_source_completion_provider_get_type(),
    }
}

pub const NONE_COMPLETION_PROVIDER: Option<&CompletionProvider> = None;

pub trait CompletionProviderExt: 'static {
    #[doc(alias = "gtk_source_completion_provider_activate")]
    fn activate<P: IsA<CompletionProposal>>(&self, context: &CompletionContext, proposal: &P);

    #[doc(alias = "gtk_source_completion_provider_display")]
    fn display<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
        cell: &CompletionCell,
    );

    #[doc(alias = "gtk_source_completion_provider_get_priority")]
    fn get_priority(&self, context: &CompletionContext) -> i32;

    #[doc(alias = "gtk_source_completion_provider_get_title")]
    fn get_title(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_completion_provider_is_trigger")]
    fn is_trigger(&self, iter: &gtk::TextIter, ch: char) -> bool;

    #[doc(alias = "gtk_source_completion_provider_key_activates")]
    fn key_activates<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
        keyval: u32,
        state: gdk::ModifierType,
    ) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_completion_provider_list_alternates")]
    fn list_alternates<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
    ) -> Vec<CompletionProposal>;

    #[doc(alias = "gtk_source_completion_provider_populate_async")]
    fn populate_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<gio::ListModel, glib::Error>) + Send + 'static,
    >(
        &self,
        context: &CompletionContext,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn populate_async_future(
        &self,
        context: &CompletionContext,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::ListModel, glib::Error>> + 'static>>;

    #[doc(alias = "gtk_source_completion_provider_refilter")]
    fn refilter<P: IsA<gio::ListModel>>(&self, context: &CompletionContext, model: &P);
}

impl<O: IsA<CompletionProvider>> CompletionProviderExt for O {
    fn activate<P: IsA<CompletionProposal>>(&self, context: &CompletionContext, proposal: &P) {
        unsafe {
            ffi::gtk_source_completion_provider_activate(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                proposal.as_ref().to_glib_none().0,
            );
        }
    }

    fn display<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
        cell: &CompletionCell,
    ) {
        unsafe {
            ffi::gtk_source_completion_provider_display(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                proposal.as_ref().to_glib_none().0,
                cell.to_glib_none().0,
            );
        }
    }

    fn get_priority(&self, context: &CompletionContext) -> i32 {
        unsafe {
            ffi::gtk_source_completion_provider_get_priority(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            )
        }
    }

    fn get_title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_provider_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_trigger(&self, iter: &gtk::TextIter, ch: char) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_provider_is_trigger(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                ch.to_glib(),
            ))
        }
    }

    fn key_activates<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
        keyval: u32,
        state: gdk::ModifierType,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_provider_key_activates(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                proposal.as_ref().to_glib_none().0,
                keyval,
                state.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn list_alternates<P: IsA<CompletionProposal>>(
        &self,
        context: &CompletionContext,
        proposal: &P,
    ) -> Vec<CompletionProposal> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gtk_source_completion_provider_list_alternates(
                    self.as_ref().to_glib_none().0,
                    context.to_glib_none().0,
                    proposal.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn populate_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<gio::ListModel, glib::Error>) + Send + 'static,
    >(
        &self,
        context: &CompletionContext,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn populate_async_trampoline<
            Q: FnOnce(Result<gio::ListModel, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_source_completion_provider_populate_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = populate_async_trampoline::<Q>;
        unsafe {
            ffi::gtk_source_completion_provider_populate_async(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn populate_async_future(
        &self,
        context: &CompletionContext,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::ListModel, glib::Error>> + 'static>>
    {
        let context = context.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.populate_async(&context, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn refilter<P: IsA<gio::ListModel>>(&self, context: &CompletionContext, model: &P) {
        unsafe {
            ffi::gtk_source_completion_provider_refilter(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                model.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for CompletionProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CompletionProvider")
    }
}
