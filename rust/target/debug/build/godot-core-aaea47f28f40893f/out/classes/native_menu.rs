#![doc = "Sidecar module for class [`NativeMenu`][crate::classes::NativeMenu].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NativeMenu` enums](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NativeMenu`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`native_menu`][crate::classes::native_menu]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `NativeMenu`](https://docs.godotengine.org/en/stable/classes/class_nativemenu.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`NativeMenu` handles low-level access to the OS native global menu bar and popup menus.\n\n**Note:** This is low-level API, consider using [`MenuBar`][crate::classes::MenuBar] with \\[member MenuBar.prefer_global_menu] set to `true`, and [`PopupMenu`][crate::classes::PopupMenu] with \\[member PopupMenu.prefer_native_menu] set to `true`.\n\nTo create a menu, use [`create_menu`][`crate::classes::NativeMenu::create_menu`], add menu items using `add_*_item` methods. To remove a menu, use [`free_menu`][`crate::classes::NativeMenu::free_menu`].\n\n```gdscript\nvar menu\n\nfunc _menu_callback(item_id):\n\tif item_id == \"ITEM_CUT\":\n\t\tcut()\n\telif item_id == \"ITEM_COPY\":\n\t\tcopy()\n\telif item_id == \"ITEM_PASTE\":\n\t\tpaste()\n\nfunc _enter_tree():\n\t# Create new menu and add items:\n\tmenu = NativeMenu.create_menu()\n\tNativeMenu.add_item(menu, \"Cut\", _menu_callback, Callable(), \"ITEM_CUT\")\n\tNativeMenu.add_item(menu, \"Copy\", _menu_callback, Callable(), \"ITEM_COPY\")\n\tNativeMenu.add_separator(menu)\n\tNativeMenu.add_item(menu, \"Paste\", _menu_callback, Callable(), \"ITEM_PASTE\")\n\nfunc _on_button_pressed():\n\t# Show popup menu at mouse position:\n\tNativeMenu.popup(menu, DisplayServer.mouse_get_position())\n\nfunc _exit_tree():\n\t# Remove menu when it's no longer needed:\n\tNativeMenu.free_menu(menu)\n```"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NativeMenu {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl NativeMenu {
        #[doc = "Returns `true` if the specified `feature` is supported by the current `NativeMenu`, `false` otherwise.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn has_feature(&self, feature: crate::classes::native_menu::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::native_menu::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a special system menu is supported.\n\n**Note:** This method is implemented only on macOS."]
        pub fn has_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns RID of a special system menu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_system_menu(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns readable name of a special system menu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_system_menu_name(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of the system menu item.\n\n**Note:** This method is implemented on macOS."]
        pub fn get_system_menu_text(&self, menu_id: crate::classes::native_menu::SystemMenus,) -> GString {
            type CallRet = GString;
            type CallParams = (crate::classes::native_menu::SystemMenus,);
            let args = (menu_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_system_menu_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the text of the system menu item.\n\n**Note:** This method is implemented on macOS."]
        pub fn set_system_menu_text(&mut self, menu_id: crate::classes::native_menu::SystemMenus, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::native_menu::SystemMenus, CowArg < 'a0, GString >,);
            let args = (menu_id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_system_menu_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new global menu object.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn create_menu(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "create_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `rid` is valid global menu.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn has_menu(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "has_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Frees a global menu object created by this `NativeMenu`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn free_menu(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "free_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns global menu size.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_size(&self, rid: Rid,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Shows the global menu at `position` in the screen coordinates.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn popup(&mut self, rid: Rid, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (rid, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "popup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the menu text layout direction from right-to-left if `is_rtl` is `true`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_interface_direction(&mut self, rid: Rid, is_rtl: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (rid, is_rtl,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_interface_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers callable to emit after the menu is closed.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_popup_open_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_open_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns global menu open callback.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_popup_open_callback(&self, rid: Rid,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_open_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Registers callable to emit when the menu is about to show.\n\n**Note:** The OS can simulate menu opening to track menu item changes and global shortcuts, in which case the corresponding close callback is not triggered. Use [`is_opened`][`crate::classes::NativeMenu::is_opened`] to check if the menu is currently opened.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_popup_close_callback(&mut self, rid: Rid, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >,);
            let args = (rid, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_popup_close_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns global menu close callback.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_popup_close_callback(&self, rid: Rid,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_popup_close_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the minimum width of the global menu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_minimum_width(&mut self, rid: Rid, width: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (rid, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns global menu minimum width.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_minimum_width(&self, rid: Rid,) -> f32 {
            type CallRet = f32;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the menu is currently opened.\n\n**Note:** This method is implemented only on macOS."]
        pub fn is_opened(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_opened", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an item that will act as a submenu of the global menu `rid`. The `submenu_rid` argument is the RID of the global menu that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub(crate) fn add_submenu_item_full(&mut self, rid: Rid, label: CowArg < GString >, submenu_rid: Rid, tag: RefArg < Variant >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, GString >, Rid, RefArg < 'a1, Variant >, i32,);
            let args = (rid, label, submenu_rid, tag, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_submenu_item_ex`][Self::add_submenu_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an item that will act as a submenu of the global menu `rid`. The `submenu_rid` argument is the RID of the global menu that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn add_submenu_item(&mut self, rid: Rid, label: impl AsArg < GString >, submenu_rid: Rid,) -> i32 {
            self.add_submenu_item_ex(rid, label, submenu_rid,) . done()
        }
        #[doc = "Adds an item that will act as a submenu of the global menu `rid`. The `submenu_rid` argument is the RID of the global menu that will be shown when the item is clicked.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn add_submenu_item_ex < 'ex > (&'ex mut self, rid: Rid, label: impl AsArg < GString > + 'ex, submenu_rid: Rid,) -> ExAddSubmenuItem < 'ex > {
            ExAddSubmenuItem::new(self, rid, label, submenu_rid,)
        }
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_item_ex`][Self::add_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(rid, label,) . done()
        }
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_item_ex < 'ex > (&'ex mut self, rid: Rid, label: impl AsArg < GString > + 'ex,) -> ExAddItem < 'ex > {
            ExAddItem::new(self, rid, label,)
        }
        #[doc = "Adds a new checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_check_item_ex`][Self::add_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_check_item_ex(rid, label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_check_item_ex < 'ex > (&'ex mut self, rid: Rid, label: impl AsArg < GString > + 'ex,) -> ExAddCheckItem < 'ex > {
            ExAddCheckItem::new(self, rid, label,)
        }
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_icon_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_item_ex`][Self::add_icon_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_item_ex(rid, icon, label,) . done()
        }
        #[doc = "Adds a new item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_item_ex < 'ex > (&'ex mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconItem < 'ex > {
            ExAddIconItem::new(self, rid, icon, label,)
        }
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_icon_check_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_check_item_ex`][Self::add_icon_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_check_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_check_item_ex(rid, icon, label,) . done()
        }
        #[doc = "Adds a new checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_check_item_ex < 'ex > (&'ex mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconCheckItem < 'ex > {
            ExAddIconCheckItem::new(self, rid, icon, label,)
        }
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_radio_check_item_full(&mut self, rid: Rid, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_radio_check_item_ex`][Self::add_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_radio_check_item(&mut self, rid: Rid, label: impl AsArg < GString >,) -> i32 {
            self.add_radio_check_item_ex(rid, label,) . done()
        }
        #[doc = "Adds a new radio-checkable item with text `label` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_radio_check_item_ex < 'ex > (&'ex mut self, rid: Rid, label: impl AsArg < GString > + 'ex,) -> ExAddRadioCheckItem < 'ex > {
            ExAddRadioCheckItem::new(self, rid, label,)
        }
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_icon_radio_check_item_full(&mut self, rid: Rid, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, RefArg < 'a3, Callable >, RefArg < 'a4, Variant >, crate::global::Key, i32,);
            let args = (rid, icon, label, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_icon_radio_check_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_icon_radio_check_item_ex`][Self::add_icon_radio_check_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_radio_check_item(&mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) -> i32 {
            self.add_icon_radio_check_item_ex(rid, icon, label,) . done()
        }
        #[doc = "Adds a new radio-checkable item with text `label` and icon `icon` to the global menu `rid`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See [`set_item_checked`][`crate::classes::NativeMenu::set_item_checked`] for more info on how to control it.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_icon_radio_check_item_ex < 'ex > (&'ex mut self, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> ExAddIconRadioCheckItem < 'ex > {
            ExAddIconRadioCheckItem::new(self, rid, icon, label,)
        }
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        pub(crate) fn add_multistate_item_full(&mut self, rid: Rid, label: CowArg < GString >, max_states: i32, default_state: i32, callback: RefArg < Callable >, key_callback: RefArg < Callable >, tag: RefArg < Variant >, accelerator: crate::global::Key, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, CowArg < 'a0, GString >, i32, i32, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >, RefArg < 'a3, Variant >, crate::global::Key, i32,);
            let args = (rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_multistate_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_multistate_item_ex`][Self::add_multistate_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_multistate_item(&mut self, rid: Rid, label: impl AsArg < GString >, max_states: i32, default_state: i32,) -> i32 {
            self.add_multistate_item_ex(rid, label, max_states, default_state,) . done()
        }
        #[doc = "Adds a new item with text `label` to the global menu `rid`.\n\nContrarily to normal binary items, multistate items can have more than two states, as defined by `max_states`. Each press or activate of the item will increase the state by one. The default value is defined by `default_state`.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\nAn `accelerator` can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The `accelerator` is generally a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** By default, there's no indication of the current item state, it should be changed manually.\n\n**Note:** The `callback` and `key_callback` Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to `tag`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** On Windows, `accelerator` and `key_callback` are ignored."]
        #[inline]
        pub fn add_multistate_item_ex < 'ex > (&'ex mut self, rid: Rid, label: impl AsArg < GString > + 'ex, max_states: i32, default_state: i32,) -> ExAddMultistateItem < 'ex > {
            ExAddMultistateItem::new(self, rid, label, max_states, default_state,)
        }
        #[doc = "Adds a separator between items to the global menu `rid`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub(crate) fn add_separator_full(&mut self, rid: Rid, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_separator_ex`][Self::add_separator_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a separator between items to the global menu `rid`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn add_separator(&mut self, rid: Rid,) -> i32 {
            self.add_separator_ex(rid,) . done()
        }
        #[doc = "Adds a separator between items to the global menu `rid`. Separators also occupy an index.\n\nReturns index of the inserted item, it's not guaranteed to be the same as `index` value.\n\n**Note:** This method is implemented on macOS and Windows."]
        #[inline]
        pub fn add_separator_ex < 'ex > (&'ex mut self, rid: Rid,) -> ExAddSeparator < 'ex > {
            ExAddSeparator::new(self, rid,)
        }
        #[doc = "Returns the index of the item with the specified `text`. Indices are automatically assigned to each item by the engine, and cannot be set manually.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn find_item_index_with_text(&self, rid: Rid, text: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (rid, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the item with the specified `tag`. Indices are automatically assigned to each item by the engine, and cannot be set manually.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn find_item_index_with_tag(&self, rid: Rid, tag: &Variant,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (rid, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the item with the submenu specified by `submenu_rid`. Indices are automatically assigned to each item by the engine, and cannot be set manually.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn find_item_index_with_submenu(&self, rid: Rid, submenu_rid: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, Rid,);
            let args = (rid, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "find_item_index_with_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is checked.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn is_item_checked(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is checkable in some way, i.e. if it has a checkbox or radio button.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn is_item_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` has radio button-style checkability.\n\n**Note:** This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn is_item_radio_checkable(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the callback of the item at index `idx`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the callback of the item accelerator at index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_item_key_callback(&self, rid: Rid, idx: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata of the specified item, which might be of any type. You can set it with [`set_item_tag`][`crate::classes::NativeMenu::set_item_tag`], which provides a simple way of assigning context data to items.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_tag(&self, rid: Rid, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text of the item at index `idx`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_text(&self, rid: Rid, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the submenu ID of the item at index `idx`. See [`add_submenu_item`][`crate::classes::NativeMenu::add_submenu_item`] for more info on how to add a submenu.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_submenu(&self, rid: Rid, idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the accelerator of the item at index `idx`. Accelerators are special combinations of keys that activate the item, no matter which control is focused.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_item_accelerator(&self, rid: Rid, idx: i32,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is disabled. When it is disabled it can't be selected, or its action invoked.\n\nSee [`set_item_disabled`][`crate::classes::NativeMenu::set_item_disabled`] for more info on how to disable an item.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn is_item_disabled(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the item at index `idx` is hidden.\n\nSee [`set_item_hidden`][`crate::classes::NativeMenu::set_item_hidden`] for more info on how to hide an item.\n\n**Note:** This method is implemented only on macOS."]
        pub fn is_item_hidden(&self, rid: Rid, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip associated with the specified index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_item_tooltip(&self, rid: Rid, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the state of a multistate item. See [`add_multistate_item`][`crate::classes::NativeMenu::add_multistate_item`] for details.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_state(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of states of a multistate item. See [`add_multistate_item`][`crate::classes::NativeMenu::add_multistate_item`] for details.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_max_states(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon of the item at index `idx`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_icon(&self, rid: Rid, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the horizontal offset of the item at the given `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn get_item_indentation_level(&self, rid: Rid, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the checkstate status of the item at index `idx`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_checked(&mut self, rid: Rid, idx: i32, checked: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the item at index `idx` has a checkbox. If `false`, sets the type of the item to plain text.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the type of the item at the specified index `idx` to radio button. If `false`, sets the type of the item to plain text.\n\n**Note:** This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_radio_checkable(&mut self, rid: Rid, idx: i32, checkable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, checkable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_radio_checkable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. Callback is emitted when an item is pressed.\n\n**Note:** The `callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_callback(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. The callback is emitted when an item is hovered.\n\n**Note:** The `callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_hover_callbacks(&mut self, rid: Rid, idx: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hover_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback of the item at index `idx`. Callback is emitted when its accelerator is activated.\n\n**Note:** The `key_callback` Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the `tag` parameter when the menu item was created.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_key_callback(&mut self, rid: Rid, idx: i32, key_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Callable >,);
            let args = (rid, idx, RefArg::new(key_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_key_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata of an item, which may be of any type. You can later get it with [`get_item_tag`][`crate::classes::NativeMenu::get_item_tag`], which provides a simple way of assigning context data to items.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_tag(&mut self, rid: Rid, idx: i32, tag: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, RefArg < 'a0, Variant >,);
            let args = (rid, idx, RefArg::new(tag),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the text of the item at index `idx`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_text(&mut self, rid: Rid, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (rid, idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the submenu RID of the item at index `idx`. The submenu is a global menu that would be shown when the item is clicked.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_submenu(&mut self, rid: Rid, idx: i32, submenu_rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (rid, idx, submenu_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_submenu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the accelerator of the item at index `idx`. `keycode` can be a single \\[enum Key], or a combination of \\[enum KeyModifierMask]s and \\[enum Key]s using bitwise OR such as `KEY_MASK_CTRL | KEY_A` (`Ctrl + A`).\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_accelerator(&mut self, rid: Rid, idx: i32, keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (Rid, i32, crate::global::Key,);
            let args = (rid, idx, keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_accelerator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables/disables the item at index `idx`. When it is disabled, it can't be selected and its action can't be invoked.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_disabled(&mut self, rid: Rid, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Hides/shows the item at index `idx`. When it is hidden, an item does not appear in a menu and its action cannot be invoked.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_hidden(&mut self, rid: Rid, idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (rid, idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_hidden", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`String`][crate::builtin::GString] tooltip of the item at the specified index `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_tooltip(&mut self, rid: Rid, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, GString >,);
            let args = (rid, idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the state of a multistate item. See [`add_multistate_item`][`crate::classes::NativeMenu::add_multistate_item`] for details.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_state(&mut self, rid: Rid, idx: i32, state: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets number of state of a multistate item. See [`add_multistate_item`][`crate::classes::NativeMenu::add_multistate_item`] for details.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn set_item_max_states(&mut self, rid: Rid, idx: i32, max_states: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, max_states,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_max_states", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Replaces the [`Texture2D`][crate::classes::Texture2D] icon of the specified `idx`.\n\n**Note:** This method is implemented on macOS and Windows.\n\n**Note:** This method is not supported by macOS Dock menu items."]
        pub fn set_item_icon(&mut self, rid: Rid, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (rid, idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the horizontal offset of the item at the given `idx`.\n\n**Note:** This method is implemented only on macOS."]
        pub fn set_item_indentation_level(&mut self, rid: Rid, idx: i32, level: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (rid, idx, level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "set_item_indentation_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns number of items in the global menu `rid`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn get_item_count(&self, rid: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Return `true` is global menu is a special system menu.\n\n**Note:** This method is implemented only on macOS."]
        pub fn is_system_menu(&self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "is_system_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the item at index `idx` from the global menu `rid`.\n\n**Note:** The indices of items after the removed item will be shifted by one.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn remove_item(&mut self, rid: Rid, idx: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (rid, idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all items from the global menu `rid`.\n\n**Note:** This method is implemented on macOS and Windows."]
        pub fn clear(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NativeMenu", "clear", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for NativeMenu {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("NativeMenu"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NativeMenu {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NativeMenu {
        
    }
    impl crate::obj::Singleton for NativeMenu {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"NativeMenu"))
            }
        }
    }
    impl std::ops::Deref for NativeMenu {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NativeMenu {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_NativeMenu__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `NativeMenu` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_submenu_item_ex`][super::NativeMenu::add_submenu_item_ex]."]
#[must_use]
pub struct ExAddSubmenuItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: CowArg < 'ex, GString >, submenu_rid: Rid, tag: CowArg < 'ex, Variant >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSubmenuItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'ex, submenu_rid: Rid,) -> Self {
        let tag = Variant::nil();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), submenu_rid: submenu_rid, tag: CowArg::Owned(tag), index: index,
        }
    }
    #[inline]
    pub fn tag(self, tag: &'ex Variant) -> Self {
        Self {
            tag: CowArg::Borrowed(tag), .. self
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
            _phantom, surround_object, rid, label, submenu_rid, tag, index,
        }
        = self;
        re_export::NativeMenu::add_submenu_item_full(surround_object, rid, label, submenu_rid, tag.cow_as_arg(), index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_item_ex`][super::NativeMenu::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_check_item_ex`][super::NativeMenu::add_check_item_ex]."]
#[must_use]
pub struct ExAddCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_item_ex`][super::NativeMenu::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_check_item_ex`][super::NativeMenu::add_icon_check_item_ex]."]
#[must_use]
pub struct ExAddIconCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_check_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_radio_check_item_ex`][super::NativeMenu::add_radio_check_item_ex]."]
#[must_use]
pub struct ExAddRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_radio_check_item_full(surround_object, rid, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_icon_radio_check_item_ex`][super::NativeMenu::add_icon_radio_check_item_ex]."]
#[must_use]
pub struct ExAddIconRadioCheckItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'ex, GString >, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddIconRadioCheckItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex, label: impl AsArg < GString > + 'ex,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, icon: icon.into_arg(), label: label.into_arg(), callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, icon, label, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_icon_radio_check_item_full(surround_object, rid, icon, label, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_multistate_item_ex`][super::NativeMenu::add_multistate_item_ex]."]
#[must_use]
pub struct ExAddMultistateItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: CowArg < 'ex, GString >, max_states: i32, default_state: i32, callback: CowArg < 'ex, Callable >, key_callback: CowArg < 'ex, Callable >, tag: CowArg < 'ex, Variant >, accelerator: crate::global::Key, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddMultistateItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid, label: impl AsArg < GString > + 'ex, max_states: i32, default_state: i32,) -> Self {
        let callback = Callable::invalid();
        let key_callback = Callable::invalid();
        let tag = Variant::nil();
        let accelerator = crate::obj::EngineEnum::from_ord(0);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, label: label.into_arg(), max_states: max_states, default_state: default_state, callback: CowArg::Owned(callback), key_callback: CowArg::Owned(key_callback), tag: CowArg::Owned(tag), accelerator: accelerator, index: index,
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
            _phantom, surround_object, rid, label, max_states, default_state, callback, key_callback, tag, accelerator, index,
        }
        = self;
        re_export::NativeMenu::add_multistate_item_full(surround_object, rid, label, max_states, default_state, callback.cow_as_arg(), key_callback.cow_as_arg(), tag.cow_as_arg(), accelerator, index,)
    }
}
#[doc = "Default-param extender for [`NativeMenu::add_separator_ex`][super::NativeMenu::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::NativeMenu, rid: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddSeparator < 'ex > {
    fn new(surround_object: &'ex mut re_export::NativeMenu, rid: Rid,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rid: rid, index: index,
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
            _phantom, surround_object, rid, index,
        }
        = self;
        re_export::NativeMenu::add_separator_full(surround_object, rid, index,)
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
    #[doc(alias = "FEATURE_POPUP_MENU")]
    #[doc = "Godot enumerator name: `FEATURE_POPUP_MENU`"]
    pub const POPUP_MENU: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_OPEN_CLOSE_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_OPEN_CLOSE_CALLBACK`"]
    pub const OPEN_CLOSE_CALLBACK: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_HOVER_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_HOVER_CALLBACK`"]
    pub const HOVER_CALLBACK: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_KEY_CALLBACK")]
    #[doc = "Godot enumerator name: `FEATURE_KEY_CALLBACK`"]
    pub const KEY_CALLBACK: Feature = Feature {
        ord: 4i32
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
            Self::GLOBAL_MENU => "GLOBAL_MENU", Self::POPUP_MENU => "POPUP_MENU", Self::OPEN_CLOSE_CALLBACK => "OPEN_CLOSE_CALLBACK", Self::HOVER_CALLBACK => "HOVER_CALLBACK", Self::KEY_CALLBACK => "KEY_CALLBACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::GLOBAL_MENU, Feature::POPUP_MENU, Feature::OPEN_CLOSE_CALLBACK, Feature::HOVER_CALLBACK, Feature::KEY_CALLBACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLOBAL_MENU", "FEATURE_GLOBAL_MENU", Feature::GLOBAL_MENU), crate::meta::inspect::EnumConstant::new("POPUP_MENU", "FEATURE_POPUP_MENU", Feature::POPUP_MENU), crate::meta::inspect::EnumConstant::new("OPEN_CLOSE_CALLBACK", "FEATURE_OPEN_CLOSE_CALLBACK", Feature::OPEN_CLOSE_CALLBACK), crate::meta::inspect::EnumConstant::new("HOVER_CALLBACK", "FEATURE_HOVER_CALLBACK", Feature::HOVER_CALLBACK), crate::meta::inspect::EnumConstant::new("KEY_CALLBACK", "FEATURE_KEY_CALLBACK", Feature::KEY_CALLBACK)]
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
            &[EnumeratorShape::new_int("Feature Global Menu", 0i64), EnumeratorShape::new_int("Feature Popup Menu", 1i64), EnumeratorShape::new_int("Feature Open Close Callback", 2i64), EnumeratorShape::new_int("Feature Hover Callback", 3i64), EnumeratorShape::new_int("Feature Key Callback", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("NativeMenu.Feature")), is_bitfield: false,
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
pub struct SystemMenus {
    ord: i32
}
impl SystemMenus {
    pub const INVALID_MENU_ID: SystemMenus = SystemMenus {
        ord: 0i32
    };
    pub const MAIN_MENU_ID: SystemMenus = SystemMenus {
        ord: 1i32
    };
    pub const APPLICATION_MENU_ID: SystemMenus = SystemMenus {
        ord: 2i32
    };
    pub const WINDOW_MENU_ID: SystemMenus = SystemMenus {
        ord: 3i32
    };
    pub const HELP_MENU_ID: SystemMenus = SystemMenus {
        ord: 4i32
    };
    pub const DOCK_MENU_ID: SystemMenus = SystemMenus {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SystemMenus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SystemMenus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SystemMenus {
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
            Self::INVALID_MENU_ID => "INVALID_MENU_ID", Self::MAIN_MENU_ID => "MAIN_MENU_ID", Self::APPLICATION_MENU_ID => "APPLICATION_MENU_ID", Self::WINDOW_MENU_ID => "WINDOW_MENU_ID", Self::HELP_MENU_ID => "HELP_MENU_ID", Self::DOCK_MENU_ID => "DOCK_MENU_ID", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SystemMenus::INVALID_MENU_ID, SystemMenus::MAIN_MENU_ID, SystemMenus::APPLICATION_MENU_ID, SystemMenus::WINDOW_MENU_ID, SystemMenus::HELP_MENU_ID, SystemMenus::DOCK_MENU_ID]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SystemMenus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID_MENU_ID", "INVALID_MENU_ID", SystemMenus::INVALID_MENU_ID), crate::meta::inspect::EnumConstant::new("MAIN_MENU_ID", "MAIN_MENU_ID", SystemMenus::MAIN_MENU_ID), crate::meta::inspect::EnumConstant::new("APPLICATION_MENU_ID", "APPLICATION_MENU_ID", SystemMenus::APPLICATION_MENU_ID), crate::meta::inspect::EnumConstant::new("WINDOW_MENU_ID", "WINDOW_MENU_ID", SystemMenus::WINDOW_MENU_ID), crate::meta::inspect::EnumConstant::new("HELP_MENU_ID", "HELP_MENU_ID", SystemMenus::HELP_MENU_ID), crate::meta::inspect::EnumConstant::new("DOCK_MENU_ID", "DOCK_MENU_ID", SystemMenus::DOCK_MENU_ID)]
        }
    }
}
impl crate::meta::GodotConvert for SystemMenus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Invalid Menu Id", 0i64), EnumeratorShape::new_int("Main Menu Id", 1i64), EnumeratorShape::new_int("Application Menu Id", 2i64), EnumeratorShape::new_int("Window Menu Id", 3i64), EnumeratorShape::new_int("Help Menu Id", 4i64), EnumeratorShape::new_int("Dock Menu Id", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("NativeMenu.SystemMenus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SystemMenus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SystemMenus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SystemMenus {
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
impl crate::registry::property::Export for SystemMenus {
    
}
impl crate::meta::Element for SystemMenus {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::NativeMenu;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for NativeMenu {
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