#![doc = "Sidecar module for class [`AStar2D`][crate::classes::AStar2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AStar2D` enums](https://docs.godotengine.org/en/stable/classes/class_astar2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AStar2D`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`a_star_2d`][crate::classes::a_star_2d]: sidecar module with related enum/flag types\n* [`IAStar2D`][crate::classes::IAStar2D]: virtual methods\n\n\nSee also [Godot docs for `AStar2D`](https://docs.godotengine.org/en/stable/classes/class_astar2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AStar2D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAn implementation of the A* algorithm, used to find the shortest path between two vertices on a connected graph in 2D space.\n\nSee [`AStar3D`][crate::classes::AStar3D] for a more thorough explanation on how to use this class. `AStar2D` is a wrapper for [`AStar3D`][crate::classes::AStar3D] that enforces 2D coordinates."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AStar2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AStar2D`][crate::classes::AStar2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AStar2D` methods](https://docs.godotengine.org/en/stable/classes/class_astar2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAStar2D: crate::obj::GodotClass < Base = AStar2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when neighboring enters processing and if \\[member neighbor_filter_enabled] is `true`. If `true` is returned the point will not be processed.\n\nNote that this function is hidden in the default `AStar2D` class."]
        fn filter_neighbor(&self, from_id: i64, neighbor_id: i64,) -> bool {
            unimplemented !()
        }
        #[doc = "Called when estimating the cost between a point and the path's ending point.\n\nNote that this function is hidden in the default `AStar2D` class."]
        fn estimate_cost(&self, from_id: i64, end_id: i64,) -> f32 {
            unimplemented !()
        }
        #[doc = "Called when computing the cost between two connected points.\n\nNote that this function is hidden in the default `AStar2D` class."]
        fn compute_cost(&self, from_id: i64, to_id: i64,) -> f32 {
            unimplemented !()
        }
    }
    impl AStar2D {
        #[doc = "Returns the next available point ID with no point associated to it."]
        pub fn get_available_point_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_available_point_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new point at the given position with the given identifier. The `id` must be 0 or larger, and the `weight_scale` must be 0.0 or greater.\n\nThe `weight_scale` is multiplied by the result of [`compute_cost`][`crate::classes::IAStar2D::compute_cost`] when determining the overall cost of traveling across a segment from a neighboring point to this point. Thus, all else being equal, the algorithm prefers points with lower `weight_scale`s to form a path.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 0), 4) # Adds the point (1, 0) with weight_scale 4 and id 1\n```\n\n\nIf there already exists a point for the given `id`, its position and weight scale are updated to the given values."]
        pub(crate) fn add_point_full(&mut self, id: i64, position: Vector2, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (i64, Vector2, f32,);
            let args = (id, position, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "add_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_point_ex`][Self::add_point_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new point at the given position with the given identifier. The `id` must be 0 or larger, and the `weight_scale` must be 0.0 or greater.\n\nThe `weight_scale` is multiplied by the result of [`compute_cost`][`crate::classes::IAStar2D::compute_cost`] when determining the overall cost of traveling across a segment from a neighboring point to this point. Thus, all else being equal, the algorithm prefers points with lower `weight_scale`s to form a path.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 0), 4) # Adds the point (1, 0) with weight_scale 4 and id 1\n```\n\n\nIf there already exists a point for the given `id`, its position and weight scale are updated to the given values."]
        #[inline]
        pub fn add_point(&mut self, id: i64, position: Vector2,) {
            self.add_point_ex(id, position,) . done()
        }
        #[doc = "Adds a new point at the given position with the given identifier. The `id` must be 0 or larger, and the `weight_scale` must be 0.0 or greater.\n\nThe `weight_scale` is multiplied by the result of [`compute_cost`][`crate::classes::IAStar2D::compute_cost`] when determining the overall cost of traveling across a segment from a neighboring point to this point. Thus, all else being equal, the algorithm prefers points with lower `weight_scale`s to form a path.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 0), 4) # Adds the point (1, 0) with weight_scale 4 and id 1\n```\n\n\nIf there already exists a point for the given `id`, its position and weight scale are updated to the given values."]
        #[inline]
        pub fn add_point_ex < 'ex > (&'ex mut self, id: i64, position: Vector2,) -> ExAddPoint < 'ex > {
            ExAddPoint::new(self, id, position,)
        }
        #[doc = "Returns the position of the point associated with the given `id`."]
        pub fn get_point_position(&self, id: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `position` for the point with the given `id`."]
        pub fn set_point_position(&mut self, id: i64, position: Vector2,) {
            type CallRet = ();
            type CallParams = (i64, Vector2,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "set_point_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the weight scale of the point associated with the given `id`."]
        pub fn get_point_weight_scale(&self, id: i64,) -> f32 {
            type CallRet = f32;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_weight_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `weight_scale` for the point with the given `id`. The `weight_scale` is multiplied by the result of [`compute_cost`][`crate::classes::IAStar2D::compute_cost`] when determining the overall cost of traveling across a segment from a neighboring point to this point."]
        pub fn set_point_weight_scale(&mut self, id: i64, weight_scale: f32,) {
            type CallRet = ();
            type CallParams = (i64, f32,);
            let args = (id, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "set_point_weight_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the point associated with the given `id` from the points pool."]
        pub fn remove_point(&mut self, id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "remove_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether a point associated with the given `id` exists."]
        pub fn has_point(&self, id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "has_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with the IDs of the points that form the connection with the given point.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(0, 0))\nastar.add_point(2, Vector2(0, 1))\nastar.add_point(3, Vector2(1, 1))\nastar.add_point(4, Vector2(2, 0))\n\nastar.connect_points(1, 2, true)\nastar.connect_points(1, 3, true)\n\nvar neighbors = astar.get_point_connections(1) # Returns [2, 3]\n```\n"]
        pub fn get_point_connections(&self, id: i64,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_connections", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of all point IDs."]
        pub fn get_point_ids(&self,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_ids", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_neighbor_filter_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "set_neighbor_filter_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_neighbor_filter_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "is_neighbor_filter_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disables or enables the specified point for pathfinding. Useful for making a temporary obstacle."]
        pub(crate) fn set_point_disabled_full(&mut self, id: i64, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i64, bool,);
            let args = (id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "set_point_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_point_disabled_ex`][Self::set_point_disabled_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Disables or enables the specified point for pathfinding. Useful for making a temporary obstacle."]
        #[inline]
        pub fn set_point_disabled(&mut self, id: i64,) {
            self.set_point_disabled_ex(id,) . done()
        }
        #[doc = "Disables or enables the specified point for pathfinding. Useful for making a temporary obstacle."]
        #[inline]
        pub fn set_point_disabled_ex < 'ex > (&'ex mut self, id: i64,) -> ExSetPointDisabled < 'ex > {
            ExSetPointDisabled::new(self, id,)
        }
        #[doc = "Returns whether a point is disabled or not for pathfinding. By default, all points are enabled."]
        pub fn is_point_disabled(&self, id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "is_point_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is allowed, not the reverse direction.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 1))\nastar.add_point(2, Vector2(0, 5))\nastar.connect_points(1, 2, false)\n```\n"]
        pub(crate) fn connect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type CallRet = ();
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "connect_points", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`connect_points_ex`][Self::connect_points_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is allowed, not the reverse direction.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 1))\nastar.add_point(2, Vector2(0, 5))\nastar.connect_points(1, 2, false)\n```\n"]
        #[inline]
        pub fn connect_points(&mut self, id: i64, to_id: i64,) {
            self.connect_points_ex(id, to_id,) . done()
        }
        #[doc = "Creates a segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is allowed, not the reverse direction.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(1, 1))\nastar.add_point(2, Vector2(0, 5))\nastar.connect_points(1, 2, false)\n```\n"]
        #[inline]
        pub fn connect_points_ex < 'ex > (&'ex mut self, id: i64, to_id: i64,) -> ExConnectPoints < 'ex > {
            ExConnectPoints::new(self, id, to_id,)
        }
        #[doc = "Deletes the segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is prevented, and a unidirectional segment possibly remains."]
        pub(crate) fn disconnect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type CallRet = ();
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "disconnect_points", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`disconnect_points_ex`][Self::disconnect_points_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Deletes the segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is prevented, and a unidirectional segment possibly remains."]
        #[inline]
        pub fn disconnect_points(&mut self, id: i64, to_id: i64,) {
            self.disconnect_points_ex(id, to_id,) . done()
        }
        #[doc = "Deletes the segment between the given points. If `bidirectional` is `false`, only movement from `id` to `to_id` is prevented, and a unidirectional segment possibly remains."]
        #[inline]
        pub fn disconnect_points_ex < 'ex > (&'ex mut self, id: i64, to_id: i64,) -> ExDisconnectPoints < 'ex > {
            ExDisconnectPoints::new(self, id, to_id,)
        }
        #[doc = "Returns whether there is a connection/segment between the given points. If `bidirectional` is `false`, returns whether movement from `id` to `to_id` is possible through this segment."]
        pub(crate) fn are_points_connected_full(&self, id: i64, to_id: i64, bidirectional: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i64, i64, bool,);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "are_points_connected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`are_points_connected_ex`][Self::are_points_connected_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns whether there is a connection/segment between the given points. If `bidirectional` is `false`, returns whether movement from `id` to `to_id` is possible through this segment."]
        #[inline]
        pub fn are_points_connected(&self, id: i64, to_id: i64,) -> bool {
            self.are_points_connected_ex(id, to_id,) . done()
        }
        #[doc = "Returns whether there is a connection/segment between the given points. If `bidirectional` is `false`, returns whether movement from `id` to `to_id` is possible through this segment."]
        #[inline]
        pub fn are_points_connected_ex < 'ex > (&'ex self, id: i64, to_id: i64,) -> ExArePointsConnected < 'ex > {
            ExArePointsConnected::new(self, id, to_id,)
        }
        #[doc = "Returns the number of points currently in the points pool."]
        pub fn get_point_count(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the capacity of the structure backing the points, useful in conjunction with [`reserve_space`][`crate::classes::AStar2D::reserve_space`]."]
        pub fn get_point_capacity(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_capacity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reserves space internally for `num_nodes` points. Useful if you're adding a known large number of points at once, such as points on a grid."]
        pub fn reserve_space(&mut self, num_nodes: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (num_nodes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "reserve_space", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all the points and segments."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the closest point to `to_position`, optionally taking disabled points into account. Returns `-1` if there are no points in the points pool.\n\n**Note:** If several points are the closest to `to_position`, the one with the smallest ID will be returned, ensuring a deterministic result."]
        pub(crate) fn get_closest_point_full(&self, to_position: Vector2, include_disabled: bool,) -> i64 {
            type CallRet = i64;
            type CallParams = (Vector2, bool,);
            let args = (to_position, include_disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_closest_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_closest_point_ex`][Self::get_closest_point_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the ID of the closest point to `to_position`, optionally taking disabled points into account. Returns `-1` if there are no points in the points pool.\n\n**Note:** If several points are the closest to `to_position`, the one with the smallest ID will be returned, ensuring a deterministic result."]
        #[inline]
        pub fn get_closest_point(&self, to_position: Vector2,) -> i64 {
            self.get_closest_point_ex(to_position,) . done()
        }
        #[doc = "Returns the ID of the closest point to `to_position`, optionally taking disabled points into account. Returns `-1` if there are no points in the points pool.\n\n**Note:** If several points are the closest to `to_position`, the one with the smallest ID will be returned, ensuring a deterministic result."]
        #[inline]
        pub fn get_closest_point_ex < 'ex > (&'ex self, to_position: Vector2,) -> ExGetClosestPoint < 'ex > {
            ExGetClosestPoint::new(self, to_position,)
        }
        #[doc = "Returns the closest position to `to_position` that resides inside a segment between two connected points.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(0, 0))\nastar.add_point(2, Vector2(0, 5))\nastar.connect_points(1, 2)\nvar res = astar.get_closest_position_in_segment(Vector2(3, 3)) # Returns (0, 3)\n```\n\n\nThe result is in the segment that goes from `y = 0` to `y = 5`. It's the closest position in the segment to the given point."]
        pub fn get_closest_position_in_segment(&self, to_position: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_closest_position_in_segment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with the points that are in the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** This method is not thread-safe; it can only be used from a single `Thread` at a given time. Consider using `Mutex` to ensure exclusive access to one thread to avoid race conditions.\n\nAdditionally, when `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish."]
        pub(crate) fn get_point_path_full(&self, from_id: i64, to_id: i64, allow_partial_path: bool,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i64, i64, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_point_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_point_path_ex`][Self::get_point_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array with the points that are in the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** This method is not thread-safe; it can only be used from a single `Thread` at a given time. Consider using `Mutex` to ensure exclusive access to one thread to avoid race conditions.\n\nAdditionally, when `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish."]
        #[inline]
        pub fn get_point_path(&self, from_id: i64, to_id: i64,) -> PackedVector2Array {
            self.get_point_path_ex(from_id, to_id,) . done()
        }
        #[doc = "Returns an array with the points that are in the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** This method is not thread-safe; it can only be used from a single `Thread` at a given time. Consider using `Mutex` to ensure exclusive access to one thread to avoid race conditions.\n\nAdditionally, when `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish."]
        #[inline]
        pub fn get_point_path_ex < 'ex > (&'ex self, from_id: i64, to_id: i64,) -> ExGetPointPath < 'ex > {
            ExGetPointPath::new(self, from_id, to_id,)
        }
        #[doc = "Returns an array with the IDs of the points that form the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** When `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(0, 0))\nastar.add_point(2, Vector2(0, 1), 1) # Default weight is 1\nastar.add_point(3, Vector2(1, 1))\nastar.add_point(4, Vector2(2, 0))\n\nastar.connect_points(1, 2, false)\nastar.connect_points(2, 3, false)\nastar.connect_points(4, 3, false)\nastar.connect_points(1, 4, false)\n\nvar res = astar.get_id_path(1, 3) # Returns [1, 2, 3]\n```\n\n\nIf you change the 2nd point's weight to 3, then the result will be `[1, 4, 3]` instead, because now even though the distance is longer, it's \"easier\" to get through point 4 than through point 2."]
        pub(crate) fn get_id_path_full(&self, from_id: i64, to_id: i64, allow_partial_path: bool,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (i64, i64, bool,);
            let args = (from_id, to_id, allow_partial_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AStar2D", "get_id_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_id_path_ex`][Self::get_id_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array with the IDs of the points that form the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** When `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(0, 0))\nastar.add_point(2, Vector2(0, 1), 1) # Default weight is 1\nastar.add_point(3, Vector2(1, 1))\nastar.add_point(4, Vector2(2, 0))\n\nastar.connect_points(1, 2, false)\nastar.connect_points(2, 3, false)\nastar.connect_points(4, 3, false)\nastar.connect_points(1, 4, false)\n\nvar res = astar.get_id_path(1, 3) # Returns [1, 2, 3]\n```\n\n\nIf you change the 2nd point's weight to 3, then the result will be `[1, 4, 3]` instead, because now even though the distance is longer, it's \"easier\" to get through point 4 than through point 2."]
        #[inline]
        pub fn get_id_path(&self, from_id: i64, to_id: i64,) -> PackedInt64Array {
            self.get_id_path_ex(from_id, to_id,) . done()
        }
        #[doc = "Returns an array with the IDs of the points that form the path found by AStar2D between the given points. The array is ordered from the starting point to the ending point of the path.\n\nIf `from_id` point is disabled, returns an empty array (even if `from_id == to_id`).\n\nIf `from_id` point is not disabled, there is no valid path to the target, and `allow_partial_path` is `true`, returns a path to the point closest to the target that can be reached.\n\n**Note:** When `allow_partial_path` is `true` and `to_id` is disabled the search may take an unusually long time to finish.\n\n\n```gdscript\nvar astar = AStar2D.new()\nastar.add_point(1, Vector2(0, 0))\nastar.add_point(2, Vector2(0, 1), 1) # Default weight is 1\nastar.add_point(3, Vector2(1, 1))\nastar.add_point(4, Vector2(2, 0))\n\nastar.connect_points(1, 2, false)\nastar.connect_points(2, 3, false)\nastar.connect_points(4, 3, false)\nastar.connect_points(1, 4, false)\n\nvar res = astar.get_id_path(1, 3) # Returns [1, 2, 3]\n```\n\n\nIf you change the 2nd point's weight to 3, then the result will be `[1, 4, 3]` instead, because now even though the distance is longer, it's \"easier\" to get through point 4 than through point 2."]
        #[inline]
        pub fn get_id_path_ex < 'ex > (&'ex self, from_id: i64, to_id: i64,) -> ExGetIdPath < 'ex > {
            ExGetIdPath::new(self, from_id, to_id,)
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
    impl crate::obj::GodotClass for AStar2D {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AStar2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AStar2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AStar2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AStar2D {
        
    }
    impl crate::obj::cap::GodotDefault for AStar2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AStar2D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AStar2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AStar2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AStar2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AStar2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AStar2D::add_point_ex`][super::AStar2D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AStar2D, id: i64, position: Vector2, weight_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPoint < 'ex > {
    fn new(surround_object: &'ex mut re_export::AStar2D, id: i64, position: Vector2,) -> Self {
        let weight_scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, position: position, weight_scale: weight_scale,
        }
    }
    #[inline]
    pub fn weight_scale(self, weight_scale: f32) -> Self {
        Self {
            weight_scale: weight_scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, position, weight_scale,
        }
        = self;
        re_export::AStar2D::add_point_full(surround_object, id, position, weight_scale,)
    }
}
#[doc = "Default-param extender for [`AStar2D::set_point_disabled_ex`][super::AStar2D::set_point_disabled_ex]."]
#[must_use]
pub struct ExSetPointDisabled < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AStar2D, id: i64, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetPointDisabled < 'ex > {
    fn new(surround_object: &'ex mut re_export::AStar2D, id: i64,) -> Self {
        let disabled = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, disabled: disabled,
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, disabled,
        }
        = self;
        re_export::AStar2D::set_point_disabled_full(surround_object, id, disabled,)
    }
}
#[doc = "Default-param extender for [`AStar2D::connect_points_ex`][super::AStar2D::connect_points_ex]."]
#[must_use]
pub struct ExConnectPoints < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AStar2D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExConnectPoints < 'ex > {
    fn new(surround_object: &'ex mut re_export::AStar2D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar2D::connect_points_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar2D::disconnect_points_ex`][super::AStar2D::disconnect_points_ex]."]
#[must_use]
pub struct ExDisconnectPoints < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AStar2D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDisconnectPoints < 'ex > {
    fn new(surround_object: &'ex mut re_export::AStar2D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar2D::disconnect_points_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar2D::are_points_connected_ex`][super::AStar2D::are_points_connected_ex]."]
#[must_use]
pub struct ExArePointsConnected < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::AStar2D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExArePointsConnected < 'ex > {
    fn new(surround_object: &'ex re_export::AStar2D, id: i64, to_id: i64,) -> Self {
        let bidirectional = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, to_id: to_id, bidirectional: bidirectional,
        }
    }
    #[inline]
    pub fn bidirectional(self, bidirectional: bool) -> Self {
        Self {
            bidirectional: bidirectional, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, id, to_id, bidirectional,
        }
        = self;
        re_export::AStar2D::are_points_connected_full(surround_object, id, to_id, bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar2D::get_closest_point_ex`][super::AStar2D::get_closest_point_ex]."]
#[must_use]
pub struct ExGetClosestPoint < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::AStar2D, to_position: Vector2, include_disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetClosestPoint < 'ex > {
    fn new(surround_object: &'ex re_export::AStar2D, to_position: Vector2,) -> Self {
        let include_disabled = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position, include_disabled: include_disabled,
        }
    }
    #[inline]
    pub fn include_disabled(self, include_disabled: bool) -> Self {
        Self {
            include_disabled: include_disabled, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, to_position, include_disabled,
        }
        = self;
        re_export::AStar2D::get_closest_point_full(surround_object, to_position, include_disabled,)
    }
}
#[doc = "Default-param extender for [`AStar2D::get_point_path_ex`][super::AStar2D::get_point_path_ex]."]
#[must_use]
pub struct ExGetPointPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::AStar2D, from_id: i64, to_id: i64, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPointPath < 'ex > {
    fn new(surround_object: &'ex re_export::AStar2D, from_id: i64, to_id: i64,) -> Self {
        let allow_partial_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_id: from_id, to_id: to_id, allow_partial_path: allow_partial_path,
        }
    }
    #[inline]
    pub fn allow_partial_path(self, allow_partial_path: bool) -> Self {
        Self {
            allow_partial_path: allow_partial_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector2Array {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStar2D::get_point_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
#[doc = "Default-param extender for [`AStar2D::get_id_path_ex`][super::AStar2D::get_id_path_ex]."]
#[must_use]
pub struct ExGetIdPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::AStar2D, from_id: i64, to_id: i64, allow_partial_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetIdPath < 'ex > {
    fn new(surround_object: &'ex re_export::AStar2D, from_id: i64, to_id: i64,) -> Self {
        let allow_partial_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_id: from_id, to_id: to_id, allow_partial_path: allow_partial_path,
        }
    }
    #[inline]
    pub fn allow_partial_path(self, allow_partial_path: bool) -> Self {
        Self {
            allow_partial_path: allow_partial_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, from_id, to_id, allow_partial_path,
        }
        = self;
        re_export::AStar2D::get_id_path_full(surround_object, from_id, to_id, allow_partial_path,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AStar2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AStar2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}