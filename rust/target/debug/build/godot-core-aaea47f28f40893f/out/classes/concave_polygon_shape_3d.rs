#![doc = "Sidecar module for class [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ConcavePolygonShape3D` enums](https://docs.godotengine.org/en/stable/classes/class_concavepolygonshape3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ConcavePolygonShape3D`.\n\nInherits [`Shape3D`][crate::classes::Shape3D].\n\nRelated symbols:\n\n* [`IConcavePolygonShape3D`][crate::classes::IConcavePolygonShape3D]: virtual methods\n\n\nSee also [Godot docs for `ConcavePolygonShape3D`](https://docs.godotengine.org/en/stable/classes/class_concavepolygonshape3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ConcavePolygonShape3D::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA 3D trimesh shape, intended for use in physics. Usually used to provide a shape for a [`CollisionShape3D`][crate::classes::CollisionShape3D].\n\nBeing just a collection of interconnected triangles, `ConcavePolygonShape3D` is the most freely configurable single 3D shape. It can be used to form polyhedra of any nature, or even shapes that don't enclose a volume. However, `ConcavePolygonShape3D` is _hollow_ even if the interconnected triangles do enclose a volume, which often makes it unsuitable for physics or detection.\n\n**Note:** When used for collision, `ConcavePolygonShape3D` is intended to work with static [`CollisionShape3D`][crate::classes::CollisionShape3D] nodes like [`StaticBody3D`][crate::classes::StaticBody3D] and will likely not behave well for [`CharacterBody3D`][crate::classes::CharacterBody3D]s or [`RigidBody3D`][crate::classes::RigidBody3D]s in a mode other than Static.\n\n**Warning:** Physics bodies that are small have a chance to clip through this shape when moving fast. This happens because on one frame, the physics body may be on the \"outside\" of the shape, and on the next frame it may be \"inside\" it. `ConcavePolygonShape3D` is hollow, so it won't detect a collision.\n\n**Performance:** Due to its complexity, `ConcavePolygonShape3D` is the slowest 3D collision shape to check collisions against. Its use should generally be limited to level geometry. For convex geometry, [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D] should be used. For dynamic physics bodies that need concave collision, several [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D]s can be used to represent its collision by using convex decomposition; see [`ConvexPolygonShape3D`][crate::classes::ConvexPolygonShape3D]'s documentation for instructions."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ConcavePolygonShape3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ConcavePolygonShape3D`][crate::classes::ConcavePolygonShape3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IShape3D`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ConcavePolygonShape3D` methods](https://docs.godotengine.org/en/stable/classes/class_concavepolygonshape3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IConcavePolygonShape3D: crate::obj::GodotClass < Base = ConcavePolygonShape3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ConcavePolygonShape3D {
        #[doc = "Sets the faces of the trimesh shape from an array of vertices. The `faces` array should be composed of triples such that each triple of vertices defines a triangle."]
        pub fn set_faces(&mut self, faces: &PackedVector3Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(faces),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConcavePolygonShape3D", "set_faces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the faces of the trimesh shape as an array of vertices. The array (of length divisible by three) is naturally divided into triples; each triple of vertices defines a triangle."]
        pub fn get_faces(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConcavePolygonShape3D", "get_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backface_collision_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConcavePolygonShape3D", "set_backface_collision_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_backface_collision_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ConcavePolygonShape3D", "is_backface_collision_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ConcavePolygonShape3D {
        type Base = crate::classes::Shape3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ConcavePolygonShape3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ConcavePolygonShape3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Shape3D > for ConcavePolygonShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ConcavePolygonShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ConcavePolygonShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ConcavePolygonShape3D {
        
    }
    impl crate::obj::cap::GodotDefault for ConcavePolygonShape3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ConcavePolygonShape3D {
        type Target = crate::classes::Shape3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ConcavePolygonShape3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ConcavePolygonShape3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ConcavePolygonShape3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConcavePolygonShape3D > for $Class {
                
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
    use super::re_export::ConcavePolygonShape3D;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ConcavePolygonShape3D {
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