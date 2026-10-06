#![doc = "Sidecar module for class [`EditorFeatureProfile`][crate::classes::EditorFeatureProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFeatureProfile` enums](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFeatureProfile`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_feature_profile`][crate::classes::editor_feature_profile]: sidecar module with related enum/flag types\n* [`IEditorFeatureProfile`][crate::classes::IEditorFeatureProfile]: virtual methods\n\n\nSee also [Godot docs for `EditorFeatureProfile`](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorFeatureProfile::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAn editor feature profile can be used to disable specific features of the Godot editor. When disabled, the features won't appear in the editor, which makes the editor less cluttered. This is useful in education settings to reduce confusion or when working in a team. For example, artists and level designers could use a feature profile that disables the script editor to avoid accidentally making changes to files they aren't supposed to edit.\n\nTo manage editor feature profiles visually, use **Editor > Manage Feature Profiles...** at the top of the editor window."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFeatureProfile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorFeatureProfile`][crate::classes::EditorFeatureProfile].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorFeatureProfile` methods](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFeatureProfile: crate::obj::GodotClass < Base = EditorFeatureProfile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorFeatureProfile {
        #[doc = "If `disable` is `true`, disables the class specified by `class_name`. When disabled, the class won't appear in the Create New Node dialog."]
        pub fn set_disable_class(&mut self, class_name: impl AsArg < StringName >, disable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class_name.into_arg(), disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(49usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "set_disable_class", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the class specified by `class_name` is disabled. When disabled, the class won't appear in the Create New Node dialog."]
        pub fn is_class_disabled(&self, class_name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(50usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "is_class_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `disable` is `true`, disables editing for the class specified by `class_name`. When disabled, the class will still appear in the Create New Node dialog but the Inspector will be read-only when selecting a node that extends the class."]
        pub fn set_disable_class_editor(&mut self, class_name: impl AsArg < StringName >, disable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class_name.into_arg(), disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(51usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "set_disable_class_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if editing for the class specified by `class_name` is disabled. When disabled, the class will still appear in the Create New Node dialog but the Inspector will be read-only when selecting a node that extends the class."]
        pub fn is_class_editor_disabled(&self, class_name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(52usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "is_class_editor_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `disable` is `true`, disables editing for `property` in the class specified by `class_name`. When a property is disabled, it won't appear in the Inspector when selecting a node that extends the class specified by `class_name`."]
        pub fn set_disable_class_property(&mut self, class_name: impl AsArg < StringName >, property: impl AsArg < StringName >, disable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class_name.into_arg(), property.into_arg(), disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "set_disable_class_property", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `property` is disabled in the class specified by `class_name`. When a property is disabled, it won't appear in the Inspector when selecting a node that extends the class specified by `class_name`."]
        pub fn is_class_property_disabled(&self, class_name: impl AsArg < StringName >, property: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class_name.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "is_class_property_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `disable` is `true`, disables the editor feature specified in `feature`. When a feature is disabled, it will disappear from the editor entirely."]
        pub fn set_disable_feature(&mut self, feature: crate::classes::editor_feature_profile::Feature, disable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::editor_feature_profile::Feature, bool,);
            let args = (feature, disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "set_disable_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the `feature` is disabled. When a feature is disabled, it will disappear from the editor entirely."]
        pub fn is_feature_disabled(&self, feature: crate::classes::editor_feature_profile::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::editor_feature_profile::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "is_feature_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the specified `feature`'s human-readable name."]
        pub fn get_feature_name(&self, feature: crate::classes::editor_feature_profile::Feature,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::editor_feature_profile::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "get_feature_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the editor feature profile to a file in JSON format. It can then be imported using the feature profile manager's **Import** button or the [`load_from_file`][`crate::classes::EditorFeatureProfile::load_from_file`] method.\n\n**Note:** Feature profiles created via the user interface are saved in the `feature_profiles` directory, as a file with the `.profile` extension. The editor configuration folder can be found by using [`get_config_dir`][`crate::classes::EditorPaths::get_config_dir`]."]
        pub fn save_to_file(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "save_to_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Loads an editor feature profile from a file. The file must follow the JSON format obtained by using the feature profile manager's **Export** button or the [`save_to_file`][`crate::classes::EditorFeatureProfile::save_to_file`] method.\n\n**Note:** Feature profiles created via the user interface are loaded from the `feature_profiles` directory, as a file with the `.profile` extension. The editor configuration folder can be found by using [`get_config_dir`][`crate::classes::EditorPaths::get_config_dir`]."]
        pub fn load_from_file(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFeatureProfile", "load_from_file", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorFeatureProfile {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorFeatureProfile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFeatureProfile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorFeatureProfile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorFeatureProfile {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFeatureProfile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFeatureProfile {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFeatureProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorFeatureProfile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorFeatureProfile__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorFeatureProfile > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    pub const FEATURE_3D: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_SCRIPT")]
    #[doc = "Godot enumerator name: `FEATURE_SCRIPT`"]
    pub const SCRIPT: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_ASSET_LIB")]
    #[doc = "Godot enumerator name: `FEATURE_ASSET_LIB`"]
    pub const ASSET_LIB: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_SCENE_TREE")]
    #[doc = "Godot enumerator name: `FEATURE_SCENE_TREE`"]
    pub const SCENE_TREE: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_NODE_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_NODE_DOCK`"]
    pub const NODE_DOCK: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_FILESYSTEM_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_FILESYSTEM_DOCK`"]
    pub const FILESYSTEM_DOCK: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_IMPORT_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_IMPORT_DOCK`"]
    pub const IMPORT_DOCK: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_HISTORY_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_HISTORY_DOCK`"]
    pub const HISTORY_DOCK: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_GAME")]
    #[doc = "Godot enumerator name: `FEATURE_GAME`"]
    pub const GAME: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_SIGNALS_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_SIGNALS_DOCK`"]
    pub const SIGNALS_DOCK: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_GROUPS_DOCK")]
    #[doc = "Godot enumerator name: `FEATURE_GROUPS_DOCK`"]
    pub const GROUPS_DOCK: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_MAX")]
    #[doc = "Godot enumerator name: `FEATURE_MAX`"]
    pub const MAX: Feature = Feature {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::FEATURE_3D => "FEATURE_3D", Self::SCRIPT => "SCRIPT", Self::ASSET_LIB => "ASSET_LIB", Self::SCENE_TREE => "SCENE_TREE", Self::NODE_DOCK => "NODE_DOCK", Self::FILESYSTEM_DOCK => "FILESYSTEM_DOCK", Self::IMPORT_DOCK => "IMPORT_DOCK", Self::HISTORY_DOCK => "HISTORY_DOCK", Self::GAME => "GAME", Self::SIGNALS_DOCK => "SIGNALS_DOCK", Self::GROUPS_DOCK => "GROUPS_DOCK", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::FEATURE_3D, Feature::SCRIPT, Feature::ASSET_LIB, Feature::SCENE_TREE, Feature::NODE_DOCK, Feature::FILESYSTEM_DOCK, Feature::IMPORT_DOCK, Feature::HISTORY_DOCK, Feature::GAME, Feature::SIGNALS_DOCK, Feature::GROUPS_DOCK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FEATURE_3D", "FEATURE_3D", Feature::FEATURE_3D), crate::meta::inspect::EnumConstant::new("SCRIPT", "FEATURE_SCRIPT", Feature::SCRIPT), crate::meta::inspect::EnumConstant::new("ASSET_LIB", "FEATURE_ASSET_LIB", Feature::ASSET_LIB), crate::meta::inspect::EnumConstant::new("SCENE_TREE", "FEATURE_SCENE_TREE", Feature::SCENE_TREE), crate::meta::inspect::EnumConstant::new("NODE_DOCK", "FEATURE_NODE_DOCK", Feature::NODE_DOCK), crate::meta::inspect::EnumConstant::new("FILESYSTEM_DOCK", "FEATURE_FILESYSTEM_DOCK", Feature::FILESYSTEM_DOCK), crate::meta::inspect::EnumConstant::new("IMPORT_DOCK", "FEATURE_IMPORT_DOCK", Feature::IMPORT_DOCK), crate::meta::inspect::EnumConstant::new("HISTORY_DOCK", "FEATURE_HISTORY_DOCK", Feature::HISTORY_DOCK), crate::meta::inspect::EnumConstant::new("GAME", "FEATURE_GAME", Feature::GAME), crate::meta::inspect::EnumConstant::new("SIGNALS_DOCK", "FEATURE_SIGNALS_DOCK", Feature::SIGNALS_DOCK), crate::meta::inspect::EnumConstant::new("GROUPS_DOCK", "FEATURE_GROUPS_DOCK", Feature::GROUPS_DOCK), crate::meta::inspect::EnumConstant::new("MAX", "FEATURE_MAX", Feature::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Feature {
    const ENUMERATOR_COUNT: usize = 11usize;
    
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Feature 3d", 0i64), EnumeratorShape::new_int("Feature Script", 1i64), EnumeratorShape::new_int("Feature Asset Lib", 2i64), EnumeratorShape::new_int("Feature Scene Tree", 3i64), EnumeratorShape::new_int("Feature Node Dock", 4i64), EnumeratorShape::new_int("Feature Filesystem Dock", 5i64), EnumeratorShape::new_int("Feature Import Dock", 6i64), EnumeratorShape::new_int("Feature History Dock", 7i64), EnumeratorShape::new_int("Feature Game", 8i64), EnumeratorShape::new_int("Feature Signals Dock", 9i64), EnumeratorShape::new_int("Feature Groups Dock", 10i64), EnumeratorShape::new_int("Feature Max", 11i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("EditorFeatureProfile.Feature")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Feature {
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
impl crate::registry::property::Export for Feature {
    
}
impl crate::meta::Element for Feature {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorFeatureProfile;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorFeatureProfile {
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