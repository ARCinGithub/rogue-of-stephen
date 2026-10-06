#![doc = "Sidecar module for class [`PackedScene`][crate::classes::PackedScene].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PackedScene` enums](https://docs.godotengine.org/en/stable/classes/class_packedscene.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PackedScene`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`packed_scene`][crate::classes::packed_scene]: sidecar module with related enum/flag types\n* [`IPackedScene`][crate::classes::IPackedScene]: virtual methods\n\n\nSee also [Godot docs for `PackedScene`](https://docs.godotengine.org/en/stable/classes/class_packedscene.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PackedScene::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA simplified interface to a scene file. Provides access to operations and checks that can be performed on the scene resource itself.\n\nCan be used to save a node to a file. When saving, the node as well as all the nodes it owns get saved (see \\[member Node.owner] property).\n\n**Note:** The node doesn't need to own itself.\n\n**Example:** Load a saved scene:\n\n\n```gdscript\n# Use load() instead of preload() if the path isn't known at compile-time.\nvar scene = preload(\"res://scene.tscn\").instantiate()\n# Add the node as a child of the node the script is attached to.\nadd_child(scene)\n```\n\n\n**Example:** Save a node with different owners. The following example creates 3 objects: [`Node2D`][crate::classes::Node2D] (`node`), [`RigidBody2D`][crate::classes::RigidBody2D] (`body`) and [`CollisionObject2D`][crate::classes::CollisionObject2D] (`collision`). `collision` is a child of `body` which is a child of `node`. Only `body` is owned by `node` and [`pack`][`crate::classes::PackedScene::pack`] will therefore only save those two nodes, but not `collision`.\n\n\n```gdscript\n# Create the objects.\nvar node = Node2D.new()\nvar body = RigidBody2D.new()\nvar collision = CollisionShape2D.new()\n\n# Create the object hierarchy.\nbody.add_child(collision)\nnode.add_child(body)\n\n# Change owner of `body`, but not of `collision`.\nbody.owner = node\nvar scene = PackedScene.new()\n\n# Only `node` and `body` are now packed.\nvar result = scene.pack(node)\nif result == OK:\n\tvar error = ResourceSaver.save(scene, \"res://path/name.tscn\")  # Or \"user://...\"\n\tif error != OK:\n\t\tpush_error(\"An error occurred while saving the scene to disk.\")\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PackedScene {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PackedScene`][crate::classes::PackedScene].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `PackedScene` methods](https://docs.godotengine.org/en/stable/classes/class_packedscene.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPackedScene: crate::obj::GodotClass < Base = PackedScene > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PackedScene {
        #[doc = "Packs the `path` node, and all owned sub-nodes, into this `PackedScene`. Any existing data will be cleared. See \\[member Node.owner]."]
        pub fn pack(&mut self, path: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PackedScene", "pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Instantiates the scene's node hierarchy. Triggers child scene instantiation(s). Triggers a [`NodeNotification::SCENE_INSTANTIATED`][`crate::classes::notify::NodeNotification::SCENE_INSTANTIATED`] notification on the root node."]
        pub(crate) fn instantiate_full(&self, edit_state: crate::classes::packed_scene::GenEditState,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (crate::classes::packed_scene::GenEditState,);
            let args = (edit_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PackedScene", "instantiate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`instantiate_ex`][Self::instantiate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Instantiates the scene's node hierarchy. Triggers child scene instantiation(s). Triggers a [`NodeNotification::SCENE_INSTANTIATED`][`crate::classes::notify::NodeNotification::SCENE_INSTANTIATED`] notification on the root node."]
        #[inline]
        pub fn instantiate(&self,) -> Option < Gd < crate::classes::Node > > {
            self.instantiate_ex() . done()
        }
        #[doc = "Instantiates the scene's node hierarchy. Triggers child scene instantiation(s). Triggers a [`NodeNotification::SCENE_INSTANTIATED`][`crate::classes::notify::NodeNotification::SCENE_INSTANTIATED`] notification on the root node."]
        #[inline]
        pub fn instantiate_ex < 'ex > (&'ex self,) -> ExInstantiate < 'ex > {
            ExInstantiate::new(self,)
        }
        #[doc = "Returns `true` if the scene file has nodes."]
        pub fn can_instantiate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PackedScene", "can_instantiate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`SceneState`][crate::classes::SceneState] representing the scene file contents."]
        pub fn get_state(&self,) -> Option < Gd < crate::classes::SceneState > > {
            type CallRet = Option < Gd < crate::classes::SceneState > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PackedScene", "get_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PackedScene {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PackedScene"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PackedScene {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for PackedScene {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PackedScene {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PackedScene {
        
    }
    impl crate::obj::cap::GodotDefault for PackedScene {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PackedScene {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PackedScene {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PackedScene`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PackedScene__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PackedScene > for $Class {
                
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
#[doc = "Default-param extender for [`PackedScene::instantiate_ex`][super::PackedScene::instantiate_ex]."]
#[must_use]
pub struct ExInstantiate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::PackedScene, edit_state: crate::classes::packed_scene::GenEditState,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInstantiate < 'ex > {
    fn new(surround_object: &'ex re_export::PackedScene,) -> Self {
        let edit_state = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, edit_state: edit_state,
        }
    }
    #[inline]
    pub fn edit_state(self, edit_state: crate::classes::packed_scene::GenEditState) -> Self {
        Self {
            edit_state: edit_state, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, edit_state,
        }
        = self;
        re_export::PackedScene::instantiate_full(surround_object, edit_state,)
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("PackedScene.GenEditState")), is_bitfield: false,
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
    use super::re_export::PackedScene;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for PackedScene {
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