#![doc = "Sidecar module for class [`TileSet`][crate::classes::TileSet].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSet` enums](https://docs.godotengine.org/en/stable/classes/class_tileset.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSet`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`tile_set`][crate::classes::tile_set]: sidecar module with related enum/flag types\n* [`ITileSet`][crate::classes::ITileSet]: virtual methods\n\n\nSee also [Godot docs for `TileSet`](https://docs.godotengine.org/en/stable/classes/class_tileset.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSet::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA TileSet is a library of tiles for a [`TileMapLayer`][crate::classes::TileMapLayer]. A TileSet handles a list of [`TileSetSource`][crate::classes::TileSetSource], each of them storing a set of tiles.\n\nTiles can either be from a [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource], which renders tiles out of a texture with support for physics, navigation, etc., or from a [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource], which exposes scene-based tiles.\n\nTiles are referenced by using three IDs: their source ID, their atlas coordinates ID, and their alternative tile ID.\n\nA TileSet can be configured so that its tiles expose more or fewer properties. To do so, the TileSet resources use property layers, which you can add or remove depending on your needs.\n\nFor example, adding a physics layer allows giving collision shapes to your tiles. Each layer has dedicated properties (physics layer and mask), so you may add several TileSet physics layers for each type of collision you need.\n\nSee the functions to add new layers for more information."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSet {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileSet`][crate::classes::TileSet].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TileSet` methods](https://docs.godotengine.org/en/stable/classes/class_tileset.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSet: crate::obj::GodotClass < Base = TileSet > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSet {
        #[doc = "Returns a new unused source ID. This generated ID is the same that a call to [`add_source`][`crate::classes::TileSet::add_source`] would return."]
        pub fn get_next_source_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_next_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a [`TileSetSource`][crate::classes::TileSetSource] to the TileSet. If `atlas_source_id_override` is not -1, also set its source ID. Otherwise, a unique identifier is automatically generated.\n\nThe function returns the added source ID or -1 if the source could not be added.\n\n**Warning:** A source cannot belong to two TileSets at the same time. If the added source was attached to another `TileSet`, it will be removed from that one."]
        pub(crate) fn add_source_full(&mut self, source: CowArg < Option < Gd < crate::classes::TileSetSource > > >, atlas_source_id_override: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileSetSource > > >, i32,);
            let args = (source, atlas_source_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_source_ex`][Self::add_source_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a [`TileSetSource`][crate::classes::TileSetSource] to the TileSet. If `atlas_source_id_override` is not -1, also set its source ID. Otherwise, a unique identifier is automatically generated.\n\nThe function returns the added source ID or -1 if the source could not be added.\n\n**Warning:** A source cannot belong to two TileSets at the same time. If the added source was attached to another `TileSet`, it will be removed from that one."]
        #[inline]
        pub fn add_source(&mut self, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> >,) -> i32 {
            self.add_source_ex(source,) . done()
        }
        #[doc = "Adds a [`TileSetSource`][crate::classes::TileSetSource] to the TileSet. If `atlas_source_id_override` is not -1, also set its source ID. Otherwise, a unique identifier is automatically generated.\n\nThe function returns the added source ID or -1 if the source could not be added.\n\n**Warning:** A source cannot belong to two TileSets at the same time. If the added source was attached to another `TileSet`, it will be removed from that one."]
        #[inline]
        pub fn add_source_ex < 'ex > (&'ex mut self, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> > + 'ex,) -> ExAddSource < 'ex > {
            ExAddSource::new(self, source,)
        }
        #[doc = "Removes the source with the given source ID."]
        pub fn remove_source(&mut self, source_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes a source's ID."]
        pub fn set_source_id(&mut self, source_id: i32, new_source_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (source_id, new_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of [`TileSetSource`][crate::classes::TileSetSource] in this TileSet."]
        pub fn get_source_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the source ID for source with index `index`."]
        pub fn get_source_id(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if this TileSet has a source for the given source ID."]
        pub fn has_source(&self, source_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`TileSetSource`][crate::classes::TileSetSource] with ID `source_id`."]
        pub fn get_source(&self, source_id: i32,) -> Option < Gd < crate::classes::TileSetSource > > {
            type CallRet = Option < Gd < crate::classes::TileSetSource > >;
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_shape(&mut self, shape: crate::classes::tile_set::TileShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_shape(&self,) -> crate::classes::tile_set::TileShape {
            type CallRet = crate::classes::tile_set::TileShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_layout(&mut self, layout: crate::classes::tile_set::TileLayout,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileLayout,);
            let args = (layout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_layout(&self,) -> crate::classes::tile_set::TileLayout {
            type CallRet = crate::classes::tile_set::TileLayout;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_offset_axis(&mut self, alignment: crate::classes::tile_set::TileOffsetAxis,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileOffsetAxis,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_offset_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_offset_axis(&self,) -> crate::classes::tile_set::TileOffsetAxis {
            type CallRet = crate::classes::tile_set::TileOffsetAxis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_offset_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv_clipping(&mut self, uv_clipping: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (uv_clipping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_uv_clipping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uv_clipping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "is_uv_clipping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the occlusion layers count."]
        pub fn get_occlusion_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an occlusion layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nOcclusion layers allow assigning occlusion polygons to atlas tiles."]
        pub(crate) fn add_occlusion_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_occlusion_layer_ex`][Self::add_occlusion_layer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an occlusion layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nOcclusion layers allow assigning occlusion polygons to atlas tiles."]
        #[inline]
        pub fn add_occlusion_layer(&mut self,) {
            self.add_occlusion_layer_ex() . done()
        }
        #[doc = "Adds an occlusion layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nOcclusion layers allow assigning occlusion polygons to atlas tiles."]
        #[inline]
        pub fn add_occlusion_layer_ex < 'ex > (&'ex mut self,) -> ExAddOcclusionLayer < 'ex > {
            ExAddOcclusionLayer::new(self,)
        }
        #[doc = "Moves the occlusion layer at index `layer_index` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_occlusion_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the occlusion layer at index `layer_index`. Also updates the atlas tiles accordingly."]
        pub fn remove_occlusion_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the occlusion layer (as in the rendering server) for occluders in the given TileSet occlusion layer."]
        pub fn set_occlusion_layer_light_mask(&mut self, layer_index: i32, light_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the light mask of the occlusion layer."]
        pub fn get_occlusion_layer_light_mask(&self, layer_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables SDF collision for occluders in the given TileSet occlusion layer."]
        pub fn set_occlusion_layer_sdf_collision(&mut self, layer_index: i32, sdf_collision: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_index, sdf_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if the occluders from this layer use `sdf_collision`."]
        pub fn get_occlusion_layer_sdf_collision(&self, layer_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics layers count."]
        pub fn get_physics_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a physics layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nPhysics layers allow assigning collision polygons to atlas tiles."]
        pub(crate) fn add_physics_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_physics_layer_ex`][Self::add_physics_layer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a physics layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nPhysics layers allow assigning collision polygons to atlas tiles."]
        #[inline]
        pub fn add_physics_layer(&mut self,) {
            self.add_physics_layer_ex() . done()
        }
        #[doc = "Adds a physics layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nPhysics layers allow assigning collision polygons to atlas tiles."]
        #[inline]
        pub fn add_physics_layer_ex < 'ex > (&'ex mut self,) -> ExAddPhysicsLayer < 'ex > {
            ExAddPhysicsLayer::new(self,)
        }
        #[doc = "Moves the physics layer at index `layer_index` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_physics_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the physics layer at index `layer_index`. Also updates the atlas tiles accordingly."]
        pub fn remove_physics_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the collision layer (as in the physics server) for bodies in the given TileSet physics layer."]
        pub fn set_physics_layer_collision_layer(&mut self, layer_index: i32, layer: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision layer (as in the physics server) bodies on the given TileSet's physics layer are in."]
        pub fn get_physics_layer_collision_layer(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the collision mask for bodies in the given TileSet physics layer."]
        pub fn set_physics_layer_collision_mask(&mut self, layer_index: i32, mask: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision mask of bodies on the given TileSet's physics layer."]
        pub fn get_physics_layer_collision_mask(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the collision priority for bodies in the given TileSet physics layer."]
        pub fn set_physics_layer_collision_priority(&mut self, layer_index: i32, priority: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (layer_index, priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the collision priority of bodies on the given TileSet's physics layer."]
        pub fn get_physics_layer_collision_priority(&self, layer_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics material for bodies in the given TileSet physics layer."]
        pub fn set_physics_layer_physics_material(&mut self, layer_index: i32, physics_material: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (layer_index, physics_material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the physics material of bodies on the given TileSet's physics layer."]
        pub fn get_physics_layer_physics_material(&self, layer_index: i32,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the terrain sets count."]
        pub fn get_terrain_sets_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_sets_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new terrain set at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        pub(crate) fn add_terrain_set_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_terrain_set_ex`][Self::add_terrain_set_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new terrain set at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        #[inline]
        pub fn add_terrain_set(&mut self,) {
            self.add_terrain_set_ex() . done()
        }
        #[doc = "Adds a new terrain set at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        #[inline]
        pub fn add_terrain_set_ex < 'ex > (&'ex mut self,) -> ExAddTerrainSet < 'ex > {
            ExAddTerrainSet::new(self,)
        }
        #[doc = "Moves the terrain set at index `terrain_set` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_terrain_set(&mut self, terrain_set: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the terrain set at index `terrain_set`. Also updates the atlas tiles accordingly."]
        pub fn remove_terrain_set(&mut self, terrain_set: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a terrain mode. Each mode determines which bits of a tile shape is used to match the neighboring tiles' terrains."]
        pub fn set_terrain_set_mode(&mut self, terrain_set: i32, mode: crate::classes::tile_set::TerrainMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::tile_set::TerrainMode,);
            let args = (terrain_set, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a terrain set mode."]
        pub fn get_terrain_set_mode(&self, terrain_set: i32,) -> crate::classes::tile_set::TerrainMode {
            type CallRet = crate::classes::tile_set::TerrainMode;
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of terrains in the given terrain set."]
        pub fn get_terrains_count(&self, terrain_set: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrains_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a new terrain to the given terrain set `terrain_set` at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        pub(crate) fn add_terrain_full(&mut self, terrain_set: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_terrain_ex`][Self::add_terrain_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new terrain to the given terrain set `terrain_set` at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        #[inline]
        pub fn add_terrain(&mut self, terrain_set: i32,) {
            self.add_terrain_ex(terrain_set,) . done()
        }
        #[doc = "Adds a new terrain to the given terrain set `terrain_set` at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array."]
        #[inline]
        pub fn add_terrain_ex < 'ex > (&'ex mut self, terrain_set: i32,) -> ExAddTerrain < 'ex > {
            ExAddTerrain::new(self, terrain_set,)
        }
        #[doc = "Moves the terrain at index `terrain_index` for terrain set `terrain_set` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_terrain(&mut self, terrain_set: i32, terrain_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32,);
            let args = (terrain_set, terrain_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the terrain at index `terrain_index` in the given terrain set `terrain_set`. Also updates the atlas tiles accordingly."]
        pub fn remove_terrain(&mut self, terrain_set: i32, terrain_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a terrain's name."]
        pub fn set_terrain_name(&mut self, terrain_set: i32, terrain_index: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (terrain_set, terrain_index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a terrain's name."]
        pub fn get_terrain_name(&self, terrain_set: i32, terrain_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a terrain's color. This color is used for identifying the different terrains in the TileSet editor."]
        pub fn set_terrain_color(&mut self, terrain_set: i32, terrain_index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (terrain_set, terrain_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a terrain's color."]
        pub fn get_terrain_color(&self, terrain_set: i32, terrain_index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the navigation layers count."]
        pub fn get_navigation_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a navigation layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nNavigation layers allow assigning a navigable area to atlas tiles."]
        pub(crate) fn add_navigation_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_navigation_layer_ex`][Self::add_navigation_layer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a navigation layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nNavigation layers allow assigning a navigable area to atlas tiles."]
        #[inline]
        pub fn add_navigation_layer(&mut self,) {
            self.add_navigation_layer_ex() . done()
        }
        #[doc = "Adds a navigation layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nNavigation layers allow assigning a navigable area to atlas tiles."]
        #[inline]
        pub fn add_navigation_layer_ex < 'ex > (&'ex mut self,) -> ExAddNavigationLayer < 'ex > {
            ExAddNavigationLayer::new(self,)
        }
        #[doc = "Moves the navigation layer at index `layer_index` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_navigation_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the navigation layer at index `layer_index`. Also updates the atlas tiles accordingly."]
        pub fn remove_navigation_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the navigation layers (as in the navigation server) for navigation regions in the given TileSet navigation layer."]
        pub fn set_navigation_layer_layers(&mut self, layer_index: i32, layers: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the navigation layers (as in the Navigation server) of the given TileSet navigation layer."]
        pub fn get_navigation_layer_layers(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Based on `value`, enables or disables the specified navigation layer of the TileSet navigation data layer identified by the given `layer_index`, given a navigation_layers `layer_number` between 1 and 32."]
        pub fn set_navigation_layer_layer_value(&mut self, layer_index: i32, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (layer_index, layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether or not the specified navigation layer of the TileSet navigation data layer identified by the given `layer_index` is enabled, given a navigation_layers `layer_number` between 1 and 32."]
        pub fn get_navigation_layer_layer_value(&self, layer_index: i32, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (layer_index, layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom data layers count."]
        pub fn get_custom_data_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a custom data layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nCustom data layers allow assigning custom properties to atlas tiles."]
        pub(crate) fn add_custom_data_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_custom_data_layer_ex`][Self::add_custom_data_layer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a custom data layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nCustom data layers allow assigning custom properties to atlas tiles."]
        #[inline]
        pub fn add_custom_data_layer(&mut self,) {
            self.add_custom_data_layer_ex() . done()
        }
        #[doc = "Adds a custom data layer to the TileSet at the given position `to_position` in the array. If `to_position` is -1, adds it at the end of the array.\n\nCustom data layers allow assigning custom properties to atlas tiles."]
        #[inline]
        pub fn add_custom_data_layer_ex < 'ex > (&'ex mut self,) -> ExAddCustomDataLayer < 'ex > {
            ExAddCustomDataLayer::new(self,)
        }
        #[doc = "Moves the custom data layer at index `layer_index` to the given position `to_position` in the array. Also updates the atlas tiles accordingly."]
        pub fn move_custom_data_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the custom data layer at index `layer_index`. Also updates the atlas tiles accordingly."]
        pub fn remove_custom_data_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the custom data layer identified by the given name."]
        pub fn get_custom_data_layer_by_name(&self, layer_name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_by_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the custom data layer identified by the given index. Names are identifiers of the layer therefore if the name is already taken it will fail and raise an error."]
        pub fn set_custom_data_layer_name(&mut self, layer_index: i32, layer_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (layer_index, layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if there is a custom data layer named `layer_name`."]
        pub fn has_custom_data_layer_by_name(&self, layer_name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_custom_data_layer_by_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the custom data layer identified by the given index."]
        pub fn get_custom_data_layer_name(&self, layer_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of the custom data layer identified by the given index."]
        pub fn set_custom_data_layer_type(&mut self, layer_index: i32, layer_type: VariantType,) {
            type CallRet = ();
            type CallParams = (i32, VariantType,);
            let args = (layer_index, layer_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the custom data layer identified by the given index."]
        pub fn get_custom_data_layer_type(&self, layer_index: i32,) -> VariantType {
            type CallRet = VariantType;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a source-level proxy for the given source ID. A proxy will map set of tile identifiers to another set of identifiers. Both the atlas coordinates ID and the alternative tile ID are kept the same when using source-level proxies.\n\nProxied tiles can be automatically replaced in TileMapLayer nodes using the editor."]
        pub fn set_source_level_tile_proxy(&mut self, source_from: i32, source_to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (source_from, source_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the source-level proxy for the given source identifier.\n\nIf the TileSet has no proxy for the given identifier, returns -1."]
        pub fn get_source_level_tile_proxy(&self, source_from: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if there is a source-level proxy for the given source ID."]
        pub fn has_source_level_tile_proxy(&self, source_from: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a source-level tile proxy."]
        pub fn remove_source_level_tile_proxy(&mut self, source_from: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a coordinates-level proxy for the given identifiers. A proxy will map set of tile identifiers to another set of identifiers. The alternative tile ID is kept the same when using coordinates-level proxies.\n\nProxied tiles can be automatically replaced in TileMapLayer nodes using the editor."]
        pub fn set_coords_level_tile_proxy(&mut self, p_source_from: i32, coords_from: Vector2i, source_to: i32, coords_to: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2i,);
            let args = (p_source_from, coords_from, source_to, coords_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the coordinate-level proxy for the given identifiers. The returned array contains the two target identifiers of the proxy (source ID and atlas coordinates ID).\n\nIf the TileSet has no proxy for the given identifiers, returns an empty Array."]
        pub fn get_coords_level_tile_proxy(&self, source_from: i32, coords_from: Vector2i,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if there is a coodinates-level proxy for the given identifiers."]
        pub fn has_coords_level_tile_proxy(&self, source_from: i32, coords_from: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a coordinates-level proxy for the given identifiers."]
        pub fn remove_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Create an alternative-level proxy for the given identifiers. A proxy will map set of tile identifiers to another set of identifiers.\n\nProxied tiles can be automatically replaced in TileMapLayer nodes using the editor."]
        pub fn set_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32, source_to: i32, coords_to: Vector2i, alternative_to: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from, source_to, coords_to, alternative_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the alternative-level proxy for the given identifiers. The returned array contains the three proxie's target identifiers (source ID, atlas coords ID and alternative tile ID).\n\nIf the TileSet has no proxy for the given identifiers, returns an empty Array."]
        pub fn get_alternative_level_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if there is an alternative-level proxy for the given identifiers."]
        pub fn has_alternative_level_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes an alternative-level proxy for the given identifiers."]
        pub fn remove_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "According to the configured proxies, maps the provided identifiers to a new set of identifiers. The source ID, atlas coordinates ID and alternative tile ID are returned as a 3 elements Array.\n\nThis function first look for matching alternative-level proxies, then coordinates-level proxies, then source-level proxies.\n\nIf no proxy corresponding to provided identifiers are found, returns the same values the ones used as arguments."]
        pub fn map_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "map_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears tile proxies pointing to invalid tiles."]
        pub fn cleanup_invalid_tile_proxies(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "cleanup_invalid_tile_proxies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears all tile proxies."]
        pub fn clear_tile_proxies(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "clear_tile_proxies", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a [`TileMapPattern`][crate::classes::TileMapPattern] to be stored in the TileSet resource. If provided, insert it at the given `index`."]
        pub(crate) fn add_pattern_full(&mut self, pattern: CowArg < Option < Gd < crate::classes::TileMapPattern > > >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >, i32,);
            let args = (pattern, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_pattern_ex`][Self::add_pattern_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a [`TileMapPattern`][crate::classes::TileMapPattern] to be stored in the TileSet resource. If provided, insert it at the given `index`."]
        #[inline]
        pub fn add_pattern(&mut self, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) -> i32 {
            self.add_pattern_ex(pattern,) . done()
        }
        #[doc = "Adds a [`TileMapPattern`][crate::classes::TileMapPattern] to be stored in the TileSet resource. If provided, insert it at the given `index`."]
        #[inline]
        pub fn add_pattern_ex < 'ex > (&'ex mut self, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> > + 'ex,) -> ExAddPattern < 'ex > {
            ExAddPattern::new(self, pattern,)
        }
        #[doc = "Returns the [`TileMapPattern`][crate::classes::TileMapPattern] at the given `index`."]
        pub(crate) fn get_pattern_full(&self, index: i32,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallRet = Option < Gd < crate::classes::TileMapPattern > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_pattern_ex`][Self::get_pattern_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the [`TileMapPattern`][crate::classes::TileMapPattern] at the given `index`."]
        #[inline]
        pub fn get_pattern(&self,) -> Option < Gd < crate::classes::TileMapPattern > > {
            self.get_pattern_ex() . done()
        }
        #[doc = "Returns the [`TileMapPattern`][crate::classes::TileMapPattern] at the given `index`."]
        #[inline]
        pub fn get_pattern_ex < 'ex > (&'ex self,) -> ExGetPattern < 'ex > {
            ExGetPattern::new(self,)
        }
        #[doc = "Remove the [`TileMapPattern`][crate::classes::TileMapPattern] at the given index."]
        pub fn remove_pattern(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of [`TileMapPattern`][crate::classes::TileMapPattern] this tile set handles."]
        pub fn get_patterns_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_patterns_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileSet {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileSet"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSet {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSet {
        
    }
    impl crate::obj::cap::GodotDefault for TileSet {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSet {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSet`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileSet__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSet > for $Class {
                
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
#[doc = "Default-param extender for [`TileSet::add_source_ex`][super::TileSet::add_source_ex]."]
#[must_use]
pub struct ExAddSource < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, source: CowArg < 'ex, Option < Gd < crate::classes::TileSetSource > > >, atlas_source_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSource < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> > + 'ex,) -> Self {
        let atlas_source_id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source: source.into_arg(), atlas_source_id_override: atlas_source_id_override,
        }
    }
    #[inline]
    pub fn atlas_source_id_override(self, atlas_source_id_override: i32) -> Self {
        Self {
            atlas_source_id_override: atlas_source_id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, source, atlas_source_id_override,
        }
        = self;
        re_export::TileSet::add_source_full(surround_object, source, atlas_source_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_occlusion_layer_ex`][super::TileSet::add_occlusion_layer_ex]."]
#[must_use]
pub struct ExAddOcclusionLayer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddOcclusionLayer < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_occlusion_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_physics_layer_ex`][super::TileSet::add_physics_layer_ex]."]
#[must_use]
pub struct ExAddPhysicsLayer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPhysicsLayer < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_physics_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_set_ex`][super::TileSet::add_terrain_set_ex]."]
#[must_use]
pub struct ExAddTerrainSet < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddTerrainSet < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_set_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_ex`][super::TileSet::add_terrain_ex]."]
#[must_use]
pub struct ExAddTerrain < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, terrain_set: i32, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddTerrain < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet, terrain_set: i32,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, terrain_set: terrain_set, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, terrain_set, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_full(surround_object, terrain_set, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_navigation_layer_ex`][super::TileSet::add_navigation_layer_ex]."]
#[must_use]
pub struct ExAddNavigationLayer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddNavigationLayer < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_navigation_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_custom_data_layer_ex`][super::TileSet::add_custom_data_layer_ex]."]
#[must_use]
pub struct ExAddCustomDataLayer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCustomDataLayer < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_custom_data_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_pattern_ex`][super::TileSet::add_pattern_ex]."]
#[must_use]
pub struct ExAddPattern < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSet, pattern: CowArg < 'ex, Option < Gd < crate::classes::TileMapPattern > > >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPattern < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSet, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> > + 'ex,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, pattern, index,
        }
        = self;
        re_export::TileSet::add_pattern_full(surround_object, pattern, index,)
    }
}
#[doc = "Default-param extender for [`TileSet::get_pattern_ex`][super::TileSet::get_pattern_ex]."]
#[must_use]
pub struct ExGetPattern < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileSet, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPattern < 'ex > {
    fn new(surround_object: &'ex re_export::TileSet,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TileMapPattern > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TileSet::get_pattern_full(surround_object, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileShape {
    ord: i32
}
impl TileShape {
    #[doc(alias = "TILE_SHAPE_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_SQUARE`"]
    pub const SQUARE: TileShape = TileShape {
        ord: 0i32
    };
    #[doc(alias = "TILE_SHAPE_ISOMETRIC")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_ISOMETRIC`"]
    pub const ISOMETRIC: TileShape = TileShape {
        ord: 1i32
    };
    #[doc(alias = "TILE_SHAPE_HALF_OFFSET_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HALF_OFFSET_SQUARE`"]
    pub const HALF_OFFSET_SQUARE: TileShape = TileShape {
        ord: 2i32
    };
    #[doc(alias = "TILE_SHAPE_HEXAGON")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HEXAGON`"]
    pub const HEXAGON: TileShape = TileShape {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TileShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::SQUARE => "SQUARE", Self::ISOMETRIC => "ISOMETRIC", Self::HALF_OFFSET_SQUARE => "HALF_OFFSET_SQUARE", Self::HEXAGON => "HEXAGON", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileShape::SQUARE, TileShape::ISOMETRIC, TileShape::HALF_OFFSET_SQUARE, TileShape::HEXAGON]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SQUARE", "TILE_SHAPE_SQUARE", TileShape::SQUARE), crate::meta::inspect::EnumConstant::new("ISOMETRIC", "TILE_SHAPE_ISOMETRIC", TileShape::ISOMETRIC), crate::meta::inspect::EnumConstant::new("HALF_OFFSET_SQUARE", "TILE_SHAPE_HALF_OFFSET_SQUARE", TileShape::HALF_OFFSET_SQUARE), crate::meta::inspect::EnumConstant::new("HEXAGON", "TILE_SHAPE_HEXAGON", TileShape::HEXAGON)]
        }
    }
}
impl crate::meta::GodotConvert for TileShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tile Shape Square", 0i64), EnumeratorShape::new_int("Tile Shape Isometric", 1i64), EnumeratorShape::new_int("Tile Shape Half Offset Square", 2i64), EnumeratorShape::new_int("Tile Shape Hexagon", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSet.TileShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TileShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TileShape {
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
impl crate::registry::property::Export for TileShape {
    
}
impl crate::meta::Element for TileShape {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileLayout {
    ord: i32
}
impl TileLayout {
    #[doc(alias = "TILE_LAYOUT_STACKED")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED`"]
    pub const STACKED: TileLayout = TileLayout {
        ord: 0i32
    };
    #[doc(alias = "TILE_LAYOUT_STACKED_OFFSET")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED_OFFSET`"]
    pub const STACKED_OFFSET: TileLayout = TileLayout {
        ord: 1i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_RIGHT`"]
    pub const STAIRS_RIGHT: TileLayout = TileLayout {
        ord: 2i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_DOWN`"]
    pub const STAIRS_DOWN: TileLayout = TileLayout {
        ord: 3i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_RIGHT`"]
    pub const DIAMOND_RIGHT: TileLayout = TileLayout {
        ord: 4i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_DOWN`"]
    pub const DIAMOND_DOWN: TileLayout = TileLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for TileLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileLayout {
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
            Self::STACKED => "STACKED", Self::STACKED_OFFSET => "STACKED_OFFSET", Self::STAIRS_RIGHT => "STAIRS_RIGHT", Self::STAIRS_DOWN => "STAIRS_DOWN", Self::DIAMOND_RIGHT => "DIAMOND_RIGHT", Self::DIAMOND_DOWN => "DIAMOND_DOWN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileLayout::STACKED, TileLayout::STACKED_OFFSET, TileLayout::STAIRS_RIGHT, TileLayout::STAIRS_DOWN, TileLayout::DIAMOND_RIGHT, TileLayout::DIAMOND_DOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileLayout >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STACKED", "TILE_LAYOUT_STACKED", TileLayout::STACKED), crate::meta::inspect::EnumConstant::new("STACKED_OFFSET", "TILE_LAYOUT_STACKED_OFFSET", TileLayout::STACKED_OFFSET), crate::meta::inspect::EnumConstant::new("STAIRS_RIGHT", "TILE_LAYOUT_STAIRS_RIGHT", TileLayout::STAIRS_RIGHT), crate::meta::inspect::EnumConstant::new("STAIRS_DOWN", "TILE_LAYOUT_STAIRS_DOWN", TileLayout::STAIRS_DOWN), crate::meta::inspect::EnumConstant::new("DIAMOND_RIGHT", "TILE_LAYOUT_DIAMOND_RIGHT", TileLayout::DIAMOND_RIGHT), crate::meta::inspect::EnumConstant::new("DIAMOND_DOWN", "TILE_LAYOUT_DIAMOND_DOWN", TileLayout::DIAMOND_DOWN)]
        }
    }
}
impl crate::meta::GodotConvert for TileLayout {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tile Layout Stacked", 0i64), EnumeratorShape::new_int("Tile Layout Stacked Offset", 1i64), EnumeratorShape::new_int("Tile Layout Stairs Right", 2i64), EnumeratorShape::new_int("Tile Layout Stairs Down", 3i64), EnumeratorShape::new_int("Tile Layout Diamond Right", 4i64), EnumeratorShape::new_int("Tile Layout Diamond Down", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSet.TileLayout")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TileLayout {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TileLayout {
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
impl crate::registry::property::Export for TileLayout {
    
}
impl crate::meta::Element for TileLayout {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileOffsetAxis {
    ord: i32
}
impl TileOffsetAxis {
    #[doc(alias = "TILE_OFFSET_AXIS_HORIZONTAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_HORIZONTAL`"]
    pub const HORIZONTAL: TileOffsetAxis = TileOffsetAxis {
        ord: 0i32
    };
    #[doc(alias = "TILE_OFFSET_AXIS_VERTICAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_VERTICAL`"]
    pub const VERTICAL: TileOffsetAxis = TileOffsetAxis {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TileOffsetAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileOffsetAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileOffsetAxis {
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
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileOffsetAxis::HORIZONTAL, TileOffsetAxis::VERTICAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileOffsetAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HORIZONTAL", "TILE_OFFSET_AXIS_HORIZONTAL", TileOffsetAxis::HORIZONTAL), crate::meta::inspect::EnumConstant::new("VERTICAL", "TILE_OFFSET_AXIS_VERTICAL", TileOffsetAxis::VERTICAL)]
        }
    }
}
impl crate::meta::GodotConvert for TileOffsetAxis {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tile Offset Axis Horizontal", 0i64), EnumeratorShape::new_int("Tile Offset Axis Vertical", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSet.TileOffsetAxis")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TileOffsetAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileOffsetAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TileOffsetAxis {
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
impl crate::registry::property::Export for TileOffsetAxis {
    
}
impl crate::meta::Element for TileOffsetAxis {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellNeighbor {
    ord: i32
}
impl CellNeighbor {
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_SIDE`"]
    pub const RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 0i32
    };
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_CORNER`"]
    pub const RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 1i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE`"]
    pub const BOTTOM_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 2i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER`"]
    pub const BOTTOM_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 3i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_SIDE`"]
    pub const BOTTOM_SIDE: CellNeighbor = CellNeighbor {
        ord: 4i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_CORNER`"]
    pub const BOTTOM_CORNER: CellNeighbor = CellNeighbor {
        ord: 5i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_SIDE`"]
    pub const BOTTOM_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 6i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_CORNER`"]
    pub const BOTTOM_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 7i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_SIDE`"]
    pub const LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 8i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_CORNER`"]
    pub const LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 9i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_SIDE`"]
    pub const TOP_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 10i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_CORNER`"]
    pub const TOP_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 11i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_SIDE`"]
    pub const TOP_SIDE: CellNeighbor = CellNeighbor {
        ord: 12i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_CORNER`"]
    pub const TOP_CORNER: CellNeighbor = CellNeighbor {
        ord: 13i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_SIDE`"]
    pub const TOP_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 14i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_CORNER`"]
    pub const TOP_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 15i32
    };
    
}
impl std::fmt::Debug for CellNeighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellNeighbor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellNeighbor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
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
            Self::RIGHT_SIDE => "RIGHT_SIDE", Self::RIGHT_CORNER => "RIGHT_CORNER", Self::BOTTOM_RIGHT_SIDE => "BOTTOM_RIGHT_SIDE", Self::BOTTOM_RIGHT_CORNER => "BOTTOM_RIGHT_CORNER", Self::BOTTOM_SIDE => "BOTTOM_SIDE", Self::BOTTOM_CORNER => "BOTTOM_CORNER", Self::BOTTOM_LEFT_SIDE => "BOTTOM_LEFT_SIDE", Self::BOTTOM_LEFT_CORNER => "BOTTOM_LEFT_CORNER", Self::LEFT_SIDE => "LEFT_SIDE", Self::LEFT_CORNER => "LEFT_CORNER", Self::TOP_LEFT_SIDE => "TOP_LEFT_SIDE", Self::TOP_LEFT_CORNER => "TOP_LEFT_CORNER", Self::TOP_SIDE => "TOP_SIDE", Self::TOP_CORNER => "TOP_CORNER", Self::TOP_RIGHT_SIDE => "TOP_RIGHT_SIDE", Self::TOP_RIGHT_CORNER => "TOP_RIGHT_CORNER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CellNeighbor::RIGHT_SIDE, CellNeighbor::RIGHT_CORNER, CellNeighbor::BOTTOM_RIGHT_SIDE, CellNeighbor::BOTTOM_RIGHT_CORNER, CellNeighbor::BOTTOM_SIDE, CellNeighbor::BOTTOM_CORNER, CellNeighbor::BOTTOM_LEFT_SIDE, CellNeighbor::BOTTOM_LEFT_CORNER, CellNeighbor::LEFT_SIDE, CellNeighbor::LEFT_CORNER, CellNeighbor::TOP_LEFT_SIDE, CellNeighbor::TOP_LEFT_CORNER, CellNeighbor::TOP_SIDE, CellNeighbor::TOP_CORNER, CellNeighbor::TOP_RIGHT_SIDE, CellNeighbor::TOP_RIGHT_CORNER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CellNeighbor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RIGHT_SIDE", "CELL_NEIGHBOR_RIGHT_SIDE", CellNeighbor::RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("RIGHT_CORNER", "CELL_NEIGHBOR_RIGHT_CORNER", CellNeighbor::RIGHT_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT_SIDE", "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE", CellNeighbor::BOTTOM_RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT_CORNER", "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER", CellNeighbor::BOTTOM_RIGHT_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_SIDE", "CELL_NEIGHBOR_BOTTOM_SIDE", CellNeighbor::BOTTOM_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_CORNER", "CELL_NEIGHBOR_BOTTOM_CORNER", CellNeighbor::BOTTOM_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT_SIDE", "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE", CellNeighbor::BOTTOM_LEFT_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT_CORNER", "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER", CellNeighbor::BOTTOM_LEFT_CORNER), crate::meta::inspect::EnumConstant::new("LEFT_SIDE", "CELL_NEIGHBOR_LEFT_SIDE", CellNeighbor::LEFT_SIDE), crate::meta::inspect::EnumConstant::new("LEFT_CORNER", "CELL_NEIGHBOR_LEFT_CORNER", CellNeighbor::LEFT_CORNER), crate::meta::inspect::EnumConstant::new("TOP_LEFT_SIDE", "CELL_NEIGHBOR_TOP_LEFT_SIDE", CellNeighbor::TOP_LEFT_SIDE), crate::meta::inspect::EnumConstant::new("TOP_LEFT_CORNER", "CELL_NEIGHBOR_TOP_LEFT_CORNER", CellNeighbor::TOP_LEFT_CORNER), crate::meta::inspect::EnumConstant::new("TOP_SIDE", "CELL_NEIGHBOR_TOP_SIDE", CellNeighbor::TOP_SIDE), crate::meta::inspect::EnumConstant::new("TOP_CORNER", "CELL_NEIGHBOR_TOP_CORNER", CellNeighbor::TOP_CORNER), crate::meta::inspect::EnumConstant::new("TOP_RIGHT_SIDE", "CELL_NEIGHBOR_TOP_RIGHT_SIDE", CellNeighbor::TOP_RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("TOP_RIGHT_CORNER", "CELL_NEIGHBOR_TOP_RIGHT_CORNER", CellNeighbor::TOP_RIGHT_CORNER)]
        }
    }
}
impl crate::meta::GodotConvert for CellNeighbor {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cell Neighbor Right Side", 0i64), EnumeratorShape::new_int("Cell Neighbor Right Corner", 1i64), EnumeratorShape::new_int("Cell Neighbor Bottom Right Side", 2i64), EnumeratorShape::new_int("Cell Neighbor Bottom Right Corner", 3i64), EnumeratorShape::new_int("Cell Neighbor Bottom Side", 4i64), EnumeratorShape::new_int("Cell Neighbor Bottom Corner", 5i64), EnumeratorShape::new_int("Cell Neighbor Bottom Left Side", 6i64), EnumeratorShape::new_int("Cell Neighbor Bottom Left Corner", 7i64), EnumeratorShape::new_int("Cell Neighbor Left Side", 8i64), EnumeratorShape::new_int("Cell Neighbor Left Corner", 9i64), EnumeratorShape::new_int("Cell Neighbor Top Left Side", 10i64), EnumeratorShape::new_int("Cell Neighbor Top Left Corner", 11i64), EnumeratorShape::new_int("Cell Neighbor Top Side", 12i64), EnumeratorShape::new_int("Cell Neighbor Top Corner", 13i64), EnumeratorShape::new_int("Cell Neighbor Top Right Side", 14i64), EnumeratorShape::new_int("Cell Neighbor Top Right Corner", 15i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSet.CellNeighbor")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CellNeighbor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellNeighbor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CellNeighbor {
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
impl crate::registry::property::Export for CellNeighbor {
    
}
impl crate::meta::Element for CellNeighbor {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TerrainMode {
    ord: i32
}
impl TerrainMode {
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS_AND_SIDES`"]
    pub const CORNERS_AND_SIDES: TerrainMode = TerrainMode {
        ord: 0i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS`"]
    pub const CORNERS: TerrainMode = TerrainMode {
        ord: 1i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_SIDES`"]
    pub const SIDES: TerrainMode = TerrainMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TerrainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TerrainMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TerrainMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::CORNERS_AND_SIDES => "CORNERS_AND_SIDES", Self::CORNERS => "CORNERS", Self::SIDES => "SIDES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TerrainMode::CORNERS_AND_SIDES, TerrainMode::CORNERS, TerrainMode::SIDES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TerrainMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CORNERS_AND_SIDES", "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES", TerrainMode::CORNERS_AND_SIDES), crate::meta::inspect::EnumConstant::new("CORNERS", "TERRAIN_MODE_MATCH_CORNERS", TerrainMode::CORNERS), crate::meta::inspect::EnumConstant::new("SIDES", "TERRAIN_MODE_MATCH_SIDES", TerrainMode::SIDES)]
        }
    }
}
impl crate::meta::GodotConvert for TerrainMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Terrain Mode Match Corners And Sides", 0i64), EnumeratorShape::new_int("Terrain Mode Match Corners", 1i64), EnumeratorShape::new_int("Terrain Mode Match Sides", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSet.TerrainMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TerrainMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TerrainMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TerrainMode {
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
impl crate::registry::property::Export for TerrainMode {
    
}
impl crate::meta::Element for TerrainMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileSet;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileSet {
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