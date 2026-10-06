#![doc = "Sidecar module for class [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetAtlasSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetAtlasSource`.\n\nInherits [`TileSetSource`][crate::classes::TileSetSource].\n\nRelated symbols:\n\n* [`tile_set_atlas_source`][crate::classes::tile_set_atlas_source]: sidecar module with related enum/flag types\n* [`ITileSetAtlasSource`][crate::classes::ITileSetAtlasSource]: virtual methods\n\n\nSee also [Godot docs for `TileSetAtlasSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSetAtlasSource::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAn atlas is a grid of tiles laid out on a texture. Each tile in the grid must be exposed using [`create_tile`][`crate::classes::TileSetAtlasSource::create_tile`]. Those tiles are then indexed using their coordinates in the grid.\n\nEach tile can also have a size in the grid coordinates, making it more or less cells in the atlas.\n\nAlternatives version of a tile can be created using [`create_alternative_tile`][`crate::classes::TileSetAtlasSource::create_alternative_tile`], which are then indexed using an alternative ID. The main tile (the one in the grid), is accessed with an alternative ID equal to 0.\n\nEach tile alternate has a set of properties that is defined by the source's [`TileSet`][crate::classes::TileSet] layers. Those properties are stored in a TileData object that can be accessed and modified using [`get_tile_data`][`crate::classes::TileSetAtlasSource::get_tile_data`].\n\nAs TileData properties are stored directly in the TileSetAtlasSource resource, their properties might also be set using `TileSetAtlasSource.set(\"<coords_x>:<coords_y>/<alternative_id>/<tile_data_property>\")`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetAtlasSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ITileSetSource`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TileSetAtlasSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSetAtlasSource: crate::obj::GodotClass < Base = TileSetAtlasSource > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSetAtlasSource {
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_margins(&mut self, margins: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (margins,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_margins", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_margins(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_margins", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_separation(&mut self, separation: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_separation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_separation(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_separation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_region_size(&mut self, texture_region_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_texture_region_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_region_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_texture_region_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_texture_padding(&mut self, use_texture_padding: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_texture_padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_use_texture_padding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_texture_padding(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_use_texture_padding", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new tile at coordinates `atlas_coords` with the given `size`."]
        pub(crate) fn create_tile_full(&mut self, atlas_coords: Vector2i, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i, Vector2i,);
            let args = (atlas_coords, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "create_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_tile_ex`][Self::create_tile_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new tile at coordinates `atlas_coords` with the given `size`."]
        #[inline]
        pub fn create_tile(&mut self, atlas_coords: Vector2i,) {
            self.create_tile_ex(atlas_coords,) . done()
        }
        #[doc = "Creates a new tile at coordinates `atlas_coords` with the given `size`."]
        #[inline]
        pub fn create_tile_ex < 'ex > (&'ex mut self, atlas_coords: Vector2i,) -> ExCreateTile < 'ex > {
            ExCreateTile::new(self, atlas_coords,)
        }
        #[doc = "Remove a tile and its alternative at coordinates `atlas_coords`."]
        pub fn remove_tile(&mut self, atlas_coords: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "remove_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Move the tile and its alternatives at the `atlas_coords` coordinates to the `new_atlas_coords` coordinates with the `new_size` size. This functions will fail if a tile is already present in the given area.\n\nIf `new_atlas_coords` is `Vector2i(-1, -1)`, keeps the tile's coordinates. If `new_size` is `Vector2i(-1, -1)`, keeps the tile's size.\n\nTo avoid an error, first check if a move is possible using [`has_room_for_tile`][`crate::classes::TileSetAtlasSource::has_room_for_tile`]."]
        pub(crate) fn move_tile_in_atlas_full(&mut self, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i, Vector2i, Vector2i,);
            let args = (atlas_coords, new_atlas_coords, new_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "move_tile_in_atlas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`move_tile_in_atlas_ex`][Self::move_tile_in_atlas_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Move the tile and its alternatives at the `atlas_coords` coordinates to the `new_atlas_coords` coordinates with the `new_size` size. This functions will fail if a tile is already present in the given area.\n\nIf `new_atlas_coords` is `Vector2i(-1, -1)`, keeps the tile's coordinates. If `new_size` is `Vector2i(-1, -1)`, keeps the tile's size.\n\nTo avoid an error, first check if a move is possible using [`has_room_for_tile`][`crate::classes::TileSetAtlasSource::has_room_for_tile`]."]
        #[inline]
        pub fn move_tile_in_atlas(&mut self, atlas_coords: Vector2i,) {
            self.move_tile_in_atlas_ex(atlas_coords,) . done()
        }
        #[doc = "Move the tile and its alternatives at the `atlas_coords` coordinates to the `new_atlas_coords` coordinates with the `new_size` size. This functions will fail if a tile is already present in the given area.\n\nIf `new_atlas_coords` is `Vector2i(-1, -1)`, keeps the tile's coordinates. If `new_size` is `Vector2i(-1, -1)`, keeps the tile's size.\n\nTo avoid an error, first check if a move is possible using [`has_room_for_tile`][`crate::classes::TileSetAtlasSource::has_room_for_tile`]."]
        #[inline]
        pub fn move_tile_in_atlas_ex < 'ex > (&'ex mut self, atlas_coords: Vector2i,) -> ExMoveTileInAtlas < 'ex > {
            ExMoveTileInAtlas::new(self, atlas_coords,)
        }
        #[doc = "Returns the size of the tile (in the grid coordinates system) at coordinates `atlas_coords`."]
        pub fn get_tile_size_in_atlas(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_size_in_atlas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether there is enough room in an atlas to create/modify a tile with the given properties. If `ignored_tile` is provided, act as is the given tile was not present in the atlas. This may be used when you want to modify a tile's properties."]
        pub(crate) fn has_room_for_tile_full(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i, Vector2i, i32, Vector2i, i32, Vector2i,);
            let args = (atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "has_room_for_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`has_room_for_tile_ex`][Self::has_room_for_tile_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns whether there is enough room in an atlas to create/modify a tile with the given properties. If `ignored_tile` is provided, act as is the given tile was not present in the atlas. This may be used when you want to modify a tile's properties."]
        #[inline]
        pub fn has_room_for_tile(&self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> bool {
            self.has_room_for_tile_ex(atlas_coords, size, animation_columns, animation_separation, frames_count,) . done()
        }
        #[doc = "Returns whether there is enough room in an atlas to create/modify a tile with the given properties. If `ignored_tile` is provided, act as is the given tile was not present in the atlas. This may be used when you want to modify a tile's properties."]
        #[inline]
        pub fn has_room_for_tile_ex < 'ex > (&'ex self, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> ExHasRoomForTile < 'ex > {
            ExHasRoomForTile::new(self, atlas_coords, size, animation_columns, animation_separation, frames_count,)
        }
        #[doc = "Returns an array of tiles coordinates ID that will be automatically removed when modifying one or several of those properties: `texture`, `margins`, `separation` or `texture_region_size`. This can be used to undo changes that would have caused tiles data loss."]
        pub fn get_tiles_to_be_removed_on_change(&self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, margins: Vector2i, separation: Vector2i, texture_region_size: Vector2i,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, Vector2i, Vector2i, Vector2i,);
            let args = (texture.into_arg(), margins, separation, texture_region_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tiles_to_be_removed_on_change", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If there is a tile covering the `atlas_coords` coordinates, returns the top-left coordinates of the tile (thus its coordinate ID). Returns `Vector2i(-1, -1)` otherwise."]
        pub fn get_tile_at_coords(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_at_coords", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Checks if the source has any tiles that don't fit the texture area (either partially or completely)."]
        pub fn has_tiles_outside_texture(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "has_tiles_outside_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all tiles that don't fit the available texture area. This method iterates over all the source's tiles, so it's advised to use [`has_tiles_outside_texture`][`crate::classes::TileSetAtlasSource::has_tiles_outside_texture`] beforehand."]
        pub fn clear_tiles_outside_texture(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "clear_tiles_outside_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of columns in the animation layout of the tile at coordinates `atlas_coords`. If set to 0, then the different frames of the animation are laid out as a single horizontal line in the atlas."]
        pub fn set_tile_animation_columns(&mut self, atlas_coords: Vector2i, frame_columns: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, frame_columns,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_columns", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns how many columns the tile at `atlas_coords` has in its animation layout."]
        pub fn get_tile_animation_columns(&self, atlas_coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_columns", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the margin (in grid tiles) between each tile in the animation layout of the tile at coordinates `atlas_coords` has."]
        pub fn set_tile_animation_separation(&mut self, atlas_coords: Vector2i, separation: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i, Vector2i,);
            let args = (atlas_coords, separation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_separation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the separation (as in the atlas grid) between each frame of an animated tile at coordinates `atlas_coords`."]
        pub fn get_tile_animation_separation(&self, atlas_coords: Vector2i,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_separation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the animation speed of the tile at coordinates `atlas_coords` has."]
        pub fn set_tile_animation_speed(&mut self, atlas_coords: Vector2i, speed: f32,) {
            type CallRet = ();
            type CallParams = (Vector2i, f32,);
            let args = (atlas_coords, speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the animation speed of the tile at coordinates `atlas_coords`."]
        pub fn get_tile_animation_speed(&self, atlas_coords: Vector2i,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tile animation mode of the tile at `atlas_coords` to `mode`. See also [`get_tile_animation_mode`][`crate::classes::TileSetAtlasSource::get_tile_animation_mode`]."]
        pub fn set_tile_animation_mode(&mut self, atlas_coords: Vector2i, mode: crate::classes::tile_set_atlas_source::TileAnimationMode,) {
            type CallRet = ();
            type CallParams = (Vector2i, crate::classes::tile_set_atlas_source::TileAnimationMode,);
            let args = (atlas_coords, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile animation mode of the tile at `atlas_coords`. See also [`set_tile_animation_mode`][`crate::classes::TileSetAtlasSource::set_tile_animation_mode`]."]
        pub fn get_tile_animation_mode(&self, atlas_coords: Vector2i,) -> crate::classes::tile_set_atlas_source::TileAnimationMode {
            type CallRet = crate::classes::tile_set_atlas_source::TileAnimationMode;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets how many animation frames the tile at coordinates `atlas_coords` has."]
        pub fn set_tile_animation_frames_count(&mut self, atlas_coords: Vector2i, frames_count: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, frames_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_frames_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns how many animation frames has the tile at coordinates `atlas_coords`."]
        pub fn get_tile_animation_frames_count(&self, atlas_coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_frames_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the animation frame `duration` of frame `frame_index` for the tile at coordinates `atlas_coords`."]
        pub fn set_tile_animation_frame_duration(&mut self, atlas_coords: Vector2i, frame_index: i32, duration: f32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32, f32,);
            let args = (atlas_coords, frame_index, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_tile_animation_frame_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the animation frame duration of frame `frame_index` for the tile at coordinates `atlas_coords`."]
        pub fn get_tile_animation_frame_duration(&self, atlas_coords: Vector2i, frame_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, frame_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_frame_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the sum of the sum of the frame durations of the tile at coordinates `atlas_coords`. This value needs to be divided by the animation speed to get the actual animation loop duration."]
        pub fn get_tile_animation_total_duration(&self, atlas_coords: Vector2i,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_animation_total_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an alternative tile for the tile at coordinates `atlas_coords`. If `alternative_id_override` is -1, give it an automatically generated unique ID, or assigns it the given ID otherwise.\n\nReturns the new alternative identifier, or -1 if the alternative could not be created with a provided `alternative_id_override`."]
        pub(crate) fn create_alternative_tile_full(&mut self, atlas_coords: Vector2i, alternative_id_override: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, alternative_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "create_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_alternative_tile_ex`][Self::create_alternative_tile_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an alternative tile for the tile at coordinates `atlas_coords`. If `alternative_id_override` is -1, give it an automatically generated unique ID, or assigns it the given ID otherwise.\n\nReturns the new alternative identifier, or -1 if the alternative could not be created with a provided `alternative_id_override`."]
        #[inline]
        pub fn create_alternative_tile(&mut self, atlas_coords: Vector2i,) -> i32 {
            self.create_alternative_tile_ex(atlas_coords,) . done()
        }
        #[doc = "Creates an alternative tile for the tile at coordinates `atlas_coords`. If `alternative_id_override` is -1, give it an automatically generated unique ID, or assigns it the given ID otherwise.\n\nReturns the new alternative identifier, or -1 if the alternative could not be created with a provided `alternative_id_override`."]
        #[inline]
        pub fn create_alternative_tile_ex < 'ex > (&'ex mut self, atlas_coords: Vector2i,) -> ExCreateAlternativeTile < 'ex > {
            ExCreateAlternativeTile::new(self, atlas_coords,)
        }
        #[doc = "Remove a tile's alternative with alternative ID `alternative_tile`.\n\nCalling this function with `alternative_tile` equals to 0 will fail, as the base tile alternative cannot be removed."]
        pub fn remove_alternative_tile(&mut self, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "remove_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Change a tile's alternative ID from `alternative_tile` to `new_id`.\n\nCalling this function with `new_id` of 0 will fail, as the base tile alternative cannot be moved."]
        pub fn set_alternative_tile_id(&mut self, atlas_coords: Vector2i, alternative_tile: i32, new_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32, i32,);
            let args = (atlas_coords, alternative_tile, new_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "set_alternative_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the alternative ID a following call to [`create_alternative_tile`][`crate::classes::TileSetAtlasSource::create_alternative_tile`] would return."]
        pub fn get_next_alternative_tile_id(&self, atlas_coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_next_alternative_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`TileData`][crate::classes::TileData] object for the given atlas coordinates and alternative ID."]
        pub fn get_tile_data(&self, atlas_coords: Vector2i, alternative_tile: i32,) -> Option < Gd < crate::classes::TileData > > {
            type CallRet = Option < Gd < crate::classes::TileData > >;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the atlas grid size, which depends on how many tiles can fit in the texture. It thus depends on the \\[member texture]'s size, the atlas \\[member margins], and the tiles' \\[member texture_region_size]."]
        pub fn get_atlas_grid_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_atlas_grid_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a tile's texture region in the atlas texture. For animated tiles, a `frame` argument might be provided for the different frames of the animation."]
        pub(crate) fn get_tile_texture_region_full(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_tile_texture_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_tile_texture_region_ex`][Self::get_tile_texture_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a tile's texture region in the atlas texture. For animated tiles, a `frame` argument might be provided for the different frames of the animation."]
        #[inline]
        pub fn get_tile_texture_region(&self, atlas_coords: Vector2i,) -> Rect2i {
            self.get_tile_texture_region_ex(atlas_coords,) . done()
        }
        #[doc = "Returns a tile's texture region in the atlas texture. For animated tiles, a `frame` argument might be provided for the different frames of the animation."]
        #[inline]
        pub fn get_tile_texture_region_ex < 'ex > (&'ex self, atlas_coords: Vector2i,) -> ExGetTileTextureRegion < 'ex > {
            ExGetTileTextureRegion::new(self, atlas_coords,)
        }
        #[doc = "If \\[member use_texture_padding] is `false`, returns \\[member texture]. Otherwise, returns an internal [`ImageTexture`][crate::classes::ImageTexture] created that includes the padding."]
        pub fn get_runtime_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_runtime_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the region of the tile at coordinates `atlas_coords` for the given `frame` inside the texture returned by [`get_runtime_texture`][`crate::classes::TileSetAtlasSource::get_runtime_texture`].\n\n**Note:** If \\[member use_texture_padding] is `false`, returns the same as [`get_tile_texture_region`][`crate::classes::TileSetAtlasSource::get_tile_texture_region`]."]
        pub fn get_runtime_tile_texture_region(&self, atlas_coords: Vector2i, frame: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetAtlasSource", "get_runtime_tile_texture_region", Some(self.__validated_obj()), args,)
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
        pub const TRANSFORM_FLIP_H: i32 = 4096i32;
        pub const TRANSFORM_FLIP_V: i32 = 8192i32;
        pub const TRANSFORM_TRANSPOSE: i32 = 16384i32;
        
    }
    impl crate::obj::GodotClass for TileSetAtlasSource {
        type Base = crate::classes::TileSetSource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileSetAtlasSource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetAtlasSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TileSetSource > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSetAtlasSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSetAtlasSource {
        
    }
    impl crate::obj::cap::GodotDefault for TileSetAtlasSource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSetAtlasSource {
        type Target = crate::classes::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetAtlasSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSetAtlasSource`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileSetAtlasSource__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetAtlasSource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetSource > for $Class {
                
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
#[doc = "Default-param extender for [`TileSetAtlasSource::create_tile_ex`][super::TileSetAtlasSource::create_tile_ex]."]
#[must_use]
pub struct ExCreateTile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateTile < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let size = Vector2i::new(1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, size: size,
        }
    }
    #[inline]
    pub fn size(self, size: Vector2i) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, atlas_coords, size,
        }
        = self;
        re_export::TileSetAtlasSource::create_tile_full(surround_object, atlas_coords, size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::move_tile_in_atlas_ex`][super::TileSetAtlasSource::move_tile_in_atlas_ex]."]
#[must_use]
pub struct ExMoveTileInAtlas < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, new_atlas_coords: Vector2i, new_size: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMoveTileInAtlas < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let new_atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let new_size = Vector2i::new(- 1 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, new_atlas_coords: new_atlas_coords, new_size: new_size,
        }
    }
    #[inline]
    pub fn new_atlas_coords(self, new_atlas_coords: Vector2i) -> Self {
        Self {
            new_atlas_coords: new_atlas_coords, .. self
        }
    }
    #[inline]
    pub fn new_size(self, new_size: Vector2i) -> Self {
        Self {
            new_size: new_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, atlas_coords, new_atlas_coords, new_size,
        }
        = self;
        re_export::TileSetAtlasSource::move_tile_in_atlas_full(surround_object, atlas_coords, new_atlas_coords, new_size,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::has_room_for_tile_ex`][super::TileSetAtlasSource::has_room_for_tile_ex]."]
#[must_use]
pub struct ExHasRoomForTile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32, ignored_tile: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExHasRoomForTile < 'ex > {
    fn new(surround_object: &'ex re_export::TileSetAtlasSource, atlas_coords: Vector2i, size: Vector2i, animation_columns: i32, animation_separation: Vector2i, frames_count: i32,) -> Self {
        let ignored_tile = Vector2i::new(- 1 as _, - 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, size: size, animation_columns: animation_columns, animation_separation: animation_separation, frames_count: frames_count, ignored_tile: ignored_tile,
        }
    }
    #[inline]
    pub fn ignored_tile(self, ignored_tile: Vector2i) -> Self {
        Self {
            ignored_tile: ignored_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,
        }
        = self;
        re_export::TileSetAtlasSource::has_room_for_tile_full(surround_object, atlas_coords, size, animation_columns, animation_separation, frames_count, ignored_tile,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::create_alternative_tile_ex`][super::TileSetAtlasSource::create_alternative_tile_ex]."]
#[must_use]
pub struct ExCreateAlternativeTile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i, alternative_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateAlternativeTile < 'ex > {
    fn new(surround_object: &'ex mut re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let alternative_id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, alternative_id_override: alternative_id_override,
        }
    }
    #[inline]
    pub fn alternative_id_override(self, alternative_id_override: i32) -> Self {
        Self {
            alternative_id_override: alternative_id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, atlas_coords, alternative_id_override,
        }
        = self;
        re_export::TileSetAtlasSource::create_alternative_tile_full(surround_object, atlas_coords, alternative_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSetAtlasSource::get_tile_texture_region_ex`][super::TileSetAtlasSource::get_tile_texture_region_ex]."]
#[must_use]
pub struct ExGetTileTextureRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TileSetAtlasSource, atlas_coords: Vector2i, frame: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetTileTextureRegion < 'ex > {
    fn new(surround_object: &'ex re_export::TileSetAtlasSource, atlas_coords: Vector2i,) -> Self {
        let frame = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, atlas_coords: atlas_coords, frame: frame,
        }
    }
    #[inline]
    pub fn frame(self, frame: i32) -> Self {
        Self {
            frame: frame, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        let Self {
            _phantom, surround_object, atlas_coords, frame,
        }
        = self;
        re_export::TileSetAtlasSource::get_tile_texture_region_full(surround_object, atlas_coords, frame,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileAnimationMode {
    ord: i32
}
impl TileAnimationMode {
    #[doc(alias = "TILE_ANIMATION_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_DEFAULT`"]
    pub const DEFAULT: TileAnimationMode = TileAnimationMode {
        ord: 0i32
    };
    #[doc(alias = "TILE_ANIMATION_MODE_RANDOM_START_TIMES")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_RANDOM_START_TIMES`"]
    pub const RANDOM_START_TIMES: TileAnimationMode = TileAnimationMode {
        ord: 1i32
    };
    #[doc(alias = "TILE_ANIMATION_MODE_MAX")]
    #[doc = "Godot enumerator name: `TILE_ANIMATION_MODE_MAX`"]
    pub const MAX: TileAnimationMode = TileAnimationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TileAnimationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileAnimationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileAnimationMode {
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
            Self::DEFAULT => "DEFAULT", Self::RANDOM_START_TIMES => "RANDOM_START_TIMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileAnimationMode::DEFAULT, TileAnimationMode::RANDOM_START_TIMES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileAnimationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "TILE_ANIMATION_MODE_DEFAULT", TileAnimationMode::DEFAULT), crate::meta::inspect::EnumConstant::new("RANDOM_START_TIMES", "TILE_ANIMATION_MODE_RANDOM_START_TIMES", TileAnimationMode::RANDOM_START_TIMES), crate::meta::inspect::EnumConstant::new("MAX", "TILE_ANIMATION_MODE_MAX", TileAnimationMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TileAnimationMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for TileAnimationMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tile Animation Mode Default", 0i64), EnumeratorShape::new_int("Tile Animation Mode Random Start Times", 1i64), EnumeratorShape::new_int("Tile Animation Mode Max", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TileSetAtlasSource.TileAnimationMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TileAnimationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileAnimationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TileAnimationMode {
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
impl crate::registry::property::Export for TileAnimationMode {
    
}
impl crate::meta::Element for TileAnimationMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileSetAtlasSource;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileSetAtlasSource {
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