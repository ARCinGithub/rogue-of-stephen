#![doc = "Sidecar module for class [`Texture2D`][crate::classes::Texture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Texture2D` enums](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Texture2D`.\n\nInherits [`Texture`][crate::classes::Texture].\n\nRelated symbols:\n\n* [`texture_2d`][crate::classes::texture_2d]: sidecar module with related enum/flag types\n* [`ITexture2D`][crate::classes::ITexture2D]: virtual methods\n\n\nSee also [Godot docs for `Texture2D`](https://docs.godotengine.org/en/stable/classes/class_texture2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Texture2D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA texture works by registering an image in the video hardware, which then can be used in 3D models or 2D [`Sprite2D`][crate::classes::Sprite2D] or GUI [`Control`][crate::classes::Control].\n\nTextures are often created by loading them from a file. See [`load`][`crate::tools::load`].\n\n`Texture2D` is a base for other resources. It cannot be used directly.\n\n**Note:** The maximum texture size is 16384×16384 pixels due to graphics hardware limitations. Larger textures may fail to import."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Texture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Texture2D`][crate::classes::Texture2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Texture2D` methods](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITexture2D: crate::obj::GodotClass < Base = Texture2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Texture2D {
        #[doc = "Returns the texture width in pixels."]
        pub fn get_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the texture height in pixels."]
        pub fn get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the texture size in pixels."]
        pub fn get_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this `Texture2D` has an alpha channel."]
        pub fn has_alpha(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "has_alpha", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API at the specified `position`."]
        pub(crate) fn draw_full(&self, canvas_item: Rid, position: Vector2, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Color, bool,);
            let args = (canvas_item, position, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_ex`][Self::draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API at the specified `position`."]
        #[inline]
        pub fn draw(&self, canvas_item: Rid, position: Vector2,) {
            self.draw_ex(canvas_item, position,) . done()
        }
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API at the specified `position`."]
        #[inline]
        pub fn draw_ex < 'ex > (&'ex self, canvas_item: Rid, position: Vector2,) -> ExDraw < 'ex > {
            ExDraw::new(self, canvas_item, position,)
        }
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        pub(crate) fn draw_rect_full(&self, canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, bool, Color, bool,);
            let args = (canvas_item, rect, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "draw_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_rect_ex`][Self::draw_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        #[inline]
        pub fn draw_rect(&self, canvas_item: Rid, rect: Rect2, tile: bool,) {
            self.draw_rect_ex(canvas_item, rect, tile,) . done()
        }
        #[doc = "Draws the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        #[inline]
        pub fn draw_rect_ex < 'ex > (&'ex self, canvas_item: Rid, rect: Rect2, tile: bool,) -> ExDrawRect < 'ex > {
            ExDrawRect::new(self, canvas_item, rect, tile,)
        }
        #[doc = "Draws a part of the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        pub(crate) fn draw_rect_region_full(&self, canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rect2, Color, bool, bool,);
            let args = (canvas_item, rect, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "draw_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_rect_region_ex`][Self::draw_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a part of the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        #[inline]
        pub fn draw_rect_region(&self, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) {
            self.draw_rect_region_ex(canvas_item, rect, src_rect,) . done()
        }
        #[doc = "Draws a part of the texture using a [`CanvasItem`][crate::classes::CanvasItem] with the [`RenderingServer`][crate::classes::RenderingServer] API."]
        #[inline]
        pub fn draw_rect_region_ex < 'ex > (&'ex self, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) -> ExDrawRectRegion < 'ex > {
            ExDrawRectRegion::new(self, canvas_item, rect, src_rect,)
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] that is a copy of data from this `Texture2D` (a new [`Image`][crate::classes::Image] is created each time). [`Image`][crate::classes::Image]s can be accessed and manipulated directly.\n\n**Note:** This will return `null` if this `Texture2D` is invalid.\n\n**Note:** This will fetch the texture data from the GPU, which might cause performance problems when overused. Avoid calling [`get_image`][`crate::classes::Texture2D::get_image`] every frame, especially on large textures."]
        pub fn get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a placeholder version of this resource ([`PlaceholderTexture2D`][crate::classes::PlaceholderTexture2D])."]
        pub fn create_placeholder(&self,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Texture2D", "create_placeholder", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Texture2D {
        type Base = crate::classes::Texture;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Texture2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Texture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for Texture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Texture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Texture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Texture2D {
        
    }
    impl crate::obj::cap::GodotDefault for Texture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Texture2D {
        type Target = crate::classes::Texture;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Texture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Texture2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Texture2D__ensure_class_exists {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`Texture2D::draw_ex`][super::Texture2D::draw_ex]."]
#[must_use]
pub struct ExDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Texture2D, canvas_item: Rid, position: Vector2, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDraw < 'ex > {
    fn new(surround_object: &'ex re_export::Texture2D, canvas_item: Rid, position: Vector2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, position: position, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, position, modulate, transpose,
        }
        = self;
        re_export::Texture2D::draw_full(surround_object, canvas_item, position, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`Texture2D::draw_rect_ex`][super::Texture2D::draw_rect_ex]."]
#[must_use]
pub struct ExDrawRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Texture2D, canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawRect < 'ex > {
    fn new(surround_object: &'ex re_export::Texture2D, canvas_item: Rid, rect: Rect2, tile: bool,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, rect: rect, tile: tile, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, rect, tile, modulate, transpose,
        }
        = self;
        re_export::Texture2D::draw_rect_full(surround_object, canvas_item, rect, tile, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`Texture2D::draw_rect_region_ex`][super::Texture2D::draw_rect_region_ex]."]
#[must_use]
pub struct ExDrawRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Texture2D, canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawRectRegion < 'ex > {
    fn new(surround_object: &'ex re_export::Texture2D, canvas_item: Rid, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        let clip_uv = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas_item: canvas_item, rect: rect, src_rect: src_rect, modulate: modulate, transpose: transpose, clip_uv: clip_uv,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, clip_uv: bool) -> Self {
        Self {
            clip_uv: clip_uv, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas_item, rect, src_rect, modulate, transpose, clip_uv,
        }
        = self;
        re_export::Texture2D::draw_rect_region_full(surround_object, canvas_item, rect, src_rect, modulate, transpose, clip_uv,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Texture2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Texture2D {
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