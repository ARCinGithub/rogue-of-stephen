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
pub struct PhysicsServer2DExtensionShapeRestInfo {
    pub point: Vector2, pub normal: Vector2, pub rid: Rid, pub collider_id: ObjectId, pub shape: i32, pub linear_velocity: Vector2,
}
impl PhysicsServer2DExtensionShapeRestInfo {
    
}