#![doc = "Sidecar module for class [`Crypto`][crate::classes::Crypto].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Crypto` enums](https://docs.godotengine.org/en/stable/classes/class_crypto.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Crypto`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`crypto`][crate::classes::crypto]: sidecar module with related enum/flag types\n* [`ICrypto`][crate::classes::ICrypto]: virtual methods\n\n\nSee also [Godot docs for `Crypto`](https://docs.godotengine.org/en/stable/classes/class_crypto.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Crypto::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThe Crypto class provides access to advanced cryptographic functionalities.\n\nCurrently, this includes asymmetric key encryption/decryption, signing/verification, and generating cryptographically secure random bytes, RSA keys, HMAC digests, and self-signed [`X509Certificate`][crate::classes::X509Certificate]s.\n\n\n```gdscript\nvar crypto = Crypto.new()\n\n# Generate new RSA key.\nvar key = crypto.generate_rsa(4096)\n\n# Generate new self-signed certificate with the given key.\nvar cert = crypto.generate_self_signed_certificate(key, \"CN=mydomain.com,O=My Game Company,C=IT\")\n\n# Save key and certificate in the user folder.\nkey.save(\"user://generated.key\")\ncert.save(\"user://generated.crt\")\n\n# Encryption\nvar data = \"Some data\"\nvar encrypted = crypto.encrypt(key, data.to_utf8_buffer())\n\n# Decryption\nvar decrypted = crypto.decrypt(key, encrypted)\n\n# Signing\nvar signature = crypto.sign(HashingContext.HASH_SHA256, data.sha256_buffer(), key)\n\n# Verifying\nvar verified = crypto.verify(HashingContext.HASH_SHA256, data.sha256_buffer(), signature, key)\n\n# Checks\nassert(verified)\nassert(data.to_utf8_buffer() == decrypted)\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Crypto {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Crypto`][crate::classes::Crypto].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Crypto` methods](https://docs.godotengine.org/en/stable/classes/class_crypto.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICrypto: crate::obj::GodotClass < Base = Crypto > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Crypto {
        #[doc = "Generates a [`PackedByteArray`][crate::builtin::PackedByteArray] of cryptographically secure random bytes with given `size`."]
        pub fn generate_random_bytes(&mut self, size: i32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "generate_random_bytes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates an RSA [`CryptoKey`][crate::classes::CryptoKey] that can be used for creating self-signed certificates and passed to [`accept_stream`][`crate::classes::StreamPeerTls::accept_stream`]."]
        pub fn generate_rsa(&mut self, size: i32,) -> Option < Gd < crate::classes::CryptoKey > > {
            type CallRet = Option < Gd < crate::classes::CryptoKey > >;
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "generate_rsa", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates a self-signed [`X509Certificate`][crate::classes::X509Certificate] from the given [`CryptoKey`][crate::classes::CryptoKey] and `issuer_name`. The certificate validity will be defined by `not_before` and `not_after` (first valid date and last valid date). The `issuer_name` must contain at least \"CN=\" (common name, i.e. the domain name), \"O=\" (organization, i.e. your company name), \"C=\" (country, i.e. 2 lettered ISO-3166 code of the country the organization is based in).\n\nA small example to generate an RSA key and an X509 self-signed certificate.\n\n\n```gdscript\nvar crypto = Crypto.new()\n# Generate 4096 bits RSA key.\nvar key = crypto.generate_rsa(4096)\n# Generate self-signed certificate using the given key.\nvar cert = crypto.generate_self_signed_certificate(key, \"CN=example.com,O=A Game Company,C=IT\")\n```\n"]
        pub(crate) fn generate_self_signed_certificate_full(&mut self, key: CowArg < Option < Gd < crate::classes::CryptoKey > > >, issuer_name: CowArg < GString >, not_before: CowArg < GString >, not_after: CowArg < GString >,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallRet = Option < Gd < crate::classes::X509Certificate > >;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::CryptoKey > > >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >,);
            let args = (key, issuer_name, not_before, not_after,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "generate_self_signed_certificate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`generate_self_signed_certificate_ex`][Self::generate_self_signed_certificate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Generates a self-signed [`X509Certificate`][crate::classes::X509Certificate] from the given [`CryptoKey`][crate::classes::CryptoKey] and `issuer_name`. The certificate validity will be defined by `not_before` and `not_after` (first valid date and last valid date). The `issuer_name` must contain at least \"CN=\" (common name, i.e. the domain name), \"O=\" (organization, i.e. your company name), \"C=\" (country, i.e. 2 lettered ISO-3166 code of the country the organization is based in).\n\nA small example to generate an RSA key and an X509 self-signed certificate.\n\n\n```gdscript\nvar crypto = Crypto.new()\n# Generate 4096 bits RSA key.\nvar key = crypto.generate_rsa(4096)\n# Generate self-signed certificate using the given key.\nvar cert = crypto.generate_self_signed_certificate(key, \"CN=example.com,O=A Game Company,C=IT\")\n```\n"]
        #[inline]
        pub fn generate_self_signed_certificate(&mut self, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >,) -> Option < Gd < crate::classes::X509Certificate > > {
            self.generate_self_signed_certificate_ex(key,) . done()
        }
        #[doc = "Generates a self-signed [`X509Certificate`][crate::classes::X509Certificate] from the given [`CryptoKey`][crate::classes::CryptoKey] and `issuer_name`. The certificate validity will be defined by `not_before` and `not_after` (first valid date and last valid date). The `issuer_name` must contain at least \"CN=\" (common name, i.e. the domain name), \"O=\" (organization, i.e. your company name), \"C=\" (country, i.e. 2 lettered ISO-3166 code of the country the organization is based in).\n\nA small example to generate an RSA key and an X509 self-signed certificate.\n\n\n```gdscript\nvar crypto = Crypto.new()\n# Generate 4096 bits RSA key.\nvar key = crypto.generate_rsa(4096)\n# Generate self-signed certificate using the given key.\nvar cert = crypto.generate_self_signed_certificate(key, \"CN=example.com,O=A Game Company,C=IT\")\n```\n"]
        #[inline]
        pub fn generate_self_signed_certificate_ex < 'ex > (&'ex mut self, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> > + 'ex,) -> ExGenerateSelfSignedCertificate < 'ex > {
            ExGenerateSelfSignedCertificate::new(self, key,)
        }
        #[doc = "Sign a given `hash` of type `hash_type` with the provided private `key`."]
        pub fn sign(&mut self, hash_type: crate::classes::hashing_context::HashType, hash: &PackedByteArray, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (crate::classes::hashing_context::HashType, RefArg < 'a0, PackedByteArray >, CowArg < 'a1, Option < Gd < crate::classes::CryptoKey > > >,);
            let args = (hash_type, RefArg::new(hash), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "sign", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Verify that a given `signature` for `hash` of type `hash_type` against the provided public `key`."]
        pub fn verify(&mut self, hash_type: crate::classes::hashing_context::HashType, hash: &PackedByteArray, signature: &PackedByteArray, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::hashing_context::HashType, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >, CowArg < 'a2, Option < Gd < crate::classes::CryptoKey > > >,);
            let args = (hash_type, RefArg::new(hash), RefArg::new(signature), key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "verify", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Encrypt the given `plaintext` with the provided public `key`.\n\n**Note:** The maximum size of accepted plaintext is limited by the key size."]
        pub fn encrypt(&mut self, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >, plaintext: &PackedByteArray,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::CryptoKey > > >, RefArg < 'a1, PackedByteArray >,);
            let args = (key.into_arg(), RefArg::new(plaintext),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "encrypt", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Decrypt the given `ciphertext` with the provided private `key`.\n\n**Note:** The maximum size of accepted ciphertext is limited by the key size."]
        pub fn decrypt(&mut self, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >, ciphertext: &PackedByteArray,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::CryptoKey > > >, RefArg < 'a1, PackedByteArray >,);
            let args = (key.into_arg(), RefArg::new(ciphertext),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "decrypt", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates an [HMAC](https://en.wikipedia.org/wiki/HMAC) digest of `msg` using `key`. The `hash_type` parameter is the hashing algorithm that is used for the inner and outer hashes.\n\nCurrently, only [`HashType::SHA256`][`crate::classes::hashing_context::HashType::SHA256`] and [`HashType::SHA1`][`crate::classes::hashing_context::HashType::SHA1`] are supported."]
        pub fn hmac_digest(&mut self, hash_type: crate::classes::hashing_context::HashType, key: &PackedByteArray, msg: &PackedByteArray,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (crate::classes::hashing_context::HashType, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >,);
            let args = (hash_type, RefArg::new(key), RefArg::new(msg),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "hmac_digest", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Compares two [`PackedByteArray`][crate::builtin::PackedByteArray]s for equality without leaking timing information in order to prevent timing attacks.\n\nSee [this blog post](https://paragonie.com/blog/2015/11/preventing-timing-attacks-on-string-comparison-with-double-hmac-strategy) for more information."]
        pub fn constant_time_compare(&mut self, trusted: &PackedByteArray, received: &PackedByteArray,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >,);
            let args = (RefArg::new(trusted), RefArg::new(received),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Crypto", "constant_time_compare", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Crypto {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Crypto"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Crypto {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Crypto {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Crypto {
        
    }
    impl crate::obj::cap::GodotDefault for Crypto {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Crypto {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Crypto {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Crypto`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Crypto__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Crypto > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Crypto::generate_self_signed_certificate_ex`][super::Crypto::generate_self_signed_certificate_ex]."]
#[must_use]
pub struct ExGenerateSelfSignedCertificate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Crypto, key: CowArg < 'ex, Option < Gd < crate::classes::CryptoKey > > >, issuer_name: CowArg < 'ex, GString >, not_before: CowArg < 'ex, GString >, not_after: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGenerateSelfSignedCertificate < 'ex > {
    fn new(surround_object: &'ex mut re_export::Crypto, key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> > + 'ex,) -> Self {
        let issuer_name = GString::from("CN=myserver,O=myorganisation,C=IT");
        let not_before = GString::from("20140101000000");
        let not_after = GString::from("20340101000000");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: key.into_arg(), issuer_name: CowArg::Owned(issuer_name), not_before: CowArg::Owned(not_before), not_after: CowArg::Owned(not_after),
        }
    }
    #[inline]
    pub fn issuer_name(self, issuer_name: impl AsArg < GString > + 'ex) -> Self {
        Self {
            issuer_name: issuer_name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn not_before(self, not_before: impl AsArg < GString > + 'ex) -> Self {
        Self {
            not_before: not_before.into_arg(), .. self
        }
    }
    #[inline]
    pub fn not_after(self, not_after: impl AsArg < GString > + 'ex) -> Self {
        Self {
            not_after: not_after.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::X509Certificate > > {
        let Self {
            _phantom, surround_object, key, issuer_name, not_before, not_after,
        }
        = self;
        re_export::Crypto::generate_self_signed_certificate_full(surround_object, key, issuer_name, not_before, not_after,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Crypto;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Crypto {
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