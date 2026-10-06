#![doc = "Sidecar module for class [`GltfBufferView`][crate::classes::GltfBufferView].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFBufferView` enums](https://docs.godotengine.org/en/stable/classes/class_gltfbufferview.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFBufferView`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfBufferView`][crate::classes::IGltfBufferView]: virtual methods\n\n\nSee also [Godot docs for `GLTFBufferView`](https://docs.godotengine.org/en/stable/classes/class_gltfbufferview.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfBufferView::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nGLTFBufferView is a data structure representing a glTF `bufferView` that would be found in the `\"bufferViews\"` array. A buffer is a blob of binary data. A buffer view is a slice of a buffer that can be used to identify and extract data from the buffer.\n\nMost custom uses of buffers only need to use the \\[member buffer], \\[member byte_length], and \\[member byte_offset]. The \\[member byte_stride] and \\[member indices] properties are for more advanced use cases such as interleaved mesh data encoded for the GPU."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfBufferView {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfBufferView`][crate::classes::GltfBufferView].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFBufferView` methods](https://docs.godotengine.org/en/stable/classes/class_gltfbufferview.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfBufferView: crate::obj::GodotClass < Base = GltfBufferView > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfBufferView {
        #[doc = "Loads the buffer view data from the buffer referenced by this buffer view in the given [`GLTFState`][crate::classes::GltfState]. Interleaved data with a byte stride is not yet supported by this method. The data is returned as a [`PackedByteArray`][crate::builtin::PackedByteArray]."]
        pub fn load_buffer_view_data(&self, state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >,);
            let args = (state.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "load_buffer_view_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new GLTFBufferView instance by parsing the given [`Dictionary`][crate::builtin::Dictionary]."]
        pub fn from_dictionary(dictionary: &AnyDictionary,) -> Option < Gd < crate::classes::GltfBufferView > > {
            type CallRet = Option < Gd < crate::classes::GltfBufferView > >;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(dictionary),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "from_dictionary", None, args,)
            }
        }
        #[doc = "Serializes this GLTFBufferView instance into a [`Dictionary`][crate::builtin::Dictionary]."]
        pub fn to_dictionary(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "to_dictionary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer(&mut self, buffer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_byte_offset(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_byte_offset(&mut self, byte_offset: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (byte_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_byte_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_byte_length(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_byte_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_byte_length(&mut self, byte_length: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (byte_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_byte_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_byte_stride(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_byte_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_byte_stride(&mut self, byte_stride: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (byte_stride,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_byte_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_indices(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_indices", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indices(&mut self, indices: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_indices", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_attributes(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "get_vertex_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_attributes(&mut self, is_attributes: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (is_attributes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfBufferView", "set_vertex_attributes", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GltfBufferView {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFBufferView"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfBufferView {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfBufferView {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfBufferView {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfBufferView {
        
    }
    impl crate::obj::cap::GodotDefault for GltfBufferView {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfBufferView {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfBufferView {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfBufferView`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfBufferView__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfBufferView > for $Class {
                
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
    use super::re_export::GltfBufferView;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfBufferView {
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