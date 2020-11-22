// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::Buffer;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::Completion;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::CompletionActivation;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::CompletionProvider;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::Language;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use crate::View;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct CompletionContext(Object<ffi::GtkSourceCompletionContext, ffi::GtkSourceCompletionContextClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_source_completion_context_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct CompletionContextBuilder {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    completion: Option<Completion>,
}

impl CompletionContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CompletionContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v5_0", feature = "dox"))]
        if let Some(ref completion) = self.completion {
            properties.push(("completion", completion));
        }
        let ret = glib::Object::new(CompletionContext::static_type(), &properties)
            .expect("object new")
            .downcast::<CompletionContext>()
            .expect("downcast");
        ret
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    pub fn completion<P: IsA<Completion>>(mut self, completion: &P) -> Self {
        self.completion = Some(completion.clone().upcast());
        self
    }
}

pub const NONE_COMPLETION_CONTEXT: Option<&CompletionContext> = None;

pub trait CompletionContextExt: 'static {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_activation(&self) -> CompletionActivation;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_busy(&self) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_completion(&self) -> Option<Completion>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_empty(&self) -> bool;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_language(&self) -> Option<Language>;

    fn get_start_iter(&self, iter: &mut gtk::TextIter);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_view(&self) -> Option<View>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_word(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_proposals_for_provider<P: IsA<CompletionProvider>, Q: IsA<gio::ListModel>>(
        &self,
        provider: &P,
        results: Option<&Q>,
    );

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_property_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_property_empty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionContext>> CompletionContextExt for O {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_activation(&self) -> CompletionActivation {
        unsafe {
            from_glib(ffi::gtk_source_completion_context_get_activation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_bounds(&self) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut begin = gtk::TextIter::uninitialized();
            let mut end = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_completion_context_get_bounds(
                self.as_ref().to_glib_none().0,
                begin.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            ));
            if ret {
                Some((begin, end))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_context_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_busy(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_context_get_busy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_completion(&self) -> Option<Completion> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_context_get_completion(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_empty(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_context_get_empty(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_language(&self) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_context_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_start_iter(&self, iter: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_completion_context_get_start_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_context_get_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_word(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_context_get_word(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn set_proposals_for_provider<P: IsA<CompletionProvider>, Q: IsA<gio::ListModel>>(
        &self,
        provider: &P,
        results: Option<&Q>,
    ) {
        unsafe {
            ffi::gtk_source_completion_context_set_proposals_for_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                results.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_property_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_busy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionContext>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::busy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_busy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_property_empty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_empty_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionContext>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::empty\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_empty_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionContext")
    }
}
