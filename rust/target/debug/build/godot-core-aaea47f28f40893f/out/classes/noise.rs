#![doc = "Sidecar module for class [`Noise`][crate::classes::Noise].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Noise` enums](https://docs.godotengine.org/en/stable/classes/class_noise.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Noise`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`noise`][crate::classes::noise]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Noise`](https://docs.godotengine.org/en/stable/classes/class_noise.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Noise>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThis class defines the interface for noise generation libraries to inherit from.\n\nA default [`get_seamless_image`][`crate::classes::Noise::get_seamless_image`] implementation is provided for libraries that do not provide seamless noise. This function requests a larger image from the [`get_image`][`crate::classes::Noise::get_image`] method, reverses the quadrants of the image, then uses the strips of extra width to blend over the seams.\n\nInheriting noise classes can optionally override this function to provide a more optimal algorithm."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Noise {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Noise {
        #[doc = "Returns the 1D noise value at the given (x) coordinate."]
        pub fn get_noise_1d(&self, x: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32,);
            let args = (x,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_noise_1d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D noise value at the given position."]
        pub fn get_noise_2d(&self, x: f32, y: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32, f32,);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_noise_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D noise value at the given position."]
        pub fn get_noise_2dv(&self, v: Vector2,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2,);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_noise_2dv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 3D noise value at the given position."]
        pub fn get_noise_3d(&self, x: f32, y: f32, z: f32,) -> f32 {
            type CallRet = f32;
            type CallParams = (f32, f32, f32,);
            let args = (x, y, z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_noise_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 3D noise value at the given position."]
        pub fn get_noise_3dv(&self, v: Vector3,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector3,);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_noise_3dv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] containing 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        pub(crate) fn get_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, bool, bool,);
            let args = (width, height, invert, in_3d_space, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_image_ex`][Self::get_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an [`Image`][crate::classes::Image] containing 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_image(&self, width: i32, height: i32,) -> Option < Gd < crate::classes::Image > > {
            self.get_image_ex(width, height,) . done()
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] containing 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_image_ex < 'ex > (&'ex self, width: i32, height: i32,) -> ExGetImage < 'ex > {
            ExGetImage::new(self, width, height,)
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] containing seamless 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        pub(crate) fn get_seamless_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, bool, f32, bool,);
            let args = (width, height, invert, in_3d_space, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_seamless_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_seamless_image_ex`][Self::get_seamless_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an [`Image`][crate::classes::Image] containing seamless 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_seamless_image(&self, width: i32, height: i32,) -> Option < Gd < crate::classes::Image > > {
            self.get_seamless_image_ex(width, height,) . done()
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] containing seamless 2D noise values.\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_seamless_image_ex < 'ex > (&'ex self, width: i32, height: i32,) -> ExGetSeamlessImage < 'ex > {
            ExGetSeamlessImage::new(self, width, height,)
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        pub(crate) fn get_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, i32, bool, bool,);
            let args = (width, height, depth, invert, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_image_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_image_3d_ex`][Self::get_image_3d_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::classes::Image > > {
            self.get_image_3d_ex(width, height, depth,) . done()
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_image_3d_ex < 'ex > (&'ex self, width: i32, height: i32, depth: i32,) -> ExGetImage3d < 'ex > {
            ExGetImage3d::new(self, width, height, depth,)
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing seamless 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        pub(crate) fn get_seamless_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, i32, bool, f32, bool,);
            let args = (width, height, depth, invert, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Noise", "get_seamless_image_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_seamless_image_3d_ex`][Self::get_seamless_image_3d_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing seamless 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_seamless_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::classes::Image > > {
            self.get_seamless_image_3d_ex(width, height, depth,) . done()
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Image`][crate::classes::Image]s containing seamless 3D noise values for use with [`create`][`crate::classes::ImageTexture3D::create`].\n\n**Note:** With `normalize` set to `false`, the default implementation expects the noise generator to return values in the range `-1.0` to `1.0`."]
        #[inline]
        pub fn get_seamless_image_3d_ex < 'ex > (&'ex self, width: i32, height: i32, depth: i32,) -> ExGetSeamlessImage3d < 'ex > {
            ExGetSeamlessImage3d::new(self, width, height, depth,)
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
    impl crate::obj::GodotClass for Noise {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Noise"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Noise {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Noise {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Noise {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Noise {
        
    }
    impl std::ops::Deref for Noise {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Noise {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Noise__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Noise` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Noise::get_image_ex`][super::Noise::get_image_ex]."]
#[must_use]
pub struct ExGetImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetImage < 'ex > {
    fn new(surround_object: &'ex re_export::Noise, width: i32, height: i32,) -> Self {
        let invert = false;
        let in_3d_space = false;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, invert: invert, in_3d_space: in_3d_space, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, in_3d_space: bool) -> Self {
        Self {
            in_3d_space: in_3d_space, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, invert, in_3d_space, normalize,
        }
        = self;
        re_export::Noise::get_image_full(surround_object, width, height, invert, in_3d_space, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_ex`][super::Noise::get_seamless_image_ex]."]
#[must_use]
pub struct ExGetSeamlessImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSeamlessImage < 'ex > {
    fn new(surround_object: &'ex re_export::Noise, width: i32, height: i32,) -> Self {
        let invert = false;
        let in_3d_space = false;
        let skirt = 0.1f32;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, invert: invert, in_3d_space: in_3d_space, skirt: skirt, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, in_3d_space: bool) -> Self {
        Self {
            in_3d_space: in_3d_space, .. self
        }
    }
    #[inline]
    pub fn skirt(self, skirt: f32) -> Self {
        Self {
            skirt: skirt, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, invert, in_3d_space, skirt, normalize,
        }
        = self;
        re_export::Noise::get_seamless_image_full(surround_object, width, height, invert, in_3d_space, skirt, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_image_3d_ex`][super::Noise::get_image_3d_ex]."]
#[must_use]
pub struct ExGetImage3d < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetImage3d < 'ex > {
    fn new(surround_object: &'ex re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        let invert = false;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, depth: depth, invert: invert, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, depth, invert, normalize,
        }
        = self;
        re_export::Noise::get_image_3d_full(surround_object, width, height, depth, invert, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_3d_ex`][super::Noise::get_seamless_image_3d_ex]."]
#[must_use]
pub struct ExGetSeamlessImage3d < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetSeamlessImage3d < 'ex > {
    fn new(surround_object: &'ex re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        let invert = false;
        let skirt = 0.1f32;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, depth: depth, invert: invert, skirt: skirt, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn skirt(self, skirt: f32) -> Self {
        Self {
            skirt: skirt, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, depth, invert, skirt, normalize,
        }
        = self;
        re_export::Noise::get_seamless_image_3d_full(surround_object, width, height, depth, invert, skirt, normalize,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Noise;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Noise {
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