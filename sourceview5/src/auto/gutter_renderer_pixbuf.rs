// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::GutterRenderer;
use crate::GutterRendererAlignmentMode;
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
    pub struct GutterRendererPixbuf(Object<ffi::GtkSourceGutterRendererPixbuf, ffi::GtkSourceGutterRendererPixbufClass>) @extends GutterRenderer, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        get_type => || ffi::gtk_source_gutter_renderer_pixbuf_get_type(),
    }
}

impl GutterRendererPixbuf {
    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_new")]
    pub fn new() -> GutterRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            GutterRenderer::from_glib_full(ffi::gtk_source_gutter_renderer_pixbuf_new())
                .unsafe_cast()
        }
    }
}

impl Default for GutterRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct GutterRendererPixbufBuilder {
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    paintable: Option<gdk::Paintable>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    alignment_mode: Option<GutterRendererAlignmentMode>,
    xalign: Option<f32>,
    xpad: Option<i32>,
    yalign: Option<f32>,
    ypad: Option<i32>,
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

impl GutterRendererPixbufBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GutterRendererPixbuf {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref paintable) = self.paintable {
            properties.push(("paintable", paintable));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        if let Some(ref alignment_mode) = self.alignment_mode {
            properties.push(("alignment-mode", alignment_mode));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
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
        let ret = glib::Object::new::<GutterRendererPixbuf>(&properties).expect("object new");
        ret
    }

    pub fn gicon<P: IsA<gio::Icon>>(mut self, gicon: &P) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn paintable<P: IsA<gdk::Paintable>>(mut self, paintable: &P) -> Self {
        self.paintable = Some(paintable.clone().upcast());
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }

    pub fn alignment_mode(mut self, alignment_mode: GutterRendererAlignmentMode) -> Self {
        self.alignment_mode = Some(alignment_mode);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: i32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: i32) -> Self {
        self.ypad = Some(ypad);
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

pub const NONE_GUTTER_RENDERER_PIXBUF: Option<&GutterRendererPixbuf> = None;

pub trait GutterRendererPixbufExt: 'static {
    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_gicon")]
    fn get_gicon(&self) -> Option<gio::Icon>;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_icon_name")]
    fn get_icon_name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_paintable")]
    fn get_paintable(&self) -> Option<gdk::Paintable>;

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_get_pixbuf")]
    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_overlay_paintable")]
    fn overlay_paintable<P: IsA<gdk::Paintable>>(&self, paintable: &P);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_gicon")]
    fn set_gicon<P: IsA<gio::Icon>>(&self, icon: Option<&P>);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_icon_name")]
    fn set_icon_name(&self, icon_name: Option<&str>);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_paintable")]
    fn set_paintable<P: IsA<gdk::Paintable>>(&self, paintable: Option<&P>);

    #[doc(alias = "gtk_source_gutter_renderer_pixbuf_set_pixbuf")]
    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_paintable(&self) -> Option<gdk::Paintable>;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GutterRendererPixbuf>> GutterRendererPixbufExt for O {
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_gicon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn get_paintable(&self) -> Option<gdk::Paintable> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_paintable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_gutter_renderer_pixbuf_get_pixbuf(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v5_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
    fn overlay_paintable<P: IsA<gdk::Paintable>>(&self, paintable: &P) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_overlay_paintable(
                self.as_ref().to_glib_none().0,
                paintable.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_gicon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_gicon(
                self.as_ref().to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    fn set_paintable<P: IsA<gdk::Paintable>>(&self, paintable: Option<&P>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_paintable(
                self.as_ref().to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_source_gutter_renderer_pixbuf_set_pixbuf(
                self.as_ref().to_glib_none().0,
                pixbuf.to_glib_none().0,
            );
        }
    }

    fn get_property_paintable(&self) -> Option<gdk::Paintable> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::Paintable as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"paintable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `paintable` getter")
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSourceGutterRendererPixbuf,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GutterRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&GutterRendererPixbuf::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GutterRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GutterRendererPixbuf")
    }
}
