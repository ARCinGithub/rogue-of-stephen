#![doc = "Sidecar module for class [`TileSetSource`][crate::classes::TileSetSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetsource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetSource`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `TileSetSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetsource.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TileSetSource>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nExposes a set of tiles for a [`TileSet`][crate::classes::TileSet] resource.\n\nTiles in a source are indexed with two IDs, coordinates ID (of type Vector2i) and an alternative ID (of type int), named according to their use in the [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource] class.\n\nDepending on the TileSet source type, those IDs might have restrictions on their values, this is why the base `TileSetSource` class only exposes getters for them.\n\nYou can iterate over all tiles exposed by a TileSetSource by first iterating over coordinates IDs using [`get_tiles_count`][`crate::classes::TileSetSource::get_tiles_count`] and [`get_tile_id`][`crate::classes::TileSetSource::get_tile_id`], then over alternative IDs using [`get_alternative_tiles_count`][`crate::classes::TileSetSource::get_alternative_tiles_count`] and [`get_alternative_tile_id`][`crate::classes::TileSetSource::get_alternative_tile_id`].\n\n**Warning:** `TileSetSource` can only be added to one TileSet at the same time. Calling [`add_source`][`crate::classes::TileSet::add_source`] on a second [`TileSet`][crate::classes::TileSet] will remove the source from the first one."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TileSetSource {
        #[doc = "Returns how many tiles this atlas source defines (not including alternative tiles)."]
        pub fn get_tiles_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "get_tiles_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tile coordinates ID of the tile with index `index`."]
        pub fn get_tile_id(&self, index: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "get_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if this atlas has a tile with coordinates ID `atlas_coords`."]
        pub fn has_tile(&self, atlas_coords: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "has_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of alternatives tiles for the coordinates ID `atlas_coords`.\n\nFor [`TileSetAtlasSource`][crate::classes::TileSetAtlasSource], this always return at least 1, as the base tile with ID 0 is always part of the alternatives list.\n\nReturns -1 if there is not tile at the given coords."]
        pub fn get_alternative_tiles_count(&self, atlas_coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (atlas_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "get_alternative_tiles_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the alternative ID for the tile with coordinates ID `atlas_coords` at index `index`."]
        pub fn get_alternative_tile_id(&self, atlas_coords: Vector2i, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "get_alternative_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if the base tile at coordinates `atlas_coords` has an alternative with ID `alternative_tile`."]
        pub fn has_alternative_tile(&self, atlas_coords: Vector2i, alternative_tile: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i, i32,);
            let args = (atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetSource", "has_alternative_tile", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileSetSource {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileSetSource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSetSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSetSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSetSource {
        
    }
    impl std::ops::Deref for TileSetSource {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileSetSource__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TileSetSource` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileSetSource;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileSetSource {
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