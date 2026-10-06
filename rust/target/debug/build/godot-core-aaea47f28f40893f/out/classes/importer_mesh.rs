#![doc = "Sidecar module for class [`ImporterMesh`][crate::classes::ImporterMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImporterMesh` enums](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImporterMesh`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`importer_mesh`][crate::classes::importer_mesh]: sidecar module with related enum/flag types\n* [`IImporterMesh`][crate::classes::IImporterMesh]: virtual methods\n\n\nSee also [Godot docs for `ImporterMesh`](https://docs.godotengine.org/en/stable/classes/class_importermesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ImporterMesh::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nImporterMesh is a type of [`Resource`][crate::classes::Resource] analogous to [`ArrayMesh`][crate::classes::ArrayMesh]. It contains vertex array-based geometry, divided in _surfaces_. Each surface contains a completely separate array and a material used to draw it. Design wise, a mesh with multiple surfaces is preferred to a single surface, because objects created in 3D editing software commonly contain multiple materials.\n\nUnlike its runtime counterpart, `ImporterMesh` contains mesh data before various import steps, such as lod and shadow mesh generation, have taken place. Modify surface data by calling [`clear`][`crate::classes::ImporterMesh::clear`], followed by [`add_surface`][`crate::classes::ImporterMesh::add_surface`] for each surface."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImporterMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ImporterMesh`][crate::classes::ImporterMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ImporterMesh` methods](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImporterMesh: crate::obj::GodotClass < Base = ImporterMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ImporterMesh {
        #[doc = "Adds name for a blend shape that will be added with [`add_surface`][`crate::classes::ImporterMesh::add_surface`]. Must be called before surface is added."]
        pub fn add_blend_shape(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "add_blend_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of blend shapes that the mesh holds."]
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the blend shape at this index."]
        pub fn get_blend_shape_name(&self, blend_shape_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the blend shape mode."]
        pub fn set_blend_shape_mode(&mut self, mode: crate::classes::mesh::BlendShapeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::mesh::BlendShapeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the blend shape mode for this Mesh."]
        pub fn get_blend_shape_mode(&self,) -> crate::classes::mesh::BlendShapeMode {
            type CallRet = crate::classes::mesh::BlendShapeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        pub(crate) fn add_surface_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: RefArg < AnyArray >, blend_shapes: RefArg < Array < AnyArray > >, lods: RefArg < AnyDictionary >, material: CowArg < Option < Gd < crate::classes::Material > > >, name: CowArg < GString >, flags: u64,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (crate::classes::mesh::PrimitiveType, RefArg < 'a0, AnyArray >, RefArg < 'a1, Array < AnyArray > >, RefArg < 'a2, AnyDictionary >, CowArg < 'a3, Option < Gd < crate::classes::Material > > >, CowArg < 'a4, GString >, u64,);
            let args = (primitive, arrays, blend_shapes, lods, material, name, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "add_surface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_surface_ex`][Self::add_surface_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        #[inline]
        pub fn add_surface(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &AnyArray,) {
            self.add_surface_ex(primitive, arrays,) . done()
        }
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        #[inline]
        pub fn add_surface_ex < 'ex > (&'ex mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &'ex AnyArray,) -> ExAddSurface < 'ex > {
            ExAddSurface::new(self, primitive, arrays,)
        }
        #[doc = "Returns the number of surfaces that the mesh holds."]
        pub fn get_surface_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the primitive type of the requested surface (see [`add_surface`][`crate::classes::ImporterMesh::add_surface`])."]
        pub fn get_surface_primitive_type(&self, surface_idx: i32,) -> crate::classes::mesh::PrimitiveType {
            type CallRet = crate::classes::mesh::PrimitiveType;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_primitive_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the name assigned to this surface."]
        pub fn get_surface_name(&self, surface_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the arrays for the vertices, normals, UVs, etc. that make up the requested surface. See [`add_surface`][`crate::classes::ImporterMesh::add_surface`]."]
        pub fn get_surface_arrays(&self, surface_idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a single set of blend shape arrays for the requested blend shape index for a surface."]
        pub fn get_surface_blend_shape_arrays(&self, surface_idx: i32, blend_shape_idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, i32,);
            let args = (surface_idx, blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_blend_shape_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of lods that the mesh holds on a given surface."]
        pub fn get_surface_lod_count(&self, surface_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the screen ratio which activates a lod for a surface."]
        pub fn get_surface_lod_size(&self, surface_idx: i32, lod_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index buffer of a lod for a surface."]
        pub fn get_surface_lod_indices(&self, surface_idx: i32, lod_idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32, i32,);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_lod_indices", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Material`][crate::classes::Material] in a given surface. Surface is rendered using this material."]
        pub fn get_surface_material(&self, surface_idx: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the format of the surface that the mesh holds."]
        pub fn get_surface_format(&self, surface_idx: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_surface_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a name for a given surface."]
        pub fn set_surface_name(&mut self, surface_idx: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (surface_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_surface_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a [`Material`][crate::classes::Material] for a given surface. Surface will be rendered using this material."]
        pub fn set_surface_material(&mut self, surface_idx: i32, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (surface_idx, material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_surface_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates all lods for this ImporterMesh.\n\n`normal_merge_angle` is in degrees and used in the same way as the importer settings in `lods`.\n\n`normal_split_angle` is not used and only remains for compatibility with older versions of the API.\n\nThe number of generated lods can be accessed using [`get_surface_lod_count`][`crate::classes::ImporterMesh::get_surface_lod_count`], and each LOD is available in [`get_surface_lod_size`][`crate::classes::ImporterMesh::get_surface_lod_size`] and [`get_surface_lod_indices`][`crate::classes::ImporterMesh::get_surface_lod_indices`].\n\n`bone_transform_array` is an [`Array`][crate::builtin::Array] which can be either empty or contain [`Transform3D`][crate::builtin::Transform3D]s which, for each of the mesh's bone IDs, will apply mesh skinning when generating the LOD mesh variations. This is usually used to account for discrepancies in scale between the mesh itself and its skinning data."]
        pub fn generate_lods(&mut self, normal_merge_angle: f32, normal_split_angle: f32, bone_transform_array: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (f32, f32, RefArg < 'a0, AnyArray >,);
            let args = (normal_merge_angle, normal_split_angle, RefArg::new(bone_transform_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "generate_lods", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the mesh data represented by this `ImporterMesh` as a usable [`ArrayMesh`][crate::classes::ArrayMesh].\n\nThis method caches the returned mesh, and subsequent calls will return the cached data until [`clear`][`crate::classes::ImporterMesh::clear`] is called.\n\nIf not yet cached and `base_mesh` is provided, `base_mesh` will be used and mutated."]
        pub(crate) fn get_mesh_full(&self, base_mesh: CowArg < Option < Gd < crate::classes::ArrayMesh > > >,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >,);
            let args = (base_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_mesh_ex`][Self::get_mesh_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the mesh data represented by this `ImporterMesh` as a usable [`ArrayMesh`][crate::classes::ArrayMesh].\n\nThis method caches the returned mesh, and subsequent calls will return the cached data until [`clear`][`crate::classes::ImporterMesh::clear`] is called.\n\nIf not yet cached and `base_mesh` is provided, `base_mesh` will be used and mutated."]
        #[inline]
        pub fn get_mesh(&self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.get_mesh_ex() . done()
        }
        #[doc = "Returns the mesh data represented by this `ImporterMesh` as a usable [`ArrayMesh`][crate::classes::ArrayMesh].\n\nThis method caches the returned mesh, and subsequent calls will return the cached data until [`clear`][`crate::classes::ImporterMesh::clear`] is called.\n\nIf not yet cached and `base_mesh` is provided, `base_mesh` will be used and mutated."]
        #[inline]
        pub fn get_mesh_ex < 'ex > (&'ex self,) -> ExGetMesh < 'ex > {
            ExGetMesh::new(self,)
        }
        #[doc = "Converts the given [`Mesh`][crate::classes::Mesh] into an `ImporterMesh` by copying all its surfaces, blend shapes, materials, and metadata into a new `ImporterMesh` object."]
        pub fn from_mesh(mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) -> Option < Gd < crate::classes::ImporterMesh > > {
            type CallRet = Option < Gd < crate::classes::ImporterMesh > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "from_mesh", None, args,)
            }
        }
        #[doc = "Removes all surfaces and blend shapes from this `ImporterMesh`."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the size hint of this mesh for lightmap-unwrapping in UV-space."]
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "set_lightmap_size_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the size hint of this mesh for lightmap-unwrapping in UV-space."]
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImporterMesh", "get_lightmap_size_hint", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ImporterMesh {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ImporterMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImporterMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ImporterMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImporterMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ImporterMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImporterMesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImporterMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ImporterMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ImporterMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ImporterMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ImporterMesh::add_surface_ex`][super::ImporterMesh::add_surface_ex]."]
#[must_use]
pub struct ExAddSurface < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ImporterMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: CowArg < 'ex, AnyArray >, blend_shapes: CowArg < 'ex, Array < AnyArray > >, lods: CowArg < 'ex, AnyDictionary >, material: CowArg < 'ex, Option < Gd < crate::classes::Material > > >, name: CowArg < 'ex, GString >, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSurface < 'ex > {
    fn new(surround_object: &'ex mut re_export::ImporterMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: &'ex AnyArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = AnyDictionary::new_untyped();
        let material = Gd::null_arg();
        let name = GString::from("");
        let flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), material: material.into_arg(), name: CowArg::Owned(name), flags: flags,
        }
    }
    #[inline]
    pub fn blend_shapes(self, blend_shapes: &'ex Array < AnyArray >) -> Self {
        Self {
            blend_shapes: CowArg::Borrowed(blend_shapes), .. self
        }
    }
    #[inline]
    pub fn lods(self, lods: &'ex AnyDictionary) -> Self {
        Self {
            lods: CowArg::Borrowed(lods), .. self
        }
    }
    #[inline]
    pub fn material(self, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'ex) -> Self {
        Self {
            material: material.into_arg(), .. self
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'ex) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: u64) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, arrays, blend_shapes, lods, material, name, flags,
        }
        = self;
        re_export::ImporterMesh::add_surface_full(surround_object, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), material, name, flags,)
    }
}
#[doc = "Default-param extender for [`ImporterMesh::get_mesh_ex`][super::ImporterMesh::get_mesh_ex]."]
#[must_use]
pub struct ExGetMesh < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::ImporterMesh, base_mesh: CowArg < 'ex, Option < Gd < crate::classes::ArrayMesh > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetMesh < 'ex > {
    fn new(surround_object: &'ex re_export::ImporterMesh,) -> Self {
        let base_mesh = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, base_mesh: base_mesh.into_arg(),
        }
    }
    #[inline]
    pub fn base_mesh(self, base_mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'ex) -> Self {
        Self {
            base_mesh: base_mesh.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, base_mesh,
        }
        = self;
        re_export::ImporterMesh::get_mesh_full(surround_object, base_mesh,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ImporterMesh;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ImporterMesh {
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