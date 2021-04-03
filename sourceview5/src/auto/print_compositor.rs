// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buffer;
use crate::View;
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
    pub struct PrintCompositor(Object<ffi::GtkSourcePrintCompositor, ffi::GtkSourcePrintCompositorClass>);

    match fn {
        get_type => || ffi::gtk_source_print_compositor_get_type(),
    }
}

impl PrintCompositor {
    #[doc(alias = "gtk_source_print_compositor_new")]
    pub fn new<P: IsA<Buffer>>(buffer: &P) -> PrintCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_new(
                buffer.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_source_print_compositor_new_from_view")]
    pub fn from_view<P: IsA<View>>(view: &P) -> PrintCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_new_from_view(
                view.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct PrintCompositorBuilder {
    body_font_name: Option<String>,
    buffer: Option<Buffer>,
    footer_font_name: Option<String>,
    header_font_name: Option<String>,
    highlight_syntax: Option<bool>,
    line_numbers_font_name: Option<String>,
    print_footer: Option<bool>,
    print_header: Option<bool>,
    print_line_numbers: Option<u32>,
    tab_width: Option<u32>,
    wrap_mode: Option<gtk::WrapMode>,
}

impl PrintCompositorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> PrintCompositor {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref body_font_name) = self.body_font_name {
            properties.push(("body-font-name", body_font_name));
        }
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref footer_font_name) = self.footer_font_name {
            properties.push(("footer-font-name", footer_font_name));
        }
        if let Some(ref header_font_name) = self.header_font_name {
            properties.push(("header-font-name", header_font_name));
        }
        if let Some(ref highlight_syntax) = self.highlight_syntax {
            properties.push(("highlight-syntax", highlight_syntax));
        }
        if let Some(ref line_numbers_font_name) = self.line_numbers_font_name {
            properties.push(("line-numbers-font-name", line_numbers_font_name));
        }
        if let Some(ref print_footer) = self.print_footer {
            properties.push(("print-footer", print_footer));
        }
        if let Some(ref print_header) = self.print_header {
            properties.push(("print-header", print_header));
        }
        if let Some(ref print_line_numbers) = self.print_line_numbers {
            properties.push(("print-line-numbers", print_line_numbers));
        }
        if let Some(ref tab_width) = self.tab_width {
            properties.push(("tab-width", tab_width));
        }
        if let Some(ref wrap_mode) = self.wrap_mode {
            properties.push(("wrap-mode", wrap_mode));
        }
        let ret = glib::Object::new::<PrintCompositor>(&properties).expect("object new");
        ret
    }

    pub fn body_font_name(mut self, body_font_name: &str) -> Self {
        self.body_font_name = Some(body_font_name.to_string());
        self
    }

    pub fn buffer<P: IsA<Buffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn footer_font_name(mut self, footer_font_name: &str) -> Self {
        self.footer_font_name = Some(footer_font_name.to_string());
        self
    }

    pub fn header_font_name(mut self, header_font_name: &str) -> Self {
        self.header_font_name = Some(header_font_name.to_string());
        self
    }

    pub fn highlight_syntax(mut self, highlight_syntax: bool) -> Self {
        self.highlight_syntax = Some(highlight_syntax);
        self
    }

    pub fn line_numbers_font_name(mut self, line_numbers_font_name: &str) -> Self {
        self.line_numbers_font_name = Some(line_numbers_font_name.to_string());
        self
    }

    pub fn print_footer(mut self, print_footer: bool) -> Self {
        self.print_footer = Some(print_footer);
        self
    }

    pub fn print_header(mut self, print_header: bool) -> Self {
        self.print_header = Some(print_header);
        self
    }

    pub fn print_line_numbers(mut self, print_line_numbers: u32) -> Self {
        self.print_line_numbers = Some(print_line_numbers);
        self
    }

    pub fn tab_width(mut self, tab_width: u32) -> Self {
        self.tab_width = Some(tab_width);
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: gtk::WrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }
}

pub const NONE_PRINT_COMPOSITOR: Option<&PrintCompositor> = None;

pub trait PrintCompositorExt: 'static {
    #[doc(alias = "gtk_source_print_compositor_draw_page")]
    fn draw_page(&self, context: &gtk::PrintContext, page_nr: i32);

    #[doc(alias = "gtk_source_print_compositor_get_body_font_name")]
    fn get_body_font_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_print_compositor_get_bottom_margin")]
    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64;

    #[doc(alias = "gtk_source_print_compositor_get_buffer")]
    fn get_buffer(&self) -> Option<Buffer>;

    #[doc(alias = "gtk_source_print_compositor_get_footer_font_name")]
    fn get_footer_font_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_print_compositor_get_header_font_name")]
    fn get_header_font_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_print_compositor_get_highlight_syntax")]
    fn get_highlight_syntax(&self) -> bool;

    #[doc(alias = "gtk_source_print_compositor_get_left_margin")]
    fn get_left_margin(&self, unit: gtk::Unit) -> f64;

    #[doc(alias = "gtk_source_print_compositor_get_line_numbers_font_name")]
    fn get_line_numbers_font_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_source_print_compositor_get_n_pages")]
    fn get_n_pages(&self) -> i32;

    #[doc(alias = "gtk_source_print_compositor_get_pagination_progress")]
    fn get_pagination_progress(&self) -> f64;

    #[doc(alias = "gtk_source_print_compositor_get_print_footer")]
    fn get_print_footer(&self) -> bool;

    #[doc(alias = "gtk_source_print_compositor_get_print_header")]
    fn get_print_header(&self) -> bool;

    #[doc(alias = "gtk_source_print_compositor_get_print_line_numbers")]
    fn get_print_line_numbers(&self) -> u32;

    #[doc(alias = "gtk_source_print_compositor_get_right_margin")]
    fn get_right_margin(&self, unit: gtk::Unit) -> f64;

    #[doc(alias = "gtk_source_print_compositor_get_tab_width")]
    fn get_tab_width(&self) -> u32;

    #[doc(alias = "gtk_source_print_compositor_get_top_margin")]
    fn get_top_margin(&self, unit: gtk::Unit) -> f64;

    #[doc(alias = "gtk_source_print_compositor_get_wrap_mode")]
    fn get_wrap_mode(&self) -> gtk::WrapMode;

    #[doc(alias = "gtk_source_print_compositor_paginate")]
    fn paginate(&self, context: &gtk::PrintContext) -> bool;

    #[doc(alias = "gtk_source_print_compositor_set_body_font_name")]
    fn set_body_font_name(&self, font_name: &str);

    #[doc(alias = "gtk_source_print_compositor_set_bottom_margin")]
    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit);

    #[doc(alias = "gtk_source_print_compositor_set_footer_font_name")]
    fn set_footer_font_name(&self, font_name: Option<&str>);

    #[doc(alias = "gtk_source_print_compositor_set_footer_format")]
    fn set_footer_format(
        &self,
        separator: bool,
        left: Option<&str>,
        center: Option<&str>,
        right: Option<&str>,
    );

    #[doc(alias = "gtk_source_print_compositor_set_header_font_name")]
    fn set_header_font_name(&self, font_name: Option<&str>);

    #[doc(alias = "gtk_source_print_compositor_set_header_format")]
    fn set_header_format(
        &self,
        separator: bool,
        left: Option<&str>,
        center: Option<&str>,
        right: Option<&str>,
    );

    #[doc(alias = "gtk_source_print_compositor_set_highlight_syntax")]
    fn set_highlight_syntax(&self, highlight: bool);

    #[doc(alias = "gtk_source_print_compositor_set_left_margin")]
    fn set_left_margin(&self, margin: f64, unit: gtk::Unit);

    #[doc(alias = "gtk_source_print_compositor_set_line_numbers_font_name")]
    fn set_line_numbers_font_name(&self, font_name: Option<&str>);

    #[doc(alias = "gtk_source_print_compositor_set_print_footer")]
    fn set_print_footer(&self, print: bool);

    #[doc(alias = "gtk_source_print_compositor_set_print_header")]
    fn set_print_header(&self, print: bool);

    #[doc(alias = "gtk_source_print_compositor_set_print_line_numbers")]
    fn set_print_line_numbers(&self, interval: u32);

    #[doc(alias = "gtk_source_print_compositor_set_right_margin")]
    fn set_right_margin(&self, margin: f64, unit: gtk::Unit);

    #[doc(alias = "gtk_source_print_compositor_set_tab_width")]
    fn set_tab_width(&self, width: u32);

    #[doc(alias = "gtk_source_print_compositor_set_top_margin")]
    fn set_top_margin(&self, margin: f64, unit: gtk::Unit);

    #[doc(alias = "gtk_source_print_compositor_set_wrap_mode")]
    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode);

    fn connect_property_body_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_footer_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_header_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_highlight_syntax_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_line_numbers_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_footer_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_print_header_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_print_line_numbers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintCompositor>> PrintCompositorExt for O {
    fn draw_page(&self, context: &gtk::PrintContext, page_nr: i32) {
        unsafe {
            ffi::gtk_source_print_compositor_draw_page(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                page_nr,
            );
        }
    }

    fn get_body_font_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_body_font_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_bottom_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_bottom_margin(
                self.as_ref().to_glib_none().0,
                unit.to_glib(),
            )
        }
    }

    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_print_compositor_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_footer_font_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_footer_font_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_header_font_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_header_font_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_highlight_syntax(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_highlight_syntax(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_left_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_left_margin(
                self.as_ref().to_glib_none().0,
                unit.to_glib(),
            )
        }
    }

    fn get_line_numbers_font_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_source_print_compositor_get_line_numbers_font_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe { ffi::gtk_source_print_compositor_get_n_pages(self.as_ref().to_glib_none().0) }
    }

    fn get_pagination_progress(&self) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_pagination_progress(self.as_ref().to_glib_none().0)
        }
    }

    fn get_print_footer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_print_footer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_print_header(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_print_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_print_line_numbers(&self) -> u32 {
        unsafe {
            ffi::gtk_source_print_compositor_get_print_line_numbers(self.as_ref().to_glib_none().0)
        }
    }

    fn get_right_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_right_margin(
                self.as_ref().to_glib_none().0,
                unit.to_glib(),
            )
        }
    }

    fn get_tab_width(&self) -> u32 {
        unsafe { ffi::gtk_source_print_compositor_get_tab_width(self.as_ref().to_glib_none().0) }
    }

    fn get_top_margin(&self, unit: gtk::Unit) -> f64 {
        unsafe {
            ffi::gtk_source_print_compositor_get_top_margin(
                self.as_ref().to_glib_none().0,
                unit.to_glib(),
            )
        }
    }

    fn get_wrap_mode(&self) -> gtk::WrapMode {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_get_wrap_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn paginate(&self, context: &gtk::PrintContext) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_print_compositor_paginate(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    fn set_body_font_name(&self, font_name: &str) {
        unsafe {
            ffi::gtk_source_print_compositor_set_body_font_name(
                self.as_ref().to_glib_none().0,
                font_name.to_glib_none().0,
            );
        }
    }

    fn set_bottom_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_bottom_margin(
                self.as_ref().to_glib_none().0,
                margin,
                unit.to_glib(),
            );
        }
    }

    fn set_footer_font_name(&self, font_name: Option<&str>) {
        unsafe {
            ffi::gtk_source_print_compositor_set_footer_font_name(
                self.as_ref().to_glib_none().0,
                font_name.to_glib_none().0,
            );
        }
    }

    fn set_footer_format(
        &self,
        separator: bool,
        left: Option<&str>,
        center: Option<&str>,
        right: Option<&str>,
    ) {
        unsafe {
            ffi::gtk_source_print_compositor_set_footer_format(
                self.as_ref().to_glib_none().0,
                separator.to_glib(),
                left.to_glib_none().0,
                center.to_glib_none().0,
                right.to_glib_none().0,
            );
        }
    }

    fn set_header_font_name(&self, font_name: Option<&str>) {
        unsafe {
            ffi::gtk_source_print_compositor_set_header_font_name(
                self.as_ref().to_glib_none().0,
                font_name.to_glib_none().0,
            );
        }
    }

    fn set_header_format(
        &self,
        separator: bool,
        left: Option<&str>,
        center: Option<&str>,
        right: Option<&str>,
    ) {
        unsafe {
            ffi::gtk_source_print_compositor_set_header_format(
                self.as_ref().to_glib_none().0,
                separator.to_glib(),
                left.to_glib_none().0,
                center.to_glib_none().0,
                right.to_glib_none().0,
            );
        }
    }

    fn set_highlight_syntax(&self, highlight: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_highlight_syntax(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    fn set_left_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_left_margin(
                self.as_ref().to_glib_none().0,
                margin,
                unit.to_glib(),
            );
        }
    }

    fn set_line_numbers_font_name(&self, font_name: Option<&str>) {
        unsafe {
            ffi::gtk_source_print_compositor_set_line_numbers_font_name(
                self.as_ref().to_glib_none().0,
                font_name.to_glib_none().0,
            );
        }
    }

    fn set_print_footer(&self, print: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_footer(
                self.as_ref().to_glib_none().0,
                print.to_glib(),
            );
        }
    }

    fn set_print_header(&self, print: bool) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_header(
                self.as_ref().to_glib_none().0,
                print.to_glib(),
            );
        }
    }

    fn set_print_line_numbers(&self, interval: u32) {
        unsafe {
            ffi::gtk_source_print_compositor_set_print_line_numbers(
                self.as_ref().to_glib_none().0,
                interval,
            );
        }
    }

    fn set_right_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_right_margin(
                self.as_ref().to_glib_none().0,
                margin,
                unit.to_glib(),
            );
        }
    }

    fn set_tab_width(&self, width: u32) {
        unsafe {
            ffi::gtk_source_print_compositor_set_tab_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_top_margin(&self, margin: f64, unit: gtk::Unit) {
        unsafe {
            ffi::gtk_source_print_compositor_set_top_margin(
                self.as_ref().to_glib_none().0,
                margin,
                unit.to_glib(),
            );
        }
    }

    fn set_wrap_mode(&self, wrap_mode: gtk::WrapMode) {
        unsafe {
            ffi::gtk_source_print_compositor_set_wrap_mode(
                self.as_ref().to_glib_none().0,
                wrap_mode.to_glib(),
            );
        }
    }

    fn connect_property_body_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_body_font_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::body-font-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_body_font_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_footer_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_footer_font_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::footer-font-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_footer_font_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_header_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_font_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::header-font-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_header_font_name_trampoline::<Self, F> as *const (),
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
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_line_numbers_font_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_line_numbers_font_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::line-numbers-font-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_line_numbers_font_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_n_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_pages_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_pages_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_print_footer_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_print_footer_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::print-footer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_print_footer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_print_header_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_print_header_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::print-header\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_print_header_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_print_line_numbers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_print_line_numbers_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::print-line-numbers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_print_line_numbers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tab_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tab-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tab_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_wrap_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourcePrintCompositor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<PrintCompositor>,
        {
            let f: &F = &*(f as *const F);
            f(&PrintCompositor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PrintCompositor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintCompositor")
    }
}
