#![doc = "Sidecar module for class [`PhysicsDirectSpaceState3D`][crate::classes::PhysicsDirectSpaceState3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectSpaceState3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectSpaceState3D`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_space_state_3d`][crate::classes::physics_direct_space_state_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsDirectSpaceState3D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectSpaceState3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nProvides direct access to a physics space in the [`PhysicsServer3D`][crate::classes::PhysicsServer3D]. It's used mainly to do queries against objects and areas residing in a given space.\n\n**Note:** This class is not meant to be instantiated directly. Use \\[member World3D.direct_space_state] to get the world's physics 3D space state."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectSpaceState3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsDirectSpaceState3D {
        #[doc = "Checks whether a point is inside any solid shape. Position and other parameters are defined through [`PhysicsPointQueryParameters3D`][crate::classes::PhysicsPointQueryParameters3D]. The shapes the point is inside of are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time."]
        pub(crate) fn intersect_point_full(&mut self, parameters: CowArg < Gd < crate::classes::PhysicsPointQueryParameters3D > >, max_results: i32,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsPointQueryParameters3D > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "intersect_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`intersect_point_ex`][Self::intersect_point_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Checks whether a point is inside any solid shape. Position and other parameters are defined through [`PhysicsPointQueryParameters3D`][crate::classes::PhysicsPointQueryParameters3D]. The shapes the point is inside of are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time."]
        #[inline]
        pub fn intersect_point(&mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsPointQueryParameters3D >>,) -> Array < VarDictionary > {
            self.intersect_point_ex(parameters,) . done()
        }
        #[doc = "Checks whether a point is inside any solid shape. Position and other parameters are defined through [`PhysicsPointQueryParameters3D`][crate::classes::PhysicsPointQueryParameters3D]. The shapes the point is inside of are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time."]
        #[inline]
        pub fn intersect_point_ex < 'ex > (&'ex mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsPointQueryParameters3D >> + 'ex,) -> ExIntersectPoint < 'ex > {
            ExIntersectPoint::new(self, parameters,)
        }
        #[doc = "Intersects a ray in a given space. Ray position and other parameters are defined through [`PhysicsRayQueryParameters3D`][crate::classes::PhysicsRayQueryParameters3D]. The returned object is a dictionary with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`normal`: The object's surface normal at the intersection point, or `Vector3(0, 0, 0)` if the ray starts inside the shape and \\[member PhysicsRayQueryParameters3D.hit_from_inside] is `true`.\n\n`position`: The intersection point.\n\n`face_index`: The face index at the intersection point.\n\n**Note:** Returns a valid number only if the intersected shape is a [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D]. Otherwise, `-1` is returned.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nIf the ray did not intersect anything, then an empty dictionary is returned instead."]
        pub fn intersect_ray(&mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsRayQueryParameters3D >>,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsRayQueryParameters3D > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "intersect_ray", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The intersected shapes are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        pub(crate) fn intersect_shape_full(&mut self, parameters: CowArg < Gd < crate::classes::PhysicsShapeQueryParameters3D > >, max_results: i32,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsShapeQueryParameters3D > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "intersect_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`intersect_shape_ex`][Self::intersect_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The intersected shapes are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        #[inline]
        pub fn intersect_shape(&mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >>,) -> Array < VarDictionary > {
            self.intersect_shape_ex(parameters,) . done()
        }
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The intersected shapes are returned in an array containing dictionaries with the following fields:\n\n`collider`: The colliding object.\n\n`collider_id`: The colliding object's ID.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nThe number of intersections can be limited with the `max_results` parameter, to reduce the processing time.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        #[inline]
        pub fn intersect_shape_ex < 'ex > (&'ex mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >> + 'ex,) -> ExIntersectShape < 'ex > {
            ExIntersectShape::new(self, parameters,)
        }
        #[doc = "Checks how far a [`Shape3D`][crate::classes::Shape3D] can move without colliding. All the parameters for the query, including the shape and the motion, are supplied through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object.\n\nReturns an array with the safe and unsafe proportions (between 0 and 1) of the motion. The safe proportion is the maximum fraction of the motion that can be made without a collision. The unsafe proportion is the minimum fraction of the distance that must be moved for a collision. If no collision is detected a result of `[1.0, 1.0]` will be returned.\n\n**Note:** Any [`Shape3D`][crate::classes::Shape3D]s that the shape is already colliding with e.g. inside of, will be ignored. Use [`collide_shape`][`crate::classes::PhysicsDirectSpaceState3D::collide_shape`] to determine the [`Shape3D`][crate::classes::Shape3D]s that the shape is already colliding with."]
        pub fn cast_motion(&mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >>,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsShapeQueryParameters3D > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "cast_motion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The resulting array contains a list of points where the shape intersects another. Like with [`intersect_shape`][`crate::classes::PhysicsDirectSpaceState3D::intersect_shape`], the number of returned results can be limited to save processing time.\n\nReturned points are a list of pairs of contact points. For each pair the first one is in the shape passed in [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, second one is in the collided shape from the physics space.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        pub(crate) fn collide_shape_full(&mut self, parameters: CowArg < Gd < crate::classes::PhysicsShapeQueryParameters3D > >, max_results: i32,) -> Array < Vector3 > {
            type CallRet = Array < Vector3 >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsShapeQueryParameters3D > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "collide_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`collide_shape_ex`][Self::collide_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The resulting array contains a list of points where the shape intersects another. Like with [`intersect_shape`][`crate::classes::PhysicsDirectSpaceState3D::intersect_shape`], the number of returned results can be limited to save processing time.\n\nReturned points are a list of pairs of contact points. For each pair the first one is in the shape passed in [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, second one is in the collided shape from the physics space.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        #[inline]
        pub fn collide_shape(&mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >>,) -> Array < Vector3 > {
            self.collide_shape_ex(parameters,) . done()
        }
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. The resulting array contains a list of points where the shape intersects another. Like with [`intersect_shape`][`crate::classes::PhysicsDirectSpaceState3D::intersect_shape`], the number of returned results can be limited to save processing time.\n\nReturned points are a list of pairs of contact points. For each pair the first one is in the shape passed in [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, second one is in the collided shape from the physics space.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        #[inline]
        pub fn collide_shape_ex < 'ex > (&'ex mut self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >> + 'ex,) -> ExCollideShape < 'ex > {
            ExCollideShape::new(self, parameters,)
        }
        #[doc = "Checks the intersections of a shape, given through a [`PhysicsShapeQueryParameters3D`][crate::classes::PhysicsShapeQueryParameters3D] object, against the space. If it collides with more than one shape, the nearest one is selected. The returned object is a dictionary containing the following fields:\n\n`collider_id`: The colliding object's ID.\n\n`linear_velocity`: The colliding object's velocity [`Vector3`][crate::builtin::Vector3]. If the object is an [`Area3D`][crate::classes::Area3D], the result is `(0, 0, 0)`.\n\n`normal`: The collision normal of the query shape at the intersection point, pointing away from the intersecting object.\n\n`point`: The intersection point.\n\n`rid`: The intersecting object's [`RID`][crate::builtin::Rid].\n\n`shape`: The shape index of the colliding shape.\n\nIf the shape did not intersect anything, then an empty dictionary is returned instead.\n\n**Note:** This method does not take into account the `motion` property of the object."]
        pub fn get_rest_info(&self, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >>,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::PhysicsShapeQueryParameters3D > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState3D", "get_rest_info", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectSpaceState3D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsDirectSpaceState3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectSpaceState3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectSpaceState3D {
        
    }
    impl std::ops::Deref for PhysicsDirectSpaceState3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectSpaceState3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsDirectSpaceState3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsDirectSpaceState3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState3D::intersect_point_ex`][super::PhysicsDirectSpaceState3D::intersect_point_ex]."]
#[must_use]
pub struct ExIntersectPoint < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: CowArg < 'ex, Gd < crate::classes::PhysicsPointQueryParameters3D > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIntersectPoint < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: impl AsArg < Gd < crate::classes::PhysicsPointQueryParameters3D >> + 'ex,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState3D::intersect_point_full(surround_object, parameters, max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState3D::intersect_shape_ex`][super::PhysicsDirectSpaceState3D::intersect_shape_ex]."]
#[must_use]
pub struct ExIntersectShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: CowArg < 'ex, Gd < crate::classes::PhysicsShapeQueryParameters3D > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIntersectShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >> + 'ex,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState3D::intersect_shape_full(surround_object, parameters, max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState3D::collide_shape_ex`][super::PhysicsDirectSpaceState3D::collide_shape_ex]."]
#[must_use]
pub struct ExCollideShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: CowArg < 'ex, Gd < crate::classes::PhysicsShapeQueryParameters3D > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCollideShape < 'ex > {
    fn new(surround_object: &'ex mut re_export::PhysicsDirectSpaceState3D, parameters: impl AsArg < Gd < crate::classes::PhysicsShapeQueryParameters3D >> + 'ex,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector3 > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState3D::collide_shape_full(surround_object, parameters, max_results,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsDirectSpaceState3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsDirectSpaceState3D {
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