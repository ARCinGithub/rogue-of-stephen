#![doc = "Sidecar module for class [`EditorDebuggerPlugin`][crate::classes::EditorDebuggerPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorDebuggerPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editordebuggerplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorDebuggerPlugin`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IEditorDebuggerPlugin`][crate::classes::IEditorDebuggerPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorDebuggerPlugin`](https://docs.godotengine.org/en/stable/classes/class_editordebuggerplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorDebuggerPlugin::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\n`EditorDebuggerPlugin` provides functions related to the editor side of the debugger.\n\nTo interact with the debugger, an instance of this class must be added to the editor via [`add_debugger_plugin`][`crate::classes::EditorPlugin::add_debugger_plugin`].\n\nOnce added, the [`setup_session`][`crate::classes::IEditorDebuggerPlugin::setup_session`] callback will be called for every [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] available to the plugin, and when new ones are created (the sessions may be inactive during this stage).\n\nYou can retrieve the available [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession]s via [`get_sessions`][`crate::classes::EditorDebuggerPlugin::get_sessions`] or get a specific one via [`get_session`][`crate::classes::EditorDebuggerPlugin::get_session`].\n\n\n```gdscript\n@tool\nextends EditorPlugin\n\nclass ExampleEditorDebugger extends EditorDebuggerPlugin:\n\n\tfunc _has_capture(capture):\n\t\t# Return true if you wish to handle messages with the prefix \"my_plugin:\".\n\t\treturn capture == \"my_plugin\"\n\n\tfunc _capture(message, data, session_id):\n\t\tif message == \"my_plugin:ping\":\n\t\t\tget_session(session_id).send_message(\"my_plugin:echo\", data)\n\t\t\treturn true\n\t\treturn false\n\n\tfunc _setup_session(session_id):\n\t\t# Add a new tab in the debugger session UI containing a label.\n\t\tvar label = Label.new()\n\t\tlabel.name = \"Example plugin\" # Will be used as the tab title.\n\t\tlabel.text = \"Example plugin\"\n\t\tvar session = get_session(session_id)\n\t\t# Listens to the session started and stopped signals.\n\t\tsession.started.connect(func (): print(\"Session started\"))\n\t\tsession.stopped.connect(func (): print(\"Session stopped\"))\n\t\tsession.add_session_tab(label)\n\nvar debugger = ExampleEditorDebugger.new()\n\nfunc _enter_tree():\n\tadd_debugger_plugin(debugger)\n\nfunc _exit_tree():\n\tremove_debugger_plugin(debugger)\n```\n\n\nTo connect on the running game side, use the [`EngineDebugger`][crate::classes::EngineDebugger] singleton:\n\n\n```gdscript\nextends Node\n\nfunc _ready():\n\tEngineDebugger.register_message_capture(\"my_plugin\", _capture)\n\tEngineDebugger.send_message(\"my_plugin:ping\", [\"test\"])\n\nfunc _capture(message, data):\n\t# Note that the \"my_plugin:\" prefix is not used here.\n\tif message == \"echo\":\n\t\tprints(\"Echo received:\", data)\n\t\treturn true\n\treturn false\n```\n\n\n**Note:** While the game is running, [`print`][`crate::global::print`] and similar functions _called in the editor_ do not print anything, the Output Log prints only game messages."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorDebuggerPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorDebuggerPlugin`][crate::classes::EditorDebuggerPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorDebuggerPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editordebuggerplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorDebuggerPlugin: crate::obj::GodotClass < Base = EditorDebuggerPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to be notified whenever a new [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] is created. Note that the session may be inactive during this stage."]
        fn setup_session(&mut self, session_id: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to enable receiving messages from the debugger. If `capture` is \"my_message\" then messages starting with \"my_message:\" will be passed to the [`capture`][`crate::classes::IEditorDebuggerPlugin::capture`] method."]
        fn has_capture(&self, capture: GString,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to process incoming messages. The `session_id` is the ID of the [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] that received the `message`. Use [`get_session`][`crate::classes::EditorDebuggerPlugin::get_session`] to retrieve the session. This method should return `true` if the message is recognized."]
        fn capture(&mut self, message: GString, data: VarArray, session_id: i32,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to be notified when a breakpoint line has been clicked in the debugger breakpoint panel."]
        fn goto_script_line(&mut self, script: Option < Gd < crate::classes::Script > >, line: i32,) {
            unimplemented !()
        }
        #[doc = "Override this method to be notified when all breakpoints are cleared in the editor."]
        fn breakpoints_cleared_in_tree(&mut self,) {
            unimplemented !()
        }
        #[doc = "Override this method to be notified when a breakpoint is set in the editor."]
        fn breakpoint_set_in_tree(&mut self, script: Option < Gd < crate::classes::Script > >, line: i32, enabled: bool,) {
            unimplemented !()
        }
    }
    impl EditorDebuggerPlugin {
        #[doc = "Returns the [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] with the given `id`."]
        pub fn get_session(&self, id: i32,) -> Option < Gd < crate::classes::EditorDebuggerSession > > {
            type CallRet = Option < Gd < crate::classes::EditorDebuggerSession > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(13usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerPlugin", "get_session", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] currently available to this debugger plugin.\n\n**Note:** Sessions in the array may be inactive, check their state via [`is_active`][`crate::classes::EditorDebuggerSession::is_active`]."]
        pub fn get_sessions(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(14usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerPlugin", "get_sessions", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorDebuggerPlugin {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorDebuggerPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorDebuggerPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorDebuggerPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorDebuggerPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorDebuggerPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorDebuggerPlugin {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorDebuggerPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorDebuggerPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorDebuggerPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorDebuggerPlugin > for $Class {
                
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
    use super::re_export::EditorDebuggerPlugin;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorDebuggerPlugin {
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