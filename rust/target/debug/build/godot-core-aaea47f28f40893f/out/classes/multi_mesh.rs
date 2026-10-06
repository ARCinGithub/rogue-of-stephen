#![doc = "Sidecar module for class [`MultiMesh`][crate::classes::MultiMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiMesh` enums](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiMesh`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`multi_mesh`][crate::classes::multi_mesh]: sidecar module with related enum/flag types\n* [`IMultiMesh`][crate::classes::IMultiMesh]: virtual methods\n\n\nSee also [Godot docs for `MultiMesh`](https://docs.godotengine.org/en/stable/classes/class_multimesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MultiMesh::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nMultiMesh provides low-level mesh instancing. Drawing thousands of [`MeshInstance3D`][crate::classes::MeshInstance3D] nodes can be slow, since each object is submitted to the GPU then drawn individually.\n\nMultiMesh is much faster as it can draw thousands of instances with a single draw call, resulting in less API overhead.\n\nAs a drawback, if the instances are too far away from each other, performance may be reduced as every single instance will always render (they are spatially indexed as one, for the whole object).\n\nSince instances may have any behavior, the AABB used for visibility must be provided by the user.\n\n**Note:** A MultiMesh is a single object, therefore the same maximum lights per object restriction applies. This means, that once the maximum lights are consumed by one or more instances, the rest of the MultiMesh instances will **not** receive any lighting.\n\n**Note:** Blend Shapes will be ignored if used in a MultiMesh."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MultiMesh`][crate::classes::MultiMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MultiMesh` methods](https://docs.godotengine.org/en/stable/classes/class_multimesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiMesh: crate::obj::GodotClass < Base = MultiMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiMesh {
        pub fn set_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_colors(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_use_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_colors(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "is_using_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_data(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_use_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_data(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "is_using_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform_format(&mut self, format: crate::classes::multi_mesh::TransformFormat,) {
            type CallRet = ();
            type CallParams = (crate::classes::multi_mesh::TransformFormat,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_transform_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform_format(&self,) -> crate::classes::multi_mesh::TransformFormat {
            type CallRet = crate::classes::multi_mesh::TransformFormat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_transform_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_instance_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_instance_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_instance_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_visible_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_instance_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_visible_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_interpolation_quality(&mut self, quality: crate::classes::multi_mesh::PhysicsInterpolationQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::multi_mesh::PhysicsInterpolationQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_physics_interpolation_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_interpolation_quality(&self,) -> crate::classes::multi_mesh::PhysicsInterpolationQuality {
            type CallRet = crate::classes::multi_mesh::PhysicsInterpolationQuality;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_physics_interpolation_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform3D`][crate::builtin::Transform3D] for a specific instance."]
        pub fn set_instance_transform(&mut self, instance: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (i32, Transform3D,);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform2D`][crate::builtin::Transform2D] for a specific instance."]
        pub fn set_instance_transform_2d(&mut self, instance: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (i32, Transform2D,);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform3D`][crate::builtin::Transform3D] of a specific instance."]
        pub fn get_instance_transform(&self, instance: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (i32,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform2D`][crate::builtin::Transform2D] of a specific instance."]
        pub fn get_instance_transform_2d(&self, instance: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (i32,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of a specific instance by _multiplying_ the mesh's existing vertex colors. This allows for different color tinting per instance.\n\n**Note:** Each component is stored in 32 bits in the Forward+ and Mobile rendering methods, but is packed into 16 bits in the Compatibility rendering method.\n\nFor the color to take effect, ensure that \\[member use_colors] is `true` on the `MultiMesh` and \\[member BaseMaterial3D.vertex_color_use_as_albedo] is `true` on the material. If you intend to set an absolute color instead of tinting, make sure the material's albedo color is set to pure white (`Color(1, 1, 1)`)."]
        pub fn set_instance_color(&mut self, instance: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (instance, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets a specific instance's color multiplier."]
        pub fn get_instance_color(&self, instance: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets custom data for a specific instance. `custom_data` is a [`Color`][crate::builtin::Color] type only to contain 4 floating-point numbers.\n\n**Note:** Each number is stored in 32 bits in the Forward+ and Mobile rendering methods, but is packed into 16 bits in the Compatibility rendering method.\n\nFor the custom data to be used, ensure that \\[member use_custom_data] is `true`.\n\nThis custom instance data has to be manually accessed in your custom shader using `INSTANCE_CUSTOM`."]
        pub fn set_instance_custom_data(&mut self, instance: i32, custom_data: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (instance, custom_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_instance_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom data that has been set for a specific instance."]
        pub fn get_instance_custom_data(&self, instance: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_instance_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When using _physics interpolation_, this function allows you to prevent interpolation on an instance in the current physics tick.\n\nThis allows you to move instances instantaneously, and should usually be used when initially placing an instance such as a bullet to prevent graphical glitches."]
        pub fn reset_instance_physics_interpolation(&mut self, instance: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "reset_instance_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When using _physics interpolation_, this function allows you to prevent interpolation for all instances in the current physics tick.\n\nThis allows you to move all instances instantaneously, and should usually be used when initially placing instances to prevent graphical glitches."]
        pub fn reset_instances_physics_interpolation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "reset_instances_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the visibility axis-aligned bounding box in local space."]
        pub fn get_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer(&mut self, buffer: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "An alternative to setting the \\[member buffer] property, which can be used with _physics interpolation_. This method takes two arrays, and can set the data for the current and previous tick in one go. The renderer will automatically interpolate the data at each frame.\n\nThis is useful for situations where the order of instances may change from physics tick to tick, such as particle systems.\n\nWhen the order of instances is coherent, the simpler alternative of setting \\[member buffer] can still be used with interpolation."]
        pub fn set_buffer_interpolated(&mut self, buffer_curr: &PackedFloat32Array, buffer_prev: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedFloat32Array >, RefArg < 'a1, PackedFloat32Array >,);
            let args = (RefArg::new(buffer_curr), RefArg::new(buffer_prev),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiMesh", "set_buffer_interpolated", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MultiMesh {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for MultiMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiMesh {
        
    }
    impl crate::obj::cap::GodotDefault for MultiMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiMesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiMesh > for $Class {
                
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
pub struct TransformFormat {
    ord: i32
}
impl TransformFormat {
    pub const TRANSFORM_2D: TransformFormat = TransformFormat {
        ord: 0i32
    };
    pub const TRANSFORM_3D: TransformFormat = TransformFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TransformFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransformFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransformFormat {
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
            Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM_3D => "TRANSFORM_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransformFormat::TRANSFORM_2D, TransformFormat::TRANSFORM_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransformFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TRANSFORM_2D", "TRANSFORM_2D", TransformFormat::TRANSFORM_2D), crate::meta::inspect::EnumConstant::new("TRANSFORM_3D", "TRANSFORM_3D", TransformFormat::TRANSFORM_3D)]
        }
    }
}
impl crate::meta::GodotConvert for TransformFormat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Transform 2d", 0i64), EnumeratorShape::new_int("Transform 3d", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiMesh.TransformFormat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TransformFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TransformFormat {
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
impl crate::registry::property::Export for TransformFormat {
    
}
impl crate::meta::Element for TransformFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicsInterpolationQuality {
    ord: i32
}
impl PhysicsInterpolationQuality {
    #[doc(alias = "INTERP_QUALITY_FAST")]
    #[doc = "Godot enumerator name: `INTERP_QUALITY_FAST`"]
    pub const FAST: PhysicsInterpolationQuality = PhysicsInterpolationQuality {
        ord: 0i32
    };
    #[doc(alias = "INTERP_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `INTERP_QUALITY_HIGH`"]
    pub const HIGH: PhysicsInterpolationQuality = PhysicsInterpolationQuality {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PhysicsInterpolationQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PhysicsInterpolationQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PhysicsInterpolationQuality {
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
            Self::FAST => "FAST", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PhysicsInterpolationQuality::FAST, PhysicsInterpolationQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PhysicsInterpolationQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FAST", "INTERP_QUALITY_FAST", PhysicsInterpolationQuality::FAST), crate::meta::inspect::EnumConstant::new("HIGH", "INTERP_QUALITY_HIGH", PhysicsInterpolationQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for PhysicsInterpolationQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Interp Quality Fast", 0i64), EnumeratorShape::new_int("Interp Quality High", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiMesh.PhysicsInterpolationQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PhysicsInterpolationQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PhysicsInterpolationQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PhysicsInterpolationQuality {
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
impl crate::registry::property::Export for PhysicsInterpolationQuality {
    
}
impl crate::meta::Element for PhysicsInterpolationQuality {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiMesh;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for MultiMesh {
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