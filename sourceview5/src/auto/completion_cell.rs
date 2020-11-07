// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use CompletionColumn;

glib_wrapper! {
    pub struct CompletionCell(Object<gtk_source_sys::GtkSourceCompletionCell, gtk_source_sys::GtkSourceCompletionCellClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        get_type => || gtk_source_sys::gtk_source_completion_cell_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct CompletionCellBuilder {
    column: Option<CompletionColumn>,
    markup: Option<String>,
    paintable: Option<gdk::Paintable>,
    text: Option<String>,
    widget: Option<gtk::Widget>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    //cursor: /*Unknown type*/,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    //halign: /*Unknown type*/,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    //layout-manager: /*Unknown type*/,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    //overflow: /*Unknown type*/,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    //accessible-role: /*Unknown type*/,
}

impl CompletionCellBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CompletionCell {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref column) = self.column {
            properties.push(("column", column));
        }
        if let Some(ref markup) = self.markup {
            properties.push(("markup", markup));
        }
        if let Some(ref paintable) = self.paintable {
            properties.push(("paintable", paintable));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        let ret = glib::Object::new(CompletionCell::static_type(), &properties)
            .expect("object new")
            .downcast::<CompletionCell>()
            .expect("downcast");
        ret
    }

    pub fn column(mut self, column: CompletionColumn) -> Self {
        self.column = Some(column);
        self
    }

    pub fn markup(mut self, markup: &str) -> Self {
        self.markup = Some(markup.to_string());
        self
    }

    pub fn paintable<P: IsA<gdk::Paintable>>(mut self, paintable: &P) -> Self {
        self.paintable = Some(paintable.clone().upcast());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn widget<P: IsA<gtk::Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_COMPLETION_CELL: Option<&CompletionCell> = None;

pub trait CompletionCellExt: 'static {
    fn get_column(&self) -> CompletionColumn;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget>;

    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: &P);

    fn set_icon_name(&self, icon_name: &str);

    fn set_markup(&self, markup: &str);

    fn set_paintable<P: IsA<gdk::Paintable>>(&self, paintable: &P);

    fn set_text(&self, text: &str);

    //fn set_text_with_attributes(&self, text: &str, attrs: /*Ignored*/&pango::AttrList);

    fn set_widget<P: IsA<gtk::Widget>>(&self, child: &P);

    fn get_property_markup(&self) -> Option<GString>;

    fn get_property_paintable(&self) -> Option<gdk::Paintable>;

    fn get_property_text(&self) -> Option<GString>;

    fn get_property_widget(&self) -> Option<gtk::Widget>;

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionCell>> CompletionCellExt for O {
    fn get_column(&self) -> CompletionColumn {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_completion_cell_get_column(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_cell_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: &P) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_gicon(
                self.as_ref().to_glib_none().0,
                gicon.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    fn set_markup(&self, markup: &str) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_markup(
                self.as_ref().to_glib_none().0,
                markup.to_glib_none().0,
            );
        }
    }

    fn set_paintable<P: IsA<gdk::Paintable>>(&self, paintable: &P) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_paintable(
                self.as_ref().to_glib_none().0,
                paintable.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    //fn set_text_with_attributes(&self, text: &str, attrs: /*Ignored*/&pango::AttrList) {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_completion_cell_set_text_with_attributes() }
    //}

    fn set_widget<P: IsA<gtk::Widget>>(&self, child: &P) {
        unsafe {
            gtk_source_sys::gtk_source_completion_cell_set_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_property_markup(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"markup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `markup` getter")
        }
    }

    fn get_property_paintable(&self) -> Option<gdk::Paintable> {
        unsafe {
            let mut value = Value::from_type(<gdk::Paintable as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"paintable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `paintable` getter")
        }
    }

    fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    fn get_property_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            let mut value = Value::from_type(<gtk::Widget as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"widget\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `widget` getter")
        }
    }

    fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_markup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionCell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionCell>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionCell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_markup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionCell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionCell>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionCell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::paintable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionCell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionCell>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionCell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceCompletionCell,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CompletionCell>,
        {
            let f: &F = &*(f as *const F);
            f(&CompletionCell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CompletionCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompletionCell")
    }
}
