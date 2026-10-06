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
pub struct Glyph {
    pub start: i32, pub end: i32, pub count: u8, pub repeat: u8, pub flags: u16, pub x_off: f32, pub y_off: f32, pub advance: f32, pub font_rid: Rid, pub font_size: i32, pub index: i32, pub span_index: i32,
}
impl Glyph {
    
}