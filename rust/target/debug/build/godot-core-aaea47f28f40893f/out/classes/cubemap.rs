#![doc = "Sidecar module for class [`Cubemap`][crate::classes::Cubemap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Cubemap` enums](https://docs.godotengine.org/en/stable/classes/class_cubemap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Cubemap`.\n\nInherits [`ImageTextureLayered`][crate::classes::ImageTextureLayered].\n\nRelated symbols:\n\n* [`ICubemap`][crate::classes::ICubemap]: virtual methods\n\n\nSee also [Godot docs for `Cubemap`](https://docs.godotengine.org/en/stable/classes/class_cubemap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Cubemap::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA cubemap is made of 6 textures organized in layers. They are typically used for faking reflections in 3D rendering (see [`ReflectionProbe`][crate::classes::ReflectionProbe]). It can be used to make an object look as if it's reflecting its surroundings. This usually delivers much better performance than other reflection methods.\n\nThis resource is typically used as a uniform in custom shaders. Few core Godot methods make use of `Cubemap` resources.\n\nTo create such a texture file yourself, reimport your image files using the Godot Editor import presets. To create a Cubemap from code, use [`create_from_images`][`crate::classes::ImageTextureLayered::create_from_images`] on an instance of the Cubemap class.\n\nThe expected image order is X+, X-, Y+, Y-, Z+, Z- (in Godot's coordinate system, so Y+ is \"up\" and Z- is \"forward\"). You can use one of the following templates as a base:\n\n- [2×3 cubemap template (default layout option)](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/assets_pipeline/img/cubemap_template_2x3.webp)\n\n- [3×2 cubemap template](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/assets_pipeline/img/cubemap_template_3x2.webp)\n\n- [1×6 cubemap template](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/assets_pipeline/img/cubemap_template_1x6.webp)\n\n- [6×1 cubemap template](https://raw.githubusercontent.com/godotengine/godot-docs/master/tutorials/assets_pipeline/img/cubemap_template_6x1.webp)\n\n**Note:** Godot doesn't support using cubemaps in a [`PanoramaSkyMaterial`][crate::classes::PanoramaSkyMaterial]. To use a cubemap as a skybox, convert the default [`PanoramaSkyMaterial`][crate::classes::PanoramaSkyMaterial] to a [`ShaderMaterial`][crate::classes::ShaderMaterial] using the **Convert to ShaderMaterial** resource dropdown option, then replace its code with the following:\n\n```text\nshader_type sky;\n\nuniform samplerCube source_panorama : filter_linear, source_color, hint_default_black;\nuniform float exposure : hint_range(0, 128) = 1.0;\n\nvoid sky() {\n\t// If importing a cubemap from another engine, you may need to flip one of the `EYEDIR` components below\n\t// by replacing it with `-EYEDIR`.\n\tvec3 eyedir = vec3(EYEDIR.x, EYEDIR.y, EYEDIR.z);\n\tCOLOR = texture(source_panorama, eyedir).rgb * exposure;\n}\n```\n\nAfter replacing the shader code and saving, specify the imported Cubemap resource in the Shader Parameters section of the ShaderMaterial in the inspector.\n\nAlternatively, you can use [this tool](https://danilw.github.io/GLSL-howto/cubemap_to_panorama_js/cubemap_to_panorama.html) to convert a cubemap to an equirectangular sky map and use [`PanoramaSkyMaterial`][crate::classes::PanoramaSkyMaterial] as usual."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Cubemap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Cubemap`][crate::classes::Cubemap].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IImageTextureLayered`~~ > [`ITextureLayered`][crate::classes::ITextureLayered] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Cubemap` methods](https://docs.godotengine.org/en/stable/classes/class_cubemap.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICubemap: crate::obj::GodotClass < Base = Cubemap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Cubemap {
        #[doc = "Creates a placeholder version of this resource ([`PlaceholderCubemap`][crate::classes::PlaceholderCubemap])."]
        pub fn create_placeholder(&self,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Cubemap", "create_placeholder", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Cubemap {
        type Base = crate::classes::ImageTextureLayered;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Cubemap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Cubemap {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ImageTextureLayered > for Cubemap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextureLayered > for Cubemap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for Cubemap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Cubemap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Cubemap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Cubemap {
        
    }
    impl crate::obj::cap::GodotDefault for Cubemap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Cubemap {
        type Target = crate::classes::ImageTextureLayered;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Cubemap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Cubemap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Cubemap__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Cubemap > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ImageTextureLayered > for $Class {
                
            }
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Cubemap;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Cubemap {
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