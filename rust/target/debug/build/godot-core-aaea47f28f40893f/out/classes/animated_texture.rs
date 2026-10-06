#![doc = "Sidecar module for class [`AnimatedTexture`][crate::classes::AnimatedTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimatedTexture` enums](https://docs.godotengine.org/en/stable/classes/class_animatedtexture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimatedTexture`.\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`IAnimatedTexture`][crate::classes::IAnimatedTexture]: virtual methods\n\n\nSee also [Godot docs for `AnimatedTexture`](https://docs.godotengine.org/en/stable/classes/class_animatedtexture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimatedTexture::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`AnimatedTexture` is a resource format for frame-based animations, where multiple textures can be chained automatically with a predefined delay for each frame. Unlike [`AnimationPlayer`][crate::classes::AnimationPlayer] or [`AnimatedSprite2D`][crate::classes::AnimatedSprite2D], it isn't a [`Node`][crate::classes::Node], but has the advantage of being usable anywhere a [`Texture2D`][crate::classes::Texture2D] resource can be used, e.g. in a [`TileSet`][crate::classes::TileSet].\n\nThe playback of the animation is controlled by the \\[member speed_scale] property, as well as each frame's duration (see [`set_frame_duration`][`crate::classes::AnimatedTexture::set_frame_duration`]). The animation loops, i.e. it will restart at frame 0 automatically after playing the last frame.\n\n`AnimatedTexture` currently requires all frame textures to have the same size, otherwise the bigger ones will be cropped to match the smallest one.\n\n**Note:** AnimatedTexture doesn't support using [`AtlasTexture`][crate::classes::AtlasTexture]s. Each frame needs to be a separate [`Texture2D`][crate::classes::Texture2D].\n\n**Warning:** The current implementation is not efficient for the modern renderers."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimatedTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimatedTexture`][crate::classes::AnimatedTexture].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture2D`][crate::classes::ITexture2D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimatedTexture` methods](https://docs.godotengine.org/en/stable/classes/class_animatedtexture.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimatedTexture: crate::obj::GodotClass < Base = AnimatedTexture > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimatedTexture {
        pub fn set_frames(&mut self, frames: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_frame(&mut self, frame: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_current_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_current_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pause(&mut self, pause: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pause(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_one_shot(&mut self, one_shot: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (one_shot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns a [`Texture2D`][crate::classes::Texture2D] to the given frame. Frame IDs start at 0, so the first frame has ID 0, and the last frame of the animation has ID \\[member frames] - 1.\n\nYou can define any number of textures up to `MAX_FRAMES`, but keep in mind that only frames from 0 to \\[member frames] - 1 will be part of the animation."]
        pub fn set_frame_texture(&mut self, frame: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (frame, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_frame_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given frame's [`Texture2D`][crate::classes::Texture2D]."]
        pub fn get_frame_texture(&self, frame: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_frame_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the duration of any given `frame`. The final duration is affected by the \\[member speed_scale]. If set to `0`, the frame is skipped during playback."]
        pub fn set_frame_duration(&mut self, frame: i32, duration: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (frame, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "set_frame_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given `frame`'s duration, in seconds."]
        pub fn get_frame_duration(&self, frame: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedTexture", "get_frame_duration", Some(self.__validated_obj()), args,)
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
        pub const MAX_FRAMES: i32 = 256i32;
        
    }
    impl crate::obj::GodotClass for AnimatedTexture {
        type Base = crate::classes::Texture2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimatedTexture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimatedTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for AnimatedTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for AnimatedTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimatedTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimatedTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimatedTexture {
        
    }
    impl crate::obj::cap::GodotDefault for AnimatedTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimatedTexture {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimatedTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimatedTexture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimatedTexture__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimatedTexture > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimatedTexture;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for AnimatedTexture {
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