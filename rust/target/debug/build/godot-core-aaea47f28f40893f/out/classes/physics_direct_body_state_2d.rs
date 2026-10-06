#![doc = "Sidecar module for class [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState2D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_body_state_2d`][crate::classes::physics_direct_body_state_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsDirectBodyState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectBodyState2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nProvides direct access to a physics body in the [`PhysicsServer2D`][crate::classes::PhysicsServer2D], allowing safe changes to physics properties. This object is passed via the direct state callback of [`RigidBody2D`][crate::classes::RigidBody2D], and is intended for changing the direct state of that body. See [`integrate_forces`][`crate::classes::IRigidBody2D::integrate_forces`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsDirectBodyState2D {
        pub fn get_total_gravity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass_local(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_center_of_mass_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_inverse_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_inertia(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_inverse_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's velocity at the given relative position, including both translation and rotation."]
        pub fn get_velocity_at_local_position(&self, local_position: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_velocity_at_local_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a directional impulse without affecting rotation.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\nThis is equivalent to using [`apply_impulse`][`crate::classes::PhysicsDirectBodyState2D::apply_impulse`] at the body's center of mass."]
        pub fn apply_central_impulse(&mut self, impulse: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a rotational impulse to the body without affecting the position.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n**Note:** \\[member inverse_inertia] is required for this to work. To have \\[member inverse_inertia], an active [`CollisionShape2D`][crate::classes::CollisionShape2D] must be a child of the node, or you can manually set \\[member inverse_inertia]."]
        pub fn apply_torque_impulse(&mut self, impulse: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_impulse_ex`][Self::apply_impulse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector2,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[doc = "Applies a positioned impulse to the body.\n\nAn impulse is time-independent! Applying an impulse every frame would result in a framerate-dependent force. For this reason, it should only be used when simulating one-time impacts (use the \"_force\" functions otherwise).\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_impulse_ex < 'ex > (&'ex mut self, impulse: Vector2,) -> ExApplyImpulse < 'ex > {
            ExApplyImpulse::new(self, impulse,)
        }
        #[doc = "Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`apply_force`][`crate::classes::PhysicsDirectBodyState2D::apply_force`] at the body's center of mass."]
        pub(crate) fn apply_central_force_full(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_central_force_ex`][Self::apply_central_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`apply_force`][`crate::classes::PhysicsDirectBodyState2D::apply_force`] at the body's center of mass."]
        #[inline]
        pub fn apply_central_force(&mut self,) {
            self.apply_central_force_ex() . done()
        }
        #[doc = "Applies a directional force without affecting rotation. A force is time dependent and meant to be applied every physics update.\n\nThis is equivalent to using [`apply_force`][`crate::classes::PhysicsDirectBodyState2D::apply_force`] at the body's center of mass."]
        #[inline]
        pub fn apply_central_force_ex < 'ex > (&'ex mut self,) -> ExApplyCentralForce < 'ex > {
            ExApplyCentralForce::new(self,)
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`apply_force_ex`][Self::apply_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector2,) {
            self.apply_force_ex(force,) . done()
        }
        #[doc = "Applies a positioned force to the body. A force is time dependent and meant to be applied every physics update.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn apply_force_ex < 'ex > (&'ex mut self, force: Vector2,) -> ExApplyForce < 'ex > {
            ExApplyForce::new(self, force,)
        }
        #[doc = "Applies a rotational force without affecting position. A force is time dependent and meant to be applied every physics update.\n\n**Note:** \\[member inverse_inertia] is required for this to work. To have \\[member inverse_inertia], an active [`CollisionShape2D`][crate::classes::CollisionShape2D] must be a child of the node, or you can manually set \\[member inverse_inertia]."]
        pub fn apply_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\nThis is equivalent to using [`add_constant_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_force`] at the body's center of mass."]
        pub(crate) fn add_constant_central_force_full(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_constant_central_force_ex`][Self::add_constant_central_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\nThis is equivalent to using [`add_constant_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_force`] at the body's center of mass."]
        #[inline]
        pub fn add_constant_central_force(&mut self,) {
            self.add_constant_central_force_ex() . done()
        }
        #[doc = "Adds a constant directional force without affecting rotation that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\nThis is equivalent to using [`add_constant_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_force`] at the body's center of mass."]
        #[inline]
        pub fn add_constant_central_force_ex < 'ex > (&'ex mut self,) -> ExAddConstantCentralForce < 'ex > {
            ExAddConstantCentralForce::new(self,)
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_constant_force_ex`][Self::add_constant_force_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector2,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[doc = "Adds a constant positioned force to the body that keeps being applied over time until cleared with `constant_force = Vector2(0, 0)`.\n\n`position` is the offset from the body origin in global coordinates."]
        #[inline]
        pub fn add_constant_force_ex < 'ex > (&'ex mut self, force: Vector2,) -> ExAddConstantForce < 'ex > {
            ExAddConstantForce::new(self, force,)
        }
        #[doc = "Adds a constant rotational force without affecting position that keeps being applied over time until cleared with `constant_torque = 0`."]
        pub fn add_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant positional forces applied during each physics update.\n\nSee [`add_constant_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_force`] and [`add_constant_central_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_central_force`]."]
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant positional forces applied during each physics update.\n\nSee [`add_constant_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_force`] and [`add_constant_central_force`][`crate::classes::PhysicsDirectBodyState2D::add_constant_central_force`]."]
        pub fn get_constant_force(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the body's total constant rotational forces applied during each physics update.\n\nSee [`add_constant_torque`][`crate::classes::PhysicsDirectBodyState2D::add_constant_torque`]."]
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the body's total constant rotational forces applied during each physics update.\n\nSee [`add_constant_torque`][`crate::classes::PhysicsDirectBodyState2D::add_constant_torque`]."]
        pub fn get_constant_torque(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sleep_state(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_sleep_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "is_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of contacts this body has with other bodies.\n\n**Note:** By default, this returns 0 unless bodies are configured to monitor contacts. See \\[member RigidBody2D.contact_monitor]."]
        pub fn get_contact_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position of the contact point on the body in the global coordinate system."]
        pub fn get_contact_local_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local normal at the contact point."]
        pub fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local shape index of the collision."]
        pub fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the velocity vector at the body's contact point."]
        pub fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collider's [`RID`][crate::builtin::Rid]."]
        pub fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position of the contact point on the collider in the global coordinate system."]
        pub fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collider's object id."]
        pub fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collider object. This depends on how it was created (will return a scene node if such was used to create it)."]
        pub fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collider's shape index."]
        pub fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the velocity vector at the collider's contact point."]
        pub fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the impulse created by the contact."]
        pub fn get_contact_impulse(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_step", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the body's linear and angular velocity by applying gravity and damping for the equivalent of one physics tick."]
        pub fn integrate_forces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "integrate_forces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current state of the space, useful for queries."]
        pub fn get_space_state(&self,) -> Gd < crate::classes::PhysicsDirectSpaceState2D > {
            type CallRet = Gd < crate::classes::PhysicsDirectSpaceState2D >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_space_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState2D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsDirectBodyState2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectBodyState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectBodyState2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsDirectBodyState2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsDirectBodyState2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_impulse_ex`][super::PhysicsDirectBodyState2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyImpulse < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_central_force_ex`][super::PhysicsDirectBodyState2D::apply_central_force_ex]."]
#[must_use]
pub struct ExApplyCentralForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyCentralForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectBodyState2D,) -> Self {
        let force = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector2) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_force_ex`][super::PhysicsDirectBodyState2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExApplyForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_central_force_ex`][super::PhysicsDirectBodyState2D::add_constant_central_force_ex]."]
#[must_use]
pub struct ExAddConstantCentralForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddConstantCentralForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectBodyState2D,) -> Self {
        let force = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector2) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::add_constant_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_force_ex`][super::PhysicsDirectBodyState2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddConstantForce < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::add_constant_force_full(surround_object, force, position,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsDirectBodyState2D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsDirectBodyState2D {
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