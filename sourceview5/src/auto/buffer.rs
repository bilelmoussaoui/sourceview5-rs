// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::BracketMatchType;
use crate::ChangeCaseType;
use crate::Language;
use crate::Mark;
use crate::SortFlags;
use crate::StyleScheme;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Buffer(Object<ffi::GtkSourceBuffer, ffi::GtkSourceBufferClass>) @extends gtk::TextBuffer;

    match fn {
        get_type => || ffi::gtk_source_buffer_get_type(),
    }
}

impl Buffer {
    #[doc(alias = "gtk_source_buffer_new")]
    pub fn new(table: Option<&gtk::TextTagTable>) -> Buffer {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_source_buffer_new(table.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_source_buffer_new_with_language")]
    pub fn with_language(language: &Language) -> Buffer {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_buffer_new_with_language(
                language.to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct BufferBuilder {
    highlight_matching_brackets: Option<bool>,
    highlight_syntax: Option<bool>,
    implicit_trailing_newline: Option<bool>,
    language: Option<Language>,
    style_scheme: Option<StyleScheme>,
    enable_undo: Option<bool>,
    tag_table: Option<gtk::TextTagTable>,
    text: Option<String>,
}

impl BufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Buffer {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref highlight_matching_brackets) = self.highlight_matching_brackets {
            properties.push(("highlight-matching-brackets", highlight_matching_brackets));
        }
        if let Some(ref highlight_syntax) = self.highlight_syntax {
            properties.push(("highlight-syntax", highlight_syntax));
        }
        if let Some(ref implicit_trailing_newline) = self.implicit_trailing_newline {
            properties.push(("implicit-trailing-newline", implicit_trailing_newline));
        }
        if let Some(ref language) = self.language {
            properties.push(("language", language));
        }
        if let Some(ref style_scheme) = self.style_scheme {
            properties.push(("style-scheme", style_scheme));
        }
        if let Some(ref enable_undo) = self.enable_undo {
            properties.push(("enable-undo", enable_undo));
        }
        if let Some(ref tag_table) = self.tag_table {
            properties.push(("tag-table", tag_table));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        let ret = glib::Object::new::<Buffer>(&properties).expect("object new");
        ret
    }

    pub fn highlight_matching_brackets(mut self, highlight_matching_brackets: bool) -> Self {
        self.highlight_matching_brackets = Some(highlight_matching_brackets);
        self
    }

    pub fn highlight_syntax(mut self, highlight_syntax: bool) -> Self {
        self.highlight_syntax = Some(highlight_syntax);
        self
    }

    pub fn implicit_trailing_newline(mut self, implicit_trailing_newline: bool) -> Self {
        self.implicit_trailing_newline = Some(implicit_trailing_newline);
        self
    }

    pub fn language(mut self, language: &Language) -> Self {
        self.language = Some(language.clone());
        self
    }

    pub fn style_scheme(mut self, style_scheme: &StyleScheme) -> Self {
        self.style_scheme = Some(style_scheme.clone());
        self
    }

    pub fn enable_undo(mut self, enable_undo: bool) -> Self {
        self.enable_undo = Some(enable_undo);
        self
    }

    pub fn tag_table(mut self, tag_table: &gtk::TextTagTable) -> Self {
        self.tag_table = Some(tag_table.clone());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

pub const NONE_BUFFER: Option<&Buffer> = None;

pub trait BufferExt: 'static {
    //#[doc(alias = "gtk_source_buffer_backward_iter_to_source_mark")]
    //fn backward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool;

    #[doc(alias = "gtk_source_buffer_change_case")]
    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    );

    #[doc(alias = "gtk_source_buffer_create_source_mark")]
    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark>;

    //#[doc(alias = "gtk_source_buffer_create_source_tag")]
    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag>;

    #[doc(alias = "gtk_source_buffer_ensure_highlight")]
    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter);

    //#[doc(alias = "gtk_source_buffer_forward_iter_to_source_mark")]
    //fn forward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool;

    #[doc(alias = "gtk_source_buffer_get_context_classes_at_iter")]
    fn get_context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<glib::GString>;

    #[doc(alias = "gtk_source_buffer_get_highlight_matching_brackets")]
    fn get_highlight_matching_brackets(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_highlight_syntax")]
    fn get_highlight_syntax(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_implicit_trailing_newline")]
    fn get_implicit_trailing_newline(&self) -> bool;

    #[doc(alias = "gtk_source_buffer_get_language")]
    fn get_language(&self) -> Option<Language>;

    #[doc(alias = "gtk_source_buffer_get_source_marks_at_iter")]
    fn get_source_marks_at_iter(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> Vec<Mark>;

    #[doc(alias = "gtk_source_buffer_get_source_marks_at_line")]
    fn get_source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark>;

    #[doc(alias = "gtk_source_buffer_get_style_scheme")]
    fn get_style_scheme(&self) -> Option<StyleScheme>;

    //#[doc(alias = "gtk_source_buffer_iter_backward_to_context_class_toggle")]
    //fn iter_backward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool;

    //#[doc(alias = "gtk_source_buffer_iter_forward_to_context_class_toggle")]
    //fn iter_forward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool;

    #[doc(alias = "gtk_source_buffer_iter_has_context_class")]
    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool;

    #[doc(alias = "gtk_source_buffer_join_lines")]
    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter);

    #[doc(alias = "gtk_source_buffer_remove_source_marks")]
    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    );

    #[doc(alias = "gtk_source_buffer_set_highlight_matching_brackets")]
    fn set_highlight_matching_brackets(&self, highlight: bool);

    #[doc(alias = "gtk_source_buffer_set_highlight_syntax")]
    fn set_highlight_syntax(&self, highlight: bool);

    #[doc(alias = "gtk_source_buffer_set_implicit_trailing_newline")]
    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool);

    #[doc(alias = "gtk_source_buffer_set_language")]
    fn set_language(&self, language: Option<&Language>);

    #[doc(alias = "gtk_source_buffer_set_style_scheme")]
    fn set_style_scheme(&self, scheme: Option<&StyleScheme>);

    #[doc(alias = "gtk_source_buffer_sort_lines")]
    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    );

    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_cursor_moved<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Buffer>> BufferExt for O {
    //fn backward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_backward_iter_to_source_mark() }
    //}

    fn change_case(
        &self,
        case_type: ChangeCaseType,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
    ) {
        unsafe {
            ffi::gtk_source_buffer_change_case(
                self.as_ref().to_glib_none().0,
                case_type.to_glib(),
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn create_source_mark(
        &self,
        name: Option<&str>,
        category: &str,
        where_: &gtk::TextIter,
    ) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_create_source_mark(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                category.to_glib_none().0,
                where_.to_glib_none().0,
            ))
        }
    }

    //fn create_source_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<gtk::TextTag> {
    //    unsafe { TODO: call ffi:gtk_source_buffer_create_source_tag() }
    //}

    fn ensure_highlight(&self, start: &gtk::TextIter, end: &gtk::TextIter) {
        unsafe {
            ffi::gtk_source_buffer_ensure_highlight(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
            );
        }
    }

    //fn forward_iter_to_source_mark(&self, iter: /*Unimplemented*/gtk::TextIter, category: Option<&str>) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_forward_iter_to_source_mark() }
    //}

    fn get_context_classes_at_iter(&self, iter: &gtk::TextIter) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                ffi::gtk_source_buffer_get_context_classes_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none().0,
                ),
            )
        }
    }

    fn get_highlight_matching_brackets(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_matching_brackets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_highlight_syntax(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_implicit_trailing_newline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_get_implicit_trailing_newline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_language(&self) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_source_marks_at_iter(
        &self,
        iter: &mut gtk::TextIter,
        category: Option<&str>,
    ) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::gtk_source_buffer_get_source_marks_at_iter(
                    self.as_ref().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn get_source_marks_at_line(&self, line: i32, category: Option<&str>) -> Vec<Mark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::gtk_source_buffer_get_source_marks_at_line(
                    self.as_ref().to_glib_none().0,
                    line,
                    category.to_glib_none().0,
                ),
            )
        }
    }

    fn get_style_scheme(&self) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_buffer_get_style_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn iter_backward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_iter_backward_to_context_class_toggle() }
    //}

    //fn iter_forward_to_context_class_toggle(&self, iter: /*Unimplemented*/gtk::TextIter, context_class: &str) -> bool {
    //    unsafe { TODO: call ffi:gtk_source_buffer_iter_forward_to_context_class_toggle() }
    //}

    fn iter_has_context_class(&self, iter: &gtk::TextIter, context_class: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_buffer_iter_has_context_class(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                context_class.to_glib_none().0,
            ))
        }
    }

    fn join_lines(&self, start: &mut gtk::TextIter, end: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_buffer_join_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
            );
        }
    }

    fn remove_source_marks(
        &self,
        start: &gtk::TextIter,
        end: &gtk::TextIter,
        category: Option<&str>,
    ) {
        unsafe {
            ffi::gtk_source_buffer_remove_source_marks(
                self.as_ref().to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                category.to_glib_none().0,
            );
        }
    }

    fn set_highlight_matching_brackets(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_matching_brackets(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_highlight_syntax(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    fn set_implicit_trailing_newline(&self, implicit_trailing_newline: bool) {
        unsafe {
            ffi::gtk_source_buffer_set_implicit_trailing_newline(
                self.as_ref().to_glib_none().0,
                implicit_trailing_newline.to_glib(),
            );
        }
    }

    fn set_language(&self, language: Option<&Language>) {
        unsafe {
            ffi::gtk_source_buffer_set_language(
                self.as_ref().to_glib_none().0,
                language.to_glib_none().0,
            );
        }
    }

    fn set_style_scheme(&self, scheme: Option<&StyleScheme>) {
        unsafe {
            ffi::gtk_source_buffer_set_style_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn sort_lines(
        &self,
        start: &mut gtk::TextIter,
        end: &mut gtk::TextIter,
        flags: SortFlags,
        column: i32,
    ) {
        unsafe {
            ffi::gtk_source_buffer_sort_lines(
                self.as_ref().to_glib_none().0,
                start.to_glib_none_mut().0,
                end.to_glib_none_mut().0,
                flags.to_glib(),
                column,
            );
        }
    }

    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn bracket_matched_trampoline<
            P,
            F: Fn(&P, Option<&gtk::TextIter>, BracketMatchType) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            iter: *mut gtk::ffi::GtkTextIter,
            state: ffi::GtkSourceBracketMatchType,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gtk::TextIter>::from_glib_borrow(iter)
                    .as_ref()
                    .as_ref(),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bracket-matched\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bracket_matched_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn connect_cursor_moved<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cursor_moved_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cursor-moved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cursor_moved_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_highlight_updated<F: Fn(&Self, &gtk::TextIter, &gtk::TextIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn highlight_updated_trampoline<
            P,
            F: Fn(&P, &gtk::TextIter, &gtk::TextIter) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            start: *mut gtk::ffi::GtkTextIter,
            end: *mut gtk::ffi::GtkTextIter,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(start),
                &from_glib_borrow(end),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"highlight-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    highlight_updated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_source_mark_updated<F: Fn(&Self, &gtk::TextMark) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn source_mark_updated_trampoline<
            P,
            F: Fn(&P, &gtk::TextMark) + 'static,
        >(
            this: *mut ffi::GtkSourceBuffer,
            mark: *mut gtk::ffi::GtkTextMark,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(mark),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"source-mark-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    source_mark_updated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_highlight_matching_brackets_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_matching_brackets_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-matching-brackets\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_matching_brackets_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_syntax_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight-syntax\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_syntax_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_implicit_trailing_newline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_implicit_trailing_newline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::implicit-trailing-newline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_implicit_trailing_newline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::language\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_language_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_style_scheme_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_scheme_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceBuffer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(&Buffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-scheme\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_style_scheme_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Buffer")
    }
}
