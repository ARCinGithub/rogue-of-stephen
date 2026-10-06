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
    pub struct InnerVector3 < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerVector3 < 'inner > {
    pub fn from_outer(outer: &Vector3) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the axis of the vector's lowest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_Z`."]
    pub fn min_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(210usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "min_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the axis of the vector's highest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_X`."]
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(211usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "max_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the unsigned minimum angle to the given vector, in radians."]
    pub fn angle_to(&self, to: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(212usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "angle_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the signed angle to the given vector, in radians. The sign of the angle is positive in a counter-clockwise direction and negative in a clockwise direction when viewed from the side specified by the `axis`."]
    pub fn signed_angle_to(&self, to: Vector3, axis: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3, Vector3,);
        let args = (to, axis,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(213usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "signed_angle_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the normalized vector pointing from this vector to `to`. This is equivalent to using `(b - a).normalized()`."]
    pub fn direction_to(&self, to: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(214usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "direction_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the distance between this vector and `to`."]
    pub fn distance_to(&self, to: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(215usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "distance_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared distance between this vector and `to`.\n\nThis method runs faster than \\[method distance_to], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn distance_squared_to(&self, to: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(216usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "distance_squared_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length (magnitude) of this vector."]
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(217usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared length (squared magnitude) of this vector.\n\nThis method runs faster than \\[method length], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(218usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "length_squared", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vector with a maximum length by limiting its length to `length`. If the vector is non-finite, the result is undefined."]
    pub fn limit_length(&self, length: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(219usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "limit_length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `(0, 0, 0)` if `v.length() == 0`. See also \\[method is_normalized].\n\n**Note:** This function may return incorrect values if the input vector length is near zero."]
    pub fn normalized(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(220usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the vector is normalized, i.e. its length is approximately equal to 1."]
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(221usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "is_normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector and `to` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, to: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(222usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector's values are approximately zero, by running [`is_zero_approx`][`crate::global::is_zero_approx`] on each component.\n\nThis method is faster than using \\[method is_equal_approx] with one value as a zero vector."]
    pub fn is_zero_approx(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(223usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "is_zero_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(224usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the inverse of the vector. This is the same as `Vector3(1.0 / v.x, 1.0 / v.y, 1.0 / v.z)`."]
    pub fn inverse(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(225usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between the components of `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clamp(&self, min: Vector3, max: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, Vector3,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(226usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "clamp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clampf(&self, min: f64, max: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64, f64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(227usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "clampf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snapped(&self, step: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(228usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "snapped", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snappedf(&self, step: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(229usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "snappedf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of rotating this vector around a given axis by `angle` (in radians). The axis must be a normalized vector. See also [`deg_to_rad`][`crate::global::deg_to_rad`]."]
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(230usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "rotated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation."]
    pub fn lerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(231usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "lerp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of spherical linear interpolation between this vector and `to`, by amount `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation.\n\nThis method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like \\[method lerp]."]
    pub fn slerp(&self, to: Vector3, weight: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(232usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "slerp", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation."]
    pub fn cubic_interpolate(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, Vector3, Vector3, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(233usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation.\n\nIt can perform smoother interpolation than \\[method cubic_interpolate] by the time values."]
    pub fn cubic_interpolate_in_time(&self, b: Vector3, pre_a: Vector3, post_b: Vector3, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, Vector3, Vector3, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(234usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the point at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points."]
    pub fn bezier_interpolate(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, Vector3, Vector3, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(235usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "bezier_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the derivative at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points."]
    pub fn bezier_derivative(&self, control_1: Vector3, control_2: Vector3, end: Vector3, t: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, Vector3, Vector3, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(236usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "bezier_derivative", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector moved toward `to` by the fixed `delta` amount. Will not go past the final value."]
    pub fn move_toward(&self, to: Vector3, delta: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3, f64,);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(237usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "move_toward", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dot product of this vector and `with`. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.\n\nThe dot product will be `0` for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.\n\nWhen using unit (normalized) vectors, the result will always be between `-1.0` (180 degree angle) when the vectors are facing opposite directions, and `1.0` (0 degree angle) when the vectors are aligned.\n\n**Note:** `a.dot(b)` is equivalent to `b.dot(a)`."]
    pub fn dot(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(238usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "dot", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the cross product of this vector and `with`.\n\nThis returns a vector perpendicular to both this and `with`, which would be the normal vector of the plane defined by the two vectors. As there are two such vectors, in opposite directions, this method returns the vector defined by a right-handed coordinate system. If the two vectors are parallel this returns an empty vector, making it useful for testing if two vectors are parallel."]
    pub fn cross(&self, with: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(239usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "cross", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the outer product with `with`."]
    pub fn outer(&self, with: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(240usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "outer", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components in absolute values (i.e. positive)."]
    pub fn abs(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(241usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "abs", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded down (towards negative infinity)."]
    pub fn floor(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(242usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "floor", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded up (towards positive infinity)."]
    pub fn ceil(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(243usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "ceil", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero."]
    pub fn round(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(244usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "round", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `mod`."]
    pub fn posmod(&self, mod_: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64,);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(245usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "posmod", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `modv`'s components."]
    pub fn posmodv(&self, modv: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(246usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "posmodv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector resulting from projecting this vector onto the given vector `b`. The resulting new vector is parallel to `b`. See also \\[method slide].\n\n**Note:** If the vector `b` is a zero vector, the components of the resulting new vector will be `@GDScript.NAN`."]
    pub fn project(&self, b: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(247usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "project", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector resulting from sliding this vector along a plane with normal `n`. The resulting new vector is perpendicular to `n`, and is equivalent to this vector minus its projection on `n`. See also \\[method project].\n\n**Note:** The vector `n` must be normalized. See also \\[method normalized]."]
    pub fn slide(&self, n: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(248usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "slide", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vector \"bounced off\" from a plane defined by the given normal `n`.\n\n**Note:** \\[method bounce] performs the operation that most engines and frameworks call `reflect()`."]
    pub fn bounce(&self, n: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(249usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "bounce", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of reflecting the vector through a plane defined by the given normal vector `n`.\n\n**Note:** \\[method reflect] differs from what other engines and frameworks call `reflect()`. In other engines, `reflect()` returns the result of the vector reflected by the given plane. The reflection thus passes through the given normal. While in Godot the reflection passes through the plane and can be thought of as bouncing off the normal. See also \\[method bounce] which does what most engines call `reflect()`."]
    pub fn reflect(&self, n: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(250usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "reflect", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling [`sign`][`crate::global::sign`] on each component."]
    pub fn sign(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(251usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "sign", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the octahedral-encoded (oct32) form of this [`Vector3`][crate::builtin::Vector3] as a [`Vector2`][crate::builtin::Vector2]. Since a [`Vector2`][crate::builtin::Vector2] occupies 1/3 less memory compared to [`Vector3`][crate::builtin::Vector3], this form of compression can be used to pass greater amounts of \\[method normalized] [`Vector3`][crate::builtin::Vector3]s without increasing storage or memory requirements. See also \\[method octahedron_decode].\n\n**Note:** \\[method octahedron_encode] can only be used for \\[method normalized] vectors. \\[method octahedron_encode] does _not_ check whether this [`Vector3`][crate::builtin::Vector3] is normalized, and will return a value that does not decompress to the original value if the [`Vector3`][crate::builtin::Vector3] is not normalized.\n\n**Note:** Octahedral compression is _lossy_, although visual differences are rarely perceptible in real world scenarios."]
    pub fn octahedron_encode(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(252usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "octahedron_encode", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector3(minf(x, with.x), minf(y, with.y), minf(z, with.z))`."]
    pub fn min(&self, with: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(253usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "min", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector3(minf(x, with), minf(y, with), minf(z, with))`."]
    pub fn minf(&self, with: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(254usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "minf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector3(maxf(x, with.x), maxf(y, with.y), maxf(z, with.z))`."]
    pub fn max(&self, with: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(255usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "max", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector3(maxf(x, with), maxf(y, with), maxf(z, with))`."]
    pub fn maxf(&self, with: f64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(256usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "maxf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [`Vector3`][crate::builtin::Vector3] from an octahedral-compressed form created using \\[method octahedron_encode] (stored as a [`Vector2`][crate::builtin::Vector2])."]
    pub fn octahedron_decode(uv: Vector2,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector2,);
        let args = (uv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(257usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3", "octahedron_decode", std::ptr::null_mut(), args)
        }
    }
}
pub use re_export::InnerVector3;
impl Vector3 {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Axis {
    ord: i32
}
impl Axis {
    #[doc(alias = "AXIS_X")]
    #[doc = "Godot enumerator name: `AXIS_X`"]
    pub const X: Axis = Axis {
        ord: 0i32
    };
    #[doc(alias = "AXIS_Y")]
    #[doc = "Godot enumerator name: `AXIS_Y`"]
    pub const Y: Axis = Axis {
        ord: 1i32
    };
    #[doc(alias = "AXIS_Z")]
    #[doc = "Godot enumerator name: `AXIS_Z`"]
    pub const Z: Axis = Axis {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Axis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Axis {
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
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Axis::X, Axis::Y, Axis::Z]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Axis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_X", Axis::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_Y", Axis::Y), crate::meta::inspect::EnumConstant::new("Z", "AXIS_Z", Axis::Z)]
        }
    }
}
impl crate::meta::GodotConvert for Axis {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Axis X", 0i64), EnumeratorShape::new_int("Axis Y", 1i64), EnumeratorShape::new_int("Axis Z", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Vector3.Axis")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Axis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Axis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Axis {
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
impl crate::registry::property::Export for Axis {
    
}
impl crate::meta::Element for Axis {
    
}