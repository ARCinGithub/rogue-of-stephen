#![doc = "Sidecar module for class [`HeightMapShape3D`][crate::classes::HeightMapShape3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HeightMapShape3D` enums](https://docs.godotengine.org/en/stable/classes/class_heightmapshape3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HeightMapShape3D`.\n\nInherits [`Shape3D`][crate::classes::Shape3D].\n\nRelated symbols:\n\n* [`IHeightMapShape3D`][crate::classes::IHeightMapShape3D]: virtual methods\n\n\nSee also [Godot docs for `HeightMapShape3D`](https://docs.godotengine.org/en/stable/classes/class_heightmapshape3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`HeightMapShape3D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA 3D heightmap shape, intended for use in physics to provide a shape for a [`CollisionShape3D`][crate::classes::CollisionShape3D]. This type is most commonly used for terrain with vertices placed in a fixed-width grid.\n\nThe heightmap is represented as a 2D grid of height values, which represent the position of grid points on the Y axis. Grid points are spaced 1 unit apart on the X and Z axes, and the grid is centered on the origin of the [`CollisionShape3D`][crate::classes::CollisionShape3D] node. Internally, each grid square is divided into two triangles.\n\nDue to the nature of the heightmap, it cannot be used to model overhangs or caves, which would require multiple vertices at the same vertical location. Holes can be punched through the collision by assigning `@GDScript.NAN` to the height of the desired vertices (this is supported in both GodotPhysics3D and Jolt Physics). You could then insert meshes with their own separate collision to provide overhangs, caves, and so on.\n\n**Performance:** `HeightMapShape3D` is faster to check collisions against than [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D], but it is significantly slower than primitive shapes like [`BoxShape3D`][crate::classes::BoxShape3D].\n\nA heightmap collision shape can also be built by using an [`Image`][crate::classes::Image] reference:\n\n\n```gdscript\nvar heightmap_texture = ResourceLoader.load(\"res://heightmap_image.exr\")\nvar heightmap_image = heightmap_texture.get_image()\nheightmap_image.convert(Image.FORMAT_RF)\n\nvar height_min = 0.0\nvar height_max = 10.0\n\nupdate_map_data_from_image(heightmap_image, height_min, height_max)\n```\n\n\n**Note:** If you need to use a spacing different than 1 unit, you can adjust the \\[member Node3D.scale] of the shape. However, keep in mind that GodotPhysics3D does not support non-uniform scaling: you'll need to scale the Y axis by the same amount as the X and Z axes, which means the values in \\[member map_data] will need to be pre-scaled by the inverse of that scale. Also note that GodotPhysics3D does not support scaling at all for dynamic bodies (that is, non-frozen [`RigidBody3D`][crate::classes::RigidBody3D] nodes); to use a scaled `HeightMapShape3D` with those, you will need to use Jolt Physics."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HeightMapShape3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`HeightMapShape3D`][crate::classes::HeightMapShape3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IShape3D`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `HeightMapShape3D` methods](https://docs.godotengine.org/en/stable/classes/class_heightmapshape3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHeightMapShape3D: crate::obj::GodotClass < Base = HeightMapShape3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl HeightMapShape3D {
        pub fn set_map_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "set_map_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_map_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "get_map_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_map_depth(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "set_map_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_map_depth(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "get_map_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_map_data(&mut self, data: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "set_map_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_map_data(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "get_map_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the smallest height value found in \\[member map_data]. Recalculates only when \\[member map_data] changes."]
        pub fn get_min_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "get_min_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the largest height value found in \\[member map_data]. Recalculates only when \\[member map_data] changes."]
        pub fn get_max_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "get_max_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates \\[member map_data] with data read from an [`Image`][crate::classes::Image] reference. Automatically resizes heightmap \\[member map_width] and \\[member map_depth] to fit the full image width and height.\n\nThe image needs to be in either [`Format::RF`][`crate::classes::image::Format::RF`] (32 bit), [`Format::RH`][`crate::classes::image::Format::RH`] (16 bit), or [`Format::R8`][`crate::classes::image::Format::R8`] (8 bit).\n\nEach image pixel is read in as a float on the range from `0.0` (black pixel) to `1.0` (white pixel). This range value gets remapped to `height_min` and `height_max` to form the final height value.\n\n**Note:** Using a heightmap with 16-bit or 32-bit data, stored in EXR or HDR format is recommended. Using 8-bit height data, or a format like PNG that Godot imports as 8-bit, will result in a terraced terrain."]
        pub fn update_map_data_from_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, height_min: f32, height_max: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, f32, f32,);
            let args = (image.into_arg(), height_min, height_max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HeightMapShape3D", "update_map_data_from_image", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for HeightMapShape3D {
        type Base = crate::classes::Shape3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("HeightMapShape3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HeightMapShape3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Shape3D > for HeightMapShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for HeightMapShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for HeightMapShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for HeightMapShape3D {
        
    }
    impl crate::obj::cap::GodotDefault for HeightMapShape3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HeightMapShape3D {
        type Target = crate::classes::Shape3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HeightMapShape3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`HeightMapShape3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_HeightMapShape3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::HeightMapShape3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Shape3D > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::HeightMapShape3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for HeightMapShape3D {
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