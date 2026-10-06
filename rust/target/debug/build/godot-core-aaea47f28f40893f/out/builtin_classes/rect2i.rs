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
    pub struct InnerRect2i < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerRect2i < 'inner > {
    pub fn from_outer(outer: &Rect2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the center point of the rectangle. This is the same as `position + (size / 2)`.\n\n**Note:** If the \\[member size] is odd, the result will be rounded towards \\[member position]."]
    pub fn get_center(&self,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(197usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_center", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the rectangle's area. This is equivalent to `size.x * size.y`. See also \\[method has_area]."]
    pub fn get_area(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(198usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_area", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle has positive width and height. See also \\[method get_area]."]
    pub fn has_area(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(199usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_area", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the rectangle contains the given `point`. By convention, points on the right and bottom edges are **not** included.\n\n**Note:** This method is not reliable for [`Rect2i`][crate::builtin::Rect2i] with a _negative_ \\[member size]. Use \\[method abs] first to get a valid rectangle."]
    pub fn has_point(&self, point: Vector2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2i,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(200usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_point", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this rectangle overlaps with the `b` rectangle. The edges of both rectangles are excluded."]
    pub fn intersects(&self, b: Rect2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(201usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersects", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this [`Rect2i`][crate::builtin::Rect2i] completely encloses another one."]
    pub fn encloses(&self, b: Rect2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(202usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "encloses", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the intersection between this rectangle and `b`. If the rectangles do not intersect, returns an empty [`Rect2i`][crate::builtin::Rect2i].\n\n\n```gdscript\nvar a = Rect2i(0, 0, 5, 10)\nvar b = Rect2i(2, 0, 8, 4)\n\nvar c = a.intersection(b) # c is Rect2i(2, 0, 3, 4)\n```\n\n\n**Note:** If you only need to know whether two rectangles are overlapping, use \\[method intersects], instead."]
    pub fn intersection(&self, b: Rect2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(203usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersection", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Rect2i`][crate::builtin::Rect2i] that encloses both this rectangle and `b` around the edges. See also \\[method encloses]."]
    pub fn merge(&self, b: Rect2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(204usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "merge", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle expanded to align the edges with the given `to` point, if necessary.\n\n\n```gdscript\nvar rect = Rect2i(0, 0, 5, 2)\n\nrect = rect.expand(Vector2i(10, 0)) # rect is Rect2i(0, 0, 10, 2)\nrect = rect.expand(Vector2i(-5, 5)) # rect is Rect2i(-5, 0, 15, 5)\n```\n"]
    pub fn expand(&self, to: Vector2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Vector2i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(205usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "expand", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle extended on all sides by the given `amount`. A negative `amount` shrinks the rectangle instead. See also \\[method grow_individual] and \\[method grow_side].\n\n\n```gdscript\nvar a = Rect2i(4, 4, 8, 8).grow(4) # a is Rect2i(0, 0, 16, 16)\nvar b = Rect2i(0, 0, 8, 4).grow(2) # b is Rect2i(-2, -2, 12, 8)\n```\n"]
    pub fn grow(&self, amount: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(206usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle with its `side` extended by the given `amount` (see \\[enum Side] constants). A negative `amount` shrinks the rectangle, instead. See also \\[method grow] and \\[method grow_individual]."]
    pub fn grow_side(&self, side: i64, amount: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64, i64,);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(207usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_side", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this rectangle with its `left`, `top`, `right`, and `bottom` sides extended by the given amounts. Negative values shrink the sides, instead. See also \\[method grow] and \\[method grow_side]."]
    pub fn grow_individual(&self, left: i64, top: i64, right: i64, bottom: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64, i64, i64, i64,);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(208usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_individual", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Rect2i`][crate::builtin::Rect2i] equivalent to this rectangle, with its width and height modified to be non-negative values, and with its \\[member position] being the top-left corner of the rectangle.\n\n\n```gdscript\nvar rect = Rect2i(25, 25, -100, -50)\nvar absolute = rect.abs() # absolute is Rect2i(-75, -25, 100, 50)\n```\n\n\n**Note:** It's recommended to use this method when \\[member size] is negative, as most other methods in Godot assume that the \\[member position] is the top-left corner, and the \\[member end] is the bottom-right corner."]
    pub fn abs(&self,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(209usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "abs", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerRect2i;
impl Rect2i {
    
}