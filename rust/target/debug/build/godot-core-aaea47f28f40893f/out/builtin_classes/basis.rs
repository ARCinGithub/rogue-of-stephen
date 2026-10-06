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
    pub struct InnerBasis < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerBasis < 'inner > {
    pub fn from_outer(outer: &Basis) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the [inverse of this basis's matrix](https://en.wikipedia.org/wiki/Invertible_matrix)."]
    pub fn inverse(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(398usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the transposed version of this basis. This turns the basis matrix's columns into rows, and its rows into columns.\n\n\n```gdscript\nvar my_basis = Basis(\n\tVector3(1, 2, 3),\n\tVector3(4, 5, 6),\n\tVector3(7, 8, 9)\n)\nmy_basis = my_basis.transposed()\n\nprint(my_basis.x) # Prints (1.0, 4.0, 7.0)\nprint(my_basis.y) # Prints (2.0, 5.0, 8.0)\nprint(my_basis.z) # Prints (3.0, 6.0, 9.0)\n```\n"]
    pub fn transposed(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(399usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "transposed", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the orthonormalized version of this basis. An orthonormal basis is both _orthogonal_ (the axes are perpendicular to each other) and _normalized_ (the axes have a length of `1.0`), which also means it can only represent a rotation.\n\nIt is often useful to call this method to avoid rounding errors on a rotating basis:\n\n\n```gdscript\n# Rotate this Node3D every frame.\nfunc _process(delta):\n\tbasis = basis.rotated(Vector3.UP, TAU * delta)\n\tbasis = basis.rotated(Vector3.RIGHT, TAU * delta)\n\tbasis = basis.orthonormalized()\n```\n"]
    pub fn orthonormalized(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(400usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "orthonormalized", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [determinant](https://en.wikipedia.org/wiki/Determinant) of this basis's matrix. For advanced math, this number can be used to determine a few attributes:\n\n- If the determinant is exactly `0.0`, the basis is not invertible (see \\[method inverse]).\n\n- If the determinant is a negative number, the basis represents a negative scale.\n\n**Note:** If the basis's scale is the same for every axis, its determinant is always that scale by the power of 3."]
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(401usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "determinant", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this basis rotated around the given `axis` by the given `angle` (in radians).\n\nThe `axis` must be a normalized vector (see [`normalized`][`crate::builtin::Vector3::normalized`]). If `angle` is positive, the basis is rotated counter-clockwise around the axis.\n\n\n```gdscript\nvar my_basis = Basis.IDENTITY\nvar angle = TAU / 2\n\nmy_basis = my_basis.rotated(Vector3.UP, angle)    # Rotate around the up axis (yaw).\nmy_basis = my_basis.rotated(Vector3.RIGHT, angle) # Rotate around the right axis (pitch).\nmy_basis = my_basis.rotated(Vector3.BACK, angle)  # Rotate around the back axis (roll).\n```\n"]
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(402usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "rotated", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this basis with each axis's components scaled by the given `scale`'s components.\n\nThe basis matrix's rows are multiplied by `scale`'s components. This operation is a global scale (relative to the parent).\n\n\n```gdscript\nvar my_basis = Basis(\n\tVector3(1, 1, 1),\n\tVector3(2, 2, 2),\n\tVector3(3, 3, 3)\n)\nmy_basis = my_basis.scaled(Vector3(0, 2, -2))\n\nprint(my_basis.x) # Prints (0.0, 2.0, -2.0)\nprint(my_basis.y) # Prints (0.0, 4.0, -4.0)\nprint(my_basis.z) # Prints (0.0, 6.0, -6.0)\n```\n"]
    pub fn scaled(&self, scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(403usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "scaled", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this basis with each axis scaled by the corresponding component in the given `scale`.\n\nThe basis matrix's columns are multiplied by `scale`'s components. This operation is a local scale (relative to self).\n\n\n```gdscript\nvar my_basis = Basis(\n    Vector3(1, 1, 1),\n    Vector3(2, 2, 2),\n    Vector3(3, 3, 3)\n)\nmy_basis = my_basis.scaled_local(Vector3(0, 2, -2))\n\nprint(my_basis.x) # Prints (0.0, 0.0, 0.0)\nprint(my_basis.y) # Prints (4.0, 4.0, 4.0)\nprint(my_basis.z) # Prints (-6.0, -6.0, -6.0)\n```\n"]
    pub fn scaled_local(&self, scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(404usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "scaled_local", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the length of each axis of this basis, as a [`Vector3`][crate::builtin::Vector3]. If the basis is not sheared, this value is the scaling factor. It is not affected by rotation.\n\n\n```gdscript\nvar my_basis = Basis(\n\tVector3(2, 0, 0),\n\tVector3(0, 4, 0),\n\tVector3(0, 0, 8)\n)\n# Rotating the Basis in any way preserves its scale.\nmy_basis = my_basis.rotated(Vector3.UP, TAU / 2)\nmy_basis = my_basis.rotated(Vector3.RIGHT, TAU / 4)\n\nprint(my_basis.get_scale()) # Prints (2.0, 4.0, 8.0)\n```\n\n\n**Note:** If the value returned by \\[method determinant] is negative, the scale is also negative."]
    pub fn get_scale(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(405usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_scale", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this basis's rotation as a [`Vector3`][crate::builtin::Vector3] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians. For the returned value:\n\n- The \\[member Vector3.x] contains the angle around the \\[member x] axis (pitch);\n\n- The \\[member Vector3.y] contains the angle around the \\[member y] axis (yaw);\n\n- The \\[member Vector3.z] contains the angle around the \\[member z] axis (roll).\n\nThe order of each consecutive rotation can be changed with `order` (see \\[enum EulerOrder] constants). By default, the YXZ convention is used ([`EulerOrder::YXZ`][`crate::builtin::EulerOrder::YXZ`]): Z (roll) is calculated first, then X (pitch), and lastly Y (yaw). When using the opposite method \\[method from_euler], this order is reversed.\n\n**Note:** For this method to return correctly, the basis needs to be _orthonormal_ (see \\[method orthonormalized]).\n\n**Note:** Euler angles are much more intuitive but are not suitable for 3D math. Because of this, consider using the \\[method get_rotation_quaternion] method instead, which returns a [`Quaternion`][crate::builtin::Quaternion].\n\n**Note:** In the Inspector dock, a basis's rotation is often displayed in Euler angles (in degrees), as is the case with the \\[member Node3D.rotation] property."]
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(406usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_euler", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the transposed dot product between `with` and the \\[member x] axis (see \\[method transposed]).\n\nThis is equivalent to `basis.x.dot(vector)`."]
    pub fn tdotx(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(407usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdotx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the transposed dot product between `with` and the \\[member y] axis (see \\[method transposed]).\n\nThis is equivalent to `basis.y.dot(vector)`."]
    pub fn tdoty(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(408usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdoty", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the transposed dot product between `with` and the \\[member z] axis (see \\[method transposed]).\n\nThis is equivalent to `basis.z.dot(vector)`."]
    pub fn tdotz(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(409usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdotz", self.sys_ptr, args)
        }
    }
    #[doc = "Performs a spherical-linear interpolation with the `to` basis, given a `weight`. Both this basis and `to` should represent a rotation.\n\n**Example:** Smoothly rotate a [`Node3D`][crate::classes::Node3D] to the target basis over time, with a [`Tween`][crate::classes::Tween]:\n\n```gdscript\nvar start_basis = Basis.IDENTITY\nvar target_basis = Basis.IDENTITY.rotated(Vector3.UP, TAU / 2)\n\nfunc _ready():\n\tcreate_tween().tween_method(interpolate, 0.0, 1.0, 5.0).set_trans(Tween.TRANS_EXPO)\n\nfunc interpolate(weight):\n\tbasis = start_basis.slerp(target_basis, weight)\n```"]
    pub fn slerp(&self, to: Basis, weight: f64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Basis, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(410usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "slerp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this basis is conformal. A conformal basis is both _orthogonal_ (the axes are perpendicular to each other) and _uniform_ (the axes share the same length). This method can be especially useful during physics calculations."]
    pub fn is_conformal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(411usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_conformal", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this basis and `b` are approximately equal, by calling [`is_equal_approx`][`crate::global::is_equal_approx`] on all vector components."]
    pub fn is_equal_approx(&self, b: Basis,) -> bool {
        type CallRet = bool;
        type CallParams = (Basis,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(412usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this basis is finite, by calling [`is_finite`][`crate::global::is_finite`] on all vector components."]
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(413usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_finite", self.sys_ptr, args)
        }
    }
    #[doc = "Returns this basis's rotation as a [`Quaternion`][crate::builtin::Quaternion].\n\n**Note:** Quaternions are much more suitable for 3D math but are less intuitive. For user interfaces, consider using the \\[method get_euler] method, which returns Euler angles."]
    pub fn get_rotation_quaternion(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(414usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_rotation_quaternion", self.sys_ptr, args)
        }
    }
    #[doc = "Constructs a new [`Basis`][crate::builtin::Basis] that only represents scale, with no rotation or shear, from the given `scale` vector.\n\n\n```gdscript\nvar my_basis = Basis.from_scale(Vector3(2, 4, 8))\n\nprint(my_basis.x) # Prints (2.0, 0.0, 0.0)\nprint(my_basis.y) # Prints (0.0, 4.0, 0.0)\nprint(my_basis.z) # Prints (0.0, 0.0, 8.0)\n```\n\n\n**Note:** In linear algebra, the matrix of this basis is also known as a [diagonal matrix](https://en.wikipedia.org/wiki/Diagonal_matrix)."]
    pub fn from_scale(scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(416usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "from_scale", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Constructs a new [`Basis`][crate::builtin::Basis] that only represents rotation from the given [`Vector3`][crate::builtin::Vector3] of [Euler angles](https://en.wikipedia.org/wiki/Euler_angles), in radians.\n\n- The \\[member Vector3.x] should contain the angle around the \\[member x] axis (pitch);\n\n- The \\[member Vector3.y] should contain the angle around the \\[member y] axis (yaw);\n\n- The \\[member Vector3.z] should contain the angle around the \\[member z] axis (roll).\n\n\n```gdscript\n# Creates a Basis whose z axis points down.\nvar my_basis = Basis.from_euler(Vector3(TAU / 4, 0, 0))\n\nprint(my_basis.z) # Prints (0.0, -1.0, 0.0)\n```\n\n\nThe order of each consecutive rotation can be changed with `order` (see \\[enum EulerOrder] constants). By default, the YXZ convention is used ([`EulerOrder::YXZ`][`crate::builtin::EulerOrder::YXZ`]): the basis rotates first around the Y axis (yaw), then X (pitch), and lastly Z (roll). When using the opposite method \\[method get_euler], this order is reversed."]
    pub fn from_euler(euler: Vector3, order: i64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, i64,);
        let args = (euler, order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(417usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "from_euler", std::ptr::null_mut(), args)
        }
    }
}
pub use re_export::InnerBasis;
impl Basis {
    #[doc = "Creates a new [`Basis`][crate::builtin::Basis] with a rotation such that the forward axis (-Z) points towards the `target` position.\n\nBy default, the -Z axis (camera forward) is treated as forward (implies +X is right). If `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The returned basis is orthonormalized (see \\[method orthonormalized]).\n\nThe `target` and the `up` cannot be `Vector3.ZERO`, and shouldn't be colinear to avoid unintended rotation around local Z axis."]
    pub(crate) fn looking_at_full(target: Vector3, up: Vector3, use_model_front: bool,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, Vector3, bool,);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(415usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "looking_at", std::ptr::null_mut(), args)
        }
    }
    #[doc = "To set the default parameters, use [`looking_at_ex`][Self::looking_at_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
    #[doc = "Creates a new [`Basis`][crate::builtin::Basis] with a rotation such that the forward axis (-Z) points towards the `target` position.\n\nBy default, the -Z axis (camera forward) is treated as forward (implies +X is right). If `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The returned basis is orthonormalized (see \\[method orthonormalized]).\n\nThe `target` and the `up` cannot be `Vector3.ZERO`, and shouldn't be colinear to avoid unintended rotation around local Z axis."]
    #[inline]
    pub fn looking_at(target: Vector3,) -> Basis {
        Self::looking_at_ex(target,) . done()
    }
    #[doc = "Creates a new [`Basis`][crate::builtin::Basis] with a rotation such that the forward axis (-Z) points towards the `target` position.\n\nBy default, the -Z axis (camera forward) is treated as forward (implies +X is right). If `use_model_front` is `true`, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the `target` position.\n\nThe up axis (+Y) points as close to the `up` vector as possible while staying perpendicular to the forward axis. The returned basis is orthonormalized (see \\[method orthonormalized]).\n\nThe `target` and the `up` cannot be `Vector3.ZERO`, and shouldn't be colinear to avoid unintended rotation around local Z axis."]
    #[inline]
    pub fn looking_at_ex < 'ex > (target: Vector3,) -> ExLookingAt < 'ex > {
        ExLookingAt::new(target,)
    }
}
#[doc = "Default-param extender for [`Basis::looking_at_ex`][crate::builtin::Basis::looking_at_ex]."]
#[must_use]
pub struct ExLookingAt < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLookingAt < 'ex > {
    fn new(target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, target: target, up: up, use_model_front: use_model_front,
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
    pub fn done(self) -> Basis {
        let Self {
            _phantom, target, up, use_model_front,
        }
        = self;
        Basis::looking_at_full(target, up, use_model_front,)
    }
}