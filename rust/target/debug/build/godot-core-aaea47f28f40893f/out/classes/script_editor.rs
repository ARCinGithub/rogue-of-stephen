#![doc = "Sidecar module for class [`ScriptEditor`][crate::classes::ScriptEditor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptEditor` enums](https://docs.godotengine.org/en/stable/classes/class_scripteditor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptEditor`.\n\nInherits [`PanelContainer`][crate::classes::PanelContainer].\n\nRelated symbols:\n\n* [`script_editor`][crate::classes::script_editor]: sidecar module with related enum/flag types\n* [`SignalsOfScriptEditor`][crate::classes::script_editor::SignalsOfScriptEditor]: signal collection\n\n\nSee also [Godot docs for `ScriptEditor`](https://docs.godotengine.org/en/stable/classes/class_scripteditor.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ScriptEditor>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nGodot editor's script editor.\n\n**Note:** This class shouldn't be instantiated directly. Instead, access the singleton using [`get_script_editor`][`crate::classes::EditorInterface::get_script_editor`]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptEditor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ScriptEditor {
        #[doc = "Returns the [`ScriptEditorBase`][crate::classes::ScriptEditorBase] object that the user is currently editing."]
        pub fn get_current_editor(&self,) -> Option < Gd < crate::classes::ScriptEditorBase > > {
            type CallRet = Option < Gd < crate::classes::ScriptEditorBase > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "get_current_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with all [`ScriptEditorBase`][crate::classes::ScriptEditorBase] objects which are currently open in editor."]
        pub fn get_open_script_editors(&self,) -> Array < Gd < crate::classes::ScriptEditorBase > > {
            type CallRet = Array < Gd < crate::classes::ScriptEditorBase > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "get_open_script_editors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns array of breakpoints."]
        pub fn get_breakpoints(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "get_breakpoints", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers the [`EditorSyntaxHighlighter`][crate::classes::EditorSyntaxHighlighter] to the editor, the [`EditorSyntaxHighlighter`][crate::classes::EditorSyntaxHighlighter] will be available on all open scripts.\n\n**Note:** Does not apply to scripts that are already opened."]
        pub fn register_syntax_highlighter(&mut self, syntax_highlighter: impl AsArg < Option < Gd < crate::classes::EditorSyntaxHighlighter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSyntaxHighlighter > > >,);
            let args = (syntax_highlighter.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "register_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters the [`EditorSyntaxHighlighter`][crate::classes::EditorSyntaxHighlighter] from the editor.\n\n**Note:** The [`EditorSyntaxHighlighter`][crate::classes::EditorSyntaxHighlighter] will still be applied to scripts that are already opened."]
        pub fn unregister_syntax_highlighter(&mut self, syntax_highlighter: impl AsArg < Option < Gd < crate::classes::EditorSyntaxHighlighter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSyntaxHighlighter > > >,);
            let args = (syntax_highlighter.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "unregister_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Goes to the specified line in the current script."]
        pub fn goto_line(&mut self, line_number: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line_number,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "goto_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`Script`][crate::classes::Script] that is currently active in editor."]
        pub fn get_current_script(&self,) -> Option < Gd < crate::classes::Script > > {
            type CallRet = Option < Gd < crate::classes::Script > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "get_current_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with all [`Script`][crate::classes::Script] objects which are currently open in editor."]
        pub fn get_open_scripts(&self,) -> Array < Gd < crate::classes::Script > > {
            type CallRet = Array < Gd < crate::classes::Script > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "get_open_scripts", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Opens the script create dialog. The script will extend `base_name`. The file extension can be omitted from `base_path`. It will be added based on the selected scripting language."]
        pub fn open_script_create_dialog(&mut self, base_name: impl AsArg < GString >, base_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (base_name.into_arg(), base_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "open_script_create_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Opens help for the given topic. The `topic` is an encoded string that controls which class, method, constant, signal, annotation, property, or theme item should be focused.\n\nThe supported `topic` formats include `class_name:class`, `class_method:class:method`, `class_constant:class:constant`, `class_signal:class:signal`, `class_annotation:class:@annotation`, `class_property:class:property`, and `class_theme_item:class:item`, where `class` is the class name, `method` is the method name, `constant` is the constant name, `signal` is the signal name, `annotation` is the annotation name, `property` is the property name, and `item` is the theme item.\n\n```gdscript\n# Shows help for the Node class.\nclass_name:Node\n# Shows help for the global min function.\n# Global objects are accessible in the `@GlobalScope` namespace, shown here.\nclass_method:@GlobalScope:min\n# Shows help for get_viewport in the Node class.\nclass_method:Node:get_viewport\n# Shows help for the Input constant MOUSE_BUTTON_MIDDLE.\nclass_constant:Input:MOUSE_BUTTON_MIDDLE\n# Shows help for the BaseButton signal pressed.\nclass_signal:BaseButton:pressed\n# Shows help for the CanvasItem property visible.\nclass_property:CanvasItem:visible\n# Shows help for the GDScript annotation export.\n# Annotations should be prefixed with the `@` symbol in the descriptor, as shown here.\nclass_annotation:@GDScript:@export\n# Shows help for the GraphNode theme item named panel_selected.\nclass_theme_item:GraphNode:panel_selected\n```"]
        pub fn goto_help(&mut self, topic: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (topic.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "goto_help", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the documentation for the given `script`.\n\n**Note:** This should be called whenever the script is changed to keep the open documentation state up to date."]
        pub fn update_docs_from_script(&mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Script > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "update_docs_from_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the documentation for the given `script`.\n\n**Note:** This should be called whenever the script is changed to keep the open documentation state up to date."]
        pub fn clear_docs_from_script(&mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Script > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditor", "clear_docs_from_script", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ScriptEditor {
        type Base = crate::classes::PanelContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScriptEditor"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for ScriptEditor {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PanelContainer > for ScriptEditor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for ScriptEditor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ScriptEditor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ScriptEditor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ScriptEditor {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScriptEditor {
        
    }
    impl std::ops::Deref for ScriptEditor {
        type Target = crate::classes::PanelContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptEditor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScriptEditor__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ScriptEditor` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScriptEditor;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ScriptEditor`][crate::classes::ScriptEditor] class."]
    pub struct SignalsOfScriptEditor < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfScriptEditor < 'c, C > {
        #[doc = "Signature: `(script: Gd<Script>)`"]
        pub fn editor_script_changed(&mut self) -> SigEditorScriptChanged < 'c, C > {
            SigEditorScriptChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "editor_script_changed")
            }
        }
        #[doc = "Signature: `(script: Gd<Script>)`"]
        pub fn script_close(&mut self) -> SigScriptClose < 'c, C > {
            SigScriptClose {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "script_close")
            }
        }
    }
    type TypedSigEditorScriptChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Script >,) >;
    pub struct SigEditorScriptChanged < 'c, C: WithSignals > {
        typed: TypedSigEditorScriptChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditorScriptChanged < 'c, C > {
        pub fn emit(&mut self, script: Gd < crate::classes::Script >,) {
            self.typed.emit_tuple((script,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditorScriptChanged < 'c, C > {
        type Target = TypedSigEditorScriptChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditorScriptChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigScriptClose < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Script >,) >;
    pub struct SigScriptClose < 'c, C: WithSignals > {
        typed: TypedSigScriptClose < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScriptClose < 'c, C > {
        pub fn emit(&mut self, script: Gd < crate::classes::Script >,) {
            self.typed.emit_tuple((script,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScriptClose < 'c, C > {
        type Target = TypedSigScriptClose < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScriptClose < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ScriptEditor {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfScriptEditor < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfScriptEditor < 'c, C > {
        type Target = < < ScriptEditor as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ScriptEditor;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfScriptEditor < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ScriptEditor;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}