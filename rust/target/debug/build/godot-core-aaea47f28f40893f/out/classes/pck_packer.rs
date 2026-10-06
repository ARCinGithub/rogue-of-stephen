#![doc = "Sidecar module for class [`PckPacker`][crate::classes::PckPacker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PCKPacker` enums](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PCKPacker`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`pck_packer`][crate::classes::pck_packer]: sidecar module with related enum/flag types\n* [`IPckPacker`][crate::classes::IPckPacker]: virtual methods\n\n\nSee also [Godot docs for `PCKPacker`](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PckPacker::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThe `PCKPacker` is used to create packages that can be loaded into a running project using [`load_resource_pack`][`crate::classes::ProjectSettings::load_resource_pack`].\n\n\n```gdscript\nvar packer = PCKPacker.new()\npacker.pck_start(\"test.pck\")\npacker.add_file(\"res://text.txt\", \"text.txt\")\npacker.flush()\n```\n\n\nThe above `PCKPacker` creates package `test.pck`, then adds a file named `text.txt` at the root of the package.\n\n**Note:** PCK is Godot's own pack file format. To create ZIP archives that can be read by any program, use [`ZIPPacker`][crate::classes::ZipPacker] instead."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PckPacker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PckPacker`][crate::classes::PckPacker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `PCKPacker` methods](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPckPacker: crate::obj::GodotClass < Base = PckPacker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PckPacker {
        #[doc = "Creates a new PCK file at the file path `pck_path`. The `.pck` file extension isn't added automatically, so it should be part of `pck_path` (even though it's not required)."]
        pub(crate) fn pck_start_full(&mut self, pck_path: CowArg < GString >, alignment: i32, key: CowArg < GString >, encrypt_directory: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >, bool,);
            let args = (pck_path, alignment, key, encrypt_directory,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PckPacker", "pck_start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`pck_start_ex`][Self::pck_start_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new PCK file at the file path `pck_path`. The `.pck` file extension isn't added automatically, so it should be part of `pck_path` (even though it's not required)."]
        #[inline]
        pub fn pck_start(&mut self, pck_path: impl AsArg < GString >,) -> crate::global::Error {
            self.pck_start_ex(pck_path,) . done()
        }
        #[doc = "Creates a new PCK file at the file path `pck_path`. The `.pck` file extension isn't added automatically, so it should be part of `pck_path` (even though it's not required)."]
        #[inline]
        pub fn pck_start_ex < 'ex > (&'ex mut self, pck_path: impl AsArg < GString > + 'ex,) -> ExPckStart < 'ex > {
            ExPckStart::new(self, pck_path,)
        }
        #[doc = "Adds the `source_path` file to the current PCK package at the `target_path` internal path. The `res://` prefix for `target_path` is optional and stripped internally. File content is immediately written to the PCK."]
        pub(crate) fn add_file_full(&mut self, target_path: CowArg < GString >, source_path: CowArg < GString >, encrypt: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (target_path, source_path, encrypt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PckPacker", "add_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_file_ex`][Self::add_file_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds the `source_path` file to the current PCK package at the `target_path` internal path. The `res://` prefix for `target_path` is optional and stripped internally. File content is immediately written to the PCK."]
        #[inline]
        pub fn add_file(&mut self, target_path: impl AsArg < GString >, source_path: impl AsArg < GString >,) -> crate::global::Error {
            self.add_file_ex(target_path, source_path,) . done()
        }
        #[doc = "Adds the `source_path` file to the current PCK package at the `target_path` internal path. The `res://` prefix for `target_path` is optional and stripped internally. File content is immediately written to the PCK."]
        #[inline]
        pub fn add_file_ex < 'ex > (&'ex mut self, target_path: impl AsArg < GString > + 'ex, source_path: impl AsArg < GString > + 'ex,) -> ExAddFile < 'ex > {
            ExAddFile::new(self, target_path, source_path,)
        }
        #[doc = "Registers a file removal of the `target_path` internal path to the PCK. This is mainly used for patches. If the file at this path has been loaded from a previous PCK, it will be removed. The `res://` prefix for `target_path` is optional and stripped internally."]
        pub fn add_file_removal(&mut self, target_path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (target_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PckPacker", "add_file_removal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Writes the file directory and closes the PCK. If `verbose` is `true`, a list of files added will be printed to the console for easier debugging.\n\n**Note:** `PCKPacker` will automatically flush when it's freed, which happens when it goes out of scope or when it gets assigned with `null`. In C# the reference must be disposed after use, either with the `using` statement or by calling the `Dispose` method directly."]
        pub(crate) fn flush_full(&mut self, verbose: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (bool,);
            let args = (verbose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PckPacker", "flush", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`flush_ex`][Self::flush_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Writes the file directory and closes the PCK. If `verbose` is `true`, a list of files added will be printed to the console for easier debugging.\n\n**Note:** `PCKPacker` will automatically flush when it's freed, which happens when it goes out of scope or when it gets assigned with `null`. In C# the reference must be disposed after use, either with the `using` statement or by calling the `Dispose` method directly."]
        #[inline]
        pub fn flush(&mut self,) -> crate::global::Error {
            self.flush_ex() . done()
        }
        #[doc = "Writes the file directory and closes the PCK. If `verbose` is `true`, a list of files added will be printed to the console for easier debugging.\n\n**Note:** `PCKPacker` will automatically flush when it's freed, which happens when it goes out of scope or when it gets assigned with `null`. In C# the reference must be disposed after use, either with the `using` statement or by calling the `Dispose` method directly."]
        #[inline]
        pub fn flush_ex < 'ex > (&'ex mut self,) -> ExFlush < 'ex > {
            ExFlush::new(self,)
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
    impl crate::obj::GodotClass for PckPacker {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PCKPacker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PckPacker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PckPacker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PckPacker {
        
    }
    impl crate::obj::cap::GodotDefault for PckPacker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PckPacker {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PckPacker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PckPacker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PckPacker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PckPacker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PckPacker::pck_start_ex`][super::PckPacker::pck_start_ex]."]
#[must_use]
pub struct ExPckStart < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PckPacker, pck_path: CowArg < 'ex, GString >, alignment: i32, key: CowArg < 'ex, GString >, encrypt_directory: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPckStart < 'ex > {
    fn new(surround_object: &'ex mut re_export::PckPacker, pck_path: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = 32i32;
        let key = GString::from("0000000000000000000000000000000000000000000000000000000000000000");
        let encrypt_directory = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pck_path: pck_path.into_arg(), alignment: alignment, key: CowArg::Owned(key), encrypt_directory: encrypt_directory,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: i32) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn key(self, key: impl AsArg < GString > + 'ex) -> Self {
        Self {
            key: key.into_arg(), .. self
        }
    }
    #[inline]
    pub fn encrypt_directory(self, encrypt_directory: bool) -> Self {
        Self {
            encrypt_directory: encrypt_directory, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, pck_path, alignment, key, encrypt_directory,
        }
        = self;
        re_export::PckPacker::pck_start_full(surround_object, pck_path, alignment, key, encrypt_directory,)
    }
}
#[doc = "Default-param extender for [`PckPacker::add_file_ex`][super::PckPacker::add_file_ex]."]
#[must_use]
pub struct ExAddFile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PckPacker, target_path: CowArg < 'ex, GString >, source_path: CowArg < 'ex, GString >, encrypt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddFile < 'ex > {
    fn new(surround_object: &'ex mut re_export::PckPacker, target_path: impl AsArg < GString > + 'ex, source_path: impl AsArg < GString > + 'ex,) -> Self {
        let encrypt = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, target_path: target_path.into_arg(), source_path: source_path.into_arg(), encrypt: encrypt,
        }
    }
    #[inline]
    pub fn encrypt(self, encrypt: bool) -> Self {
        Self {
            encrypt: encrypt, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, target_path, source_path, encrypt,
        }
        = self;
        re_export::PckPacker::add_file_full(surround_object, target_path, source_path, encrypt,)
    }
}
#[doc = "Default-param extender for [`PckPacker::flush_ex`][super::PckPacker::flush_ex]."]
#[must_use]
pub struct ExFlush < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PckPacker, verbose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFlush < 'ex > {
    fn new(surround_object: &'ex mut re_export::PckPacker,) -> Self {
        let verbose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, verbose: verbose,
        }
    }
    #[inline]
    pub fn verbose(self, verbose: bool) -> Self {
        Self {
            verbose: verbose, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, verbose,
        }
        = self;
        re_export::PckPacker::flush_full(surround_object, verbose,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PckPacker;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PckPacker {
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