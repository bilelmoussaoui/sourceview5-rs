use crate::{MarkAttributes, Snippet, View};
use glib::translate::*;
use glib::IsA;

pub trait ViewManualExt {
    fn get_mark_attributes(&self, category: &str, priority: i32) -> Option<MarkAttributes>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn push_snippet<P: IsA<Snippet>>(&self, snippet: &P, location: Option<&mut gtk::TextIter>);
}

impl<O: IsA<View>> ViewManualExt for O {
    fn get_mark_attributes(&self, category: &str, priority: i32) -> Option<MarkAttributes> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_mark_attributes(
                self.as_ref().to_glib_none().0,
                category.to_glib_none().0,
                priority as *mut _,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn push_snippet<P: IsA<Snippet>>(&self, snippet: &P, mut location: Option<&mut gtk::TextIter>) {
        unsafe {
            ffi::gtk_source_view_push_snippet(
                self.as_ref().to_glib_none().0,
                snippet.as_ref().to_glib_none().0,
                location.to_glib_none_mut().0,
            );
        }
    }
}
