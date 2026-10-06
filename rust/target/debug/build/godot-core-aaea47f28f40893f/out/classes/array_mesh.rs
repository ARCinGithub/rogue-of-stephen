#![doc = "Sidecar module for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayMesh` enums](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ArrayMesh`.\n\nInherits [`Mesh`][crate::classes::Mesh].\n\nRelated symbols:\n\n* [`array_mesh`][crate::classes::array_mesh]: sidecar module with related enum/flag types\n* [`IArrayMesh`][crate::classes::IArrayMesh]: virtual methods\n\n\nSee also [Godot docs for `ArrayMesh`](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ArrayMesh::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThe `ArrayMesh` is used to construct a [`Mesh`][crate::classes::Mesh] by specifying the attributes as arrays.\n\nThe most basic example is the creation of a single triangle:\n\n\n```gdscript\nvar vertices = PackedVector3Array()\nvertices.push_back(Vector3(0, 1, 0))\nvertices.push_back(Vector3(1, 0, 0))\nvertices.push_back(Vector3(0, 0, 1))\n\n# Initialize the ArrayMesh.\nvar arr_mesh = ArrayMesh.new()\nvar arrays = []\narrays.resize(Mesh.ARRAY_MAX)\narrays[Mesh.ARRAY_VERTEX] = vertices\n\n# Create the Mesh.\narr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)\nvar m = MeshInstance3D.new()\nm.mesh = arr_mesh\n```\n\n\nThe [`MeshInstance3D`][crate::classes::MeshInstance3D] is ready to be added to the [`SceneTree`][crate::classes::SceneTree] to be shown.\n\nSee also [`ImmediateMesh`][crate::classes::ImmediateMesh], [`MeshDataTool`][crate::classes::MeshDataTool] and [`SurfaceTool`][crate::classes::SurfaceTool] for procedural geometry generation.\n\n**Note:** Godot uses clockwise [winding order](https://learnopengl.com/Advanced-OpenGL/Face-culling) for front faces of triangle primitive modes."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ArrayMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ArrayMesh` methods](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArrayMesh: crate::obj::GodotClass < Base = ArrayMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ArrayMesh {
        #[doc = "Adds name for a blend shape that will be added with [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`]. Must be called before surface is added."]
        pub fn add_blend_shape(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_blend_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of blend shapes that the `ArrayMesh` holds."]
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the blend shape at this index."]
        pub fn get_blend_shape_name(&self, index: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the blend shape at this index."]
        pub fn set_blend_shape_name(&mut self, index: i32, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all blend shapes from this `ArrayMesh`."]
        pub fn clear_blend_shapes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_blend_shapes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::classes::mesh::BlendShapeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::mesh::BlendShapeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::classes::mesh::BlendShapeMode {
            type CallRet = crate::classes::mesh::BlendShapeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        pub(crate) fn add_surface_from_arrays_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: RefArg < AnyArray >, blend_shapes: RefArg < Array < AnyArray > >, lods: RefArg < AnyDictionary >, flags: crate::classes::mesh::ArrayFormat,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::mesh::PrimitiveType, RefArg < 'a0, AnyArray >, RefArg < 'a1, Array < AnyArray > >, RefArg < 'a2, AnyDictionary >, crate::classes::mesh::ArrayFormat,);
            let args = (primitive, arrays, blend_shapes, lods, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_surface_from_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_surface_from_arrays_ex`][Self::add_surface_from_arrays_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        #[inline]
        pub fn add_surface_from_arrays(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &AnyArray,) {
            self.add_surface_from_arrays_ex(primitive, arrays,) . done()
        }
        #[doc = "Creates a new surface. [`get_surface_count`][`crate::classes::Mesh::get_surface_count`] will become the `surf_idx` for this new surface.\n\nSurfaces are created to be rendered using a `primitive`, which may be any of the values defined in \\[enum Mesh.PrimitiveType].\n\nThe `arrays` argument is an array of arrays. Each of the [`ArrayType::MAX`][`crate::classes::mesh::ArrayType::MAX`] elements contains an array with some of the mesh data for this surface as described by the corresponding member of \\[enum Mesh.ArrayType] or `null` if it is not used by the surface. For example, `arrays[0]` is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into \"index mode\" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] if it is used.\n\nThe `blend_shapes` argument is an array of vertex data for each blend shape. Each element is an array of the same structure as `arrays`, but [`ArrayType::VERTEX`][`crate::classes::mesh::ArrayType::VERTEX`], [`ArrayType::NORMAL`][`crate::classes::mesh::ArrayType::NORMAL`], and [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`] are set if and only if they are set in `arrays` and all other entries are `null`.\n\nThe `lods` argument is a dictionary with `float` keys and [`PackedInt32Array`][crate::builtin::PackedInt32Array] values. Each entry in the dictionary represents an LOD level of the surface, where the value is the [`ArrayType::INDEX`][`crate::classes::mesh::ArrayType::INDEX`] array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.\n\nThe `flags` argument is the bitwise OR of, as required: One value of \\[enum Mesh.ArrayCustomFormat] left shifted by `ARRAY_FORMAT_CUSTOMn_SHIFT` for each custom channel in use, [`ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`][`crate::classes::mesh::ArrayFormat::FLAG_USE_DYNAMIC_UPDATE`], [`ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`][`crate::classes::mesh::ArrayFormat::FLAG_USE_8_BONE_WEIGHTS`], or [`ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`][`crate::classes::mesh::ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY`].\n\n**Note:** When using indices, it is recommended to only use points, lines, or triangles."]
        #[inline]
        pub fn add_surface_from_arrays_ex < 'ex > (&'ex mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &'ex AnyArray,) -> ExAddSurfaceFromArrays < 'ex > {
            ExAddSurfaceFromArrays::new(self, primitive, arrays,)
        }
        #[doc = "Removes all surfaces from this `ArrayMesh`."]
        pub fn clear_surfaces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_surfaces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the surface at the given index from the Mesh, shifting surfaces with higher index down by one."]
        pub fn surface_remove(&mut self, surf_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_remove", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_vertex_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_vertex_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_attribute_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_attribute_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_skin_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_skin_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the length in vertices of the vertex array in the requested surface (see [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`])."]
        pub fn surface_get_array_len(&self, surf_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_len", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the length in indices of the index array in the requested surface (see [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`])."]
        pub fn surface_get_array_index_len(&self, surf_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_index_len", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the format mask of the requested surface (see [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`])."]
        pub fn surface_get_format(&self, surf_idx: i32,) -> crate::classes::mesh::ArrayFormat {
            type CallRet = crate::classes::mesh::ArrayFormat;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the primitive type of the requested surface (see [`add_surface_from_arrays`][`crate::classes::ArrayMesh::add_surface_from_arrays`])."]
        pub fn surface_get_primitive_type(&self, surf_idx: i32,) -> crate::classes::mesh::PrimitiveType {
            type CallRet = crate::classes::mesh::PrimitiveType;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_primitive_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the first surface with this name held within this `ArrayMesh`. If none are found, -1 is returned."]
        pub fn surface_find_by_name(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_find_by_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a name for a given surface."]
        pub fn surface_set_name(&mut self, surf_idx: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (surf_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_set_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the name assigned to this surface."]
        pub fn surface_get_name(&self, surf_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Regenerates tangents for each of the `ArrayMesh`'s surfaces."]
        pub fn regen_normal_maps(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "regen_normal_maps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Performs a UV unwrap on the `ArrayMesh` to prepare the mesh for lightmapping."]
        pub fn lightmap_unwrap(&mut self, transform: Transform3D, texel_size: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Transform3D, f32,);
            let args = (transform, texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "lightmap_unwrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_shadow_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_mesh(&self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_shadow_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ArrayMesh {
        type Base = crate::classes::Mesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ArrayMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ArrayMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ArrayMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ArrayMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ArrayMesh {
        type Target = crate::classes::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ArrayMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ArrayMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ArrayMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ArrayMesh > for $Class {
                
            }
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
#[doc = "Default-param extender for [`ArrayMesh::add_surface_from_arrays_ex`][super::ArrayMesh::add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExAddSurfaceFromArrays < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: CowArg < 'ex, AnyArray >, blend_shapes: CowArg < 'ex, Array < AnyArray > >, lods: CowArg < 'ex, AnyDictionary >, flags: crate::classes::mesh::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSurfaceFromArrays < 'ex > {
    fn new(surround_object: &'ex mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: &'ex AnyArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = AnyDictionary::new_untyped();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), flags: flags,
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
    pub fn flags(self, flags: crate::classes::mesh::ArrayFormat) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, arrays, blend_shapes, lods, flags,
        }
        = self;
        re_export::ArrayMesh::add_surface_from_arrays_full(surround_object, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ArrayMesh;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ArrayMesh {
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