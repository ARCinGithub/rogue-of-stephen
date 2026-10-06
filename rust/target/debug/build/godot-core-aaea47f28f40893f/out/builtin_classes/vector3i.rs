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
    pub struct InnerVector3i < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerVector3i < 'inner > {
    pub fn from_outer(outer: &Vector3i) -> Self {
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
            let method_bind = sys::builtin_method_table() . fptr_by_index(258usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "min_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the axis of the vector's highest value. See `AXIS_*` constants. If all components are equal, this method returns `AXIS_X`."]
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(259usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "max_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the distance between this vector and `to`."]
    pub fn distance_to(&self, to: Vector3i,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(260usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "distance_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared distance between this vector and `to`.\n\nThis method runs faster than \\[method distance_to], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn distance_squared_to(&self, to: Vector3i,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector3i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(261usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "distance_squared_to", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length (magnitude) of this vector."]
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(262usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "length", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the squared length (squared magnitude) of this vector.\n\nThis method runs faster than \\[method length], so prefer it if you need to compare vectors or need the squared distance for some formula."]
    pub fn length_squared(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(263usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "length_squared", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component set to `1` if it's positive, `-1` if it's negative, and `0` if it's zero. The result is identical to calling [`sign`][`crate::global::sign`] on each component."]
    pub fn sign(&self,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(264usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "sign", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components in absolute values (i.e. positive)."]
    pub fn abs(&self,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(265usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "abs", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between the components of `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clamp(&self, min: Vector3i, max: Vector3i,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (Vector3i, Vector3i,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(266usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "clamp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with all components clamped between `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clampi(&self, min: i64, max: i64,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (i64, i64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(267usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "clampi", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the closest multiple of the corresponding component in `step`."]
    pub fn snapped(&self, step: Vector3i,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (Vector3i,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(268usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "snapped", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new vector with each component snapped to the closest multiple of `step`."]
    pub fn snappedi(&self, step: i64,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (i64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(269usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "snappedi", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector3i(mini(x, with.x), mini(y, with.y), mini(z, with.z))`."]
    pub fn min(&self, with: Vector3i,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (Vector3i,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(270usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "min", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise minimum of this and `with`, equivalent to `Vector3i(mini(x, with), mini(y, with), mini(z, with))`."]
    pub fn mini(&self, with: i64,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (i64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(271usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "mini", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector3i(maxi(x, with.x), maxi(y, with.y), maxi(z, with.z))`."]
    pub fn max(&self, with: Vector3i,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (Vector3i,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(272usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "max", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the component-wise maximum of this and `with`, equivalent to `Vector3i(maxi(x, with), maxi(y, with), maxi(z, with))`."]
    pub fn maxi(&self, with: i64,) -> Vector3i {
        type CallRet = Vector3i;
        type CallParams = (i64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(273usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector3i", "maxi", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerVector3i;
impl Vector3i {
    
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Vector3i.Axis")), is_bitfield: false,
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