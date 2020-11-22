// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CompletionProvider;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct CompletionWords(Object<ffi::GtkSourceCompletionWords, ffi::GtkSourceCompletionWordsClass>) @implements CompletionProvider;

    match fn {
        get_type => || ffi::gtk_source_completion_words_get_type(),
    }
}

impl CompletionWords {
    pub fn new(title: Option<&str>) -> CompletionWords {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_completion_words_new(title.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
pub struct CompletionWordsBuilder {
    minimum_word_size: Option<u32>,
    priority: Option<i32>,
    proposals_batch_size: Option<u32>,
    scan_batch_size: Option<u32>,
    title: Option<String>,
}

impl CompletionWordsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CompletionWords {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref minimum_word_size) = self.minimum_word_size {
            properties.push(("minimum-word-size", minimum_word_size));
        }
        if let Some(ref priority) = self.priority {
            properties.push(("priority", priority));
        }
        if let Some(ref proposals_batch_size) = self.proposals_batch_size {
            properties.push(("proposals-batch-size", proposals_batch_size));
        }
        if let Some(ref scan_batch_size) = self.scan_batch_size {
            properties.push(("scan-batch-size", scan_batch_size));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        let ret = glib::Object::new(CompletionWords::static_type(), &properties)
            .expect("object new")
            .downcast::<CompletionWords>()
            .expect("downcast");
        ret
    }

    pub fn minimum_word_size(mut self, minimum_word_size: u32) -> Self {
        self.minimum_word_size = Some(minimum_word_size);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn proposals_batch_size(mut self, proposals_batch_size: u32) -> Self {
        self.proposals_batch_size = Some(proposals_batch_size);
        self
    }

    pub fn scan_batch_size(mut self, scan_batch_size: u32) -> Self {
        self.scan_batch_size = Some(scan_batch_size);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

pub const NONE_COMPLETION_WORDS: Option<&CompletionWords> = None;

pub trait CompletionWordsExt: 'static {
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P);

    fn get_property_minimum_word_size(&self) -> u32;

    fn set_property_minimum_word_size(&self, minimum_word_size: u32);

    fn get_property_priority(&self) -> i32;

    fn set_property_priority(&self, priority: i32);

    fn get_property_proposals_batch_size(&self) -> u32;

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32);

    fn get_property_scan_batch_size(&self) -> u32;

    fn set_property_scan_batch_size(&self, scan_batch_size: u32);

    fn set_property_title(&self, title: Option<&str>);

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionWords>> CompletionWordsExt for O {
    fn register<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            ffi::gtk_source_completion_words_register(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    fn unregister<P: IsA<gtk::TextBuffer>>(&self, buffer: &P) {
        unsafe {
            ffi::gtk_source_completion_words_unregister(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_property_minimum_word_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"minimum-word-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `minimum-word-size` getter")
                .unwrap()
        }
    }

    fn set_property_minimum_word_size(&self, minimum_word_size: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"minimum-word-size\0".as_ptr() as *const _,
                Value::from(&minimum_word_size).to_glib_none().0,
            );
        }
    }

    fn get_property_priority(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"priority\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `priority` getter")
                .unwrap()
        }
    }

    fn set_property_priority(&self, priority: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"priority\0".as_ptr() as *const _,
                Value::from(&priority).to_glib_none().0,
            );
        }
    }

    fn get_property_proposals_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"proposals-batch-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `proposals-batch-size` getter")
                .unwrap()
        }
    }

    fn set_property_proposals_batch_size(&self, proposals_batch_size: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"proposals-batch-size\0".as_ptr() as *const _,
                Value::from(&proposals_batch_size).to_glib_none().0,
            );
        }
    }

    fn get_property_scan_batch_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"scan-batch-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `scan-batch-size` getter")
                .unwrap()
        }
    }

    fn set_property_scan_batch_size(&self, scan_batch_size: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"scan-batch-size\0".as_ptr() as *const _,
                Value::from(&scan_batch_size).to_glib_none().0,
            );
        }
    }

    fn set_property_title(&self, title: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                Value::from(title).to_glib_none().0,
            );
        }
    }

    fn connect_property_minimum_word_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_word_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionWords>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::minimum-word-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_minimum_word_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionWords>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_proposals_batch_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proposals_batch_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionWords>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proposals-batch-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proposals_batch_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_scan_batch_size_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scan_batch_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionWords>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scan-batch-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scan_batch_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceCompletionWords,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CompletionWords>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionWords::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionWords")
    }
}
