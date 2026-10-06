#![doc = "Sidecar module for class [`EditorInspectorPlugin`][crate::classes::EditorInspectorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInspectorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInspectorPlugin`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_inspector_plugin`][crate::classes::editor_inspector_plugin]: sidecar module with related enum/flag types\n* [`IEditorInspectorPlugin`][crate::classes::IEditorInspectorPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorInspectorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorInspectorPlugin::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`EditorInspectorPlugin` allows adding custom property editors to [`EditorInspector`][crate::classes::EditorInspector].\n\nWhen an object is edited, the [`can_handle`][`crate::classes::IEditorInspectorPlugin::can_handle`] function is called and must return `true` if the object type is supported.\n\nIf supported, the function [`parse_begin`][`crate::classes::IEditorInspectorPlugin::parse_begin`] will be called, allowing to place custom controls at the beginning of the class.\n\nSubsequently, the [`parse_category`][`crate::classes::IEditorInspectorPlugin::parse_category`] and [`parse_property`][`crate::classes::IEditorInspectorPlugin::parse_property`] are called for every category and property. They offer the ability to add custom controls to the inspector too.\n\nFinally, [`parse_end`][`crate::classes::IEditorInspectorPlugin::parse_end`] will be called.\n\nOn each of these calls, the \"add\" functions can be called.\n\nTo use `EditorInspectorPlugin`, register it using the [`add_inspector_plugin`][`crate::classes::EditorPlugin::add_inspector_plugin`] method first."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInspectorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorInspectorPlugin`][crate::classes::EditorInspectorPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorInspectorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorinspectorplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorInspectorPlugin: crate::obj::GodotClass < Base = EditorInspectorPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Returns `true` if this object can be handled by this plugin."]
        fn can_handle(&self, object: Option < Gd < crate::classes::Object > >,) -> bool {
            unimplemented !()
        }
        #[doc = "Called to allow adding controls at the beginning of the property list for `object`."]
        fn parse_begin(&mut self, object: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
        #[doc = "Called to allow adding controls at the beginning of a category in the property list for `object`."]
        fn parse_category(&mut self, object: Option < Gd < crate::classes::Object > >, category: GString,) {
            unimplemented !()
        }
        #[doc = "Called to allow adding controls at the beginning of a group or a sub-group in the property list for `object`."]
        fn parse_group(&mut self, object: Option < Gd < crate::classes::Object > >, group: GString,) {
            unimplemented !()
        }
        #[doc = "Called to allow adding property-specific editors to the property list for `object`. The added editor control must extend [`EditorProperty`][crate::classes::EditorProperty]. Returning `true` removes the built-in editor for this property, otherwise allows to insert a custom editor before the built-in one."]
        fn parse_property(&mut self, object: Option < Gd < crate::classes::Object > >, type_: VariantType, name: GString, hint_type: crate::registry::info::PropertyHint, hint_string: GString, usage_flags: crate::registry::info::PropertyUsageFlags, wide: bool,) -> bool {
            unimplemented !()
        }
        #[doc = "Called to allow adding controls at the end of the property list for `object`."]
        fn parse_end(&mut self, object: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
    }
    impl EditorInspectorPlugin {
        #[doc = "Adds a custom control, which is not necessarily a property editor."]
        pub fn add_custom_control(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspectorPlugin", "add_custom_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a property editor for an individual property. The `editor` control must extend [`EditorProperty`][crate::classes::EditorProperty].\n\nThere can be multiple property editors for a property. If `add_to_end` is `true`, this newly added editor will be displayed after all the other editors of the property whose `add_to_end` is `false`. For example, the editor uses this parameter to add an \"Edit Region\" button for \\[member Sprite2D.region_rect] below the regular [`Rect2`][crate::builtin::Rect2] editor.\n\n`label` can be used to choose a custom label for the property editor in the inspector. If left empty, the label is computed from the name of the property instead."]
        pub(crate) fn add_property_editor_full(&mut self, property: CowArg < GString >, editor: CowArg < Option < Gd < crate::classes::Control > > >, add_to_end: bool, label: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Control > > >, bool, CowArg < 'a2, GString >,);
            let args = (property, editor, add_to_end, label,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspectorPlugin", "add_property_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_property_editor_ex`][Self::add_property_editor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a property editor for an individual property. The `editor` control must extend [`EditorProperty`][crate::classes::EditorProperty].\n\nThere can be multiple property editors for a property. If `add_to_end` is `true`, this newly added editor will be displayed after all the other editors of the property whose `add_to_end` is `false`. For example, the editor uses this parameter to add an \"Edit Region\" button for \\[member Sprite2D.region_rect] below the regular [`Rect2`][crate::builtin::Rect2] editor.\n\n`label` can be used to choose a custom label for the property editor in the inspector. If left empty, the label is computed from the name of the property instead."]
        #[inline]
        pub fn add_property_editor(&mut self, property: impl AsArg < GString >, editor: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            self.add_property_editor_ex(property, editor,) . done()
        }
        #[doc = "Adds a property editor for an individual property. The `editor` control must extend [`EditorProperty`][crate::classes::EditorProperty].\n\nThere can be multiple property editors for a property. If `add_to_end` is `true`, this newly added editor will be displayed after all the other editors of the property whose `add_to_end` is `false`. For example, the editor uses this parameter to add an \"Edit Region\" button for \\[member Sprite2D.region_rect] below the regular [`Rect2`][crate::builtin::Rect2] editor.\n\n`label` can be used to choose a custom label for the property editor in the inspector. If left empty, the label is computed from the name of the property instead."]
        #[inline]
        pub fn add_property_editor_ex < 'ex > (&'ex mut self, property: impl AsArg < GString > + 'ex, editor: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex,) -> ExAddPropertyEditor < 'ex > {
            ExAddPropertyEditor::new(self, property, editor,)
        }
        #[doc = "Adds an editor that allows modifying multiple properties. The `editor` control must extend [`EditorProperty`][crate::classes::EditorProperty]."]
        pub fn add_property_editor_for_multiple_properties(&mut self, label: impl AsArg < GString >, properties: &PackedStringArray, editor: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, Option < Gd < crate::classes::Control > > >,);
            let args = (label.into_arg(), RefArg::new(properties), editor.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspectorPlugin", "add_property_editor_for_multiple_properties", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorInspectorPlugin {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorInspectorPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInspectorPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorInspectorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorInspectorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorInspectorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorInspectorPlugin {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInspectorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorInspectorPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorInspectorPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorInspectorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorInspectorPlugin::add_property_editor_ex`][super::EditorInspectorPlugin::add_property_editor_ex]."]
#[must_use]
pub struct ExAddPropertyEditor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInspectorPlugin, property: CowArg < 'ex, GString >, editor: CowArg < 'ex, Option < Gd < crate::classes::Control > > >, add_to_end: bool, label: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPropertyEditor < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInspectorPlugin, property: impl AsArg < GString > + 'ex, editor: impl AsArg < Option < Gd < crate::classes::Control >> > + 'ex,) -> Self {
        let add_to_end = false;
        let label = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, property: property.into_arg(), editor: editor.into_arg(), add_to_end: add_to_end, label: CowArg::Owned(label),
        }
    }
    #[inline]
    pub fn add_to_end(self, add_to_end: bool) -> Self {
        Self {
            add_to_end: add_to_end, .. self
        }
    }
    #[inline]
    pub fn label(self, label: impl AsArg < GString > + 'ex) -> Self {
        Self {
            label: label.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, property, editor, add_to_end, label,
        }
        = self;
        re_export::EditorInspectorPlugin::add_property_editor_full(surround_object, property, editor, add_to_end, label,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorInspectorPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorInspectorPlugin {
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