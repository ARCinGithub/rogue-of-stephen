#![doc = "Sidecar module for class [`StyleBoxTexture`][crate::classes::StyleBoxTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StyleBoxTexture` enums](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StyleBoxTexture`.\n\nInherits [`StyleBox`][crate::classes::StyleBox].\n\nRelated symbols:\n\n* [`style_box_texture`][crate::classes::style_box_texture]: sidecar module with related enum/flag types\n* [`IStyleBoxTexture`][crate::classes::IStyleBoxTexture]: virtual methods\n\n\nSee also [Godot docs for `StyleBoxTexture`](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StyleBoxTexture::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA texture-based nine-patch [`StyleBox`][crate::classes::StyleBox], in a way similar to [`NinePatchRect`][crate::classes::NinePatchRect]. This stylebox performs a 3×3 scaling of a texture, where only the center cell is fully stretched. This makes it possible to design bordered styles regardless of the stylebox's size."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StyleBoxTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`StyleBoxTexture`][crate::classes::StyleBoxTexture].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IStyleBox`][crate::classes::IStyleBox] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `StyleBoxTexture` methods](https://docs.godotengine.org/en/stable/classes/class_styleboxtexture.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStyleBoxTexture: crate::obj::GodotClass < Base = StyleBoxTexture > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn draw(&self, to_canvas_item: Rid, rect: Rect2,);
        fn get_draw_rect(&self, rect: Rect2,) -> Rect2 {
            unimplemented !()
        }
        #[doc = "Virtual method to be implemented by the user. Returns a custom minimum size that the stylebox must respect when drawing. By default [`get_minimum_size`][`crate::classes::StyleBox::get_minimum_size`] only takes content margins into account. This method can be overridden to add another size restriction. A combination of the default behavior and the output of this method will be used, to account for both sizes."]
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn test_mask(&self, point: Vector2, rect: Rect2,) -> bool {
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
    impl StyleBoxTexture {
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the margin to `size` pixels for the specified \\[enum Side]."]
        pub fn set_texture_margin(&mut self, margin: crate::builtin::Side, size: f32,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32,);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the margin to `size` pixels for all sides."]
        pub fn set_texture_margin_all(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_texture_margin_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the margin size of the specified \\[enum Side]."]
        pub fn get_texture_margin(&self, margin: crate::builtin::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::builtin::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_texture_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the expand margin to `size` pixels for the specified \\[enum Side]."]
        pub fn set_expand_margin(&mut self, margin: crate::builtin::Side, size: f32,) {
            type CallRet = ();
            type CallParams = (crate::builtin::Side, f32,);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_expand_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the expand margin to `size` pixels for all sides."]
        pub fn set_expand_margin_all(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_expand_margin_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the expand margin size of the specified \\[enum Side]."]
        pub fn get_expand_margin(&self, margin: crate::builtin::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::builtin::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_expand_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_region_rect(&mut self, region: Rect2,) {
            type CallRet = ();
            type CallParams = (Rect2,);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_region_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_region_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_region_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_center(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_draw_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_center_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "is_draw_center_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_axis_stretch_mode(&mut self, mode: crate::classes::style_box_texture::AxisStretchMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::style_box_texture::AxisStretchMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_h_axis_stretch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_axis_stretch_mode(&self,) -> crate::classes::style_box_texture::AxisStretchMode {
            type CallRet = crate::classes::style_box_texture::AxisStretchMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_h_axis_stretch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_axis_stretch_mode(&mut self, mode: crate::classes::style_box_texture::AxisStretchMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::style_box_texture::AxisStretchMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "set_v_axis_stretch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_axis_stretch_mode(&self,) -> crate::classes::style_box_texture::AxisStretchMode {
            type CallRet = crate::classes::style_box_texture::AxisStretchMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxTexture", "get_v_axis_stretch_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for StyleBoxTexture {
        type Base = crate::classes::StyleBox;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("StyleBoxTexture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StyleBoxTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StyleBox > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StyleBoxTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StyleBoxTexture {
        
    }
    impl crate::obj::cap::GodotDefault for StyleBoxTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StyleBoxTexture {
        type Target = crate::classes::StyleBox;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StyleBoxTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StyleBoxTexture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StyleBoxTexture__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBoxTexture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBox > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AxisStretchMode {
    ord: i32
}
impl AxisStretchMode {
    #[doc(alias = "AXIS_STRETCH_MODE_STRETCH")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_STRETCH`"]
    pub const STRETCH: AxisStretchMode = AxisStretchMode {
        ord: 0i32
    };
    #[doc(alias = "AXIS_STRETCH_MODE_TILE")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_TILE`"]
    pub const TILE: AxisStretchMode = AxisStretchMode {
        ord: 1i32
    };
    #[doc(alias = "AXIS_STRETCH_MODE_TILE_FIT")]
    #[doc = "Godot enumerator name: `AXIS_STRETCH_MODE_TILE_FIT`"]
    pub const TILE_FIT: AxisStretchMode = AxisStretchMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AxisStretchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AxisStretchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AxisStretchMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STRETCH => "STRETCH", Self::TILE => "TILE", Self::TILE_FIT => "TILE_FIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AxisStretchMode::STRETCH, AxisStretchMode::TILE, AxisStretchMode::TILE_FIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AxisStretchMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRETCH", "AXIS_STRETCH_MODE_STRETCH", AxisStretchMode::STRETCH), crate::meta::inspect::EnumConstant::new("TILE", "AXIS_STRETCH_MODE_TILE", AxisStretchMode::TILE), crate::meta::inspect::EnumConstant::new("TILE_FIT", "AXIS_STRETCH_MODE_TILE_FIT", AxisStretchMode::TILE_FIT)]
        }
    }
}
impl crate::meta::GodotConvert for AxisStretchMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Axis Stretch Mode Stretch", 0i64), EnumeratorShape::new_int("Axis Stretch Mode Tile", 1i64), EnumeratorShape::new_int("Axis Stretch Mode Tile Fit", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("StyleBoxTexture.AxisStretchMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AxisStretchMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AxisStretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AxisStretchMode {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* field)
    }
    fn var_set(field: &mut Self, value: Self::Via) {
        field.ord = value;
        
    }
    fn var_pub_get(field: &Self) -> Self::PubType {
        * field
    }
    fn var_pub_set(field: &mut Self, value: Self::PubType) {
        * field = value;
        
    }
}
impl crate::registry::property::Export for AxisStretchMode {
    
}
impl crate::meta::Element for AxisStretchMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StyleBoxTexture;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for StyleBoxTexture {
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