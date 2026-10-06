#![doc = "Sidecar module for class [`DisplayServer`][crate::classes::DisplayServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DisplayServer` enums](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DisplayServer`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`display_server`][crate::classes::display_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `DisplayServer`](https://docs.godotengine.org/en/stable/classes/class_displayserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`DisplayServer` handles everything related to window management. It is separated from [`OS`][crate::classes::Os] as a single operating system may support multiple display servers.\n\n**Headless mode:** Starting the engine with the `--headless` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html) disables all rendering and window management functions. Most functions from `DisplayServer` will return dummy values in this case."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DisplayServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl DisplayServer {
        #[doc = "Returns `true` if the specified `feature` is supported by the current `DisplayServer`, `false` otherwise."]
        pub fn has_feature(&self, feature: crate::classes::display_server::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::display_server::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the `DisplayServer` currently in use. Most operating systems only have a single `DisplayServer`, but Linux has access to more than one `DisplayServer` (currently X11 and Wayland).\n\nThe names of built-in display servers are `Windows`, `macOS`, `X11` (Linux), `Wayland` (Linux), `Android`, `iOS`, `web` (HTML5), and `headless` (when started with the `--headless` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html))."]
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets native help system search callbacks.\n\n`search_callback` has the following arguments: `String search_string, int result_limit` and return a [`Dictionary`][crate::builtin::Dictionary] with \"key, display name\" pairs for the search results. Called when the user enters search terms in the `Help` menu.\n\n`action_callback` has the following arguments: `String key`. Called when the user selects a search result in the `Help` menu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn help_set_search_callbacks(&mut self, search_callback: &Callable, action_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (RefArg::new(search_callback), RefArg::new(action_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "help_set_search_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers callables to emit when the menu is respectively about to show or closed. Callback methods should have zero arguments."]
        pub fn global_menu_set_popup_callbacks(&mut self, menu_root: impl AsArg < GString >, open_callback: &Callable, close_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >,);
            let args = (menu_root.into_arg(), RefArg::new(open_callback), RefArg::new(close_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_popup_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an item that will act as a submenu of the global menu `menu_root`. The `submenu` argument is the ID of the global menu root that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_submenu_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, submenu: CowArg < GString >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, i32,);
            let args = (menu_root, label, submenu, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_submenu_item_ex`][Self::global_menu_add_submenu_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item that will act as a submenu of the global menu `menu_root`. The `submenu` argument is the ID of the global menu root that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_submenu_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, submenu: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_submenu_item_ex(menu_root, label, submenu,) . done()
        }
        #[doc = "Adds an item that will act as a submenu of the global menu `menu_root`. The `submenu` argument is the ID of the global menu root that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_submenu_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddSubmenuItem < 'ex > {
            ExGlobalMenuAddSubmenuItem::new(self, menu_root, label, submenu,)
        }
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_item_ex`][Self::global_menu_add_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_item_ex(menu_root, label,) . done()
        }
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddItem < 'ex > {
            ExGlobalMenuAddItem::new(self, menu_root, label,)
        }
        #[doc = "Adds a new checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_check_item_ex`][Self::global_menu_add_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_check_item_ex(menu_root, label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_check_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddCheckItem < 'ex > {
            ExGlobalMenuAddCheckItem::new(self, menu_root, label,)
        }
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_icon_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_icon_item_ex`][Self::global_menu_add_icon_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_item_ex(menu_root, icon, label,) . done()
        }
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddIconItem < 'ex > {
            ExGlobalMenuAddIconItem::new(self, menu_root, icon, label,)
        }
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_icon_check_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_icon_check_item_ex`][Self::global_menu_add_icon_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_check_item_ex(menu_root, icon, label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_check_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddIconCheckItem < 'ex > {
            ExGlobalMenuAddIconCheckItem::new(self, menu_root, icon, label,)
        }
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_radio_check_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_radio_check_item_ex`][Self::global_menu_add_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_radio_check_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_radio_check_item_ex(menu_root, label,) . done()
        }
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_radio_check_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddRadioCheckItem < 'ex > {
            ExGlobalMenuAddRadioCheckItem::new(self, menu_root, label,)
        }
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_icon_radio_check_item_full(&mut self, menu_root: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >, RefArg < 'a4, Callable >, RefArg < 'a5, Variant >, crate::global::Key, i32,);
            let args = (menu_root, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_icon_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_icon_radio_check_item_ex`][Self::global_menu_add_icon_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_radio_check_item(&mut self, menu_root: impl AsArg < GString >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_icon_radio_check_item_ex(menu_root, icon, label,) . done()
        }
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu with ID `menu_root`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`global_menu_set_item_checked`][`crate::classes::DisplayServer::global_menu_set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_icon_radio_check_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddIconRadioCheckItem < 'ex > {
            ExGlobalMenuAddIconRadioCheckItem::new(self, menu_root, icon, label,)
        }
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_multistate_item_full(&mut self, menu_root: CowArg < GString >, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, i32, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_multistate_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_multistate_item_ex`][Self::global_menu_add_multistate_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_multistate_item(&mut self, menu_root: impl AsArg < GString >, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.global_menu_add_multistate_item_ex(menu_root, label, max_states, default_state,) . done()
        }
        #[doc = "Adds a new item with text `label` to the global menu with ID `menu_root`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_multistate_item_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex, max_states: i32, default_state: i32,) -> ExGlobalMenuAddMultistateItem < 'ex > {
            ExGlobalMenuAddMultistateItem::new(self, menu_root, label, max_states, default_state,)
        }
        #[doc = "Adds a separator between items to the global menu with ID `menu_root`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub(crate) fn global_menu_add_separator_full(&mut self, menu_root: CowArg < GString >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`global_menu_add_separator_ex`][Self::global_menu_add_separator_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a separator between items to the global menu with ID `menu_root`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_separator(&mut self, menu_root: impl AsArg < GString >,) -> i32 {
            self.global_menu_add_separator_ex(menu_root,) . done()
        }
        #[doc = "Adds a separator between items to the global menu with ID `menu_root`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        #[inline]
        pub fn global_menu_add_separator_ex < 'ex > (&'ex mut self, menu_root: impl AsArg < GString > + 'ex,) -> ExGlobalMenuAddSeparator < 'ex > {
            ExGlobalMenuAddSeparator::new(self, menu_root,)
        }
        #[doc = "Returns the index of the item with the specified `text`. Indices are automatically assigned to each item by the engine, and cannot be set manually.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_index_from_text(&self, menu_root: impl AsArg < GString >, text: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the item with the specified `tag`. Indices are automatically assigned to each item by the engine, and cannot be set manually.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_index_from_tag(&self, menu_root: impl AsArg < GString >, tag: &Variant,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (menu_root.into_arg(), RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_index_from_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is checked.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_is_item_checked(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is checkable in some way, i.e. if it has a checkbox or radio button.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_is_item_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` has radio button-style checkability.\n\n**Note:** This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_is_item_radio_checkable(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the callback of the item at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the callback of the item accelerator at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_key_callback(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata of the specified item, which might be of any type. You can set it with [`global_menu_set_item_tag`][`crate::classes::DisplayServer::global_menu_set_item_tag`], which provides a simple way of assigning context data to items.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_tag(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of the item at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_text(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the submenu ID of the item at index `idx`. See [`global_menu_add_submenu_item`][`crate::classes::DisplayServer::global_menu_add_submenu_item`] for more info on how to add a submenu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_submenu(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the accelerator of the item at index `idx`. Accelerators are special combinations of keys that activate the item, no matter which control is focused.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_accelerator(&self, menu_root: impl AsArg < GString >, idx: i32,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is disabled. When it is disabled it can't be selected, or its action invoked.\n\nSee [`global_menu_set_item_disabled`][`crate::classes::DisplayServer::global_menu_set_item_disabled`] for more info on how to disable an item.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_is_item_disabled(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is hidden.\n\nSee [`global_menu_set_item_hidden`][`crate::classes::DisplayServer::global_menu_set_item_hidden`] for more info on how to hide an item.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_is_item_hidden(&self, menu_root: impl AsArg < GString >, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_is_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip associated with the specified index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_tooltip(&self, menu_root: impl AsArg < GString >, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the state of a multistate item. See [`global_menu_add_multistate_item`][`crate::classes::DisplayServer::global_menu_add_multistate_item`] for details.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_state(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of states of a multistate item. See [`global_menu_add_multistate_item`][`crate::classes::DisplayServer::global_menu_add_multistate_item`] for details.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_max_states(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon of the item at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_icon(&self, menu_root: impl AsArg < GString >, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the horizontal offset of the item at the given `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_indentation_level(&self, menu_root: impl AsArg < GString >, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the checkstate status of the item at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_checked(&mut self, menu_root: impl AsArg < GString >, idx: i32, checked: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checked,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the item at index `idx` has a checkbox. If `false`, sets the type of the item to plain text.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of the item at the specified index `idx` to radio button. If `false`, sets the type of the item to plain text.\n\n**Note:** This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_radio_checkable(&mut self, menu_root: impl AsArg < GString >, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, checkable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. Callback is emitted when an item is pressed.\n\n**Note:** The `callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. The callback is emitted when an item is hovered.\n\n**Note:** The `callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_hover_callbacks(&mut self, menu_root: impl AsArg < GString >, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hover_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. Callback is emitted when its accelerator is activated.\n\n**Note:** The `key_callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_key_callback(&mut self, menu_root: impl AsArg < GString >, idx: i32, key_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Callable >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of an item, which may be of any type. You can later get it with [`global_menu_get_item_tag`][`crate::classes::DisplayServer::global_menu_get_item_tag`], which provides a simple way of assigning context data to items.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_tag(&mut self, menu_root: impl AsArg < GString >, idx: i32, tag: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, Variant >,);
            let args = (menu_root.into_arg(), idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the text of the item at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_text(&mut self, menu_root: impl AsArg < GString >, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the submenu of the item at index `idx`. The submenu is the ID of a global menu root that would be shown when the item is clicked.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_submenu(&mut self, menu_root: impl AsArg < GString >, idx: i32, submenu: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the accelerator of the item at index `idx`. `keycode` can be a single \\[enum Key], or a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_accelerator(&mut self, menu_root: impl AsArg < GString >, idx: i32, keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, crate::global::Key,);
            let args = (menu_root.into_arg(), idx, keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables/disables the item at index `idx`. When it is disabled, it can't be selected and its action can't be invoked.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_disabled(&mut self, menu_root: impl AsArg < GString >, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, disabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Hides/shows the item at index `idx`. When it is hidden, an item does not appear in a menu and its action cannot be invoked.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_hidden(&mut self, menu_root: impl AsArg < GString >, idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (menu_root.into_arg(), idx, hidden,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`String`][crate::builtin::GString] tooltip of the item at the specified index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_tooltip(&mut self, menu_root: impl AsArg < GString >, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (menu_root.into_arg(), idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the state of a multistate item. See [`global_menu_add_multistate_item`][`crate::classes::DisplayServer::global_menu_add_multistate_item`] for details.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_state(&mut self, menu_root: impl AsArg < GString >, idx: i32, state: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, state,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets number of state of a multistate item. See [`global_menu_add_multistate_item`][`crate::classes::DisplayServer::global_menu_add_multistate_item`] for details.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_max_states(&mut self, menu_root: impl AsArg < GString >, idx: i32, max_states: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, max_states,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Replaces the [`Texture2D`][crate::classes::Texture2D] icon of the specified `idx`.\n\n**Note:** This method is implemented only on macOS.\n\n**Note:** This method is not supported by macOS \"_dock\" menu items."]
        pub fn global_menu_set_item_icon(&mut self, menu_root: impl AsArg < GString >, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (menu_root.into_arg(), idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the horizontal offset of the item at the given `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_set_item_indentation_level(&mut self, menu_root: impl AsArg < GString >, idx: i32, level: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (menu_root.into_arg(), idx, level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_set_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of items in the global menu with ID `menu_root`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_item_count(&self, menu_root: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item at index `idx` from the global menu `menu_root`.\n\n**Note:** The indices of items after the removed item will be shifted by one.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_remove_item(&mut self, menu_root: impl AsArg < GString >, idx: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (menu_root.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all items from the global menu with ID `menu_root`.\n\n**Note:** This method is implemented only on macOS.\n\n**Supported system menu IDs:**\n\n```text\n\"_main\" - Main menu (macOS).\n\"_dock\" - Dock popup menu (macOS).\n\"_apple\" - Apple menu (macOS, custom items added before \"Services\").\n\"_window\" - Window menu (macOS, custom items added after \"Bring All to Front\").\n\"_help\" - Help menu (macOS).\n```"]
        pub fn global_menu_clear(&mut self, menu_root: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (menu_root.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns Dictionary of supported system menu IDs and names.\n\n**Note:** This method is implemented only on macOS."]
        pub fn global_menu_get_system_menu_roots(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "global_menu_get_system_menu_roots", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the synthesizer is generating speech, or have utterance waiting in the queue.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_is_speaking(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_speaking", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the synthesizer is in a paused state.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_is_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_is_paused", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of voice information dictionaries.\n\nEach [`Dictionary`][crate::builtin::Dictionary] contains two [`String`][crate::builtin::GString] entries:\n\n- `name` is voice name.\n\n- `id` is voice identifier.\n\n- `language` is language code in `lang_Variant` format. The `lang` part is a 2 or 3-letter code based on the ISO-639 standard, in lowercase. The `Variant` part is an engine-dependent string describing country, region or/and dialect.\n\nNote that Godot depends on system libraries for text-to-speech functionality. These libraries are installed by default on Windows and macOS, but not on all Linux distributions. If they are not present, this method will return an empty list. This applies to both Godot users on Linux, as well as end-users on Linux running Godot games that use text-to-speech.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_get_voices(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedStringArray`][crate::builtin::PackedStringArray] of voice identifiers for the `language`.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_get_voices_for_language(&self, language: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_get_voices_for_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an utterance to the queue. If `interrupt` is `true`, the queue is cleared first.\n\n- `voice` identifier is one of the `\"id\"` values returned by [`tts_get_voices`][`crate::classes::DisplayServer::tts_get_voices`] or one of the values returned by [`tts_get_voices_for_language`][`crate::classes::DisplayServer::tts_get_voices_for_language`].\n\n- `volume` ranges from `0` (lowest) to `100` (highest).\n\n- `pitch` ranges from `0.0` (lowest) to `2.0` (highest), `1.0` is default pitch for the current voice.\n\n- `rate` ranges from `0.1` (lowest) to `10.0` (highest), `1.0` is a normal speaking rate. Other values act as a percentage relative.\n\n- `utterance_id` is passed as a parameter to the callback functions.\n\n**Note:** On Windows and Linux (X11/Wayland), utterance `text` can use SSML markup. SSML support is engine and voice dependent. If the engine does not support SSML, you should strip out all XML markup before calling [`tts_speak`][`crate::classes::DisplayServer::tts_speak`].\n\n**Note:** The granularity of pitch, rate, and volume is engine and voice dependent. Values may be truncated.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub(crate) fn tts_speak_full(&mut self, text: CowArg < GString >, voice: CowArg < GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i64, interrupt: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32, f32, f32, i64, bool,);
            let args = (text, voice, volume, pitch, rate, utterance_id, interrupt,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_speak", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`tts_speak_ex`][Self::tts_speak_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an utterance to the queue. If `interrupt` is `true`, the queue is cleared first.\n\n- `voice` identifier is one of the `\"id\"` values returned by [`tts_get_voices`][`crate::classes::DisplayServer::tts_get_voices`] or one of the values returned by [`tts_get_voices_for_language`][`crate::classes::DisplayServer::tts_get_voices_for_language`].\n\n- `volume` ranges from `0` (lowest) to `100` (highest).\n\n- `pitch` ranges from `0.0` (lowest) to `2.0` (highest), `1.0` is default pitch for the current voice.\n\n- `rate` ranges from `0.1` (lowest) to `10.0` (highest), `1.0` is a normal speaking rate. Other values act as a percentage relative.\n\n- `utterance_id` is passed as a parameter to the callback functions.\n\n**Note:** On Windows and Linux (X11/Wayland), utterance `text` can use SSML markup. SSML support is engine and voice dependent. If the engine does not support SSML, you should strip out all XML markup before calling [`tts_speak`][`crate::classes::DisplayServer::tts_speak`].\n\n**Note:** The granularity of pitch, rate, and volume is engine and voice dependent. Values may be truncated.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn tts_speak(&mut self, text: impl AsArg < GString >, voice: impl AsArg < GString >,) {
            self.tts_speak_ex(text, voice,) . done()
        }
        #[doc = "Adds an utterance to the queue. If `interrupt` is `true`, the queue is cleared first.\n\n- `voice` identifier is one of the `\"id\"` values returned by [`tts_get_voices`][`crate::classes::DisplayServer::tts_get_voices`] or one of the values returned by [`tts_get_voices_for_language`][`crate::classes::DisplayServer::tts_get_voices_for_language`].\n\n- `volume` ranges from `0` (lowest) to `100` (highest).\n\n- `pitch` ranges from `0.0` (lowest) to `2.0` (highest), `1.0` is default pitch for the current voice.\n\n- `rate` ranges from `0.1` (lowest) to `10.0` (highest), `1.0` is a normal speaking rate. Other values act as a percentage relative.\n\n- `utterance_id` is passed as a parameter to the callback functions.\n\n**Note:** On Windows and Linux (X11/Wayland), utterance `text` can use SSML markup. SSML support is engine and voice dependent. If the engine does not support SSML, you should strip out all XML markup before calling [`tts_speak`][`crate::classes::DisplayServer::tts_speak`].\n\n**Note:** The granularity of pitch, rate, and volume is engine and voice dependent. Values may be truncated.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn tts_speak_ex < 'ex > (&'ex mut self, text: impl AsArg < GString > + 'ex, voice: impl AsArg < GString > + 'ex,) -> ExTtsSpeak < 'ex > {
            ExTtsSpeak::new(self, text, voice,)
        }
        #[doc = "Puts the synthesizer into a paused state.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_pause", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resumes the synthesizer if it was paused.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_resume(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_resume", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops synthesis in progress and removes all utterances from the queue.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_stop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a callback, which is called when the utterance has started, finished, canceled or reached a text boundary.\n\n- [`TtsUtteranceEvent::STARTED`][`crate::classes::display_server::TtsUtteranceEvent::STARTED`], [`TtsUtteranceEvent::ENDED`][`crate::classes::display_server::TtsUtteranceEvent::ENDED`], and [`TtsUtteranceEvent::CANCELED`][`crate::classes::display_server::TtsUtteranceEvent::CANCELED`] callable's method should take one `int` parameter, the utterance ID.\n\n- [`TtsUtteranceEvent::BOUNDARY`][`crate::classes::display_server::TtsUtteranceEvent::BOUNDARY`] callable's method should take two `int` parameters, the index of the character and the utterance ID.\n\n**Note:** The granularity of the boundary callbacks is engine dependent.\n\n**Note:** This method is implemented on Android, iOS, Web, Linux (X11/Wayland), macOS, and Windows."]
        pub fn tts_set_utterance_callback(&mut self, event: crate::classes::display_server::TtsUtteranceEvent, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::display_server::TtsUtteranceEvent, RefArg < 'a0, Callable >,);
            let args = (event, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tts_set_utterance_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OS supports dark mode.\n\n**Note:** This method is implemented on Android, iOS, macOS, Windows, and Linux (X11/Wayland)."]
        pub fn is_dark_mode_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode_supported", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if OS is using dark mode.\n\n**Note:** This method is implemented on Android, iOS, macOS, Windows, and Linux (X11/Wayland)."]
        pub fn is_dark_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_dark_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns OS theme accent color. Returns `Color(0, 0, 0, 0)`, if accent color is unknown.\n\n**Note:** This method is implemented on macOS, Windows, Android, and Linux (X11/Wayland)."]
        pub fn get_accent_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_accent_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the OS theme base color (default control background). Returns `Color(0, 0, 0, 0)` if the base color is unknown.\n\n**Note:** This method is implemented on macOS, Windows, and Android."]
        pub fn get_base_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_base_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback that should be called when the system's theme settings are changed. `callable` should accept zero arguments.\n\n**Note:** This method is implemented on Android, iOS, macOS, Windows, and Linux (X11/Wayland)."]
        pub fn set_system_theme_change_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_system_theme_change_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the current mouse mode. See also [`mouse_get_mode`][`crate::classes::DisplayServer::mouse_get_mode`]."]
        pub fn mouse_set_mode(&mut self, mouse_mode: crate::classes::display_server::MouseMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::MouseMode,);
            let args = (mouse_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current mouse mode. See also [`mouse_set_mode`][`crate::classes::DisplayServer::mouse_set_mode`]."]
        pub fn mouse_get_mode(&self,) -> crate::classes::display_server::MouseMode {
            type CallRet = crate::classes::display_server::MouseMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mouse cursor position to the given `position` relative to an origin at the upper left corner of the currently focused game Window Manager window.\n\n**Note:** [`warp_mouse`][`crate::classes::DisplayServer::warp_mouse`] is only supported on Windows, macOS, and Linux (X11/Wayland). It has no effect on Android, iOS, and Web."]
        pub fn warp_mouse(&mut self, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "warp_mouse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the mouse cursor's current position in screen coordinates."]
        pub fn mouse_get_position(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current state of mouse buttons (whether each button is pressed) as a bitmask. If multiple mouse buttons are pressed at the same time, the bits are added together. Equivalent to [`get_mouse_button_mask`][`crate::classes::Input::get_mouse_button_mask`]."]
        pub fn mouse_get_button_state(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "mouse_get_button_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the user's clipboard content to the given string."]
        pub fn clipboard_set(&mut self, clipboard: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (clipboard.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the user's clipboard as a string if possible."]
        pub fn clipboard_get(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the user's clipboard as an image if possible.\n\n**Note:** This method uses the copied pixel data, e.g. from an image editing software or a web browser, not an image file copied from file explorer."]
        pub fn clipboard_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is a text content on the user's clipboard."]
        pub fn clipboard_has(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if there is an image content on the user's clipboard."]
        pub fn clipboard_has_image(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_has_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the user's [primary](https://unix.stackexchange.com/questions/139191/whats-the-difference-between-primary-selection-and-clipboard-buffer) clipboard content to the given string. This is the clipboard that is set when the user selects text in any application, rather than when pressing `Ctrl + C`. The clipboard data can then be pasted by clicking the middle mouse button in any application that supports the primary clipboard mechanism.\n\n**Note:** This method is only implemented on Linux (X11/Wayland)."]
        pub fn clipboard_set_primary(&mut self, clipboard_primary: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (clipboard_primary.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_set_primary", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the user's [primary](https://unix.stackexchange.com/questions/139191/whats-the-difference-between-primary-selection-and-clipboard-buffer) clipboard as a string if possible. This is the clipboard that is set when the user selects text in any application, rather than when pressing `Ctrl + C`. The clipboard data can then be pasted by clicking the middle mouse button in any application that supports the primary clipboard mechanism.\n\n**Note:** This method is only implemented on Linux (X11/Wayland)."]
        pub fn clipboard_get_primary(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "clipboard_get_primary", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Array`][crate::builtin::Array] of [`Rect2`][crate::builtin::Rect2], each of which is the bounding rectangle for a display cutout or notch. These are non-functional areas on edge-to-edge screens used by cameras and sensors. Returns an empty array if the device does not have cutouts. See also [`get_display_safe_area`][`crate::classes::DisplayServer::get_display_safe_area`].\n\n**Note:** Currently only implemented on Android. Other platforms will return an empty array even if they do have display cutouts or notches."]
        pub fn get_display_cutouts(&self,) -> Array < Rect2 > {
            type CallRet = Array < Rect2 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_cutouts", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the unobscured area of the display where interactive controls should be rendered. See also [`get_display_cutouts`][`crate::classes::DisplayServer::get_display_cutouts`].\n\n**Note:** Currently only implemented on Android and iOS. On other platforms, `screen_get_usable_rect(SCREEN_OF_MAIN_WINDOW)` will be returned as a fallback. See also [`screen_get_usable_rect`][`crate::classes::DisplayServer::screen_get_usable_rect`]."]
        pub fn get_display_safe_area(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_display_safe_area", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of displays available.\n\n**Note:** This method is implemented on Linux (X11 and Wayland), macOS, and Windows. On other platforms, this method always returns `1`."]
        pub fn get_screen_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the primary screen.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `0`."]
        pub fn get_primary_screen(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_primary_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the screen containing the window with the keyboard focus, or the primary screen if there's no focused window.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns the primary screen."]
        pub fn get_keyboard_focus_screen(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_keyboard_focus_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the screen that overlaps the most with the given rectangle. Returns `INVALID_SCREEN` if the rectangle doesn't overlap with any screen or has no area."]
        pub fn get_screen_from_rect(&self, rect: Rect2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rect2,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_screen_from_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the screen's top-left corner position in pixels. Returns `Vector2i.ZERO` if `screen` is invalid. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin might be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        pub(crate) fn screen_get_position_full(&self, screen: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_position_ex`][Self::screen_get_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the screen's top-left corner position in pixels. Returns `Vector2i.ZERO` if `screen` is invalid. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin might be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        #[inline]
        pub fn screen_get_position(&self,) -> Vector2i {
            self.screen_get_position_ex() . done()
        }
        #[doc = "Returns the screen's top-left corner position in pixels. Returns `Vector2i.ZERO` if `screen` is invalid. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin might be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        #[inline]
        pub fn screen_get_position_ex < 'ex > (&'ex self,) -> ExScreenGetPosition < 'ex > {
            ExScreenGetPosition::new(self,)
        }
        #[doc = "Returns the screen's size in pixels. See also [`screen_get_position`][`crate::classes::DisplayServer::screen_get_position`] and [`screen_get_usable_rect`][`crate::classes::DisplayServer::screen_get_usable_rect`]. Returns `Vector2i.ZERO` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        pub(crate) fn screen_get_size_full(&self, screen: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_size_ex`][Self::screen_get_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the screen's size in pixels. See also [`screen_get_position`][`crate::classes::DisplayServer::screen_get_position`] and [`screen_get_usable_rect`][`crate::classes::DisplayServer::screen_get_usable_rect`]. Returns `Vector2i.ZERO` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        #[inline]
        pub fn screen_get_size(&self,) -> Vector2i {
            self.screen_get_size_ex() . done()
        }
        #[doc = "Returns the screen's size in pixels. See also [`screen_get_position`][`crate::classes::DisplayServer::screen_get_position`] and [`screen_get_usable_rect`][`crate::classes::DisplayServer::screen_get_usable_rect`]. Returns `Vector2i.ZERO` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`."]
        #[inline]
        pub fn screen_get_size_ex < 'ex > (&'ex self,) -> ExScreenGetSize < 'ex > {
            ExScreenGetSize::new(self,)
        }
        #[doc = "Returns the portion of the screen that is not obstructed by a status bar in pixels. See also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `Rect2i(screen_get_position(screen), screen_get_size(screen))`."]
        pub(crate) fn screen_get_usable_rect_full(&self, screen: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_usable_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_usable_rect_ex`][Self::screen_get_usable_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the portion of the screen that is not obstructed by a status bar in pixels. See also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `Rect2i(screen_get_position(screen), screen_get_size(screen))`."]
        #[inline]
        pub fn screen_get_usable_rect(&self,) -> Rect2i {
            self.screen_get_usable_rect_ex() . done()
        }
        #[doc = "Returns the portion of the screen that is not obstructed by a status bar in pixels. See also [`screen_get_size`][`crate::classes::DisplayServer::screen_get_size`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `Rect2i(screen_get_position(screen), screen_get_size(screen))`."]
        #[inline]
        pub fn screen_get_usable_rect_ex < 'ex > (&'ex self,) -> ExScreenGetUsableRect < 'ex > {
            ExScreenGetUsableRect::new(self,)
        }
        #[doc = "Returns the dots per inch density of the specified screen. Returns platform specific default value if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, returned value is inaccurate if fractional display scaling mode is used.\n\n**Note:** On Android devices, the actual screen densities are grouped into six generalized densities:\n\n```text\n   ldpi - 120 dpi\n   mdpi - 160 dpi\n   hdpi - 240 dpi\n  xhdpi - 320 dpi\n xxhdpi - 480 dpi\nxxxhdpi - 640 dpi\n```\n\n**Note:** This method is implemented on Android, iOS, Linux (X11/Wayland), macOS, Web, and Windows. On other platforms, this method always returns `72`."]
        pub(crate) fn screen_get_dpi_full(&self, screen: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_dpi", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_dpi_ex`][Self::screen_get_dpi_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the dots per inch density of the specified screen. Returns platform specific default value if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, returned value is inaccurate if fractional display scaling mode is used.\n\n**Note:** On Android devices, the actual screen densities are grouped into six generalized densities:\n\n```text\n   ldpi - 120 dpi\n   mdpi - 160 dpi\n   hdpi - 240 dpi\n  xhdpi - 320 dpi\n xxhdpi - 480 dpi\nxxxhdpi - 640 dpi\n```\n\n**Note:** This method is implemented on Android, iOS, Linux (X11/Wayland), macOS, Web, and Windows. On other platforms, this method always returns `72`."]
        #[inline]
        pub fn screen_get_dpi(&self,) -> i32 {
            self.screen_get_dpi_ex() . done()
        }
        #[doc = "Returns the dots per inch density of the specified screen. Returns platform specific default value if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, returned value is inaccurate if fractional display scaling mode is used.\n\n**Note:** On Android devices, the actual screen densities are grouped into six generalized densities:\n\n```text\n   ldpi - 120 dpi\n   mdpi - 160 dpi\n   hdpi - 240 dpi\n  xhdpi - 320 dpi\n xxhdpi - 480 dpi\nxxxhdpi - 640 dpi\n```\n\n**Note:** This method is implemented on Android, iOS, Linux (X11/Wayland), macOS, Web, and Windows. On other platforms, this method always returns `72`."]
        #[inline]
        pub fn screen_get_dpi_ex < 'ex > (&'ex self,) -> ExScreenGetDpi < 'ex > {
            ExScreenGetDpi::new(self,)
        }
        #[doc = "Returns the scale factor of the specified screen by index. Returns `1.0` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, the returned value is `2.0` for hiDPI (Retina) screens, and `1.0` for all other cases.\n\n**Note:** On Linux (Wayland), the returned value is accurate only when `screen` is `SCREEN_OF_MAIN_WINDOW`. Due to API limitations, passing a direct index will return a rounded-up integer, if the screen has a fractional scale (e.g. `1.25` would get rounded up to `2.0`).\n\n**Note:** This method is implemented on Android, iOS, Web, macOS, and Linux (Wayland). On other platforms, this method always returns `1.0`."]
        pub(crate) fn screen_get_scale_full(&self, screen: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_scale_ex`][Self::screen_get_scale_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the scale factor of the specified screen by index. Returns `1.0` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, the returned value is `2.0` for hiDPI (Retina) screens, and `1.0` for all other cases.\n\n**Note:** On Linux (Wayland), the returned value is accurate only when `screen` is `SCREEN_OF_MAIN_WINDOW`. Due to API limitations, passing a direct index will return a rounded-up integer, if the screen has a fractional scale (e.g. `1.25` would get rounded up to `2.0`).\n\n**Note:** This method is implemented on Android, iOS, Web, macOS, and Linux (Wayland). On other platforms, this method always returns `1.0`."]
        #[inline]
        pub fn screen_get_scale(&self,) -> f32 {
            self.screen_get_scale_ex() . done()
        }
        #[doc = "Returns the scale factor of the specified screen by index. Returns `1.0` if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** On macOS, the returned value is `2.0` for hiDPI (Retina) screens, and `1.0` for all other cases.\n\n**Note:** On Linux (Wayland), the returned value is accurate only when `screen` is `SCREEN_OF_MAIN_WINDOW`. Due to API limitations, passing a direct index will return a rounded-up integer, if the screen has a fractional scale (e.g. `1.25` would get rounded up to `2.0`).\n\n**Note:** This method is implemented on Android, iOS, Web, macOS, and Linux (Wayland). On other platforms, this method always returns `1.0`."]
        #[inline]
        pub fn screen_get_scale_ex < 'ex > (&'ex self,) -> ExScreenGetScale < 'ex > {
            ExScreenGetScale::new(self,)
        }
        #[doc = "Returns `true` if touch events are available (Android or iOS), the capability is detected on the Web platform or if \\[member ProjectSettings.input_devices/pointing/emulate_touch_from_mouse] is `true`."]
        pub fn is_touchscreen_available(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_touchscreen_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the greatest scale factor of all screens.\n\n**Note:** On macOS returned value is `2.0` if there is at least one hiDPI (Retina) screen in the system, and `1.0` in all other cases.\n\n**Note:** This method is implemented only on macOS."]
        pub fn screen_get_max_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_max_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current refresh rate of the specified screen. When V-Sync is enabled, this returns the maximum framerate the project can effectively reach. Returns `-1.0` if `screen` is invalid or the `DisplayServer` fails to find the refresh rate for the specified screen.\n\nTo fallback to a default refresh rate if the method fails, try:\n\n```gdscript\nvar refresh_rate = DisplayServer.screen_get_refresh_rate()\nif refresh_rate < 0:\n\trefresh_rate = 60.0\n```\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android, iOS, macOS, Linux (X11 and Wayland), and Windows. On other platforms, this method always returns `-1.0`."]
        pub(crate) fn screen_get_refresh_rate_full(&self, screen: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_refresh_rate_ex`][Self::screen_get_refresh_rate_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current refresh rate of the specified screen. When V-Sync is enabled, this returns the maximum framerate the project can effectively reach. Returns `-1.0` if `screen` is invalid or the `DisplayServer` fails to find the refresh rate for the specified screen.\n\nTo fallback to a default refresh rate if the method fails, try:\n\n```gdscript\nvar refresh_rate = DisplayServer.screen_get_refresh_rate()\nif refresh_rate < 0:\n\trefresh_rate = 60.0\n```\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android, iOS, macOS, Linux (X11 and Wayland), and Windows. On other platforms, this method always returns `-1.0`."]
        #[inline]
        pub fn screen_get_refresh_rate(&self,) -> f32 {
            self.screen_get_refresh_rate_ex() . done()
        }
        #[doc = "Returns the current refresh rate of the specified screen. When V-Sync is enabled, this returns the maximum framerate the project can effectively reach. Returns `-1.0` if `screen` is invalid or the `DisplayServer` fails to find the refresh rate for the specified screen.\n\nTo fallback to a default refresh rate if the method fails, try:\n\n```gdscript\nvar refresh_rate = DisplayServer.screen_get_refresh_rate()\nif refresh_rate < 0:\n\trefresh_rate = 60.0\n```\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android, iOS, macOS, Linux (X11 and Wayland), and Windows. On other platforms, this method always returns `-1.0`."]
        #[inline]
        pub fn screen_get_refresh_rate_ex < 'ex > (&'ex self,) -> ExScreenGetRefreshRate < 'ex > {
            ExScreenGetRefreshRate::new(self,)
        }
        #[doc = "Returns the color of the pixel at the given screen `position`. On multi-monitor setups, the screen position is relative to the virtual desktop area.\n\n**Note:** This method is implemented on Linux (X11, excluding XWayland), macOS, and Windows. On other platforms, this method always returns `Color(0, 0, 0, 1)`.\n\n**Note:** On macOS, this method requires the \"Screen Recording\" permission. If permission is not granted, this method returns a color from a screenshot that will not include other application windows or OS elements not related to the application."]
        pub fn screen_get_pixel(&self, position: Vector2i,) -> Color {
            type CallRet = Color;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_pixel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a screenshot of the `screen`. Returns `null` if `screen` is invalid or the `DisplayServer` fails to capture screenshot.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux (X11, excluding XWayland), macOS, and Windows. On other platforms, this method always returns `null`.\n\n**Note:** On macOS, this method requires the \"Screen Recording\" permission. If permission is not granted, this method returns a screenshot that will not include other application windows or OS elements not related to the application."]
        pub(crate) fn screen_get_image_full(&self, screen: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_image_ex`][Self::screen_get_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a screenshot of the `screen`. Returns `null` if `screen` is invalid or the `DisplayServer` fails to capture screenshot.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux (X11, excluding XWayland), macOS, and Windows. On other platforms, this method always returns `null`.\n\n**Note:** On macOS, this method requires the \"Screen Recording\" permission. If permission is not granted, this method returns a screenshot that will not include other application windows or OS elements not related to the application."]
        #[inline]
        pub fn screen_get_image(&self,) -> Option < Gd < crate::classes::Image > > {
            self.screen_get_image_ex() . done()
        }
        #[doc = "Returns a screenshot of the `screen`. Returns `null` if `screen` is invalid or the `DisplayServer` fails to capture screenshot.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux (X11, excluding XWayland), macOS, and Windows. On other platforms, this method always returns `null`.\n\n**Note:** On macOS, this method requires the \"Screen Recording\" permission. If permission is not granted, this method returns a screenshot that will not include other application windows or OS elements not related to the application."]
        #[inline]
        pub fn screen_get_image_ex < 'ex > (&'ex self,) -> ExScreenGetImage < 'ex > {
            ExScreenGetImage::new(self,)
        }
        #[doc = "Returns a screenshot of the screen region defined by `rect`. Returns `null` if `rect` is outside screen bounds or the `DisplayServer` fails to capture screenshot.\n\n**Note:** This method is implemented on macOS and Windows. On other platforms, this method always returns `null`.\n\n**Note:** On macOS, this method requires the \"Screen Recording\" permission. If permission is not granted, this method returns a screenshot that will not include other application windows or OS elements not related to the application."]
        pub fn screen_get_image_rect(&self, rect: Rect2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rect2i,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_image_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `screen`'s `orientation`. See also [`screen_get_orientation`][`crate::classes::DisplayServer::screen_get_orientation`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS.\n\n**Note:** On iOS, this method has no effect if \\[member ProjectSettings.display/window/handheld/orientation] is not set to [`ScreenOrientation::SENSOR`][`crate::classes::display_server::ScreenOrientation::SENSOR`]."]
        pub(crate) fn screen_set_orientation_full(&mut self, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::ScreenOrientation, i32,);
            let args = (orientation, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_set_orientation_ex`][Self::screen_set_orientation_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `screen`'s `orientation`. See also [`screen_get_orientation`][`crate::classes::DisplayServer::screen_get_orientation`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS.\n\n**Note:** On iOS, this method has no effect if \\[member ProjectSettings.display/window/handheld/orientation] is not set to [`ScreenOrientation::SENSOR`][`crate::classes::display_server::ScreenOrientation::SENSOR`]."]
        #[inline]
        pub fn screen_set_orientation(&mut self, orientation: crate::classes::display_server::ScreenOrientation,) {
            self.screen_set_orientation_ex(orientation,) . done()
        }
        #[doc = "Sets the `screen`'s `orientation`. See also [`screen_get_orientation`][`crate::classes::DisplayServer::screen_get_orientation`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS.\n\n**Note:** On iOS, this method has no effect if \\[member ProjectSettings.display/window/handheld/orientation] is not set to [`ScreenOrientation::SENSOR`][`crate::classes::display_server::ScreenOrientation::SENSOR`]."]
        #[inline]
        pub fn screen_set_orientation_ex < 'ex > (&'ex mut self, orientation: crate::classes::display_server::ScreenOrientation,) -> ExScreenSetOrientation < 'ex > {
            ExScreenSetOrientation::new(self, orientation,)
        }
        #[doc = "Returns the `screen`'s current orientation. See also [`screen_set_orientation`][`crate::classes::DisplayServer::screen_set_orientation`]. Returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`] if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS. On other platforms, this method always returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`]."]
        pub(crate) fn screen_get_orientation_full(&self, screen: i32,) -> crate::classes::display_server::ScreenOrientation {
            type CallRet = crate::classes::display_server::ScreenOrientation;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`screen_get_orientation_ex`][Self::screen_get_orientation_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the `screen`'s current orientation. See also [`screen_set_orientation`][`crate::classes::DisplayServer::screen_set_orientation`]. Returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`] if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS. On other platforms, this method always returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`]."]
        #[inline]
        pub fn screen_get_orientation(&self,) -> crate::classes::display_server::ScreenOrientation {
            self.screen_get_orientation_ex() . done()
        }
        #[doc = "Returns the `screen`'s current orientation. See also [`screen_set_orientation`][`crate::classes::DisplayServer::screen_set_orientation`]. Returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`] if `screen` is invalid.\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Android and iOS. On other platforms, this method always returns [`ScreenOrientation::LANDSCAPE`][`crate::classes::display_server::ScreenOrientation::LANDSCAPE`]."]
        #[inline]
        pub fn screen_get_orientation_ex < 'ex > (&'ex self,) -> ExScreenGetOrientation < 'ex > {
            ExScreenGetOrientation::new(self,)
        }
        #[doc = "Sets whether the screen should never be turned off by the operating system's power-saving measures. See also [`screen_is_kept_on`][`crate::classes::DisplayServer::screen_is_kept_on`]."]
        pub fn screen_set_keep_on(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_set_keep_on", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the screen should never be turned off by the operating system's power-saving measures. See also [`screen_set_keep_on`][`crate::classes::DisplayServer::screen_set_keep_on`]."]
        pub fn screen_is_kept_on(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "screen_is_kept_on", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of Godot window IDs belonging to this process.\n\n**Note:** Native dialogs are not included in this list."]
        pub fn get_window_list(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the window at the specified screen `position` (in pixels). On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin may be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```"]
        pub fn get_window_at_screen_position(&self, position: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_window_at_screen_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns internal structure pointers for use in plugins.\n\n**Note:** This method is implemented on Android, Linux (X11/Wayland), macOS, and Windows."]
        pub(crate) fn window_get_native_handle_full(&self, handle_type: crate::classes::display_server::HandleType, window_id: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (crate::classes::display_server::HandleType, i32,);
            let args = (handle_type, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_native_handle_ex`][Self::window_get_native_handle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns internal structure pointers for use in plugins.\n\n**Note:** This method is implemented on Android, Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_get_native_handle(&self, handle_type: crate::classes::display_server::HandleType,) -> i64 {
            self.window_get_native_handle_ex(handle_type,) . done()
        }
        #[doc = "Returns internal structure pointers for use in plugins.\n\n**Note:** This method is implemented on Android, Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_get_native_handle_ex < 'ex > (&'ex self, handle_type: crate::classes::display_server::HandleType,) -> ExWindowGetNativeHandle < 'ex > {
            ExWindowGetNativeHandle::new(self, handle_type,)
        }
        #[doc = "Returns ID of the active popup window, or `INVALID_WINDOW_ID` if there is none."]
        pub fn window_get_active_popup(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_active_popup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the bounding box of control, or menu item that was used to open the popup window, in the screen coordinate system. Clicking this area will not auto-close this popup."]
        pub fn window_set_popup_safe_rect(&mut self, window: i32, rect: Rect2i,) {
            type CallRet = ();
            type CallParams = (i32, Rect2i,);
            let args = (window, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_popup_safe_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the bounding box of control, or menu item that was used to open the popup window, in the screen coordinate system."]
        pub fn window_get_popup_safe_rect(&self, window: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32,);
            let args = (window,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_popup_safe_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the title of the given window to `title`.\n\n**Note:** It's recommended to change this value using \\[member Window.title] instead.\n\n**Note:** Avoid changing the window title every frame, as this can cause performance issues on certain window managers. Try to change the window title only a few times per second at most."]
        pub(crate) fn window_set_title_full(&mut self, title: CowArg < GString >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_title_ex`][Self::window_set_title_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the title of the given window to `title`.\n\n**Note:** It's recommended to change this value using \\[member Window.title] instead.\n\n**Note:** Avoid changing the window title every frame, as this can cause performance issues on certain window managers. Try to change the window title only a few times per second at most."]
        #[inline]
        pub fn window_set_title(&mut self, title: impl AsArg < GString >,) {
            self.window_set_title_ex(title,) . done()
        }
        #[doc = "Sets the title of the given window to `title`.\n\n**Note:** It's recommended to change this value using \\[member Window.title] instead.\n\n**Note:** Avoid changing the window title every frame, as this can cause performance issues on certain window managers. Try to change the window title only a few times per second at most."]
        #[inline]
        pub fn window_set_title_ex < 'ex > (&'ex mut self, title: impl AsArg < GString > + 'ex,) -> ExWindowSetTitle < 'ex > {
            ExWindowSetTitle::new(self, title,)
        }
        #[doc = "Returns the estimated window title bar size (including text and window buttons) for the window specified by `window_id` (in pixels). This method does not change the window title.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub(crate) fn window_get_title_size_full(&self, title: CowArg < GString >, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (title, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_title_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_title_size_ex`][Self::window_get_title_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the estimated window title bar size (including text and window buttons) for the window specified by `window_id` (in pixels). This method does not change the window title.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn window_get_title_size(&self, title: impl AsArg < GString >,) -> Vector2i {
            self.window_get_title_size_ex(title,) . done()
        }
        #[doc = "Returns the estimated window title bar size (including text and window buttons) for the window specified by `window_id` (in pixels). This method does not change the window title.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn window_get_title_size_ex < 'ex > (&'ex self, title: impl AsArg < GString > + 'ex,) -> ExWindowGetTitleSize < 'ex > {
            ExWindowGetTitleSize::new(self, title,)
        }
        #[doc = "Sets a polygonal region of the window which accepts mouse events. Mouse events outside the region will be passed through.\n\nPassing an empty array will disable passthrough support (all mouse events will be intercepted by the window, which is the default behavior).\n\n\n```gdscript\n# Set region, using Path2D node.\nDisplayServer.window_set_mouse_passthrough($Path2D.curve.get_baked_points())\n\n# Set region, using Polygon2D node.\nDisplayServer.window_set_mouse_passthrough($Polygon2D.polygon)\n\n# Reset region to default.\nDisplayServer.window_set_mouse_passthrough([])\n```\n\n\n**Note:** On Windows, the portion of a window that lies outside the region is not drawn, while on Linux (X11) and macOS it is.\n\n**Note:** This method is implemented on Linux (X11), macOS and Windows."]
        pub(crate) fn window_set_mouse_passthrough_full(&mut self, region: RefArg < PackedVector2Array >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, i32,);
            let args = (region, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mouse_passthrough", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_mouse_passthrough_ex`][Self::window_set_mouse_passthrough_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a polygonal region of the window which accepts mouse events. Mouse events outside the region will be passed through.\n\nPassing an empty array will disable passthrough support (all mouse events will be intercepted by the window, which is the default behavior).\n\n\n```gdscript\n# Set region, using Path2D node.\nDisplayServer.window_set_mouse_passthrough($Path2D.curve.get_baked_points())\n\n# Set region, using Polygon2D node.\nDisplayServer.window_set_mouse_passthrough($Polygon2D.polygon)\n\n# Reset region to default.\nDisplayServer.window_set_mouse_passthrough([])\n```\n\n\n**Note:** On Windows, the portion of a window that lies outside the region is not drawn, while on Linux (X11) and macOS it is.\n\n**Note:** This method is implemented on Linux (X11), macOS and Windows."]
        #[inline]
        pub fn window_set_mouse_passthrough(&mut self, region: &PackedVector2Array,) {
            self.window_set_mouse_passthrough_ex(region,) . done()
        }
        #[doc = "Sets a polygonal region of the window which accepts mouse events. Mouse events outside the region will be passed through.\n\nPassing an empty array will disable passthrough support (all mouse events will be intercepted by the window, which is the default behavior).\n\n\n```gdscript\n# Set region, using Path2D node.\nDisplayServer.window_set_mouse_passthrough($Path2D.curve.get_baked_points())\n\n# Set region, using Polygon2D node.\nDisplayServer.window_set_mouse_passthrough($Polygon2D.polygon)\n\n# Reset region to default.\nDisplayServer.window_set_mouse_passthrough([])\n```\n\n\n**Note:** On Windows, the portion of a window that lies outside the region is not drawn, while on Linux (X11) and macOS it is.\n\n**Note:** This method is implemented on Linux (X11), macOS and Windows."]
        #[inline]
        pub fn window_set_mouse_passthrough_ex < 'ex > (&'ex mut self, region: &'ex PackedVector2Array,) -> ExWindowSetMousePassthrough < 'ex > {
            ExWindowSetMousePassthrough::new(self, region,)
        }
        #[doc = "Returns the screen the window specified by `window_id` is currently positioned on. If the screen overlaps multiple displays, the screen where the window's center is located is returned. See also [`window_set_current_screen`][`crate::classes::DisplayServer::window_set_current_screen`]. Returns `INVALID_SCREEN` if `window_id` is invalid.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `0`."]
        pub(crate) fn window_get_current_screen_full(&self, window_id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_current_screen_ex`][Self::window_get_current_screen_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the screen the window specified by `window_id` is currently positioned on. If the screen overlaps multiple displays, the screen where the window's center is located is returned. See also [`window_set_current_screen`][`crate::classes::DisplayServer::window_set_current_screen`]. Returns `INVALID_SCREEN` if `window_id` is invalid.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `0`."]
        #[inline]
        pub fn window_get_current_screen(&self,) -> i32 {
            self.window_get_current_screen_ex() . done()
        }
        #[doc = "Returns the screen the window specified by `window_id` is currently positioned on. If the screen overlaps multiple displays, the screen where the window's center is located is returned. See also [`window_set_current_screen`][`crate::classes::DisplayServer::window_set_current_screen`]. Returns `INVALID_SCREEN` if `window_id` is invalid.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows. On other platforms, this method always returns `0`."]
        #[inline]
        pub fn window_get_current_screen_ex < 'ex > (&'ex self,) -> ExWindowGetCurrentScreen < 'ex > {
            ExWindowGetCurrentScreen::new(self,)
        }
        #[doc = "Moves the window specified by `window_id` to the specified `screen`. See also [`window_get_current_screen`][`crate::classes::DisplayServer::window_get_current_screen`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows."]
        pub(crate) fn window_set_current_screen_full(&mut self, screen: i32, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (screen, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_current_screen_ex`][Self::window_set_current_screen_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Moves the window specified by `window_id` to the specified `screen`. See also [`window_get_current_screen`][`crate::classes::DisplayServer::window_get_current_screen`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows."]
        #[inline]
        pub fn window_set_current_screen(&mut self, screen: i32,) {
            self.window_set_current_screen_ex(screen,) . done()
        }
        #[doc = "Moves the window specified by `window_id` to the specified `screen`. See also [`window_get_current_screen`][`crate::classes::DisplayServer::window_get_current_screen`].\n\n**Note:** One of the following constants can be used as `screen`: `SCREEN_OF_MAIN_WINDOW`, `SCREEN_PRIMARY`, `SCREEN_WITH_MOUSE_FOCUS`, or `SCREEN_WITH_KEYBOARD_FOCUS`.\n\n**Note:** This method is implemented on Linux/X11, macOS, and Windows."]
        #[inline]
        pub fn window_set_current_screen_ex < 'ex > (&'ex mut self, screen: i32,) -> ExWindowSetCurrentScreen < 'ex > {
            ExWindowSetCurrentScreen::new(self, screen,)
        }
        #[doc = "Returns the position of the client area of the given window on the screen."]
        pub(crate) fn window_get_position_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_position_ex`][Self::window_get_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the position of the client area of the given window on the screen."]
        #[inline]
        pub fn window_get_position(&self,) -> Vector2i {
            self.window_get_position_ex() . done()
        }
        #[doc = "Returns the position of the client area of the given window on the screen."]
        #[inline]
        pub fn window_get_position_ex < 'ex > (&'ex self,) -> ExWindowGetPosition < 'ex > {
            ExWindowGetPosition::new(self,)
        }
        #[doc = "Returns the position of the given window on the screen including the borders drawn by the operating system. See also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        pub(crate) fn window_get_position_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_position_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_position_with_decorations_ex`][Self::window_get_position_with_decorations_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the position of the given window on the screen including the borders drawn by the operating system. See also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        #[inline]
        pub fn window_get_position_with_decorations(&self,) -> Vector2i {
            self.window_get_position_with_decorations_ex() . done()
        }
        #[doc = "Returns the position of the given window on the screen including the borders drawn by the operating system. See also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        #[inline]
        pub fn window_get_position_with_decorations_ex < 'ex > (&'ex self,) -> ExWindowGetPositionWithDecorations < 'ex > {
            ExWindowGetPositionWithDecorations::new(self,)
        }
        #[doc = "Sets the position of the given window to `position`. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin may be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`] and [`window_set_size`][`crate::classes::DisplayServer::window_set_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.position] instead.\n\n**Note:** On Linux (Wayland): this method is a no-op."]
        pub(crate) fn window_set_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_position_ex`][Self::window_set_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the position of the given window to `position`. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin may be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`] and [`window_set_size`][`crate::classes::DisplayServer::window_set_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.position] instead.\n\n**Note:** On Linux (Wayland): this method is a no-op."]
        #[inline]
        pub fn window_set_position(&mut self, position: Vector2i,) {
            self.window_set_position_ex(position,) . done()
        }
        #[doc = "Sets the position of the given window to `position`. On multi-monitor setups, the screen position is relative to the virtual desktop area. On multi-monitor setups with different screen resolutions or orientations, the origin may be located outside any display like this:\n\n```text\n* (0, 0)        +-------+\n                |       |\n+-------------+ |       |\n|             | |       |\n|             | |       |\n+-------------+ +-------+\n```\n\nSee also [`window_get_position`][`crate::classes::DisplayServer::window_get_position`] and [`window_set_size`][`crate::classes::DisplayServer::window_set_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.position] instead.\n\n**Note:** On Linux (Wayland): this method is a no-op."]
        #[inline]
        pub fn window_set_position_ex < 'ex > (&'ex mut self, position: Vector2i,) -> ExWindowSetPosition < 'ex > {
            ExWindowSetPosition::new(self, position,)
        }
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), excluding the borders drawn by the operating system. This is also called the \"client area\". See also [`window_get_size_with_decorations`][`crate::classes::DisplayServer::window_get_size_with_decorations`], [`window_set_size`][`crate::classes::DisplayServer::window_set_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        pub(crate) fn window_get_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_size_ex`][Self::window_get_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), excluding the borders drawn by the operating system. This is also called the \"client area\". See also [`window_get_size_with_decorations`][`crate::classes::DisplayServer::window_get_size_with_decorations`], [`window_set_size`][`crate::classes::DisplayServer::window_set_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        #[inline]
        pub fn window_get_size(&self,) -> Vector2i {
            self.window_get_size_ex() . done()
        }
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), excluding the borders drawn by the operating system. This is also called the \"client area\". See also [`window_get_size_with_decorations`][`crate::classes::DisplayServer::window_get_size_with_decorations`], [`window_set_size`][`crate::classes::DisplayServer::window_set_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`]."]
        #[inline]
        pub fn window_get_size_ex < 'ex > (&'ex self,) -> ExWindowGetSize < 'ex > {
            ExWindowGetSize::new(self,)
        }
        #[doc = "Sets the size of the given window to `size` (in pixels). See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`].\n\n**Note:** It's recommended to change this value using \\[member Window.size] instead."]
        pub(crate) fn window_set_size_full(&mut self, size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_size_ex`][Self::window_set_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the size of the given window to `size` (in pixels). See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`].\n\n**Note:** It's recommended to change this value using \\[member Window.size] instead."]
        #[inline]
        pub fn window_set_size(&mut self, size: Vector2i,) {
            self.window_set_size_ex(size,) . done()
        }
        #[doc = "Sets the size of the given window to `size` (in pixels). See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`] and [`window_get_position`][`crate::classes::DisplayServer::window_get_position`].\n\n**Note:** It's recommended to change this value using \\[member Window.size] instead."]
        #[inline]
        pub fn window_set_size_ex < 'ex > (&'ex mut self, size: Vector2i,) -> ExWindowSetSize < 'ex > {
            ExWindowSetSize::new(self, size,)
        }
        #[doc = "Sets the `callback` that will be called when the window specified by `window_id` is moved or resized.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        pub(crate) fn window_set_rect_changed_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_rect_changed_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_rect_changed_callback_ex`][Self::window_set_rect_changed_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `callback` that will be called when the window specified by `window_id` is moved or resized.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_rect_changed_callback(&mut self, callback: &Callable,) {
            self.window_set_rect_changed_callback_ex(callback,) . done()
        }
        #[doc = "Sets the `callback` that will be called when the window specified by `window_id` is moved or resized.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_rect_changed_callback_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExWindowSetRectChangedCallback < 'ex > {
            ExWindowSetRectChangedCallback::new(self, callback,)
        }
        #[doc = "Sets the `callback` that will be called when an event occurs in the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        pub(crate) fn window_set_window_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_event_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_window_event_callback_ex`][Self::window_set_window_event_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `callback` that will be called when an event occurs in the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_window_event_callback(&mut self, callback: &Callable,) {
            self.window_set_window_event_callback_ex(callback,) . done()
        }
        #[doc = "Sets the `callback` that will be called when an event occurs in the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_window_event_callback_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExWindowSetWindowEventCallback < 'ex > {
            ExWindowSetWindowEventCallback::new(self, callback,)
        }
        #[doc = "Sets the `callback` that should be called when any [`InputEvent`][crate::classes::InputEvent] is sent to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        pub(crate) fn window_set_input_event_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_event_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_input_event_callback_ex`][Self::window_set_input_event_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `callback` that should be called when any [`InputEvent`][crate::classes::InputEvent] is sent to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_input_event_callback(&mut self, callback: &Callable,) {
            self.window_set_input_event_callback_ex(callback,) . done()
        }
        #[doc = "Sets the `callback` that should be called when any [`InputEvent`][crate::classes::InputEvent] is sent to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_input_event_callback_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExWindowSetInputEventCallback < 'ex > {
            ExWindowSetInputEventCallback::new(self, callback,)
        }
        #[doc = "Sets the `callback` that should be called when text is entered using the virtual keyboard to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        pub(crate) fn window_set_input_text_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_input_text_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_input_text_callback_ex`][Self::window_set_input_text_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `callback` that should be called when text is entered using the virtual keyboard to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_input_text_callback(&mut self, callback: &Callable,) {
            self.window_set_input_text_callback_ex(callback,) . done()
        }
        #[doc = "Sets the `callback` that should be called when text is entered using the virtual keyboard to the window specified by `window_id`.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs."]
        #[inline]
        pub fn window_set_input_text_callback_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExWindowSetInputTextCallback < 'ex > {
            ExWindowSetInputTextCallback::new(self, callback,)
        }
        #[doc = "Sets the `callback` that should be called when files are dropped from the operating system's file manager to the window specified by `window_id`. `callback` should take one [`PackedStringArray`][crate::builtin::PackedStringArray] argument, which is the list of dropped files.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs.\n\n**Note:** This method is implemented on Windows, macOS, Linux (X11/Wayland), and Web."]
        pub(crate) fn window_set_drop_files_callback_full(&mut self, callback: RefArg < Callable >, window_id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i32,);
            let args = (callback, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_drop_files_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_drop_files_callback_ex`][Self::window_set_drop_files_callback_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `callback` that should be called when files are dropped from the operating system's file manager to the window specified by `window_id`. `callback` should take one [`PackedStringArray`][crate::builtin::PackedStringArray] argument, which is the list of dropped files.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs.\n\n**Note:** This method is implemented on Windows, macOS, Linux (X11/Wayland), and Web."]
        #[inline]
        pub fn window_set_drop_files_callback(&mut self, callback: &Callable,) {
            self.window_set_drop_files_callback_ex(callback,) . done()
        }
        #[doc = "Sets the `callback` that should be called when files are dropped from the operating system's file manager to the window specified by `window_id`. `callback` should take one [`PackedStringArray`][crate::builtin::PackedStringArray] argument, which is the list of dropped files.\n\n**Warning:** Advanced users only! Adding such a callback to a [`Window`][crate::classes::Window] node will override its default implementation, which can introduce bugs.\n\n**Note:** This method is implemented on Windows, macOS, Linux (X11/Wayland), and Web."]
        #[inline]
        pub fn window_set_drop_files_callback_ex < 'ex > (&'ex mut self, callback: &'ex Callable,) -> ExWindowSetDropFilesCallback < 'ex > {
            ExWindowSetDropFilesCallback::new(self, callback,)
        }
        #[doc = "Returns the [`instance_id`][`crate::obj::Gd::instance_id`] of the [`Window`][crate::classes::Window] the `window_id` is attached to."]
        pub(crate) fn window_get_attached_instance_id_full(&self, window_id: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_attached_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_attached_instance_id_ex`][Self::window_get_attached_instance_id_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the [`instance_id`][`crate::obj::Gd::instance_id`] of the [`Window`][crate::classes::Window] the `window_id` is attached to."]
        #[inline]
        pub fn window_get_attached_instance_id(&self,) -> u64 {
            self.window_get_attached_instance_id_ex() . done()
        }
        #[doc = "Returns the [`instance_id`][`crate::obj::Gd::instance_id`] of the [`Window`][crate::classes::Window] the `window_id` is attached to."]
        #[inline]
        pub fn window_get_attached_instance_id_ex < 'ex > (&'ex self,) -> ExWindowGetAttachedInstanceId < 'ex > {
            ExWindowGetAttachedInstanceId::new(self,)
        }
        #[doc = "Returns the window's maximum size (in pixels). See also [`window_set_max_size`][`crate::classes::DisplayServer::window_set_max_size`]."]
        pub(crate) fn window_get_max_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_max_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_max_size_ex`][Self::window_get_max_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the window's maximum size (in pixels). See also [`window_set_max_size`][`crate::classes::DisplayServer::window_set_max_size`]."]
        #[inline]
        pub fn window_get_max_size(&self,) -> Vector2i {
            self.window_get_max_size_ex() . done()
        }
        #[doc = "Returns the window's maximum size (in pixels). See also [`window_set_max_size`][`crate::classes::DisplayServer::window_set_max_size`]."]
        #[inline]
        pub fn window_get_max_size_ex < 'ex > (&'ex self,) -> ExWindowGetMaxSize < 'ex > {
            ExWindowGetMaxSize::new(self,)
        }
        #[doc = "Sets the maximum size of the window specified by `window_id` in pixels. Normally, the user will not be able to drag the window to make it larger than the specified size. See also [`window_get_max_size`][`crate::classes::DisplayServer::window_get_max_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.max_size] instead.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        pub(crate) fn window_set_max_size_full(&mut self, max_size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (max_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_max_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_max_size_ex`][Self::window_set_max_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the maximum size of the window specified by `window_id` in pixels. Normally, the user will not be able to drag the window to make it larger than the specified size. See also [`window_get_max_size`][`crate::classes::DisplayServer::window_get_max_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.max_size] instead.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        #[inline]
        pub fn window_set_max_size(&mut self, max_size: Vector2i,) {
            self.window_set_max_size_ex(max_size,) . done()
        }
        #[doc = "Sets the maximum size of the window specified by `window_id` in pixels. Normally, the user will not be able to drag the window to make it larger than the specified size. See also [`window_get_max_size`][`crate::classes::DisplayServer::window_get_max_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.max_size] instead.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        #[inline]
        pub fn window_set_max_size_ex < 'ex > (&'ex mut self, max_size: Vector2i,) -> ExWindowSetMaxSize < 'ex > {
            ExWindowSetMaxSize::new(self, max_size,)
        }
        #[doc = "Returns the window's minimum size (in pixels). See also [`window_set_min_size`][`crate::classes::DisplayServer::window_set_min_size`]."]
        pub(crate) fn window_get_min_size_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_min_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_min_size_ex`][Self::window_get_min_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the window's minimum size (in pixels). See also [`window_set_min_size`][`crate::classes::DisplayServer::window_set_min_size`]."]
        #[inline]
        pub fn window_get_min_size(&self,) -> Vector2i {
            self.window_get_min_size_ex() . done()
        }
        #[doc = "Returns the window's minimum size (in pixels). See also [`window_set_min_size`][`crate::classes::DisplayServer::window_set_min_size`]."]
        #[inline]
        pub fn window_get_min_size_ex < 'ex > (&'ex self,) -> ExWindowGetMinSize < 'ex > {
            ExWindowGetMinSize::new(self,)
        }
        #[doc = "Sets the minimum size for the given window to `min_size` in pixels. Normally, the user will not be able to drag the window to make it smaller than the specified size. See also [`window_get_min_size`][`crate::classes::DisplayServer::window_get_min_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.min_size] instead.\n\n**Note:** By default, the main window has a minimum size of `Vector2i(64, 64)`. This prevents issues that can arise when the window is resized to a near-zero size.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        pub(crate) fn window_set_min_size_full(&mut self, min_size: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (min_size, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_min_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_min_size_ex`][Self::window_set_min_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the minimum size for the given window to `min_size` in pixels. Normally, the user will not be able to drag the window to make it smaller than the specified size. See also [`window_get_min_size`][`crate::classes::DisplayServer::window_get_min_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.min_size] instead.\n\n**Note:** By default, the main window has a minimum size of `Vector2i(64, 64)`. This prevents issues that can arise when the window is resized to a near-zero size.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        #[inline]
        pub fn window_set_min_size(&mut self, min_size: Vector2i,) {
            self.window_set_min_size_ex(min_size,) . done()
        }
        #[doc = "Sets the minimum size for the given window to `min_size` in pixels. Normally, the user will not be able to drag the window to make it smaller than the specified size. See also [`window_get_min_size`][`crate::classes::DisplayServer::window_get_min_size`].\n\n**Note:** It's recommended to change this value using \\[member Window.min_size] instead.\n\n**Note:** By default, the main window has a minimum size of `Vector2i(64, 64)`. This prevents issues that can arise when the window is resized to a near-zero size.\n\n**Note:** Using third-party tools, it is possible for users to disable window geometry restrictions and therefore bypass this limit."]
        #[inline]
        pub fn window_set_min_size_ex < 'ex > (&'ex mut self, min_size: Vector2i,) -> ExWindowSetMinSize < 'ex > {
            ExWindowSetMinSize::new(self, min_size,)
        }
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), including the borders drawn by the operating system. See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`]."]
        pub(crate) fn window_get_size_with_decorations_full(&self, window_id: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_size_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_size_with_decorations_ex`][Self::window_get_size_with_decorations_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), including the borders drawn by the operating system. See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`]."]
        #[inline]
        pub fn window_get_size_with_decorations(&self,) -> Vector2i {
            self.window_get_size_with_decorations_ex() . done()
        }
        #[doc = "Returns the size of the window specified by `window_id` (in pixels), including the borders drawn by the operating system. See also [`window_get_size`][`crate::classes::DisplayServer::window_get_size`]."]
        #[inline]
        pub fn window_get_size_with_decorations_ex < 'ex > (&'ex self,) -> ExWindowGetSizeWithDecorations < 'ex > {
            ExWindowGetSizeWithDecorations::new(self,)
        }
        #[doc = "Returns the mode of the given window."]
        pub(crate) fn window_get_mode_full(&self, window_id: i32,) -> crate::classes::display_server::WindowMode {
            type CallRet = crate::classes::display_server::WindowMode;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_mode_ex`][Self::window_get_mode_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the mode of the given window."]
        #[inline]
        pub fn window_get_mode(&self,) -> crate::classes::display_server::WindowMode {
            self.window_get_mode_ex() . done()
        }
        #[doc = "Returns the mode of the given window."]
        #[inline]
        pub fn window_get_mode_ex < 'ex > (&'ex self,) -> ExWindowGetMode < 'ex > {
            ExWindowGetMode::new(self,)
        }
        #[doc = "Sets window mode for the given window to `mode`.\n\n**Note:** On Android, setting it to [`WindowMode::FULLSCREEN`][`crate::classes::display_server::WindowMode::FULLSCREEN`] or [`WindowMode::EXCLUSIVE_FULLSCREEN`][`crate::classes::display_server::WindowMode::EXCLUSIVE_FULLSCREEN`] will enable immersive mode.\n\n**Note:** Setting the window to full screen forcibly sets the borderless flag to `true`, so make sure to set it back to `false` when not wanted."]
        pub(crate) fn window_set_mode_full(&mut self, mode: crate::classes::display_server::WindowMode, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowMode, i32,);
            let args = (mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_mode_ex`][Self::window_set_mode_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets window mode for the given window to `mode`.\n\n**Note:** On Android, setting it to [`WindowMode::FULLSCREEN`][`crate::classes::display_server::WindowMode::FULLSCREEN`] or [`WindowMode::EXCLUSIVE_FULLSCREEN`][`crate::classes::display_server::WindowMode::EXCLUSIVE_FULLSCREEN`] will enable immersive mode.\n\n**Note:** Setting the window to full screen forcibly sets the borderless flag to `true`, so make sure to set it back to `false` when not wanted."]
        #[inline]
        pub fn window_set_mode(&mut self, mode: crate::classes::display_server::WindowMode,) {
            self.window_set_mode_ex(mode,) . done()
        }
        #[doc = "Sets window mode for the given window to `mode`.\n\n**Note:** On Android, setting it to [`WindowMode::FULLSCREEN`][`crate::classes::display_server::WindowMode::FULLSCREEN`] or [`WindowMode::EXCLUSIVE_FULLSCREEN`][`crate::classes::display_server::WindowMode::EXCLUSIVE_FULLSCREEN`] will enable immersive mode.\n\n**Note:** Setting the window to full screen forcibly sets the borderless flag to `true`, so make sure to set it back to `false` when not wanted."]
        #[inline]
        pub fn window_set_mode_ex < 'ex > (&'ex mut self, mode: crate::classes::display_server::WindowMode,) -> ExWindowSetMode < 'ex > {
            ExWindowSetMode::new(self, mode,)
        }
        #[doc = "Enables or disables the given window's given `flag`."]
        pub(crate) fn window_set_flag_full(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowFlags, bool, i32,);
            let args = (flag, enabled, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_flag_ex`][Self::window_set_flag_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Enables or disables the given window's given `flag`."]
        #[inline]
        pub fn window_set_flag(&mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) {
            self.window_set_flag_ex(flag, enabled,) . done()
        }
        #[doc = "Enables or disables the given window's given `flag`."]
        #[inline]
        pub fn window_set_flag_ex < 'ex > (&'ex mut self, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> ExWindowSetFlag < 'ex > {
            ExWindowSetFlag::new(self, flag, enabled,)
        }
        #[doc = "Returns the current value of the given window's `flag`."]
        pub(crate) fn window_get_flag_full(&self, flag: crate::classes::display_server::WindowFlags, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::display_server::WindowFlags, i32,);
            let args = (flag, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_flag_ex`][Self::window_get_flag_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current value of the given window's `flag`."]
        #[inline]
        pub fn window_get_flag(&self, flag: crate::classes::display_server::WindowFlags,) -> bool {
            self.window_get_flag_ex(flag,) . done()
        }
        #[doc = "Returns the current value of the given window's `flag`."]
        #[inline]
        pub fn window_get_flag_ex < 'ex > (&'ex self, flag: crate::classes::display_server::WindowFlags,) -> ExWindowGetFlag < 'ex > {
            ExWindowGetFlag::new(self, flag,)
        }
        #[doc = "When [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set, set offset to the center of the first titlebar button.\n\n**Note:** This flag is implemented only on macOS."]
        pub(crate) fn window_set_window_buttons_offset_full(&mut self, offset: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (offset, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_window_buttons_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_window_buttons_offset_ex`][Self::window_set_window_buttons_offset_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "When [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set, set offset to the center of the first titlebar button.\n\n**Note:** This flag is implemented only on macOS."]
        #[inline]
        pub fn window_set_window_buttons_offset(&mut self, offset: Vector2i,) {
            self.window_set_window_buttons_offset_ex(offset,) . done()
        }
        #[doc = "When [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set, set offset to the center of the first titlebar button.\n\n**Note:** This flag is implemented only on macOS."]
        #[inline]
        pub fn window_set_window_buttons_offset_ex < 'ex > (&'ex mut self, offset: Vector2i,) -> ExWindowSetWindowButtonsOffset < 'ex > {
            ExWindowSetWindowButtonsOffset::new(self, offset,)
        }
        #[doc = "Returns left margins (`x`), right margins (`y`) and height (`z`) of the title that are safe to use (contains no buttons or other elements) when [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set."]
        pub(crate) fn window_get_safe_title_margins_full(&self, window_id: i32,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_safe_title_margins", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_safe_title_margins_ex`][Self::window_get_safe_title_margins_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns left margins (`x`), right margins (`y`) and height (`z`) of the title that are safe to use (contains no buttons or other elements) when [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set."]
        #[inline]
        pub fn window_get_safe_title_margins(&self,) -> Vector3i {
            self.window_get_safe_title_margins_ex() . done()
        }
        #[doc = "Returns left margins (`x`), right margins (`y`) and height (`z`) of the title that are safe to use (contains no buttons or other elements) when [`WindowFlags::EXTEND_TO_TITLE`][`crate::classes::display_server::WindowFlags::EXTEND_TO_TITLE`] flag is set."]
        #[inline]
        pub fn window_get_safe_title_margins_ex < 'ex > (&'ex self,) -> ExWindowGetSafeTitleMargins < 'ex > {
            ExWindowGetSafeTitleMargins::new(self,)
        }
        #[doc = "Makes the window specified by `window_id` request attention, which is materialized by the window title and taskbar entry blinking until the window is focused. This usually has no visible effect if the window is currently focused. The exact behavior varies depending on the operating system."]
        pub(crate) fn window_request_attention_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_request_attention", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_request_attention_ex`][Self::window_request_attention_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Makes the window specified by `window_id` request attention, which is materialized by the window title and taskbar entry blinking until the window is focused. This usually has no visible effect if the window is currently focused. The exact behavior varies depending on the operating system."]
        #[inline]
        pub fn window_request_attention(&mut self,) {
            self.window_request_attention_ex() . done()
        }
        #[doc = "Makes the window specified by `window_id` request attention, which is materialized by the window title and taskbar entry blinking until the window is focused. This usually has no visible effect if the window is currently focused. The exact behavior varies depending on the operating system."]
        #[inline]
        pub fn window_request_attention_ex < 'ex > (&'ex mut self,) -> ExWindowRequestAttention < 'ex > {
            ExWindowRequestAttention::new(self,)
        }
        #[doc = "Moves the window specified by `window_id` to the foreground, so that it is visible over other windows."]
        pub(crate) fn window_move_to_foreground_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_move_to_foreground", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_move_to_foreground_ex`][Self::window_move_to_foreground_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Moves the window specified by `window_id` to the foreground, so that it is visible over other windows."]
        #[inline]
        pub fn window_move_to_foreground(&mut self,) {
            self.window_move_to_foreground_ex() . done()
        }
        #[doc = "Moves the window specified by `window_id` to the foreground, so that it is visible over other windows."]
        #[inline]
        pub fn window_move_to_foreground_ex < 'ex > (&'ex mut self,) -> ExWindowMoveToForeground < 'ex > {
            ExWindowMoveToForeground::new(self,)
        }
        #[doc = "Returns `true` if the window specified by `window_id` is focused."]
        pub(crate) fn window_is_focused_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_focused", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_is_focused_ex`][Self::window_is_focused_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the window specified by `window_id` is focused."]
        #[inline]
        pub fn window_is_focused(&self,) -> bool {
            self.window_is_focused_ex() . done()
        }
        #[doc = "Returns `true` if the window specified by `window_id` is focused."]
        #[inline]
        pub fn window_is_focused_ex < 'ex > (&'ex self,) -> ExWindowIsFocused < 'ex > {
            ExWindowIsFocused::new(self,)
        }
        #[doc = "Returns `true` if anything can be drawn in the window specified by `window_id`, `false` otherwise. Using the `--disable-render-loop` command line argument or a headless build will return `false`."]
        pub(crate) fn window_can_draw_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_can_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_can_draw_ex`][Self::window_can_draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if anything can be drawn in the window specified by `window_id`, `false` otherwise. Using the `--disable-render-loop` command line argument or a headless build will return `false`."]
        #[inline]
        pub fn window_can_draw(&self,) -> bool {
            self.window_can_draw_ex() . done()
        }
        #[doc = "Returns `true` if anything can be drawn in the window specified by `window_id`, `false` otherwise. Using the `--disable-render-loop` command line argument or a headless build will return `false`."]
        #[inline]
        pub fn window_can_draw_ex < 'ex > (&'ex self,) -> ExWindowCanDraw < 'ex > {
            ExWindowCanDraw::new(self,)
        }
        #[doc = "Sets window transient parent. Transient window will be destroyed with its transient parent and will return focus to their parent when closed. The transient window is displayed on top of a non-exclusive full-screen parent window. Transient windows can't enter full-screen mode.\n\n**Note:** It's recommended to change this value using \\[member Window.transient] instead.\n\n**Note:** The behavior might be different depending on the platform."]
        pub fn window_set_transient(&mut self, window_id: i32, parent_window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (window_id, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_transient", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, this window will always stay on top of its parent window, parent window will ignore input while this window is opened.\n\n**Note:** On macOS, exclusive windows are confined to the same space (virtual desktop or screen) as the parent window.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn window_set_exclusive(&mut self, window_id: i32, exclusive: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (window_id, exclusive,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_exclusive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) should be enabled for the window specified by `window_id`. See also [`window_set_ime_position`][`crate::classes::DisplayServer::window_set_ime_position`]."]
        pub(crate) fn window_set_ime_active_full(&mut self, active: bool, window_id: i32,) {
            type CallRet = ();
            type CallParams = (bool, i32,);
            let args = (active, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_ime_active_ex`][Self::window_set_ime_active_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets whether [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) should be enabled for the window specified by `window_id`. See also [`window_set_ime_position`][`crate::classes::DisplayServer::window_set_ime_position`]."]
        #[inline]
        pub fn window_set_ime_active(&mut self, active: bool,) {
            self.window_set_ime_active_ex(active,) . done()
        }
        #[doc = "Sets whether [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) should be enabled for the window specified by `window_id`. See also [`window_set_ime_position`][`crate::classes::DisplayServer::window_set_ime_position`]."]
        #[inline]
        pub fn window_set_ime_active_ex < 'ex > (&'ex mut self, active: bool,) -> ExWindowSetImeActive < 'ex > {
            ExWindowSetImeActive::new(self, active,)
        }
        #[doc = "Sets the position of the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) popup for the specified `window_id`. Only effective if [`window_set_ime_active`][`crate::classes::DisplayServer::window_set_ime_active`] was set to `true` for the specified `window_id`."]
        pub(crate) fn window_set_ime_position_full(&mut self, position: Vector2i, window_id: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32,);
            let args = (position, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_ime_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_ime_position_ex`][Self::window_set_ime_position_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the position of the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) popup for the specified `window_id`. Only effective if [`window_set_ime_active`][`crate::classes::DisplayServer::window_set_ime_active`] was set to `true` for the specified `window_id`."]
        #[inline]
        pub fn window_set_ime_position(&mut self, position: Vector2i,) {
            self.window_set_ime_position_ex(position,) . done()
        }
        #[doc = "Sets the position of the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) popup for the specified `window_id`. Only effective if [`window_set_ime_active`][`crate::classes::DisplayServer::window_set_ime_active`] was set to `true` for the specified `window_id`."]
        #[inline]
        pub fn window_set_ime_position_ex < 'ex > (&'ex mut self, position: Vector2i,) -> ExWindowSetImePosition < 'ex > {
            ExWindowSetImePosition::new(self, position,)
        }
        #[doc = "Sets the V-Sync mode of the given window. See also \\[member ProjectSettings.display/window/vsync/vsync_mode].\n\nDepending on the platform and used renderer, the engine will fall back to [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] if the desired mode is not supported.\n\n**Note:** V-Sync modes other than [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] are only supported in the Forward+ and Mobile rendering methods, not Compatibility."]
        pub(crate) fn window_set_vsync_mode_full(&mut self, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::VSyncMode, i32,);
            let args = (vsync_mode, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_vsync_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_set_vsync_mode_ex`][Self::window_set_vsync_mode_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the V-Sync mode of the given window. See also \\[member ProjectSettings.display/window/vsync/vsync_mode].\n\nDepending on the platform and used renderer, the engine will fall back to [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] if the desired mode is not supported.\n\n**Note:** V-Sync modes other than [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] are only supported in the Forward+ and Mobile rendering methods, not Compatibility."]
        #[inline]
        pub fn window_set_vsync_mode(&mut self, vsync_mode: crate::classes::display_server::VSyncMode,) {
            self.window_set_vsync_mode_ex(vsync_mode,) . done()
        }
        #[doc = "Sets the V-Sync mode of the given window. See also \\[member ProjectSettings.display/window/vsync/vsync_mode].\n\nDepending on the platform and used renderer, the engine will fall back to [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] if the desired mode is not supported.\n\n**Note:** V-Sync modes other than [`VSyncMode::ENABLED`][`crate::classes::display_server::VSyncMode::ENABLED`] are only supported in the Forward+ and Mobile rendering methods, not Compatibility."]
        #[inline]
        pub fn window_set_vsync_mode_ex < 'ex > (&'ex mut self, vsync_mode: crate::classes::display_server::VSyncMode,) -> ExWindowSetVSyncMode < 'ex > {
            ExWindowSetVSyncMode::new(self, vsync_mode,)
        }
        #[doc = "Returns the V-Sync mode of the given window."]
        pub(crate) fn window_get_vsync_mode_full(&self, window_id: i32,) -> crate::classes::display_server::VSyncMode {
            type CallRet = crate::classes::display_server::VSyncMode;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_get_vsync_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_get_vsync_mode_ex`][Self::window_get_vsync_mode_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the V-Sync mode of the given window."]
        #[inline]
        pub fn window_get_vsync_mode(&self,) -> crate::classes::display_server::VSyncMode {
            self.window_get_vsync_mode_ex() . done()
        }
        #[doc = "Returns the V-Sync mode of the given window."]
        #[inline]
        pub fn window_get_vsync_mode_ex < 'ex > (&'ex self,) -> ExWindowGetVSyncMode < 'ex > {
            ExWindowGetVSyncMode::new(self,)
        }
        #[doc = "Returns `true` if the given window can be maximized (the maximize button is enabled)."]
        pub(crate) fn window_is_maximize_allowed_full(&self, window_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_is_maximize_allowed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_is_maximize_allowed_ex`][Self::window_is_maximize_allowed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if the given window can be maximized (the maximize button is enabled)."]
        #[inline]
        pub fn window_is_maximize_allowed(&self,) -> bool {
            self.window_is_maximize_allowed_ex() . done()
        }
        #[doc = "Returns `true` if the given window can be maximized (the maximize button is enabled)."]
        #[inline]
        pub fn window_is_maximize_allowed_ex < 'ex > (&'ex self,) -> ExWindowIsMaximizeAllowed < 'ex > {
            ExWindowIsMaximizeAllowed::new(self,)
        }
        #[doc = "Returns `true` if double-clicking on a window's title should maximize it.\n\n**Note:** This method is implemented only on macOS."]
        pub fn window_maximize_on_title_dbl_click(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_maximize_on_title_dbl_click", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if double-clicking on a window's title should minimize it.\n\n**Note:** This method is implemented only on macOS."]
        pub fn window_minimize_on_title_dbl_click(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_minimize_on_title_dbl_click", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Starts an interactive drag operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's title bar. Using this method allows the window to participate in space switching, tiling, and other system features.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        pub(crate) fn window_start_drag_full(&mut self, window_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_start_drag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_start_drag_ex`][Self::window_start_drag_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts an interactive drag operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's title bar. Using this method allows the window to participate in space switching, tiling, and other system features.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_start_drag(&mut self,) {
            self.window_start_drag_ex() . done()
        }
        #[doc = "Starts an interactive drag operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's title bar. Using this method allows the window to participate in space switching, tiling, and other system features.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_start_drag_ex < 'ex > (&'ex mut self,) -> ExWindowStartDrag < 'ex > {
            ExWindowStartDrag::new(self,)
        }
        #[doc = "Starts an interactive resize operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's edge.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        pub(crate) fn window_start_resize_full(&mut self, edge: crate::classes::display_server::WindowResizeEdge, window_id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowResizeEdge, i32,);
            let args = (edge, window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_start_resize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`window_start_resize_ex`][Self::window_start_resize_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts an interactive resize operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's edge.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_start_resize(&mut self, edge: crate::classes::display_server::WindowResizeEdge,) {
            self.window_start_resize_ex(edge,) . done()
        }
        #[doc = "Starts an interactive resize operation on the window with the given `window_id`, using the current mouse position. Call this method when handling a mouse button being pressed to simulate a pressed event on the window's edge.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        #[inline]
        pub fn window_start_resize_ex < 'ex > (&'ex mut self, edge: crate::classes::display_server::WindowResizeEdge,) -> ExWindowStartResize < 'ex > {
            ExWindowStartResize::new(self, edge,)
        }
        #[doc = "Sets the background color of the root window.\n\n**Note:** This method is implemented only on Android."]
        pub fn window_set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "window_set_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `1` if a high-contrast user interface theme should be used, `0` otherwise. Returns `-1` if status is unknown.\n\n**Note:** This method is implemented on Linux (X11/Wayland, GNOME), macOS, and Windows."]
        pub fn accessibility_should_increase_contrast(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_increase_contrast", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `1` if flashing, blinking, and other moving content that can cause seizures in users with photosensitive epilepsy should be disabled, `0` otherwise. Returns `-1` if status is unknown.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn accessibility_should_reduce_animation(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_reduce_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `1` if background images, transparency, and other features that can reduce the contrast between the foreground and background should be disabled, `0` otherwise. Returns `-1` if status is unknown.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn accessibility_should_reduce_transparency(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_should_reduce_transparency", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `1` if a screen reader, Braille display or other assistive app is active, `0` otherwise. Returns `-1` if status is unknown.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Accessibility debugging tools, such as Accessibility Insights for Windows, Accessibility Inspector (macOS), or AT-SPI Browser (Linux/BSD), do not count as assistive apps and will not affect this value. To test your project with these tools, set \\[member ProjectSettings.accessibility/general/accessibility_support] to `1`."]
        pub fn accessibility_screen_reader_active(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_screen_reader_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new, empty accessibility element resource.\n\n**Note:** An accessibility element is created and freed automatically for each [`Node`][crate::classes::Node]. In general, this function should not be called manually."]
        pub fn accessibility_create_element(&mut self, window_id: i32, role: crate::classes::display_server::AccessibilityRole,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32, crate::classes::display_server::AccessibilityRole,);
            let args = (window_id, role,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new, empty accessibility sub-element resource. Sub-elements can be used to provide accessibility information for objects which are not [`Node`][crate::classes::Node]s, such as list items, table cells, or menu items. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method."]
        pub(crate) fn accessibility_create_sub_element_full(&mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole, insert_pos: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, crate::classes::display_server::AccessibilityRole, i32,);
            let args = (parent_rid, role, insert_pos,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_sub_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`accessibility_create_sub_element_ex`][Self::accessibility_create_sub_element_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new, empty accessibility sub-element resource. Sub-elements can be used to provide accessibility information for objects which are not [`Node`][crate::classes::Node]s, such as list items, table cells, or menu items. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method."]
        #[inline]
        pub fn accessibility_create_sub_element(&mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> Rid {
            self.accessibility_create_sub_element_ex(parent_rid, role,) . done()
        }
        #[doc = "Creates a new, empty accessibility sub-element resource. Sub-elements can be used to provide accessibility information for objects which are not [`Node`][crate::classes::Node]s, such as list items, table cells, or menu items. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method."]
        #[inline]
        pub fn accessibility_create_sub_element_ex < 'ex > (&'ex mut self, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> ExAccessibilityCreateSubElement < 'ex > {
            ExAccessibilityCreateSubElement::new(self, parent_rid, role,)
        }
        #[doc = "Creates a new, empty accessibility sub-element from the shaped text buffer. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method.\n\nIf `is_last_line` is `true`, no trailing newline is appended to the text content. Set to `true` for the last line in multi-line text fields and for single-line text fields."]
        pub(crate) fn accessibility_create_sub_text_edit_elements_full(&mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32, insert_pos: i32, is_last_line: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Rid, f32, i32, bool,);
            let args = (parent_rid, shaped_text, min_height, insert_pos, is_last_line,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_create_sub_text_edit_elements", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`accessibility_create_sub_text_edit_elements_ex`][Self::accessibility_create_sub_text_edit_elements_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new, empty accessibility sub-element from the shaped text buffer. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method.\n\nIf `is_last_line` is `true`, no trailing newline is appended to the text content. Set to `true` for the last line in multi-line text fields and for single-line text fields."]
        #[inline]
        pub fn accessibility_create_sub_text_edit_elements(&mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> Rid {
            self.accessibility_create_sub_text_edit_elements_ex(parent_rid, shaped_text, min_height,) . done()
        }
        #[doc = "Creates a new, empty accessibility sub-element from the shaped text buffer. Sub-elements are freed automatically when the parent element is freed, or can be freed early using the [`accessibility_free_element`][`crate::classes::DisplayServer::accessibility_free_element`] method.\n\nIf `is_last_line` is `true`, no trailing newline is appended to the text content. Set to `true` for the last line in multi-line text fields and for single-line text fields."]
        #[inline]
        pub fn accessibility_create_sub_text_edit_elements_ex < 'ex > (&'ex mut self, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> ExAccessibilityCreateSubTextEditElements < 'ex > {
            ExAccessibilityCreateSubTextEditElements::new(self, parent_rid, shaped_text, min_height,)
        }
        #[doc = "Returns `true` if `id` is a valid accessibility element."]
        pub fn accessibility_has_element(&self, id: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_has_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees the accessibility element `id` created by [`accessibility_create_element`][`crate::classes::DisplayServer::accessibility_create_element`], [`accessibility_create_sub_element`][`crate::classes::DisplayServer::accessibility_create_sub_element`], or [`accessibility_create_sub_text_edit_elements`][`crate::classes::DisplayServer::accessibility_create_sub_text_edit_elements`]."]
        pub fn accessibility_free_element(&mut self, id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_free_element", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of the accessibility element `id` to `meta`."]
        pub fn accessibility_element_set_meta(&mut self, id: Rid, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (id, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_element_set_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata of the accessibility element `id`."]
        pub fn accessibility_element_get_meta(&self, id: Rid,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_element_get_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets window outer (with decorations) and inner (without decorations) bounds for assistive apps.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Advanced users only! [`Window`][crate::classes::Window] objects call this method automatically."]
        pub fn accessibility_set_window_rect(&mut self, window_id: i32, rect_out: Rect2, rect_in: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2, Rect2,);
            let args = (window_id, rect_out, rect_in,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_set_window_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the window focused state for assistive apps.\n\n**Note:** This method is implemented on Linux, macOS, and Windows.\n\n**Note:** Advanced users only! [`Window`][crate::classes::Window] objects call this method automatically."]
        pub fn accessibility_set_window_focused(&mut self, window_id: i32, focused: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (window_id, focused,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_set_window_focused", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets currently focused element."]
        pub fn accessibility_update_set_focus(&mut self, id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the main accessibility element of the OS native window."]
        pub fn accessibility_get_window_root(&self, window_id: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_get_window_root", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element accessibility role."]
        pub fn accessibility_update_set_role(&mut self, id: Rid, role: crate::classes::display_server::AccessibilityRole,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityRole,);
            let args = (id, role,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_role", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element accessibility name."]
        pub fn accessibility_update_set_name(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element accessibility extra information added to the element name."]
        pub fn accessibility_update_set_extra_info(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_extra_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element accessibility description."]
        pub fn accessibility_update_set_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element text value."]
        pub fn accessibility_update_set_value(&mut self, id: Rid, value: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, value.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets tooltip text."]
        pub fn accessibility_update_set_tooltip(&mut self, id: Rid, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element bounding box, relative to the node position."]
        pub fn accessibility_update_set_bounds(&mut self, id: Rid, p_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2,);
            let args = (id, p_rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_bounds", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element 2D transform."]
        pub fn accessibility_update_set_transform(&mut self, id: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (id, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a child accessibility element.\n\n**Note:** [`Node`][crate::classes::Node] children and sub-elements are added to the child list automatically."]
        pub fn accessibility_update_add_child(&mut self, id: Rid, child_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, child_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that is controlled by this element."]
        pub fn accessibility_update_add_related_controls(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_controls", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that details this element."]
        pub fn accessibility_update_add_related_details(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_details", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that describes this element."]
        pub fn accessibility_update_add_related_described_by(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_described_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that this element flow into."]
        pub fn accessibility_update_add_related_flow_to(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_flow_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that labels this element."]
        pub fn accessibility_update_add_related_labeled_by(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_labeled_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that is part of the same radio group.\n\n**Note:** This method should be called on each element of the group, using all other elements as `related_id`."]
        pub fn accessibility_update_add_related_radio_group(&mut self, id: Rid, related_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, related_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_related_radio_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an element that is an active descendant of this element."]
        pub fn accessibility_update_set_active_descendant(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_active_descendant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets next element on the line."]
        pub fn accessibility_update_set_next_on_line(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_next_on_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets previous element on the line."]
        pub fn accessibility_update_set_previous_on_line(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_previous_on_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the element to be a member of the group."]
        pub fn accessibility_update_set_member_of(&mut self, id: Rid, group_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, group_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_member_of", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets target element for the link."]
        pub fn accessibility_update_set_in_page_link_target(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_in_page_link_target", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an element which contains an error message for this element."]
        pub fn accessibility_update_set_error_message(&mut self, id: Rid, other_id: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (id, other_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_error_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the priority of the live region updates."]
        pub fn accessibility_update_set_live(&mut self, id: Rid, live: crate::classes::display_server::AccessibilityLiveMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityLiveMode,);
            let args = (id, live,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_live", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a callback for the accessibility action (action which can be performed by using a special screen reader command or buttons on the Braille display), and marks this action as supported. The action callback receives one [`Variant`][crate::builtin::Variant] argument, which value depends on action type."]
        pub fn accessibility_update_add_action(&mut self, id: Rid, action: crate::classes::display_server::AccessibilityAction, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::display_server::AccessibilityAction, RefArg < 'a0, Callable >,);
            let args = (id, action, RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds support for a custom accessibility action. `action_id` is passed as an argument to the callback of [`AccessibilityAction::CUSTOM`][`crate::classes::display_server::AccessibilityAction::CUSTOM`] action."]
        pub fn accessibility_update_add_custom_action(&mut self, id: Rid, action_id: i32, action_description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (id, action_id, action_description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_add_custom_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets number of rows in the table."]
        pub fn accessibility_update_set_table_row_count(&mut self, id: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_row_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets number of columns in the table."]
        pub fn accessibility_update_set_table_column_count(&mut self, id: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_column_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets position of the row in the table."]
        pub fn accessibility_update_set_table_row_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_row_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets position of the column."]
        pub fn accessibility_update_set_table_column_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_column_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets cell position in the table."]
        pub fn accessibility_update_set_table_cell_position(&mut self, id: Rid, row_index: i32, column_index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (id, row_index, column_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_cell_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets cell row/column span."]
        pub fn accessibility_update_set_table_cell_span(&mut self, id: Rid, row_span: i32, column_span: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (id, row_span, column_span,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_table_cell_span", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets number of items in the list."]
        pub fn accessibility_update_set_list_item_count(&mut self, id: Rid, size: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the position of the element in the list."]
        pub fn accessibility_update_set_list_item_index(&mut self, id: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the hierarchical level of the element in the list."]
        pub fn accessibility_update_set_list_item_level(&mut self, id: Rid, level: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (id, level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets list/tree item selected status."]
        pub fn accessibility_update_set_list_item_selected(&mut self, id: Rid, selected: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, selected,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets list/tree item expanded status."]
        pub fn accessibility_update_set_list_item_expanded(&mut self, id: Rid, expanded: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, expanded,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_item_expanded", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets popup type for popup buttons."]
        pub fn accessibility_update_set_popup_type(&mut self, id: Rid, popup: crate::classes::display_server::AccessibilityPopupType,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityPopupType,);
            let args = (id, popup,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_popup_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element checked state."]
        pub fn accessibility_update_set_checked(&mut self, id: Rid, checekd: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, checekd,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets numeric value."]
        pub fn accessibility_update_set_num_value(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets numeric value range."]
        pub fn accessibility_update_set_num_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets numeric value step."]
        pub fn accessibility_update_set_num_step(&mut self, id: Rid, step: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, step,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_step", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets numeric value jump."]
        pub fn accessibility_update_set_num_jump(&mut self, id: Rid, jump: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, jump,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_num_jump", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scroll bar x position."]
        pub fn accessibility_update_set_scroll_x(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_x", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scroll bar x range."]
        pub fn accessibility_update_set_scroll_x_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_x_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scroll bar y position."]
        pub fn accessibility_update_set_scroll_y(&mut self, id: Rid, position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_y", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets scroll bar y range."]
        pub fn accessibility_update_set_scroll_y_range(&mut self, id: Rid, min: f64, max: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64,);
            let args = (id, min, max,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_scroll_y_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets text underline/overline/strikethrough."]
        pub fn accessibility_update_set_text_decorations(&mut self, id: Rid, underline: bool, strikethrough: bool, overline: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool, bool, bool,);
            let args = (id, underline, strikethrough, overline,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_decorations", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element text alignment."]
        pub fn accessibility_update_set_text_align(&mut self, id: Rid, align: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (Rid, crate::global::HorizontalAlignment,);
            let args = (id, align,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_align", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets text selection to the text field. `text_start_id` and `text_end_id` should be elements created by [`accessibility_create_sub_text_edit_elements`][`crate::classes::DisplayServer::accessibility_create_sub_text_edit_elements`]. Character offsets are relative to the corresponding element."]
        pub fn accessibility_update_set_text_selection(&mut self, id: Rid, text_start_id: Rid, start_char: i32, text_end_id: Rid, end_char: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i32, Rid, i32,);
            let args = (id, text_start_id, start_char, text_end_id, end_char,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element flag."]
        pub fn accessibility_update_set_flag(&mut self, id: Rid, flag: crate::classes::display_server::AccessibilityFlags, value: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::display_server::AccessibilityFlags, bool,);
            let args = (id, flag, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element class name."]
        pub fn accessibility_update_set_classname(&mut self, id: Rid, classname: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, classname.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_classname", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets placeholder text."]
        pub fn accessibility_update_set_placeholder(&mut self, id: Rid, placeholder: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, placeholder.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element text language."]
        pub fn accessibility_update_set_language(&mut self, id: Rid, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets text orientation."]
        pub fn accessibility_update_set_text_orientation(&mut self, id: Rid, vertical: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, vertical,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_text_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the orientation of the list elements."]
        pub fn accessibility_update_set_list_orientation(&mut self, id: Rid, vertical: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (id, vertical,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_list_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the list of keyboard shortcuts used by element."]
        pub fn accessibility_update_set_shortcut(&mut self, id: Rid, shortcut: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, shortcut.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets link URL."]
        pub fn accessibility_update_set_url(&mut self, id: Rid, url: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, url.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_url", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element accessibility role description text."]
        pub fn accessibility_update_set_role_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_role_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets human-readable description of the current checked state."]
        pub fn accessibility_update_set_state_description(&mut self, id: Rid, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_state_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element color value."]
        pub fn accessibility_update_set_color_value(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_color_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element background color."]
        pub fn accessibility_update_set_background_color(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_background_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets element foreground color."]
        pub fn accessibility_update_set_foreground_color(&mut self, id: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (id, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "accessibility_update_set_foreground_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text selection in the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) composition string, with the [`Vector2i`][crate::builtin::Vector2i]'s `x` component being the caret position and `y` being the length of the selection.\n\n**Note:** This method is implemented only on macOS."]
        pub fn ime_get_selection(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the composition string contained within the [Input Method Editor](https://en.wikipedia.org/wiki/Input_method) window.\n\n**Note:** This method is implemented only on macOS."]
        pub fn ime_get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "ime_get_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows the virtual keyboard if the platform has one.\n\n`existing_text` parameter is useful for implementing your own [`LineEdit`][crate::classes::LineEdit] or [`TextEdit`][crate::classes::TextEdit], as it tells the virtual keyboard what text has already been typed (the virtual keyboard uses it for auto-correct and predictions).\n\n`position` parameter is the screen space [`Rect2`][crate::builtin::Rect2] of the edited text.\n\n`type` parameter allows configuring which type of virtual keyboard to show.\n\n`max_length` limits the number of characters that can be entered if different from `-1`.\n\n`cursor_start` can optionally define the current text cursor position if `cursor_end` is not set.\n\n`cursor_start` and `cursor_end` can optionally define the current text selection.\n\n**Note:** This method is implemented on Android, iOS and Web."]
        pub(crate) fn virtual_keyboard_show_full(&mut self, existing_text: CowArg < GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Rect2, crate::classes::display_server::VirtualKeyboardType, i32, i32, i32,);
            let args = (existing_text, position, type_, max_length, cursor_start, cursor_end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`virtual_keyboard_show_ex`][Self::virtual_keyboard_show_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Shows the virtual keyboard if the platform has one.\n\n`existing_text` parameter is useful for implementing your own [`LineEdit`][crate::classes::LineEdit] or [`TextEdit`][crate::classes::TextEdit], as it tells the virtual keyboard what text has already been typed (the virtual keyboard uses it for auto-correct and predictions).\n\n`position` parameter is the screen space [`Rect2`][crate::builtin::Rect2] of the edited text.\n\n`type` parameter allows configuring which type of virtual keyboard to show.\n\n`max_length` limits the number of characters that can be entered if different from `-1`.\n\n`cursor_start` can optionally define the current text cursor position if `cursor_end` is not set.\n\n`cursor_start` and `cursor_end` can optionally define the current text selection.\n\n**Note:** This method is implemented on Android, iOS and Web."]
        #[inline]
        pub fn virtual_keyboard_show(&mut self, existing_text: impl AsArg < GString >,) {
            self.virtual_keyboard_show_ex(existing_text,) . done()
        }
        #[doc = "Shows the virtual keyboard if the platform has one.\n\n`existing_text` parameter is useful for implementing your own [`LineEdit`][crate::classes::LineEdit] or [`TextEdit`][crate::classes::TextEdit], as it tells the virtual keyboard what text has already been typed (the virtual keyboard uses it for auto-correct and predictions).\n\n`position` parameter is the screen space [`Rect2`][crate::builtin::Rect2] of the edited text.\n\n`type` parameter allows configuring which type of virtual keyboard to show.\n\n`max_length` limits the number of characters that can be entered if different from `-1`.\n\n`cursor_start` can optionally define the current text cursor position if `cursor_end` is not set.\n\n`cursor_start` and `cursor_end` can optionally define the current text selection.\n\n**Note:** This method is implemented on Android, iOS and Web."]
        #[inline]
        pub fn virtual_keyboard_show_ex < 'ex > (&'ex mut self, existing_text: impl AsArg < GString > + 'ex,) -> ExVirtualKeyboardShow < 'ex > {
            ExVirtualKeyboardShow::new(self, existing_text,)
        }
        #[doc = "Hides the virtual keyboard if it is shown, does nothing otherwise."]
        pub fn virtual_keyboard_hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_hide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the on-screen keyboard's height in pixels. Returns `0` if there is no keyboard or if it is currently hidden.\n\n**Note:** On Android 7 and 8, the keyboard height may return `0` the first time the keyboard is opened in non-immersive mode. This behavior does not occur in immersive mode."]
        pub fn virtual_keyboard_get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "virtual_keyboard_get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a hardware keyboard is connected.\n\n**Note:** This method is implemented on Android and iOS. On other platforms, this method always returns `true`."]
        pub fn has_hardware_keyboard(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_hardware_keyboard", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback that should be called when a hardware keyboard is connected or disconnected. `callable` should accept a single `bool` argument indicating whether the keyboard has been connected (`true`) or disconnected (`false`).\n\n**Note:** This method is only implemented on Android."]
        pub fn set_hardware_keyboard_connection_change_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_hardware_keyboard_connection_change_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default mouse cursor shape. The cursor's appearance will vary depending on the user's operating system and mouse cursor theme. See also [`cursor_get_shape`][`crate::classes::DisplayServer::cursor_get_shape`] and [`cursor_set_custom_image`][`crate::classes::DisplayServer::cursor_set_custom_image`]."]
        pub fn cursor_set_shape(&mut self, shape: crate::classes::display_server::CursorShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::CursorShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default mouse cursor shape set by [`cursor_set_shape`][`crate::classes::DisplayServer::cursor_set_shape`]."]
        pub fn cursor_get_shape(&self,) -> crate::classes::display_server::CursorShape {
            type CallRet = crate::classes::display_server::CursorShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom mouse cursor image for the given `shape`. This means the user's operating system and mouse cursor theme will no longer influence the mouse cursor's appearance.\n\n`cursor` can be either a [`Texture2D`][crate::classes::Texture2D] or an [`Image`][crate::classes::Image], and it should not be larger than 256×256 to display correctly. Optionally, `hotspot` can be set to offset the image's position relative to the click point. By default, `hotspot` is set to the top-left corner of the image. See also [`cursor_set_shape`][`crate::classes::DisplayServer::cursor_set_shape`].\n\n**Note:** On Web, calling this method every frame can cause the cursor to flicker."]
        pub(crate) fn cursor_set_custom_image_full(&mut self, cursor: CowArg < Option < Gd < crate::classes::Resource > > >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, crate::classes::display_server::CursorShape, Vector2,);
            let args = (cursor, shape, hotspot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "cursor_set_custom_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`cursor_set_custom_image_ex`][Self::cursor_set_custom_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a custom mouse cursor image for the given `shape`. This means the user's operating system and mouse cursor theme will no longer influence the mouse cursor's appearance.\n\n`cursor` can be either a [`Texture2D`][crate::classes::Texture2D] or an [`Image`][crate::classes::Image], and it should not be larger than 256×256 to display correctly. Optionally, `hotspot` can be set to offset the image's position relative to the click point. By default, `hotspot` is set to the top-left corner of the image. See also [`cursor_set_shape`][`crate::classes::DisplayServer::cursor_set_shape`].\n\n**Note:** On Web, calling this method every frame can cause the cursor to flicker."]
        #[inline]
        pub fn cursor_set_custom_image(&mut self, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            self.cursor_set_custom_image_ex(cursor,) . done()
        }
        #[doc = "Sets a custom mouse cursor image for the given `shape`. This means the user's operating system and mouse cursor theme will no longer influence the mouse cursor's appearance.\n\n`cursor` can be either a [`Texture2D`][crate::classes::Texture2D] or an [`Image`][crate::classes::Image], and it should not be larger than 256×256 to display correctly. Optionally, `hotspot` can be set to offset the image's position relative to the click point. By default, `hotspot` is set to the top-left corner of the image. See also [`cursor_set_shape`][`crate::classes::DisplayServer::cursor_set_shape`].\n\n**Note:** On Web, calling this method every frame can cause the cursor to flicker."]
        #[inline]
        pub fn cursor_set_custom_image_ex < 'ex > (&'ex mut self, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'ex,) -> ExCursorSetCustomImage < 'ex > {
            ExCursorSetCustomImage::new(self, cursor,)
        }
        #[doc = "Returns `true` if positions of **OK** and **Cancel** buttons are swapped in dialogs. This is enabled by default on Windows to follow interface conventions, and be toggled by changing \\[member ProjectSettings.gui/common/swap_cancel_ok].\n\n**Note:** This doesn't affect native dialogs such as the ones spawned by [`dialog_show`][`crate::classes::DisplayServer::dialog_show`]."]
        pub fn get_swap_cancel_ok(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "get_swap_cancel_ok", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Allows the `process_id` PID to steal focus from this window. In other words, this disables the operating system's focus stealing protection for the specified PID.\n\n**Note:** This method is implemented only on Windows."]
        pub fn enable_for_stealing_focus(&mut self, process_id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (process_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "enable_for_stealing_focus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows a text dialog which uses the operating system's native look-and-feel. `callback` should accept a single `int` parameter which corresponds to the index of the pressed button.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG`][`crate::classes::display_server::Feature::NATIVE_DIALOG`] feature. Supported platforms include macOS, Windows, and Android."]
        pub fn dialog_show(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, buttons: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, RefArg < 'a3, Callable >,);
            let args = (title.into_arg(), description.into_arg(), RefArg::new(buttons), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows a text input dialog which uses the operating system's native look-and-feel. `callback` should accept a single [`String`][crate::builtin::GString] parameter which contains the text field's contents.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_INPUT`][`crate::classes::display_server::Feature::NATIVE_DIALOG_INPUT`] feature. Supported platforms include macOS, Windows, and Android."]
        pub fn dialog_input_text(&mut self, title: impl AsArg < GString >, description: impl AsArg < GString >, existing_text: impl AsArg < GString >, callback: &Callable,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, RefArg < 'a3, Callable >,);
            let args = (title.into_arg(), description.into_arg(), existing_text.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "dialog_input_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Displays OS native dialog for selecting files or directories in the file system.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int`. **On Android,** the third callback argument (`selected_filter_index`) is always `0`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE`] feature. Supported platforms include Linux (X11/Wayland), Windows, macOS, and Android (API level 29+).\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Android and Linux, `show_hidden` is ignored.\n\n**Note:** On Android and macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks.\n\n**Note:** On Android, this method uses the Android Storage Access Framework (SAF).\n\nThe file picker returns a URI instead of a filesystem path. This URI can be passed directly to [`FileAccess`][crate::classes::FileAccess] to perform read/write operations.\n\nWhen using [`FileDialogMode::OPEN_DIR`][`crate::classes::display_server::FileDialogMode::OPEN_DIR`], it returns a tree URI that grants full access to the selected directory. File operations inside this directory can be performed by passing a path on the form `treeUri#relative/path/to/file` to [`FileAccess`][crate::classes::FileAccess].\n\nTo avoid opening the file picker again after each app restart, you can take persistable URI permission as follows:\n\n\n```gdscript\nval uri = \"content://com.android...\" # URI of the selected file or folder.\nval persist = true # Set to false to release the persistable permission.\nvar android_runtime = Engine.get_singleton(\"AndroidRuntime\")\nandroid_runtime.updatePersistableUriPermission(uri, persist)\n```\n\n\nThe persistable URI permission remains valid across app restarts as long as the directory is not moved, renamed, or deleted."]
        pub(crate) fn file_dialog_show_full(&mut self, title: CowArg < GString >, current_directory: CowArg < GString >, filename: CowArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: RefArg < PackedStringArray >, callback: RefArg < Callable >, parent_window_id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a3, PackedStringArray >, RefArg < 'a4, Callable >, i32,);
            let args = (title, current_directory, filename, show_hidden, mode, filters, callback, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`file_dialog_show_ex`][Self::file_dialog_show_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Displays OS native dialog for selecting files or directories in the file system.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int`. **On Android,** the third callback argument (`selected_filter_index`) is always `0`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE`] feature. Supported platforms include Linux (X11/Wayland), Windows, macOS, and Android (API level 29+).\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Android and Linux, `show_hidden` is ignored.\n\n**Note:** On Android and macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks.\n\n**Note:** On Android, this method uses the Android Storage Access Framework (SAF).\n\nThe file picker returns a URI instead of a filesystem path. This URI can be passed directly to [`FileAccess`][crate::classes::FileAccess] to perform read/write operations.\n\nWhen using [`FileDialogMode::OPEN_DIR`][`crate::classes::display_server::FileDialogMode::OPEN_DIR`], it returns a tree URI that grants full access to the selected directory. File operations inside this directory can be performed by passing a path on the form `treeUri#relative/path/to/file` to [`FileAccess`][crate::classes::FileAccess].\n\nTo avoid opening the file picker again after each app restart, you can take persistable URI permission as follows:\n\n\n```gdscript\nval uri = \"content://com.android...\" # URI of the selected file or folder.\nval persist = true # Set to false to release the persistable permission.\nvar android_runtime = Engine.get_singleton(\"AndroidRuntime\")\nandroid_runtime.updatePersistableUriPermission(uri, persist)\n```\n\n\nThe persistable URI permission remains valid across app restarts as long as the directory is not moved, renamed, or deleted."]
        #[inline]
        pub fn file_dialog_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, callback: &Callable,) -> crate::global::Error {
            self.file_dialog_show_ex(title, current_directory, filename, show_hidden, mode, filters, callback,) . done()
        }
        #[doc = "Displays OS native dialog for selecting files or directories in the file system.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int`. **On Android,** the third callback argument (`selected_filter_index`) is always `0`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE`] feature. Supported platforms include Linux (X11/Wayland), Windows, macOS, and Android (API level 29+).\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Android and Linux, `show_hidden` is ignored.\n\n**Note:** On Android and macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks.\n\n**Note:** On Android, this method uses the Android Storage Access Framework (SAF).\n\nThe file picker returns a URI instead of a filesystem path. This URI can be passed directly to [`FileAccess`][crate::classes::FileAccess] to perform read/write operations.\n\nWhen using [`FileDialogMode::OPEN_DIR`][`crate::classes::display_server::FileDialogMode::OPEN_DIR`], it returns a tree URI that grants full access to the selected directory. File operations inside this directory can be performed by passing a path on the form `treeUri#relative/path/to/file` to [`FileAccess`][crate::classes::FileAccess].\n\nTo avoid opening the file picker again after each app restart, you can take persistable URI permission as follows:\n\n\n```gdscript\nval uri = \"content://com.android...\" # URI of the selected file or folder.\nval persist = true # Set to false to release the persistable permission.\nvar android_runtime = Engine.get_singleton(\"AndroidRuntime\")\nandroid_runtime.updatePersistableUriPermission(uri, persist)\n```\n\n\nThe persistable URI permission remains valid across app restarts as long as the directory is not moved, renamed, or deleted."]
        #[inline]
        pub fn file_dialog_show_ex < 'ex > (&'ex mut self, title: impl AsArg < GString > + 'ex, current_directory: impl AsArg < GString > + 'ex, filename: impl AsArg < GString > + 'ex, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'ex PackedStringArray, callback: &'ex Callable,) -> ExFileDialogShow < 'ex > {
            ExFileDialogShow::new(self, title, current_directory, filename, show_hidden, mode, filters, callback,)
        }
        #[doc = "Displays OS native dialog for selecting files or directories in the file system with additional user selectable options.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\n`options` is array of [`Dictionary`][crate::builtin::Dictionary]s with the following keys:\n\n- `\"name\"` - option's name [`String`][crate::builtin::GString].\n\n- `\"values\"` - [`PackedStringArray`][crate::builtin::PackedStringArray] of values. If empty, boolean option (check box) is used.\n\n- `\"default\"` - default selected option index (`int`) or default boolean value (`bool`).\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int, selected_option: Dictionary`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE_EXTRA`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE_EXTRA`] feature. Supported platforms include Linux (X11/Wayland), Windows, and macOS.\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Linux (X11), `show_hidden` is ignored.\n\n**Note:** On macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks."]
        pub(crate) fn file_dialog_with_options_show_full(&mut self, title: CowArg < GString >, current_directory: CowArg < GString >, root: CowArg < GString >, filename: CowArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: RefArg < PackedStringArray >, options: RefArg < Array < AnyDictionary > >, callback: RefArg < Callable >, parent_window_id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, 'a6, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, bool, crate::classes::display_server::FileDialogMode, RefArg < 'a4, PackedStringArray >, RefArg < 'a5, Array < AnyDictionary > >, RefArg < 'a6, Callable >, i32,);
            let args = (title, current_directory, root, filename, show_hidden, mode, filters, options, callback, parent_window_id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "file_dialog_with_options_show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`file_dialog_with_options_show_ex`][Self::file_dialog_with_options_show_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Displays OS native dialog for selecting files or directories in the file system with additional user selectable options.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\n`options` is array of [`Dictionary`][crate::builtin::Dictionary]s with the following keys:\n\n- `\"name\"` - option's name [`String`][crate::builtin::GString].\n\n- `\"values\"` - [`PackedStringArray`][crate::builtin::PackedStringArray] of values. If empty, boolean option (check box) is used.\n\n- `\"default\"` - default selected option index (`int`) or default boolean value (`bool`).\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int, selected_option: Dictionary`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE_EXTRA`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE_EXTRA`] feature. Supported platforms include Linux (X11/Wayland), Windows, and macOS.\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Linux (X11), `show_hidden` is ignored.\n\n**Note:** On macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks."]
        #[inline]
        pub fn file_dialog_with_options_show(&mut self, title: impl AsArg < GString >, current_directory: impl AsArg < GString >, root: impl AsArg < GString >, filename: impl AsArg < GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &PackedStringArray, options: &Array < AnyDictionary >, callback: &Callable,) -> crate::global::Error {
            self.file_dialog_with_options_show_ex(title, current_directory, root, filename, show_hidden, mode, filters, options, callback,) . done()
        }
        #[doc = "Displays OS native dialog for selecting files or directories in the file system with additional user selectable options.\n\nEach filter string in the `filters` array should be formatted like this: `*.png,*.jpg,*.jpeg;Image Files;image/png,image/jpeg`. The description text of the filter is optional and can be omitted. It is recommended to set both file extension and MIME type. See also \\[member FileDialog.filters].\n\n`options` is array of [`Dictionary`][crate::builtin::Dictionary]s with the following keys:\n\n- `\"name\"` - option's name [`String`][crate::builtin::GString].\n\n- `\"values\"` - [`PackedStringArray`][crate::builtin::PackedStringArray] of values. If empty, boolean option (check box) is used.\n\n- `\"default\"` - default selected option index (`int`) or default boolean value (`bool`).\n\nCallbacks have the following arguments: `status: bool, selected_paths: PackedStringArray, selected_filter_index: int, selected_option: Dictionary`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_DIALOG_FILE_EXTRA`][`crate::classes::display_server::Feature::NATIVE_DIALOG_FILE_EXTRA`] feature. Supported platforms include Linux (X11/Wayland), Windows, and macOS.\n\n**Note:** `current_directory` might be ignored.\n\n**Note:** Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.\n\n**Note:** On Linux (X11), `show_hidden` is ignored.\n\n**Note:** On macOS, native file dialogs have no title.\n\n**Note:** On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use [`get_granted_permissions`][`crate::classes::Os::get_granted_permissions`] to get a list of saved bookmarks."]
        #[inline]
        pub fn file_dialog_with_options_show_ex < 'ex > (&'ex mut self, title: impl AsArg < GString > + 'ex, current_directory: impl AsArg < GString > + 'ex, root: impl AsArg < GString > + 'ex, filename: impl AsArg < GString > + 'ex, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'ex PackedStringArray, options: &'ex Array < AnyDictionary >, callback: &'ex Callable,) -> ExFileDialogWithOptionsShow < 'ex > {
            ExFileDialogWithOptionsShow::new(self, title, current_directory, root, filename, show_hidden, mode, filters, options, callback,)
        }
        #[doc = "Plays the beep sound from the operative system, if possible. Because it comes from the OS, the beep sound will be audible even if the application is muted. It may also be disabled for the entire OS by the user.\n\n**Note:** This method is implemented on macOS, Linux (X11/Wayland), and Windows."]
        pub fn beep(&self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "beep", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of keyboard layouts.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_get_layout_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns active keyboard layout index.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS, and Windows."]
        pub fn keyboard_get_current_layout(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_current_layout", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the active keyboard layout.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_set_current_layout(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_set_current_layout", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ISO-639/BCP-47 language code of the keyboard layout at position `index`.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_get_layout_language(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the localized name of the keyboard layout at position `index`.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_get_layout_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_layout_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a physical (US QWERTY) `keycode` to one in the active keyboard layout.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_get_keycode_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_keycode_from_physical", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a physical (US QWERTY) `keycode` to localized label printed on the key in the active keyboard layout.\n\n**Note:** This method is implemented on Linux (X11/Wayland), macOS and Windows."]
        pub fn keyboard_get_label_from_physical(&self, keycode: crate::global::Key,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "keyboard_get_label_from_physical", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Opens system emoji and symbol picker.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn show_emoji_and_symbol_picker(&self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "show_emoji_and_symbol_picker", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Displays OS native color picker.\n\nCallbacks have the following arguments: `status: bool, color: Color`.\n\n**Note:** This method is implemented if the display server has the [`Feature::NATIVE_COLOR_PICKER`][`crate::classes::display_server::Feature::NATIVE_COLOR_PICKER`] feature.\n\n**Note:** This method is only implemented on Linux (X11/Wayland)."]
        pub fn color_picker(&mut self, callback: &Callable,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "color_picker", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Perform window manager processing, including input flushing. See also [`force_process_and_drop_events`][`crate::classes::DisplayServer::force_process_and_drop_events`], [`flush_buffered_events`][`crate::classes::Input::flush_buffered_events`] and \\[member Input.use_accumulated_input]."]
        pub fn process_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "process_events", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces window manager processing while ignoring all [`InputEvent`][crate::classes::InputEvent]s. See also [`process_events`][`crate::classes::DisplayServer::process_events`].\n\n**Note:** This method is implemented on Windows and macOS."]
        pub fn force_process_and_drop_events(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "force_process_and_drop_events", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the window icon (usually displayed in the top-left corner) in the operating system's _native_ format. The file at `filename` must be in `.ico` format on Windows or `.icns` on macOS. By using specially crafted `.ico` or `.icns` icons, [`set_native_icon`][`crate::classes::DisplayServer::set_native_icon`] allows specifying different icons depending on the size the icon is displayed at. This size is determined by the operating system and user preferences (including the display scale factor). To use icons in other formats, use [`set_icon`][`crate::classes::DisplayServer::set_icon`] instead.\n\n**Note:** Requires support for [`Feature::NATIVE_ICON`][`crate::classes::display_server::Feature::NATIVE_ICON`]."]
        pub fn set_native_icon(&mut self, filename: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_native_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the window icon (usually displayed in the top-left corner) with an [`Image`][crate::classes::Image]. To use icons in the operating system's native format, use [`set_native_icon`][`crate::classes::DisplayServer::set_native_icon`] instead.\n\n**Note:** Requires support for [`Feature::ICON`][`crate::classes::display_server::Feature::ICON`]."]
        pub fn set_icon(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (image.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new application status indicator with the specified icon, tooltip, and activation callback.\n\n`callback` should take two arguments: the pressed mouse button (one of the \\[enum MouseButton] constants) and the click position in screen coordinates (a [`Vector2i`][crate::builtin::Vector2i])."]
        pub fn create_status_indicator(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, tooltip: impl AsArg < GString >, callback: &Callable,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >,);
            let args = (icon.into_arg(), tooltip.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "create_status_indicator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the application status indicator icon.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn status_indicator_set_icon(&mut self, id: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (id, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the application status indicator tooltip.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn status_indicator_set_tooltip(&mut self, id: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the application status indicator native popup menu.\n\n**Note:** On macOS, the menu is activated by any mouse button. Its activation callback is _not_ triggered.\n\n**Note:** On Windows, the menu is activated by the right mouse button, selecting the status icon and pressing `Shift + F10`, or the applications key. The menu's activation callback for the other mouse buttons is still triggered.\n\n**Note:** Native popup is only supported if [`NativeMenu`][crate::classes::NativeMenu] supports the [`Feature::POPUP_MENU`][`crate::classes::native_menu::Feature::POPUP_MENU`] feature."]
        pub fn status_indicator_set_menu(&mut self, id: i32, menu_rid: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (id, menu_rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the application status indicator activation callback. `callback` should take two arguments: `int` mouse button index (one of \\[enum MouseButton] values) and [`Vector2i`][crate::builtin::Vector2i] click position in screen coordinates.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn status_indicator_set_callback(&mut self, id: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (id, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_set_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the rectangle for the given status indicator `id` in screen coordinates. If the status indicator is not visible, returns an empty [`Rect2`][crate::builtin::Rect2].\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn status_indicator_get_rect(&self, id: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "status_indicator_get_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the application status indicator."]
        pub fn delete_status_indicator(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "delete_status_indicator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total number of available tablet drivers.\n\n**Note:** This method is implemented only on Windows."]
        pub fn tablet_get_driver_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tablet driver name for the given index.\n\n**Note:** This method is implemented only on Windows."]
        pub fn tablet_get_driver_name(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_driver_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns current active tablet driver name.\n\n**Note:** This method is implemented only on Windows."]
        pub fn tablet_get_current_driver(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_get_current_driver", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set active tablet driver name.\n\nSupported drivers:\n\n- `winink`: Windows Ink API, default.\n\n- `wintab`: Wacom Wintab API (compatible device driver required).\n\n- `dummy`: Dummy driver, tablet input is disabled.\n\n**Note:** This method is implemented only on Windows."]
        pub fn tablet_set_current_driver(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "tablet_set_current_driver", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the window background can be made transparent. This method returns `false` if \\[member ProjectSettings.display/window/per_pixel_transparency/allowed] is set to `false`, or if transparency is not supported by the renderer or OS compositor."]
        pub fn is_window_transparency_available(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "is_window_transparency_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers an [`Object`][crate::classes::Object] which represents an additional output that will be rendered too, beyond normal windows. The [`Object`][crate::classes::Object] is only used as an identifier, which can be later passed to [`unregister_additional_output`][`crate::classes::DisplayServer::unregister_additional_output`].\n\nThis can be used to prevent Godot from skipping rendering when no normal windows are visible."]
        pub fn register_additional_output(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "register_additional_output", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unregisters an [`Object`][crate::classes::Object] representing an additional output, that was registered via [`register_additional_output`][`crate::classes::DisplayServer::register_additional_output`]."]
        pub fn unregister_additional_output(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "unregister_additional_output", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if any additional outputs have been registered via [`register_additional_output`][`crate::classes::DisplayServer::register_additional_output`]."]
        pub fn has_additional_outputs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DisplayServer", "has_additional_outputs", Some(self.__validated_obj()), args,)
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
        pub const INVALID_SCREEN: i32 = - 1i32;
        pub const SCREEN_WITH_MOUSE_FOCUS: i32 = - 4i32;
        pub const SCREEN_WITH_KEYBOARD_FOCUS: i32 = - 3i32;
        pub const SCREEN_PRIMARY: i32 = - 2i32;
        pub const SCREEN_OF_MAIN_WINDOW: i32 = - 1i32;
        pub const MAIN_WINDOW_ID: i32 = 0i32;
        pub const INVALID_WINDOW_ID: i32 = - 1i32;
        pub const INVALID_INDICATOR_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for DisplayServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DisplayServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for DisplayServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DisplayServer {
        
    }
    impl crate::obj::Singleton for DisplayServer {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"DisplayServer"))
            }
        }
    }
    impl std::ops::Deref for DisplayServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DisplayServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DisplayServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `DisplayServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_submenu_item_ex`][super::DisplayServer::global_menu_add_submenu_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSubmenuItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, label: CowArg < 'ex, GString >, submenu: CowArg < 'ex, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddSubmenuItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex, submenu: impl AsArg < GString > + 'ex,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), submenu: submenu.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, submenu, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_submenu_item_full(surround_object, menu_root, label, submenu, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_item_ex`][super::DisplayServer::global_menu_add_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_check_item_ex`][super::DisplayServer::global_menu_add_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_item_ex`][super::DisplayServer::global_menu_add_icon_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddIconItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_check_item_ex`][super::DisplayServer::global_menu_add_icon_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddIconCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_check_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_radio_check_item_ex`][super::DisplayServer::global_menu_add_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_radio_check_item_full(surround_object, menu_root, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_icon_radio_check_item_ex`][super::DisplayServer::global_menu_add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddIconRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddIconRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_icon_radio_check_item_full(surround_object, menu_root, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_multistate_item_ex`][super::DisplayServer::global_menu_add_multistate_item_ex]."]
#[must_use]
pub struct ExGlobalMenuAddMultistateItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, label: CowArg < 'ex, GString >, max_states: i32, default_state: i32, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddMultistateItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex, label: impl AsArg < GString > + 'ex, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
        }
    }
    #[inline]
    pub fn callback(self, callback: &'ex Callable) -> Self {
        Self {
            callback: CowArg::Borrowed(callback), .. self
        }
    }
    #[inline]
    pub fn key_callback(self, key_callback: &'ex Callable) -> Self {
        Self {
            key_callback: CowArg::Borrowed(key_callback), .. self
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
        }
    }
    #[inline]
    pub fn accelerator(self, accelerator: crate::global::Key) -> Self {
        Self {
            accelerator: accelerator, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_multistate_item_full(surround_object, menu_root, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::global_menu_add_separator_ex`][super::DisplayServer::global_menu_add_separator_ex]."]
#[must_use]
pub struct ExGlobalMenuAddSeparator < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, menu_root: CowArg < 'ex, GString >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGlobalMenuAddSeparator < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, menu_root: impl AsArg < GString > + 'ex,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu_root: menu_root.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, menu_root, index,
        }
        = self;
        re_export::DisplayServer::global_menu_add_separator_full(surround_object, menu_root, index,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::tts_speak_ex`][super::DisplayServer::tts_speak_ex]."]
#[must_use]
pub struct ExTtsSpeak < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, text: CowArg < 'ex, GString >, voice: CowArg < 'ex, GString >, volume: i32, pitch: f32, rate: f32, utterance_id: i64, interrupt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTtsSpeak < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, text: impl AsArg < GString > + 'ex, voice: impl AsArg < GString > + 'ex,) -> Self {
        let volume = 50i32;
        let pitch = 1f32;
        let rate = 1f32;
        let utterance_id = 0i64;
        let interrupt = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), voice: voice.into_arg(), volume: volume, pitch: pitch, rate: rate, utterance_id: utterance_id, interrupt: interrupt,
        }
    }
    #[inline]
    pub fn volume(self, volume: i32) -> Self {
        Self {
            volume: volume, .. self
        }
    }
    #[inline]
    pub fn pitch(self, pitch: f32) -> Self {
        Self {
            pitch: pitch, .. self
        }
    }
    #[inline]
    pub fn rate(self, rate: f32) -> Self {
        Self {
            rate: rate, .. self
        }
    }
    #[inline]
    pub fn utterance_id(self, utterance_id: i64) -> Self {
        Self {
            utterance_id: utterance_id, .. self
        }
    }
    #[inline]
    pub fn interrupt(self, interrupt: bool) -> Self {
        Self {
            interrupt: interrupt, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,
        }
        = self;
        re_export::DisplayServer::tts_speak_full(surround_object, text, voice, volume, pitch, rate, utterance_id, interrupt,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_position_ex`][super::DisplayServer::screen_get_position_ex]."]
#[must_use]
pub struct ExScreenGetPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetPosition < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_position_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_size_ex`][super::DisplayServer::screen_get_size_ex]."]
#[must_use]
pub struct ExScreenGetSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetSize < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_size_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_usable_rect_ex`][super::DisplayServer::screen_get_usable_rect_ex]."]
#[must_use]
pub struct ExScreenGetUsableRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetUsableRect < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2i {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_usable_rect_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_dpi_ex`][super::DisplayServer::screen_get_dpi_ex]."]
#[must_use]
pub struct ExScreenGetDpi < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetDpi < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_dpi_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_scale_ex`][super::DisplayServer::screen_get_scale_ex]."]
#[must_use]
pub struct ExScreenGetScale < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetScale < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_scale_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_refresh_rate_ex`][super::DisplayServer::screen_get_refresh_rate_ex]."]
#[must_use]
pub struct ExScreenGetRefreshRate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetRefreshRate < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_refresh_rate_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_image_ex`][super::DisplayServer::screen_get_image_ex]."]
#[must_use]
pub struct ExScreenGetImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetImage < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_image_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_set_orientation_ex`][super::DisplayServer::screen_set_orientation_ex]."]
#[must_use]
pub struct ExScreenSetOrientation < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenSetOrientation < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, orientation: crate::classes::display_server::ScreenOrientation,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, orientation: orientation, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, orientation, screen,
        }
        = self;
        re_export::DisplayServer::screen_set_orientation_full(surround_object, orientation, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::screen_get_orientation_ex`][super::DisplayServer::screen_get_orientation_ex]."]
#[must_use]
pub struct ExScreenGetOrientation < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExScreenGetOrientation < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let screen = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::ScreenOrientation {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::DisplayServer::screen_get_orientation_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_native_handle_ex`][super::DisplayServer::window_get_native_handle_ex]."]
#[must_use]
pub struct ExWindowGetNativeHandle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetNativeHandle < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer, handle_type: crate::classes::display_server::HandleType,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, handle_type: handle_type, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, handle_type, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_native_handle_full(surround_object, handle_type, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_title_ex`][super::DisplayServer::window_set_title_ex]."]
#[must_use]
pub struct ExWindowSetTitle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, title: CowArg < 'ex, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetTitle < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, title: impl AsArg < GString > + 'ex,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_title_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_title_size_ex`][super::DisplayServer::window_get_title_size_ex]."]
#[must_use]
pub struct ExWindowGetTitleSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, title: CowArg < 'ex, GString >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetTitleSize < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer, title: impl AsArg < GString > + 'ex,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, title, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_title_size_full(surround_object, title, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mouse_passthrough_ex`][super::DisplayServer::window_set_mouse_passthrough_ex]."]
#[must_use]
pub struct ExWindowSetMousePassthrough < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, region: CowArg < 'ex, PackedVector2Array >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetMousePassthrough < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, region: &'ex PackedVector2Array,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, region: CowArg::Borrowed(region), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, region, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mouse_passthrough_full(surround_object, region.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_current_screen_ex`][super::DisplayServer::window_get_current_screen_ex]."]
#[must_use]
pub struct ExWindowGetCurrentScreen < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetCurrentScreen < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_current_screen_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_current_screen_ex`][super::DisplayServer::window_set_current_screen_ex]."]
#[must_use]
pub struct ExWindowSetCurrentScreen < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, screen: i32, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetCurrentScreen < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, screen: i32,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, screen, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_current_screen_full(surround_object, screen, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_ex`][super::DisplayServer::window_get_position_ex]."]
#[must_use]
pub struct ExWindowGetPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetPosition < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_position_with_decorations_ex`][super::DisplayServer::window_get_position_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetPositionWithDecorations < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetPositionWithDecorations < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_position_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_position_ex`][super::DisplayServer::window_set_position_ex]."]
#[must_use]
pub struct ExWindowSetPosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetPosition < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_ex`][super::DisplayServer::window_get_size_ex]."]
#[must_use]
pub struct ExWindowGetSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetSize < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_size_ex`][super::DisplayServer::window_set_size_ex]."]
#[must_use]
pub struct ExWindowSetSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetSize < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_size_full(surround_object, size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_rect_changed_callback_ex`][super::DisplayServer::window_set_rect_changed_callback_ex]."]
#[must_use]
pub struct ExWindowSetRectChangedCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, callback: CowArg < 'ex, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetRectChangedCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, callback: &'ex Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_rect_changed_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_event_callback_ex`][super::DisplayServer::window_set_window_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetWindowEventCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, callback: CowArg < 'ex, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetWindowEventCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, callback: &'ex Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_event_callback_ex`][super::DisplayServer::window_set_input_event_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputEventCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, callback: CowArg < 'ex, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetInputEventCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, callback: &'ex Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_event_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_input_text_callback_ex`][super::DisplayServer::window_set_input_text_callback_ex]."]
#[must_use]
pub struct ExWindowSetInputTextCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, callback: CowArg < 'ex, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetInputTextCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, callback: &'ex Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_input_text_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_drop_files_callback_ex`][super::DisplayServer::window_set_drop_files_callback_ex]."]
#[must_use]
pub struct ExWindowSetDropFilesCallback < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, callback: CowArg < 'ex, Callable >, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetDropFilesCallback < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, callback: &'ex Callable,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_drop_files_callback_full(surround_object, callback.cow_as_arg(), window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_attached_instance_id_ex`][super::DisplayServer::window_get_attached_instance_id_ex]."]
#[must_use]
pub struct ExWindowGetAttachedInstanceId < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetAttachedInstanceId < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_attached_instance_id_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_max_size_ex`][super::DisplayServer::window_get_max_size_ex]."]
#[must_use]
pub struct ExWindowGetMaxSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetMaxSize < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_max_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_max_size_ex`][super::DisplayServer::window_set_max_size_ex]."]
#[must_use]
pub struct ExWindowSetMaxSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, max_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetMaxSize < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, max_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_size: max_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, max_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_max_size_full(surround_object, max_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_min_size_ex`][super::DisplayServer::window_get_min_size_ex]."]
#[must_use]
pub struct ExWindowGetMinSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetMinSize < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_min_size_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_min_size_ex`][super::DisplayServer::window_set_min_size_ex]."]
#[must_use]
pub struct ExWindowSetMinSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, min_size: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetMinSize < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, min_size: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, min_size: min_size, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, min_size, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_min_size_full(surround_object, min_size, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_size_with_decorations_ex`][super::DisplayServer::window_get_size_with_decorations_ex]."]
#[must_use]
pub struct ExWindowGetSizeWithDecorations < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetSizeWithDecorations < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_size_with_decorations_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_mode_ex`][super::DisplayServer::window_get_mode_ex]."]
#[must_use]
pub struct ExWindowGetMode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetMode < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::WindowMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_mode_ex`][super::DisplayServer::window_set_mode_ex]."]
#[must_use]
pub struct ExWindowSetMode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetMode < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, mode: crate::classes::display_server::WindowMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_mode_full(surround_object, mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_flag_ex`][super::DisplayServer::window_set_flag_ex]."]
#[must_use]
pub struct ExWindowSetFlag < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetFlag < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, enabled: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, enabled: enabled, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, flag, enabled, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_flag_full(surround_object, flag, enabled, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_flag_ex`][super::DisplayServer::window_get_flag_ex]."]
#[must_use]
pub struct ExWindowGetFlag < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetFlag < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer, flag: crate::classes::display_server::WindowFlags,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flag: flag, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, flag, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_flag_full(surround_object, flag, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_window_buttons_offset_ex`][super::DisplayServer::window_set_window_buttons_offset_ex]."]
#[must_use]
pub struct ExWindowSetWindowButtonsOffset < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, offset: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetWindowButtonsOffset < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, offset: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, offset: offset, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, offset, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_window_buttons_offset_full(surround_object, offset, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_safe_title_margins_ex`][super::DisplayServer::window_get_safe_title_margins_ex]."]
#[must_use]
pub struct ExWindowGetSafeTitleMargins < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetSafeTitleMargins < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3i {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_safe_title_margins_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_request_attention_ex`][super::DisplayServer::window_request_attention_ex]."]
#[must_use]
pub struct ExWindowRequestAttention < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowRequestAttention < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_request_attention_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_move_to_foreground_ex`][super::DisplayServer::window_move_to_foreground_ex]."]
#[must_use]
pub struct ExWindowMoveToForeground < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowMoveToForeground < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_move_to_foreground_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_focused_ex`][super::DisplayServer::window_is_focused_ex]."]
#[must_use]
pub struct ExWindowIsFocused < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowIsFocused < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_focused_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_can_draw_ex`][super::DisplayServer::window_can_draw_ex]."]
#[must_use]
pub struct ExWindowCanDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowCanDraw < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_can_draw_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_active_ex`][super::DisplayServer::window_set_ime_active_ex]."]
#[must_use]
pub struct ExWindowSetImeActive < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, active: bool, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetImeActive < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, active: bool,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, active: active, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, active, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_active_full(surround_object, active, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_ime_position_ex`][super::DisplayServer::window_set_ime_position_ex]."]
#[must_use]
pub struct ExWindowSetImePosition < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, position: Vector2i, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetImePosition < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, position: Vector2i,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_ime_position_full(surround_object, position, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_set_vsync_mode_ex`][super::DisplayServer::window_set_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowSetVSyncMode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowSetVSyncMode < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, vsync_mode: crate::classes::display_server::VSyncMode,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vsync_mode: vsync_mode, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, vsync_mode, window_id,
        }
        = self;
        re_export::DisplayServer::window_set_vsync_mode_full(surround_object, vsync_mode, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_get_vsync_mode_ex`][super::DisplayServer::window_get_vsync_mode_ex]."]
#[must_use]
pub struct ExWindowGetVSyncMode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowGetVSyncMode < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::display_server::VSyncMode {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_get_vsync_mode_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_is_maximize_allowed_ex`][super::DisplayServer::window_is_maximize_allowed_ex]."]
#[must_use]
pub struct ExWindowIsMaximizeAllowed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowIsMaximizeAllowed < 'ex > {
    fn new(surround_object: &'ex re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_is_maximize_allowed_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_start_drag_ex`][super::DisplayServer::window_start_drag_ex]."]
#[must_use]
pub struct ExWindowStartDrag < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowStartDrag < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, window_id,
        }
        = self;
        re_export::DisplayServer::window_start_drag_full(surround_object, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::window_start_resize_ex`][super::DisplayServer::window_start_resize_ex]."]
#[must_use]
pub struct ExWindowStartResize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, edge: crate::classes::display_server::WindowResizeEdge, window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExWindowStartResize < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, edge: crate::classes::display_server::WindowResizeEdge,) -> Self {
        let window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, edge: edge, window_id: window_id,
        }
    }
    #[inline]
    pub fn window_id(self, window_id: i32) -> Self {
        Self {
            window_id: window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, edge, window_id,
        }
        = self;
        re_export::DisplayServer::window_start_resize_full(surround_object, edge, window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::accessibility_create_sub_element_ex`][super::DisplayServer::accessibility_create_sub_element_ex]."]
#[must_use]
pub struct ExAccessibilityCreateSubElement < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole, insert_pos: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAccessibilityCreateSubElement < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, parent_rid: Rid, role: crate::classes::display_server::AccessibilityRole,) -> Self {
        let insert_pos = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent_rid: parent_rid, role: role, insert_pos: insert_pos,
        }
    }
    #[inline]
    pub fn insert_pos(self, insert_pos: i32) -> Self {
        Self {
            insert_pos: insert_pos, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, parent_rid, role, insert_pos,
        }
        = self;
        re_export::DisplayServer::accessibility_create_sub_element_full(surround_object, parent_rid, role, insert_pos,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::accessibility_create_sub_text_edit_elements_ex`][super::DisplayServer::accessibility_create_sub_text_edit_elements_ex]."]
#[must_use]
pub struct ExAccessibilityCreateSubTextEditElements < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, parent_rid: Rid, shaped_text: Rid, min_height: f32, insert_pos: i32, is_last_line: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAccessibilityCreateSubTextEditElements < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, parent_rid: Rid, shaped_text: Rid, min_height: f32,) -> Self {
        let insert_pos = - 1i32;
        let is_last_line = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent_rid: parent_rid, shaped_text: shaped_text, min_height: min_height, insert_pos: insert_pos, is_last_line: is_last_line,
        }
    }
    #[inline]
    pub fn insert_pos(self, insert_pos: i32) -> Self {
        Self {
            insert_pos: insert_pos, .. self
        }
    }
    #[inline]
    pub fn is_last_line(self, is_last_line: bool) -> Self {
        Self {
            is_last_line: is_last_line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, parent_rid, shaped_text, min_height, insert_pos, is_last_line,
        }
        = self;
        re_export::DisplayServer::accessibility_create_sub_text_edit_elements_full(surround_object, parent_rid, shaped_text, min_height, insert_pos, is_last_line,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::virtual_keyboard_show_ex`][super::DisplayServer::virtual_keyboard_show_ex]."]
#[must_use]
pub struct ExVirtualKeyboardShow < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, existing_text: CowArg < 'ex, GString >, position: Rect2, type_: crate::classes::display_server::VirtualKeyboardType, max_length: i32, cursor_start: i32, cursor_end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExVirtualKeyboardShow < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, existing_text: impl AsArg < GString > + 'ex,) -> Self {
        let position = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let type_ = crate::obj::EngineEnum::from_ord(0);
        let max_length = - 1i32;
        let cursor_start = - 1i32;
        let cursor_end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing_text: existing_text.into_arg(), position: position, type_: type_, max_length: max_length, cursor_start: cursor_start, cursor_end: cursor_end,
        }
    }
    #[inline]
    pub fn position(self, position: Rect2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn type_(self, type_: crate::classes::display_server::VirtualKeyboardType) -> Self {
        Self {
            type_: type_, .. self
        }
    }
    #[inline]
    pub fn max_length(self, max_length: i32) -> Self {
        Self {
            max_length: max_length, .. self
        }
    }
    #[inline]
    pub fn cursor_start(self, cursor_start: i32) -> Self {
        Self {
            cursor_start: cursor_start, .. self
        }
    }
    #[inline]
    pub fn cursor_end(self, cursor_end: i32) -> Self {
        Self {
            cursor_end: cursor_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,
        }
        = self;
        re_export::DisplayServer::virtual_keyboard_show_full(surround_object, existing_text, position, type_, max_length, cursor_start, cursor_end,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::cursor_set_custom_image_ex`][super::DisplayServer::cursor_set_custom_image_ex]."]
#[must_use]
pub struct ExCursorSetCustomImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, cursor: CowArg < 'ex, Option < Gd < crate::classes::Resource > > >, shape: crate::classes::display_server::CursorShape, hotspot: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCursorSetCustomImage < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, cursor: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'ex,) -> Self {
        let shape = crate::obj::EngineEnum::from_ord(0);
        let hotspot = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, cursor: cursor.into_arg(), shape: shape, hotspot: hotspot,
        }
    }
    #[inline]
    pub fn shape(self, shape: crate::classes::display_server::CursorShape) -> Self {
        Self {
            shape: shape, .. self
        }
    }
    #[inline]
    pub fn hotspot(self, hotspot: Vector2) -> Self {
        Self {
            hotspot: hotspot, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, cursor, shape, hotspot,
        }
        = self;
        re_export::DisplayServer::cursor_set_custom_image_full(surround_object, cursor, shape, hotspot,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::file_dialog_show_ex`][super::DisplayServer::file_dialog_show_ex]."]
#[must_use]
pub struct ExFileDialogShow < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, title: CowArg < 'ex, GString >, current_directory: CowArg < 'ex, GString >, filename: CowArg < 'ex, GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: CowArg < 'ex, PackedStringArray >, callback: CowArg < 'ex, Callable >, parent_window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFileDialogShow < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, title: impl AsArg < GString > + 'ex, current_directory: impl AsArg < GString > + 'ex, filename: impl AsArg < GString > + 'ex, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'ex PackedStringArray, callback: &'ex Callable,) -> Self {
        let parent_window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), current_directory: current_directory.into_arg(), filename: filename.into_arg(), show_hidden: show_hidden, mode: mode, filters: CowArg::Borrowed(filters), callback: CowArg::Borrowed(callback), parent_window_id: parent_window_id,
        }
    }
    #[inline]
    pub fn parent_window_id(self, parent_window_id: i32) -> Self {
        Self {
            parent_window_id: parent_window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, title, current_directory, filename, show_hidden, mode, filters, callback, parent_window_id,
        }
        = self;
        re_export::DisplayServer::file_dialog_show_full(surround_object, title, current_directory, filename, show_hidden, mode, filters.cow_as_arg(), callback.cow_as_arg(), parent_window_id,)
    }
}
#[doc = "Default-param extender for [`DisplayServer::file_dialog_with_options_show_ex`][super::DisplayServer::file_dialog_with_options_show_ex]."]
#[must_use]
pub struct ExFileDialogWithOptionsShow < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::DisplayServer, title: CowArg < 'ex, GString >, current_directory: CowArg < 'ex, GString >, root: CowArg < 'ex, GString >, filename: CowArg < 'ex, GString >, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: CowArg < 'ex, PackedStringArray >, options: CowArg < 'ex, Array < AnyDictionary > >, callback: CowArg < 'ex, Callable >, parent_window_id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExFileDialogWithOptionsShow < 'ex > {
    fn new(surround_object: &'ex mut re_export::DisplayServer, title: impl AsArg < GString > + 'ex, current_directory: impl AsArg < GString > + 'ex, root: impl AsArg < GString > + 'ex, filename: impl AsArg < GString > + 'ex, show_hidden: bool, mode: crate::classes::display_server::FileDialogMode, filters: &'ex PackedStringArray, options: &'ex Array < AnyDictionary >, callback: &'ex Callable,) -> Self {
        let parent_window_id = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, title: title.into_arg(), current_directory: current_directory.into_arg(), root: root.into_arg(), filename: filename.into_arg(), show_hidden: show_hidden, mode: mode, filters: CowArg::Borrowed(filters), options: CowArg::Borrowed(options), callback: CowArg::Borrowed(callback), parent_window_id: parent_window_id,
        }
    }
    #[inline]
    pub fn parent_window_id(self, parent_window_id: i32) -> Self {
        Self {
            parent_window_id: parent_window_id, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, title, current_directory, root, filename, show_hidden, mode, filters, options, callback, parent_window_id,
        }
        = self;
        re_export::DisplayServer::file_dialog_with_options_show_full(surround_object, title, current_directory, root, filename, show_hidden, mode, filters.cow_as_arg(), options.cow_as_arg(), callback.cow_as_arg(), parent_window_id,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_GLOBAL_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_GLOBAL_MENU`"]
    pub const GLOBAL_MENU: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_SUBWINDOWS")]
    #[doc = "Godot enumerator name: `FEATURE_SUBWINDOWS`"]
    pub const SUBWINDOWS: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_TOUCHSCREEN")]
    #[doc = "Godot enumerator name: `FEATURE_TOUCHSCREEN`"]
    pub const TOUCHSCREEN: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_MOUSE")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE`"]
    pub const MOUSE: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_MOUSE_WARP")]
    #[doc = "Godot enumerator name: `FEATURE_MOUSE_WARP`"]
    pub const MOUSE_WARP: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD`"]
    pub const CLIPBOARD: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_VIRTUAL_KEYBOARD")]
    #[doc = "Godot enumerator name: `FEATURE_VIRTUAL_KEYBOARD`"]
    pub const VIRTUAL_KEYBOARD: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CURSOR_SHAPE`"]
    pub const CURSOR_SHAPE: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_CUSTOM_CURSOR_SHAPE")]
    #[doc = "Godot enumerator name: `FEATURE_CUSTOM_CURSOR_SHAPE`"]
    pub const CUSTOM_CURSOR_SHAPE: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG`"]
    pub const NATIVE_DIALOG: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_IME")]
    #[doc = "Godot enumerator name: `FEATURE_IME`"]
    pub const IME: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_WINDOW_TRANSPARENCY")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_TRANSPARENCY`"]
    pub const WINDOW_TRANSPARENCY: Feature = Feature {
        ord: 11i32
    };
    #[doc(alias = "FEATURE_HIDPI")]
    #[doc = "Godot enumerator name: `FEATURE_HIDPI`"]
    pub const HIDPI: Feature = Feature {
        ord: 12i32
    };
    #[doc(alias = "FEATURE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_ICON`"]
    pub const ICON: Feature = Feature {
        ord: 13i32
    };
    #[doc(alias = "FEATURE_NATIVE_ICON")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_ICON`"]
    pub const NATIVE_ICON: Feature = Feature {
        ord: 14i32
    };
    #[doc(alias = "FEATURE_ORIENTATION")]
    #[doc = "Godot enumerator name: `FEATURE_ORIENTATION`"]
    pub const ORIENTATION: Feature = Feature {
        ord: 15i32
    };
    #[doc(alias = "FEATURE_SWAP_BUFFERS")]
    #[doc = "Godot enumerator name: `FEATURE_SWAP_BUFFERS`"]
    pub const SWAP_BUFFERS: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_CLIPBOARD_PRIMARY")]
    #[doc = "Godot enumerator name: `FEATURE_CLIPBOARD_PRIMARY`"]
    pub const CLIPBOARD_PRIMARY: Feature = Feature {
        ord: 18i32
    };
    #[doc(alias = "FEATURE_TEXT_TO_SPEECH")]
    #[doc = "Godot enumerator name: `FEATURE_TEXT_TO_SPEECH`"]
    pub const TEXT_TO_SPEECH: Feature = Feature {
        ord: 19i32
    };
    #[doc(alias = "FEATURE_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `FEATURE_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: Feature = Feature {
        ord: 20i32
    };
    #[doc(alias = "FEATURE_SCREEN_CAPTURE")]
    #[doc = "Godot enumerator name: `FEATURE_SCREEN_CAPTURE`"]
    pub const SCREEN_CAPTURE: Feature = Feature {
        ord: 21i32
    };
    #[doc(alias = "FEATURE_STATUS_INDICATOR")]
    #[doc = "Godot enumerator name: `FEATURE_STATUS_INDICATOR`"]
    pub const STATUS_INDICATOR: Feature = Feature {
        ord: 22i32
    };
    #[doc(alias = "FEATURE_NATIVE_HELP")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_HELP`"]
    pub const NATIVE_HELP: Feature = Feature {
        ord: 23i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_INPUT")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_INPUT`"]
    pub const NATIVE_DIALOG_INPUT: Feature = Feature {
        ord: 24i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE`"]
    pub const NATIVE_DIALOG_FILE: Feature = Feature {
        ord: 25i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE_EXTRA")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE_EXTRA`"]
    pub const NATIVE_DIALOG_FILE_EXTRA: Feature = Feature {
        ord: 26i32
    };
    #[doc(alias = "FEATURE_WINDOW_DRAG")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_DRAG`"]
    pub const WINDOW_DRAG: Feature = Feature {
        ord: 27i32
    };
    #[doc(alias = "FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE")]
    #[doc = "Godot enumerator name: `FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE`"]
    pub const SCREEN_EXCLUDE_FROM_CAPTURE: Feature = Feature {
        ord: 28i32
    };
    #[doc(alias = "FEATURE_WINDOW_EMBEDDING")]
    #[doc = "Godot enumerator name: `FEATURE_WINDOW_EMBEDDING`"]
    pub const WINDOW_EMBEDDING: Feature = Feature {
        ord: 29i32
    };
    #[doc(alias = "FEATURE_NATIVE_DIALOG_FILE_MIME")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_DIALOG_FILE_MIME`"]
    pub const NATIVE_DIALOG_FILE_MIME: Feature = Feature {
        ord: 30i32
    };
    #[doc(alias = "FEATURE_EMOJI_AND_SYMBOL_PICKER")]
    #[doc = "Godot enumerator name: `FEATURE_EMOJI_AND_SYMBOL_PICKER`"]
    pub const EMOJI_AND_SYMBOL_PICKER: Feature = Feature {
        ord: 31i32
    };
    #[doc(alias = "FEATURE_NATIVE_COLOR_PICKER")]
    #[doc = "Godot enumerator name: `FEATURE_NATIVE_COLOR_PICKER`"]
    pub const NATIVE_COLOR_PICKER: Feature = Feature {
        ord: 32i32
    };
    #[doc(alias = "FEATURE_SELF_FITTING_WINDOWS")]
    #[doc = "Godot enumerator name: `FEATURE_SELF_FITTING_WINDOWS`"]
    pub const SELF_FITTING_WINDOWS: Feature = Feature {
        ord: 33i32
    };
    #[doc(alias = "FEATURE_ACCESSIBILITY_SCREEN_READER")]
    #[doc = "Godot enumerator name: `FEATURE_ACCESSIBILITY_SCREEN_READER`"]
    pub const ACCESSIBILITY_SCREEN_READER: Feature = Feature {
        ord: 34i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 => Some(Self {
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::SUBWINDOWS => "SUBWINDOWS", Self::TOUCHSCREEN => "TOUCHSCREEN", Self::MOUSE => "MOUSE", Self::MOUSE_WARP => "MOUSE_WARP", Self::CLIPBOARD => "CLIPBOARD", Self::VIRTUAL_KEYBOARD => "VIRTUAL_KEYBOARD", Self::CURSOR_SHAPE => "CURSOR_SHAPE", Self::CUSTOM_CURSOR_SHAPE => "CUSTOM_CURSOR_SHAPE", Self::NATIVE_DIALOG => "NATIVE_DIALOG", Self::IME => "IME", Self::WINDOW_TRANSPARENCY => "WINDOW_TRANSPARENCY", Self::HIDPI => "HIDPI", Self::ICON => "ICON", Self::NATIVE_ICON => "NATIVE_ICON", Self::ORIENTATION => "ORIENTATION", Self::SWAP_BUFFERS => "SWAP_BUFFERS", Self::CLIPBOARD_PRIMARY => "CLIPBOARD_PRIMARY", Self::TEXT_TO_SPEECH => "TEXT_TO_SPEECH", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::SCREEN_CAPTURE => "SCREEN_CAPTURE", Self::STATUS_INDICATOR => "STATUS_INDICATOR", Self::NATIVE_HELP => "NATIVE_HELP", Self::NATIVE_DIALOG_INPUT => "NATIVE_DIALOG_INPUT", Self::NATIVE_DIALOG_FILE => "NATIVE_DIALOG_FILE", Self::NATIVE_DIALOG_FILE_EXTRA => "NATIVE_DIALOG_FILE_EXTRA", Self::WINDOW_DRAG => "WINDOW_DRAG", Self::SCREEN_EXCLUDE_FROM_CAPTURE => "SCREEN_EXCLUDE_FROM_CAPTURE", Self::WINDOW_EMBEDDING => "WINDOW_EMBEDDING", Self::NATIVE_DIALOG_FILE_MIME => "NATIVE_DIALOG_FILE_MIME", Self::EMOJI_AND_SYMBOL_PICKER => "EMOJI_AND_SYMBOL_PICKER", Self::NATIVE_COLOR_PICKER => "NATIVE_COLOR_PICKER", Self::SELF_FITTING_WINDOWS => "SELF_FITTING_WINDOWS", Self::ACCESSIBILITY_SCREEN_READER => "ACCESSIBILITY_SCREEN_READER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::GLOBAL_MENU, Feature::SUBWINDOWS, Feature::TOUCHSCREEN, Feature::MOUSE, Feature::MOUSE_WARP, Feature::CLIPBOARD, Feature::VIRTUAL_KEYBOARD, Feature::CURSOR_SHAPE, Feature::CUSTOM_CURSOR_SHAPE, Feature::NATIVE_DIALOG, Feature::IME, Feature::WINDOW_TRANSPARENCY, Feature::HIDPI, Feature::ICON, Feature::NATIVE_ICON, Feature::ORIENTATION, Feature::SWAP_BUFFERS, Feature::CLIPBOARD_PRIMARY, Feature::TEXT_TO_SPEECH, Feature::EXTEND_TO_TITLE, Feature::SCREEN_CAPTURE, Feature::STATUS_INDICATOR, Feature::NATIVE_HELP, Feature::NATIVE_DIALOG_INPUT, Feature::NATIVE_DIALOG_FILE, Feature::NATIVE_DIALOG_FILE_EXTRA, Feature::WINDOW_DRAG, Feature::SCREEN_EXCLUDE_FROM_CAPTURE, Feature::WINDOW_EMBEDDING, Feature::NATIVE_DIALOG_FILE_MIME, Feature::EMOJI_AND_SYMBOL_PICKER, Feature::NATIVE_COLOR_PICKER, Feature::SELF_FITTING_WINDOWS, Feature::ACCESSIBILITY_SCREEN_READER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLOBAL_MENU", "FEATURE_GLOBAL_MENU", Feature::GLOBAL_MENU), crate::meta::inspect::EnumConstant::new("SUBWINDOWS", "FEATURE_SUBWINDOWS", Feature::SUBWINDOWS), crate::meta::inspect::EnumConstant::new("TOUCHSCREEN", "FEATURE_TOUCHSCREEN", Feature::TOUCHSCREEN), crate::meta::inspect::EnumConstant::new("MOUSE", "FEATURE_MOUSE", Feature::MOUSE), crate::meta::inspect::EnumConstant::new("MOUSE_WARP", "FEATURE_MOUSE_WARP", Feature::MOUSE_WARP), crate::meta::inspect::EnumConstant::new("CLIPBOARD", "FEATURE_CLIPBOARD", Feature::CLIPBOARD), crate::meta::inspect::EnumConstant::new("VIRTUAL_KEYBOARD", "FEATURE_VIRTUAL_KEYBOARD", Feature::VIRTUAL_KEYBOARD), crate::meta::inspect::EnumConstant::new("CURSOR_SHAPE", "FEATURE_CURSOR_SHAPE", Feature::CURSOR_SHAPE), crate::meta::inspect::EnumConstant::new("CUSTOM_CURSOR_SHAPE", "FEATURE_CUSTOM_CURSOR_SHAPE", Feature::CUSTOM_CURSOR_SHAPE), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG", "FEATURE_NATIVE_DIALOG", Feature::NATIVE_DIALOG), crate::meta::inspect::EnumConstant::new("IME", "FEATURE_IME", Feature::IME), crate::meta::inspect::EnumConstant::new("WINDOW_TRANSPARENCY", "FEATURE_WINDOW_TRANSPARENCY", Feature::WINDOW_TRANSPARENCY), crate::meta::inspect::EnumConstant::new("HIDPI", "FEATURE_HIDPI", Feature::HIDPI), crate::meta::inspect::EnumConstant::new("ICON", "FEATURE_ICON", Feature::ICON), crate::meta::inspect::EnumConstant::new("NATIVE_ICON", "FEATURE_NATIVE_ICON", Feature::NATIVE_ICON), crate::meta::inspect::EnumConstant::new("ORIENTATION", "FEATURE_ORIENTATION", Feature::ORIENTATION), crate::meta::inspect::EnumConstant::new("SWAP_BUFFERS", "FEATURE_SWAP_BUFFERS", Feature::SWAP_BUFFERS), crate::meta::inspect::EnumConstant::new("CLIPBOARD_PRIMARY", "FEATURE_CLIPBOARD_PRIMARY", Feature::CLIPBOARD_PRIMARY), crate::meta::inspect::EnumConstant::new("TEXT_TO_SPEECH", "FEATURE_TEXT_TO_SPEECH", Feature::TEXT_TO_SPEECH), crate::meta::inspect::EnumConstant::new("EXTEND_TO_TITLE", "FEATURE_EXTEND_TO_TITLE", Feature::EXTEND_TO_TITLE), crate::meta::inspect::EnumConstant::new("SCREEN_CAPTURE", "FEATURE_SCREEN_CAPTURE", Feature::SCREEN_CAPTURE), crate::meta::inspect::EnumConstant::new("STATUS_INDICATOR", "FEATURE_STATUS_INDICATOR", Feature::STATUS_INDICATOR), crate::meta::inspect::EnumConstant::new("NATIVE_HELP", "FEATURE_NATIVE_HELP", Feature::NATIVE_HELP), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_INPUT", "FEATURE_NATIVE_DIALOG_INPUT", Feature::NATIVE_DIALOG_INPUT), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE", "FEATURE_NATIVE_DIALOG_FILE", Feature::NATIVE_DIALOG_FILE), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE_EXTRA", "FEATURE_NATIVE_DIALOG_FILE_EXTRA", Feature::NATIVE_DIALOG_FILE_EXTRA), crate::meta::inspect::EnumConstant::new("WINDOW_DRAG", "FEATURE_WINDOW_DRAG", Feature::WINDOW_DRAG), crate::meta::inspect::EnumConstant::new("SCREEN_EXCLUDE_FROM_CAPTURE", "FEATURE_SCREEN_EXCLUDE_FROM_CAPTURE", Feature::SCREEN_EXCLUDE_FROM_CAPTURE), crate::meta::inspect::EnumConstant::new("WINDOW_EMBEDDING", "FEATURE_WINDOW_EMBEDDING", Feature::WINDOW_EMBEDDING), crate::meta::inspect::EnumConstant::new("NATIVE_DIALOG_FILE_MIME", "FEATURE_NATIVE_DIALOG_FILE_MIME", Feature::NATIVE_DIALOG_FILE_MIME), crate::meta::inspect::EnumConstant::new("EMOJI_AND_SYMBOL_PICKER", "FEATURE_EMOJI_AND_SYMBOL_PICKER", Feature::EMOJI_AND_SYMBOL_PICKER), crate::meta::inspect::EnumConstant::new("NATIVE_COLOR_PICKER", "FEATURE_NATIVE_COLOR_PICKER", Feature::NATIVE_COLOR_PICKER), crate::meta::inspect::EnumConstant::new("SELF_FITTING_WINDOWS", "FEATURE_SELF_FITTING_WINDOWS", Feature::SELF_FITTING_WINDOWS), crate::meta::inspect::EnumConstant::new("ACCESSIBILITY_SCREEN_READER", "FEATURE_ACCESSIBILITY_SCREEN_READER", Feature::ACCESSIBILITY_SCREEN_READER)]
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Feature Global Menu", 0i64), EnumeratorShape::new_int("Feature Subwindows", 1i64), EnumeratorShape::new_int("Feature Touchscreen", 2i64), EnumeratorShape::new_int("Feature Mouse", 3i64), EnumeratorShape::new_int("Feature Mouse Warp", 4i64), EnumeratorShape::new_int("Feature Clipboard", 5i64), EnumeratorShape::new_int("Feature Virtual Keyboard", 6i64), EnumeratorShape::new_int("Feature Cursor Shape", 7i64), EnumeratorShape::new_int("Feature Custom Cursor Shape", 8i64), EnumeratorShape::new_int("Feature Native Dialog", 9i64), EnumeratorShape::new_int("Feature Ime", 10i64), EnumeratorShape::new_int("Feature Window Transparency", 11i64), EnumeratorShape::new_int("Feature Hidpi", 12i64), EnumeratorShape::new_int("Feature Icon", 13i64), EnumeratorShape::new_int("Feature Native Icon", 14i64), EnumeratorShape::new_int("Feature Orientation", 15i64), EnumeratorShape::new_int("Feature Swap Buffers", 16i64), EnumeratorShape::new_int("Feature Clipboard Primary", 18i64), EnumeratorShape::new_int("Feature Text To Speech", 19i64), EnumeratorShape::new_int("Feature Extend To Title", 20i64), EnumeratorShape::new_int("Feature Screen Capture", 21i64), EnumeratorShape::new_int("Feature Status Indicator", 22i64), EnumeratorShape::new_int("Feature Native Help", 23i64), EnumeratorShape::new_int("Feature Native Dialog Input", 24i64), EnumeratorShape::new_int("Feature Native Dialog File", 25i64), EnumeratorShape::new_int("Feature Native Dialog File Extra", 26i64), EnumeratorShape::new_int("Feature Window Drag", 27i64), EnumeratorShape::new_int("Feature Screen Exclude From Capture", 28i64), EnumeratorShape::new_int("Feature Window Embedding", 29i64), EnumeratorShape::new_int("Feature Native Dialog File Mime", 30i64), EnumeratorShape::new_int("Feature Emoji And Symbol Picker", 31i64), EnumeratorShape::new_int("Feature Native Color Picker", 32i64), EnumeratorShape::new_int("Feature Self Fitting Windows", 33i64), EnumeratorShape::new_int("Feature Accessibility Screen Reader", 34i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.Feature")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Feature {
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
impl crate::registry::property::Export for Feature {
    
}
impl crate::meta::Element for Feature {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityRole {
    ord: i32
}
impl AccessibilityRole {
    #[doc(alias = "ROLE_UNKNOWN")]
    #[doc = "Godot enumerator name: `ROLE_UNKNOWN`"]
    pub const UNKNOWN: AccessibilityRole = AccessibilityRole {
        ord: 0i32
    };
    #[doc(alias = "ROLE_DEFAULT_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_DEFAULT_BUTTON`"]
    pub const DEFAULT_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 1i32
    };
    #[doc(alias = "ROLE_AUDIO")]
    #[doc = "Godot enumerator name: `ROLE_AUDIO`"]
    pub const AUDIO: AccessibilityRole = AccessibilityRole {
        ord: 2i32
    };
    #[doc(alias = "ROLE_VIDEO")]
    #[doc = "Godot enumerator name: `ROLE_VIDEO`"]
    pub const VIDEO: AccessibilityRole = AccessibilityRole {
        ord: 3i32
    };
    #[doc(alias = "ROLE_STATIC_TEXT")]
    #[doc = "Godot enumerator name: `ROLE_STATIC_TEXT`"]
    pub const STATIC_TEXT: AccessibilityRole = AccessibilityRole {
        ord: 4i32
    };
    #[doc(alias = "ROLE_CONTAINER")]
    #[doc = "Godot enumerator name: `ROLE_CONTAINER`"]
    pub const CONTAINER: AccessibilityRole = AccessibilityRole {
        ord: 5i32
    };
    #[doc(alias = "ROLE_PANEL")]
    #[doc = "Godot enumerator name: `ROLE_PANEL`"]
    pub const PANEL: AccessibilityRole = AccessibilityRole {
        ord: 6i32
    };
    #[doc(alias = "ROLE_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_BUTTON`"]
    pub const BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 7i32
    };
    #[doc(alias = "ROLE_LINK")]
    #[doc = "Godot enumerator name: `ROLE_LINK`"]
    pub const LINK: AccessibilityRole = AccessibilityRole {
        ord: 8i32
    };
    #[doc(alias = "ROLE_CHECK_BOX")]
    #[doc = "Godot enumerator name: `ROLE_CHECK_BOX`"]
    pub const CHECK_BOX: AccessibilityRole = AccessibilityRole {
        ord: 9i32
    };
    #[doc(alias = "ROLE_RADIO_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_RADIO_BUTTON`"]
    pub const RADIO_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 10i32
    };
    #[doc(alias = "ROLE_CHECK_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_CHECK_BUTTON`"]
    pub const CHECK_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 11i32
    };
    #[doc(alias = "ROLE_SCROLL_BAR")]
    #[doc = "Godot enumerator name: `ROLE_SCROLL_BAR`"]
    pub const SCROLL_BAR: AccessibilityRole = AccessibilityRole {
        ord: 12i32
    };
    #[doc(alias = "ROLE_SCROLL_VIEW")]
    #[doc = "Godot enumerator name: `ROLE_SCROLL_VIEW`"]
    pub const SCROLL_VIEW: AccessibilityRole = AccessibilityRole {
        ord: 13i32
    };
    #[doc(alias = "ROLE_SPLITTER")]
    #[doc = "Godot enumerator name: `ROLE_SPLITTER`"]
    pub const SPLITTER: AccessibilityRole = AccessibilityRole {
        ord: 14i32
    };
    #[doc(alias = "ROLE_SLIDER")]
    #[doc = "Godot enumerator name: `ROLE_SLIDER`"]
    pub const SLIDER: AccessibilityRole = AccessibilityRole {
        ord: 15i32
    };
    #[doc(alias = "ROLE_SPIN_BUTTON")]
    #[doc = "Godot enumerator name: `ROLE_SPIN_BUTTON`"]
    pub const SPIN_BUTTON: AccessibilityRole = AccessibilityRole {
        ord: 16i32
    };
    #[doc(alias = "ROLE_PROGRESS_INDICATOR")]
    #[doc = "Godot enumerator name: `ROLE_PROGRESS_INDICATOR`"]
    pub const PROGRESS_INDICATOR: AccessibilityRole = AccessibilityRole {
        ord: 17i32
    };
    #[doc(alias = "ROLE_TEXT_FIELD")]
    #[doc = "Godot enumerator name: `ROLE_TEXT_FIELD`"]
    pub const TEXT_FIELD: AccessibilityRole = AccessibilityRole {
        ord: 18i32
    };
    #[doc(alias = "ROLE_MULTILINE_TEXT_FIELD")]
    #[doc = "Godot enumerator name: `ROLE_MULTILINE_TEXT_FIELD`"]
    pub const MULTILINE_TEXT_FIELD: AccessibilityRole = AccessibilityRole {
        ord: 19i32
    };
    #[doc(alias = "ROLE_COLOR_PICKER")]
    #[doc = "Godot enumerator name: `ROLE_COLOR_PICKER`"]
    pub const COLOR_PICKER: AccessibilityRole = AccessibilityRole {
        ord: 20i32
    };
    #[doc(alias = "ROLE_TABLE")]
    #[doc = "Godot enumerator name: `ROLE_TABLE`"]
    pub const TABLE: AccessibilityRole = AccessibilityRole {
        ord: 21i32
    };
    #[doc(alias = "ROLE_CELL")]
    #[doc = "Godot enumerator name: `ROLE_CELL`"]
    pub const CELL: AccessibilityRole = AccessibilityRole {
        ord: 22i32
    };
    #[doc(alias = "ROLE_ROW")]
    #[doc = "Godot enumerator name: `ROLE_ROW`"]
    pub const ROW: AccessibilityRole = AccessibilityRole {
        ord: 23i32
    };
    #[doc(alias = "ROLE_ROW_GROUP")]
    #[doc = "Godot enumerator name: `ROLE_ROW_GROUP`"]
    pub const ROW_GROUP: AccessibilityRole = AccessibilityRole {
        ord: 24i32
    };
    #[doc(alias = "ROLE_ROW_HEADER")]
    #[doc = "Godot enumerator name: `ROLE_ROW_HEADER`"]
    pub const ROW_HEADER: AccessibilityRole = AccessibilityRole {
        ord: 25i32
    };
    #[doc(alias = "ROLE_COLUMN_HEADER")]
    #[doc = "Godot enumerator name: `ROLE_COLUMN_HEADER`"]
    pub const COLUMN_HEADER: AccessibilityRole = AccessibilityRole {
        ord: 26i32
    };
    #[doc(alias = "ROLE_TREE")]
    #[doc = "Godot enumerator name: `ROLE_TREE`"]
    pub const TREE: AccessibilityRole = AccessibilityRole {
        ord: 27i32
    };
    #[doc(alias = "ROLE_TREE_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_TREE_ITEM`"]
    pub const TREE_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 28i32
    };
    #[doc(alias = "ROLE_LIST")]
    #[doc = "Godot enumerator name: `ROLE_LIST`"]
    pub const LIST: AccessibilityRole = AccessibilityRole {
        ord: 29i32
    };
    #[doc(alias = "ROLE_LIST_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_LIST_ITEM`"]
    pub const LIST_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 30i32
    };
    #[doc(alias = "ROLE_LIST_BOX")]
    #[doc = "Godot enumerator name: `ROLE_LIST_BOX`"]
    pub const LIST_BOX: AccessibilityRole = AccessibilityRole {
        ord: 31i32
    };
    #[doc(alias = "ROLE_LIST_BOX_OPTION")]
    #[doc = "Godot enumerator name: `ROLE_LIST_BOX_OPTION`"]
    pub const LIST_BOX_OPTION: AccessibilityRole = AccessibilityRole {
        ord: 32i32
    };
    #[doc(alias = "ROLE_TAB_BAR")]
    #[doc = "Godot enumerator name: `ROLE_TAB_BAR`"]
    pub const TAB_BAR: AccessibilityRole = AccessibilityRole {
        ord: 33i32
    };
    #[doc(alias = "ROLE_TAB")]
    #[doc = "Godot enumerator name: `ROLE_TAB`"]
    pub const TAB: AccessibilityRole = AccessibilityRole {
        ord: 34i32
    };
    #[doc(alias = "ROLE_TAB_PANEL")]
    #[doc = "Godot enumerator name: `ROLE_TAB_PANEL`"]
    pub const TAB_PANEL: AccessibilityRole = AccessibilityRole {
        ord: 35i32
    };
    #[doc(alias = "ROLE_MENU_BAR")]
    #[doc = "Godot enumerator name: `ROLE_MENU_BAR`"]
    pub const MENU_BAR: AccessibilityRole = AccessibilityRole {
        ord: 36i32
    };
    #[doc(alias = "ROLE_MENU")]
    #[doc = "Godot enumerator name: `ROLE_MENU`"]
    pub const MENU: AccessibilityRole = AccessibilityRole {
        ord: 37i32
    };
    #[doc(alias = "ROLE_MENU_ITEM")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM`"]
    pub const MENU_ITEM: AccessibilityRole = AccessibilityRole {
        ord: 38i32
    };
    #[doc(alias = "ROLE_MENU_ITEM_CHECK_BOX")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM_CHECK_BOX`"]
    pub const MENU_ITEM_CHECK_BOX: AccessibilityRole = AccessibilityRole {
        ord: 39i32
    };
    #[doc(alias = "ROLE_MENU_ITEM_RADIO")]
    #[doc = "Godot enumerator name: `ROLE_MENU_ITEM_RADIO`"]
    pub const MENU_ITEM_RADIO: AccessibilityRole = AccessibilityRole {
        ord: 40i32
    };
    #[doc(alias = "ROLE_IMAGE")]
    #[doc = "Godot enumerator name: `ROLE_IMAGE`"]
    pub const IMAGE: AccessibilityRole = AccessibilityRole {
        ord: 41i32
    };
    #[doc(alias = "ROLE_WINDOW")]
    #[doc = "Godot enumerator name: `ROLE_WINDOW`"]
    pub const WINDOW: AccessibilityRole = AccessibilityRole {
        ord: 42i32
    };
    #[doc(alias = "ROLE_TITLE_BAR")]
    #[doc = "Godot enumerator name: `ROLE_TITLE_BAR`"]
    pub const TITLE_BAR: AccessibilityRole = AccessibilityRole {
        ord: 43i32
    };
    #[doc(alias = "ROLE_DIALOG")]
    #[doc = "Godot enumerator name: `ROLE_DIALOG`"]
    pub const DIALOG: AccessibilityRole = AccessibilityRole {
        ord: 44i32
    };
    #[doc(alias = "ROLE_TOOLTIP")]
    #[doc = "Godot enumerator name: `ROLE_TOOLTIP`"]
    pub const TOOLTIP: AccessibilityRole = AccessibilityRole {
        ord: 45i32
    };
    
}
impl std::fmt::Debug for AccessibilityRole {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityRole") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityRole {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::DEFAULT_BUTTON => "DEFAULT_BUTTON", Self::AUDIO => "AUDIO", Self::VIDEO => "VIDEO", Self::STATIC_TEXT => "STATIC_TEXT", Self::CONTAINER => "CONTAINER", Self::PANEL => "PANEL", Self::BUTTON => "BUTTON", Self::LINK => "LINK", Self::CHECK_BOX => "CHECK_BOX", Self::RADIO_BUTTON => "RADIO_BUTTON", Self::CHECK_BUTTON => "CHECK_BUTTON", Self::SCROLL_BAR => "SCROLL_BAR", Self::SCROLL_VIEW => "SCROLL_VIEW", Self::SPLITTER => "SPLITTER", Self::SLIDER => "SLIDER", Self::SPIN_BUTTON => "SPIN_BUTTON", Self::PROGRESS_INDICATOR => "PROGRESS_INDICATOR", Self::TEXT_FIELD => "TEXT_FIELD", Self::MULTILINE_TEXT_FIELD => "MULTILINE_TEXT_FIELD", Self::COLOR_PICKER => "COLOR_PICKER", Self::TABLE => "TABLE", Self::CELL => "CELL", Self::ROW => "ROW", Self::ROW_GROUP => "ROW_GROUP", Self::ROW_HEADER => "ROW_HEADER", Self::COLUMN_HEADER => "COLUMN_HEADER", Self::TREE => "TREE", Self::TREE_ITEM => "TREE_ITEM", Self::LIST => "LIST", Self::LIST_ITEM => "LIST_ITEM", Self::LIST_BOX => "LIST_BOX", Self::LIST_BOX_OPTION => "LIST_BOX_OPTION", Self::TAB_BAR => "TAB_BAR", Self::TAB => "TAB", Self::TAB_PANEL => "TAB_PANEL", Self::MENU_BAR => "MENU_BAR", Self::MENU => "MENU", Self::MENU_ITEM => "MENU_ITEM", Self::MENU_ITEM_CHECK_BOX => "MENU_ITEM_CHECK_BOX", Self::MENU_ITEM_RADIO => "MENU_ITEM_RADIO", Self::IMAGE => "IMAGE", Self::WINDOW => "WINDOW", Self::TITLE_BAR => "TITLE_BAR", Self::DIALOG => "DIALOG", Self::TOOLTIP => "TOOLTIP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityRole::UNKNOWN, AccessibilityRole::DEFAULT_BUTTON, AccessibilityRole::AUDIO, AccessibilityRole::VIDEO, AccessibilityRole::STATIC_TEXT, AccessibilityRole::CONTAINER, AccessibilityRole::PANEL, AccessibilityRole::BUTTON, AccessibilityRole::LINK, AccessibilityRole::CHECK_BOX, AccessibilityRole::RADIO_BUTTON, AccessibilityRole::CHECK_BUTTON, AccessibilityRole::SCROLL_BAR, AccessibilityRole::SCROLL_VIEW, AccessibilityRole::SPLITTER, AccessibilityRole::SLIDER, AccessibilityRole::SPIN_BUTTON, AccessibilityRole::PROGRESS_INDICATOR, AccessibilityRole::TEXT_FIELD, AccessibilityRole::MULTILINE_TEXT_FIELD, AccessibilityRole::COLOR_PICKER, AccessibilityRole::TABLE, AccessibilityRole::CELL, AccessibilityRole::ROW, AccessibilityRole::ROW_GROUP, AccessibilityRole::ROW_HEADER, AccessibilityRole::COLUMN_HEADER, AccessibilityRole::TREE, AccessibilityRole::TREE_ITEM, AccessibilityRole::LIST, AccessibilityRole::LIST_ITEM, AccessibilityRole::LIST_BOX, AccessibilityRole::LIST_BOX_OPTION, AccessibilityRole::TAB_BAR, AccessibilityRole::TAB, AccessibilityRole::TAB_PANEL, AccessibilityRole::MENU_BAR, AccessibilityRole::MENU, AccessibilityRole::MENU_ITEM, AccessibilityRole::MENU_ITEM_CHECK_BOX, AccessibilityRole::MENU_ITEM_RADIO, AccessibilityRole::IMAGE, AccessibilityRole::WINDOW, AccessibilityRole::TITLE_BAR, AccessibilityRole::DIALOG, AccessibilityRole::TOOLTIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityRole >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "ROLE_UNKNOWN", AccessibilityRole::UNKNOWN), crate::meta::inspect::EnumConstant::new("DEFAULT_BUTTON", "ROLE_DEFAULT_BUTTON", AccessibilityRole::DEFAULT_BUTTON), crate::meta::inspect::EnumConstant::new("AUDIO", "ROLE_AUDIO", AccessibilityRole::AUDIO), crate::meta::inspect::EnumConstant::new("VIDEO", "ROLE_VIDEO", AccessibilityRole::VIDEO), crate::meta::inspect::EnumConstant::new("STATIC_TEXT", "ROLE_STATIC_TEXT", AccessibilityRole::STATIC_TEXT), crate::meta::inspect::EnumConstant::new("CONTAINER", "ROLE_CONTAINER", AccessibilityRole::CONTAINER), crate::meta::inspect::EnumConstant::new("PANEL", "ROLE_PANEL", AccessibilityRole::PANEL), crate::meta::inspect::EnumConstant::new("BUTTON", "ROLE_BUTTON", AccessibilityRole::BUTTON), crate::meta::inspect::EnumConstant::new("LINK", "ROLE_LINK", AccessibilityRole::LINK), crate::meta::inspect::EnumConstant::new("CHECK_BOX", "ROLE_CHECK_BOX", AccessibilityRole::CHECK_BOX), crate::meta::inspect::EnumConstant::new("RADIO_BUTTON", "ROLE_RADIO_BUTTON", AccessibilityRole::RADIO_BUTTON), crate::meta::inspect::EnumConstant::new("CHECK_BUTTON", "ROLE_CHECK_BUTTON", AccessibilityRole::CHECK_BUTTON), crate::meta::inspect::EnumConstant::new("SCROLL_BAR", "ROLE_SCROLL_BAR", AccessibilityRole::SCROLL_BAR), crate::meta::inspect::EnumConstant::new("SCROLL_VIEW", "ROLE_SCROLL_VIEW", AccessibilityRole::SCROLL_VIEW), crate::meta::inspect::EnumConstant::new("SPLITTER", "ROLE_SPLITTER", AccessibilityRole::SPLITTER), crate::meta::inspect::EnumConstant::new("SLIDER", "ROLE_SLIDER", AccessibilityRole::SLIDER), crate::meta::inspect::EnumConstant::new("SPIN_BUTTON", "ROLE_SPIN_BUTTON", AccessibilityRole::SPIN_BUTTON), crate::meta::inspect::EnumConstant::new("PROGRESS_INDICATOR", "ROLE_PROGRESS_INDICATOR", AccessibilityRole::PROGRESS_INDICATOR), crate::meta::inspect::EnumConstant::new("TEXT_FIELD", "ROLE_TEXT_FIELD", AccessibilityRole::TEXT_FIELD), crate::meta::inspect::EnumConstant::new("MULTILINE_TEXT_FIELD", "ROLE_MULTILINE_TEXT_FIELD", AccessibilityRole::MULTILINE_TEXT_FIELD), crate::meta::inspect::EnumConstant::new("COLOR_PICKER", "ROLE_COLOR_PICKER", AccessibilityRole::COLOR_PICKER), crate::meta::inspect::EnumConstant::new("TABLE", "ROLE_TABLE", AccessibilityRole::TABLE), crate::meta::inspect::EnumConstant::new("CELL", "ROLE_CELL", AccessibilityRole::CELL), crate::meta::inspect::EnumConstant::new("ROW", "ROLE_ROW", AccessibilityRole::ROW), crate::meta::inspect::EnumConstant::new("ROW_GROUP", "ROLE_ROW_GROUP", AccessibilityRole::ROW_GROUP), crate::meta::inspect::EnumConstant::new("ROW_HEADER", "ROLE_ROW_HEADER", AccessibilityRole::ROW_HEADER), crate::meta::inspect::EnumConstant::new("COLUMN_HEADER", "ROLE_COLUMN_HEADER", AccessibilityRole::COLUMN_HEADER), crate::meta::inspect::EnumConstant::new("TREE", "ROLE_TREE", AccessibilityRole::TREE), crate::meta::inspect::EnumConstant::new("TREE_ITEM", "ROLE_TREE_ITEM", AccessibilityRole::TREE_ITEM), crate::meta::inspect::EnumConstant::new("LIST", "ROLE_LIST", AccessibilityRole::LIST), crate::meta::inspect::EnumConstant::new("LIST_ITEM", "ROLE_LIST_ITEM", AccessibilityRole::LIST_ITEM), crate::meta::inspect::EnumConstant::new("LIST_BOX", "ROLE_LIST_BOX", AccessibilityRole::LIST_BOX), crate::meta::inspect::EnumConstant::new("LIST_BOX_OPTION", "ROLE_LIST_BOX_OPTION", AccessibilityRole::LIST_BOX_OPTION), crate::meta::inspect::EnumConstant::new("TAB_BAR", "ROLE_TAB_BAR", AccessibilityRole::TAB_BAR), crate::meta::inspect::EnumConstant::new("TAB", "ROLE_TAB", AccessibilityRole::TAB), crate::meta::inspect::EnumConstant::new("TAB_PANEL", "ROLE_TAB_PANEL", AccessibilityRole::TAB_PANEL), crate::meta::inspect::EnumConstant::new("MENU_BAR", "ROLE_MENU_BAR", AccessibilityRole::MENU_BAR), crate::meta::inspect::EnumConstant::new("MENU", "ROLE_MENU", AccessibilityRole::MENU), crate::meta::inspect::EnumConstant::new("MENU_ITEM", "ROLE_MENU_ITEM", AccessibilityRole::MENU_ITEM), crate::meta::inspect::EnumConstant::new("MENU_ITEM_CHECK_BOX", "ROLE_MENU_ITEM_CHECK_BOX", AccessibilityRole::MENU_ITEM_CHECK_BOX), crate::meta::inspect::EnumConstant::new("MENU_ITEM_RADIO", "ROLE_MENU_ITEM_RADIO", AccessibilityRole::MENU_ITEM_RADIO), crate::meta::inspect::EnumConstant::new("IMAGE", "ROLE_IMAGE", AccessibilityRole::IMAGE), crate::meta::inspect::EnumConstant::new("WINDOW", "ROLE_WINDOW", AccessibilityRole::WINDOW), crate::meta::inspect::EnumConstant::new("TITLE_BAR", "ROLE_TITLE_BAR", AccessibilityRole::TITLE_BAR), crate::meta::inspect::EnumConstant::new("DIALOG", "ROLE_DIALOG", AccessibilityRole::DIALOG), crate::meta::inspect::EnumConstant::new("TOOLTIP", "ROLE_TOOLTIP", AccessibilityRole::TOOLTIP)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityRole {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Role Unknown", 0i64), EnumeratorShape::new_int("Role Default Button", 1i64), EnumeratorShape::new_int("Role Audio", 2i64), EnumeratorShape::new_int("Role Video", 3i64), EnumeratorShape::new_int("Role Static Text", 4i64), EnumeratorShape::new_int("Role Container", 5i64), EnumeratorShape::new_int("Role Panel", 6i64), EnumeratorShape::new_int("Role Button", 7i64), EnumeratorShape::new_int("Role Link", 8i64), EnumeratorShape::new_int("Role Check Box", 9i64), EnumeratorShape::new_int("Role Radio Button", 10i64), EnumeratorShape::new_int("Role Check Button", 11i64), EnumeratorShape::new_int("Role Scroll Bar", 12i64), EnumeratorShape::new_int("Role Scroll View", 13i64), EnumeratorShape::new_int("Role Splitter", 14i64), EnumeratorShape::new_int("Role Slider", 15i64), EnumeratorShape::new_int("Role Spin Button", 16i64), EnumeratorShape::new_int("Role Progress Indicator", 17i64), EnumeratorShape::new_int("Role Text Field", 18i64), EnumeratorShape::new_int("Role Multiline Text Field", 19i64), EnumeratorShape::new_int("Role Color Picker", 20i64), EnumeratorShape::new_int("Role Table", 21i64), EnumeratorShape::new_int("Role Cell", 22i64), EnumeratorShape::new_int("Role Row", 23i64), EnumeratorShape::new_int("Role Row Group", 24i64), EnumeratorShape::new_int("Role Row Header", 25i64), EnumeratorShape::new_int("Role Column Header", 26i64), EnumeratorShape::new_int("Role Tree", 27i64), EnumeratorShape::new_int("Role Tree Item", 28i64), EnumeratorShape::new_int("Role List", 29i64), EnumeratorShape::new_int("Role List Item", 30i64), EnumeratorShape::new_int("Role List Box", 31i64), EnumeratorShape::new_int("Role List Box Option", 32i64), EnumeratorShape::new_int("Role Tab Bar", 33i64), EnumeratorShape::new_int("Role Tab", 34i64), EnumeratorShape::new_int("Role Tab Panel", 35i64), EnumeratorShape::new_int("Role Menu Bar", 36i64), EnumeratorShape::new_int("Role Menu", 37i64), EnumeratorShape::new_int("Role Menu Item", 38i64), EnumeratorShape::new_int("Role Menu Item Check Box", 39i64), EnumeratorShape::new_int("Role Menu Item Radio", 40i64), EnumeratorShape::new_int("Role Image", 41i64), EnumeratorShape::new_int("Role Window", 42i64), EnumeratorShape::new_int("Role Title Bar", 43i64), EnumeratorShape::new_int("Role Dialog", 44i64), EnumeratorShape::new_int("Role Tooltip", 45i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityRole")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityRole {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityRole {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityRole {
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
impl crate::registry::property::Export for AccessibilityRole {
    
}
impl crate::meta::Element for AccessibilityRole {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityPopupType {
    ord: i32
}
impl AccessibilityPopupType {
    #[doc(alias = "POPUP_MENU")]
    #[doc = "Godot enumerator name: `POPUP_MENU`"]
    pub const MENU: AccessibilityPopupType = AccessibilityPopupType {
        ord: 0i32
    };
    #[doc(alias = "POPUP_LIST")]
    #[doc = "Godot enumerator name: `POPUP_LIST`"]
    pub const LIST: AccessibilityPopupType = AccessibilityPopupType {
        ord: 1i32
    };
    #[doc(alias = "POPUP_TREE")]
    #[doc = "Godot enumerator name: `POPUP_TREE`"]
    pub const TREE: AccessibilityPopupType = AccessibilityPopupType {
        ord: 2i32
    };
    #[doc(alias = "POPUP_DIALOG")]
    #[doc = "Godot enumerator name: `POPUP_DIALOG`"]
    pub const DIALOG: AccessibilityPopupType = AccessibilityPopupType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AccessibilityPopupType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityPopupType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityPopupType {
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
            Self::MENU => "MENU", Self::LIST => "LIST", Self::TREE => "TREE", Self::DIALOG => "DIALOG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityPopupType::MENU, AccessibilityPopupType::LIST, AccessibilityPopupType::TREE, AccessibilityPopupType::DIALOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityPopupType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MENU", "POPUP_MENU", AccessibilityPopupType::MENU), crate::meta::inspect::EnumConstant::new("LIST", "POPUP_LIST", AccessibilityPopupType::LIST), crate::meta::inspect::EnumConstant::new("TREE", "POPUP_TREE", AccessibilityPopupType::TREE), crate::meta::inspect::EnumConstant::new("DIALOG", "POPUP_DIALOG", AccessibilityPopupType::DIALOG)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityPopupType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Popup Menu", 0i64), EnumeratorShape::new_int("Popup List", 1i64), EnumeratorShape::new_int("Popup Tree", 2i64), EnumeratorShape::new_int("Popup Dialog", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityPopupType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityPopupType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityPopupType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityPopupType {
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
impl crate::registry::property::Export for AccessibilityPopupType {
    
}
impl crate::meta::Element for AccessibilityPopupType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityFlags {
    ord: i32
}
impl AccessibilityFlags {
    #[doc(alias = "FLAG_HIDDEN")]
    #[doc = "Godot enumerator name: `FLAG_HIDDEN`"]
    pub const HIDDEN: AccessibilityFlags = AccessibilityFlags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_MULTISELECTABLE")]
    #[doc = "Godot enumerator name: `FLAG_MULTISELECTABLE`"]
    pub const MULTISELECTABLE: AccessibilityFlags = AccessibilityFlags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_REQUIRED")]
    #[doc = "Godot enumerator name: `FLAG_REQUIRED`"]
    pub const REQUIRED: AccessibilityFlags = AccessibilityFlags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_VISITED")]
    #[doc = "Godot enumerator name: `FLAG_VISITED`"]
    pub const VISITED: AccessibilityFlags = AccessibilityFlags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_BUSY")]
    #[doc = "Godot enumerator name: `FLAG_BUSY`"]
    pub const BUSY: AccessibilityFlags = AccessibilityFlags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_MODAL")]
    #[doc = "Godot enumerator name: `FLAG_MODAL`"]
    pub const MODAL: AccessibilityFlags = AccessibilityFlags {
        ord: 5i32
    };
    #[doc(alias = "FLAG_TOUCH_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `FLAG_TOUCH_PASSTHROUGH`"]
    pub const TOUCH_PASSTHROUGH: AccessibilityFlags = AccessibilityFlags {
        ord: 6i32
    };
    #[doc(alias = "FLAG_READONLY")]
    #[doc = "Godot enumerator name: `FLAG_READONLY`"]
    pub const READONLY: AccessibilityFlags = AccessibilityFlags {
        ord: 7i32
    };
    #[doc(alias = "FLAG_DISABLED")]
    #[doc = "Godot enumerator name: `FLAG_DISABLED`"]
    pub const DISABLED: AccessibilityFlags = AccessibilityFlags {
        ord: 8i32
    };
    #[doc(alias = "FLAG_CLIPS_CHILDREN")]
    #[doc = "Godot enumerator name: `FLAG_CLIPS_CHILDREN`"]
    pub const CLIPS_CHILDREN: AccessibilityFlags = AccessibilityFlags {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for AccessibilityFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::HIDDEN => "HIDDEN", Self::MULTISELECTABLE => "MULTISELECTABLE", Self::REQUIRED => "REQUIRED", Self::VISITED => "VISITED", Self::BUSY => "BUSY", Self::MODAL => "MODAL", Self::TOUCH_PASSTHROUGH => "TOUCH_PASSTHROUGH", Self::READONLY => "READONLY", Self::DISABLED => "DISABLED", Self::CLIPS_CHILDREN => "CLIPS_CHILDREN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityFlags::HIDDEN, AccessibilityFlags::MULTISELECTABLE, AccessibilityFlags::REQUIRED, AccessibilityFlags::VISITED, AccessibilityFlags::BUSY, AccessibilityFlags::MODAL, AccessibilityFlags::TOUCH_PASSTHROUGH, AccessibilityFlags::READONLY, AccessibilityFlags::DISABLED, AccessibilityFlags::CLIPS_CHILDREN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HIDDEN", "FLAG_HIDDEN", AccessibilityFlags::HIDDEN), crate::meta::inspect::EnumConstant::new("MULTISELECTABLE", "FLAG_MULTISELECTABLE", AccessibilityFlags::MULTISELECTABLE), crate::meta::inspect::EnumConstant::new("REQUIRED", "FLAG_REQUIRED", AccessibilityFlags::REQUIRED), crate::meta::inspect::EnumConstant::new("VISITED", "FLAG_VISITED", AccessibilityFlags::VISITED), crate::meta::inspect::EnumConstant::new("BUSY", "FLAG_BUSY", AccessibilityFlags::BUSY), crate::meta::inspect::EnumConstant::new("MODAL", "FLAG_MODAL", AccessibilityFlags::MODAL), crate::meta::inspect::EnumConstant::new("TOUCH_PASSTHROUGH", "FLAG_TOUCH_PASSTHROUGH", AccessibilityFlags::TOUCH_PASSTHROUGH), crate::meta::inspect::EnumConstant::new("READONLY", "FLAG_READONLY", AccessibilityFlags::READONLY), crate::meta::inspect::EnumConstant::new("DISABLED", "FLAG_DISABLED", AccessibilityFlags::DISABLED), crate::meta::inspect::EnumConstant::new("CLIPS_CHILDREN", "FLAG_CLIPS_CHILDREN", AccessibilityFlags::CLIPS_CHILDREN)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Flag Hidden", 0i64), EnumeratorShape::new_int("Flag Multiselectable", 1i64), EnumeratorShape::new_int("Flag Required", 2i64), EnumeratorShape::new_int("Flag Visited", 3i64), EnumeratorShape::new_int("Flag Busy", 4i64), EnumeratorShape::new_int("Flag Modal", 5i64), EnumeratorShape::new_int("Flag Touch Passthrough", 6i64), EnumeratorShape::new_int("Flag Readonly", 7i64), EnumeratorShape::new_int("Flag Disabled", 8i64), EnumeratorShape::new_int("Flag Clips Children", 9i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityFlags {
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
impl crate::registry::property::Export for AccessibilityFlags {
    
}
impl crate::meta::Element for AccessibilityFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityAction {
    ord: i32
}
impl AccessibilityAction {
    #[doc(alias = "ACTION_CLICK")]
    #[doc = "Godot enumerator name: `ACTION_CLICK`"]
    pub const CLICK: AccessibilityAction = AccessibilityAction {
        ord: 0i32
    };
    #[doc(alias = "ACTION_FOCUS")]
    #[doc = "Godot enumerator name: `ACTION_FOCUS`"]
    pub const FOCUS: AccessibilityAction = AccessibilityAction {
        ord: 1i32
    };
    #[doc(alias = "ACTION_BLUR")]
    #[doc = "Godot enumerator name: `ACTION_BLUR`"]
    pub const BLUR: AccessibilityAction = AccessibilityAction {
        ord: 2i32
    };
    #[doc(alias = "ACTION_COLLAPSE")]
    #[doc = "Godot enumerator name: `ACTION_COLLAPSE`"]
    pub const COLLAPSE: AccessibilityAction = AccessibilityAction {
        ord: 3i32
    };
    #[doc(alias = "ACTION_EXPAND")]
    #[doc = "Godot enumerator name: `ACTION_EXPAND`"]
    pub const EXPAND: AccessibilityAction = AccessibilityAction {
        ord: 4i32
    };
    #[doc(alias = "ACTION_DECREMENT")]
    #[doc = "Godot enumerator name: `ACTION_DECREMENT`"]
    pub const DECREMENT: AccessibilityAction = AccessibilityAction {
        ord: 5i32
    };
    #[doc(alias = "ACTION_INCREMENT")]
    #[doc = "Godot enumerator name: `ACTION_INCREMENT`"]
    pub const INCREMENT: AccessibilityAction = AccessibilityAction {
        ord: 6i32
    };
    #[doc(alias = "ACTION_HIDE_TOOLTIP")]
    #[doc = "Godot enumerator name: `ACTION_HIDE_TOOLTIP`"]
    pub const HIDE_TOOLTIP: AccessibilityAction = AccessibilityAction {
        ord: 7i32
    };
    #[doc(alias = "ACTION_SHOW_TOOLTIP")]
    #[doc = "Godot enumerator name: `ACTION_SHOW_TOOLTIP`"]
    pub const SHOW_TOOLTIP: AccessibilityAction = AccessibilityAction {
        ord: 8i32
    };
    #[doc(alias = "ACTION_SET_TEXT_SELECTION")]
    #[doc = "Godot enumerator name: `ACTION_SET_TEXT_SELECTION`"]
    pub const SET_TEXT_SELECTION: AccessibilityAction = AccessibilityAction {
        ord: 9i32
    };
    #[doc(alias = "ACTION_REPLACE_SELECTED_TEXT")]
    #[doc = "Godot enumerator name: `ACTION_REPLACE_SELECTED_TEXT`"]
    pub const REPLACE_SELECTED_TEXT: AccessibilityAction = AccessibilityAction {
        ord: 10i32
    };
    #[doc(alias = "ACTION_SCROLL_BACKWARD")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_BACKWARD`"]
    pub const SCROLL_BACKWARD: AccessibilityAction = AccessibilityAction {
        ord: 11i32
    };
    #[doc(alias = "ACTION_SCROLL_DOWN")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_DOWN`"]
    pub const SCROLL_DOWN: AccessibilityAction = AccessibilityAction {
        ord: 12i32
    };
    #[doc(alias = "ACTION_SCROLL_FORWARD")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_FORWARD`"]
    pub const SCROLL_FORWARD: AccessibilityAction = AccessibilityAction {
        ord: 13i32
    };
    #[doc(alias = "ACTION_SCROLL_LEFT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_LEFT`"]
    pub const SCROLL_LEFT: AccessibilityAction = AccessibilityAction {
        ord: 14i32
    };
    #[doc(alias = "ACTION_SCROLL_RIGHT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_RIGHT`"]
    pub const SCROLL_RIGHT: AccessibilityAction = AccessibilityAction {
        ord: 15i32
    };
    #[doc(alias = "ACTION_SCROLL_UP")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_UP`"]
    pub const SCROLL_UP: AccessibilityAction = AccessibilityAction {
        ord: 16i32
    };
    #[doc(alias = "ACTION_SCROLL_INTO_VIEW")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_INTO_VIEW`"]
    pub const SCROLL_INTO_VIEW: AccessibilityAction = AccessibilityAction {
        ord: 17i32
    };
    #[doc(alias = "ACTION_SCROLL_TO_POINT")]
    #[doc = "Godot enumerator name: `ACTION_SCROLL_TO_POINT`"]
    pub const SCROLL_TO_POINT: AccessibilityAction = AccessibilityAction {
        ord: 18i32
    };
    #[doc(alias = "ACTION_SET_SCROLL_OFFSET")]
    #[doc = "Godot enumerator name: `ACTION_SET_SCROLL_OFFSET`"]
    pub const SET_SCROLL_OFFSET: AccessibilityAction = AccessibilityAction {
        ord: 19i32
    };
    #[doc(alias = "ACTION_SET_VALUE")]
    #[doc = "Godot enumerator name: `ACTION_SET_VALUE`"]
    pub const SET_VALUE: AccessibilityAction = AccessibilityAction {
        ord: 20i32
    };
    #[doc(alias = "ACTION_SHOW_CONTEXT_MENU")]
    #[doc = "Godot enumerator name: `ACTION_SHOW_CONTEXT_MENU`"]
    pub const SHOW_CONTEXT_MENU: AccessibilityAction = AccessibilityAction {
        ord: 21i32
    };
    #[doc(alias = "ACTION_CUSTOM")]
    #[doc = "Godot enumerator name: `ACTION_CUSTOM`"]
    pub const CUSTOM: AccessibilityAction = AccessibilityAction {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for AccessibilityAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::CLICK => "CLICK", Self::FOCUS => "FOCUS", Self::BLUR => "BLUR", Self::COLLAPSE => "COLLAPSE", Self::EXPAND => "EXPAND", Self::DECREMENT => "DECREMENT", Self::INCREMENT => "INCREMENT", Self::HIDE_TOOLTIP => "HIDE_TOOLTIP", Self::SHOW_TOOLTIP => "SHOW_TOOLTIP", Self::SET_TEXT_SELECTION => "SET_TEXT_SELECTION", Self::REPLACE_SELECTED_TEXT => "REPLACE_SELECTED_TEXT", Self::SCROLL_BACKWARD => "SCROLL_BACKWARD", Self::SCROLL_DOWN => "SCROLL_DOWN", Self::SCROLL_FORWARD => "SCROLL_FORWARD", Self::SCROLL_LEFT => "SCROLL_LEFT", Self::SCROLL_RIGHT => "SCROLL_RIGHT", Self::SCROLL_UP => "SCROLL_UP", Self::SCROLL_INTO_VIEW => "SCROLL_INTO_VIEW", Self::SCROLL_TO_POINT => "SCROLL_TO_POINT", Self::SET_SCROLL_OFFSET => "SET_SCROLL_OFFSET", Self::SET_VALUE => "SET_VALUE", Self::SHOW_CONTEXT_MENU => "SHOW_CONTEXT_MENU", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityAction::CLICK, AccessibilityAction::FOCUS, AccessibilityAction::BLUR, AccessibilityAction::COLLAPSE, AccessibilityAction::EXPAND, AccessibilityAction::DECREMENT, AccessibilityAction::INCREMENT, AccessibilityAction::HIDE_TOOLTIP, AccessibilityAction::SHOW_TOOLTIP, AccessibilityAction::SET_TEXT_SELECTION, AccessibilityAction::REPLACE_SELECTED_TEXT, AccessibilityAction::SCROLL_BACKWARD, AccessibilityAction::SCROLL_DOWN, AccessibilityAction::SCROLL_FORWARD, AccessibilityAction::SCROLL_LEFT, AccessibilityAction::SCROLL_RIGHT, AccessibilityAction::SCROLL_UP, AccessibilityAction::SCROLL_INTO_VIEW, AccessibilityAction::SCROLL_TO_POINT, AccessibilityAction::SET_SCROLL_OFFSET, AccessibilityAction::SET_VALUE, AccessibilityAction::SHOW_CONTEXT_MENU, AccessibilityAction::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLICK", "ACTION_CLICK", AccessibilityAction::CLICK), crate::meta::inspect::EnumConstant::new("FOCUS", "ACTION_FOCUS", AccessibilityAction::FOCUS), crate::meta::inspect::EnumConstant::new("BLUR", "ACTION_BLUR", AccessibilityAction::BLUR), crate::meta::inspect::EnumConstant::new("COLLAPSE", "ACTION_COLLAPSE", AccessibilityAction::COLLAPSE), crate::meta::inspect::EnumConstant::new("EXPAND", "ACTION_EXPAND", AccessibilityAction::EXPAND), crate::meta::inspect::EnumConstant::new("DECREMENT", "ACTION_DECREMENT", AccessibilityAction::DECREMENT), crate::meta::inspect::EnumConstant::new("INCREMENT", "ACTION_INCREMENT", AccessibilityAction::INCREMENT), crate::meta::inspect::EnumConstant::new("HIDE_TOOLTIP", "ACTION_HIDE_TOOLTIP", AccessibilityAction::HIDE_TOOLTIP), crate::meta::inspect::EnumConstant::new("SHOW_TOOLTIP", "ACTION_SHOW_TOOLTIP", AccessibilityAction::SHOW_TOOLTIP), crate::meta::inspect::EnumConstant::new("SET_TEXT_SELECTION", "ACTION_SET_TEXT_SELECTION", AccessibilityAction::SET_TEXT_SELECTION), crate::meta::inspect::EnumConstant::new("REPLACE_SELECTED_TEXT", "ACTION_REPLACE_SELECTED_TEXT", AccessibilityAction::REPLACE_SELECTED_TEXT), crate::meta::inspect::EnumConstant::new("SCROLL_BACKWARD", "ACTION_SCROLL_BACKWARD", AccessibilityAction::SCROLL_BACKWARD), crate::meta::inspect::EnumConstant::new("SCROLL_DOWN", "ACTION_SCROLL_DOWN", AccessibilityAction::SCROLL_DOWN), crate::meta::inspect::EnumConstant::new("SCROLL_FORWARD", "ACTION_SCROLL_FORWARD", AccessibilityAction::SCROLL_FORWARD), crate::meta::inspect::EnumConstant::new("SCROLL_LEFT", "ACTION_SCROLL_LEFT", AccessibilityAction::SCROLL_LEFT), crate::meta::inspect::EnumConstant::new("SCROLL_RIGHT", "ACTION_SCROLL_RIGHT", AccessibilityAction::SCROLL_RIGHT), crate::meta::inspect::EnumConstant::new("SCROLL_UP", "ACTION_SCROLL_UP", AccessibilityAction::SCROLL_UP), crate::meta::inspect::EnumConstant::new("SCROLL_INTO_VIEW", "ACTION_SCROLL_INTO_VIEW", AccessibilityAction::SCROLL_INTO_VIEW), crate::meta::inspect::EnumConstant::new("SCROLL_TO_POINT", "ACTION_SCROLL_TO_POINT", AccessibilityAction::SCROLL_TO_POINT), crate::meta::inspect::EnumConstant::new("SET_SCROLL_OFFSET", "ACTION_SET_SCROLL_OFFSET", AccessibilityAction::SET_SCROLL_OFFSET), crate::meta::inspect::EnumConstant::new("SET_VALUE", "ACTION_SET_VALUE", AccessibilityAction::SET_VALUE), crate::meta::inspect::EnumConstant::new("SHOW_CONTEXT_MENU", "ACTION_SHOW_CONTEXT_MENU", AccessibilityAction::SHOW_CONTEXT_MENU), crate::meta::inspect::EnumConstant::new("CUSTOM", "ACTION_CUSTOM", AccessibilityAction::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityAction {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Action Click", 0i64), EnumeratorShape::new_int("Action Focus", 1i64), EnumeratorShape::new_int("Action Blur", 2i64), EnumeratorShape::new_int("Action Collapse", 3i64), EnumeratorShape::new_int("Action Expand", 4i64), EnumeratorShape::new_int("Action Decrement", 5i64), EnumeratorShape::new_int("Action Increment", 6i64), EnumeratorShape::new_int("Action Hide Tooltip", 7i64), EnumeratorShape::new_int("Action Show Tooltip", 8i64), EnumeratorShape::new_int("Action Set Text Selection", 9i64), EnumeratorShape::new_int("Action Replace Selected Text", 10i64), EnumeratorShape::new_int("Action Scroll Backward", 11i64), EnumeratorShape::new_int("Action Scroll Down", 12i64), EnumeratorShape::new_int("Action Scroll Forward", 13i64), EnumeratorShape::new_int("Action Scroll Left", 14i64), EnumeratorShape::new_int("Action Scroll Right", 15i64), EnumeratorShape::new_int("Action Scroll Up", 16i64), EnumeratorShape::new_int("Action Scroll Into View", 17i64), EnumeratorShape::new_int("Action Scroll To Point", 18i64), EnumeratorShape::new_int("Action Set Scroll Offset", 19i64), EnumeratorShape::new_int("Action Set Value", 20i64), EnumeratorShape::new_int("Action Show Context Menu", 21i64), EnumeratorShape::new_int("Action Custom", 22i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityAction")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityAction {
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
impl crate::registry::property::Export for AccessibilityAction {
    
}
impl crate::meta::Element for AccessibilityAction {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityLiveMode {
    ord: i32
}
impl AccessibilityLiveMode {
    #[doc(alias = "LIVE_OFF")]
    #[doc = "Godot enumerator name: `LIVE_OFF`"]
    pub const OFF: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 0i32
    };
    #[doc(alias = "LIVE_POLITE")]
    #[doc = "Godot enumerator name: `LIVE_POLITE`"]
    pub const POLITE: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 1i32
    };
    #[doc(alias = "LIVE_ASSERTIVE")]
    #[doc = "Godot enumerator name: `LIVE_ASSERTIVE`"]
    pub const ASSERTIVE: AccessibilityLiveMode = AccessibilityLiveMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AccessibilityLiveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityLiveMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityLiveMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::OFF => "OFF", Self::POLITE => "POLITE", Self::ASSERTIVE => "ASSERTIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityLiveMode::OFF, AccessibilityLiveMode::POLITE, AccessibilityLiveMode::ASSERTIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityLiveMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "LIVE_OFF", AccessibilityLiveMode::OFF), crate::meta::inspect::EnumConstant::new("POLITE", "LIVE_POLITE", AccessibilityLiveMode::POLITE), crate::meta::inspect::EnumConstant::new("ASSERTIVE", "LIVE_ASSERTIVE", AccessibilityLiveMode::ASSERTIVE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityLiveMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Live Off", 0i64), EnumeratorShape::new_int("Live Polite", 1i64), EnumeratorShape::new_int("Live Assertive", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityLiveMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityLiveMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityLiveMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityLiveMode {
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
impl crate::registry::property::Export for AccessibilityLiveMode {
    
}
impl crate::meta::Element for AccessibilityLiveMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityScrollUnit {
    ord: i32
}
impl AccessibilityScrollUnit {
    #[doc(alias = "SCROLL_UNIT_ITEM")]
    #[doc = "Godot enumerator name: `SCROLL_UNIT_ITEM`"]
    pub const ITEM: AccessibilityScrollUnit = AccessibilityScrollUnit {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_UNIT_PAGE")]
    #[doc = "Godot enumerator name: `SCROLL_UNIT_PAGE`"]
    pub const PAGE: AccessibilityScrollUnit = AccessibilityScrollUnit {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AccessibilityScrollUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityScrollUnit") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityScrollUnit {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::ITEM => "ITEM", Self::PAGE => "PAGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityScrollUnit::ITEM, AccessibilityScrollUnit::PAGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityScrollUnit >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ITEM", "SCROLL_UNIT_ITEM", AccessibilityScrollUnit::ITEM), crate::meta::inspect::EnumConstant::new("PAGE", "SCROLL_UNIT_PAGE", AccessibilityScrollUnit::PAGE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityScrollUnit {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Scroll Unit Item", 0i64), EnumeratorShape::new_int("Scroll Unit Page", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityScrollUnit")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityScrollUnit {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityScrollUnit {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityScrollUnit {
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
impl crate::registry::property::Export for AccessibilityScrollUnit {
    
}
impl crate::meta::Element for AccessibilityScrollUnit {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AccessibilityScrollHint {
    ord: i32
}
impl AccessibilityScrollHint {
    #[doc(alias = "SCROLL_HINT_TOP_LEFT")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_TOP_LEFT`"]
    pub const TOP_LEFT: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_HINT_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 1i32
    };
    #[doc(alias = "SCROLL_HINT_TOP_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_TOP_EDGE`"]
    pub const TOP_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 2i32
    };
    #[doc(alias = "SCROLL_HINT_BOTTOM_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_BOTTOM_EDGE`"]
    pub const BOTTOM_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 3i32
    };
    #[doc(alias = "SCROLL_HINT_LEFT_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_LEFT_EDGE`"]
    pub const LEFT_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 4i32
    };
    #[doc(alias = "SCROLL_HINT_RIGHT_EDGE")]
    #[doc = "Godot enumerator name: `SCROLL_HINT_RIGHT_EDGE`"]
    pub const RIGHT_EDGE: AccessibilityScrollHint = AccessibilityScrollHint {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for AccessibilityScrollHint {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AccessibilityScrollHint") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AccessibilityScrollHint {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::TOP_EDGE => "TOP_EDGE", Self::BOTTOM_EDGE => "BOTTOM_EDGE", Self::LEFT_EDGE => "LEFT_EDGE", Self::RIGHT_EDGE => "RIGHT_EDGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AccessibilityScrollHint::TOP_LEFT, AccessibilityScrollHint::BOTTOM_RIGHT, AccessibilityScrollHint::TOP_EDGE, AccessibilityScrollHint::BOTTOM_EDGE, AccessibilityScrollHint::LEFT_EDGE, AccessibilityScrollHint::RIGHT_EDGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AccessibilityScrollHint >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP_LEFT", "SCROLL_HINT_TOP_LEFT", AccessibilityScrollHint::TOP_LEFT), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "SCROLL_HINT_BOTTOM_RIGHT", AccessibilityScrollHint::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("TOP_EDGE", "SCROLL_HINT_TOP_EDGE", AccessibilityScrollHint::TOP_EDGE), crate::meta::inspect::EnumConstant::new("BOTTOM_EDGE", "SCROLL_HINT_BOTTOM_EDGE", AccessibilityScrollHint::BOTTOM_EDGE), crate::meta::inspect::EnumConstant::new("LEFT_EDGE", "SCROLL_HINT_LEFT_EDGE", AccessibilityScrollHint::LEFT_EDGE), crate::meta::inspect::EnumConstant::new("RIGHT_EDGE", "SCROLL_HINT_RIGHT_EDGE", AccessibilityScrollHint::RIGHT_EDGE)]
        }
    }
}
impl crate::meta::GodotConvert for AccessibilityScrollHint {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Scroll Hint Top Left", 0i64), EnumeratorShape::new_int("Scroll Hint Bottom Right", 1i64), EnumeratorShape::new_int("Scroll Hint Top Edge", 2i64), EnumeratorShape::new_int("Scroll Hint Bottom Edge", 3i64), EnumeratorShape::new_int("Scroll Hint Left Edge", 4i64), EnumeratorShape::new_int("Scroll Hint Right Edge", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.AccessibilityScrollHint")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for AccessibilityScrollHint {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AccessibilityScrollHint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for AccessibilityScrollHint {
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
impl crate::registry::property::Export for AccessibilityScrollHint {
    
}
impl crate::meta::Element for AccessibilityScrollHint {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseMode {
    ord: i32
}
impl MouseMode {
    #[doc(alias = "MOUSE_MODE_VISIBLE")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_VISIBLE`"]
    pub const VISIBLE: MouseMode = MouseMode {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_MODE_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_HIDDEN`"]
    pub const HIDDEN: MouseMode = MouseMode {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_MODE_CAPTURED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CAPTURED`"]
    pub const CAPTURED: MouseMode = MouseMode {
        ord: 2i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED`"]
    pub const CONFINED: MouseMode = MouseMode {
        ord: 3i32
    };
    #[doc(alias = "MOUSE_MODE_CONFINED_HIDDEN")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_CONFINED_HIDDEN`"]
    pub const CONFINED_HIDDEN: MouseMode = MouseMode {
        ord: 4i32
    };
    #[doc(alias = "MOUSE_MODE_MAX")]
    #[doc = "Godot enumerator name: `MOUSE_MODE_MAX`"]
    pub const MAX: MouseMode = MouseMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for MouseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::VISIBLE => "VISIBLE", Self::HIDDEN => "HIDDEN", Self::CAPTURED => "CAPTURED", Self::CONFINED => "CONFINED", Self::CONFINED_HIDDEN => "CONFINED_HIDDEN", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MouseMode::VISIBLE, MouseMode::HIDDEN, MouseMode::CAPTURED, MouseMode::CONFINED, MouseMode::CONFINED_HIDDEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MouseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "MOUSE_MODE_VISIBLE", MouseMode::VISIBLE), crate::meta::inspect::EnumConstant::new("HIDDEN", "MOUSE_MODE_HIDDEN", MouseMode::HIDDEN), crate::meta::inspect::EnumConstant::new("CAPTURED", "MOUSE_MODE_CAPTURED", MouseMode::CAPTURED), crate::meta::inspect::EnumConstant::new("CONFINED", "MOUSE_MODE_CONFINED", MouseMode::CONFINED), crate::meta::inspect::EnumConstant::new("CONFINED_HIDDEN", "MOUSE_MODE_CONFINED_HIDDEN", MouseMode::CONFINED_HIDDEN), crate::meta::inspect::EnumConstant::new("MAX", "MOUSE_MODE_MAX", MouseMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MouseMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for MouseMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mouse Mode Visible", 0i64), EnumeratorShape::new_int("Mouse Mode Hidden", 1i64), EnumeratorShape::new_int("Mouse Mode Captured", 2i64), EnumeratorShape::new_int("Mouse Mode Confined", 3i64), EnumeratorShape::new_int("Mouse Mode Confined Hidden", 4i64), EnumeratorShape::new_int("Mouse Mode Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.MouseMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MouseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MouseMode {
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
impl crate::registry::property::Export for MouseMode {
    
}
impl crate::meta::Element for MouseMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ScreenOrientation {
    ord: i32
}
impl ScreenOrientation {
    #[doc(alias = "SCREEN_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_LANDSCAPE`"]
    pub const LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 0i32
    };
    #[doc(alias = "SCREEN_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_PORTRAIT`"]
    pub const PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 1i32
    };
    #[doc(alias = "SCREEN_REVERSE_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_LANDSCAPE`"]
    pub const REVERSE_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 2i32
    };
    #[doc(alias = "SCREEN_REVERSE_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_REVERSE_PORTRAIT`"]
    pub const REVERSE_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 3i32
    };
    #[doc(alias = "SCREEN_SENSOR_LANDSCAPE")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_LANDSCAPE`"]
    pub const SENSOR_LANDSCAPE: ScreenOrientation = ScreenOrientation {
        ord: 4i32
    };
    #[doc(alias = "SCREEN_SENSOR_PORTRAIT")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR_PORTRAIT`"]
    pub const SENSOR_PORTRAIT: ScreenOrientation = ScreenOrientation {
        ord: 5i32
    };
    #[doc(alias = "SCREEN_SENSOR")]
    #[doc = "Godot enumerator name: `SCREEN_SENSOR`"]
    pub const SENSOR: ScreenOrientation = ScreenOrientation {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ScreenOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScreenOrientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScreenOrientation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::LANDSCAPE => "LANDSCAPE", Self::PORTRAIT => "PORTRAIT", Self::REVERSE_LANDSCAPE => "REVERSE_LANDSCAPE", Self::REVERSE_PORTRAIT => "REVERSE_PORTRAIT", Self::SENSOR_LANDSCAPE => "SENSOR_LANDSCAPE", Self::SENSOR_PORTRAIT => "SENSOR_PORTRAIT", Self::SENSOR => "SENSOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScreenOrientation::LANDSCAPE, ScreenOrientation::PORTRAIT, ScreenOrientation::REVERSE_LANDSCAPE, ScreenOrientation::REVERSE_PORTRAIT, ScreenOrientation::SENSOR_LANDSCAPE, ScreenOrientation::SENSOR_PORTRAIT, ScreenOrientation::SENSOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScreenOrientation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LANDSCAPE", "SCREEN_LANDSCAPE", ScreenOrientation::LANDSCAPE), crate::meta::inspect::EnumConstant::new("PORTRAIT", "SCREEN_PORTRAIT", ScreenOrientation::PORTRAIT), crate::meta::inspect::EnumConstant::new("REVERSE_LANDSCAPE", "SCREEN_REVERSE_LANDSCAPE", ScreenOrientation::REVERSE_LANDSCAPE), crate::meta::inspect::EnumConstant::new("REVERSE_PORTRAIT", "SCREEN_REVERSE_PORTRAIT", ScreenOrientation::REVERSE_PORTRAIT), crate::meta::inspect::EnumConstant::new("SENSOR_LANDSCAPE", "SCREEN_SENSOR_LANDSCAPE", ScreenOrientation::SENSOR_LANDSCAPE), crate::meta::inspect::EnumConstant::new("SENSOR_PORTRAIT", "SCREEN_SENSOR_PORTRAIT", ScreenOrientation::SENSOR_PORTRAIT), crate::meta::inspect::EnumConstant::new("SENSOR", "SCREEN_SENSOR", ScreenOrientation::SENSOR)]
        }
    }
}
impl crate::meta::GodotConvert for ScreenOrientation {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Screen Landscape", 0i64), EnumeratorShape::new_int("Screen Portrait", 1i64), EnumeratorShape::new_int("Screen Reverse Landscape", 2i64), EnumeratorShape::new_int("Screen Reverse Portrait", 3i64), EnumeratorShape::new_int("Screen Sensor Landscape", 4i64), EnumeratorShape::new_int("Screen Sensor Portrait", 5i64), EnumeratorShape::new_int("Screen Sensor", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.ScreenOrientation")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ScreenOrientation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScreenOrientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ScreenOrientation {
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
impl crate::registry::property::Export for ScreenOrientation {
    
}
impl crate::meta::Element for ScreenOrientation {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    #[doc(alias = "KEYBOARD_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_DEFAULT`"]
    pub const DEFAULT: VirtualKeyboardType = VirtualKeyboardType {
        ord: 0i32
    };
    #[doc(alias = "KEYBOARD_TYPE_MULTILINE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_MULTILINE`"]
    pub const MULTILINE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 1i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER`"]
    pub const NUMBER: VirtualKeyboardType = VirtualKeyboardType {
        ord: 2i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER_DECIMAL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER_DECIMAL`"]
    pub const NUMBER_DECIMAL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 3i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PHONE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PHONE`"]
    pub const PHONE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 4i32
    };
    #[doc(alias = "KEYBOARD_TYPE_EMAIL_ADDRESS")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_EMAIL_ADDRESS`"]
    pub const EMAIL_ADDRESS: VirtualKeyboardType = VirtualKeyboardType {
        ord: 5i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PASSWORD")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PASSWORD`"]
    pub const PASSWORD: VirtualKeyboardType = VirtualKeyboardType {
        ord: 6i32
    };
    #[doc(alias = "KEYBOARD_TYPE_URL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_URL`"]
    pub const URL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for VirtualKeyboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VirtualKeyboardType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::MULTILINE => "MULTILINE", Self::NUMBER => "NUMBER", Self::NUMBER_DECIMAL => "NUMBER_DECIMAL", Self::PHONE => "PHONE", Self::EMAIL_ADDRESS => "EMAIL_ADDRESS", Self::PASSWORD => "PASSWORD", Self::URL => "URL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VirtualKeyboardType::DEFAULT, VirtualKeyboardType::MULTILINE, VirtualKeyboardType::NUMBER, VirtualKeyboardType::NUMBER_DECIMAL, VirtualKeyboardType::PHONE, VirtualKeyboardType::EMAIL_ADDRESS, VirtualKeyboardType::PASSWORD, VirtualKeyboardType::URL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VirtualKeyboardType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "KEYBOARD_TYPE_DEFAULT", VirtualKeyboardType::DEFAULT), crate::meta::inspect::EnumConstant::new("MULTILINE", "KEYBOARD_TYPE_MULTILINE", VirtualKeyboardType::MULTILINE), crate::meta::inspect::EnumConstant::new("NUMBER", "KEYBOARD_TYPE_NUMBER", VirtualKeyboardType::NUMBER), crate::meta::inspect::EnumConstant::new("NUMBER_DECIMAL", "KEYBOARD_TYPE_NUMBER_DECIMAL", VirtualKeyboardType::NUMBER_DECIMAL), crate::meta::inspect::EnumConstant::new("PHONE", "KEYBOARD_TYPE_PHONE", VirtualKeyboardType::PHONE), crate::meta::inspect::EnumConstant::new("EMAIL_ADDRESS", "KEYBOARD_TYPE_EMAIL_ADDRESS", VirtualKeyboardType::EMAIL_ADDRESS), crate::meta::inspect::EnumConstant::new("PASSWORD", "KEYBOARD_TYPE_PASSWORD", VirtualKeyboardType::PASSWORD), crate::meta::inspect::EnumConstant::new("URL", "KEYBOARD_TYPE_URL", VirtualKeyboardType::URL)]
        }
    }
}
impl crate::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Keyboard Type Default", 0i64), EnumeratorShape::new_int("Keyboard Type Multiline", 1i64), EnumeratorShape::new_int("Keyboard Type Number", 2i64), EnumeratorShape::new_int("Keyboard Type Number Decimal", 3i64), EnumeratorShape::new_int("Keyboard Type Phone", 4i64), EnumeratorShape::new_int("Keyboard Type Email Address", 5i64), EnumeratorShape::new_int("Keyboard Type Password", 6i64), EnumeratorShape::new_int("Keyboard Type Url", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.VirtualKeyboardType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VirtualKeyboardType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VirtualKeyboardType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VirtualKeyboardType {
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
impl crate::registry::property::Export for VirtualKeyboardType {
    
}
impl crate::meta::Element for VirtualKeyboardType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    #[doc(alias = "CURSOR_MAX")]
    #[doc = "Godot enumerator name: `CURSOR_MAX`"]
    pub const MAX: CursorShape = CursorShape {
        ord: 17i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CursorShape::ARROW, CursorShape::IBEAM, CursorShape::POINTING_HAND, CursorShape::CROSS, CursorShape::WAIT, CursorShape::BUSY, CursorShape::DRAG, CursorShape::CAN_DROP, CursorShape::FORBIDDEN, CursorShape::VSIZE, CursorShape::HSIZE, CursorShape::BDIAGSIZE, CursorShape::FDIAGSIZE, CursorShape::MOVE, CursorShape::VSPLIT, CursorShape::HSPLIT, CursorShape::HELP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CursorShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ARROW", "CURSOR_ARROW", CursorShape::ARROW), crate::meta::inspect::EnumConstant::new("IBEAM", "CURSOR_IBEAM", CursorShape::IBEAM), crate::meta::inspect::EnumConstant::new("POINTING_HAND", "CURSOR_POINTING_HAND", CursorShape::POINTING_HAND), crate::meta::inspect::EnumConstant::new("CROSS", "CURSOR_CROSS", CursorShape::CROSS), crate::meta::inspect::EnumConstant::new("WAIT", "CURSOR_WAIT", CursorShape::WAIT), crate::meta::inspect::EnumConstant::new("BUSY", "CURSOR_BUSY", CursorShape::BUSY), crate::meta::inspect::EnumConstant::new("DRAG", "CURSOR_DRAG", CursorShape::DRAG), crate::meta::inspect::EnumConstant::new("CAN_DROP", "CURSOR_CAN_DROP", CursorShape::CAN_DROP), crate::meta::inspect::EnumConstant::new("FORBIDDEN", "CURSOR_FORBIDDEN", CursorShape::FORBIDDEN), crate::meta::inspect::EnumConstant::new("VSIZE", "CURSOR_VSIZE", CursorShape::VSIZE), crate::meta::inspect::EnumConstant::new("HSIZE", "CURSOR_HSIZE", CursorShape::HSIZE), crate::meta::inspect::EnumConstant::new("BDIAGSIZE", "CURSOR_BDIAGSIZE", CursorShape::BDIAGSIZE), crate::meta::inspect::EnumConstant::new("FDIAGSIZE", "CURSOR_FDIAGSIZE", CursorShape::FDIAGSIZE), crate::meta::inspect::EnumConstant::new("MOVE", "CURSOR_MOVE", CursorShape::MOVE), crate::meta::inspect::EnumConstant::new("VSPLIT", "CURSOR_VSPLIT", CursorShape::VSPLIT), crate::meta::inspect::EnumConstant::new("HSPLIT", "CURSOR_HSPLIT", CursorShape::HSPLIT), crate::meta::inspect::EnumConstant::new("HELP", "CURSOR_HELP", CursorShape::HELP), crate::meta::inspect::EnumConstant::new("MAX", "CURSOR_MAX", CursorShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CursorShape {
    const ENUMERATOR_COUNT: usize = 17usize;
    
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cursor Arrow", 0i64), EnumeratorShape::new_int("Cursor Ibeam", 1i64), EnumeratorShape::new_int("Cursor Pointing Hand", 2i64), EnumeratorShape::new_int("Cursor Cross", 3i64), EnumeratorShape::new_int("Cursor Wait", 4i64), EnumeratorShape::new_int("Cursor Busy", 5i64), EnumeratorShape::new_int("Cursor Drag", 6i64), EnumeratorShape::new_int("Cursor Can Drop", 7i64), EnumeratorShape::new_int("Cursor Forbidden", 8i64), EnumeratorShape::new_int("Cursor Vsize", 9i64), EnumeratorShape::new_int("Cursor Hsize", 10i64), EnumeratorShape::new_int("Cursor Bdiagsize", 11i64), EnumeratorShape::new_int("Cursor Fdiagsize", 12i64), EnumeratorShape::new_int("Cursor Move", 13i64), EnumeratorShape::new_int("Cursor Vsplit", 14i64), EnumeratorShape::new_int("Cursor Hsplit", 15i64), EnumeratorShape::new_int("Cursor Help", 16i64), EnumeratorShape::new_int("Cursor Max", 17i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.CursorShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CursorShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CursorShape {
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
impl crate::registry::property::Export for CursorShape {
    
}
impl crate::meta::Element for CursorShape {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileDialogMode {
    ord: i32
}
impl FileDialogMode {
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileDialogMode = FileDialogMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileDialogMode = FileDialogMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileDialogMode = FileDialogMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileDialogMode = FileDialogMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_DIALOG_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_DIALOG_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileDialogMode = FileDialogMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileDialogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileDialogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileDialogMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FileDialogMode::OPEN_FILE, FileDialogMode::OPEN_FILES, FileDialogMode::OPEN_DIR, FileDialogMode::OPEN_ANY, FileDialogMode::SAVE_FILE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FileDialogMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPEN_FILE", "FILE_DIALOG_MODE_OPEN_FILE", FileDialogMode::OPEN_FILE), crate::meta::inspect::EnumConstant::new("OPEN_FILES", "FILE_DIALOG_MODE_OPEN_FILES", FileDialogMode::OPEN_FILES), crate::meta::inspect::EnumConstant::new("OPEN_DIR", "FILE_DIALOG_MODE_OPEN_DIR", FileDialogMode::OPEN_DIR), crate::meta::inspect::EnumConstant::new("OPEN_ANY", "FILE_DIALOG_MODE_OPEN_ANY", FileDialogMode::OPEN_ANY), crate::meta::inspect::EnumConstant::new("SAVE_FILE", "FILE_DIALOG_MODE_SAVE_FILE", FileDialogMode::SAVE_FILE)]
        }
    }
}
impl crate::meta::GodotConvert for FileDialogMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("File Dialog Mode Open File", 0i64), EnumeratorShape::new_int("File Dialog Mode Open Files", 1i64), EnumeratorShape::new_int("File Dialog Mode Open Dir", 2i64), EnumeratorShape::new_int("File Dialog Mode Open Any", 3i64), EnumeratorShape::new_int("File Dialog Mode Save File", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.FileDialogMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FileDialogMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileDialogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FileDialogMode {
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
impl crate::registry::property::Export for FileDialogMode {
    
}
impl crate::meta::Element for FileDialogMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowMode {
    ord: i32
}
impl WindowMode {
    #[doc(alias = "WINDOW_MODE_WINDOWED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_WINDOWED`"]
    pub const WINDOWED: WindowMode = WindowMode {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_MODE_MINIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MINIMIZED`"]
    pub const MINIMIZED: WindowMode = WindowMode {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_MODE_MAXIMIZED")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_MAXIMIZED`"]
    pub const MAXIMIZED: WindowMode = WindowMode {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_MODE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_FULLSCREEN`"]
    pub const FULLSCREEN: WindowMode = WindowMode {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_MODE_EXCLUSIVE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_MODE_EXCLUSIVE_FULLSCREEN`"]
    pub const EXCLUSIVE_FULLSCREEN: WindowMode = WindowMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for WindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::WINDOWED => "WINDOWED", Self::MINIMIZED => "MINIMIZED", Self::MAXIMIZED => "MAXIMIZED", Self::FULLSCREEN => "FULLSCREEN", Self::EXCLUSIVE_FULLSCREEN => "EXCLUSIVE_FULLSCREEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowMode::WINDOWED, WindowMode::MINIMIZED, WindowMode::MAXIMIZED, WindowMode::FULLSCREEN, WindowMode::EXCLUSIVE_FULLSCREEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WINDOWED", "WINDOW_MODE_WINDOWED", WindowMode::WINDOWED), crate::meta::inspect::EnumConstant::new("MINIMIZED", "WINDOW_MODE_MINIMIZED", WindowMode::MINIMIZED), crate::meta::inspect::EnumConstant::new("MAXIMIZED", "WINDOW_MODE_MAXIMIZED", WindowMode::MAXIMIZED), crate::meta::inspect::EnumConstant::new("FULLSCREEN", "WINDOW_MODE_FULLSCREEN", WindowMode::FULLSCREEN), crate::meta::inspect::EnumConstant::new("EXCLUSIVE_FULLSCREEN", "WINDOW_MODE_EXCLUSIVE_FULLSCREEN", WindowMode::EXCLUSIVE_FULLSCREEN)]
        }
    }
}
impl crate::meta::GodotConvert for WindowMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Window Mode Windowed", 0i64), EnumeratorShape::new_int("Window Mode Minimized", 1i64), EnumeratorShape::new_int("Window Mode Maximized", 2i64), EnumeratorShape::new_int("Window Mode Fullscreen", 3i64), EnumeratorShape::new_int("Window Mode Exclusive Fullscreen", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.WindowMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for WindowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for WindowMode {
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
impl crate::registry::property::Export for WindowMode {
    
}
impl crate::meta::Element for WindowMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowFlags {
    ord: i32
}
impl WindowFlags {
    #[doc(alias = "WINDOW_FLAG_RESIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_RESIZE_DISABLED`"]
    pub const RESIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_FLAG_BORDERLESS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_BORDERLESS`"]
    pub const BORDERLESS: WindowFlags = WindowFlags {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_FLAG_ALWAYS_ON_TOP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_ALWAYS_ON_TOP`"]
    pub const ALWAYS_ON_TOP: WindowFlags = WindowFlags {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: WindowFlags = WindowFlags {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_FLAG_NO_FOCUS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_NO_FOCUS`"]
    pub const NO_FOCUS: WindowFlags = WindowFlags {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_FLAG_POPUP")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_POPUP`"]
    pub const POPUP: WindowFlags = WindowFlags {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_FLAG_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: WindowFlags = WindowFlags {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_FLAG_MOUSE_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MOUSE_PASSTHROUGH`"]
    pub const MOUSE_PASSTHROUGH: WindowFlags = WindowFlags {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_FLAG_SHARP_CORNERS")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_SHARP_CORNERS`"]
    pub const SHARP_CORNERS: WindowFlags = WindowFlags {
        ord: 8i32
    };
    #[doc(alias = "WINDOW_FLAG_EXCLUDE_FROM_CAPTURE")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_EXCLUDE_FROM_CAPTURE`"]
    pub const EXCLUDE_FROM_CAPTURE: WindowFlags = WindowFlags {
        ord: 9i32
    };
    #[doc(alias = "WINDOW_FLAG_POPUP_WM_HINT")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_POPUP_WM_HINT`"]
    pub const POPUP_WM_HINT: WindowFlags = WindowFlags {
        ord: 10i32
    };
    #[doc(alias = "WINDOW_FLAG_MINIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MINIMIZE_DISABLED`"]
    pub const MINIMIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 11i32
    };
    #[doc(alias = "WINDOW_FLAG_MAXIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MAXIMIZE_DISABLED`"]
    pub const MAXIMIZE_DISABLED: WindowFlags = WindowFlags {
        ord: 12i32
    };
    #[doc(alias = "WINDOW_FLAG_MAX")]
    #[doc = "Godot enumerator name: `WINDOW_FLAG_MAX`"]
    pub const MAX: WindowFlags = WindowFlags {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for WindowFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::RESIZE_DISABLED => "RESIZE_DISABLED", Self::BORDERLESS => "BORDERLESS", Self::ALWAYS_ON_TOP => "ALWAYS_ON_TOP", Self::TRANSPARENT => "TRANSPARENT", Self::NO_FOCUS => "NO_FOCUS", Self::POPUP => "POPUP", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::MOUSE_PASSTHROUGH => "MOUSE_PASSTHROUGH", Self::SHARP_CORNERS => "SHARP_CORNERS", Self::EXCLUDE_FROM_CAPTURE => "EXCLUDE_FROM_CAPTURE", Self::POPUP_WM_HINT => "POPUP_WM_HINT", Self::MINIMIZE_DISABLED => "MINIMIZE_DISABLED", Self::MAXIMIZE_DISABLED => "MAXIMIZE_DISABLED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowFlags::RESIZE_DISABLED, WindowFlags::BORDERLESS, WindowFlags::ALWAYS_ON_TOP, WindowFlags::TRANSPARENT, WindowFlags::NO_FOCUS, WindowFlags::POPUP, WindowFlags::EXTEND_TO_TITLE, WindowFlags::MOUSE_PASSTHROUGH, WindowFlags::SHARP_CORNERS, WindowFlags::EXCLUDE_FROM_CAPTURE, WindowFlags::POPUP_WM_HINT, WindowFlags::MINIMIZE_DISABLED, WindowFlags::MAXIMIZE_DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESIZE_DISABLED", "WINDOW_FLAG_RESIZE_DISABLED", WindowFlags::RESIZE_DISABLED), crate::meta::inspect::EnumConstant::new("BORDERLESS", "WINDOW_FLAG_BORDERLESS", WindowFlags::BORDERLESS), crate::meta::inspect::EnumConstant::new("ALWAYS_ON_TOP", "WINDOW_FLAG_ALWAYS_ON_TOP", WindowFlags::ALWAYS_ON_TOP), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "WINDOW_FLAG_TRANSPARENT", WindowFlags::TRANSPARENT), crate::meta::inspect::EnumConstant::new("NO_FOCUS", "WINDOW_FLAG_NO_FOCUS", WindowFlags::NO_FOCUS), crate::meta::inspect::EnumConstant::new("POPUP", "WINDOW_FLAG_POPUP", WindowFlags::POPUP), crate::meta::inspect::EnumConstant::new("EXTEND_TO_TITLE", "WINDOW_FLAG_EXTEND_TO_TITLE", WindowFlags::EXTEND_TO_TITLE), crate::meta::inspect::EnumConstant::new("MOUSE_PASSTHROUGH", "WINDOW_FLAG_MOUSE_PASSTHROUGH", WindowFlags::MOUSE_PASSTHROUGH), crate::meta::inspect::EnumConstant::new("SHARP_CORNERS", "WINDOW_FLAG_SHARP_CORNERS", WindowFlags::SHARP_CORNERS), crate::meta::inspect::EnumConstant::new("EXCLUDE_FROM_CAPTURE", "WINDOW_FLAG_EXCLUDE_FROM_CAPTURE", WindowFlags::EXCLUDE_FROM_CAPTURE), crate::meta::inspect::EnumConstant::new("POPUP_WM_HINT", "WINDOW_FLAG_POPUP_WM_HINT", WindowFlags::POPUP_WM_HINT), crate::meta::inspect::EnumConstant::new("MINIMIZE_DISABLED", "WINDOW_FLAG_MINIMIZE_DISABLED", WindowFlags::MINIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAXIMIZE_DISABLED", "WINDOW_FLAG_MAXIMIZE_DISABLED", WindowFlags::MAXIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAX", "WINDOW_FLAG_MAX", WindowFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for WindowFlags {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for WindowFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Window Flag Resize Disabled", 0i64), EnumeratorShape::new_int("Window Flag Borderless", 1i64), EnumeratorShape::new_int("Window Flag Always On Top", 2i64), EnumeratorShape::new_int("Window Flag Transparent", 3i64), EnumeratorShape::new_int("Window Flag No Focus", 4i64), EnumeratorShape::new_int("Window Flag Popup", 5i64), EnumeratorShape::new_int("Window Flag Extend To Title", 6i64), EnumeratorShape::new_int("Window Flag Mouse Passthrough", 7i64), EnumeratorShape::new_int("Window Flag Sharp Corners", 8i64), EnumeratorShape::new_int("Window Flag Exclude From Capture", 9i64), EnumeratorShape::new_int("Window Flag Popup Wm Hint", 10i64), EnumeratorShape::new_int("Window Flag Minimize Disabled", 11i64), EnumeratorShape::new_int("Window Flag Maximize Disabled", 12i64), EnumeratorShape::new_int("Window Flag Max", 13i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.WindowFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for WindowFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for WindowFlags {
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
impl crate::registry::property::Export for WindowFlags {
    
}
impl crate::meta::Element for WindowFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowEvent {
    ord: i32
}
impl WindowEvent {
    #[doc(alias = "WINDOW_EVENT_MOUSE_ENTER")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_ENTER`"]
    pub const MOUSE_ENTER: WindowEvent = WindowEvent {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_EVENT_MOUSE_EXIT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_MOUSE_EXIT`"]
    pub const MOUSE_EXIT: WindowEvent = WindowEvent {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_IN")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_IN`"]
    pub const FOCUS_IN: WindowEvent = WindowEvent {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_EVENT_FOCUS_OUT")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FOCUS_OUT`"]
    pub const FOCUS_OUT: WindowEvent = WindowEvent {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_EVENT_CLOSE_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_CLOSE_REQUEST`"]
    pub const CLOSE_REQUEST: WindowEvent = WindowEvent {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_EVENT_GO_BACK_REQUEST")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_GO_BACK_REQUEST`"]
    pub const GO_BACK_REQUEST: WindowEvent = WindowEvent {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_EVENT_DPI_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_DPI_CHANGE`"]
    pub const DPI_CHANGE: WindowEvent = WindowEvent {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_EVENT_TITLEBAR_CHANGE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_TITLEBAR_CHANGE`"]
    pub const TITLEBAR_CHANGE: WindowEvent = WindowEvent {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_EVENT_FORCE_CLOSE")]
    #[doc = "Godot enumerator name: `WINDOW_EVENT_FORCE_CLOSE`"]
    pub const FORCE_CLOSE: WindowEvent = WindowEvent {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowEvent {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::MOUSE_ENTER => "MOUSE_ENTER", Self::MOUSE_EXIT => "MOUSE_EXIT", Self::FOCUS_IN => "FOCUS_IN", Self::FOCUS_OUT => "FOCUS_OUT", Self::CLOSE_REQUEST => "CLOSE_REQUEST", Self::GO_BACK_REQUEST => "GO_BACK_REQUEST", Self::DPI_CHANGE => "DPI_CHANGE", Self::TITLEBAR_CHANGE => "TITLEBAR_CHANGE", Self::FORCE_CLOSE => "FORCE_CLOSE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowEvent::MOUSE_ENTER, WindowEvent::MOUSE_EXIT, WindowEvent::FOCUS_IN, WindowEvent::FOCUS_OUT, WindowEvent::CLOSE_REQUEST, WindowEvent::GO_BACK_REQUEST, WindowEvent::DPI_CHANGE, WindowEvent::TITLEBAR_CHANGE, WindowEvent::FORCE_CLOSE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowEvent >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MOUSE_ENTER", "WINDOW_EVENT_MOUSE_ENTER", WindowEvent::MOUSE_ENTER), crate::meta::inspect::EnumConstant::new("MOUSE_EXIT", "WINDOW_EVENT_MOUSE_EXIT", WindowEvent::MOUSE_EXIT), crate::meta::inspect::EnumConstant::new("FOCUS_IN", "WINDOW_EVENT_FOCUS_IN", WindowEvent::FOCUS_IN), crate::meta::inspect::EnumConstant::new("FOCUS_OUT", "WINDOW_EVENT_FOCUS_OUT", WindowEvent::FOCUS_OUT), crate::meta::inspect::EnumConstant::new("CLOSE_REQUEST", "WINDOW_EVENT_CLOSE_REQUEST", WindowEvent::CLOSE_REQUEST), crate::meta::inspect::EnumConstant::new("GO_BACK_REQUEST", "WINDOW_EVENT_GO_BACK_REQUEST", WindowEvent::GO_BACK_REQUEST), crate::meta::inspect::EnumConstant::new("DPI_CHANGE", "WINDOW_EVENT_DPI_CHANGE", WindowEvent::DPI_CHANGE), crate::meta::inspect::EnumConstant::new("TITLEBAR_CHANGE", "WINDOW_EVENT_TITLEBAR_CHANGE", WindowEvent::TITLEBAR_CHANGE), crate::meta::inspect::EnumConstant::new("FORCE_CLOSE", "WINDOW_EVENT_FORCE_CLOSE", WindowEvent::FORCE_CLOSE)]
        }
    }
}
impl crate::meta::GodotConvert for WindowEvent {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Window Event Mouse Enter", 0i64), EnumeratorShape::new_int("Window Event Mouse Exit", 1i64), EnumeratorShape::new_int("Window Event Focus In", 2i64), EnumeratorShape::new_int("Window Event Focus Out", 3i64), EnumeratorShape::new_int("Window Event Close Request", 4i64), EnumeratorShape::new_int("Window Event Go Back Request", 5i64), EnumeratorShape::new_int("Window Event Dpi Change", 6i64), EnumeratorShape::new_int("Window Event Titlebar Change", 7i64), EnumeratorShape::new_int("Window Event Force Close", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.WindowEvent")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for WindowEvent {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for WindowEvent {
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
impl crate::registry::property::Export for WindowEvent {
    
}
impl crate::meta::Element for WindowEvent {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowResizeEdge {
    ord: i32
}
impl WindowResizeEdge {
    #[doc(alias = "WINDOW_EDGE_TOP_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP_LEFT`"]
    pub const TOP_LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_EDGE_TOP")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP`"]
    pub const TOP: WindowResizeEdge = WindowResizeEdge {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_EDGE_TOP_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_TOP_RIGHT`"]
    pub const TOP_RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_EDGE_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_LEFT`"]
    pub const LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_EDGE_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_RIGHT`"]
    pub const RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM_LEFT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM_LEFT`"]
    pub const BOTTOM_LEFT: WindowResizeEdge = WindowResizeEdge {
        ord: 5i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM`"]
    pub const BOTTOM: WindowResizeEdge = WindowResizeEdge {
        ord: 6i32
    };
    #[doc(alias = "WINDOW_EDGE_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: WindowResizeEdge = WindowResizeEdge {
        ord: 7i32
    };
    #[doc(alias = "WINDOW_EDGE_MAX")]
    #[doc = "Godot enumerator name: `WINDOW_EDGE_MAX`"]
    pub const MAX: WindowResizeEdge = WindowResizeEdge {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for WindowResizeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowResizeEdge") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowResizeEdge {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::TOP => "TOP", Self::TOP_RIGHT => "TOP_RIGHT", Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::BOTTOM_LEFT => "BOTTOM_LEFT", Self::BOTTOM => "BOTTOM", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowResizeEdge::TOP_LEFT, WindowResizeEdge::TOP, WindowResizeEdge::TOP_RIGHT, WindowResizeEdge::LEFT, WindowResizeEdge::RIGHT, WindowResizeEdge::BOTTOM_LEFT, WindowResizeEdge::BOTTOM, WindowResizeEdge::BOTTOM_RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowResizeEdge >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP_LEFT", "WINDOW_EDGE_TOP_LEFT", WindowResizeEdge::TOP_LEFT), crate::meta::inspect::EnumConstant::new("TOP", "WINDOW_EDGE_TOP", WindowResizeEdge::TOP), crate::meta::inspect::EnumConstant::new("TOP_RIGHT", "WINDOW_EDGE_TOP_RIGHT", WindowResizeEdge::TOP_RIGHT), crate::meta::inspect::EnumConstant::new("LEFT", "WINDOW_EDGE_LEFT", WindowResizeEdge::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "WINDOW_EDGE_RIGHT", WindowResizeEdge::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT", "WINDOW_EDGE_BOTTOM_LEFT", WindowResizeEdge::BOTTOM_LEFT), crate::meta::inspect::EnumConstant::new("BOTTOM", "WINDOW_EDGE_BOTTOM", WindowResizeEdge::BOTTOM), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "WINDOW_EDGE_BOTTOM_RIGHT", WindowResizeEdge::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "WINDOW_EDGE_MAX", WindowResizeEdge::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for WindowResizeEdge {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for WindowResizeEdge {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Window Edge Top Left", 0i64), EnumeratorShape::new_int("Window Edge Top", 1i64), EnumeratorShape::new_int("Window Edge Top Right", 2i64), EnumeratorShape::new_int("Window Edge Left", 3i64), EnumeratorShape::new_int("Window Edge Right", 4i64), EnumeratorShape::new_int("Window Edge Bottom Left", 5i64), EnumeratorShape::new_int("Window Edge Bottom", 6i64), EnumeratorShape::new_int("Window Edge Bottom Right", 7i64), EnumeratorShape::new_int("Window Edge Max", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.WindowResizeEdge")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for WindowResizeEdge {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowResizeEdge {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for WindowResizeEdge {
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
impl crate::registry::property::Export for WindowResizeEdge {
    
}
impl crate::meta::Element for WindowResizeEdge {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VSyncMode {
    ord: i32
}
impl VSyncMode {
    #[doc(alias = "VSYNC_DISABLED")]
    #[doc = "Godot enumerator name: `VSYNC_DISABLED`"]
    pub const DISABLED: VSyncMode = VSyncMode {
        ord: 0i32
    };
    #[doc(alias = "VSYNC_ENABLED")]
    #[doc = "Godot enumerator name: `VSYNC_ENABLED`"]
    pub const ENABLED: VSyncMode = VSyncMode {
        ord: 1i32
    };
    #[doc(alias = "VSYNC_ADAPTIVE")]
    #[doc = "Godot enumerator name: `VSYNC_ADAPTIVE`"]
    pub const ADAPTIVE: VSyncMode = VSyncMode {
        ord: 2i32
    };
    #[doc(alias = "VSYNC_MAILBOX")]
    #[doc = "Godot enumerator name: `VSYNC_MAILBOX`"]
    pub const MAILBOX: VSyncMode = VSyncMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VSyncMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VSyncMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VSyncMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::ADAPTIVE => "ADAPTIVE", Self::MAILBOX => "MAILBOX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VSyncMode::DISABLED, VSyncMode::ENABLED, VSyncMode::ADAPTIVE, VSyncMode::MAILBOX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VSyncMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VSYNC_DISABLED", VSyncMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "VSYNC_ENABLED", VSyncMode::ENABLED), crate::meta::inspect::EnumConstant::new("ADAPTIVE", "VSYNC_ADAPTIVE", VSyncMode::ADAPTIVE), crate::meta::inspect::EnumConstant::new("MAILBOX", "VSYNC_MAILBOX", VSyncMode::MAILBOX)]
        }
    }
}
impl crate::meta::GodotConvert for VSyncMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Vsync Disabled", 0i64), EnumeratorShape::new_int("Vsync Enabled", 1i64), EnumeratorShape::new_int("Vsync Adaptive", 2i64), EnumeratorShape::new_int("Vsync Mailbox", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.VSyncMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VSyncMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VSyncMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VSyncMode {
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
impl crate::registry::property::Export for VSyncMode {
    
}
impl crate::meta::Element for VSyncMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandleType {
    ord: i32
}
impl HandleType {
    pub const DISPLAY_HANDLE: HandleType = HandleType {
        ord: 0i32
    };
    pub const WINDOW_HANDLE: HandleType = HandleType {
        ord: 1i32
    };
    pub const WINDOW_VIEW: HandleType = HandleType {
        ord: 2i32
    };
    pub const OPENGL_CONTEXT: HandleType = HandleType {
        ord: 3i32
    };
    pub const EGL_DISPLAY: HandleType = HandleType {
        ord: 4i32
    };
    pub const EGL_CONFIG: HandleType = HandleType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for HandleType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandleType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandleType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::DISPLAY_HANDLE => "DISPLAY_HANDLE", Self::WINDOW_HANDLE => "WINDOW_HANDLE", Self::WINDOW_VIEW => "WINDOW_VIEW", Self::OPENGL_CONTEXT => "OPENGL_CONTEXT", Self::EGL_DISPLAY => "EGL_DISPLAY", Self::EGL_CONFIG => "EGL_CONFIG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandleType::DISPLAY_HANDLE, HandleType::WINDOW_HANDLE, HandleType::WINDOW_VIEW, HandleType::OPENGL_CONTEXT, HandleType::EGL_DISPLAY, HandleType::EGL_CONFIG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandleType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISPLAY_HANDLE", "DISPLAY_HANDLE", HandleType::DISPLAY_HANDLE), crate::meta::inspect::EnumConstant::new("WINDOW_HANDLE", "WINDOW_HANDLE", HandleType::WINDOW_HANDLE), crate::meta::inspect::EnumConstant::new("WINDOW_VIEW", "WINDOW_VIEW", HandleType::WINDOW_VIEW), crate::meta::inspect::EnumConstant::new("OPENGL_CONTEXT", "OPENGL_CONTEXT", HandleType::OPENGL_CONTEXT), crate::meta::inspect::EnumConstant::new("EGL_DISPLAY", "EGL_DISPLAY", HandleType::EGL_DISPLAY), crate::meta::inspect::EnumConstant::new("EGL_CONFIG", "EGL_CONFIG", HandleType::EGL_CONFIG)]
        }
    }
}
impl crate::meta::GodotConvert for HandleType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Display Handle", 0i64), EnumeratorShape::new_int("Window Handle", 1i64), EnumeratorShape::new_int("Window View", 2i64), EnumeratorShape::new_int("Opengl Context", 3i64), EnumeratorShape::new_int("Egl Display", 4i64), EnumeratorShape::new_int("Egl Config", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.HandleType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for HandleType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandleType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for HandleType {
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
impl crate::registry::property::Export for HandleType {
    
}
impl crate::meta::Element for HandleType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `TTSUtteranceEvent`."]
pub struct TtsUtteranceEvent {
    ord: i32
}
impl TtsUtteranceEvent {
    #[doc(alias = "TTS_UTTERANCE_STARTED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_STARTED`"]
    pub const STARTED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 0i32
    };
    #[doc(alias = "TTS_UTTERANCE_ENDED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_ENDED`"]
    pub const ENDED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 1i32
    };
    #[doc(alias = "TTS_UTTERANCE_CANCELED")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_CANCELED`"]
    pub const CANCELED: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 2i32
    };
    #[doc(alias = "TTS_UTTERANCE_BOUNDARY")]
    #[doc = "Godot enumerator name: `TTS_UTTERANCE_BOUNDARY`"]
    pub const BOUNDARY: TtsUtteranceEvent = TtsUtteranceEvent {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TtsUtteranceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TtsUtteranceEvent") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TtsUtteranceEvent {
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
            Self::STARTED => "STARTED", Self::ENDED => "ENDED", Self::CANCELED => "CANCELED", Self::BOUNDARY => "BOUNDARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TtsUtteranceEvent::STARTED, TtsUtteranceEvent::ENDED, TtsUtteranceEvent::CANCELED, TtsUtteranceEvent::BOUNDARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TtsUtteranceEvent >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STARTED", "TTS_UTTERANCE_STARTED", TtsUtteranceEvent::STARTED), crate::meta::inspect::EnumConstant::new("ENDED", "TTS_UTTERANCE_ENDED", TtsUtteranceEvent::ENDED), crate::meta::inspect::EnumConstant::new("CANCELED", "TTS_UTTERANCE_CANCELED", TtsUtteranceEvent::CANCELED), crate::meta::inspect::EnumConstant::new("BOUNDARY", "TTS_UTTERANCE_BOUNDARY", TtsUtteranceEvent::BOUNDARY)]
        }
    }
}
impl crate::meta::GodotConvert for TtsUtteranceEvent {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tts Utterance Started", 0i64), EnumeratorShape::new_int("Tts Utterance Ended", 1i64), EnumeratorShape::new_int("Tts Utterance Canceled", 2i64), EnumeratorShape::new_int("Tts Utterance Boundary", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("DisplayServer.TTSUtteranceEvent")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TtsUtteranceEvent {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TtsUtteranceEvent {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TtsUtteranceEvent {
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
impl crate::registry::property::Export for TtsUtteranceEvent {
    
}
impl crate::meta::Element for TtsUtteranceEvent {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DisplayServer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for DisplayServer {
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