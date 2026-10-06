#![doc = "Sidecar module for class [`NavigationMeshGenerator`][crate::classes::NavigationMeshGenerator].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NavigationMeshGenerator` enums](https://docs.godotengine.org/en/stable/classes/class_navigationmeshgenerator.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NavigationMeshGenerator`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `NavigationMeshGenerator`](https://docs.godotengine.org/en/stable/classes/class_navigationmeshgenerator.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThis class is responsible for creating and clearing 3D navigation meshes used as `NavigationMesh` resources inside `NavigationRegion3D`. The `NavigationMeshGenerator` has very limited to no use for 2D as the navigation mesh baking process expects 3D node types and 3D source geometry to parse.\n\nThe entire navigation mesh baking is best done in a separate thread as the voxelization, collision tests and mesh optimization steps involved are very slow and performance-intensive operations.\n\nNavigation mesh baking happens in multiple steps and the result depends on 3D source geometry and properties of the `NavigationMesh` resource. In the first step, starting from a root node and depending on `NavigationMesh` properties all valid 3D source geometry nodes are collected from the [`SceneTree`][crate::classes::SceneTree]. Second, all collected nodes are parsed for their relevant 3D geometry data and a combined 3D mesh is build. Due to the many different types of parsable objects, from normal [`MeshInstance3D`][crate::classes::MeshInstance3D]s to [`CSGShape3D`][crate::classes::CsgShape3D]s or various [`CollisionObject3D`][crate::classes::CollisionObject3D]s, some operations to collect geometry data can trigger [`RenderingServer`][crate::classes::RenderingServer] and [`PhysicsServer3D`][crate::classes::PhysicsServer3D] synchronizations. Server synchronization can have a negative effect on baking time or framerate as it often involves `Mutex` locking for thread security. Many parsable objects and the continuous synchronization with other threaded Servers can increase the baking time significantly. On the other hand only a few but very large and complex objects will take some time to prepare for the Servers which can noticeably stall the next frame render. As a general rule the total number of parsable objects and their individual size and complexity should be balanced to avoid framerate issues or very long baking times. The combined mesh is then passed to the Recast Navigation Object to test the source geometry for walkable terrain suitable to `NavigationMesh` agent properties by creating a voxel world around the meshes bounding area.\n\nThe finalized navigation mesh is then returned and stored inside the `NavigationMesh` for use as a resource inside `NavigationRegion3D` nodes.\n\n**Note:** Using meshes to not only define walkable surfaces but also obstruct navigation baking does not always work. The navigation baking has no concept of what is a geometry \"inside\" when dealing with mesh source geometry and this is intentional. Depending on current baking parameters, as soon as the obstructing mesh is large enough to fit a navigation mesh area inside, the baking will generate navigation mesh areas that are inside the obstructing source geometry mesh."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NavigationMeshGenerator {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl NavigationMeshGenerator {
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
    impl crate::obj::GodotClass for NavigationMeshGenerator {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("NavigationMeshGenerator"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NavigationMeshGenerator {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NavigationMeshGenerator {
        
    }
    impl crate::obj::Singleton for NavigationMeshGenerator {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"NavigationMeshGenerator"))
            }
        }
    }
    impl std::ops::Deref for NavigationMeshGenerator {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NavigationMeshGenerator {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_NavigationMeshGenerator__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `NavigationMeshGenerator` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::NavigationMeshGenerator;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for NavigationMeshGenerator {
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