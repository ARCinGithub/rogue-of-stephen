#![doc = "Sidecar module for class [`VisualShaderNodeCustom`][crate::classes::VisualShaderNodeCustom].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeCustom` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeCustom`.\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`IVisualShaderNodeCustom`][crate::classes::IVisualShaderNodeCustom]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeCustom`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeCustom::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBy inheriting this class you can create a custom [`VisualShader`][crate::classes::VisualShader] script addon which will be automatically added to the Visual Shader Editor. The [`VisualShaderNode`][crate::classes::VisualShaderNode]'s behavior is defined by overriding the provided virtual methods.\n\nIn order for the node to be registered as an editor addon, you must use the `@tool` annotation and provide a `class_name` for your custom script. For example:\n\n```gdscript\n@tool\nextends VisualShaderNodeCustom\nclass_name VisualShaderNodeNoise\n```"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeCustom {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeCustom`][crate::classes::VisualShaderNodeCustom].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeCustom` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeCustom: crate::obj::GodotClass < Base = VisualShaderNodeCustom > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to define the name of the associated custom node in the Visual Shader Editor's members dialog and graph.\n\nDefining this method is **optional**, but recommended. If not overridden, the node will be named as \"Unnamed\"."]
        fn get_name(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the description of the associated custom node in the Visual Shader Editor's members dialog.\n\nDefining this method is **optional**."]
        fn get_description(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the path to the associated custom node in the Visual Shader Editor's members dialog. The path may look like `\"MyGame/MyFunctions/Noise\"`.\n\nDefining this method is **optional**. If not overridden, the node will be filed under the \"Addons\" category."]
        fn get_category(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the return icon of the associated custom node in the Visual Shader Editor's members dialog.\n\nDefining this method is **optional**. If not overridden, no return icon is shown."]
        fn get_return_icon_type(&self,) -> crate::classes::visual_shader_node::PortType {
            unimplemented !()
        }
        #[doc = "Override this method to define the number of input ports of the associated custom node.\n\nDefining this method is **required**. If not overridden, the node has no input ports."]
        fn get_input_port_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define the returned type of each input port of the associated custom node.\n\nDefining this method is **optional**, but recommended. If not overridden, input ports will return the [`PortType::SCALAR`][`crate::classes::visual_shader_node::PortType::SCALAR`] type."]
        fn get_input_port_type(&self, port: i32,) -> crate::classes::visual_shader_node::PortType {
            unimplemented !()
        }
        #[doc = "Override this method to define the names of input ports of the associated custom node. The names are used both for the input slots in the editor and as identifiers in the shader code, and are passed in the `input_vars` array in [`get_code`][`crate::classes::IVisualShaderNodeCustom::get_code`].\n\nDefining this method is **optional**, but recommended. If not overridden, input ports are named as `\"in\" + str(port)`."]
        fn get_input_port_name(&self, port: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the default value for the specified input port. Prefer use this over [`set_input_port_default_value`][`crate::classes::VisualShaderNode::set_input_port_default_value`].\n\nDefining this method is **required**. If not overridden, the node has no default values for their input ports."]
        fn get_input_port_default_value(&self, port: i32,) -> Variant {
            unimplemented !()
        }
        #[doc = "Override this method to define the input port which should be connected by default when this node is created as a result of dragging a connection from an existing node to the empty space on the graph.\n\nDefining this method is **optional**. If not overridden, the connection will be created to the first valid port."]
        fn get_default_input_port(&self, type_: crate::classes::visual_shader_node::PortType,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define the number of output ports of the associated custom node.\n\nDefining this method is **required**. If not overridden, the node has no output ports."]
        fn get_output_port_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define the returned type of each output port of the associated custom node.\n\nDefining this method is **optional**, but recommended. If not overridden, output ports will return the [`PortType::SCALAR`][`crate::classes::visual_shader_node::PortType::SCALAR`] type."]
        fn get_output_port_type(&self, port: i32,) -> crate::classes::visual_shader_node::PortType {
            unimplemented !()
        }
        #[doc = "Override this method to define the names of output ports of the associated custom node. The names are used both for the output slots in the editor and as identifiers in the shader code, and are passed in the `output_vars` array in [`get_code`][`crate::classes::IVisualShaderNodeCustom::get_code`].\n\nDefining this method is **optional**, but recommended. If not overridden, output ports are named as `\"out\" + str(port)`."]
        fn get_output_port_name(&self, port: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the number of the properties.\n\nDefining this method is **optional**."]
        fn get_property_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define the names of the property of the associated custom node.\n\nDefining this method is **optional**."]
        fn get_property_name(&self, index: i32,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to define the default index of the property of the associated custom node.\n\nDefining this method is **optional**."]
        fn get_property_default_index(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to define the options inside the drop-down list property of the associated custom node.\n\nDefining this method is **optional**."]
        fn get_property_options(&self, index: i32,) -> PackedStringArray {
            unimplemented !()
        }
        #[doc = "Override this method to define the actual shader code of the associated custom node. The shader code should be returned as a string, which can have multiple lines (the `\"\"\"` multiline string construct can be used for convenience).\n\nThe `input_vars` and `output_vars` arrays contain the string names of the various input and output variables, as defined by `_get_input_*` and `_get_output_*` virtual methods in this class.\n\nThe output ports can be assigned values in the shader code. For example, `return output_vars[0] + \" = \" + input_vars[0] + \";\"`.\n\nYou can customize the generated code based on the shader `mode` and/or `type`.\n\nDefining this method is **required**."]
        fn get_code(&self, input_vars: Array < GString >, output_vars: Array < GString >, mode: crate::classes::shader::Mode, type_: crate::classes::visual_shader::Type,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to add a shader code to the beginning of each shader function (once). The shader code should be returned as a string, which can have multiple lines (the `\"\"\"` multiline string construct can be used for convenience).\n\nIf there are multiple custom nodes of different types which use this feature the order of each insertion is undefined.\n\nYou can customize the generated code based on the shader `mode` and/or `type`.\n\nDefining this method is **optional**."]
        fn get_func_code(&self, mode: crate::classes::shader::Mode, type_: crate::classes::visual_shader::Type,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to add shader code on top of the global shader, to define your own standard library of reusable methods, varyings, constants, uniforms, etc. The shader code should be returned as a string, which can have multiple lines (the `\"\"\"` multiline string construct can be used for convenience).\n\nBe careful with this functionality as it can cause name conflicts with other custom nodes, so be sure to give the defined entities unique names.\n\nYou can customize the generated code based on the shader `mode`.\n\nDefining this method is **optional**."]
        fn get_global_code(&self, mode: crate::classes::shader::Mode,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to enable the high-end mark in the Visual Shader Editor's members dialog. This should return `true` for nodes that only work when using the Forward+ and Mobile renderers.\n\nDefining this method is **optional**. If not overridden, it's `false`, which indicates this node works with all renderers (including Compatibility)."]
        fn is_highend(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to prevent the node to be visible in the member dialog for the certain `mode` and/or `type`.\n\nDefining this method is **optional**. If not overridden, it's `true`."]
        fn is_available(&self, mode: crate::classes::shader::Mode, type_: crate::classes::visual_shader::Type,) -> bool {
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
    impl VisualShaderNodeCustom {
        #[doc = "Returns the selected index of the drop-down list option within a graph. You may use this function to define the specific behavior in the [`get_code`][`crate::classes::IVisualShaderNodeCustom::get_code`] or [`get_global_code`][`crate::classes::IVisualShaderNodeCustom::get_global_code`]."]
        pub fn get_option_index(&self, option: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCustom", "get_option_index", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeCustom {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeCustom"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeCustom {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeCustom {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeCustom {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeCustom {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeCustom {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeCustom {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeCustom {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeCustom {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeCustom`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeCustom__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeCustom > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
    use super::re_export::VisualShaderNodeCustom;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeCustom {
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