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
    pub struct InnerVector4 < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerVector4 < 'inner > {
    pub fn from_outer(outer: &Vector4) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the axis of the vector's lowest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_W`."]
    pub fn min_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(295usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "min_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the axis of the vector's highest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_X`."]
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(296usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "max_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length (magnitude) of this vector."]
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(297usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared length (squared magnitude) of this vector.\n\nThis method runs faster than \\[method length], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(298usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "length_squared", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components in absolute values (i.e. positive)."]
    pub fn abs(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(299usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "abs", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component set to `1.0` if it's positive, `-1.0` if it's negative, and `0.0` if it's zero. The result is identical to calling [`sign`][`crate::global::sign`] on each component."]
    pub fn sign(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(300usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "sign", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded down (towards negative infinity)."]
    pub fn floor(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(301usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "floor", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded up (towards positive infinity)."]
    pub fn ceil(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(302usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "ceil", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero."]
    pub fn round(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(303usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "round", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of the linear interpolation between this vector and `to` by amount `weight`. `weight` is on the range of `0.0` to `1.0`, representing the amount of interpolation."]
    pub fn lerp(&self, to: Vector4, weight: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(304usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "lerp", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation."]
    pub fn cubic_interpolate(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4, Vector4, Vector4, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(305usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a cubic interpolation between this vector and `b` using `pre_a` and `post_b` as handles, and returns the result at position `weight`. `weight` is on the range of 0.0 to 1.0, representing the amount of interpolation.\n\nIt can perform smoother interpolation than \\[method cubic_interpolate] by the time values."]
    pub fn cubic_interpolate_in_time(&self, b: Vector4, pre_a: Vector4, post_b: Vector4, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4, Vector4, Vector4, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(306usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `mod`."]
    pub fn posmod(&self, mod_: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (f64,);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(307usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "posmod", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a vector composed of the [`fposmod`][`crate::global::fposmod`] of this vector's components and `modv`'s components."]
    pub fn posmodv(&self, modv: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4,);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(308usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "posmodv", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snapped(&self, step: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(309usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "snapped", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the nearest multiple of `step`. This can also be used to round the components to an arbitrary number of decimals."]
    pub fn snappedf(&self, step: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (f64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(310usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "snappedf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between the components of `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clamp(&self, min: Vector4, max: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4, Vector4,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(311usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "clamp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clampf(&self, min: f64, max: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (f64, f64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(312usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "clampf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the result of scaling the vector to unit length. Equivalent to `v / v.length()`. Returns `(0, 0, 0, 0)` if `v.length() == 0`. See also \\[method is_normalized].\n\n**Note:** This function may return incorrect values if the input vector length is near zero."]
    pub fn normalized(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(313usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the vector is normalized, i.e. its length is approximately equal to 1."]
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(314usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "is_normalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the normalized vector pointing from this vector to `to`. This is equivalent to using `(b - a).normalized()`."]
    pub fn direction_to(&self, to: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(315usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "direction_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the distance between this vector and `to`."]
    pub fn distance_to(&self, to: Vector4,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector4,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(316usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "distance_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared distance between this vector and `to`.\n\nThis method runs faster than \\[method distance_to], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn distance_squared_to(&self, to: Vector4,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector4,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(317usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "distance_squared_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dot product of this vector and `with`."]
    pub fn dot(&self, with: Vector4,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector4,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(318usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "dot", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the inverse of the vector. This is the same as `Vector4(1.0 / v.x, 1.0 / v.y, 1.0 / v.z, 1.0 / v.w)`."]
    pub fn inverse(&self,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(319usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector and `to` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, to: Vector4,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector4,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(320usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector's values are approximately zero, by running [`is_zero_approx`][`crate::global::is_zero_approx`] on each component.\n\nThis method is faster than using \\[method is_equal_approx] with one value as a zero vector."]
    pub fn is_zero_approx(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(321usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "is_zero_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this vector is finite, by calling [`is_finite`][`crate::global::is_finite`] on each component."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(322usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector4(minf(x, with.x), minf(y, with.y), minf(z, with.z), minf(w, with.w))`."]
    pub fn min(&self, with: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(323usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "min", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector4(minf(x, with), minf(y, with), minf(z, with), minf(w, with))`."]
    pub fn minf(&self, with: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(324usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "minf", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector4(maxf(x, with.x), maxf(y, with.y), maxf(z, with.z), maxf(w, with.w))`."]
    pub fn max(&self, with: Vector4,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (Vector4,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(325usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "max", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector4(maxf(x, with), maxf(y, with), maxf(z, with), maxf(w, with))`."]
    pub fn maxf(&self, with: f64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(326usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector4", "maxf", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerVector4;
impl Vector4 {
    
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
    #[doc(alias = "AXIS_W")]
    #[doc = "Godot enumerator name: `AXIS_W`"]
    pub const W: Axis = Axis {
        ord: 3i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::W => "W", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Axis::X, Axis::Y, Axis::Z, Axis::W]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Axis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_X", Axis::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_Y", Axis::Y), crate::meta::inspect::EnumConstant::new("Z", "AXIS_Z", Axis::Z), crate::meta::inspect::EnumConstant::new("W", "AXIS_W", Axis::W)]
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
            &[EnumeratorShape::new_int("Axis X", 0i64), EnumeratorShape::new_int("Axis Y", 1i64), EnumeratorShape::new_int("Axis Z", 2i64), EnumeratorShape::new_int("Axis W", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Vector4.Axis")), is_bitfield: false,
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