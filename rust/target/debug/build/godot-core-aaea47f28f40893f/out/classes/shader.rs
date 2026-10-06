#![doc = "Sidecar module for class [`Shader`][crate::classes::Shader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Shader` enums](https://docs.godotengine.org/en/stable/classes/class_shader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Shader`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`shader`][crate::classes::shader]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Shader`](https://docs.godotengine.org/en/stable/classes/class_shader.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Shader::new_gd()`][crate::obj::NewGd::new_gd].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nA custom shader program implemented in the Godot shading language, saved with the `.gdshader` extension.\n\nThis class is used by a [`ShaderMaterial`][crate::classes::ShaderMaterial] and allows you to write your own custom behavior for rendering visual items or updating particle information. For a detailed explanation and usage, please see the tutorials linked below."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Shader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Shader {
        #[doc = "Returns the shader mode for the shader."]
        pub fn get_mode(&self,) -> crate::classes::shader::Mode {
            type CallRet = crate::classes::shader::Mode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code(&mut self, code: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (code.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "set_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "get_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default texture to be used with a texture uniform. The default is used if a texture is not set in the [`ShaderMaterial`][crate::classes::ShaderMaterial].\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        pub(crate) fn set_default_texture_parameter_full(&mut self, name: CowArg < StringName >, texture: CowArg < Option < Gd < crate::classes::Texture > > >, index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Texture > > >, i32,);
            let args = (name, texture, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "set_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_default_texture_parameter_ex`][Self::set_default_texture_parameter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the default texture to be used with a texture uniform. The default is used if a texture is not set in the [`ShaderMaterial`][crate::classes::ShaderMaterial].\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn set_default_texture_parameter(&mut self, name: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::Texture >> >,) {
            self.set_default_texture_parameter_ex(name, texture,) . done()
        }
        #[doc = "Sets the default texture to be used with a texture uniform. The default is used if a texture is not set in the [`ShaderMaterial`][crate::classes::ShaderMaterial].\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn set_default_texture_parameter_ex < 'ex > (&'ex mut self, name: impl AsArg < StringName > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture >> > + 'ex,) -> ExSetDefaultTextureParameter < 'ex > {
            ExSetDefaultTextureParameter::new(self, name, texture,)
        }
        #[doc = "Returns the texture that is set as default for the specified parameter.\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        pub(crate) fn get_default_texture_parameter_full(&self, name: CowArg < StringName >, index: i32,) -> Option < Gd < crate::classes::Texture > > {
            type CallRet = Option < Gd < crate::classes::Texture > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (name, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "get_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_default_texture_parameter_ex`][Self::get_default_texture_parameter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the texture that is set as default for the specified parameter.\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn get_default_texture_parameter(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture > > {
            self.get_default_texture_parameter_ex(name,) . done()
        }
        #[doc = "Returns the texture that is set as default for the specified parameter.\n\n**Note:** `name` must match the name of the uniform in the code exactly.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn get_default_texture_parameter_ex < 'ex > (&'ex self, name: impl AsArg < StringName > + 'ex,) -> ExGetDefaultTextureParameter < 'ex > {
            ExGetDefaultTextureParameter::new(self, name,)
        }
        #[doc = "Returns the list of shader uniforms that can be assigned to a [`ShaderMaterial`][crate::classes::ShaderMaterial], for use with [`set_shader_parameter`][`crate::classes::ShaderMaterial::set_shader_parameter`] and [`get_shader_parameter`][`crate::classes::ShaderMaterial::get_shader_parameter`]. The parameters returned are contained in dictionaries in a similar format to the ones returned by [`get_property_list`][`crate::classes::Object::get_property_list`].\n\nIf argument `get_groups` is `true`, parameter grouping hints are also included in the list."]
        pub(crate) fn get_shader_uniform_list_full(&self, get_groups: bool,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (bool,);
            let args = (get_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "get_shader_uniform_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_shader_uniform_list_ex`][Self::get_shader_uniform_list_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the list of shader uniforms that can be assigned to a [`ShaderMaterial`][crate::classes::ShaderMaterial], for use with [`set_shader_parameter`][`crate::classes::ShaderMaterial::set_shader_parameter`] and [`get_shader_parameter`][`crate::classes::ShaderMaterial::get_shader_parameter`]. The parameters returned are contained in dictionaries in a similar format to the ones returned by [`get_property_list`][`crate::classes::Object::get_property_list`].\n\nIf argument `get_groups` is `true`, parameter grouping hints are also included in the list."]
        #[inline]
        pub fn get_shader_uniform_list(&self,) -> VarArray {
            self.get_shader_uniform_list_ex() . done()
        }
        #[doc = "Returns the list of shader uniforms that can be assigned to a [`ShaderMaterial`][crate::classes::ShaderMaterial], for use with [`set_shader_parameter`][`crate::classes::ShaderMaterial::set_shader_parameter`] and [`get_shader_parameter`][`crate::classes::ShaderMaterial::get_shader_parameter`]. The parameters returned are contained in dictionaries in a similar format to the ones returned by [`get_property_list`][`crate::classes::Object::get_property_list`].\n\nIf argument `get_groups` is `true`, parameter grouping hints are also included in the list."]
        #[inline]
        pub fn get_shader_uniform_list_ex < 'ex > (&'ex self,) -> ExGetShaderUniformList < 'ex > {
            ExGetShaderUniformList::new(self,)
        }
        #[doc = "Only available when running in the editor. Opens a popup that visualizes the generated shader code, including all variants and internal shader code. See also [`inspect_native_shader_code`][`crate::classes::Material::inspect_native_shader_code`]."]
        pub fn inspect_native_shader_code(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shader", "inspect_native_shader_code", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Shader {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Shader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Shader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Shader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Shader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Shader {
        
    }
    impl crate::obj::cap::GodotDefault for Shader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Shader {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Shader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Shader__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Shader` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Shader::set_default_texture_parameter_ex`][super::Shader::set_default_texture_parameter_ex]."]
#[must_use]
pub struct ExSetDefaultTextureParameter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Shader, name: CowArg < 'ex, StringName >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture > > >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetDefaultTextureParameter < 'ex > {
    fn new(surround_object: &'ex mut re_export::Shader, name: impl AsArg < StringName > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture >> > + 'ex,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), texture: texture.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, texture, index,
        }
        = self;
        re_export::Shader::set_default_texture_parameter_full(surround_object, name, texture, index,)
    }
}
#[doc = "Default-param extender for [`Shader::get_default_texture_parameter_ex`][super::Shader::get_default_texture_parameter_ex]."]
#[must_use]
pub struct ExGetDefaultTextureParameter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Shader, name: CowArg < 'ex, StringName >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDefaultTextureParameter < 'ex > {
    fn new(surround_object: &'ex re_export::Shader, name: impl AsArg < StringName > + 'ex,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Texture > > {
        let Self {
            _phantom, surround_object, name, index,
        }
        = self;
        re_export::Shader::get_default_texture_parameter_full(surround_object, name, index,)
    }
}
#[doc = "Default-param extender for [`Shader::get_shader_uniform_list_ex`][super::Shader::get_shader_uniform_list_ex]."]
#[must_use]
pub struct ExGetShaderUniformList < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Shader, get_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetShaderUniformList < 'ex > {
    fn new(surround_object: &'ex re_export::Shader,) -> Self {
        let get_groups = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, get_groups: get_groups,
        }
    }
    #[inline]
    pub fn get_groups(self, get_groups: bool) -> Self {
        Self {
            get_groups: get_groups, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarArray {
        let Self {
            _phantom, surround_object, get_groups,
        }
        = self;
        re_export::Shader::get_shader_uniform_list_full(surround_object, get_groups,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_SPATIAL")]
    #[doc = "Godot enumerator name: `MODE_SPATIAL`"]
    pub const SPATIAL: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_CANVAS_ITEM")]
    #[doc = "Godot enumerator name: `MODE_CANVAS_ITEM`"]
    pub const CANVAS_ITEM: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_PARTICLES")]
    #[doc = "Godot enumerator name: `MODE_PARTICLES`"]
    pub const PARTICLES: Mode = Mode {
        ord: 2i32
    };
    #[doc(alias = "MODE_SKY")]
    #[doc = "Godot enumerator name: `MODE_SKY`"]
    pub const SKY: Mode = Mode {
        ord: 3i32
    };
    #[doc(alias = "MODE_FOG")]
    #[doc = "Godot enumerator name: `MODE_FOG`"]
    pub const FOG: Mode = Mode {
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
            Self::SPATIAL => "SPATIAL", Self::CANVAS_ITEM => "CANVAS_ITEM", Self::PARTICLES => "PARTICLES", Self::SKY => "SKY", Self::FOG => "FOG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::SPATIAL, Mode::CANVAS_ITEM, Mode::PARTICLES, Mode::SKY, Mode::FOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SPATIAL", "MODE_SPATIAL", Mode::SPATIAL), crate::meta::inspect::EnumConstant::new("CANVAS_ITEM", "MODE_CANVAS_ITEM", Mode::CANVAS_ITEM), crate::meta::inspect::EnumConstant::new("PARTICLES", "MODE_PARTICLES", Mode::PARTICLES), crate::meta::inspect::EnumConstant::new("SKY", "MODE_SKY", Mode::SKY), crate::meta::inspect::EnumConstant::new("FOG", "MODE_FOG", Mode::FOG)]
        }
    }
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mode Spatial", 0i64), EnumeratorShape::new_int("Mode Canvas Item", 1i64), EnumeratorShape::new_int("Mode Particles", 2i64), EnumeratorShape::new_int("Mode Sky", 3i64), EnumeratorShape::new_int("Mode Fog", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Shader.Mode")), is_bitfield: false,
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
    use super::re_export::Shader;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Shader {
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