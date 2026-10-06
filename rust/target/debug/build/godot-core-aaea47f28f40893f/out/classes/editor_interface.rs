#![doc = "Sidecar module for class [`EditorInterface`][crate::classes::EditorInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInterface`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_interface`][crate::classes::editor_interface]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorInterface`](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`EditorInterface` gives you control over Godot editor's window. It allows customizing the window, saving and (re-)loading scenes, rendering mesh previews, inspecting and editing resources and objects, and provides access to [`EditorSettings`][crate::classes::EditorSettings], [`EditorFileSystem`][crate::classes::EditorFileSystem], [`EditorResourcePreview`][crate::classes::EditorResourcePreview], [`ScriptEditor`][crate::classes::ScriptEditor], the editor viewport, and information about scenes.\n\n**Note:** This class shouldn't be instantiated directly. Instead, access the singleton directly by its name.\n\n\n```gdscript\nvar editor_settings = EditorInterface.get_editor_settings()\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorInterface {
        #[doc = "Restarts the editor. This closes the editor and then opens the same project. If `save` is `true`, the project will be saved before restarting."]
        pub(crate) fn restart_editor_full(&mut self, save: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (save,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "restart_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`restart_editor_ex`][Self::restart_editor_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Restarts the editor. This closes the editor and then opens the same project. If `save` is `true`, the project will be saved before restarting."]
        #[inline]
        pub fn restart_editor(&mut self,) {
            self.restart_editor_ex() . done()
        }
        #[doc = "Restarts the editor. This closes the editor and then opens the same project. If `save` is `true`, the project will be saved before restarting."]
        #[inline]
        pub fn restart_editor_ex < 'ex > (&'ex mut self,) -> ExRestartEditor < 'ex > {
            ExRestartEditor::new(self,)
        }
        #[doc = "Returns the editor's [`EditorCommandPalette`][crate::classes::EditorCommandPalette] instance.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_command_palette(&self,) -> Option < Gd < crate::classes::EditorCommandPalette > > {
            type CallRet = Option < Gd < crate::classes::EditorCommandPalette > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_command_palette", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorFileSystem`][crate::classes::EditorFileSystem] instance."]
        pub fn get_resource_filesystem(&self,) -> Option < Gd < crate::classes::EditorFileSystem > > {
            type CallRet = Option < Gd < crate::classes::EditorFileSystem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_filesystem", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`EditorPaths`][crate::classes::EditorPaths] singleton."]
        pub fn get_editor_paths(&self,) -> Option < Gd < crate::classes::EditorPaths > > {
            type CallRet = Option < Gd < crate::classes::EditorPaths > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_paths", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorResourcePreview`][crate::classes::EditorResourcePreview] instance."]
        pub fn get_resource_previewer(&self,) -> Option < Gd < crate::classes::EditorResourcePreview > > {
            type CallRet = Option < Gd < crate::classes::EditorResourcePreview > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_previewer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorSelection`][crate::classes::EditorSelection] instance."]
        pub fn get_selection(&self,) -> Option < Gd < crate::classes::EditorSelection > > {
            type CallRet = Option < Gd < crate::classes::EditorSelection > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorSettings`][crate::classes::EditorSettings] instance."]
        pub fn get_editor_settings(&self,) -> Option < Gd < crate::classes::EditorSettings > > {
            type CallRet = Option < Gd < crate::classes::EditorSettings > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_settings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorToaster`][crate::classes::EditorToaster]."]
        pub fn get_editor_toaster(&self,) -> Option < Gd < crate::classes::EditorToaster > > {
            type CallRet = Option < Gd < crate::classes::EditorToaster > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_toaster", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager]."]
        pub fn get_editor_undo_redo(&self,) -> Option < Gd < crate::classes::EditorUndoRedoManager > > {
            type CallRet = Option < Gd < crate::classes::EditorUndoRedoManager > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_undo_redo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns mesh previews rendered at the given size as an [`Array`][crate::builtin::Array] of [`Texture2D`][crate::classes::Texture2D]s."]
        pub fn make_mesh_previews(&mut self, meshes: &Array < Gd < crate::classes::Mesh > >, preview_size: i32,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallRet = Array < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Mesh > > >, i32,);
            let args = (RefArg::new(meshes), preview_size,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "make_mesh_previews", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the enabled status of a plugin. The plugin name is the same as its directory name."]
        pub fn set_plugin_enabled(&mut self, plugin: impl AsArg < GString >, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (plugin.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_plugin_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the specified `plugin` is enabled. The plugin name is the same as its directory name."]
        pub fn is_plugin_enabled(&self, plugin: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_plugin_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`Theme`][crate::classes::Theme].\n\n**Note:** When creating custom editor UI, prefer accessing theme items directly from your GUI nodes using the `get_theme_*` methods."]
        pub fn get_editor_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_theme", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the main container of Godot editor's window. For example, you can use it to retrieve the size of the container and place your controls accordingly.\n\n**Warning:** Removing and freeing this node will render the editor useless and may cause a crash."]
        pub fn get_base_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_base_control", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor control responsible for main screen plugins and tools. Use it with plugins that implement [`has_main_screen`][`crate::classes::IEditorPlugin::has_main_screen`].\n\n**Note:** This node is a [`VBoxContainer`][crate::classes::VBoxContainer], which means that if you add a [`Control`][crate::classes::Control] child to it, you need to set the child's \\[member Control.size_flags_vertical] to [`SizeFlags::EXPAND_FILL`][`crate::classes::control::SizeFlags::EXPAND_FILL`] to make it use the full available space.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_editor_main_screen(&self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallRet = Option < Gd < crate::classes::VBoxContainer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_main_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`ScriptEditor`][crate::classes::ScriptEditor] instance.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_script_editor(&self,) -> Option < Gd < crate::classes::ScriptEditor > > {
            type CallRet = Option < Gd < crate::classes::ScriptEditor > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_script_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the 2D editor [`SubViewport`][crate::classes::SubViewport]. It does not have a camera. Instead, the view transforms are done directly and can be accessed with \\[member Viewport.global_canvas_transform]."]
        pub fn get_editor_viewport_2d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the specified 3D editor [`SubViewport`][crate::classes::SubViewport], from `0` to `3`. The viewport can be used to access the active editor cameras with [`get_camera_3d`][`crate::classes::Viewport::get_camera_3d`]."]
        pub(crate) fn get_editor_viewport_3d_full(&self, idx: i32,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_editor_viewport_3d_ex`][Self::get_editor_viewport_3d_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the specified 3D editor [`SubViewport`][crate::classes::SubViewport], from `0` to `3`. The viewport can be used to access the active editor cameras with [`get_camera_3d`][`crate::classes::Viewport::get_camera_3d`]."]
        #[inline]
        pub fn get_editor_viewport_3d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            self.get_editor_viewport_3d_ex() . done()
        }
        #[doc = "Returns the specified 3D editor [`SubViewport`][crate::classes::SubViewport], from `0` to `3`. The viewport can be used to access the active editor cameras with [`get_camera_3d`][`crate::classes::Viewport::get_camera_3d`]."]
        #[inline]
        pub fn get_editor_viewport_3d_ex < 'ex > (&'ex self,) -> ExGetEditorViewport3d < 'ex > {
            ExGetEditorViewport3d::new(self,)
        }
        #[doc = "Sets the editor's current main screen to the one specified in `name`. `name` must match the title of the tab in question exactly (e.g. `2D`, `3D`, `Script`, `Game`, or `AssetLib` for default tabs)."]
        pub fn set_main_screen_editor(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_main_screen_editor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distraction_free_mode(&mut self, enter: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_distraction_free_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_distraction_free_mode_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_distraction_free_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if multiple window support is enabled in the editor. Multiple window support is enabled if _all_ of these statements are true:\n\n- \\[member EditorSettings.interface/multi_window/enable] is `true`.\n\n- \\[member EditorSettings.interface/editor/single_window_mode] is `false`.\n\n- \\[member Viewport.gui_embed_subwindows] is `false`. This is forced to `true` on platforms that don't support multiple windows such as Web, or when the `--single-window` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html) is used."]
        pub fn is_multi_window_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_multi_window_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the actual scale of the editor UI (`1.0` being 100% scale). This can be used to adjust position and dimensions of the UI added by plugins.\n\n**Note:** This value is set via the \\[member EditorSettings.interface/editor/display_scale] and \\[member EditorSettings.interface/editor/custom_display_scale] settings. The editor must be restarted for changes to be properly applied."]
        pub fn get_editor_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the language currently used for the editor interface."]
        pub fn get_editor_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the 3D editor currently has snapping mode enabled, and `false` otherwise."]
        pub fn is_node_3d_snap_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_node_3d_snap_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of units the 3D editor's translation snapping is set to."]
        pub fn get_node_3d_translate_snap(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_node_3d_translate_snap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of degrees the 3D editor's rotational snapping is set to."]
        pub fn get_node_3d_rotate_snap(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_node_3d_rotate_snap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of units the 3D editor's scale snapping is set to."]
        pub fn get_node_3d_scale_snap(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_node_3d_scale_snap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive`][`crate::classes::Window::popup_exclusive`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        pub(crate) fn popup_dialog_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, rect: Rect2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Rect2i,);
            let args = (dialog, rect,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_dialog_ex`][Self::popup_dialog_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive`][`crate::classes::Window::popup_exclusive`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_ex(dialog,) . done()
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive`][`crate::classes::Window::popup_exclusive`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_ex < 'ex > (&'ex mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> ExPopupDialog < 'ex > {
            ExPopupDialog::new(self, dialog,)
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered`][`crate::classes::Window::popup_exclusive_centered`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        pub(crate) fn popup_dialog_centered_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, minsize: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Vector2i,);
            let args = (dialog, minsize,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_dialog_centered_ex`][Self::popup_dialog_centered_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered`][`crate::classes::Window::popup_exclusive_centered`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_ex(dialog,) . done()
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered`][`crate::classes::Window::popup_exclusive_centered`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered_ex < 'ex > (&'ex mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> ExPopupDialogCentered < 'ex > {
            ExPopupDialogCentered::new(self, dialog,)
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_ratio`][`crate::classes::Window::popup_exclusive_centered_ratio`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        pub(crate) fn popup_dialog_centered_ratio_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, f32,);
            let args = (dialog, ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_dialog_centered_ratio_ex`][Self::popup_dialog_centered_ratio_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_ratio`][`crate::classes::Window::popup_exclusive_centered_ratio`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered_ratio(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_ratio_ex(dialog,) . done()
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_ratio`][`crate::classes::Window::popup_exclusive_centered_ratio`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered_ratio_ex < 'ex > (&'ex mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> ExPopupDialogCenteredRatio < 'ex > {
            ExPopupDialogCenteredRatio::new(self, dialog,)
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_clamped`][`crate::classes::Window::popup_exclusive_centered_clamped`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        pub(crate) fn popup_dialog_centered_clamped_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, minsize: Vector2i, fallback_ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Vector2i, f32,);
            let args = (dialog, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_clamped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_dialog_centered_clamped_ex`][Self::popup_dialog_centered_clamped_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_clamped`][`crate::classes::Window::popup_exclusive_centered_clamped`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered_clamped(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_clamped_ex(dialog,) . done()
        }
        #[doc = "Pops up the `dialog` in the editor UI with [`popup_exclusive_centered_clamped`][`crate::classes::Window::popup_exclusive_centered_clamped`]. The dialog must have no current parent, otherwise the method fails.\n\nSee also [`set_unparent_when_invisible`][`crate::classes::Window::set_unparent_when_invisible`]."]
        #[inline]
        pub fn popup_dialog_centered_clamped_ex < 'ex > (&'ex mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> ExPopupDialogCenteredClamped < 'ex > {
            ExPopupDialogCenteredClamped::new(self, dialog,)
        }
        #[doc = "Returns the name of the currently activated feature profile. If the default profile is currently active, an empty string is returned instead.\n\nIn order to get a reference to the [`EditorFeatureProfile`][crate::classes::EditorFeatureProfile], you must load the feature profile using [`load_from_file`][`crate::classes::EditorFeatureProfile::load_from_file`].\n\n**Note:** Feature profiles created via the user interface are loaded from the `feature_profiles` directory, as a file with the `.profile` extension. The editor configuration folder can be found by using [`get_config_dir`][`crate::classes::EditorPaths::get_config_dir`]."]
        pub fn get_current_feature_profile(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_feature_profile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects and activates the specified feature profile with the given `profile_name`. Set `profile_name` to an empty string to reset to the default feature profile.\n\nA feature profile can be created programmatically using the [`EditorFeatureProfile`][crate::classes::EditorFeatureProfile] class.\n\n**Note:** The feature profile that gets activated must be located in the `feature_profiles` directory, as a file with the `.profile` extension. If a profile could not be found, an error occurs. The editor configuration folder can be found by using [`get_config_dir`][`crate::classes::EditorPaths::get_config_dir`]."]
        pub fn set_current_feature_profile(&mut self, profile_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (profile_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_current_feature_profile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pops up an editor dialog for selecting a [`Node`][crate::classes::Node] from the edited scene. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected [`NodePath`][crate::builtin::NodePath] or the empty path `^\"\"` if the dialog is canceled. If `valid_types` is provided, the dialog will only show Nodes that match one of the listed Node types. If `current_value` is provided, the Node will be automatically selected in the tree, if it exists.\n\n**Example:** Display the node selection dialog as soon as this node is added to the tree for the first time:\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_node_selector(_on_node_selected, [\"Button\"])\n\nfunc _on_node_selected(node_path):\n\tif node_path.is_empty():\n\t\tprint(\"node selection canceled\")\n\telse:\n\t\tprint(\"selected \", node_path)\n```"]
        pub(crate) fn popup_node_selector_full(&mut self, callback: RefArg < Callable >, valid_types: RefArg < Array < StringName > >, current_value: CowArg < Option < Gd < crate::classes::Node > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Array < StringName > >, CowArg < 'a2, Option < Gd < crate::classes::Node > > >,);
            let args = (callback, valid_types, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_node_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_node_selector_ex`][Self::popup_node_selector_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up an editor dialog for selecting a [`Node`][crate::classes::Node] from the edited scene. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected [`NodePath`][crate::builtin::NodePath] or the empty path `^\"\"` if the dialog is canceled. If `valid_types` is provided, the dialog will only show Nodes that match one of the listed Node types. If `current_value` is provided, the Node will be automatically selected in the tree, if it exists.\n\n**Example:** Display the node selection dialog as soon as this node is added to the tree for the first time:\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_node_selector(_on_node_selected, [\"Button\"])\n\nfunc _on_node_selected(node_path):\n\tif node_path.is_empty():\n\t\tprint(\"node selection canceled\")\n\telse:\n\t\tprint(\"selected \", node_path)\n```"]
        #[inline]
        pub fn popup_node_selector(&mut self, callback: &Callable,) {
            self.popup_node_selector_ex(callback,) . done()
        }
        #[doc = "Pops up an editor dialog for selecting a [`Node`][crate::classes::Node] from the edited scene. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected [`NodePath`][crate::builtin::NodePath] or the empty path `^\"\"` if the dialog is canceled. If `valid_types` is provided, the dialog will only show Nodes that match one of the listed Node types. If `current_value` is provided, the Node will be automatically selected in the tree, if it exists.\n\n**Example:** Display the node selection dialog as soon as this node is added to the tree for the first time:\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_node_selector(_on_node_selected, [\"Button\"])\n\nfunc _on_node_selected(node_path):\n\tif node_path.is_empty():\n\t\tprint(\"node selection canceled\")\n\telse:\n\t\tprint(\"selected \", node_path)\n```"]
        #[inline]
        pub fn popup_node_selector_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExPopupNodeSelector < 'ex > {
            ExPopupNodeSelector::new(self, callback,)
        }
        #[doc = "Pops up an editor dialog for selecting properties from `object`. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected property path (see [`get_as_property_path`][`crate::builtin::NodePath::get_as_property_path`]) or the empty path `^\"\"` if the dialog is canceled. If `type_filter` is provided, the dialog will only show properties that match one of the listed \\[enum Variant.Type] values. If `current_value` is provided, the property will be selected automatically in the property list, if it exists.\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_property_selector(this, _on_property_selected, [TYPE_INT])\n\nfunc _on_property_selected(property_path):\n\tif property_path.is_empty():\n\t\tprint(\"property selection canceled\")\n\telse:\n\t\tprint(\"selected \", property_path)\n```"]
        pub(crate) fn popup_property_selector_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, callback: RefArg < Callable >, type_filter: RefArg < PackedInt32Array >, current_value: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Callable >, RefArg < 'a2, PackedInt32Array >, CowArg < 'a3, GString >,);
            let args = (object, callback, type_filter, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_property_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_property_selector_ex`][Self::popup_property_selector_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up an editor dialog for selecting properties from `object`. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected property path (see [`get_as_property_path`][`crate::builtin::NodePath::get_as_property_path`]) or the empty path `^\"\"` if the dialog is canceled. If `type_filter` is provided, the dialog will only show properties that match one of the listed \\[enum Variant.Type] values. If `current_value` is provided, the property will be selected automatically in the property list, if it exists.\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_property_selector(this, _on_property_selected, [TYPE_INT])\n\nfunc _on_property_selected(property_path):\n\tif property_path.is_empty():\n\t\tprint(\"property selection canceled\")\n\telse:\n\t\tprint(\"selected \", property_path)\n```"]
        #[inline]
        pub fn popup_property_selector(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: &Callable,) {
            self.popup_property_selector_ex(object, callback,) . done()
        }
        #[doc = "Pops up an editor dialog for selecting properties from `object`. The `callback` must take a single argument of type [`NodePath`][crate::builtin::NodePath]. It is called on the selected property path (see [`get_as_property_path`][`crate::builtin::NodePath::get_as_property_path`]) or the empty path `^\"\"` if the dialog is canceled. If `type_filter` is provided, the dialog will only show properties that match one of the listed \\[enum Variant.Type] values. If `current_value` is provided, the property will be selected automatically in the property list, if it exists.\n\n```gdscript\nfunc _ready():\n\tif Engine.is_editor_hint():\n\t\tEditorInterface.popup_property_selector(this, _on_property_selected, [TYPE_INT])\n\nfunc _on_property_selected(property_path):\n\tif property_path.is_empty():\n\t\tprint(\"property selection canceled\")\n\telse:\n\t\tprint(\"selected \", property_path)\n```"]
        #[inline]
        pub fn popup_property_selector_ex < 'ex > (&'ex mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, callback: &'ex Callable,) -> ExPopupPropertySelector < 'ex > {
            ExPopupPropertySelector::new(self, object, callback,)
        }
        #[doc = "Pops up an editor dialog for selecting a method from `object`. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the name of the selected method or be empty if the dialog is canceled. If `current_value` is provided, the method will be selected automatically in the method list, if it exists."]
        pub(crate) fn popup_method_selector_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, callback: RefArg < Callable >, current_value: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Callable >, CowArg < 'a2, GString >,);
            let args = (object, callback, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_method_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_method_selector_ex`][Self::popup_method_selector_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up an editor dialog for selecting a method from `object`. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the name of the selected method or be empty if the dialog is canceled. If `current_value` is provided, the method will be selected automatically in the method list, if it exists."]
        #[inline]
        pub fn popup_method_selector(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: &Callable,) {
            self.popup_method_selector_ex(object, callback,) . done()
        }
        #[doc = "Pops up an editor dialog for selecting a method from `object`. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the name of the selected method or be empty if the dialog is canceled. If `current_value` is provided, the method will be selected automatically in the method list, if it exists."]
        #[inline]
        pub fn popup_method_selector_ex < 'ex > (&'ex mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, callback: &'ex Callable,) -> ExPopupMethodSelector < 'ex > {
            ExPopupMethodSelector::new(self, object, callback,)
        }
        #[doc = "Pops up an editor dialog for quick selecting a resource file. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the path of the selected resource or be empty if the dialog is canceled. If `base_types` is provided, the dialog will only show resources that match these types. Only types deriving from [`Resource`][crate::classes::Resource] are supported."]
        pub(crate) fn popup_quick_open_full(&mut self, callback: RefArg < Callable >, base_types: RefArg < Array < StringName > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Array < StringName > >,);
            let args = (callback, base_types,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_quick_open", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_quick_open_ex`][Self::popup_quick_open_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up an editor dialog for quick selecting a resource file. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the path of the selected resource or be empty if the dialog is canceled. If `base_types` is provided, the dialog will only show resources that match these types. Only types deriving from [`Resource`][crate::classes::Resource] are supported."]
        #[inline]
        pub fn popup_quick_open(&mut self, callback: &Callable,) {
            self.popup_quick_open_ex(callback,) . done()
        }
        #[doc = "Pops up an editor dialog for quick selecting a resource file. The `callback` must take a single argument of type [`String`][crate::builtin::GString] which will contain the path of the selected resource or be empty if the dialog is canceled. If `base_types` is provided, the dialog will only show resources that match these types. Only types deriving from [`Resource`][crate::classes::Resource] are supported."]
        #[inline]
        pub fn popup_quick_open_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExPopupQuickOpen < 'ex > {
            ExPopupQuickOpen::new(self, callback,)
        }
        #[doc = "Pops up an editor dialog for creating an object.\n\nThe `callback` must take a single argument of type [`String`][crate::builtin::GString], which will contain the type name of the selected object (or the script path of the type, if the type is created from a script), or be an empty string if no item is selected.\n\nThe `base_type` specifies the base type of objects to display. For example, if you set this to \"Resource\", all types derived from [`Resource`][crate::classes::Resource] will display in the create dialog.\n\nThe `current_type` will be passed in the search box of the create dialog, and the specified type can be immediately selected when the dialog pops up. If the `current_type` is not derived from `base_type`, there will be no result of the type in the dialog.\n\nThe `dialog_title` allows you to define a custom title for the dialog. This is useful if you want to accurately hint the usage of the dialog. If the `dialog_title` is an empty string, the dialog will use \"Create New 'Base Type'\" as the default title.\n\nThe `type_blocklist` contains a list of type names, and the types in the blocklist will be hidden from the create dialog.\n\n**Note:** Trying to list the base type in the `type_blocklist` will hide all types derived from the base type from the create dialog."]
        pub(crate) fn popup_create_dialog_full(&mut self, callback: RefArg < Callable >, base_type: CowArg < StringName >, current_type: CowArg < GString >, dialog_title: CowArg < GString >, type_blocklist: RefArg < Array < StringName > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (RefArg < 'a0, Callable >, CowArg < 'a1, StringName >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, RefArg < 'a4, Array < StringName > >,);
            let args = (callback, base_type, current_type, dialog_title, type_blocklist,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_create_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`popup_create_dialog_ex`][Self::popup_create_dialog_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Pops up an editor dialog for creating an object.\n\nThe `callback` must take a single argument of type [`String`][crate::builtin::GString], which will contain the type name of the selected object (or the script path of the type, if the type is created from a script), or be an empty string if no item is selected.\n\nThe `base_type` specifies the base type of objects to display. For example, if you set this to \"Resource\", all types derived from [`Resource`][crate::classes::Resource] will display in the create dialog.\n\nThe `current_type` will be passed in the search box of the create dialog, and the specified type can be immediately selected when the dialog pops up. If the `current_type` is not derived from `base_type`, there will be no result of the type in the dialog.\n\nThe `dialog_title` allows you to define a custom title for the dialog. This is useful if you want to accurately hint the usage of the dialog. If the `dialog_title` is an empty string, the dialog will use \"Create New 'Base Type'\" as the default title.\n\nThe `type_blocklist` contains a list of type names, and the types in the blocklist will be hidden from the create dialog.\n\n**Note:** Trying to list the base type in the `type_blocklist` will hide all types derived from the base type from the create dialog."]
        #[inline]
        pub fn popup_create_dialog(&mut self, callback: &Callable,) {
            self.popup_create_dialog_ex(callback,) . done()
        }
        #[doc = "Pops up an editor dialog for creating an object.\n\nThe `callback` must take a single argument of type [`String`][crate::builtin::GString], which will contain the type name of the selected object (or the script path of the type, if the type is created from a script), or be an empty string if no item is selected.\n\nThe `base_type` specifies the base type of objects to display. For example, if you set this to \"Resource\", all types derived from [`Resource`][crate::classes::Resource] will display in the create dialog.\n\nThe `current_type` will be passed in the search box of the create dialog, and the specified type can be immediately selected when the dialog pops up. If the `current_type` is not derived from `base_type`, there will be no result of the type in the dialog.\n\nThe `dialog_title` allows you to define a custom title for the dialog. This is useful if you want to accurately hint the usage of the dialog. If the `dialog_title` is an empty string, the dialog will use \"Create New 'Base Type'\" as the default title.\n\nThe `type_blocklist` contains a list of type names, and the types in the blocklist will be hidden from the create dialog.\n\n**Note:** Trying to list the base type in the `type_blocklist` will hide all types derived from the base type from the create dialog."]
        #[inline]
        pub fn popup_create_dialog_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExPopupCreateDialog < 'ex > {
            ExPopupCreateDialog::new(self, callback,)
        }
        #[doc = "Returns the editor's [`FileSystemDock`][crate::classes::FileSystemDock] instance.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_file_system_dock(&self,) -> Option < Gd < crate::classes::FileSystemDock > > {
            type CallRet = Option < Gd < crate::classes::FileSystemDock > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_file_system_dock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the file, with the path provided by `file`, in the FileSystem dock."]
        pub fn select_file(&mut self, file: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "select_file", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array containing the paths of the currently selected files (and directories) in the [`FileSystemDock`][crate::classes::FileSystemDock]."]
        pub fn get_selected_paths(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selected_paths", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current path being viewed in the [`FileSystemDock`][crate::classes::FileSystemDock]."]
        pub fn get_current_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current directory being viewed in the [`FileSystemDock`][crate::classes::FileSystemDock]. If a file is selected, its base directory will be returned using [`get_base_dir`][`crate::builtin::GString::get_base_dir`] instead."]
        pub fn get_current_directory(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_directory", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the editor's [`EditorInspector`][crate::classes::EditorInspector] instance.\n\n**Warning:** Removing and freeing this node will render a part of the editor useless and may cause a crash."]
        pub fn get_inspector(&self,) -> Option < Gd < crate::classes::EditorInspector > > {
            type CallRet = Option < Gd < crate::classes::EditorInspector > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_inspector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows the given property on the given `object` in the editor's Inspector dock. If `inspector_only` is `true`, plugins will not attempt to edit `object`."]
        pub(crate) fn inspect_object_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, for_property: CowArg < GString >, inspector_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, GString >, bool,);
            let args = (object, for_property, inspector_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "inspect_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`inspect_object_ex`][Self::inspect_object_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Shows the given property on the given `object` in the editor's Inspector dock. If `inspector_only` is `true`, plugins will not attempt to edit `object`."]
        #[inline]
        pub fn inspect_object(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            self.inspect_object_ex(object,) . done()
        }
        #[doc = "Shows the given property on the given `object` in the editor's Inspector dock. If `inspector_only` is `true`, plugins will not attempt to edit `object`."]
        #[inline]
        pub fn inspect_object_ex < 'ex > (&'ex mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex,) -> ExInspectObject < 'ex > {
            ExInspectObject::new(self, object,)
        }
        #[doc = "Edits the given [`Resource`][crate::classes::Resource]. If the resource is a [`Script`][crate::classes::Script] you can also edit it with [`edit_script`][`crate::classes::EditorInterface::edit_script`] to specify the line and column position."]
        pub fn edit_resource(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Edits the given [`Node`][crate::classes::Node]. The node will be also selected if it's inside the scene tree."]
        pub fn edit_node(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Edits the given [`Script`][crate::classes::Script]. The line and column on which to open the script can also be specified. The script will be open with the user-configured editor for the script's language which may be an external editor."]
        pub(crate) fn edit_script_full(&mut self, script: CowArg < Option < Gd < crate::classes::Script > > >, line: i32, column: i32, grab_focus: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Script > > >, i32, i32, bool,);
            let args = (script, line, column, grab_focus,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`edit_script_ex`][Self::edit_script_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Edits the given [`Script`][crate::classes::Script]. The line and column on which to open the script can also be specified. The script will be open with the user-configured editor for the script's language which may be an external editor."]
        #[inline]
        pub fn edit_script(&mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> >,) {
            self.edit_script_ex(script,) . done()
        }
        #[doc = "Edits the given [`Script`][crate::classes::Script]. The line and column on which to open the script can also be specified. The script will be open with the user-configured editor for the script's language which may be an external editor."]
        #[inline]
        pub fn edit_script_ex < 'ex > (&'ex mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> > + 'ex,) -> ExEditScript < 'ex > {
            ExEditScript::new(self, script,)
        }
        #[doc = "Opens the scene at the given path. If `set_inherited` is `true`, creates a new inherited scene."]
        pub(crate) fn open_scene_from_path_full(&mut self, scene_filepath: CowArg < GString >, set_inherited: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (scene_filepath, set_inherited,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "open_scene_from_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`open_scene_from_path_ex`][Self::open_scene_from_path_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Opens the scene at the given path. If `set_inherited` is `true`, creates a new inherited scene."]
        #[inline]
        pub fn open_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            self.open_scene_from_path_ex(scene_filepath,) . done()
        }
        #[doc = "Opens the scene at the given path. If `set_inherited` is `true`, creates a new inherited scene."]
        #[inline]
        pub fn open_scene_from_path_ex < 'ex > (&'ex mut self, scene_filepath: impl AsArg < GString > + 'ex,) -> ExOpenSceneFromPath < 'ex > {
            ExOpenSceneFromPath::new(self, scene_filepath,)
        }
        #[doc = "Reloads the scene at the given path."]
        pub fn reload_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "reload_scene_from_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `edited` is `true`, the object is marked as edited.\n\n**Note:** This is primarily used by the editor for [`Resource`][crate::classes::Resource] based objects to track their modified state. For example, any changes to an open scene, a resource in the inspector, or an edited script will cause this method to be called with `true`. Saving the scene, script, or resource resets the edited state by calling this method with `false`.\n\n**Note:** Each call to this method increments the object's edited version. This is used to track changes in the editor and to trigger when thumbnails should be regenerated for resources."]
        pub fn set_object_edited(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, edited: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, bool,);
            let args = (object.into_arg(), edited,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_object_edited", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the object has been marked as edited through [`set_object_edited`][`crate::classes::EditorInterface::set_object_edited`]."]
        pub fn is_object_edited(&self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_object_edited", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with the file paths of the currently opened scenes."]
        pub fn get_open_scenes(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_open_scenes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array with references to the root nodes of the currently opened scenes."]
        pub fn get_open_scene_roots(&self,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_open_scene_roots", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the edited (current) scene's root [`Node`][crate::classes::Node]."]
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_edited_scene_root", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes `node` root of the currently opened scene. Only works if the scene is empty. If the `node` is a scene instance, an inheriting scene will be created."]
        pub fn add_root_node(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "add_root_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the currently active scene. Returns either [`Error::OK`][`crate::global::Error::OK`] or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`]."]
        pub fn save_scene(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the currently active scene as a file at `path`."]
        pub(crate) fn save_scene_as_full(&mut self, path: CowArg < GString >, with_preview: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, with_preview,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene_as", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`save_scene_as_ex`][Self::save_scene_as_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Saves the currently active scene as a file at `path`."]
        #[inline]
        pub fn save_scene_as(&mut self, path: impl AsArg < GString >,) {
            self.save_scene_as_ex(path,) . done()
        }
        #[doc = "Saves the currently active scene as a file at `path`."]
        #[inline]
        pub fn save_scene_as_ex < 'ex > (&'ex mut self, path: impl AsArg < GString > + 'ex,) -> ExSaveSceneAs < 'ex > {
            ExSaveSceneAs::new(self, path,)
        }
        #[doc = "Saves all opened scenes in the editor."]
        pub fn save_all_scenes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_all_scenes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes the currently active scene, discarding any pending changes in the process. Returns [`Error::OK`][`crate::global::Error::OK`] on success or [`Error::ERR_DOES_NOT_EXIST`][`crate::global::Error::ERR_DOES_NOT_EXIST`] if there is no scene to close."]
        pub fn close_scene(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "close_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Marks the current scene tab as unsaved."]
        pub fn mark_scene_as_unsaved(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "mark_scene_as_unsaved", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Plays the main scene."]
        pub fn play_main_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_main_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Plays the currently active scene."]
        pub fn play_current_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Plays the scene specified by its filepath."]
        pub fn play_custom_scene(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_custom_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops the scene that is currently playing."]
        pub fn stop_playing_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "stop_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a scene is currently being played, `false` otherwise. Paused scenes are considered as being played."]
        pub fn is_playing_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the scene that is being played. If no scene is currently being played, returns an empty string."]
        pub fn get_playing_scene(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_movie_maker_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_movie_maker_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_movie_maker_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_movie_maker_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorInterface {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorInterface {
        
    }
    impl crate::obj::Singleton for EditorInterface {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"EditorInterface"))
            }
        }
    }
    impl std::ops::Deref for EditorInterface {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorInterface__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorInterface` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorInterface::restart_editor_ex`][super::EditorInterface::restart_editor_ex]."]
#[must_use]
pub struct ExRestartEditor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, save: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRestartEditor < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface,) -> Self {
        let save = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, save: save,
        }
    }
    #[inline]
    pub fn save(self, save: bool) -> Self {
        Self {
            save: save, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, save,
        }
        = self;
        re_export::EditorInterface::restart_editor_full(surround_object, save,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::get_editor_viewport_3d_ex`][super::EditorInterface::get_editor_viewport_3d_ex]."]
#[must_use]
pub struct ExGetEditorViewport3d < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::EditorInterface, idx: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetEditorViewport3d < 'ex > {
    fn new(surround_object: &'ex re_export::EditorInterface,) -> Self {
        let idx = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx,
        }
    }
    #[inline]
    pub fn idx(self, idx: i32) -> Self {
        Self {
            idx: idx, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::SubViewport > > {
        let Self {
            _phantom, surround_object, idx,
        }
        = self;
        re_export::EditorInterface::get_editor_viewport_3d_full(surround_object, idx,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_ex`][super::EditorInterface::popup_dialog_ex]."]
#[must_use]
pub struct ExPopupDialog < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, dialog: CowArg < 'ex, Option < Gd < crate::classes::Window > > >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupDialog < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> Self {
        let rect = Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2i) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, rect,
        }
        = self;
        re_export::EditorInterface::popup_dialog_full(surround_object, dialog, rect,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ex`][super::EditorInterface::popup_dialog_centered_ex]."]
#[must_use]
pub struct ExPopupDialogCentered < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, dialog: CowArg < 'ex, Option < Gd < crate::classes::Window > > >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupDialogCentered < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), minsize: minsize,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_full(surround_object, dialog, minsize,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ratio_ex`][super::EditorInterface::popup_dialog_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredRatio < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, dialog: CowArg < 'ex, Option < Gd < crate::classes::Window > > >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupDialogCenteredRatio < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> Self {
        let ratio = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: f32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_ratio_full(surround_object, dialog, ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_clamped_ex`][super::EditorInterface::popup_dialog_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredClamped < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, dialog: CowArg < 'ex, Option < Gd < crate::classes::Window > > >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupDialogCenteredClamped < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'ex,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        let fallback_ratio = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), minsize: minsize, fallback_ratio: fallback_ratio,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, fallback_ratio: f32) -> Self {
        Self {
            fallback_ratio: fallback_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize, fallback_ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_clamped_full(surround_object, dialog, minsize, fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_node_selector_ex`][super::EditorInterface::popup_node_selector_ex]."]
#[must_use]
pub struct ExPopupNodeSelector < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, callback: CowArg < 'ex, Callable >, valid_types: CowArg < 'ex, Array < StringName > >, current_value: CowArg < 'ex, Option < Gd < crate::classes::Node > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupNodeSelector < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, callback: &'ex Callable,) -> Self {
        let valid_types = Array::new();
        let current_value = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), valid_types: CowArg::Owned(valid_types), current_value: current_value.into_arg(),
        }
    }
    #[inline]
    pub fn valid_types(self, valid_types: &'ex Array < StringName >) -> Self {
        Self {
            valid_types: CowArg::Borrowed(valid_types), .. self
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < Option < Gd < crate::classes::Node >> > + 'ex) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, valid_types, current_value,
        }
        = self;
        re_export::EditorInterface::popup_node_selector_full(surround_object, callback.cow_as_arg(), valid_types.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_property_selector_ex`][super::EditorInterface::popup_property_selector_ex]."]
#[must_use]
pub struct ExPopupPropertySelector < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, object: CowArg < 'ex, Option < Gd < crate::classes::Object > > >, callback: CowArg < 'ex, Callable >, type_filter: CowArg < 'ex, PackedInt32Array >, current_value: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupPropertySelector < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, callback: &'ex Callable,) -> Self {
        let type_filter = PackedInt32Array::new();
        let current_value = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), callback: CowArg::Borrowed(callback), type_filter: CowArg::Owned(type_filter), current_value: CowArg::Owned(current_value),
        }
    }
    #[inline]
    pub fn type_filter(self, type_filter: &'ex PackedInt32Array) -> Self {
        Self {
            type_filter: CowArg::Borrowed(type_filter), .. self
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < GString > + 'ex) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, callback, type_filter, current_value,
        }
        = self;
        re_export::EditorInterface::popup_property_selector_full(surround_object, object, callback.cow_as_arg(), type_filter.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_method_selector_ex`][super::EditorInterface::popup_method_selector_ex]."]
#[must_use]
pub struct ExPopupMethodSelector < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, object: CowArg < 'ex, Option < Gd < crate::classes::Object > > >, callback: CowArg < 'ex, Callable >, current_value: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupMethodSelector < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, callback: &'ex Callable,) -> Self {
        let current_value = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), callback: CowArg::Borrowed(callback), current_value: CowArg::Owned(current_value),
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < GString > + 'ex) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, callback, current_value,
        }
        = self;
        re_export::EditorInterface::popup_method_selector_full(surround_object, object, callback.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_quick_open_ex`][super::EditorInterface::popup_quick_open_ex]."]
#[must_use]
pub struct ExPopupQuickOpen < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, callback: CowArg < 'ex, Callable >, base_types: CowArg < 'ex, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupQuickOpen < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, callback: &'ex Callable,) -> Self {
        let base_types = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), base_types: CowArg::Owned(base_types),
        }
    }
    #[inline]
    pub fn base_types(self, base_types: &'ex Array < StringName >) -> Self {
        Self {
            base_types: CowArg::Borrowed(base_types), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, base_types,
        }
        = self;
        re_export::EditorInterface::popup_quick_open_full(surround_object, callback.cow_as_arg(), base_types.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_create_dialog_ex`][super::EditorInterface::popup_create_dialog_ex]."]
#[must_use]
pub struct ExPopupCreateDialog < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, callback: CowArg < 'ex, Callable >, base_type: CowArg < 'ex, StringName >, current_type: CowArg < 'ex, GString >, dialog_title: CowArg < 'ex, GString >, type_blocklist: CowArg < 'ex, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPopupCreateDialog < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, callback: &'ex Callable,) -> Self {
        let base_type = StringName::from("");
        let current_type = GString::from("");
        let dialog_title = GString::from("");
        let type_blocklist = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), base_type: CowArg::Owned(base_type), current_type: CowArg::Owned(current_type), dialog_title: CowArg::Owned(dialog_title), type_blocklist: CowArg::Owned(type_blocklist),
        }
    }
    #[inline]
    pub fn base_type(self, base_type: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            base_type: base_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn current_type(self, current_type: impl AsArg < GString > + 'ex) -> Self {
        Self {
            current_type: current_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn dialog_title(self, dialog_title: impl AsArg < GString > + 'ex) -> Self {
        Self {
            dialog_title: dialog_title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn type_blocklist(self, type_blocklist: &'ex Array < StringName >) -> Self {
        Self {
            type_blocklist: CowArg::Borrowed(type_blocklist), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, base_type, current_type, dialog_title, type_blocklist,
        }
        = self;
        re_export::EditorInterface::popup_create_dialog_full(surround_object, callback.cow_as_arg(), base_type, current_type, dialog_title, type_blocklist.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::inspect_object_ex`][super::EditorInterface::inspect_object_ex]."]
#[must_use]
pub struct ExInspectObject < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, object: CowArg < 'ex, Option < Gd < crate::classes::Object > > >, for_property: CowArg < 'ex, GString >, inspector_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInspectObject < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex,) -> Self {
        let for_property = GString::from("");
        let inspector_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), for_property: CowArg::Owned(for_property), inspector_only: inspector_only,
        }
    }
    #[inline]
    pub fn for_property(self, for_property: impl AsArg < GString > + 'ex) -> Self {
        Self {
            for_property: for_property.into_arg(), .. self
        }
    }
    #[inline]
    pub fn inspector_only(self, inspector_only: bool) -> Self {
        Self {
            inspector_only: inspector_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, for_property, inspector_only,
        }
        = self;
        re_export::EditorInterface::inspect_object_full(surround_object, object, for_property, inspector_only,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::edit_script_ex`][super::EditorInterface::edit_script_ex]."]
#[must_use]
pub struct ExEditScript < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, script: CowArg < 'ex, Option < Gd < crate::classes::Script > > >, line: i32, column: i32, grab_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEditScript < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, script: impl AsArg < Option < Gd < crate::classes::Script >> > + 'ex,) -> Self {
        let line = - 1i32;
        let column = 0i32;
        let grab_focus = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, script: script.into_arg(), line: line, column: column, grab_focus: grab_focus,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn grab_focus(self, grab_focus: bool) -> Self {
        Self {
            grab_focus: grab_focus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, script, line, column, grab_focus,
        }
        = self;
        re_export::EditorInterface::edit_script_full(surround_object, script, line, column, grab_focus,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::open_scene_from_path_ex`][super::EditorInterface::open_scene_from_path_ex]."]
#[must_use]
pub struct ExOpenSceneFromPath < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, scene_filepath: CowArg < 'ex, GString >, set_inherited: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExOpenSceneFromPath < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, scene_filepath: impl AsArg < GString > + 'ex,) -> Self {
        let set_inherited = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_filepath: scene_filepath.into_arg(), set_inherited: set_inherited,
        }
    }
    #[inline]
    pub fn set_inherited(self, set_inherited: bool) -> Self {
        Self {
            set_inherited: set_inherited, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_filepath, set_inherited,
        }
        = self;
        re_export::EditorInterface::open_scene_from_path_full(surround_object, scene_filepath, set_inherited,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::save_scene_as_ex`][super::EditorInterface::save_scene_as_ex]."]
#[must_use]
pub struct ExSaveSceneAs < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::EditorInterface, path: CowArg < 'ex, GString >, with_preview: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSaveSceneAs < 'ex > {
    fn new(surround_object: &'ex mut re_export::EditorInterface, path: impl AsArg < GString > + 'ex,) -> Self {
        let with_preview = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), with_preview: with_preview,
        }
    }
    #[inline]
    pub fn with_preview(self, with_preview: bool) -> Self {
        Self {
            with_preview: with_preview, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, path, with_preview,
        }
        = self;
        re_export::EditorInterface::save_scene_as_full(surround_object, path, with_preview,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorInterface;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorInterface {
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