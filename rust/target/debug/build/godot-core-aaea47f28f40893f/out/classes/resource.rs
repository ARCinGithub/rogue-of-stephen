#![doc = "Sidecar module for class [`Resource`][crate::classes::Resource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Resource` enums](https://docs.godotengine.org/en/stable/classes/class_resource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Resource`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`resource`][crate::classes::resource]: sidecar module with related enum/flag types\n* [`IResource`][crate::classes::IResource]: virtual methods\n* [`SignalsOfResource`][crate::classes::resource::SignalsOfResource]: signal collection\n\n\nSee also [Godot docs for `Resource`](https://docs.godotengine.org/en/stable/classes/class_resource.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Resource::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nResource is the base class for all Godot-specific resource types, serving primarily as data containers. Since they inherit from [`RefCounted`][crate::classes::RefCounted], resources are reference-counted and freed when no longer in use. They can also be nested within other resources, and saved on disk. [`PackedScene`][crate::classes::PackedScene], one of the most common [`Object`][crate::classes::Object]s in a Godot project, is also a resource, uniquely capable of storing and instantiating the [`Node`][crate::classes::Node]s it contains as many times as desired.\n\nIn GDScript, resources can loaded from disk by their \\[member resource_path] using [`load`][`crate::tools::load`] or \\[method @GDScript.preload].\n\nThe engine keeps a global cache of all loaded resources, referenced by paths (see [`has_cached`][`crate::classes::ResourceLoader::has_cached`]). A resource will be cached when loaded for the first time and removed from cache once all references are released. When a resource is cached, subsequent loads using its path will return the cached reference.\n\n**Note:** In C#, resources will not be freed instantly after they are no longer in use. Instead, garbage collection will run periodically and will free resources that are no longer in use. This means that unused resources will remain in memory for a while before being removed."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Resource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Resource`][crate::classes::Resource].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Resource` methods](https://docs.godotengine.org/en/stable/classes/class_resource.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResource: crate::obj::GodotClass < Base = Resource > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Resource {
        pub fn set_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member resource_path] to `path`, potentially overriding an existing cache entry for this path. Further attempts to load an overridden resource by path will instead return this resource."]
        pub fn take_over_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "take_over_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the resource's path to `path` without involving the resource cache. Useful for handling \\[enum ResourceFormatLoader.CacheMode] values when implementing a custom resource format by extending [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] and [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]."]
        pub fn set_path_cache(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_path_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of this resource (or an empty RID). Many resources (such as [`Texture2D`][crate::classes::Texture2D], [`Mesh`][crate::classes::Mesh], and so on) are high-level abstractions of resources stored in a specialized server ([`DisplayServer`][crate::classes::DisplayServer], [`RenderingServer`][crate::classes::RenderingServer], etc.), so this function will return the original [`RID`][crate::builtin::Rid]."]
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_local_to_scene(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_local_to_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "is_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If \\[member resource_local_to_scene] is set to `true` and the resource has been loaded from a [`PackedScene`][crate::classes::PackedScene] instantiation, returns the root [`Node`][crate::classes::Node] of the scene where this resource is used. Otherwise, returns `null`."]
        pub fn get_local_scene(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_local_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls [`setup_local_to_scene`][`crate::classes::IResource::setup_local_to_scene`]. If \\[member resource_local_to_scene] is set to `true`, this method is automatically called from [`instantiate`][`crate::classes::PackedScene::instantiate`] by the newly duplicated resource within the scene instance."]
        pub fn setup_local_to_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "setup_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes the resource clear its non-exported properties. See also [`reset_state`][`crate::classes::IResource::reset_state`]. Useful when implementing a custom resource format by extending [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] and [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver]."]
        pub fn reset_state(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "reset_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "In the internal cache for scene-unique IDs, sets the ID of this resource to `id` for the scene at `path`. If `id` is empty, the cache entry for `path` is cleared. Useful to keep scene-unique IDs the same when implementing a VCS-friendly custom resource format by extending [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] and [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver].\n\n**Note:** This method is only implemented when running in an editor context."]
        pub fn set_id_for_path(&mut self, path: impl AsArg < GString >, id: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), id.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "From the internal cache for scene-unique IDs, returns the ID of this resource for the scene at `path`. If there is no entry, an empty string is returned. Useful to keep scene-unique IDs the same when implementing a VCS-friendly custom resource format by extending [`ResourceFormatLoader`][crate::classes::ResourceFormatLoader] and [`ResourceFormatSaver`][crate::classes::ResourceFormatSaver].\n\n**Note:** This method is only implemented when running in an editor context. At runtime, it returns an empty string."]
        pub fn get_id_for_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the resource is saved on disk as a part of another resource's file."]
        pub fn is_built_in(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "is_built_in", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates a unique identifier for a resource to be contained inside a [`PackedScene`][crate::classes::PackedScene], based on the current date, time, and a random value. The returned string is only composed of letters (`a` to `y`) and numbers (`0` to `8`). See also \\[member resource_scene_unique_id]."]
        pub fn generate_scene_unique_id() -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "generate_scene_unique_id", None, args,)
            }
        }
        pub fn set_scene_unique_id(&mut self, id: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_scene_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_unique_id(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_scene_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Emits the `changed` signal. This method is called automatically for some built-in resources.\n\n**Note:** For custom resources, it's recommended to call this method whenever a meaningful change occurs, such as a modified property. This ensures that custom [`Object`][crate::classes::Object]s depending on the resource are properly updated.\n\n```gdscript\nvar damage:\n\tset(new_value):\n\t\tif damage != new_value:\n\t\t\tdamage = new_value\n\t\t\temit_changed()\n```"]
        pub fn emit_changed(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "emit_changed", Some(self.__validated_obj()), args,)
            }
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[doc = "Duplicates this resource, returning a new resource with its `export`ed or [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] properties copied from the original.\n\nIf `deep` is `false`, a **shallow** copy is returned: nested [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], and `Resource` properties are not duplicated and are shared with the original resource.\n\nIf `deep` is `true`, a **deep** copy is returned: all nested arrays, dictionaries, and packed arrays are also duplicated (recursively). Any `Resource` found inside will only be duplicated if it's local, like [`DeepDuplicateMode::INTERNAL`][`crate::classes::resource::DeepDuplicateMode::INTERNAL`] used with [`duplicate_deep`][`crate::classes::Resource::duplicate_deep`].\n\nThe following exceptions apply:\n\n- Subresource properties with the [`PropertyUsageFlags::ALWAYS_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::ALWAYS_DUPLICATE`] flag are always duplicated (recursively or not, depending on `deep`).\n\n- Subresource properties with the [`PropertyUsageFlags::NEVER_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::NEVER_DUPLICATE`] flag are never duplicated.\n\n**Note:** For custom resources, this method will fail if [`init`][`crate::classes::IObject::init`] has been defined with required parameters.\n\n**Note:** When duplicating with `deep` set to `true`, each resource found, including the one on which this method is called, will be only duplicated once and referenced as many times as needed in the duplicate. For instance, if you are duplicating resource A that happens to have resource B referenced twice, you'll get a new resource A' referencing a new resource B' twice."]
        pub(crate) fn duplicate_full(&self, deep: bool,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = (bool,);
            let args = (deep,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[expect(deprecated)]
        #[doc = "To set the default parameters, use [`duplicate_ex`][Self::duplicate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Duplicates this resource, returning a new resource with its `export`ed or [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] properties copied from the original.\n\nIf `deep` is `false`, a **shallow** copy is returned: nested [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], and `Resource` properties are not duplicated and are shared with the original resource.\n\nIf `deep` is `true`, a **deep** copy is returned: all nested arrays, dictionaries, and packed arrays are also duplicated (recursively). Any `Resource` found inside will only be duplicated if it's local, like [`DeepDuplicateMode::INTERNAL`][`crate::classes::resource::DeepDuplicateMode::INTERNAL`] used with [`duplicate_deep`][`crate::classes::Resource::duplicate_deep`].\n\nThe following exceptions apply:\n\n- Subresource properties with the [`PropertyUsageFlags::ALWAYS_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::ALWAYS_DUPLICATE`] flag are always duplicated (recursively or not, depending on `deep`).\n\n- Subresource properties with the [`PropertyUsageFlags::NEVER_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::NEVER_DUPLICATE`] flag are never duplicated.\n\n**Note:** For custom resources, this method will fail if [`init`][`crate::classes::IObject::init`] has been defined with required parameters.\n\n**Note:** When duplicating with `deep` set to `true`, each resource found, including the one on which this method is called, will be only duplicated once and referenced as many times as needed in the duplicate. For instance, if you are duplicating resource A that happens to have resource B referenced twice, you'll get a new resource A' referencing a new resource B' twice."]
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::Resource > > {
            self.duplicate_ex() . done()
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[doc = "Duplicates this resource, returning a new resource with its `export`ed or [`PropertyUsageFlags::STORAGE`][`crate::registry::info::PropertyUsageFlags::STORAGE`] properties copied from the original.\n\nIf `deep` is `false`, a **shallow** copy is returned: nested [`Array`][crate::builtin::Array], [`Dictionary`][crate::builtin::Dictionary], and `Resource` properties are not duplicated and are shared with the original resource.\n\nIf `deep` is `true`, a **deep** copy is returned: all nested arrays, dictionaries, and packed arrays are also duplicated (recursively). Any `Resource` found inside will only be duplicated if it's local, like [`DeepDuplicateMode::INTERNAL`][`crate::classes::resource::DeepDuplicateMode::INTERNAL`] used with [`duplicate_deep`][`crate::classes::Resource::duplicate_deep`].\n\nThe following exceptions apply:\n\n- Subresource properties with the [`PropertyUsageFlags::ALWAYS_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::ALWAYS_DUPLICATE`] flag are always duplicated (recursively or not, depending on `deep`).\n\n- Subresource properties with the [`PropertyUsageFlags::NEVER_DUPLICATE`][`crate::registry::info::PropertyUsageFlags::NEVER_DUPLICATE`] flag are never duplicated.\n\n**Note:** For custom resources, this method will fail if [`init`][`crate::classes::IObject::init`] has been defined with required parameters.\n\n**Note:** When duplicating with `deep` set to `true`, each resource found, including the one on which this method is called, will be only duplicated once and referenced as many times as needed in the duplicate. For instance, if you are duplicating resource A that happens to have resource B referenced twice, you'll get a new resource A' referencing a new resource B' twice."]
        #[inline]
        pub fn duplicate_ex < 'ex > (&'ex self,) -> ExDuplicate < 'ex > {
            ExDuplicate::new(self,)
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[doc = "Duplicates this resource, deeply, like [`duplicate`][`crate::classes::Resource::duplicate`] when passing `true`, with extra control over how subresources are handled."]
        pub(crate) fn duplicate_deep_full(&self, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = (crate::classes::resource::DeepDuplicateMode,);
            let args = (deep_subresources_mode,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "duplicate_deep", Some(self.__validated_obj()), args,)
            }
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[expect(deprecated)]
        #[doc = "To set the default parameters, use [`duplicate_deep_ex`][Self::duplicate_deep_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Duplicates this resource, deeply, like [`duplicate`][`crate::classes::Resource::duplicate`] when passing `true`, with extra control over how subresources are handled."]
        #[inline]
        pub fn duplicate_deep(&self,) -> Option < Gd < crate::classes::Resource > > {
            self.duplicate_deep_ex() . done()
        }
        #[deprecated = "Use `Gd::duplicate_resource()` or `Gd::duplicate_resource_ex()`."]
        #[doc = "Duplicates this resource, deeply, like [`duplicate`][`crate::classes::Resource::duplicate`] when passing `true`, with extra control over how subresources are handled."]
        #[inline]
        pub fn duplicate_deep_ex < 'ex > (&'ex self,) -> ExDuplicateDeep < 'ex > {
            ExDuplicateDeep::new(self,)
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
    impl crate::obj::GodotClass for Resource {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Resource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Resource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Resource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Resource {
        
    }
    impl crate::obj::cap::GodotDefault for Resource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Resource {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Resource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Resource`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Resource__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Resource::duplicate_ex`][super::Resource::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Resource, deep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDuplicate < 'ex > {
    fn new(surround_object: &'ex re_export::Resource,) -> Self {
        let deep = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, deep: deep,
        }
    }
    #[inline]
    pub fn deep(self, deep: bool) -> Self {
        Self {
            deep: deep, .. self
        }
    }
    #[inline]
    #[expect(deprecated)]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, deep,
        }
        = self;
        re_export::Resource::duplicate_full(surround_object, deep,)
    }
}
#[doc = "Default-param extender for [`Resource::duplicate_deep_ex`][super::Resource::duplicate_deep_ex]."]
#[must_use]
pub struct ExDuplicateDeep < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Resource, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDuplicateDeep < 'ex > {
    fn new(surround_object: &'ex re_export::Resource,) -> Self {
        let deep_subresources_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, deep_subresources_mode: deep_subresources_mode,
        }
    }
    #[inline]
    pub fn deep_subresources_mode(self, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode) -> Self {
        Self {
            deep_subresources_mode: deep_subresources_mode, .. self
        }
    }
    #[inline]
    #[expect(deprecated)]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, deep_subresources_mode,
        }
        = self;
        re_export::Resource::duplicate_deep_full(surround_object, deep_subresources_mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeepDuplicateMode {
    ord: i32
}
impl DeepDuplicateMode {
    #[doc(alias = "DEEP_DUPLICATE_NONE")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_NONE`"]
    pub const NONE: DeepDuplicateMode = DeepDuplicateMode {
        ord: 0i32
    };
    #[doc(alias = "DEEP_DUPLICATE_INTERNAL")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_INTERNAL`"]
    pub const INTERNAL: DeepDuplicateMode = DeepDuplicateMode {
        ord: 1i32
    };
    #[doc(alias = "DEEP_DUPLICATE_ALL")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_ALL`"]
    pub const ALL: DeepDuplicateMode = DeepDuplicateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DeepDuplicateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DeepDuplicateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DeepDuplicateMode {
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
            Self::NONE => "NONE", Self::INTERNAL => "INTERNAL", Self::ALL => "ALL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DeepDuplicateMode::NONE, DeepDuplicateMode::INTERNAL, DeepDuplicateMode::ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DeepDuplicateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "DEEP_DUPLICATE_NONE", DeepDuplicateMode::NONE), crate::meta::inspect::EnumConstant::new("INTERNAL", "DEEP_DUPLICATE_INTERNAL", DeepDuplicateMode::INTERNAL), crate::meta::inspect::EnumConstant::new("ALL", "DEEP_DUPLICATE_ALL", DeepDuplicateMode::ALL)]
        }
    }
}
impl crate::meta::GodotConvert for DeepDuplicateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Deep Duplicate None", 0i64), EnumeratorShape::new_int("Deep Duplicate Internal", 1i64), EnumeratorShape::new_int("Deep Duplicate All", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Resource.DeepDuplicateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DeepDuplicateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DeepDuplicateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DeepDuplicateMode {
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
impl crate::registry::property::Export for DeepDuplicateMode {
    
}
impl crate::meta::Element for DeepDuplicateMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Resource;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Resource`][crate::classes::Resource] class."]
    pub struct SignalsOfResource < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfResource < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn setup_local_to_scene_requested(&mut self) -> SigSetupLocalToSceneRequested < 'c, C > {
            SigSetupLocalToSceneRequested {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "setup_local_to_scene_requested")
            }
        }
    }
    type TypedSigChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChanged < 'c, C: WithSignals > {
        typed: TypedSigChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChanged < 'c, C > {
        type Target = TypedSigChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSetupLocalToSceneRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSetupLocalToSceneRequested < 'c, C: WithSignals > {
        typed: TypedSigSetupLocalToSceneRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSetupLocalToSceneRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSetupLocalToSceneRequested < 'c, C > {
        type Target = TypedSigSetupLocalToSceneRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSetupLocalToSceneRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Resource {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfResource < 'c, C > {
        type Target = < < Resource as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Resource;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfResource < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Resource;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}