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
pub(super) mod re_export {
    use super::*;
    #[doc(hidden)]
    #[repr(transparent)]
    pub struct InnerTransform2D < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerTransform2D < 'inner > {
    pub fn from_outer(outer: &Transform2D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the [inverted version of this transform](https://en.wikipedia.org/wiki/Invertible_matrix).\n\n**Note:** For this method to return correctly, the transform's basis needs to be _orthonormal_ (see \\[method orthonormalized]). That means the basis should only represent a rotation. If it does not, use \\[method affine_inverse] instead."]
    pub fn inverse(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(274usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the inverted version of this transform. Unlike \\[method inverse], this method works with almost any basis, including non-uniform ones, but is slower.\n\n**Note:** For this method to return correctly, the transform's basis needs to have a determinant that is not exactly `0.0` (see \\[method determinant])."]
    pub fn affine_inverse(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(275usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "affine_inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this transform's rotation (in radians). This is equivalent to \\[member x]'s angle (see [`angle`][`crate::builtin::Vector2::angle`])."]
    pub fn get_rotation(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(276usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_rotation", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this transform's translation. Equivalent to \\[member origin]."]
    pub fn get_origin(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(277usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_origin", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length of both \\[member x] and \\[member y], as a [`Vector2`][crate::builtin::Vector2]. If this transform's basis is not skewed, this value is the scaling factor. It is not affected by rotation.\n\n\n```gdscript\nvar my_transform = Transform2D(\n\tVector2(2, 0),\n\tVector2(0, 4),\n\tVector2(0, 0)\n)\n# Rotating the Transform2D in any way preserves its scale.\nmy_transform = my_transform.rotated(TAU / 2)\n\nprint(my_transform.get_scale()) # Prints (2.0, 4.0)\n```\n\n\n**Note:** If the value returned by \\[method determinant] is negative, the scale is also negative."]
    pub fn get_scale(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(278usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_scale", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this transform's skew (in radians)."]
    pub fn get_skew(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(279usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_skew", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform with its basis orthonormalized. An orthonormal basis is both _orthogonal_ (the axes are perpendicular to each other) and _normalized_ (the axes have a length of `1.0`), which also means it can only represent a rotation."]
    pub fn orthonormalized(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(280usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "orthonormalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform rotated by the given `angle` (in radians).\n\nIf `angle` is positive, the transform is rotated clockwise.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the left, i.e., `R * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn rotated(&self, angle: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(281usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "rotated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform rotated by the given `angle` (in radians).\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the right, i.e., `X * R`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn rotated_local(&self, angle: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(282usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "rotated_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform scaled by the given `scale` factor.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the left, i.e., `S * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn scaled(&self, scale: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(283usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "scaled", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform scaled by the given `scale` factor.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the right, i.e., `X * S`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn scaled_local(&self, scale: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(284usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "scaled_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform translated by the given `offset`.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the left, i.e., `T * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn translated(&self, offset: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(285usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "translated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform translated by the given `offset`.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the right, i.e., `X * T`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn translated_local(&self, offset: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(286usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "translated_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [determinant](https://en.wikipedia.org/wiki/Determinant) of this transform basis's matrix. For advanced math, this number can be used to determine a few attributes:\n\n- If the determinant is exactly `0.0`, the basis is not invertible (see \\[method inverse]).\n\n- If the determinant is a negative number, the basis represents a negative scale.\n\n**Note:** If the basis's scale is the same for every axis, its determinant is always that scale by the power of 2."]
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(287usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "determinant", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the `v` vector, transformed (multiplied) by the transform basis's matrix. Unlike the multiplication operator (`*`), this method ignores the \\[member origin]."]
    pub fn basis_xform(&self, v: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(288usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "basis_xform", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the `v` vector, transformed (multiplied) by the inverse transform basis's matrix (see \\[method inverse]). This method ignores the \\[member origin].\n\n**Note:** This method assumes that this transform's basis is _orthonormal_ (see \\[method orthonormalized]). If the basis is not orthonormal, `transform.affine_inverse().basis_xform(vector)` should be used instead (see \\[method affine_inverse])."]
    pub fn basis_xform_inv(&self, v: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(289usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "basis_xform_inv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of the linear interpolation between this transform and `xform` by the given `weight`.\n\nThe `weight` should be between `0.0` and `1.0` (inclusive). Values outside this range are allowed and can be used to perform _extrapolation_ instead."]
    pub fn interpolate_with(&self, xform: Transform2D, weight: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Transform2D, f64,);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(290usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "interpolate_with", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this transform's basis is conformal. A conformal basis is both _orthogonal_ (the axes are perpendicular to each other) and _uniform_ (the axes share the same length). This method can be especially useful during physics calculations."]
    pub fn is_conformal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(291usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_conformal", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this transform and `xform` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, xform: Transform2D,) -> bool {
        type CallRet = bool;
        type CallParams = (Transform2D,);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(292usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this transform is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(293usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the transform rotated such that the rotated X-axis points towards the `target` position, in global space."]
    pub fn looking_at(&self, target: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (target,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(294usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "looking_at", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerTransform2D;
impl Transform2D {
    
}