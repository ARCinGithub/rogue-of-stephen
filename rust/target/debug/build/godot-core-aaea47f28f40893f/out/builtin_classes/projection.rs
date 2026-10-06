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
    pub struct InnerProjection < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerProjection < 'inner > {
    pub fn from_outer(outer: &Projection) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions from a depth range of `-1` to `1` to one that ranges from `0` to `1`, and flips the projected positions vertically, according to `flip_y`."]
    pub fn create_depth_correction(flip_y: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (bool,);
        let args = (flip_y,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(431usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_depth_correction", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions into the given [`Rect2`][crate::builtin::Rect2]."]
    pub fn create_light_atlas_rect(rect: Rect2,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Rect2,);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(432usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_light_atlas_rect", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions using a perspective projection with the given Y-axis field of view (in degrees), X:Y aspect ratio, and clipping planes.\n\n`flip_fov` determines whether the projection's field of view is flipped over its diagonal."]
    pub fn create_perspective(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool,);
        let args = (fovy, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(433usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_perspective", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions using a perspective projection with the given Y-axis field of view (in degrees), X:Y aspect ratio, and clipping distances. The projection is adjusted for a head-mounted display with the given distance between eyes and distance to a point that can be focused on.\n\n`eye` creates the projection for the left eye when set to 1, or the right eye when set to 2.\n\n`flip_fov` determines whether the projection's field of view is flipped over its diagonal."]
    pub fn create_perspective_hmd(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool, eye: i64, intraocular_dist: f64, convergence_dist: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool, i64, f64, f64,);
        let args = (fovy, aspect, z_near, z_far, flip_fov, eye, intraocular_dist, convergence_dist,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(434usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_perspective_hmd", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] for projecting positions onto a head-mounted display with the given X:Y aspect ratio, distance between eyes, display width, distance to lens, oversampling factor, and depth clipping planes.\n\n`eye` creates the projection for the left eye when set to 1, or the right eye when set to 2."]
    pub fn create_for_hmd(eye: i64, aspect: f64, intraocular_dist: f64, display_width: f64, display_to_lens: f64, oversample: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (i64, f64, f64, f64, f64, f64, f64, f64,);
        let args = (eye, aspect, intraocular_dist, display_width, display_to_lens, oversample, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(435usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_for_hmd", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions using an orthogonal projection with the given clipping planes."]
    pub fn create_orthogonal(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, f64, f64,);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(436usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_orthogonal", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions using an orthogonal projection with the given size, X:Y aspect ratio, and clipping planes.\n\n`flip_fov` determines whether the projection's field of view is flipped over its diagonal."]
    pub fn create_orthogonal_aspect(size: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool,);
        let args = (size, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(437usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_orthogonal_aspect", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions in a frustum with the given clipping planes."]
    pub fn create_frustum(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, f64, f64,);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(438usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_frustum", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that projects positions in a frustum with the given size, X:Y aspect ratio, offset, and clipping planes.\n\n`flip_fov` determines whether the projection's field of view is flipped over its diagonal."]
    pub fn create_frustum_aspect(size: f64, aspect: f64, offset: Vector2, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, Vector2, f64, f64, bool,);
        let args = (size, aspect, offset, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(439usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_frustum_aspect", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a new [`Projection`][crate::builtin::Projection] that scales a given projection to fit around a given [`AABB`][crate::builtin::Aabb] in projection space."]
    pub fn create_fit_aabb(aabb: Aabb,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Aabb,);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(440usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_fit_aabb", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns a scalar value that is the signed factor by which areas are scaled by this matrix. If the sign is negative, the matrix flips the orientation of the area.\n\nThe determinant can be used to calculate the invertibility of a matrix or solve linear systems of equations involving the matrix, among other applications."]
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(441usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "determinant", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Projection`][crate::builtin::Projection] with the near clipping distance adjusted to be `new_znear`.\n\n**Note:** The original [`Projection`][crate::builtin::Projection] must be a perspective projection."]
    pub fn perspective_znear_adjusted(&self, new_znear: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64,);
        let args = (new_znear,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(442usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "perspective_znear_adjusted", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the clipping plane of this [`Projection`][crate::builtin::Projection] whose index is given by `plane`.\n\n`plane` should be equal to one of `PLANE_NEAR`, `PLANE_FAR`, `PLANE_LEFT`, `PLANE_TOP`, `PLANE_RIGHT`, or `PLANE_BOTTOM`."]
    pub fn get_projection_plane(&self, plane: i64,) -> Plane {
        type CallRet = Plane;
        type CallParams = (i64,);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(443usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_projection_plane", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of this [`Projection`][crate::builtin::Projection] with the signs of the values of the Y column flipped."]
    pub fn flipped_y(&self,) -> Projection {
        type CallRet = Projection;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(444usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "flipped_y", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Projection`][crate::builtin::Projection] with the X and Y values from the given [`Vector2`][crate::builtin::Vector2] added to the first and second values of the final column respectively."]
    pub fn jitter_offseted(&self, offset: Vector2,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(445usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "jitter_offseted", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the vertical field of view of the projection (in degrees) associated with the given horizontal field of view (in degrees) and aspect ratio.\n\n**Note:** Unlike most methods of [`Projection`][crate::builtin::Projection], `aspect` is expected to be 1 divided by the X:Y aspect ratio."]
    pub fn get_fovy(fovx: f64, aspect: f64,) -> f64 {
        type CallRet = f64;
        type CallParams = (f64, f64,);
        let args = (fovx, aspect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(446usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_fovy", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns the distance for this [`Projection`][crate::builtin::Projection] beyond which positions are clipped."]
    pub fn get_z_far(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(447usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_z_far", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the distance for this [`Projection`][crate::builtin::Projection] before which positions are clipped."]
    pub fn get_z_near(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(448usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_z_near", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the X:Y aspect ratio of this [`Projection`][crate::builtin::Projection]'s viewport."]
    pub fn get_aspect(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(449usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_aspect", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the horizontal field of view of the projection (in degrees)."]
    pub fn get_fov(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(450usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_fov", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this [`Projection`][crate::builtin::Projection] performs an orthogonal projection."]
    pub fn is_orthogonal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(451usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "is_orthogonal", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dimensions of the viewport plane that this [`Projection`][crate::builtin::Projection] projects positions onto, divided by two."]
    pub fn get_viewport_half_extents(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(452usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_viewport_half_extents", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the dimensions of the far clipping plane of the projection, divided by two."]
    pub fn get_far_plane_half_extents(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(453usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_far_plane_half_extents", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a [`Projection`][crate::builtin::Projection] that performs the inverse of this [`Projection`][crate::builtin::Projection]'s projective transformation."]
    pub fn inverse(&self,) -> Projection {
        type CallRet = Projection;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(454usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "inverse", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `for_pixel_width` divided by the viewport's width measured in meters on the near plane, after this [`Projection`][crate::builtin::Projection] is applied."]
    pub fn get_pixels_per_meter(&self, for_pixel_width: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (for_pixel_width,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(455usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_pixels_per_meter", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the factor by which the visible level of detail is scaled by this [`Projection`][crate::builtin::Projection]."]
    pub fn get_lod_multiplier(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(456usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_lod_multiplier", self.sys_ptr, args)
        }
    }
}
pub use re_export::InnerProjection;
impl Projection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Planes {
    ord: i32
}
impl Planes {
    #[doc(alias = "PLANE_NEAR")]
    #[doc = "Godot enumerator name: `PLANE_NEAR`"]
    pub const NEAR: Planes = Planes {
        ord: 0i32
    };
    #[doc(alias = "PLANE_FAR")]
    #[doc = "Godot enumerator name: `PLANE_FAR`"]
    pub const FAR: Planes = Planes {
        ord: 1i32
    };
    #[doc(alias = "PLANE_LEFT")]
    #[doc = "Godot enumerator name: `PLANE_LEFT`"]
    pub const LEFT: Planes = Planes {
        ord: 2i32
    };
    #[doc(alias = "PLANE_TOP")]
    #[doc = "Godot enumerator name: `PLANE_TOP`"]
    pub const TOP: Planes = Planes {
        ord: 3i32
    };
    #[doc(alias = "PLANE_RIGHT")]
    #[doc = "Godot enumerator name: `PLANE_RIGHT`"]
    pub const RIGHT: Planes = Planes {
        ord: 4i32
    };
    #[doc(alias = "PLANE_BOTTOM")]
    #[doc = "Godot enumerator name: `PLANE_BOTTOM`"]
    pub const BOTTOM: Planes = Planes {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Planes {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Planes") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Planes {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NEAR => "NEAR", Self::FAR => "FAR", Self::LEFT => "LEFT", Self::TOP => "TOP", Self::RIGHT => "RIGHT", Self::BOTTOM => "BOTTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Planes::NEAR, Planes::FAR, Planes::LEFT, Planes::TOP, Planes::RIGHT, Planes::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Planes >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAR", "PLANE_NEAR", Planes::NEAR), crate::meta::inspect::EnumConstant::new("FAR", "PLANE_FAR", Planes::FAR), crate::meta::inspect::EnumConstant::new("LEFT", "PLANE_LEFT", Planes::LEFT), crate::meta::inspect::EnumConstant::new("TOP", "PLANE_TOP", Planes::TOP), crate::meta::inspect::EnumConstant::new("RIGHT", "PLANE_RIGHT", Planes::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM", "PLANE_BOTTOM", Planes::BOTTOM)]
        }
    }
}
impl crate::meta::GodotConvert for Planes {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Plane Near", 0i64), EnumeratorShape::new_int("Plane Far", 1i64), EnumeratorShape::new_int("Plane Left", 2i64), EnumeratorShape::new_int("Plane Top", 3i64), EnumeratorShape::new_int("Plane Right", 4i64), EnumeratorShape::new_int("Plane Bottom", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Projection.Planes")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Planes {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Planes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Planes {
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
impl crate::registry::property::Export for Planes {
    
}
impl crate::meta::Element for Planes {
    
}