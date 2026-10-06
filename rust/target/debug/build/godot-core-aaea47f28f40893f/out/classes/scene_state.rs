#![doc = "Sidecar module for class [`SceneState`][crate::classes::SceneState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneState` enums](https://docs.godotengine.org/en/stable/classes/class_scenestate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneState`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`scene_state`][crate::classes::scene_state]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `SceneState`](https://docs.godotengine.org/en/stable/classes/class_scenestate.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SceneState>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nMaintains a list of resources, nodes, exported and overridden properties, and built-in scripts associated with a scene. They cannot be modified from a `SceneState`, only accessed. Useful for peeking into what a [`PackedScene`][crate::classes::PackedScene] contains without instantiating it.\n\nThis class cannot be instantiated directly, it is retrieved for a given scene as the result of [`get_state`][`crate::classes::PackedScene::get_state`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl SceneState {
        #[doc = "Returns the resource path to the represented [`PackedScene`][crate::classes::PackedScene]."]
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `SceneState` of the scene that this scene inherits from, or `null` if it doesn't inherit from any scene."]
        pub fn get_base_scene_state(&self,) -> Option < Gd < crate::classes::SceneState > > {
            type CallRet = Option < Gd < crate::classes::SceneState > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_base_scene_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of nodes in the scene.\n\nThe `idx` argument used to query node data in other `get_node_*` methods in the interval `[0, get_node_count() - 1]`."]
        pub fn get_node_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the node at `idx`."]
        pub fn get_node_type(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the node at `idx`."]
        pub fn get_node_name(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the node at `idx`.\n\nIf `for_parent` is `true`, returns the path of the `idx` node's parent instead."]
        pub(crate) fn get_node_path_full(&self, idx: i32, for_parent: bool,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32, bool,);
            let args = (idx, for_parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_node_path_ex`][Self::get_node_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the path to the node at `idx`.\n\nIf `for_parent` is `true`, returns the path of the `idx` node's parent instead."]
        #[inline]
        pub fn get_node_path(&self, idx: i32,) -> NodePath {
            self.get_node_path_ex(idx,) . done()
        }
        #[doc = "Returns the path to the node at `idx`.\n\nIf `for_parent` is `true`, returns the path of the `idx` node's parent instead."]
        #[inline]
        pub fn get_node_path_ex < 'ex > (&'ex self, idx: i32,) -> ExGetNodePath < 'ex > {
            ExGetNodePath::new(self, idx,)
        }
        #[doc = "Returns the path to the owner of the node at `idx`, relative to the root node."]
        pub fn get_node_owner_path(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_owner_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node at `idx` is an [`InstancePlaceholder`][crate::classes::InstancePlaceholder]."]
        pub fn is_node_instance_placeholder(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "is_node_instance_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the represented scene file if the node at `idx` is an [`InstancePlaceholder`][crate::classes::InstancePlaceholder]."]
        pub fn get_node_instance_placeholder(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedScene`][crate::classes::PackedScene] for the node at `idx` (i.e. the whole branch starting at this node, with its child nodes and resources), or `null` if the node is not an instance."]
        pub fn get_node_instance(&self, idx: i32,) -> Option < Gd < crate::classes::PackedScene > > {
            type CallRet = Option < Gd < crate::classes::PackedScene > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of group names associated with the node at `idx`."]
        pub fn get_node_groups(&self, idx: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_groups", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the node's index, which is its position relative to its siblings. This is only relevant and saved in scenes for cases where new nodes are added to an instantiated or inherited scene among siblings from the base scene. Despite the name, this index is not related to the `idx` argument used here and in other methods."]
        pub fn get_node_index(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of exported or overridden properties for the node at `idx`.\n\nThe `prop_idx` argument used to query node property data in other `get_node_property_*` methods in the interval `[0, get_node_property_count() - 1]`."]
        pub fn get_node_property_count(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the property at `prop_idx` for the node at `idx`."]
        pub fn get_node_property_name(&self, idx: i32, prop_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32, i32,);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the property at `prop_idx` for the node at `idx`."]
        pub fn get_node_property_value(&self, idx: i32, prop_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of signal connections in the scene.\n\nThe `idx` argument used to query connection metadata in other `get_connection_*` methods in the interval `[0, get_connection_count() - 1]`."]
        pub fn get_connection_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the node that owns the signal at `idx`, relative to the root node."]
        pub fn get_connection_source(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the signal at `idx`."]
        pub fn get_connection_signal(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the path to the node that owns the method connected to the signal at `idx`, relative to the root node."]
        pub fn get_connection_target(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_target", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the method connected to the signal at `idx`."]
        pub fn get_connection_method(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the connection flags for the signal at `idx`. See \\[enum Object.ConnectFlags] constants."]
        pub fn get_connection_flags(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of bound parameters for the signal at `idx`."]
        pub fn get_connection_binds(&self, idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_binds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of unbound parameters for the signal at `idx`."]
        pub fn get_connection_unbinds(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_unbinds", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneState {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneState {
        
    }
    impl std::ops::Deref for SceneState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneState__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `SceneState` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`SceneState::get_node_path_ex`][super::SceneState::get_node_path_ex]."]
#[must_use]
pub struct ExGetNodePath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::SceneState, idx: i32, for_parent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetNodePath < 'ex > {
    fn new(surround_object: &'ex re_export::SceneState, idx: i32,) -> Self {
        let for_parent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, for_parent: for_parent,
        }
    }
    #[inline]
    pub fn for_parent(self, for_parent: bool) -> Self {
        Self {
            for_parent: for_parent, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, idx, for_parent,
        }
        = self;
        re_export::SceneState::get_node_path_full(surround_object, idx, for_parent,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenEditState {
    ord: i32
}
impl GenEditState {
    #[doc(alias = "GEN_EDIT_STATE_DISABLED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_DISABLED`"]
    pub const DISABLED: GenEditState = GenEditState {
        ord: 0i32
    };
    #[doc(alias = "GEN_EDIT_STATE_INSTANCE")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_INSTANCE`"]
    pub const INSTANCE: GenEditState = GenEditState {
        ord: 1i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN`"]
    pub const MAIN: GenEditState = GenEditState {
        ord: 2i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN_INHERITED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN_INHERITED`"]
    pub const MAIN_INHERITED: GenEditState = GenEditState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for GenEditState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GenEditState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GenEditState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::INSTANCE => "INSTANCE", Self::MAIN => "MAIN", Self::MAIN_INHERITED => "MAIN_INHERITED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GenEditState::DISABLED, GenEditState::INSTANCE, GenEditState::MAIN, GenEditState::MAIN_INHERITED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GenEditState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "GEN_EDIT_STATE_DISABLED", GenEditState::DISABLED), crate::meta::inspect::EnumConstant::new("INSTANCE", "GEN_EDIT_STATE_INSTANCE", GenEditState::INSTANCE), crate::meta::inspect::EnumConstant::new("MAIN", "GEN_EDIT_STATE_MAIN", GenEditState::MAIN), crate::meta::inspect::EnumConstant::new("MAIN_INHERITED", "GEN_EDIT_STATE_MAIN_INHERITED", GenEditState::MAIN_INHERITED)]
        }
    }
}
impl crate::meta::GodotConvert for GenEditState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Gen Edit State Disabled", 0i64), EnumeratorShape::new_int("Gen Edit State Instance", 1i64), EnumeratorShape::new_int("Gen Edit State Main", 2i64), EnumeratorShape::new_int("Gen Edit State Main Inherited", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("SceneState.GenEditState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GenEditState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GenEditState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GenEditState {
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
impl crate::registry::property::Export for GenEditState {
    
}
impl crate::meta::Element for GenEditState {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneState;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for SceneState {
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