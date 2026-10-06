#![doc = "Sidecar module for class [`DpiTexture`][crate::classes::DpiTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DPITexture` enums](https://docs.godotengine.org/en/stable/classes/class_dpitexture.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RawPtr, RefArg
};
use crate::private::Signature;
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `DPITexture`.\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`dpi_texture`][crate::classes::dpi_texture]: sidecar module with related enum/flag types\n* [`IDpiTexture`][crate::classes::IDpiTexture]: virtual methods\n\n\nSee also [Godot docs for `DPITexture`](https://docs.godotengine.org/en/stable/classes/class_dpitexture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`DpiTexture::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAn automatically scalable [`Texture2D`][crate::classes::Texture2D] based on an SVG image. `DPITexture`s are used to automatically re-rasterize icons and other texture based UI theme elements to match viewport scale and font oversampling. See also \\[member ProjectSettings.display/window/stretch/mode] (\"canvas_items\" mode) and \\[member Viewport.oversampling_override]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DpiTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`DpiTexture`][crate::classes::DpiTexture].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture2D`][crate::classes::ITexture2D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `DPITexture` methods](https://docs.godotengine.org/en/stable/classes/class_dpitexture.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDpiTexture: crate::obj::GodotClass < Base = DpiTexture > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Called when the `Texture2D`'s width is queried."]
        fn get_width(&self,) -> i32;
        #[doc = "Called when the `Texture2D`'s height is queried."]
        fn get_height(&self,) -> i32;
        #[doc = "Called when a pixel's opaque state in the `Texture2D` is queried at the specified `(x, y)` position."]
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        #[doc = "Called when the presence of an alpha channel in the `Texture2D` is queried."]
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Called when the entire `Texture2D` is requested to be drawn over a [`CanvasItem`][crate::classes::CanvasItem], with the top-left offset specified in `pos`. `modulate` specifies a multiplier for the colors being drawn, while `transpose` specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).\n\n**Note:** This is only used in 2D rendering, not 3D."]
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        #[doc = "Called when the `Texture2D` is requested to be drawn onto [`CanvasItem`][crate::classes::CanvasItem]'s specified `rect`. `modulate` specifies a multiplier for the colors being drawn, while `transpose` specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).\n\n**Note:** This is only used in 2D rendering, not 3D."]
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        #[doc = "Called when a part of the `Texture2D` specified by `src_rect`'s coordinates is requested to be drawn onto [`CanvasItem`][crate::classes::CanvasItem]'s specified `rect`. `modulate` specifies a multiplier for the colors being drawn, while `transpose` specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).\n\n**Note:** This is only used in 2D rendering, not 3D."]
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            unimplemented !()
        }
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl DpiTexture {
        #[doc = "Creates a new `DPITexture` and initializes it by allocating and setting the SVG data to `source`."]
        pub(crate) fn create_from_string_full(source: CowArg < GString >, scale: f32, saturation: f32, color_map: RefArg < AnyDictionary >,) -> Option < Gd < crate::classes::DpiTexture > > {
            type CallRet = Option < Gd < crate::classes::DpiTexture > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, f32, f32, RefArg < 'a1, AnyDictionary >,);
            let args = (source, scale, saturation, color_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "create_from_string", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_from_string_ex`][Self::create_from_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new `DPITexture` and initializes it by allocating and setting the SVG data to `source`."]
        #[inline]
        pub fn create_from_string(source: impl AsArg < GString >,) -> Option < Gd < crate::classes::DpiTexture > > {
            Self::create_from_string_ex(source,) . done()
        }
        #[doc = "Creates a new `DPITexture` and initializes it by allocating and setting the SVG data to `source`."]
        #[inline]
        pub fn create_from_string_ex < 'ex > (source: impl AsArg < GString > + 'ex,) -> ExCreateFromString < 'ex > {
            ExCreateFromString::new(source,)
        }
        #[doc = "Sets this SVG texture's source code."]
        pub fn set_source(&mut self, source: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "set_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this SVG texture's source code."]
        pub fn get_source(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "get_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_base_scale(&mut self, base_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (base_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "set_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "get_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_saturation(&mut self, saturation: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "set_saturation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_saturation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "get_saturation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_map(&mut self, color_map: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(color_map),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "set_color_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_map(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "get_color_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resizes the texture to the specified dimensions."]
        pub fn set_size_override(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "set_size_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the texture rasterized to match the oversampling of the currently drawn canvas item."]
        pub fn get_scaled_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DpiTexture", "get_scaled_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" Creates a validated object for FFI boundary crossing."]
        #[doc = r""]
        #[doc = r" Low-level internal method. Validation (liveness/type checks) depend on safeguard level."]
        fn __validated_obj(&self) -> crate::obj::ValidatedObject {
            let raw_gd = unsafe {
                std::mem::transmute::< &Self, &crate::obj::RawGd < Self >> (self)
            };
            raw_gd.validated_object()
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for DpiTexture {
        type Base = crate::classes::Texture2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DPITexture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DpiTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for DpiTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for DpiTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for DpiTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for DpiTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DpiTexture {
        
    }
    impl crate::obj::cap::GodotDefault for DpiTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for DpiTexture {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DpiTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`DpiTexture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DpiTexture__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::DpiTexture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`DpiTexture::create_from_string_ex`][super::DpiTexture::create_from_string_ex]."]
#[must_use]
pub struct ExCreateFromString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, source: CowArg < 'ex, GString >, scale: f32, saturation: f32, color_map: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateFromString < 'ex > {
    fn new(source: impl AsArg < GString > + 'ex,) -> Self {
        let scale = 1f32;
        let saturation = 1f32;
        let color_map = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, source: source.into_arg(), scale: scale, saturation: saturation, color_map: CowArg::Owned(color_map),
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn saturation(self, saturation: f32) -> Self {
        Self {
            saturation: saturation, .. self
        }
    }
    #[inline]
    pub fn color_map(self, color_map: &'ex AnyDictionary) -> Self {
        Self {
            color_map: CowArg::Borrowed(color_map), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::DpiTexture > > {
        let Self {
            _phantom, source, scale, saturation, color_map,
        }
        = self;
        re_export::DpiTexture::create_from_string_full(source, scale, saturation, color_map.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DpiTexture;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for DpiTexture {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}