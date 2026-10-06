#![doc = "Sidecar module for class [`Geometry3D`][crate::classes::Geometry3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#enumerations).\n\n"]
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
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Geometry3D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`geometry_3d`][crate::classes::geometry_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Geometry3D`](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nProvides a set of helper functions to create geometric shapes, compute intersections between shapes, and process various other geometric operations in 3D."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Geometry3D {
        #[doc = "Calculates and returns all the vertex points of a convex shape defined by an array of `planes`."]
        pub fn compute_convex_mesh_points(&mut self, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Plane > >,);
            let args = (RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "compute_convex_mesh_points", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with 6 [`Plane`][crate::builtin::Plane]s that describe the sides of a box centered at the origin. The box size is defined by `extents`, which represents one (positive) corner of the box (i.e. half its actual size)."]
        pub fn build_box_planes(&mut self, extents: Vector3,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (Vector3,);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_box_planes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted cylinder centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the round part of the cylinder. The parameter `axis` describes the axis along which the cylinder is oriented (0 for X, 1 for Y, 2 for Z)."]
        pub(crate) fn build_cylinder_planes_full(&mut self, radius: f32, height: f32, sides: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (f32, f32, i32, Vector3Axis,);
            let args = (radius, height, sides, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_cylinder_planes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`build_cylinder_planes_ex`][Self::build_cylinder_planes_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted cylinder centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the round part of the cylinder. The parameter `axis` describes the axis along which the cylinder is oriented (0 for X, 1 for Y, 2 for Z)."]
        #[inline]
        pub fn build_cylinder_planes(&mut self, radius: f32, height: f32, sides: i32,) -> Array < Plane > {
            self.build_cylinder_planes_ex(radius, height, sides,) . done()
        }
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted cylinder centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the round part of the cylinder. The parameter `axis` describes the axis along which the cylinder is oriented (0 for X, 1 for Y, 2 for Z)."]
        #[inline]
        pub fn build_cylinder_planes_ex < 'ex > (&'ex mut self, radius: f32, height: f32, sides: i32,) -> ExBuildCylinderPlanes < 'ex > {
            ExBuildCylinderPlanes::new(self, radius, height, sides,)
        }
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted capsule centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the side part of the capsule, whereas `lats` gives the number of latitudinal steps at the bottom and top of the capsule. The parameter `axis` describes the axis along which the capsule is oriented (0 for X, 1 for Y, 2 for Z)."]
        pub(crate) fn build_capsule_planes_full(&mut self, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (f32, f32, i32, i32, Vector3Axis,);
            let args = (radius, height, sides, lats, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_capsule_planes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`build_capsule_planes_ex`][Self::build_capsule_planes_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted capsule centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the side part of the capsule, whereas `lats` gives the number of latitudinal steps at the bottom and top of the capsule. The parameter `axis` describes the axis along which the capsule is oriented (0 for X, 1 for Y, 2 for Z)."]
        #[inline]
        pub fn build_capsule_planes(&mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> Array < Plane > {
            self.build_capsule_planes_ex(radius, height, sides, lats,) . done()
        }
        #[doc = "Returns an array of [`Plane`][crate::builtin::Plane]s closely bounding a faceted capsule centered at the origin with radius `radius` and height `height`. The parameter `sides` defines how many planes will be generated for the side part of the capsule, whereas `lats` gives the number of latitudinal steps at the bottom and top of the capsule. The parameter `axis` describes the axis along which the capsule is oriented (0 for X, 1 for Y, 2 for Z)."]
        #[inline]
        pub fn build_capsule_planes_ex < 'ex > (&'ex mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> ExBuildCapsulePlanes < 'ex > {
            ExBuildCapsulePlanes::new(self, radius, height, sides, lats,)
        }
        #[doc = "Given the two 3D segments (`p1`, `p2`) and (`q1`, `q2`), finds those two points on the two segments that are closest to each other. Returns a [`PackedVector3Array`][crate::builtin::PackedVector3Array] that contains this point on (`p1`, `p2`) as well the accompanying point on (`q1`, `q2`)."]
        pub fn get_closest_points_between_segments(&self, p1: Vector3, p2: Vector3, q1: Vector3, q2: Vector3,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, Vector3, Vector3,);
            let args = (p1, p2, q1, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_points_between_segments", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 3D point on the 3D segment (`s1`, `s2`) that is closest to `point`. The returned point will always be inside the specified segment."]
        pub fn get_closest_point_to_segment(&self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 3D point on the 3D line defined by (`s1`, `s2`) that is closest to `point`. The returned point can be inside the segment (`s1`, `s2`) or outside of it, i.e. somewhere on the line extending from the segment."]
        pub fn get_closest_point_to_segment_uncapped(&self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment_uncapped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Vector3`][crate::builtin::Vector3] containing weights based on how close a 3D position (`point`) is to a triangle's different vertices (`a`, `b` and `c`). This is useful for interpolating between the data of different vertices in a triangle. One example use case is using this to smoothly rotate over a mesh instead of relying solely on face normals.\n\n[Here is a more detailed explanation of barycentric coordinates.](https://en.wikipedia.org/wiki/Barycentric_coordinate_system)"]
        pub fn get_triangle_barycentric_coords(&self, point: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3, Vector3,);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_triangle_barycentric_coords", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Tests if the 3D ray starting at `from` with the direction of `dir` intersects the triangle specified by `a`, `b` and `c`. If yes, returns the point of intersection as [`Vector3`][crate::builtin::Vector3]. If no intersection takes place, returns `null`."]
        pub fn ray_intersects_triangle(&mut self, from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector3, Vector3, Vector3, Vector3, Vector3,);
            let args = (from, dir, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "ray_intersects_triangle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Tests if the segment (`from`, `to`) intersects the triangle `a`, `b`, `c`. If yes, returns the point of intersection as [`Vector3`][crate::builtin::Vector3]. If no intersection takes place, returns `null`."]
        pub fn segment_intersects_triangle(&mut self, from: Vector3, to: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector3, Vector3, Vector3, Vector3, Vector3,);
            let args = (from, to, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_triangle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks if the segment (`from`, `to`) intersects the sphere that is located at `sphere_position` and has radius `sphere_radius`. If no, returns an empty [`PackedVector3Array`][crate::builtin::PackedVector3Array]. If yes, returns a [`PackedVector3Array`][crate::builtin::PackedVector3Array] containing the point of intersection and the sphere's normal at the point of intersection."]
        pub fn segment_intersects_sphere(&mut self, from: Vector3, to: Vector3, sphere_position: Vector3, sphere_radius: f32,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, Vector3, f32,);
            let args = (from, to, sphere_position, sphere_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_sphere", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks if the segment (`from`, `to`) intersects the cylinder with height `height` that is centered at the origin and has radius `radius`. If no, returns an empty [`PackedVector3Array`][crate::builtin::PackedVector3Array]. If an intersection takes place, the returned array contains the point of intersection and the cylinder's normal at the point of intersection."]
        pub fn segment_intersects_cylinder(&mut self, from: Vector3, to: Vector3, height: f32, radius: f32,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, f32, f32,);
            let args = (from, to, height, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_cylinder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Given a convex hull defined though the [`Plane`][crate::builtin::Plane]s in the array `planes`, tests if the segment (`from`, `to`) intersects with that hull. If an intersection is found, returns a [`PackedVector3Array`][crate::builtin::PackedVector3Array] containing the point the intersection and the hull's normal. Otherwise, returns an empty array."]
        pub fn segment_intersects_convex(&mut self, from: Vector3, to: Vector3, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (Vector3, Vector3, RefArg < 'a0, Array < Plane > >,);
            let args = (from, to, RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_convex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clips the polygon defined by the points in `points` against the `plane` and returns the points of the clipped polygon."]
        pub fn clip_polygon(&mut self, points: &PackedVector3Array, plane: Plane,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >, Plane,);
            let args = (RefArg::new(points), plane,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "clip_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Tetrahedralizes the volume specified by a discrete set of `points` in 3D space, ensuring that no point lies within the circumsphere of any resulting tetrahedron. The method returns a [`PackedInt32Array`][crate::builtin::PackedInt32Array] where each tetrahedron consists of four consecutive point indices into the `points` array (resulting in an array with `n * 4` elements, where `n` is the number of tetrahedra found). If the tetrahedralization is unsuccessful, an empty [`PackedInt32Array`][crate::builtin::PackedInt32Array] is returned."]
        pub fn tetrahedralize_delaunay(&mut self, points: &PackedVector3Array,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "tetrahedralize_delaunay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" Creates a validated object for FFI boundary crossing."]
        #[doc = r""]
        #[doc = r" Low-level internal method. Validation (liveness/type checks) depend on safeguard level."]
        fn __validated_obj(&self) -> crate::obj::ValidatedObject {
            let raw_gd = unsafe {
                std::mem::transmute::< &Self, &crate::obj::RawGd < Self >> (self)
            };
            raw_gd.validated_object()
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for Geometry3D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Geometry3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Geometry3D {
        
    }
    impl crate::obj::Singleton for Geometry3D {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Geometry3D"))
            }
        }
    }
    impl std::ops::Deref for Geometry3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Geometry3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Geometry3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_cylinder_planes_ex`][super::Geometry3D::build_cylinder_planes_ex]."]
#[must_use]
pub struct ExBuildCylinderPlanes < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBuildCylinderPlanes < 'ex > {
    fn new(surround_object: &'ex mut re_export::Geometry3D, radius: f32, height: f32, sides: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, axis,
        }
        = self;
        re_export::Geometry3D::build_cylinder_planes_full(surround_object, radius, height, sides, axis,)
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_capsule_planes_ex`][super::Geometry3D::build_capsule_planes_ex]."]
#[must_use]
pub struct ExBuildCapsulePlanes < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBuildCapsulePlanes < 'ex > {
    fn new(surround_object: &'ex mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, lats: lats, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, lats, axis,
        }
        = self;
        re_export::Geometry3D::build_capsule_planes_full(surround_object, radius, height, sides, lats, axis,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Geometry3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Geometry3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}