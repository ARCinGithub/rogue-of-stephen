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
    pub struct InnerAabb < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerAabb < 'inner > {
    pub fn from_outer(outer: &Aabb) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns an [`AABB`][crate::builtin::Aabb] equivalent to this bounding box, with its width, height, and depth modified to be non-negative values.\n\n\n```gdscript\nvar box = AABB(Vector3(5, 0, 5), Vector3(-20, -10, -5))\nvar absolute = box.abs()\nprint(absolute.position) # Prints (-15.0, -10.0, 0.0)\nprint(absolute.size)     # Prints (20.0, 10.0, 5.0)\n```\n\n\n**Note:** It's recommended to use this method when \\[member size] is negative, as most other methods in Godot assume that the \\[member size]'s components are greater than `0`."]
    pub fn abs(&self,) -> Aabb {
        type CallRet = Aabb;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(373usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "abs", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the center point of the bounding box. This is the same as `position + (size / 2.0)`."]
    pub fn get_center(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(374usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_center", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the bounding box's volume. This is equivalent to `size.x * size.y * size.z`. See also \\[method has_volume]."]
    pub fn get_volume(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(375usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_volume", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box's width, height, and depth are all positive. See also \\[method get_volume]."]
    pub fn has_volume(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(376usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "has_volume", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box has a surface or a length, that is, at least one component of \\[member size] is greater than `0`. Otherwise, returns `false`."]
    pub fn has_surface(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(377usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "has_surface", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if the bounding box contains the given `point`. By convention, points exactly on the right, top, and front sides are **not** included.\n\n**Note:** This method is not reliable for [`AABB`][crate::builtin::Aabb] with a _negative_ \\[member size]. Use \\[method abs] first to get a valid bounding box."]
    pub fn has_point(&self, point: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(378usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "has_point", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box and `aabb` are approximately equal, by calling [`is_equal_approx`][`crate::builtin::Vector3::is_equal_approx`] on the \\[member position] and the \\[member size]."]
    pub fn is_equal_approx(&self, aabb: Aabb,) -> bool {
        type CallRet = bool;
        type CallParams = (Aabb,);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(379usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box's values are finite, by calling [`is_finite`][`crate::builtin::Vector3::is_finite`] on the \\[member position] and the \\[member size]."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(380usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box overlaps with the box `with`. The edges of both boxes are _always_ excluded."]
    pub fn intersects(&self, with: Aabb,) -> bool {
        type CallRet = bool;
        type CallParams = (Aabb,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(381usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box _completely_ encloses the `with` box. The edges of both boxes are included.\n\n\n```gdscript\nvar a = AABB(Vector3(0, 0, 0), Vector3(4, 4, 4))\nvar b = AABB(Vector3(1, 1, 1), Vector3(3, 3, 3))\nvar c = AABB(Vector3(2, 2, 2), Vector3(8, 8, 8))\n\nprint(a.encloses(a)) # Prints true\nprint(a.encloses(b)) # Prints true\nprint(a.encloses(c)) # Prints false\n```\n"]
    pub fn encloses(&self, with: Aabb,) -> bool {
        type CallRet = bool;
        type CallParams = (Aabb,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(382usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "encloses", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this bounding box is on both sides of the given `plane`."]
    pub fn intersects_plane(&self, plane: Plane,) -> bool {
        type CallRet = bool;
        type CallParams = (Plane,);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(383usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_plane", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the intersection between this bounding box and `with`. If the boxes do not intersect, returns an empty [`AABB`][crate::builtin::Aabb]. If the boxes intersect at the edge, returns a flat [`AABB`][crate::builtin::Aabb] with no volume (see \\[method has_surface] and \\[method has_volume]).\n\n\n```gdscript\nvar box1 = AABB(Vector3(0, 0, 0), Vector3(5, 2, 8))\nvar box2 = AABB(Vector3(2, 0, 2), Vector3(8, 4, 4))\n\nvar intersection = box1.intersection(box2)\nprint(intersection.position) # Prints (2.0, 0.0, 2.0)\nprint(intersection.size)     # Prints (3.0, 2.0, 4.0)\n```\n\n\n**Note:** If you only need to know whether two bounding boxes are intersecting, use \\[method intersects], instead."]
    pub fn intersection(&self, with: Aabb,) -> Aabb {
        type CallRet = Aabb;
        type CallParams = (Aabb,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(384usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "intersection", self.sys_ptr, args)
        }
    }
    #[doc = "Returns an [`AABB`][crate::builtin::Aabb] that encloses both this bounding box and `with` around the edges. See also \\[method encloses]."]
    pub fn merge(&self, with: Aabb,) -> Aabb {
        type CallRet = Aabb;
        type CallParams = (Aabb,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(385usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "merge", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this bounding box expanded to align the edges with the given `to_point`, if necessary.\n\n\n```gdscript\nvar box = AABB(Vector3(0, 0, 0), Vector3(5, 2, 5))\n\nbox = box.expand(Vector3(10, 0, 0))\nprint(box.position) # Prints (0.0, 0.0, 0.0)\nprint(box.size)     # Prints (10.0, 2.0, 5.0)\n\nbox = box.expand(Vector3(-5, 0, 5))\nprint(box.position) # Prints (-5.0, 0.0, 0.0)\nprint(box.size)     # Prints (15.0, 2.0, 5.0)\n```\n"]
    pub fn expand(&self, to_point: Vector3,) -> Aabb {
        type CallRet = Aabb;
        type CallParams = (Vector3,);
        let args = (to_point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(386usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "expand", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this bounding box extended on all sides by the given amount `by`. A negative amount shrinks the box instead.\n\n\n```gdscript\nvar a = AABB(Vector3(4, 4, 4), Vector3(8, 8, 8)).grow(4)\nprint(a.position) # Prints (0.0, 0.0, 0.0)\nprint(a.size)     # Prints (16.0, 16.0, 16.0)\n\nvar b = AABB(Vector3(0, 0, 0), Vector3(8, 4, 2)).grow(2)\nprint(b.position) # Prints (-2.0, -2.0, -2.0)\nprint(b.size)     # Prints (12.0, 8.0, 6.0)\n```\n"]
    pub fn grow(&self, by: f64,) -> Aabb {
        type CallRet = Aabb;
        type CallParams = (f64,);
        let args = (by,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(387usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "grow", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vertex's position of this bounding box that's the farthest in the given direction. This point is commonly known as the support point in collision detection algorithms."]
    pub fn get_support(&self, direction: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (direction,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(388usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_support", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the longest normalized axis of this bounding box's \\[member size], as a [`Vector3`][crate::builtin::Vector3] (`Vector3.RIGHT`, `Vector3.UP`, or `Vector3.BACK`).\n\n\n```gdscript\nvar box = AABB(Vector3(0, 0, 0), Vector3(2, 4, 8))\n\nprint(box.get_longest_axis())       # Prints (0.0, 0.0, 1.0)\nprint(box.get_longest_axis_index()) # Prints 2\nprint(box.get_longest_axis_size())  # Prints 8.0\n```\n\n\nSee also \\[method get_longest_axis_index] and \\[method get_longest_axis_size]."]
    pub fn get_longest_axis(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(389usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index to the longest axis of this bounding box's \\[member size] (see `Vector3.AXIS_X`, `Vector3.AXIS_Y`, and `Vector3.AXIS_Z`).\n\nFor an example, see \\[method get_longest_axis]."]
    pub fn get_longest_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(390usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the longest dimension of this bounding box's \\[member size].\n\nFor an example, see \\[method get_longest_axis]."]
    pub fn get_longest_axis_size(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(391usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_longest_axis_size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the shortest normalized axis of this bounding box's \\[member size], as a [`Vector3`][crate::builtin::Vector3] (`Vector3.RIGHT`, `Vector3.UP`, or `Vector3.BACK`).\n\n\n```gdscript\nvar box = AABB(Vector3(0, 0, 0), Vector3(2, 4, 8))\n\nprint(box.get_shortest_axis())       # Prints (1.0, 0.0, 0.0)\nprint(box.get_shortest_axis_index()) # Prints 0\nprint(box.get_shortest_axis_size())  # Prints 2.0\n```\n\n\nSee also \\[method get_shortest_axis_index] and \\[method get_shortest_axis_size]."]
    pub fn get_shortest_axis(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(392usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the index to the shortest axis of this bounding box's \\[member size] (see `Vector3.AXIS_X`, `Vector3.AXIS_Y`, and `Vector3.AXIS_Z`).\n\nFor an example, see \\[method get_shortest_axis]."]
    pub fn get_shortest_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(393usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis_index", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the shortest dimension of this bounding box's \\[member size].\n\nFor an example, see \\[method get_shortest_axis]."]
    pub fn get_shortest_axis_size(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(394usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_shortest_axis_size", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the position of one of the 8 vertices that compose this bounding box. With an `idx` of `0` this is the same as \\[member position], and an `idx` of `7` is the same as \\[member end]."]
    pub fn get_endpoint(&self, idx: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(395usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "get_endpoint", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the first point where this bounding box and the given segment intersect, as a [`Vector3`][crate::builtin::Vector3]. If no intersection occurs, returns `null`.\n\nThe segment begins at `from` and ends at `to`."]
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type CallRet = Variant;
        type CallParams = (Vector3, Vector3,);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(396usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_segment", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the first point where this bounding box and the given ray intersect, as a [`Vector3`][crate::builtin::Vector3]. If no intersection occurs, returns `null`.\n\nThe ray begin at `from`, faces `dir` and extends towards infinity."]
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type CallRet = Variant;
        type CallParams = (Vector3, Vector3,);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(397usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Aabb", "intersects_ray", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerAabb;
impl Aabb {
    
}