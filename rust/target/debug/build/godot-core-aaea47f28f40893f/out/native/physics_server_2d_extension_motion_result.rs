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
use std::ffi::c_void;
#[doc = r" Native structure; can be passed via pointer in APIs that are not exposed to GDScript."]
#[doc = r""]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer2DExtensionMotionResult {
    pub travel: Vector2, pub remainder: Vector2, pub collision_point: Vector2, pub collision_normal: Vector2, pub collider_velocity: Vector2, pub collision_depth: real, pub collision_safe_fraction: real, pub collision_unsafe_fraction: real, pub collision_local_shape: i32, pub collider_id: ObjectId, pub collider: Rid, pub collider_shape: i32,
}
impl PhysicsServer2DExtensionMotionResult {
    
}