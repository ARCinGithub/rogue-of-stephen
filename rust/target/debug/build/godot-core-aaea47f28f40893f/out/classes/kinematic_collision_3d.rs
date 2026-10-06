#![doc = "Sidecar module for class [`KinematicCollision3D`][crate::classes::KinematicCollision3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `KinematicCollision3D` enums](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `KinematicCollision3D`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`kinematic_collision_3d`][crate::classes::kinematic_collision_3d]: sidecar module with related enum/flag types\n* [`IKinematicCollision3D`][crate::classes::IKinematicCollision3D]: virtual methods\n\n\nSee also [Godot docs for `KinematicCollision3D`](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`KinematicCollision3D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nHolds collision data from the movement of a [`PhysicsBody3D`][crate::classes::PhysicsBody3D], usually from [`move_and_collide`][`crate::classes::PhysicsBody3D::move_and_collide`]. When a [`PhysicsBody3D`][crate::classes::PhysicsBody3D] is moved, it stops if it detects a collision with another body. If a collision is detected, a `KinematicCollision3D` object is returned.\n\nThe collision data includes the colliding object, the remaining motion, and the collision position. This data can be used to determine a custom response to the collision."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct KinematicCollision3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`KinematicCollision3D`][crate::classes::KinematicCollision3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `KinematicCollision3D` methods](https://docs.godotengine.org/en/stable/classes/class_kinematiccollision3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IKinematicCollision3D: crate::obj::GodotClass < Base = KinematicCollision3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl KinematicCollision3D {
        #[doc = "Returns the moving object's travel before collision."]
        pub fn get_travel(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_travel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the moving object's remaining movement vector."]
        pub fn get_remainder(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_remainder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the colliding body's length of overlap along the collision normal."]
        pub fn get_depth(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_depth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of detected collisions."]
        pub fn get_collision_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the point of collision in global coordinates given a collision index (the deepest collision by default)."]
        pub(crate) fn get_position_full(&self, collision_index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_position_ex`][Self::get_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the point of collision in global coordinates given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_position(&self,) -> Vector3 {
            self.get_position_ex() . done()
        }
        #[doc = "Returns the point of collision in global coordinates given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_position_ex < 'ex > (&'ex self,) -> ExGetPosition < 'ex > {
            ExGetPosition::new(self,)
        }
        #[doc = "Returns the colliding body's shape's normal at the point of collision given a collision index (the deepest collision by default)."]
        pub(crate) fn get_normal_full(&self, collision_index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_normal_ex`][Self::get_normal_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's shape's normal at the point of collision given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_normal(&self,) -> Vector3 {
            self.get_normal_ex() . done()
        }
        #[doc = "Returns the colliding body's shape's normal at the point of collision given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_normal_ex < 'ex > (&'ex self,) -> ExGetNormal < 'ex > {
            ExGetNormal::new(self,)
        }
        #[doc = "Returns the collision angle according to `up_direction`, which is `Vector3.UP` by default. This value is always positive."]
        pub(crate) fn get_angle_full(&self, collision_index: i32, up_direction: Vector3,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, Vector3,);
            let args = (collision_index, up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_angle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_angle_ex`][Self::get_angle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the collision angle according to `up_direction`, which is `Vector3.UP` by default. This value is always positive."]
        #[inline]
        pub fn get_angle(&self,) -> f32 {
            self.get_angle_ex() . done()
        }
        #[doc = "Returns the collision angle according to `up_direction`, which is `Vector3.UP` by default. This value is always positive."]
        #[inline]
        pub fn get_angle_ex < 'ex > (&'ex self,) -> ExGetAngle < 'ex > {
            ExGetAngle::new(self,)
        }
        #[doc = "Returns the moving object's colliding shape given a collision index (the deepest collision by default)."]
        pub(crate) fn get_local_shape_full(&self, collision_index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_local_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_local_shape_ex`][Self::get_local_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the moving object's colliding shape given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_local_shape(&self,) -> Option < Gd < crate::classes::Object > > {
            self.get_local_shape_ex() . done()
        }
        #[doc = "Returns the moving object's colliding shape given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_local_shape_ex < 'ex > (&'ex self,) -> ExGetLocalShape < 'ex > {
            ExGetLocalShape::new(self,)
        }
        #[doc = "Returns the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default)."]
        pub(crate) fn get_collider_full(&self, collision_index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_ex`][Self::get_collider_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider(&self,) -> Option < Gd < crate::classes::Object > > {
            self.get_collider_ex() . done()
        }
        #[doc = "Returns the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_ex < 'ex > (&'ex self,) -> ExGetCollider < 'ex > {
            ExGetCollider::new(self,)
        }
        #[doc = "Returns the unique instance ID of the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default). See [`instance_id`][`crate::obj::Gd::instance_id`]."]
        pub(crate) fn get_collider_id_full(&self, collision_index: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_id_ex`][Self::get_collider_id_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the unique instance ID of the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default). See [`instance_id`][`crate::obj::Gd::instance_id`]."]
        #[inline]
        pub fn get_collider_id(&self,) -> u64 {
            self.get_collider_id_ex() . done()
        }
        #[doc = "Returns the unique instance ID of the colliding body's attached [`Object`][crate::classes::Object] given a collision index (the deepest collision by default). See [`instance_id`][`crate::obj::Gd::instance_id`]."]
        #[inline]
        pub fn get_collider_id_ex < 'ex > (&'ex self,) -> ExGetColliderId < 'ex > {
            ExGetColliderId::new(self,)
        }
        #[doc = "Returns the colliding body's [`RID`][crate::builtin::Rid] used by the [`PhysicsServer3D`][crate::classes::PhysicsServer3D] given a collision index (the deepest collision by default)."]
        pub(crate) fn get_collider_rid_full(&self, collision_index: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_rid_ex`][Self::get_collider_rid_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's [`RID`][crate::builtin::Rid] used by the [`PhysicsServer3D`][crate::classes::PhysicsServer3D] given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_rid(&self,) -> Rid {
            self.get_collider_rid_ex() . done()
        }
        #[doc = "Returns the colliding body's [`RID`][crate::builtin::Rid] used by the [`PhysicsServer3D`][crate::classes::PhysicsServer3D] given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_rid_ex < 'ex > (&'ex self,) -> ExGetColliderRid < 'ex > {
            ExGetColliderRid::new(self,)
        }
        #[doc = "Returns the colliding body's shape given a collision index (the deepest collision by default)."]
        pub(crate) fn get_collider_shape_full(&self, collision_index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_shape_ex`][Self::get_collider_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's shape given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_shape(&self,) -> Option < Gd < crate::classes::Object > > {
            self.get_collider_shape_ex() . done()
        }
        #[doc = "Returns the colliding body's shape given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_shape_ex < 'ex > (&'ex self,) -> ExGetColliderShape < 'ex > {
            ExGetColliderShape::new(self,)
        }
        #[doc = "Returns the colliding body's shape index given a collision index (the deepest collision by default). See [`CollisionObject3D`][crate::classes::CollisionObject3D]."]
        pub(crate) fn get_collider_shape_index_full(&self, collision_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider_shape_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_shape_index_ex`][Self::get_collider_shape_index_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's shape index given a collision index (the deepest collision by default). See [`CollisionObject3D`][crate::classes::CollisionObject3D]."]
        #[inline]
        pub fn get_collider_shape_index(&self,) -> i32 {
            self.get_collider_shape_index_ex() . done()
        }
        #[doc = "Returns the colliding body's shape index given a collision index (the deepest collision by default). See [`CollisionObject3D`][crate::classes::CollisionObject3D]."]
        #[inline]
        pub fn get_collider_shape_index_ex < 'ex > (&'ex self,) -> ExGetColliderShapeIndex < 'ex > {
            ExGetColliderShapeIndex::new(self,)
        }
        #[doc = "Returns the colliding body's velocity given a collision index (the deepest collision by default)."]
        pub(crate) fn get_collider_velocity_full(&self, collision_index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "KinematicCollision3D", "get_collider_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_collider_velocity_ex`][Self::get_collider_velocity_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the colliding body's velocity given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_velocity(&self,) -> Vector3 {
            self.get_collider_velocity_ex() . done()
        }
        #[doc = "Returns the colliding body's velocity given a collision index (the deepest collision by default)."]
        #[inline]
        pub fn get_collider_velocity_ex < 'ex > (&'ex self,) -> ExGetColliderVelocity < 'ex > {
            ExGetColliderVelocity::new(self,)
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
    impl crate::obj::GodotClass for KinematicCollision3D {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("KinematicCollision3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for KinematicCollision3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for KinematicCollision3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for KinematicCollision3D {
        
    }
    impl crate::obj::cap::GodotDefault for KinematicCollision3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for KinematicCollision3D {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for KinematicCollision3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`KinematicCollision3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_KinematicCollision3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::KinematicCollision3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_position_ex`][super::KinematicCollision3D::get_position_ex]."]
#[must_use]
pub struct ExGetPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPosition < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_position_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_normal_ex`][super::KinematicCollision3D::get_normal_ex]."]
#[must_use]
pub struct ExGetNormal < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetNormal < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_normal_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_angle_ex`][super::KinematicCollision3D::get_angle_ex]."]
#[must_use]
pub struct ExGetAngle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32, up_direction: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetAngle < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        let up_direction = Vector3::new(0 as _, 1 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index, up_direction: up_direction,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn up_direction(self, up_direction: Vector3) -> Self {
        Self {
            up_direction: up_direction, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, collision_index, up_direction,
        }
        = self;
        re_export::KinematicCollision3D::get_angle_full(surround_object, collision_index, up_direction,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_local_shape_ex`][super::KinematicCollision3D::get_local_shape_ex]."]
#[must_use]
pub struct ExGetLocalShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetLocalShape < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Object > > {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_local_shape_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_ex`][super::KinematicCollision3D::get_collider_ex]."]
#[must_use]
pub struct ExGetCollider < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetCollider < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Object > > {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_id_ex`][super::KinematicCollision3D::get_collider_id_ex]."]
#[must_use]
pub struct ExGetColliderId < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetColliderId < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_id_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_rid_ex`][super::KinematicCollision3D::get_collider_rid_ex]."]
#[must_use]
pub struct ExGetColliderRid < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetColliderRid < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_rid_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_shape_ex`][super::KinematicCollision3D::get_collider_shape_ex]."]
#[must_use]
pub struct ExGetColliderShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetColliderShape < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Object > > {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_shape_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_shape_index_ex`][super::KinematicCollision3D::get_collider_shape_index_ex]."]
#[must_use]
pub struct ExGetColliderShapeIndex < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetColliderShapeIndex < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_shape_index_full(surround_object, collision_index,)
    }
}
#[doc = "Default-param extender for [`KinematicCollision3D::get_collider_velocity_ex`][super::KinematicCollision3D::get_collider_velocity_ex]."]
#[must_use]
pub struct ExGetColliderVelocity < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::KinematicCollision3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetColliderVelocity < 'ex > {
    fn new(surround_object: &'ex re_export::KinematicCollision3D,) -> Self {
        let collision_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, collision_index: collision_index,
        }
    }
    #[inline]
    pub fn collision_index(self, collision_index: i32) -> Self {
        Self {
            collision_index: collision_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, collision_index,
        }
        = self;
        re_export::KinematicCollision3D::get_collider_velocity_full(surround_object, collision_index,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::KinematicCollision3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for KinematicCollision3D {
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