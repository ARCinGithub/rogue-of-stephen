#![doc = "Sidecar module for class [`ImageTextureLayered`][crate::classes::ImageTextureLayered].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImageTextureLayered` enums](https://docs.godotengine.org/en/stable/classes/class_imagetexturelayered.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImageTextureLayered`.\n\nInherits [`TextureLayered`][crate::classes::TextureLayered].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `ImageTextureLayered`](https://docs.godotengine.org/en/stable/classes/class_imagetexturelayered.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ImageTextureLayered>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nBase class for [`Texture2DArray`][crate::classes::Texture2DArray], [`Cubemap`][crate::classes::Cubemap] and [`CubemapArray`][crate::classes::CubemapArray]. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also [`Texture3D`][crate::classes::Texture3D]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImageTextureLayered {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ImageTextureLayered {
        #[doc = "Creates an `ImageTextureLayered` from an array of [`Image`][crate::classes::Image]s. See [`create`][`crate::classes::Image::create`] for the expected data format. The first image decides the width, height, image format and mipmapping setting. The other images _must_ have the same width, height, image format and mipmapping setting.\n\nEach [`Image`][crate::classes::Image] represents one `layer`.\n\n```gdscript\n# Fill in an array of Images with different colors.\nvar images = []\nconst LAYERS = 6\nfor i in LAYERS:\n\tvar image = Image.create_empty(128, 128, false, Image.FORMAT_RGB8)\n\tif i % 3 == 0:\n\t\timage.fill(Color.RED)\n\telif i % 3 == 1:\n\t\timage.fill(Color.GREEN)\n\telse:\n\t\timage.fill(Color.BLUE)\n\timages.push_back(image)\n\n# Create and save a 2D texture array. The array of images must have at least 1 Image.\nvar texture_2d_array = Texture2DArray.new()\ntexture_2d_array.create_from_images(images)\nResourceSaver.save(texture_2d_array, \"res://texture_2d_array.res\", ResourceSaver.FLAG_COMPRESS)\n\n# Create and save a cubemap. The array of images must have exactly 6 Images.\n# The cubemap's images are specified in this order: X+, X-, Y+, Y-, Z+, Z-\n# (in Godot's coordinate system, so Y+ is \"up\" and Z- is \"forward\").\nvar cubemap = Cubemap.new()\ncubemap.create_from_images(images)\nResourceSaver.save(cubemap, \"res://cubemap.res\", ResourceSaver.FLAG_COMPRESS)\n\n# Create and save a cubemap array. The array of images must have a multiple of 6 Images.\n# Each cubemap's images are specified in this order: X+, X-, Y+, Y-, Z+, Z-\n# (in Godot's coordinate system, so Y+ is \"up\" and Z- is \"forward\").\nvar cubemap_array = CubemapArray.new()\ncubemap_array.create_from_images(images)\nResourceSaver.save(cubemap_array, \"res://cubemap_array.res\", ResourceSaver.FLAG_COMPRESS)\n```"]
        pub fn create_from_images(&mut self, images: &Array < Gd < crate::classes::Image > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Image > > >,);
            let args = (RefArg::new(images),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImageTextureLayered", "create_from_images", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Replaces the existing [`Image`][crate::classes::Image] data at the given `layer` with this new image.\n\nThe given [`Image`][crate::classes::Image] must have the same width, height, image format, and mipmapping flag as the rest of the referenced images.\n\nIf the image format is unsupported, it will be decompressed and converted to a similar and supported \\[enum Image.Format].\n\nThe update is immediate: it's synchronized with drawing."]
        pub fn update_layer(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, layer: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, i32,);
            let args = (image.into_arg(), layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImageTextureLayered", "update_layer", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ImageTextureLayered {
        type Base = crate::classes::TextureLayered;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ImageTextureLayered"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImageTextureLayered {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextureLayered > for ImageTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for ImageTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ImageTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ImageTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImageTextureLayered {
        
    }
    impl std::ops::Deref for ImageTextureLayered {
        type Target = crate::classes::TextureLayered;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImageTextureLayered {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ImageTextureLayered__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ImageTextureLayered` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ImageTextureLayered;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ImageTextureLayered {
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