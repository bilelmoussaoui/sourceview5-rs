// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v5_0", feature = "dox"))]
use gio;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::GString;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib_sys;
use gtk_source_sys;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use Snippet;

glib_wrapper! {
    pub struct SnippetManager(Object<gtk_source_sys::GtkSourceSnippetManager, gtk_source_sys::GtkSourceSnippetManagerClass, SnippetManagerClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_snippet_manager_get_type(),
    }
}

impl SnippetManager {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    pub fn get_default() -> Option<SnippetManager> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_source_sys::gtk_source_snippet_manager_get_default()) }
    }
}

#[derive(Clone, Default)]
pub struct SnippetManagerBuilder {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    search_path: Option<Vec<String>>,
}

impl SnippetManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SnippetManager {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v5_0", feature = "dox"))]
        {
            if let Some(ref search_path) = self.search_path {
                properties.push(("search-path", search_path));
            }
        }
        let ret = glib::Object::new(SnippetManager::static_type(), &properties)
            .expect("object new")
            .downcast::<SnippetManager>()
            .expect("downcast");
        ret
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    pub fn search_path(mut self, search_path: Vec<String>) -> Self {
        self.search_path = Some(search_path);
        self
    }
}

pub const NONE_SNIPPET_MANAGER: Option<&SnippetManager> = None;

pub trait SnippetManagerExt: 'static {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_search_path(&self) -> Vec<GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_snippet(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger: &str,
    ) -> Option<Snippet>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn list_groups(&self) -> Vec<GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn list_matching(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger_prefix: Option<&str>,
    ) -> Option<gio::ListModel>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_search_path(&self, dirs: &[&str]);

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SnippetManager>> SnippetManagerExt for O {
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_search_path(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                gtk_source_sys::gtk_source_snippet_manager_get_search_path(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_snippet(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger: &str,
    ) -> Option<Snippet> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_snippet_manager_get_snippet(
                self.as_ref().to_glib_none().0,
                group.to_glib_none().0,
                language_id.to_glib_none().0,
                trigger.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn list_groups(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                gtk_source_sys::gtk_source_snippet_manager_list_groups(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn list_matching(
        &self,
        group: Option<&str>,
        language_id: Option<&str>,
        trigger_prefix: Option<&str>,
    ) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_snippet_manager_list_matching(
                self.as_ref().to_glib_none().0,
                group.to_glib_none().0,
                language_id.to_glib_none().0,
                trigger_prefix.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn set_search_path(&self, dirs: &[&str]) {
        unsafe {
            gtk_source_sys::gtk_source_snippet_manager_set_search_path(
                self.as_ref().to_glib_none().0,
                dirs.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_path_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSnippetManager,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SnippetManager>,
        {
            let f: &F = &*(f as *const F);
            f(&SnippetManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-path\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_path_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SnippetManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SnippetManager")
    }
}
