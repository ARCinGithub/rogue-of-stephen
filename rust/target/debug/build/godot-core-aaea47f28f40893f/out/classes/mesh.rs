#![doc = "Sidecar module for class [`Mesh`][crate::classes::Mesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Mesh` enums](https://docs.godotengine.org/en/stable/classes/class_mesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Mesh`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`mesh`][crate::classes::mesh]: sidecar module with related enum/flag types\n* [`IMesh`][crate::classes::IMesh]: virtual methods\n\n\nSee also [Godot docs for `Mesh`](https://docs.godotengine.org/en/stable/classes/class_mesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Mesh::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nMesh is a type of [`Resource`][crate::classes::Resource] that contains vertex array-based geometry, divided in _surfaces_. Each surface contains a completely separate array and a material used to draw it. Design wise, a mesh with multiple surfaces is preferred to a single surface, because objects created in 3D editing software commonly contain multiple materials. The maximum number of surfaces per mesh is `RenderingServer.MAX_MESH_SURFACES`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Mesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Mesh`][crate::classes::Mesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Mesh` methods](https://docs.godotengine.org/en/stable/classes/class_mesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMesh: crate::obj::GodotClass < Base = Mesh > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn on_get(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn on_set(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `on_get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn on_validate_property(&self, property: &mut crate::registry::info::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn on_get_property_list(&mut self) -> Vec < crate::registry::info::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn on_property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = "Virtual method to override the surface count for a custom class extending `Mesh`."]
        fn get_surface_count(&self,) -> i32;
        #[doc = "Virtual method to override the surface array length for a custom class extending `Mesh`."]
        fn surface_get_array_len(&self, index: i32,) -> i32;
        #[doc = "Virtual method to override the surface array index length for a custom class extending `Mesh`."]
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        #[doc = "Virtual method to override the surface arrays for a custom class extending `Mesh`."]
        fn surface_get_arrays(&self, index: i32,) -> AnyArray;
        #[doc = "Virtual method to override the blend shape arrays for a custom class extending `Mesh`."]
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < AnyArray >;
        #[doc = "Virtual method to override the surface LODs for a custom class extending `Mesh`."]
        fn surface_get_lods(&self, index: i32,) -> AnyDictionary;
        #[doc = "Virtual method to override the surface format for a custom class extending `Mesh`."]
        fn surface_get_format(&self, index: i32,) -> u32;
        #[doc = "Virtual method to override the surface primitive type for a custom class extending `Mesh`."]
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        #[doc = "Virtual method to override the setting of a `material` at the given `index` for a custom class extending `Mesh`."]
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        #[doc = "Virtual method to override the surface material for a custom class extending `Mesh`."]
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        #[doc = "Virtual method to override the number of blend shapes for a custom class extending `Mesh`."]
        fn get_blend_shape_count(&self,) -> i32;
        #[doc = "Virtual method to override the retrieval of blend shape names for a custom class extending `Mesh`."]
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        #[doc = "Virtual method to override the names of blend shapes for a custom class extending `Mesh`."]
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        #[doc = "Virtual method to override the [`AABB`][crate::builtin::Aabb] for a custom class extending `Mesh`."]
        fn get_aabb(&self,) -> Aabb;
        #[doc = "Override this method to customize the newly duplicated resource created from [`instantiate`][`crate::classes::PackedScene::instantiate`], if the original's \\[member resource_local_to_scene] is set to `true`.\n\n**Example:** Set a random `damage` value to every local resource from an instantiated scene:\n\n```gdscript\nextends Resource\n\nvar damage = 0\n\nfunc _setup_local_to_scene():\n\tdamage = randi_range(10, 40)\n```"]
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to return a custom [`RID`][crate::builtin::Rid] when [`get_rid`][`crate::classes::Resource::get_rid`] is called."]
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        #[doc = "For resources that store state in non-exported properties, such as via [`on_validate_property`][`crate::classes::IObject::on_validate_property`] or [`on_get_property_list`][`crate::classes::IObject::on_get_property_list`], this method must be implemented to clear them."]
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to execute additional logic after [`set_path_cache`][`crate::classes::Resource::set_path_cache`] is called on this object."]
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl Mesh {
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "set_lightmap_size_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_lightmap_size_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the smallest [`AABB`][crate::builtin::Aabb] enclosing this mesh in local space. Not affected by `custom_aabb`.\n\n**Note:** This is only implemented for [`ArrayMesh`][crate::classes::ArrayMesh] and [`PrimitiveMesh`][crate::classes::PrimitiveMesh]."]
        pub fn get_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all the vertices that make up the faces of the mesh. Each three vertices represent one triangle."]
        pub fn get_faces(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_faces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of surfaces that the `Mesh` holds. This is equivalent to [`get_surface_override_material_count`][`crate::classes::MeshInstance3D::get_surface_override_material_count`]."]
        pub fn get_surface_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_surface_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the arrays for the vertices, normals, UVs, etc. that make up the requested surface (see [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`])."]
        pub fn surface_get_arrays(&self, surf_idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the blend shape arrays for the requested surface."]
        pub fn surface_get_blend_shape_arrays(&self, surf_idx: i32,) -> Array < VarArray > {
            type CallRet = Array < VarArray >;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_blend_shape_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a [`Material`][crate::classes::Material] for a given surface. Surface will be rendered using this material.\n\n**Note:** This assigns the material within the `Mesh` resource, not the [`Material`][crate::classes::Material] associated to the [`MeshInstance3D`][crate::classes::MeshInstance3D]'s Surface Material Override properties. To set the [`Material`][crate::classes::Material] associated to the [`MeshInstance3D`][crate::classes::MeshInstance3D]'s Surface Material Override properties, use [`set_surface_override_material`][`crate::classes::MeshInstance3D::set_surface_override_material`] instead."]
        pub fn surface_set_material(&mut self, surf_idx: i32, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (surf_idx, material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Material`][crate::classes::Material] in a given surface. Surface is rendered using this material.\n\n**Note:** This returns the material within the `Mesh` resource, not the [`Material`][crate::classes::Material] associated to the [`MeshInstance3D`][crate::classes::MeshInstance3D]'s Surface Material Override properties. To get the [`Material`][crate::classes::Material] associated to the [`MeshInstance3D`][crate::classes::MeshInstance3D]'s Surface Material Override properties, use [`get_surface_override_material`][`crate::classes::MeshInstance3D::get_surface_override_material`] instead."]
        pub fn surface_get_material(&self, surf_idx: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a placeholder version of this resource ([`PlaceholderMesh`][crate::classes::PlaceholderMesh])."]
        pub fn create_placeholder(&self,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calculate a [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D] from the mesh."]
        pub fn create_trimesh_shape(&self,) -> Option < Gd < crate::classes::ConcavePolygonShape3D > > {
            type CallRet = Option < Gd < crate::classes::ConcavePolygonShape3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_trimesh_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calculate a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] from the mesh.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        pub(crate) fn create_convex_shape_full(&self, clean: bool, simplify: bool,) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
            type CallRet = Option < Gd < crate::classes::ConvexPolygonShape3D > >;
            type CallParams = (bool, bool,);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_convex_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_convex_shape_ex`][Self::create_convex_shape_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Calculate a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] from the mesh.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        #[inline]
        pub fn create_convex_shape(&self,) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
            self.create_convex_shape_ex() . done()
        }
        #[doc = "Calculate a [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] from the mesh.\n\nIf `clean` is `true` (default), duplicate and interior vertices are removed automatically. You can set it to `false` to make the process faster if not needed.\n\nIf `simplify` is `true`, the geometry can be further simplified to reduce the number of vertices. Disabled by default."]
        #[inline]
        pub fn create_convex_shape_ex < 'ex > (&'ex self,) -> ExCreateConvexShape < 'ex > {
            ExCreateConvexShape::new(self,)
        }
        #[doc = "Calculate an outline mesh at a defined offset (margin) from the original mesh.\n\n**Note:** This method typically returns the vertices in reverse order (e.g. clockwise to counterclockwise)."]
        pub fn create_outline(&self, margin: f32,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generate a [`TriangleMesh`][crate::classes::TriangleMesh] from the mesh. Considers only surfaces using one of these primitive types: [`PrimitiveType::TRIANGLES`][`crate::classes::mesh::PrimitiveType::TRIANGLES`], [`PrimitiveType::TRIANGLE_STRIP`][`crate::classes::mesh::PrimitiveType::TRIANGLE_STRIP`]."]
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::classes::TriangleMesh > > {
            type CallRet = Option < Gd < crate::classes::TriangleMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "generate_triangle_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Mesh {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Mesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Mesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Mesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Mesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Mesh {
        
    }
    impl crate::obj::cap::GodotDefault for Mesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Mesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Mesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Mesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Mesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Mesh::create_convex_shape_ex`][super::Mesh::create_convex_shape_ex]."]
#[must_use]
pub struct ExCreateConvexShape < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Mesh, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateConvexShape < 'ex > {
    fn new(surround_object: &'ex re_export::Mesh,) -> Self {
        let clean = true;
        let simplify = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, clean: clean, simplify: simplify,
        }
    }
    #[inline]
    pub fn clean(self, clean: bool) -> Self {
        Self {
            clean: clean, .. self
        }
    }
    #[inline]
    pub fn simplify(self, simplify: bool) -> Self {
        Self {
            simplify: simplify, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
        let Self {
            _phantom, surround_object, clean, simplify,
        }
        = self;
        re_export::Mesh::create_convex_shape_full(surround_object, clean, simplify,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrimitiveType {
    ord: i32
}
impl PrimitiveType {
    #[doc(alias = "PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `PRIMITIVE_POINTS`"]
    pub const POINTS: PrimitiveType = PrimitiveType {
        ord: 0i32
    };
    #[doc(alias = "PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINES`"]
    pub const LINES: PrimitiveType = PrimitiveType {
        ord: 1i32
    };
    #[doc(alias = "PRIMITIVE_LINE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINE_STRIP`"]
    pub const LINE_STRIP: PrimitiveType = PrimitiveType {
        ord: 2i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: PrimitiveType = PrimitiveType {
        ord: 3i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLE_STRIP`"]
    pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PrimitiveType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PrimitiveType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINE_STRIP => "LINE_STRIP", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLE_STRIP => "TRIANGLE_STRIP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PrimitiveType::POINTS, PrimitiveType::LINES, PrimitiveType::LINE_STRIP, PrimitiveType::TRIANGLES, PrimitiveType::TRIANGLE_STRIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PrimitiveType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINTS", "PRIMITIVE_POINTS", PrimitiveType::POINTS), crate::meta::inspect::EnumConstant::new("LINES", "PRIMITIVE_LINES", PrimitiveType::LINES), crate::meta::inspect::EnumConstant::new("LINE_STRIP", "PRIMITIVE_LINE_STRIP", PrimitiveType::LINE_STRIP), crate::meta::inspect::EnumConstant::new("TRIANGLES", "PRIMITIVE_TRIANGLES", PrimitiveType::TRIANGLES), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIP", "PRIMITIVE_TRIANGLE_STRIP", PrimitiveType::TRIANGLE_STRIP)]
        }
    }
}
impl crate::meta::GodotConvert for PrimitiveType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Primitive Points", 0i64), EnumeratorShape::new_int("Primitive Lines", 1i64), EnumeratorShape::new_int("Primitive Line Strip", 2i64), EnumeratorShape::new_int("Primitive Triangles", 3i64), EnumeratorShape::new_int("Primitive Triangle Strip", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Mesh.PrimitiveType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PrimitiveType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PrimitiveType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PrimitiveType {
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
impl crate::registry::property::Export for PrimitiveType {
    
}
impl crate::meta::Element for PrimitiveType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    ord: i32
}
impl ArrayType {
    #[doc(alias = "ARRAY_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_VERTEX`"]
    pub const VERTEX: ArrayType = ArrayType {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_NORMAL`"]
    pub const NORMAL: ArrayType = ArrayType {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_TANGENT`"]
    pub const TANGENT: ArrayType = ArrayType {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_COLOR`"]
    pub const COLOR: ArrayType = ArrayType {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV`"]
    pub const TEX_UV: ArrayType = ArrayType {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV2`"]
    pub const TEX_UV2: ArrayType = ArrayType {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM0`"]
    pub const CUSTOM0: ArrayType = ArrayType {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM1`"]
    pub const CUSTOM1: ArrayType = ArrayType {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM2`"]
    pub const CUSTOM2: ArrayType = ArrayType {
        ord: 8i32
    };
    #[doc(alias = "ARRAY_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM3`"]
    pub const CUSTOM3: ArrayType = ArrayType {
        ord: 9i32
    };
    #[doc(alias = "ARRAY_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_BONES`"]
    pub const BONES: ArrayType = ArrayType {
        ord: 10i32
    };
    #[doc(alias = "ARRAY_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_WEIGHTS`"]
    pub const WEIGHTS: ArrayType = ArrayType {
        ord: 11i32
    };
    #[doc(alias = "ARRAY_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_INDEX`"]
    pub const INDEX: ArrayType = ArrayType {
        ord: 12i32
    };
    #[doc(alias = "ARRAY_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_MAX`"]
    pub const MAX: ArrayType = ArrayType {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayType::VERTEX, ArrayType::NORMAL, ArrayType::TANGENT, ArrayType::COLOR, ArrayType::TEX_UV, ArrayType::TEX_UV2, ArrayType::CUSTOM0, ArrayType::CUSTOM1, ArrayType::CUSTOM2, ArrayType::CUSTOM3, ArrayType::BONES, ArrayType::WEIGHTS, ArrayType::INDEX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_VERTEX", ArrayType::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_NORMAL", ArrayType::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_TANGENT", ArrayType::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_COLOR", ArrayType::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_TEX_UV", ArrayType::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_TEX_UV2", ArrayType::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_CUSTOM0", ArrayType::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_CUSTOM1", ArrayType::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_CUSTOM2", ArrayType::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_CUSTOM3", ArrayType::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_BONES", ArrayType::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_WEIGHTS", ArrayType::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_INDEX", ArrayType::INDEX), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_MAX", ArrayType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayType {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for ArrayType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Vertex", 0i64), EnumeratorShape::new_int("Array Normal", 1i64), EnumeratorShape::new_int("Array Tangent", 2i64), EnumeratorShape::new_int("Array Color", 3i64), EnumeratorShape::new_int("Array Tex Uv", 4i64), EnumeratorShape::new_int("Array Tex Uv2", 5i64), EnumeratorShape::new_int("Array Custom0", 6i64), EnumeratorShape::new_int("Array Custom1", 7i64), EnumeratorShape::new_int("Array Custom2", 8i64), EnumeratorShape::new_int("Array Custom3", 9i64), EnumeratorShape::new_int("Array Bones", 10i64), EnumeratorShape::new_int("Array Weights", 11i64), EnumeratorShape::new_int("Array Index", 12i64), EnumeratorShape::new_int("Array Max", 13i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Mesh.ArrayType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ArrayType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayType {
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
impl crate::registry::property::Export for ArrayType {
    
}
impl crate::meta::Element for ArrayType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayCustomFormat {
    ord: i32
}
impl ArrayCustomFormat {
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_HALF`"]
    pub const RG_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_MAX`"]
    pub const MAX: ArrayCustomFormat = ArrayCustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ArrayCustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayCustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayCustomFormat::RGBA8_UNORM, ArrayCustomFormat::RGBA8_SNORM, ArrayCustomFormat::RG_HALF, ArrayCustomFormat::RGBA_HALF, ArrayCustomFormat::R_FLOAT, ArrayCustomFormat::RG_FLOAT, ArrayCustomFormat::RGB_FLOAT, ArrayCustomFormat::RGBA_FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayCustomFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA8_UNORM", "ARRAY_CUSTOM_RGBA8_UNORM", ArrayCustomFormat::RGBA8_UNORM), crate::meta::inspect::EnumConstant::new("RGBA8_SNORM", "ARRAY_CUSTOM_RGBA8_SNORM", ArrayCustomFormat::RGBA8_SNORM), crate::meta::inspect::EnumConstant::new("RG_HALF", "ARRAY_CUSTOM_RG_HALF", ArrayCustomFormat::RG_HALF), crate::meta::inspect::EnumConstant::new("RGBA_HALF", "ARRAY_CUSTOM_RGBA_HALF", ArrayCustomFormat::RGBA_HALF), crate::meta::inspect::EnumConstant::new("R_FLOAT", "ARRAY_CUSTOM_R_FLOAT", ArrayCustomFormat::R_FLOAT), crate::meta::inspect::EnumConstant::new("RG_FLOAT", "ARRAY_CUSTOM_RG_FLOAT", ArrayCustomFormat::RG_FLOAT), crate::meta::inspect::EnumConstant::new("RGB_FLOAT", "ARRAY_CUSTOM_RGB_FLOAT", ArrayCustomFormat::RGB_FLOAT), crate::meta::inspect::EnumConstant::new("RGBA_FLOAT", "ARRAY_CUSTOM_RGBA_FLOAT", ArrayCustomFormat::RGBA_FLOAT), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_CUSTOM_MAX", ArrayCustomFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayCustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for ArrayCustomFormat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Custom Rgba8 Unorm", 0i64), EnumeratorShape::new_int("Array Custom Rgba8 Snorm", 1i64), EnumeratorShape::new_int("Array Custom Rg Half", 2i64), EnumeratorShape::new_int("Array Custom Rgba Half", 3i64), EnumeratorShape::new_int("Array Custom R Float", 4i64), EnumeratorShape::new_int("Array Custom Rg Float", 5i64), EnumeratorShape::new_int("Array Custom Rgb Float", 6i64), EnumeratorShape::new_int("Array Custom Rgba Float", 7i64), EnumeratorShape::new_int("Array Custom Max", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Mesh.ArrayCustomFormat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ArrayCustomFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayCustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayCustomFormat {
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
impl crate::registry::property::Export for ArrayCustomFormat {
    
}
impl crate::meta::Element for ArrayCustomFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ArrayFormat {
    ord: u64
}
impl ArrayFormat {
    #[doc(alias = "ARRAY_FORMAT_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_VERTEX`"]
    pub const VERTEX: ArrayFormat = ArrayFormat {
        ord: 1u64
    };
    #[doc(alias = "ARRAY_FORMAT_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_NORMAL`"]
    pub const NORMAL: ArrayFormat = ArrayFormat {
        ord: 2u64
    };
    #[doc(alias = "ARRAY_FORMAT_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TANGENT`"]
    pub const TANGENT: ArrayFormat = ArrayFormat {
        ord: 4u64
    };
    #[doc(alias = "ARRAY_FORMAT_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_COLOR`"]
    pub const COLOR: ArrayFormat = ArrayFormat {
        ord: 8u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV`"]
    pub const TEX_UV: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV2`"]
    pub const TEX_UV2: ArrayFormat = ArrayFormat {
        ord: 32u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0`"]
    pub const CUSTOM0: ArrayFormat = ArrayFormat {
        ord: 64u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1`"]
    pub const CUSTOM1: ArrayFormat = ArrayFormat {
        ord: 128u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2`"]
    pub const CUSTOM2: ArrayFormat = ArrayFormat {
        ord: 256u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3`"]
    pub const CUSTOM3: ArrayFormat = ArrayFormat {
        ord: 512u64
    };
    #[doc(alias = "ARRAY_FORMAT_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BONES`"]
    pub const BONES: ArrayFormat = ArrayFormat {
        ord: 1024u64
    };
    #[doc(alias = "ARRAY_FORMAT_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_WEIGHTS`"]
    pub const WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 2048u64
    };
    #[doc(alias = "ARRAY_FORMAT_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_INDEX`"]
    pub const INDEX: ArrayFormat = ArrayFormat {
        ord: 4096u64
    };
    #[doc(alias = "ARRAY_FORMAT_BLEND_SHAPE_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BLEND_SHAPE_MASK`"]
    pub const BLEND_SHAPE_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BASE`"]
    pub const CUSTOM_BASE: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BITS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BITS`"]
    pub const CUSTOM_BITS: ArrayFormat = ArrayFormat {
        ord: 3u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0_SHIFT`"]
    pub const CUSTOM0_SHIFT: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1_SHIFT`"]
    pub const CUSTOM1_SHIFT: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2_SHIFT`"]
    pub const CUSTOM2_SHIFT: ArrayFormat = ArrayFormat {
        ord: 19u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3_SHIFT`"]
    pub const CUSTOM3_SHIFT: ArrayFormat = ArrayFormat {
        ord: 22u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_MASK`"]
    pub const CUSTOM_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_COMPRESS_FLAGS_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_COMPRESS_FLAGS_BASE`"]
    pub const COMPRESS_FLAGS_BASE: ArrayFormat = ArrayFormat {
        ord: 25u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_2D_VERTICES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_2D_VERTICES`"]
    pub const FLAG_USE_2D_VERTICES: ArrayFormat = ArrayFormat {
        ord: 33554432u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_DYNAMIC_UPDATE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_DYNAMIC_UPDATE`"]
    pub const FLAG_USE_DYNAMIC_UPDATE: ArrayFormat = ArrayFormat {
        ord: 67108864u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_8_BONE_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_8_BONE_WEIGHTS`"]
    pub const FLAG_USE_8_BONE_WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 134217728u64
    };
    #[doc(alias = "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY`"]
    pub const FLAG_USES_EMPTY_VERTEX_ARRAY: ArrayFormat = ArrayFormat {
        ord: 268435456u64
    };
    #[doc(alias = "ARRAY_FLAG_COMPRESS_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_COMPRESS_ATTRIBUTES`"]
    pub const FLAG_COMPRESS_ATTRIBUTES: ArrayFormat = ArrayFormat {
        ord: 536870912u64
    };
    
}
impl std::fmt::Debug for ArrayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ArrayFormat {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_FORMAT_VERTEX", ArrayFormat::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_FORMAT_NORMAL", ArrayFormat::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_FORMAT_TANGENT", ArrayFormat::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_FORMAT_COLOR", ArrayFormat::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_FORMAT_TEX_UV", ArrayFormat::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_FORMAT_TEX_UV2", ArrayFormat::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_FORMAT_CUSTOM0", ArrayFormat::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_FORMAT_CUSTOM1", ArrayFormat::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_FORMAT_CUSTOM2", ArrayFormat::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_FORMAT_CUSTOM3", ArrayFormat::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_FORMAT_BONES", ArrayFormat::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_FORMAT_WEIGHTS", ArrayFormat::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_FORMAT_INDEX", ArrayFormat::INDEX), crate::meta::inspect::EnumConstant::new("BLEND_SHAPE_MASK", "ARRAY_FORMAT_BLEND_SHAPE_MASK", ArrayFormat::BLEND_SHAPE_MASK), crate::meta::inspect::EnumConstant::new("CUSTOM_BASE", "ARRAY_FORMAT_CUSTOM_BASE", ArrayFormat::CUSTOM_BASE), crate::meta::inspect::EnumConstant::new("CUSTOM_BITS", "ARRAY_FORMAT_CUSTOM_BITS", ArrayFormat::CUSTOM_BITS), crate::meta::inspect::EnumConstant::new("CUSTOM0_SHIFT", "ARRAY_FORMAT_CUSTOM0_SHIFT", ArrayFormat::CUSTOM0_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM1_SHIFT", "ARRAY_FORMAT_CUSTOM1_SHIFT", ArrayFormat::CUSTOM1_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM2_SHIFT", "ARRAY_FORMAT_CUSTOM2_SHIFT", ArrayFormat::CUSTOM2_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM3_SHIFT", "ARRAY_FORMAT_CUSTOM3_SHIFT", ArrayFormat::CUSTOM3_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM_MASK", "ARRAY_FORMAT_CUSTOM_MASK", ArrayFormat::CUSTOM_MASK), crate::meta::inspect::EnumConstant::new("COMPRESS_FLAGS_BASE", "ARRAY_COMPRESS_FLAGS_BASE", ArrayFormat::COMPRESS_FLAGS_BASE), crate::meta::inspect::EnumConstant::new("FLAG_USE_2D_VERTICES", "ARRAY_FLAG_USE_2D_VERTICES", ArrayFormat::FLAG_USE_2D_VERTICES), crate::meta::inspect::EnumConstant::new("FLAG_USE_DYNAMIC_UPDATE", "ARRAY_FLAG_USE_DYNAMIC_UPDATE", ArrayFormat::FLAG_USE_DYNAMIC_UPDATE), crate::meta::inspect::EnumConstant::new("FLAG_USE_8_BONE_WEIGHTS", "ARRAY_FLAG_USE_8_BONE_WEIGHTS", ArrayFormat::FLAG_USE_8_BONE_WEIGHTS), crate::meta::inspect::EnumConstant::new("FLAG_USES_EMPTY_VERTEX_ARRAY", "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY", ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY), crate::meta::inspect::EnumConstant::new("FLAG_COMPRESS_ATTRIBUTES", "ARRAY_FLAG_COMPRESS_ATTRIBUTES", ArrayFormat::FLAG_COMPRESS_ATTRIBUTES)]
        }
    }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ArrayFormat {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ArrayFormat {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Format Vertex", 1i64), EnumeratorShape::new_int("Array Format Normal", 2i64), EnumeratorShape::new_int("Array Format Tangent", 4i64), EnumeratorShape::new_int("Array Format Color", 8i64), EnumeratorShape::new_int("Array Format Tex Uv", 16i64), EnumeratorShape::new_int("Array Format Tex Uv2", 32i64), EnumeratorShape::new_int("Array Format Custom0", 64i64), EnumeratorShape::new_int("Array Format Custom1", 128i64), EnumeratorShape::new_int("Array Format Custom2", 256i64), EnumeratorShape::new_int("Array Format Custom3", 512i64), EnumeratorShape::new_int("Array Format Bones", 1024i64), EnumeratorShape::new_int("Array Format Weights", 2048i64), EnumeratorShape::new_int("Array Format Index", 4096i64), EnumeratorShape::new_int("Array Format Blend Shape Mask", 7i64), EnumeratorShape::new_int("Array Format Custom Base", 13i64), EnumeratorShape::new_int("Array Format Custom Bits", 3i64), EnumeratorShape::new_int("Array Format Custom0 Shift", 13i64), EnumeratorShape::new_int("Array Format Custom1 Shift", 16i64), EnumeratorShape::new_int("Array Format Custom2 Shift", 19i64), EnumeratorShape::new_int("Array Format Custom3 Shift", 22i64), EnumeratorShape::new_int("Array Format Custom Mask", 7i64), EnumeratorShape::new_int("Array Compress Flags Base", 25i64), EnumeratorShape::new_int("Array Flag Use 2d Vertices", 33554432i64), EnumeratorShape::new_int("Array Flag Use Dynamic Update", 67108864i64), EnumeratorShape::new_int("Array Flag Use 8 Bone Weights", 134217728i64), EnumeratorShape::new_int("Array Flag Uses Empty Vertex Array", 268435456i64), EnumeratorShape::new_int("Array Flag Compress Attributes", 536870912i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Mesh.ArrayFormat")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ArrayFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayFormat {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
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
impl crate::registry::property::Export for ArrayFormat {
    
}
impl crate::meta::Element for ArrayFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendShapeMode {
    ord: i32
}
impl BlendShapeMode {
    #[doc(alias = "BLEND_SHAPE_MODE_NORMALIZED")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_NORMALIZED`"]
    pub const NORMALIZED: BlendShapeMode = BlendShapeMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_SHAPE_MODE_RELATIVE")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_RELATIVE`"]
    pub const RELATIVE: BlendShapeMode = BlendShapeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BlendShapeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendShapeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendShapeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::NORMALIZED => "NORMALIZED", Self::RELATIVE => "RELATIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendShapeMode::NORMALIZED, BlendShapeMode::RELATIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendShapeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMALIZED", "BLEND_SHAPE_MODE_NORMALIZED", BlendShapeMode::NORMALIZED), crate::meta::inspect::EnumConstant::new("RELATIVE", "BLEND_SHAPE_MODE_RELATIVE", BlendShapeMode::RELATIVE)]
        }
    }
}
impl crate::meta::GodotConvert for BlendShapeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Blend Shape Mode Normalized", 0i64), EnumeratorShape::new_int("Blend Shape Mode Relative", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Mesh.BlendShapeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BlendShapeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendShapeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BlendShapeMode {
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
impl crate::registry::property::Export for BlendShapeMode {
    
}
impl crate::meta::Element for BlendShapeMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Mesh;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Mesh {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}