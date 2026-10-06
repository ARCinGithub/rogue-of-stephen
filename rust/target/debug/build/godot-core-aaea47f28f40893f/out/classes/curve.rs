#![doc = "Sidecar module for class [`Curve`][crate::classes::Curve].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve` enums](https://docs.godotengine.org/en/stable/classes/class_curve.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Curve`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`curve`][crate::classes::curve]: sidecar module with related enum/flag types\n* [`ICurve`][crate::classes::ICurve]: virtual methods\n* [`SignalsOfCurve`][crate::classes::curve::SignalsOfCurve]: signal collection\n\n\nSee also [Godot docs for `Curve`](https://docs.godotengine.org/en/stable/classes/class_curve.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Curve::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis resource describes a mathematical curve by defining a set of points and tangents at each point. By default, it ranges between `0` and `1` on the X and Y axes, but these ranges can be changed.\n\nPlease note that many resources and nodes assume they are given _unit curves_. A unit curve is a curve whose domain (the X axis) is between `0` and `1`. Some examples of unit curve usage are \\[member CPUParticles2D.angle_curve] and \\[member Line2D.width_curve]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Curve {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Curve`][crate::classes::Curve].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Curve` methods](https://docs.godotengine.org/en/stable/classes/class_curve.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICurve: crate::obj::GodotClass < Base = Curve > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Curve {
        pub fn get_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a point to the curve. For each side, if the `*_mode` is [`TangentMode::LINEAR`][`crate::classes::curve::TangentMode::LINEAR`], the `*_tangent` angle (in degrees) uses the slope of the curve halfway to the adjacent point. Allows custom assignments to the `*_tangent` angle if `*_mode` is set to [`TangentMode::FREE`][`crate::classes::curve::TangentMode::FREE`]."]
        pub(crate) fn add_point_full(&mut self, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::classes::curve::TangentMode, right_mode: crate::classes::curve::TangentMode,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2, f32, f32, crate::classes::curve::TangentMode, crate::classes::curve::TangentMode,);
            let args = (position, left_tangent, right_tangent, left_mode, right_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "add_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_point_ex`][Self::add_point_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a point to the curve. For each side, if the `*_mode` is [`TangentMode::LINEAR`][`crate::classes::curve::TangentMode::LINEAR`], the `*_tangent` angle (in degrees) uses the slope of the curve halfway to the adjacent point. Allows custom assignments to the `*_tangent` angle if `*_mode` is set to [`TangentMode::FREE`][`crate::classes::curve::TangentMode::FREE`]."]
        #[inline]
        pub fn add_point(&mut self, position: Vector2,) -> i32 {
            self.add_point_ex(position,) . done()
        }
        #[doc = "Adds a point to the curve. For each side, if the `*_mode` is [`TangentMode::LINEAR`][`crate::classes::curve::TangentMode::LINEAR`], the `*_tangent` angle (in degrees) uses the slope of the curve halfway to the adjacent point. Allows custom assignments to the `*_tangent` angle if `*_mode` is set to [`TangentMode::FREE`][`crate::classes::curve::TangentMode::FREE`]."]
        #[inline]
        pub fn add_point_ex < 'ex > (&'ex mut self, position: Vector2,) -> ExAddPoint < 'ex > {
            ExAddPoint::new(self, position,)
        }
        #[doc = "Removes the point at `index` from the curve."]
        pub fn remove_point(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "remove_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all points from the curve."]
        pub fn clear_points(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "clear_points", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the curve coordinates for the point at `index`."]
        pub fn get_point_position(&self, index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Assigns the vertical position `y` to the point at `index`."]
        pub fn set_point_value(&mut self, index: i32, y: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the offset from `0.5`."]
        pub fn set_point_offset(&mut self, index: i32, offset: f32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f32,);
            let args = (index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the Y value for the point that would exist at the X position `offset` along the curve."]
        pub fn sample(&self, offset: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "sample", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the Y value for the point that would exist at the X position `offset` along the curve using the baked cache. Bakes the curve's points if not already baked."]
        pub fn sample_baked(&self, offset: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "sample_baked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the left tangent angle (in degrees) for the point at `index`."]
        pub fn get_point_left_tangent(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_left_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the right tangent angle (in degrees) for the point at `index`."]
        pub fn get_point_right_tangent(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_right_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the left \\[enum TangentMode] for the point at `index`."]
        pub fn get_point_left_mode(&self, index: i32,) -> crate::classes::curve::TangentMode {
            type CallRet = crate::classes::curve::TangentMode;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_left_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the right \\[enum TangentMode] for the point at `index`."]
        pub fn get_point_right_mode(&self, index: i32,) -> crate::classes::curve::TangentMode {
            type CallRet = crate::classes::curve::TangentMode;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_point_right_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the left tangent angle for the point at `index` to `tangent`."]
        pub fn set_point_left_tangent(&mut self, index: i32, tangent: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_left_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the right tangent angle for the point at `index` to `tangent`."]
        pub fn set_point_right_tangent(&mut self, index: i32, tangent: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_right_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the left \\[enum TangentMode] for the point at `index` to `mode`."]
        pub fn set_point_left_mode(&mut self, index: i32, mode: crate::classes::curve::TangentMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::curve::TangentMode,);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_left_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the right \\[enum TangentMode] for the point at `index` to `mode`."]
        pub fn set_point_right_mode(&mut self, index: i32, mode: crate::classes::curve::TangentMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::curve::TangentMode,);
            let args = (index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_point_right_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_value(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_min_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_value(&mut self, min: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_min_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_value(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_max_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_value(&mut self, max: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_max_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the difference between \\[member min_value] and \\[member max_value]."]
        pub fn get_value_range(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_value_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_domain(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_min_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_domain(&mut self, min: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (min,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_min_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_domain(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_max_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_domain(&mut self, max: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_max_domain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the difference between \\[member min_domain] and \\[member max_domain]."]
        pub fn get_domain_range(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_domain_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes duplicate points, i.e. points that are less than 0.00001 units (engine epsilon value) away from their neighbor on the curve."]
        pub fn clean_dupes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "clean_dupes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Recomputes the baked cache of points for the curve."]
        pub fn bake(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "bake", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_resolution(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "get_bake_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_resolution(&mut self, resolution: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (resolution,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve", "set_bake_resolution", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Curve {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Curve"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Curve {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Curve {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Curve {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Curve {
        
    }
    impl crate::obj::cap::GodotDefault for Curve {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Curve {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Curve {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Curve`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Curve__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Curve > for $Class {
                
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
#[doc = "Default-param extender for [`Curve::add_point_ex`][super::Curve::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Curve, position: Vector2, left_tangent: f32, right_tangent: f32, left_mode: crate::classes::curve::TangentMode, right_mode: crate::classes::curve::TangentMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPoint < 'ex > {
    fn new(surround_object: &'ex mut re_export::Curve, position: Vector2,) -> Self {
        let left_tangent = 0f32;
        let right_tangent = 0f32;
        let left_mode = crate::obj::EngineEnum::from_ord(0);
        let right_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, left_tangent: left_tangent, right_tangent: right_tangent, left_mode: left_mode, right_mode: right_mode,
        }
    }
    #[inline]
    pub fn left_tangent(self, left_tangent: f32) -> Self {
        Self {
            left_tangent: left_tangent, .. self
        }
    }
    #[inline]
    pub fn right_tangent(self, right_tangent: f32) -> Self {
        Self {
            right_tangent: right_tangent, .. self
        }
    }
    #[inline]
    pub fn left_mode(self, left_mode: crate::classes::curve::TangentMode) -> Self {
        Self {
            left_mode: left_mode, .. self
        }
    }
    #[inline]
    pub fn right_mode(self, right_mode: crate::classes::curve::TangentMode) -> Self {
        Self {
            right_mode: right_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, position, left_tangent, right_tangent, left_mode, right_mode,
        }
        = self;
        re_export::Curve::add_point_full(surround_object, position, left_tangent, right_tangent, left_mode, right_mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TangentMode {
    ord: i32
}
impl TangentMode {
    #[doc(alias = "TANGENT_FREE")]
    #[doc = "Godot enumerator name: `TANGENT_FREE`"]
    pub const FREE: TangentMode = TangentMode {
        ord: 0i32
    };
    #[doc(alias = "TANGENT_LINEAR")]
    #[doc = "Godot enumerator name: `TANGENT_LINEAR`"]
    pub const LINEAR: TangentMode = TangentMode {
        ord: 1i32
    };
    #[doc(alias = "TANGENT_MODE_COUNT")]
    #[doc = "Godot enumerator name: `TANGENT_MODE_COUNT`"]
    pub const MODE_COUNT: TangentMode = TangentMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TangentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TangentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TangentMode {
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
            Self::FREE => "FREE", Self::LINEAR => "LINEAR", Self::MODE_COUNT => "MODE_COUNT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TangentMode::FREE, TangentMode::LINEAR, TangentMode::MODE_COUNT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TangentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FREE", "TANGENT_FREE", TangentMode::FREE), crate::meta::inspect::EnumConstant::new("LINEAR", "TANGENT_LINEAR", TangentMode::LINEAR), crate::meta::inspect::EnumConstant::new("MODE_COUNT", "TANGENT_MODE_COUNT", TangentMode::MODE_COUNT)]
        }
    }
}
impl crate::meta::GodotConvert for TangentMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tangent Free", 0i64), EnumeratorShape::new_int("Tangent Linear", 1i64), EnumeratorShape::new_int("Tangent Mode Count", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Curve.TangentMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TangentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TangentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TangentMode {
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
impl crate::registry::property::Export for TangentMode {
    
}
impl crate::meta::Element for TangentMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Curve;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Curve`][crate::classes::Curve] class."]
    pub struct SignalsOfCurve < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCurve < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn range_changed(&mut self) -> SigRangeChanged < 'c, C > {
            SigRangeChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "range_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn domain_changed(&mut self) -> SigDomainChanged < 'c, C > {
            SigDomainChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "domain_changed")
            }
        }
    }
    type TypedSigRangeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigRangeChanged < 'c, C: WithSignals > {
        typed: TypedSigRangeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRangeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRangeChanged < 'c, C > {
        type Target = TypedSigRangeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRangeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDomainChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDomainChanged < 'c, C: WithSignals > {
        typed: TypedSigDomainChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDomainChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDomainChanged < 'c, C > {
        type Target = TypedSigDomainChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDomainChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Curve {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCurve < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCurve < 'c, C > {
        type Target = < < Curve as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Curve;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCurve < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Curve;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}