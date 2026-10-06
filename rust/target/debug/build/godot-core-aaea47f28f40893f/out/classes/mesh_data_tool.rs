#![doc = "Sidecar module for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshDataTool` enums](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshDataTool`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`mesh_data_tool`][crate::classes::mesh_data_tool]: sidecar module with related enum/flag types\n* [`IMeshDataTool`][crate::classes::IMeshDataTool]: virtual methods\n\n\nSee also [Godot docs for `MeshDataTool`](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MeshDataTool::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nMeshDataTool provides access to individual vertices in a [`Mesh`][crate::classes::Mesh]. It allows users to read and edit vertex data of meshes. It also creates an array of faces and edges.\n\nTo use MeshDataTool, load a mesh with [`create_from_surface`][`crate::classes::MeshDataTool::create_from_surface`]. When you are finished editing the data commit the data to a mesh with [`commit_to_surface`][`crate::classes::MeshDataTool::commit_to_surface`].\n\nBelow is an example of how MeshDataTool may be used.\n\n\n```gdscript\nvar mesh = ArrayMesh.new()\nmesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, BoxMesh.new().get_mesh_arrays())\nvar mdt = MeshDataTool.new()\nmdt.create_from_surface(mesh, 0)\nfor i in range(mdt.get_vertex_count()):\n\tvar vertex = mdt.get_vertex(i)\n\t# In this example we extend the mesh by one unit, which results in separated faces as it is flat shaded.\n\tvertex += mdt.get_vertex_normal(i)\n\t# Save your change.\n\tmdt.set_vertex(i, vertex)\nmesh.clear_surfaces()\nmdt.commit_to_surface(mesh)\nvar mi = MeshInstance.new()\nmi.mesh = mesh\nadd_child(mi)\n```\n\n\nSee also [`ArrayMesh`][crate::classes::ArrayMesh], [`ImmediateMesh`][crate::classes::ImmediateMesh] and [`SurfaceTool`][crate::classes::SurfaceTool] for procedural geometry generation.\n\n**Note:** Godot uses clockwise [winding order](https://learnopengl.com/Advanced-OpenGL/Face-culling) for front faces of triangle primitive modes."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshDataTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MeshDataTool` methods](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshDataTool: crate::obj::GodotClass < Base = MeshDataTool > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl MeshDataTool {
        #[doc = "Clears all data currently in MeshDataTool."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Uses specified surface of given [`Mesh`][crate::classes::Mesh] to populate data for MeshDataTool.\n\nRequires [`Mesh`][crate::classes::Mesh] with primitive type [`PrimitiveType::TRIANGLES`][`crate::classes::mesh::PrimitiveType::TRIANGLES`]."]
        pub fn create_from_surface(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >, surface: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >, i32,);
            let args = (mesh.into_arg(), surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "create_from_surface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new surface to specified [`Mesh`][crate::classes::Mesh] with edited data."]
        pub(crate) fn commit_to_surface_full(&mut self, mesh: CowArg < Option < Gd < crate::classes::ArrayMesh > > >, compression_flags: u64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >, u64,);
            let args = (mesh, compression_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "commit_to_surface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`commit_to_surface_ex`][Self::commit_to_surface_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new surface to specified [`Mesh`][crate::classes::Mesh] with edited data."]
        #[inline]
        pub fn commit_to_surface(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >,) -> crate::global::Error {
            self.commit_to_surface_ex(mesh,) . done()
        }
        #[doc = "Adds a new surface to specified [`Mesh`][crate::classes::Mesh] with edited data."]
        #[inline]
        pub fn commit_to_surface_ex < 'ex > (&'ex mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'ex,) -> ExCommitToSurface < 'ex > {
            ExCommitToSurface::new(self, mesh,)
        }
        #[doc = "Returns the [`Mesh`][crate::classes::Mesh]'s format as a combination of the \\[enum Mesh.ArrayFormat] flags. For example, a mesh containing both vertices and normals would return a format of `3` because [`ArrayFormat::VERTEX`][`crate::classes::mesh::ArrayFormat::VERTEX`] is `1` and [`ArrayFormat::NORMAL`][`crate::classes::mesh::ArrayFormat::NORMAL`] is `2`."]
        pub fn get_format(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of vertices in [`Mesh`][crate::classes::Mesh]."]
        pub fn get_vertex_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of edges in this [`Mesh`][crate::classes::Mesh]."]
        pub fn get_edge_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of faces in this [`Mesh`][crate::classes::Mesh]."]
        pub fn get_face_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the position of the given vertex."]
        pub fn set_vertex(&mut self, idx: i32, vertex: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the position of the given vertex."]
        pub fn get_vertex(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the normal of the given vertex."]
        pub fn set_vertex_normal(&mut self, idx: i32, normal: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (idx, normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the normal of the given vertex."]
        pub fn get_vertex_normal(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tangent of the given vertex.\n\n**Note:** Even though `tangent` is a [`Plane`][crate::builtin::Plane], it does not directly represent the tangent plane. Its \\[member Plane.x], \\[member Plane.y], and \\[member Plane.z] represent the tangent vector and \\[member Plane.d] should be either `-1` or `1`. See also [`ArrayType::TANGENT`][`crate::classes::mesh::ArrayType::TANGENT`]."]
        pub fn set_vertex_tangent(&mut self, idx: i32, tangent: Plane,) {
            type CallRet = ();
            type CallParams = (i32, Plane,);
            let args = (idx, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tangent of the given vertex."]
        pub fn get_vertex_tangent(&self, idx: i32,) -> Plane {
            type CallRet = Plane;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_tangent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the UV of the given vertex."]
        pub fn set_vertex_uv(&mut self, idx: i32, uv: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the UV of the given vertex."]
        pub fn get_vertex_uv(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the UV2 of the given vertex."]
        pub fn set_vertex_uv2(&mut self, idx: i32, uv2: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the UV2 of the given vertex."]
        pub fn get_vertex_uv2(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of the given vertex."]
        pub fn set_vertex_color(&mut self, idx: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the given vertex."]
        pub fn get_vertex_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the bones of the given vertex."]
        pub fn set_vertex_bones(&mut self, idx: i32, bones: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedInt32Array >,);
            let args = (idx, RefArg::new(bones),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_bones", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bones of the given vertex."]
        pub fn get_vertex_bones(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_bones", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the bone weights of the given vertex."]
        pub fn set_vertex_weights(&mut self, idx: i32, weights: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedFloat32Array >,);
            let args = (idx, RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_weights", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns bone weights of the given vertex."]
        pub fn get_vertex_weights(&self, idx: i32,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_weights", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata associated with the given vertex."]
        pub fn set_vertex_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata associated with the given vertex."]
        pub fn get_vertex_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of edges that share the given vertex."]
        pub fn get_vertex_edges(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_edges", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of faces that share the given vertex."]
        pub fn get_vertex_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_faces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the specified `vertex` connected to the edge at index `idx`.\n\n`vertex` can only be `0` or `1`, as edges are composed of two vertices."]
        pub fn get_edge_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_vertex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of faces that touch given edge."]
        pub fn get_edge_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_faces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of the given edge."]
        pub fn set_edge_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_edge_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns meta information assigned to given edge."]
        pub fn get_edge_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the specified vertex index of the given face.\n\n`vertex` must be either `0`, `1`, or `2` because faces contain three vertices.\n\n\n```gdscript\nvar index = mesh_data_tool.get_face_vertex(0, 1) # Gets the index of the second vertex of the first face.\nvar position = mesh_data_tool.get_vertex(index)\nvar normal = mesh_data_tool.get_vertex_normal(index)\n```\n"]
        pub fn get_face_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_vertex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the edge associated with the face at index `idx`.\n\n`edge` argument must be either `0`, `1`, or `2` because a face only has three edges."]
        pub fn get_face_edge(&self, idx: i32, edge: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_edge", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of the given face."]
        pub fn set_face_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_face_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata associated with the given face."]
        pub fn get_face_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calculates and returns the face normal of the given face."]
        pub fn get_face_normal(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_normal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the material to be used by newly-constructed [`Mesh`][crate::classes::Mesh]."]
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the material assigned to the [`Mesh`][crate::classes::Mesh]."]
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_material", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MeshDataTool {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MeshDataTool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshDataTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MeshDataTool {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshDataTool {
        
    }
    impl crate::obj::cap::GodotDefault for MeshDataTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshDataTool {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshDataTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshDataTool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MeshDataTool__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshDataTool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MeshDataTool::commit_to_surface_ex`][super::MeshDataTool::commit_to_surface_ex]."]
#[must_use]
pub struct ExCommitToSurface < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MeshDataTool, mesh: CowArg < 'ex, Option < Gd < crate::classes::ArrayMesh > > >, compression_flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCommitToSurface < 'ex > {
    fn new(surround_object: &'ex mut re_export::MeshDataTool, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'ex,) -> Self {
        let compression_flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.into_arg(), compression_flags: compression_flags,
        }
    }
    #[inline]
    pub fn compression_flags(self, compression_flags: u64) -> Self {
        Self {
            compression_flags: compression_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mesh, compression_flags,
        }
        = self;
        re_export::MeshDataTool::commit_to_surface_full(surround_object, mesh, compression_flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MeshDataTool;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for MeshDataTool {
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