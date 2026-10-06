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
    pub struct InnerVector2 < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerVector2 < 'inner > {
    pub fn from_outer(outer: &Vector2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns this vector's angle with respect to the positive X axis, or `(1, 0)` vector, in radians.\n\nFor example, `Vector2.RIGHT.angle()` will return zero, `Vector2.DOWN.angle()` will return `PI / 2` (a quarter turn, or 90 degrees), and `Vector2(1, -1).angle()` will return `-PI / 4` (a negative eighth turn, or -45 degrees).\n\nThis is equivalent to calling [`atan2`][`crate::global::atan2`] with \\[member y] and \\[member x].\n\n[Illustration of the returned angle.](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle.png)"]
    pub fn angle(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(116usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the signed angle to the given vector, in radians. The result ranges from `-PI` to `PI` (inclusive).\n\n[Illustration of the returned angle.](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to.png)"]
    pub fn angle_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(117usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the signed angle between the X axis and the line from this vector to point `to`, in radians. The result ranges from `-PI` to `PI` (inclusive).\n\n`a.angle_to_point(b)` is equivalent to `(b - a).angle()`. See also \\[method angle].\n\n[Illustration of the returned angle.](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to_point.png)"]
    pub fn angle_to_point(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(118usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle_to_point", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the normalized vector pointing from this vector to `to`.\n\n`a.direction_to(b)` is equivalent to `(b - a).normalized()`. See also \\[method normalized]."]
    pub fn direction_to(&self, to: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(119usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "direction_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the distance between this vector and `to`."]
    pub fn distance_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(120usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "distance_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared distance between this vector and `to`.\n\nThis method runs faster than \\[method distance_to], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn distance_squared_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(121usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "distance_squared_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length (magnitude) of this vector."]
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(122usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared length (squared magnitude) of this vector.\n\nThis method runs faster than \\[method length], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(123usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "length_squared", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vector with a maximum length by limiting its length to `length`. If the vector is non-finite, the result is undefined."]
    pub fn limit_length(&self, length: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(124usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "limit_length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `(0, 0)` if `v.length() == 0`. See also \\[method is_normalized].\n\n**Note:** This function may return incorrect values if the input vector length is near zero."]
    pub fn normalized(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(125usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the vector is normalized, i.e. its length is approximately equal to 1."]
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(126usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector and `to` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, to: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(127usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector's values are approximately zero, by running [`is_zero_approx`][`crate::global::is_zero_approx`] on each component.\n\nThis method is faster than using \\[method is_equal_approx] with one value as a zero vector."]
    pub fn is_zero_approx(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(128usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_zero_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(129usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `mod`."]
    pub fn posmod(&self, mod_: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(130usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "posmod", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `modv`'s components."]
    pub fn posmodv(&self, modv: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(131usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "posmodv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector resulting from projecting this vector onto the given vector `b`. The resulting new vector is parallel to `b`. See also \\[method slide].\n\n**Note:** If the vector `b` is a zero vector, the components of the resulting new vector will be `@GDScript.NAN`."]
    pub fn project(&self, b: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(132usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "project", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation."]
    pub fn lerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(133usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "lerp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of spherical linear interpolation between this vector and `to`, by amount `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation.\n\nThis method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like \\[method lerp]."]
    pub fn slerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(134usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "slerp", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation."]
    pub fn cubic_interpolate(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(135usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation.\n\nIt can perform smoother interpolation than \\[method cubic_interpolate] by the time values."]
    pub fn cubic_interpolate_in_time(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(136usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the point at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points."]
    pub fn bezier_interpolate(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(137usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bezier_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the derivative at the given `t` on the [Bézier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) defined by this vector and the given `control_1`, `control_2`, and `end` points."]
    pub fn bezier_derivative(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(138usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bezier_derivative", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the axis of the vector's highest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_X`."]
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(139usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "max_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the axis of the vector's lowest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_Y`."]
    pub fn min_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(140usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "min_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector moved toward `to` by the fixed `delta` amount. Will not go past the final value."]
    pub fn move_toward(&self, to: Vector2, delta: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(141usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "move_toward", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of rotating this vector by `angle` (in radians). See also [`deg_to_rad`][`crate::global::deg_to_rad`]."]
    pub fn rotated(&self, angle: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(142usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "rotated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a perpendicular vector rotated 90 degrees counter-clockwise compared to the original, with the same length."]
    pub fn orthogonal(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(143usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "orthogonal", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded down (towards negative infinity)."]
    pub fn floor(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(144usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "floor", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded up (towards positive infinity)."]
    pub fn ceil(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(145usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "ceil", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero."]
    pub fn round(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(146usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "round", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this vector's aspect ratio, which is \\[member x] divided by \\[member y]."]
    pub fn aspect(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(147usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "aspect", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dot product of this vector and `with`. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.\n\nThe dot product will be `0` for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.\n\nWhen using unit (normalized) vectors, the result will always be between `-1.0` (180 degree angle) when the vectors are facing opposite directions, and `1.0` (0 degree angle) when the vectors are aligned.\n\n**Note:** `a.dot(b)` is equivalent to `b.dot(a)`."]
    pub fn dot(&self, with: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(148usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "dot", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector resulting from sliding this vector along a line with normal `n`. The resulting new vector is perpendicular to `n`, and is equivalent to this vector minus its projection on `n`. See also \\[method project].\n\n**Note:** The vector `n` must be normalized. See also \\[method normalized]."]
    pub fn slide(&self, n: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(149usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "slide", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vector \"bounced off\" from a line defined by the given normal `n` perpendicular to the line.\n\n**Note:** \\[method bounce] performs the operation that most engines and frameworks call `reflect()`."]
    pub fn bounce(&self, n: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(150usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bounce", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of reflecting the vector from a line defined by the given direction vector `line`.\n\n**Note:** \\[method reflect] differs from what other engines and frameworks call `reflect()`. In other engines, `reflect()` takes a normal direction which is a direction perpendicular to the line. In Godot, you specify the direction of the line directly. See also \\[method bounce] which does what most engines call `reflect()`."]
    pub fn reflect(&self, line: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (line,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(151usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "reflect", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the 2D analog of the cross product for this vector and `with`.\n\nThis is the signed area of the parallelogram formed by the two vectors. If the second vector is clockwise from the first vector, then the cross product is the positive area. If counter-clockwise, the cross product is the negative area. If the two vectors are parallel this returns zero, making it useful for testing if two vectors are parallel.\n\n**Note:** Cross product is not defined in 2D mathematically. This method embeds the 2D vectors in the XY plane of 3D space and uses their cross product's Z component as the analog."]
    pub fn cross(&self, with: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(152usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cross", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components in absolute values (i.e. positive)."]
    pub fn abs(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(153usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "abs", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling [`sign`][`crate::global::sign`] on each component."]
    pub fn sign(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(154usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "sign", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between the components of `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clamp(&self, min: Vector2, max: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(155usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "clamp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clampf(&self, min: f64, max: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64, f64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(156usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "clampf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snapped(&self, step: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(157usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "snapped", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snappedf(&self, step: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(158usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "snappedf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector2(minf(x, with.x), minf(y, with.y))`."]
    pub fn min(&self, with: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(159usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "min", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector2(minf(x, with), minf(y, with))`."]
    pub fn minf(&self, with: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(160usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "minf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector2(maxf(x, with.x), maxf(y, with.y))`."]
    pub fn max(&self, with: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(161usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "max", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector2(maxf(x, with), maxf(y, with))`."]
    pub fn maxf(&self, with: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(162usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "maxf", self.sys_ptr, args)
        }
    }
    #[doc = "Creates a [`Vector2`][crate::builtin::Vector2] rotated to the given `angle` in radians. This is equivalent to doing `Vector2(cos(angle), sin(angle))` or `Vector2.RIGHT.rotated(angle)`.\n\n```gdscript\nprint(Vector2.from_angle(0)) # Prints (1.0, 0.0)\nprint(Vector2(1, 0).angle()) # Prints 0.0, which is the angle used above.\nprint(Vector2.from_angle(PI / 2)) # Prints (0.0, 1.0)\n```\n\n**Note:** The length of the returned [`Vector2`][crate::builtin::Vector2] is _approximately_ `1.0`, but is is not guaranteed to be exactly `1.0` due to floating-point precision issues. Call \\[method normalized] on the returned [`Vector2`][crate::builtin::Vector2] if you require a unit vector."]
    pub fn from_angle(angle: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(163usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "from_angle", std::ptr::null_mut(), args)
        }
    }
}
pub use re_export::InnerVector2;
impl Vector2 {
    
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
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::X => "X", Self::Y => "Y", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Axis::X, Axis::Y]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Axis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_X", Axis::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_Y", Axis::Y)]
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
            &[EnumeratorShape::new_int("Axis X", 0i64), EnumeratorShape::new_int("Axis Y", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Vector2.Axis")), is_bitfield: false,
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