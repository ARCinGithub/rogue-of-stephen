#![doc = "Sidecar module for class [`AesContext`][crate::classes::AesContext].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AESContext` enums](https://docs.godotengine.org/en/stable/classes/class_aescontext.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AESContext`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`aes_context`][crate::classes::aes_context]: sidecar module with related enum/flag types\n* [`IAesContext`][crate::classes::IAesContext]: virtual methods\n\n\nSee also [Godot docs for `AESContext`](https://docs.godotengine.org/en/stable/classes/class_aescontext.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AesContext::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class holds the context information required for encryption and decryption operations with AES (Advanced Encryption Standard). Both AES-ECB and AES-CBC modes are supported.\n\n\n```gdscript\nextends Node\n\nvar aes = AESContext.new()\n\nfunc _ready():\n\tvar key = \"My secret key!!!\" # Key must be either 16 or 32 bytes.\n\tvar data = \"My secret text!!\" # Data size must be multiple of 16 bytes, apply padding if needed.\n\t# Encrypt ECB\n\taes.start(AESContext.MODE_ECB_ENCRYPT, key.to_utf8_buffer())\n\tvar encrypted = aes.update(data.to_utf8_buffer())\n\taes.finish()\n\t# Decrypt ECB\n\taes.start(AESContext.MODE_ECB_DECRYPT, key.to_utf8_buffer())\n\tvar decrypted = aes.update(encrypted)\n\taes.finish()\n\t# Check ECB\n\tassert(decrypted == data.to_utf8_buffer())\n\n\tvar iv = \"My secret iv!!!!\" # IV must be of exactly 16 bytes.\n\t# Encrypt CBC\n\taes.start(AESContext.MODE_CBC_ENCRYPT, key.to_utf8_buffer(), iv.to_utf8_buffer())\n\tencrypted = aes.update(data.to_utf8_buffer())\n\taes.finish()\n\t# Decrypt CBC\n\taes.start(AESContext.MODE_CBC_DECRYPT, key.to_utf8_buffer(), iv.to_utf8_buffer())\n\tdecrypted = aes.update(encrypted)\n\taes.finish()\n\t# Check CBC\n\tassert(decrypted == data.to_utf8_buffer())\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AesContext {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AesContext`][crate::classes::AesContext].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AESContext` methods](https://docs.godotengine.org/en/stable/classes/class_aescontext.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAesContext: crate::obj::GodotClass < Base = AesContext > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AesContext {
        #[doc = "Start the AES context in the given `mode`. A `key` of either 16 or 32 bytes must always be provided, while an `iv` (initialization vector) of exactly 16 bytes, is only needed when `mode` is either [`Mode::CBC_ENCRYPT`][`crate::classes::aes_context::Mode::CBC_ENCRYPT`] or [`Mode::CBC_DECRYPT`][`crate::classes::aes_context::Mode::CBC_DECRYPT`]."]
        pub(crate) fn start_full(&mut self, mode: crate::classes::aes_context::Mode, key: RefArg < PackedByteArray >, iv: RefArg < PackedByteArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (crate::classes::aes_context::Mode, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >,);
            let args = (mode, key, iv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AesContext", "start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`start_ex`][Self::start_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Start the AES context in the given `mode`. A `key` of either 16 or 32 bytes must always be provided, while an `iv` (initialization vector) of exactly 16 bytes, is only needed when `mode` is either [`Mode::CBC_ENCRYPT`][`crate::classes::aes_context::Mode::CBC_ENCRYPT`] or [`Mode::CBC_DECRYPT`][`crate::classes::aes_context::Mode::CBC_DECRYPT`]."]
        #[inline]
        pub fn start(&mut self, mode: crate::classes::aes_context::Mode, key: &PackedByteArray,) -> crate::global::Error {
            self.start_ex(mode, key,) . done()
        }
        #[doc = "Start the AES context in the given `mode`. A `key` of either 16 or 32 bytes must always be provided, while an `iv` (initialization vector) of exactly 16 bytes, is only needed when `mode` is either [`Mode::CBC_ENCRYPT`][`crate::classes::aes_context::Mode::CBC_ENCRYPT`] or [`Mode::CBC_DECRYPT`][`crate::classes::aes_context::Mode::CBC_DECRYPT`]."]
        #[inline]
        pub fn start_ex < 'ex > (&'ex mut self, mode: crate::classes::aes_context::Mode, key: &'ex PackedByteArray,) -> ExStart < 'ex > {
            ExStart::new(self, mode, key,)
        }
        #[doc = "Run the desired operation for this AES context. Will return a [`PackedByteArray`][crate::builtin::PackedByteArray] containing the result of encrypting (or decrypting) the given `src`. See [`start`][`crate::classes::AesContext::start`] for mode of operation.\n\n**Note:** The size of `src` must be a multiple of 16. Apply some padding if needed."]
        pub fn update(&mut self, src: &PackedByteArray,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(src),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AesContext", "update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Get the current IV state for this context (IV gets updated when calling [`update`][`crate::classes::AesContext::update`]). You normally don't need this function.\n\n**Note:** This function only makes sense when the context is started with [`Mode::CBC_ENCRYPT`][`crate::classes::aes_context::Mode::CBC_ENCRYPT`] or [`Mode::CBC_DECRYPT`][`crate::classes::aes_context::Mode::CBC_DECRYPT`]."]
        pub fn get_iv_state(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AesContext", "get_iv_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Close this AES context so it can be started again. See [`start`][`crate::classes::AesContext::start`]."]
        pub fn finish(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AesContext", "finish", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AesContext {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AESContext"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AesContext {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AesContext {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AesContext {
        
    }
    impl crate::obj::cap::GodotDefault for AesContext {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AesContext {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AesContext {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AesContext`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AesContext__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AesContext > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AesContext::start_ex`][super::AesContext::start_ex]."]
#[must_use]
pub struct ExStart < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AesContext, mode: crate::classes::aes_context::Mode, key: CowArg < 'ex, PackedByteArray >, iv: CowArg < 'ex, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExStart < 'ex > {
    fn new(surround_object: &'ex mut re_export::AesContext, mode: crate::classes::aes_context::Mode, key: &'ex PackedByteArray,) -> Self {
        let iv = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, key: CowArg::Borrowed(key), iv: CowArg::Owned(iv),
        }
    }
    #[inline]
    pub fn iv(self, iv: &'ex PackedByteArray) -> Self {
        Self {
            iv: CowArg::Borrowed(iv), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mode, key, iv,
        }
        = self;
        re_export::AesContext::start_full(surround_object, mode, key.cow_as_arg(), iv.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_ECB_ENCRYPT")]
    #[doc = "Godot enumerator name: `MODE_ECB_ENCRYPT`"]
    pub const ECB_ENCRYPT: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_ECB_DECRYPT")]
    #[doc = "Godot enumerator name: `MODE_ECB_DECRYPT`"]
    pub const ECB_DECRYPT: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_CBC_ENCRYPT")]
    #[doc = "Godot enumerator name: `MODE_CBC_ENCRYPT`"]
    pub const CBC_ENCRYPT: Mode = Mode {
        ord: 2i32
    };
    #[doc(alias = "MODE_CBC_DECRYPT")]
    #[doc = "Godot enumerator name: `MODE_CBC_DECRYPT`"]
    pub const CBC_DECRYPT: Mode = Mode {
        ord: 3i32
    };
    #[doc(alias = "MODE_MAX")]
    #[doc = "Godot enumerator name: `MODE_MAX`"]
    pub const MAX: Mode = Mode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Mode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Mode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::ECB_ENCRYPT => "ECB_ENCRYPT", Self::ECB_DECRYPT => "ECB_DECRYPT", Self::CBC_ENCRYPT => "CBC_ENCRYPT", Self::CBC_DECRYPT => "CBC_DECRYPT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::ECB_ENCRYPT, Mode::ECB_DECRYPT, Mode::CBC_ENCRYPT, Mode::CBC_DECRYPT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ECB_ENCRYPT", "MODE_ECB_ENCRYPT", Mode::ECB_ENCRYPT), crate::meta::inspect::EnumConstant::new("ECB_DECRYPT", "MODE_ECB_DECRYPT", Mode::ECB_DECRYPT), crate::meta::inspect::EnumConstant::new("CBC_ENCRYPT", "MODE_CBC_ENCRYPT", Mode::CBC_ENCRYPT), crate::meta::inspect::EnumConstant::new("CBC_DECRYPT", "MODE_CBC_DECRYPT", Mode::CBC_DECRYPT), crate::meta::inspect::EnumConstant::new("MAX", "MODE_MAX", Mode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Mode {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mode Ecb Encrypt", 0i64), EnumeratorShape::new_int("Mode Ecb Decrypt", 1i64), EnumeratorShape::new_int("Mode Cbc Encrypt", 2i64), EnumeratorShape::new_int("Mode Cbc Decrypt", 3i64), EnumeratorShape::new_int("Mode Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AESContext.Mode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Mode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Mode {
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
impl crate::registry::property::Export for Mode {
    
}
impl crate::meta::Element for Mode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AesContext;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AesContext {
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