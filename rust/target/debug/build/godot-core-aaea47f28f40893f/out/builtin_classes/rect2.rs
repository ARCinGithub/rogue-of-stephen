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
    pub struct InnerRect2 < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerRect2 < 'inner > {
    pub fn from_outer(outer: &Rect2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the center point of the rectangle. This is the same as `position + (size / 2.0)`."]
    pub fn get_center(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(181usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_center", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the rectangle's area. This is equivalent to `size.x * size.y`. See also \\[method has_area]."]
    pub fn get_area(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(182usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_area", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle has positive width and height. See also \\[method get_area]."]
    pub fn has_area(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(183usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "has_area", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the rectangle contains the given `point`. By convention, points on the right and bottom edges are **not** included.\n\n**Note:** This method is not reliable for [`Rect2`][crate::builtin::Rect2] with a _negative_ \\[member size]. Use \\[method abs] first to get a valid rectangle."]
    pub fn has_point(&self, point: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(184usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "has_point", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle and `rect` are approximately equal, by calling [`is_equal_approx`][`crate::builtin::Vector2::is_equal_approx`] on the \\[member position] and the \\[member size]."]
    pub fn is_equal_approx(&self, rect: Rect2,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2,);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(185usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle's values are finite, by calling [`is_finite`][`crate::builtin::Vector2::is_finite`] on the \\[member position] and the \\[member size]."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(186usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle overlaps with the `b` rectangle. The edges of both rectangles are excluded, unless `include_borders` is `true`."]
    pub fn intersects(&self, b: Rect2, include_borders: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2, bool,);
        let args = (b, include_borders,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(187usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "intersects", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle _completely_ encloses the `b` rectangle."]
    pub fn encloses(&self, b: Rect2,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(188usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "encloses", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the intersection between this rectangle and `b`. If the rectangles do not intersect, returns an empty [`Rect2`][crate::builtin::Rect2].\n\n\n```gdscript\nvar rect1 = Rect2(0, 0, 5, 10)\nvar rect2 = Rect2(2, 0, 8, 4)\n\nvar a = rect1.intersection(rect2) # a is Rect2(2, 0, 3, 4)\n```\n\n\n**Note:** If you only need to know whether two rectangles are overlapping, use \\[method intersects], instead."]
    pub fn intersection(&self, b: Rect2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(189usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "intersection", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Rect2`][crate::builtin::Rect2] that encloses both this rectangle and `b` around the edges. See also \\[method encloses]."]
    pub fn merge(&self, b: Rect2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(190usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "merge", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle expanded to align the edges with the given `to` point, if necessary.\n\n\n```gdscript\nvar rect = Rect2(0, 0, 5, 2)\n\nrect = rect.expand(Vector2(10, 0)) # rect is Rect2(0, 0, 10, 2)\nrect = rect.expand(Vector2(-5, 5)) # rect is Rect2(-5, 0, 15, 5)\n```\n"]
    pub fn expand(&self, to: Vector2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(191usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "expand", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vertex's position of this rect that's the farthest in the given direction. This point is commonly known as the support point in collision detection algorithms."]
    pub fn get_support(&self, direction: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (direction,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(192usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_support", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle extended on all sides by the given `amount`. A negative `amount` shrinks the rectangle instead. See also \\[method grow_individual] and \\[method grow_side].\n\n\n```gdscript\nvar a = Rect2(4, 4, 8, 8).grow(4) # a is Rect2(0, 0, 16, 16)\nvar b = Rect2(0, 0, 8, 4).grow(2) # b is Rect2(-2, -2, 12, 8)\n```\n"]
    pub fn grow(&self, amount: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(193usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle with its `side` extended by the given `amount` (see \\[enum Side] constants). A negative `amount` shrinks the rectangle, instead. See also \\[method grow] and \\[method grow_individual]."]
    pub fn grow_side(&self, side: i64, amount: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (i64, f64,);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(194usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_side", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle with its `left`, `top`, `right`, and `bottom` sides extended by the given amounts. Negative values shrink the sides, instead. See also \\[method grow] and \\[method grow_side]."]
    pub fn grow_individual(&self, left: f64, top: f64, right: f64, bottom: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (f64, f64, f64, f64,);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(195usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_individual", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Rect2`][crate::builtin::Rect2] equivalent to this rectangle, with its width and height modified to be non-negative values, and with its \\[member position] being the top-left corner of the rectangle.\n\n\n```gdscript\nvar rect = Rect2(25, 25, -100, -50)\nvar absolute = rect.abs() # absolute is Rect2(-75, -25, 100, 50)\n```\n\n\n**Note:** It's recommended to use this method when \\[member size] is negative, as most other methods in Godot assume that the \\[member position] is the top-left corner, and the \\[member end] is the bottom-right corner."]
    pub fn abs(&self,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(196usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "abs", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerRect2;
impl Rect2 {
    
}