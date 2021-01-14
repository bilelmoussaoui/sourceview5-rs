// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v5_0", feature = "dox"))]
use crate::GutterLines;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v5_0", feature = "dox"))]
use glib::IsA;

pub trait GutterLinesManualExt {
    fn get_yrange(&self, line: u32, line_y: u32, line_height: u32);
}

#[cfg(any(feature = "v5_0", feature = "dox"))]
impl<O: IsA<GutterLines>> GutterLinesManualExt for O {
    fn get_yrange(&self, line: u32, line_y: u32, line_height: u32) {
        unsafe {
            ffi::gtk_source_gutter_lines_get_yrange(
                self.as_ref().to_glib_none().0,
                line,
                line_y as *mut _,
                line_height as *mut _,
            );
        }
    }
}
