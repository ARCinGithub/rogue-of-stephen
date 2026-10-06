#![doc = "Sidecar module for class [`Environment`][crate::classes::Environment].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Environment` enums](https://docs.godotengine.org/en/stable/classes/class_environment.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Environment`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`environment`][crate::classes::environment]: sidecar module with related enum/flag types\n* [`IEnvironment`][crate::classes::IEnvironment]: virtual methods\n\n\nSee also [Godot docs for `Environment`](https://docs.godotengine.org/en/stable/classes/class_environment.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Environment::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nResource for environment nodes (like [`WorldEnvironment`][crate::classes::WorldEnvironment]) that define multiple environment operations (such as background [`Sky`][crate::classes::Sky] or [`Color`][crate::builtin::Color], ambient light, fog, depth-of-field...). These parameters affect the final render of the scene. The order of these operations is:\n\n- Depth of Field Blur\n\n- Auto Exposure\n\n- Glow\n\n- Tonemap\n\n- Adjustments"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Environment {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Environment`][crate::classes::Environment].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Environment` methods](https://docs.godotengine.org/en/stable/classes/class_environment.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEnvironment: crate::obj::GodotClass < Base = Environment > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Environment {
        pub fn set_background(&mut self, mode: crate::classes::environment::BgMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::BgMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_background(&self,) -> crate::classes::environment::BgMode {
            type CallRet = crate::classes::environment::BgMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky(&mut self, sky: impl AsArg < Option < Gd < crate::classes::Sky >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Sky > > >,);
            let args = (sky.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky(&self,) -> Option < Gd < crate::classes::Sky > > {
            type CallRet = Option < Gd < crate::classes::Sky > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_custom_fov(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sky_custom_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_custom_fov(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sky_custom_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_rotation(&mut self, euler_radians: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sky_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_rotation(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sky_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bg_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bg_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bg_energy_multiplier(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_bg_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bg_energy_multiplier(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_bg_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bg_intensity(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_bg_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bg_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_bg_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_canvas_max_layer(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_canvas_max_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_canvas_max_layer(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_canvas_max_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera_feed_id(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_camera_feed_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_feed_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_camera_feed_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_light_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ambient_light_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_light_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ambient_light_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_source(&mut self, source: crate::classes::environment::AmbientSource,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::AmbientSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ambient_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_source(&self,) -> crate::classes::environment::AmbientSource {
            type CallRet = crate::classes::environment::AmbientSource;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ambient_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_light_energy(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ambient_light_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_light_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ambient_light_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_light_sky_contribution(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ambient_light_sky_contribution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_light_sky_contribution(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ambient_light_sky_contribution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reflection_source(&mut self, source: crate::classes::environment::ReflectionSource,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::ReflectionSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_reflection_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reflection_source(&self,) -> crate::classes::environment::ReflectionSource {
            type CallRet = crate::classes::environment::ReflectionSource;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_reflection_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tonemapper(&mut self, mode: crate::classes::environment::ToneMapper,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::ToneMapper,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_tonemapper", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tonemapper(&self,) -> crate::classes::environment::ToneMapper {
            type CallRet = crate::classes::environment::ToneMapper;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_tonemapper", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tonemap_exposure(&mut self, exposure: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (exposure,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_tonemap_exposure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tonemap_exposure(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_tonemap_exposure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tonemap_white(&mut self, white: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (white,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_tonemap_white", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tonemap_white(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_tonemap_white", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tonemap_agx_white(&mut self, white: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (white,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_tonemap_agx_white", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tonemap_agx_white(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_tonemap_agx_white", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tonemap_agx_contrast(&mut self, contrast: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (contrast,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_tonemap_agx_contrast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tonemap_agx_contrast(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_tonemap_agx_contrast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssr_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssr_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ssr_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_ssr_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssr_max_steps(&mut self, max_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssr_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssr_max_steps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssr_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssr_fade_in(&mut self, fade_in: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fade_in,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssr_fade_in", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssr_fade_in(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssr_fade_in", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssr_fade_out(&mut self, fade_out: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fade_out,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssr_fade_out", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssr_fade_out(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssr_fade_out", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssr_depth_tolerance(&mut self, depth_tolerance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (depth_tolerance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssr_depth_tolerance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssr_depth_tolerance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssr_depth_tolerance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ssao_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_ssao_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_intensity(&mut self, intensity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_power(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_power", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_power(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_power", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_detail(&mut self, detail: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (detail,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_detail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_detail(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_detail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_horizon(&mut self, horizon: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (horizon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_horizon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_horizon(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_horizon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_sharpness(&mut self, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_direct_light_affect(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_direct_light_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_direct_light_affect(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_direct_light_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssao_ao_channel_affect(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssao_ao_channel_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssao_ao_channel_affect(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssao_ao_channel_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssil_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssil_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ssil_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_ssil_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssil_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssil_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssil_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssil_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssil_intensity(&mut self, intensity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssil_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssil_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssil_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssil_sharpness(&mut self, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssil_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssil_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssil_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ssil_normal_rejection(&mut self, normal_rejection: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (normal_rejection,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_ssil_normal_rejection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ssil_normal_rejection(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_ssil_normal_rejection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sdfgi_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_sdfgi_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_cascades(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_cascades", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_cascades(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_cascades", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_min_cell_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_min_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_min_cell_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_min_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_max_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_max_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_cascade0_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_cascade0_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_cascade0_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_cascade0_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_y_scale(&mut self, scale: crate::classes::environment::SdfgiYScale,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::SdfgiYScale,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_y_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_y_scale(&self,) -> crate::classes::environment::SdfgiYScale {
            type CallRet = crate::classes::environment::SdfgiYScale;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_y_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_use_occlusion(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_use_occlusion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sdfgi_using_occlusion(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_sdfgi_using_occlusion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_bounce_feedback(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_bounce_feedback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_bounce_feedback(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_bounce_feedback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_read_sky_light(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_read_sky_light", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sdfgi_reading_sky_light(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_sdfgi_reading_sky_light", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_energy(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_normal_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_normal_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sdfgi_probe_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_sdfgi_probe_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sdfgi_probe_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_sdfgi_probe_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_glow_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_glow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the intensity of the glow level `idx`. A value above `0.0` enables the level. Each level relies on the previous level. This means that enabling higher glow levels will slow down the glow effect rendering, even if previous levels aren't enabled."]
        pub fn set_glow_level(&mut self, idx: i32, intensity: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (idx, intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the intensity of the glow level `idx`."]
        pub fn get_glow_level(&self, idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_normalized(&mut self, normalize: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_glow_normalized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_glow_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_intensity(&mut self, intensity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_strength(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_mix(&mut self, mix: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_mix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_mix(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_mix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_bloom(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_bloom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_bloom(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_bloom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_blend_mode(&mut self, mode: crate::classes::environment::GlowBlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::GlowBlendMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_blend_mode(&self,) -> crate::classes::environment::GlowBlendMode {
            type CallRet = crate::classes::environment::GlowBlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_hdr_bleed_threshold(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_hdr_bleed_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_hdr_bleed_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_hdr_bleed_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_hdr_bleed_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_hdr_bleed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_hdr_bleed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_hdr_bleed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_hdr_luminance_cap(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_hdr_luminance_cap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_hdr_luminance_cap(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_hdr_luminance_cap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_map_strength(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_map_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_map_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_map_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glow_map(&mut self, mode: impl AsArg < Option < Gd < crate::classes::Texture >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture > > >,);
            let args = (mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_glow_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glow_map(&self,) -> Option < Gd < crate::classes::Texture > > {
            type CallRet = Option < Gd < crate::classes::Texture > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_glow_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fog_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_fog_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_mode(&mut self, mode: crate::classes::environment::FogMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::environment::FogMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_mode(&self,) -> crate::classes::environment::FogMode {
            type CallRet = crate::classes::environment::FogMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_light_color(&mut self, light_color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (light_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_light_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_light_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_light_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_light_energy(&mut self, light_energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (light_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_light_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_light_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_light_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_sun_scatter(&mut self, sun_scatter: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sun_scatter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_sun_scatter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_sun_scatter(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_sun_scatter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_density(&mut self, density: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_density(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_height(&mut self, height: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_height_density(&mut self, height_density: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height_density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_height_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_height_density(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_height_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_aerial_perspective(&mut self, aerial_perspective: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (aerial_perspective,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_aerial_perspective", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_aerial_perspective(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_aerial_perspective", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_sky_affect(&mut self, sky_affect: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sky_affect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_sky_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_sky_affect(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_sky_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_depth_curve(&mut self, curve: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_depth_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_depth_curve(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_depth_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_depth_begin(&mut self, begin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (begin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_depth_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_depth_begin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_depth_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fog_depth_end(&mut self, end: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_fog_depth_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fog_depth_end(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_fog_depth_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_volumetric_fog_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_volumetric_fog_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_emission(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_emission(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_albedo(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_albedo(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_density(&mut self, density: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (density,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_density(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_density", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_emission_energy(&mut self, begin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (begin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_emission_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_anisotropy(&mut self, anisotropy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (anisotropy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_anisotropy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_length(&mut self, length: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_detail_spread(&mut self, detail_spread: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (detail_spread,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_detail_spread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_detail_spread(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_detail_spread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_gi_inject(&mut self, gi_inject: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gi_inject,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_gi_inject", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_gi_inject(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_gi_inject", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_ambient_inject(&mut self, enabled: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_ambient_inject", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_ambient_inject(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_ambient_inject", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_sky_affect(&mut self, sky_affect: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sky_affect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_sky_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_sky_affect(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_sky_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_temporal_reprojection_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_temporal_reprojection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_volumetric_fog_temporal_reprojection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_volumetric_fog_temporal_reprojection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volumetric_fog_temporal_reprojection_amount(&mut self, temporal_reprojection_amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (temporal_reprojection_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_volumetric_fog_temporal_reprojection_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volumetric_fog_temporal_reprojection_amount(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_volumetric_fog_temporal_reprojection_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_adjustment_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_adjustment_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_adjustment_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "is_adjustment_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_adjustment_brightness(&mut self, brightness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (brightness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_adjustment_brightness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_adjustment_brightness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_adjustment_brightness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_adjustment_contrast(&mut self, contrast: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (contrast,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_adjustment_contrast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_adjustment_contrast(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_adjustment_contrast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_adjustment_saturation(&mut self, saturation: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_adjustment_saturation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_adjustment_saturation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_adjustment_saturation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_adjustment_color_correction(&mut self, color_correction: impl AsArg < Option < Gd < crate::classes::Texture >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture > > >,);
            let args = (color_correction.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "set_adjustment_color_correction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_adjustment_color_correction(&self,) -> Option < Gd < crate::classes::Texture > > {
            type CallRet = Option < Gd < crate::classes::Texture > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Environment", "get_adjustment_color_correction", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Environment {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Environment"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Environment {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Environment {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Environment {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Environment {
        
    }
    impl crate::obj::cap::GodotDefault for Environment {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Environment {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Environment {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Environment`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Environment__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Environment > for $Class {
                
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
#[doc = "Godot enum name: `BGMode`."]
pub struct BgMode {
    ord: i32
}
impl BgMode {
    #[doc(alias = "BG_CLEAR_COLOR")]
    #[doc = "Godot enumerator name: `BG_CLEAR_COLOR`"]
    pub const CLEAR_COLOR: BgMode = BgMode {
        ord: 0i32
    };
    #[doc(alias = "BG_COLOR")]
    #[doc = "Godot enumerator name: `BG_COLOR`"]
    pub const COLOR: BgMode = BgMode {
        ord: 1i32
    };
    #[doc(alias = "BG_SKY")]
    #[doc = "Godot enumerator name: `BG_SKY`"]
    pub const SKY: BgMode = BgMode {
        ord: 2i32
    };
    #[doc(alias = "BG_CANVAS")]
    #[doc = "Godot enumerator name: `BG_CANVAS`"]
    pub const CANVAS: BgMode = BgMode {
        ord: 3i32
    };
    #[doc(alias = "BG_KEEP")]
    #[doc = "Godot enumerator name: `BG_KEEP`"]
    pub const KEEP: BgMode = BgMode {
        ord: 4i32
    };
    #[doc(alias = "BG_CAMERA_FEED")]
    #[doc = "Godot enumerator name: `BG_CAMERA_FEED`"]
    pub const CAMERA_FEED: BgMode = BgMode {
        ord: 5i32
    };
    #[doc(alias = "BG_MAX")]
    #[doc = "Godot enumerator name: `BG_MAX`"]
    pub const MAX: BgMode = BgMode {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for BgMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BgMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BgMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::CLEAR_COLOR => "CLEAR_COLOR", Self::COLOR => "COLOR", Self::SKY => "SKY", Self::CANVAS => "CANVAS", Self::KEEP => "KEEP", Self::CAMERA_FEED => "CAMERA_FEED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BgMode::CLEAR_COLOR, BgMode::COLOR, BgMode::SKY, BgMode::CANVAS, BgMode::KEEP, BgMode::CAMERA_FEED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BgMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLEAR_COLOR", "BG_CLEAR_COLOR", BgMode::CLEAR_COLOR), crate::meta::inspect::EnumConstant::new("COLOR", "BG_COLOR", BgMode::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "BG_SKY", BgMode::SKY), crate::meta::inspect::EnumConstant::new("CANVAS", "BG_CANVAS", BgMode::CANVAS), crate::meta::inspect::EnumConstant::new("KEEP", "BG_KEEP", BgMode::KEEP), crate::meta::inspect::EnumConstant::new("CAMERA_FEED", "BG_CAMERA_FEED", BgMode::CAMERA_FEED), crate::meta::inspect::EnumConstant::new("MAX", "BG_MAX", BgMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for BgMode {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for BgMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bg Clear Color", 0i64), EnumeratorShape::new_int("Bg Color", 1i64), EnumeratorShape::new_int("Bg Sky", 2i64), EnumeratorShape::new_int("Bg Canvas", 3i64), EnumeratorShape::new_int("Bg Keep", 4i64), EnumeratorShape::new_int("Bg Camera Feed", 5i64), EnumeratorShape::new_int("Bg Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.BGMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BgMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BgMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BgMode {
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
impl crate::registry::property::Export for BgMode {
    
}
impl crate::meta::Element for BgMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AmbientSource {
    ord: i32
}
impl AmbientSource {
    #[doc(alias = "AMBIENT_SOURCE_BG")]
    #[doc = "Godot enumerator name: `AMBIENT_SOURCE_BG`"]
    pub const BG: AmbientSource = AmbientSource {
        ord: 0i32
    };
    #[doc(alias = "AMBIENT_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `AMBIENT_SOURCE_DISABLED`"]
    pub const DISABLED: AmbientSource = AmbientSource {
        ord: 1i32
    };
    #[doc(alias = "AMBIENT_SOURCE_COLOR")]
    #[doc = "Godot enumerator name: `AMBIENT_SOURCE_COLOR`"]
    pub const COLOR: AmbientSource = AmbientSource {
        ord: 2i32
    };
    #[doc(alias = "AMBIENT_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `AMBIENT_SOURCE_SKY`"]
    pub const SKY: AmbientSource = AmbientSource {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AmbientSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AmbientSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AmbientSource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::COLOR => "COLOR", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AmbientSource::BG, AmbientSource::DISABLED, AmbientSource::COLOR, AmbientSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AmbientSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "AMBIENT_SOURCE_BG", AmbientSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "AMBIENT_SOURCE_DISABLED", AmbientSource::DISABLED), crate::meta::inspect::EnumConstant::new("COLOR", "AMBIENT_SOURCE_COLOR", AmbientSource::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "AMBIENT_SOURCE_SKY", AmbientSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for AmbientSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Ambient Source Bg", 0i64), EnumeratorShape::new_int("Ambient Source Disabled", 1i64), EnumeratorShape::new_int("Ambient Source Color", 2i64), EnumeratorShape::new_int("Ambient Source Sky", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.AmbientSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AmbientSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AmbientSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AmbientSource {
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
impl crate::registry::property::Export for AmbientSource {
    
}
impl crate::meta::Element for AmbientSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReflectionSource {
    ord: i32
}
impl ReflectionSource {
    #[doc(alias = "REFLECTION_SOURCE_BG")]
    #[doc = "Godot enumerator name: `REFLECTION_SOURCE_BG`"]
    pub const BG: ReflectionSource = ReflectionSource {
        ord: 0i32
    };
    #[doc(alias = "REFLECTION_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `REFLECTION_SOURCE_DISABLED`"]
    pub const DISABLED: ReflectionSource = ReflectionSource {
        ord: 1i32
    };
    #[doc(alias = "REFLECTION_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `REFLECTION_SOURCE_SKY`"]
    pub const SKY: ReflectionSource = ReflectionSource {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ReflectionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReflectionSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReflectionSource {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReflectionSource::BG, ReflectionSource::DISABLED, ReflectionSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReflectionSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "REFLECTION_SOURCE_BG", ReflectionSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "REFLECTION_SOURCE_DISABLED", ReflectionSource::DISABLED), crate::meta::inspect::EnumConstant::new("SKY", "REFLECTION_SOURCE_SKY", ReflectionSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for ReflectionSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Reflection Source Bg", 0i64), EnumeratorShape::new_int("Reflection Source Disabled", 1i64), EnumeratorShape::new_int("Reflection Source Sky", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.ReflectionSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ReflectionSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReflectionSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ReflectionSource {
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
impl crate::registry::property::Export for ReflectionSource {
    
}
impl crate::meta::Element for ReflectionSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ToneMapper {
    ord: i32
}
impl ToneMapper {
    #[doc(alias = "TONE_MAPPER_LINEAR")]
    #[doc = "Godot enumerator name: `TONE_MAPPER_LINEAR`"]
    pub const LINEAR: ToneMapper = ToneMapper {
        ord: 0i32
    };
    #[doc(alias = "TONE_MAPPER_REINHARDT")]
    #[doc = "Godot enumerator name: `TONE_MAPPER_REINHARDT`"]
    pub const REINHARDT: ToneMapper = ToneMapper {
        ord: 1i32
    };
    #[doc(alias = "TONE_MAPPER_FILMIC")]
    #[doc = "Godot enumerator name: `TONE_MAPPER_FILMIC`"]
    pub const FILMIC: ToneMapper = ToneMapper {
        ord: 2i32
    };
    #[doc(alias = "TONE_MAPPER_ACES")]
    #[doc = "Godot enumerator name: `TONE_MAPPER_ACES`"]
    pub const ACES: ToneMapper = ToneMapper {
        ord: 3i32
    };
    #[doc(alias = "TONE_MAPPER_AGX")]
    #[doc = "Godot enumerator name: `TONE_MAPPER_AGX`"]
    pub const AGX: ToneMapper = ToneMapper {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ToneMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ToneMapper") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ToneMapper {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::LINEAR => "LINEAR", Self::REINHARDT => "REINHARDT", Self::FILMIC => "FILMIC", Self::ACES => "ACES", Self::AGX => "AGX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ToneMapper::LINEAR, ToneMapper::REINHARDT, ToneMapper::FILMIC, ToneMapper::ACES, ToneMapper::AGX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ToneMapper >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "TONE_MAPPER_LINEAR", ToneMapper::LINEAR), crate::meta::inspect::EnumConstant::new("REINHARDT", "TONE_MAPPER_REINHARDT", ToneMapper::REINHARDT), crate::meta::inspect::EnumConstant::new("FILMIC", "TONE_MAPPER_FILMIC", ToneMapper::FILMIC), crate::meta::inspect::EnumConstant::new("ACES", "TONE_MAPPER_ACES", ToneMapper::ACES), crate::meta::inspect::EnumConstant::new("AGX", "TONE_MAPPER_AGX", ToneMapper::AGX)]
        }
    }
}
impl crate::meta::GodotConvert for ToneMapper {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tone Mapper Linear", 0i64), EnumeratorShape::new_int("Tone Mapper Reinhardt", 1i64), EnumeratorShape::new_int("Tone Mapper Filmic", 2i64), EnumeratorShape::new_int("Tone Mapper Aces", 3i64), EnumeratorShape::new_int("Tone Mapper Agx", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.ToneMapper")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ToneMapper {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ToneMapper {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ToneMapper {
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
impl crate::registry::property::Export for ToneMapper {
    
}
impl crate::meta::Element for ToneMapper {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlowBlendMode {
    ord: i32
}
impl GlowBlendMode {
    #[doc(alias = "GLOW_BLEND_MODE_ADDITIVE")]
    #[doc = "Godot enumerator name: `GLOW_BLEND_MODE_ADDITIVE`"]
    pub const ADDITIVE: GlowBlendMode = GlowBlendMode {
        ord: 0i32
    };
    #[doc(alias = "GLOW_BLEND_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `GLOW_BLEND_MODE_SCREEN`"]
    pub const SCREEN: GlowBlendMode = GlowBlendMode {
        ord: 1i32
    };
    #[doc(alias = "GLOW_BLEND_MODE_SOFTLIGHT")]
    #[doc = "Godot enumerator name: `GLOW_BLEND_MODE_SOFTLIGHT`"]
    pub const SOFTLIGHT: GlowBlendMode = GlowBlendMode {
        ord: 2i32
    };
    #[doc(alias = "GLOW_BLEND_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `GLOW_BLEND_MODE_REPLACE`"]
    pub const REPLACE: GlowBlendMode = GlowBlendMode {
        ord: 3i32
    };
    #[doc(alias = "GLOW_BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `GLOW_BLEND_MODE_MIX`"]
    pub const MIX: GlowBlendMode = GlowBlendMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for GlowBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GlowBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GlowBlendMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::ADDITIVE => "ADDITIVE", Self::SCREEN => "SCREEN", Self::SOFTLIGHT => "SOFTLIGHT", Self::REPLACE => "REPLACE", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GlowBlendMode::ADDITIVE, GlowBlendMode::SCREEN, GlowBlendMode::SOFTLIGHT, GlowBlendMode::REPLACE, GlowBlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GlowBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADDITIVE", "GLOW_BLEND_MODE_ADDITIVE", GlowBlendMode::ADDITIVE), crate::meta::inspect::EnumConstant::new("SCREEN", "GLOW_BLEND_MODE_SCREEN", GlowBlendMode::SCREEN), crate::meta::inspect::EnumConstant::new("SOFTLIGHT", "GLOW_BLEND_MODE_SOFTLIGHT", GlowBlendMode::SOFTLIGHT), crate::meta::inspect::EnumConstant::new("REPLACE", "GLOW_BLEND_MODE_REPLACE", GlowBlendMode::REPLACE), crate::meta::inspect::EnumConstant::new("MIX", "GLOW_BLEND_MODE_MIX", GlowBlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for GlowBlendMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Glow Blend Mode Additive", 0i64), EnumeratorShape::new_int("Glow Blend Mode Screen", 1i64), EnumeratorShape::new_int("Glow Blend Mode Softlight", 2i64), EnumeratorShape::new_int("Glow Blend Mode Replace", 3i64), EnumeratorShape::new_int("Glow Blend Mode Mix", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.GlowBlendMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GlowBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GlowBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GlowBlendMode {
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
impl crate::registry::property::Export for GlowBlendMode {
    
}
impl crate::meta::Element for GlowBlendMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FogMode {
    ord: i32
}
impl FogMode {
    #[doc(alias = "FOG_MODE_EXPONENTIAL")]
    #[doc = "Godot enumerator name: `FOG_MODE_EXPONENTIAL`"]
    pub const EXPONENTIAL: FogMode = FogMode {
        ord: 0i32
    };
    #[doc(alias = "FOG_MODE_DEPTH")]
    #[doc = "Godot enumerator name: `FOG_MODE_DEPTH`"]
    pub const DEPTH: FogMode = FogMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FogMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::EXPONENTIAL => "EXPONENTIAL", Self::DEPTH => "DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FogMode::EXPONENTIAL, FogMode::DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FogMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EXPONENTIAL", "FOG_MODE_EXPONENTIAL", FogMode::EXPONENTIAL), crate::meta::inspect::EnumConstant::new("DEPTH", "FOG_MODE_DEPTH", FogMode::DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for FogMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Fog Mode Exponential", 0i64), EnumeratorShape::new_int("Fog Mode Depth", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.FogMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FogMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FogMode {
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
impl crate::registry::property::Export for FogMode {
    
}
impl crate::meta::Element for FogMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `SDFGIYScale`."]
pub struct SdfgiYScale {
    ord: i32
}
impl SdfgiYScale {
    #[doc(alias = "SDFGI_Y_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `SDFGI_Y_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: SdfgiYScale = SdfgiYScale {
        ord: 0i32
    };
    #[doc(alias = "SDFGI_Y_SCALE_75_PERCENT")]
    #[doc = "Godot enumerator name: `SDFGI_Y_SCALE_75_PERCENT`"]
    pub const SCALE_75_PERCENT: SdfgiYScale = SdfgiYScale {
        ord: 1i32
    };
    #[doc(alias = "SDFGI_Y_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `SDFGI_Y_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: SdfgiYScale = SdfgiYScale {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SdfgiYScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SdfgiYScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SdfgiYScale {
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
            Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_75_PERCENT => "SCALE_75_PERCENT", Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SdfgiYScale::SCALE_50_PERCENT, SdfgiYScale::SCALE_75_PERCENT, SdfgiYScale::SCALE_100_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SdfgiYScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "SDFGI_Y_SCALE_50_PERCENT", SdfgiYScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_75_PERCENT", "SDFGI_Y_SCALE_75_PERCENT", SdfgiYScale::SCALE_75_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "SDFGI_Y_SCALE_100_PERCENT", SdfgiYScale::SCALE_100_PERCENT)]
        }
    }
}
impl crate::meta::GodotConvert for SdfgiYScale {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Sdfgi Y Scale 50 Percent", 0i64), EnumeratorShape::new_int("Sdfgi Y Scale 75 Percent", 1i64), EnumeratorShape::new_int("Sdfgi Y Scale 100 Percent", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Environment.SDFGIYScale")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SdfgiYScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SdfgiYScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SdfgiYScale {
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
impl crate::registry::property::Export for SdfgiYScale {
    
}
impl crate::meta::Element for SdfgiYScale {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Environment;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Environment {
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