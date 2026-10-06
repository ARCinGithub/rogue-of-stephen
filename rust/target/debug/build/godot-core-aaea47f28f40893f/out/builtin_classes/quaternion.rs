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
    pub struct InnerQuaternion < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerQuaternion < 'inner > {
    pub fn from_outer(outer: &Quaternion) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns this quaternion's length, also called magnitude."]
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(354usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this quaternion's length, squared.\n\n**Note:** This method is faster than \\[method length], so prefer it if you only need to compare quaternion lengths."]
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(355usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "length_squared", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this quaternion, normalized so that its length is `1.0`. See also \\[method is_normalized]."]
    pub fn normalized(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(356usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this quaternion is normalized. See also \\[method normalized]."]
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(357usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this quaternion and `to` are approximately equal, by calling [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, to: Quaternion,) -> bool {
        type CallRet = bool;
        type CallParams = (Quaternion,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(358usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this quaternion is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(359usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the inverse version of this quaternion, inverting the sign of every component except \\[member w]."]
    pub fn inverse(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(360usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the logarithm of this quaternion. Multiplies this quaternion's rotation axis by its rotation angle, and stores the result in the returned quaternion's vector part (\\[member x], \\[member y], and \\[member z]). The returned quaternion's real part (\\[member w]) is always `0.0`."]
    pub fn log(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(361usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "log", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the exponential of this quaternion. The rotation axis of the result is the normalized rotation axis of this quaternion, the angle of the result is the length of the vector part of this quaternion."]
    pub fn exp(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(362usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "exp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the angle between this quaternion and `to`. This is the magnitude of the angle you would need to rotate by to get from one to the other.\n\n**Note:** The magnitude of the floating-point error for this method is abnormally high, so methods such as `is_zero_approx` will not work reliably."]
    pub fn angle_to(&self, to: Quaternion,) -> f64 {
        type CallRet = f64;
        type CallParams = (Quaternion,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(363usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "angle_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dot product between this quaternion and `with`.\n\nThis is equivalent to `(quat.x * with.x) + (quat.y * with.y) + (quat.z * with.z) + (quat.w * with.w)`."]
    pub fn dot(&self, with: Quaternion,) -> f64 {
        type CallRet = f64;
        type CallParams = (Quaternion,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(364usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "dot", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a spherical-linear interpolation with the `to` quaternion, given a `weight` and returns the result. Both this quaternion and `to` must be normalized."]
    pub fn slerp(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(365usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerp", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a spherical-linear interpolation with the `to` quaternion, given a `weight` and returns the result. Unlike \\[method slerp], this method does not check if the rotation path is smaller than 90 degrees. Both this quaternion and `to` must be normalized."]
    pub fn slerpni(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(366usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerpni", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a spherical cubic interpolation between quaternions `pre_a`, this vector, `b`, and `post_b`, by the given amount `weight`."]
    pub fn spherical_cubic_interpolate(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, Quaternion, Quaternion, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(367usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a spherical cubic interpolation between quaternions `pre_a`, this vector, `b`, and `post_b`, by the given amount `weight`.\n\nIt can perform smoother interpolation than \\[method spherical_cubic_interpolate] by the time values."]
    pub fn spherical_cubic_interpolate_in_time(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, Quaternion, Quaternion, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(368usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this quaternion's rotation as a [`Vector3`][crate::builtin::Vector3] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians.\n\nThe order of each consecutive rotation can be changed with `order` (see \\[enum EulerOrder] constants). By default, the YXZ convention is used ([`EulerOrder::YXZ`][`crate::builtin::EulerOrder::YXZ`]): Z (roll) is calculated first, then X (pitch), and lastly Y (yaw). When using the opposite method \\[method from_euler], this order is reversed."]
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(369usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_euler", self.sys_ptr, args)
        }
    }
    #[doc = "Constructs a new [`Quaternion`][crate::builtin::Quaternion] from the given [`Vector3`][crate::builtin::Vector3] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians. This method always uses the YXZ convention ([`EulerOrder::YXZ`][`crate::builtin::EulerOrder::YXZ`])."]
    pub fn from_euler(euler: Vector3,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Vector3,);
        let args = (euler,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(370usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "from_euler", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns the rotation axis of the rotation represented by this quaternion."]
    pub fn get_axis(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(371usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_axis", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the angle of the rotation represented by this quaternion.\n\n**Note:** The quaternion must be normalized."]
    pub fn get_angle(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(372usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_angle", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerQuaternion;
impl Quaternion {
    
}