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
    pub struct InnerTransform3D < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerTransform3D < 'inner > {
    pub fn from_outer(outer: &Transform3D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the [inverted version of this transform](https://en.wikipedia.org/wiki/Invertible_matrix). See also [`inverse`][`crate::builtin::Basis::inverse`].\n\n**Note:** For this method to return correctly, the transform's \\[member basis] needs to be _orthonormal_ (see \\[method orthonormalized]). That means the basis should only represent a rotation. If it does not, use \\[method affine_inverse] instead."]
    pub fn inverse(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(418usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the inverted version of this transform. Unlike \\[method inverse], this method works with almost any \\[member basis], including non-uniform ones, but is slower. See also [`inverse`][`crate::builtin::Basis::inverse`].\n\n**Note:** For this method to return correctly, the transform's \\[member basis] needs to have a determinant that is not exactly `0.0` (see [`determinant`][`crate::builtin::Basis::determinant`])."]
    pub fn affine_inverse(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(419usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "affine_inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform with its \\[member basis] orthonormalized. An orthonormal basis is both _orthogonal_ (the axes are perpendicular to each other) and _normalized_ (the axes have a length of `1.0`), which also means it can only represent a rotation. See also [`orthonormalized`][`crate::builtin::Basis::orthonormalized`]."]
    pub fn orthonormalized(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(420usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "orthonormalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform rotated around the given `axis` by the given `angle` (in radians).\n\nThe `axis` must be a normalized vector (see [`normalized`][`crate::builtin::Vector3::normalized`]). If `angle` is positive, the basis is rotated counter-clockwise around the axis.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the left, i.e., `R * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(421usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform rotated around the given `axis` by the given `angle` (in radians).\n\nThe `axis` must be a normalized vector in the transform's local coordinate system. For example, to rotate around the local X-axis, use `Vector3.RIGHT`.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding rotation transform `R` from the right, i.e., `X * R`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn rotated_local(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(422usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform scaled by the given `scale` factor.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the left, i.e., `S * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn scaled(&self, scale: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(423usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform scaled by the given `scale` factor.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding scaling transform `S` from the right, i.e., `X * S`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn scaled_local(&self, scale: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(424usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform translated by the given `offset`.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the left, i.e., `T * X`.\n\nThis can be seen as transforming with respect to the global/parent frame."]
    pub fn translated(&self, offset: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(425usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this transform translated by the given `offset`.\n\nThis method is an optimized version of multiplying the given transform `X` with a corresponding translation transform `T` from the right, i.e., `X * T`.\n\nThis can be seen as transforming with respect to the local frame."]
    pub fn translated_local(&self, offset: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(426usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of the linear interpolation between this transform and `xform` by the given `weight`.\n\nThe `weight` should be between `0.0` and `1.0` (inclusive). Values outside this range are allowed and can be used to perform _extrapolation_ instead."]
    pub fn interpolate_with(&self, xform: Transform3D, weight: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Transform3D, f64,);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(428usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "interpolate_with", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this transform and `xform` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, xform: Transform3D,) -> bool {
        type CallRet = bool;
        type CallParams = (Transform3D,);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(429usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this transform is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(430usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_finite", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerTransform3D;
impl Transform3D {
    #[doc = "Returns a copy of this transform rotated so that the forward axis (-Z) points towards the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The resulting transform is orthonormalized. The existing rotation, scale, and skew information from the original transform is discarded. The `target` and `up` vectors cannot be zero, cannot be parallel to each other, and are defined in global/parent space.\n\nIf `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position. By default, the -Z axis (camera forward) is treated as forward (implies +X is right)."]
    pub(crate) fn looking_at_full(&self, target: Vector3, up: Vector3, use_model_front: bool,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, Vector3, bool,);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(427usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "looking_at", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    #[doc = "To set the default parameters, use [`looking_at_ex`][Self::looking_at_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Returns a copy of this transform rotated so that the forward axis (-Z) points towards the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The resulting transform is orthonormalized. The existing rotation, scale, and skew information from the original transform is discarded. The `target` and `up` vectors cannot be zero, cannot be parallel to each other, and are defined in global/parent space.\n\nIf `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position. By default, the -Z axis (camera forward) is treated as forward (implies +X is right)."]
    #[inline]
    pub fn looking_at(&self, target: Vector3,) -> Transform3D {
        self.looking_at_ex(target,) . done()
    }
    #[doc = "Returns a copy of this transform rotated so that the forward axis (-Z) points towards the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The resulting transform is orthonormalized. The existing rotation, scale, and skew information from the original transform is discarded. The `target` and `up` vectors cannot be zero, cannot be parallel to each other, and are defined in global/parent space.\n\nIf `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position. By default, the -Z axis (camera forward) is treated as forward (implies +X is right)."]
    #[inline]
    pub fn looking_at_ex < 'ex > (&'ex self, target: Vector3,) -> ExLookingAt < 'ex > {
        ExLookingAt::new(self, target,)
    }
}
#[doc = "Default-param extender for [`Transform3D::looking_at_ex`][crate::builtin::Transform3D::looking_at_ex]."]
#[must_use]
pub struct ExLookingAt < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex Transform3D, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLookingAt < 'ex > {
    fn new(surround_object: &'ex Transform3D, target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, target: target, up: up, use_model_front: use_model_front,
        }
    }
    #[inline]
    pub fn up(self, up: Vector3) -> Self {
        Self {
            up: up, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, use_model_front: bool) -> Self {
        Self {
            use_model_front: use_model_front, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Transform3D {
        let Self {
            _phantom, surround_object, target, up, use_model_front,
        }
        = self;
        Transform3D::looking_at_full(surround_object, target, up, use_model_front,)
    }
}