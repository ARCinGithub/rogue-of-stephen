#![doc = "Sidecar module for class [`SkinReference`][crate::classes::SkinReference].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkinReference` enums](https://docs.godotengine.org/en/stable/classes/class_skinreference.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkinReference`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `SkinReference`](https://docs.godotengine.org/en/stable/classes/class_skinreference.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SkinReference>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nAn internal object containing a mapping from a [`Skin`][crate::classes::Skin] used within the context of a particular [`MeshInstance3D`][crate::classes::MeshInstance3D] to refer to the skeleton's [`RID`][crate::builtin::Rid] in the RenderingServer.\n\nSee also [`get_skin_reference`][`crate::classes::MeshInstance3D::get_skin_reference`] and [`instance_attach_skeleton`][`crate::classes::RenderingServer::instance_attach_skeleton`].\n\nNote that despite the similar naming, the skeleton RID used in the [`RenderingServer`][crate::classes::RenderingServer] does not have a direct one-to-one correspondence to a [`Skeleton3D`][crate::classes::Skeleton3D] node.\n\nIn particular, a [`Skeleton3D`][crate::classes::Skeleton3D] node with no [`MeshInstance3D`][crate::classes::MeshInstance3D] children may be unknown to the [`RenderingServer`][crate::classes::RenderingServer].\n\nOn the other hand, a [`Skeleton3D`][crate::classes::Skeleton3D] with multiple [`MeshInstance3D`][crate::classes::MeshInstance3D] nodes which each have different \\[member MeshInstance3D.skin] objects may have multiple SkinReference instances (and hence, multiple skeleton [`RID`][crate::builtin::Rid]s)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkinReference {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl SkinReference {
        #[doc = "Returns the [`RID`][crate::builtin::Rid] owned by this SkinReference, as returned by [`skeleton_create`][`crate::classes::RenderingServer::skeleton_create`]."]
        pub fn get_skeleton(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkinReference", "get_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Skin`][crate::classes::Skin] connected to this SkinReference. In the case of [`MeshInstance3D`][crate::classes::MeshInstance3D] with no \\[member MeshInstance3D.skin] assigned, this will reference an internal default [`Skin`][crate::classes::Skin] owned by that [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nNote that a single [`Skin`][crate::classes::Skin] may have more than one `SkinReference` in the case that it is shared by meshes across multiple [`Skeleton3D`][crate::classes::Skeleton3D] nodes."]
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallRet = Option < Gd < crate::classes::Skin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkinReference", "get_skin", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SkinReference {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SkinReference"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkinReference {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SkinReference {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SkinReference {
        
    }
    impl std::ops::Deref for SkinReference {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkinReference {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SkinReference__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `SkinReference` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SkinReference;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for SkinReference {
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