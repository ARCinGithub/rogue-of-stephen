#![doc = "Sidecar module for class [`TextureLayered`][crate::classes::TextureLayered].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureLayered` enums](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureLayered`.\n\nInherits [`Texture`][crate::classes::Texture].\n\nRelated symbols:\n\n* [`texture_layered`][crate::classes::texture_layered]: sidecar module with related enum/flag types\n* [`ITextureLayered`][crate::classes::ITextureLayered]: virtual methods\n\n\nSee also [Godot docs for `TextureLayered`](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextureLayered::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBase class for [`ImageTextureLayered`][crate::classes::ImageTextureLayered] and [`CompressedTextureLayered`][crate::classes::CompressedTextureLayered]. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also [`Texture3D`][crate::classes::Texture3D].\n\nData is set on a per-layer basis. For [`Texture2DArray`][crate::classes::Texture2DArray]s, the layer specifies the array layer.\n\nAll images need to have the same width, height and number of mipmap levels.\n\nA `TextureLayered` can be loaded with [`load`][`crate::classes::ResourceLoader::load`].\n\nInternally, Godot maps these files to their respective counterparts in the target rendering driver (Vulkan, OpenGL3)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureLayered {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextureLayered`][crate::classes::TextureLayered].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TextureLayered` methods](https://docs.godotengine.org/en/stable/classes/class_texturelayered.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextureLayered: crate::obj::GodotClass < Base = TextureLayered > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Called when the `TextureLayered`'s format is queried."]
        fn get_format(&self,) -> crate::classes::image::Format;
        #[doc = "Called when the layers' type in the `TextureLayered` is queried."]
        fn get_layered_type(&self,) -> u32;
        #[doc = "Called when the `TextureLayered`'s width queried."]
        fn get_width(&self,) -> i32;
        #[doc = "Called when the `TextureLayered`'s height is queried."]
        fn get_height(&self,) -> i32;
        #[doc = "Called when the number of layers in the `TextureLayered` is queried."]
        fn get_layers(&self,) -> i32;
        #[doc = "Called when the presence of mipmaps in the `TextureLayered` is queried."]
        fn has_mipmaps(&self,) -> bool;
        #[doc = "Called when the data for a layer in the `TextureLayered` is queried."]
        fn get_layer_data(&self, layer_index: i32,) -> Option < Gd < crate::classes::Image > >;
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
    impl TextureLayered {
        #[doc = "Returns the current format being used by this texture."]
        pub fn get_format(&self,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `TextureLayered`'s type. The type determines how the data is accessed, with cubemaps having special types."]
        pub fn get_layered_type(&self,) -> crate::classes::texture_layered::LayeredType {
            type CallRet = crate::classes::texture_layered::LayeredType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_layered_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the width of the texture in pixels. Width is typically represented by the X axis."]
        pub fn get_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the height of the texture in pixels. Height is typically represented by the Y axis."]
        pub fn get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of referenced [`Image`][crate::classes::Image]s."]
        pub fn get_layers(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_layers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the layers have generated mipmaps."]
        pub fn has_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "has_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] resource with the data from specified `layer`."]
        pub fn get_layer_data(&self, layer: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayered", "get_layer_data", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextureLayered {
        type Base = crate::classes::Texture;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextureLayered"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureLayered {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for TextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextureLayered {
        
    }
    impl crate::obj::cap::GodotDefault for TextureLayered {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextureLayered {
        type Target = crate::classes::Texture;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureLayered {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextureLayered`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextureLayered__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextureLayered > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayeredType {
    ord: i32
}
impl LayeredType {
    #[doc(alias = "LAYERED_TYPE_2D_ARRAY")]
    #[doc = "Godot enumerator name: `LAYERED_TYPE_2D_ARRAY`"]
    pub const TYPE_2D_ARRAY: LayeredType = LayeredType {
        ord: 0i32
    };
    #[doc(alias = "LAYERED_TYPE_CUBEMAP")]
    #[doc = "Godot enumerator name: `LAYERED_TYPE_CUBEMAP`"]
    pub const CUBEMAP: LayeredType = LayeredType {
        ord: 1i32
    };
    #[doc(alias = "LAYERED_TYPE_CUBEMAP_ARRAY")]
    #[doc = "Godot enumerator name: `LAYERED_TYPE_CUBEMAP_ARRAY`"]
    pub const CUBEMAP_ARRAY: LayeredType = LayeredType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LayeredType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayeredType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayeredType {
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
            Self::TYPE_2D_ARRAY => "TYPE_2D_ARRAY", Self::CUBEMAP => "CUBEMAP", Self::CUBEMAP_ARRAY => "CUBEMAP_ARRAY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LayeredType::TYPE_2D_ARRAY, LayeredType::CUBEMAP, LayeredType::CUBEMAP_ARRAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LayeredType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TYPE_2D_ARRAY", "LAYERED_TYPE_2D_ARRAY", LayeredType::TYPE_2D_ARRAY), crate::meta::inspect::EnumConstant::new("CUBEMAP", "LAYERED_TYPE_CUBEMAP", LayeredType::CUBEMAP), crate::meta::inspect::EnumConstant::new("CUBEMAP_ARRAY", "LAYERED_TYPE_CUBEMAP_ARRAY", LayeredType::CUBEMAP_ARRAY)]
        }
    }
}
impl crate::meta::GodotConvert for LayeredType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Layered Type 2d Array", 0i64), EnumeratorShape::new_int("Layered Type Cubemap", 1i64), EnumeratorShape::new_int("Layered Type Cubemap Array", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TextureLayered.LayeredType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LayeredType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayeredType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LayeredType {
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
impl crate::registry::property::Export for LayeredType {
    
}
impl crate::meta::Element for LayeredType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextureLayered;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TextureLayered {
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