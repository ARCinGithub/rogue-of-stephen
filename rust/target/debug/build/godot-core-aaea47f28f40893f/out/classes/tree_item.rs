#![doc = "Sidecar module for class [`TreeItem`][crate::classes::TreeItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TreeItem` enums](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TreeItem`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`tree_item`][crate::classes::tree_item]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TreeItem`](https://docs.godotengine.org/en/stable/classes/class_treeitem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TreeItem>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nA single item of a [`Tree`][crate::classes::Tree] control. It can contain other `TreeItem`s as children, which allows it to create a hierarchy. It can also contain text and buttons. `TreeItem` is not a [`Node`][crate::classes::Node], it is internal to the [`Tree`][crate::classes::Tree].\n\nTo create a `TreeItem`, use [`create_item`][`crate::classes::Tree::create_item`] or [`create_child`][`crate::classes::TreeItem::create_child`]. To remove a `TreeItem`, use [`free`][`crate::obj::Gd::free`].\n\n**Note:** The ID values used for buttons are 32-bit, unlike `int` which is always 64-bit. They go from `-2147483648` to `2147483647`."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TreeItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TreeItem {
        #[doc = "Sets the given column's cell mode to `mode`. This determines how the cell is displayed and edited."]
        pub fn set_cell_mode(&mut self, column: i32, mode: crate::classes::tree_item::TreeCellMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::tree_item::TreeCellMode,);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_cell_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column's cell mode."]
        pub fn get_cell_mode(&self, column: i32,) -> crate::classes::tree_item::TreeCellMode {
            type CallRet = crate::classes::tree_item::TreeCellMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_cell_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's auto translate mode to `mode`.\n\nAll columns use [`AutoTranslateMode::INHERIT`][`crate::classes::node::AutoTranslateMode::INHERIT`] by default, which uses the same auto translate mode as the [`Tree`][crate::classes::Tree] itself."]
        pub fn set_auto_translate_mode(&mut self, column: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the column's auto translate mode."]
        pub fn get_auto_translate_mode(&self, column: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `multiline` is `true`, the given `column` is multiline editable.\n\n**Note:** This option only affects the type of control ([`LineEdit`][crate::classes::LineEdit] or [`TextEdit`][crate::classes::TextEdit]) that appears when editing the column. You can set multiline values with [`set_text`][`crate::classes::TreeItem::set_text`] even if the column is not multiline editable."]
        pub fn set_edit_multiline(&mut self, column: i32, multiline: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, multiline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_edit_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is multiline editable."]
        pub fn is_edit_multiline(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_edit_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `checked` is `true`, the given `column` is checked. Clears column's indeterminate status."]
        pub fn set_checked(&mut self, column: i32, checked: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `indeterminate` is `true`, the given `column` is marked indeterminate.\n\n**Note:** If set `true` from `false`, then column is cleared of checked status."]
        pub fn set_indeterminate(&mut self, column: i32, indeterminate: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is checked."]
        pub fn is_checked(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_checked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is indeterminate."]
        pub fn is_indeterminate(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Propagates this item's checked status to its children and parents for the given `column`. It is possible to process the items affected by this method call by connecting to `Tree.check_propagated_to_item`. The order that the items affected will be processed is as follows: the item invoking this method, children of that item, and finally parents of that item. If `emit_signal` is `false`, then `Tree.check_propagated_to_item` will not be emitted."]
        pub(crate) fn propagate_check_full(&mut self, column: i32, emit_signal: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, emit_signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "propagate_check", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`propagate_check_ex`][Self::propagate_check_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Propagates this item's checked status to its children and parents for the given `column`. It is possible to process the items affected by this method call by connecting to `Tree.check_propagated_to_item`. The order that the items affected will be processed is as follows: the item invoking this method, children of that item, and finally parents of that item. If `emit_signal` is `false`, then `Tree.check_propagated_to_item` will not be emitted."]
        #[inline]
        pub fn propagate_check(&mut self, column: i32,) {
            self.propagate_check_ex(column,) . done()
        }
        #[doc = "Propagates this item's checked status to its children and parents for the given `column`. It is possible to process the items affected by this method call by connecting to `Tree.check_propagated_to_item`. The order that the items affected will be processed is as follows: the item invoking this method, children of that item, and finally parents of that item. If `emit_signal` is `false`, then `Tree.check_propagated_to_item` will not be emitted."]
        #[inline]
        pub fn propagate_check_ex < 'ex > (&'ex mut self, column: i32,) -> ExPropagateCheck < 'ex > {
            ExPropagateCheck::new(self, column,)
        }
        #[doc = "Sets the given column's text value."]
        pub fn set_text(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's text."]
        pub fn get_text(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's description for assistive apps."]
        pub fn set_description(&mut self, column: i32, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's description for assistive apps."]
        pub fn get_description(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets item's text base writing direction."]
        pub fn set_text_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text base writing direction."]
        pub fn get_text_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the autowrap mode in the given `column`. If set to something other than [`AutowrapMode::OFF`][`crate::classes::text_server::AutowrapMode::OFF`], the text gets wrapped inside the cell's bounding rectangle."]
        pub fn set_autowrap_mode(&mut self, column: i32, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::AutowrapMode,);
            let args = (column, autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the text autowrap mode in the given `column`. By default it is [`AutowrapMode::OFF`][`crate::classes::text_server::AutowrapMode::OFF`]."]
        pub fn get_autowrap_mode(&self, column: i32,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the clipping behavior when the text exceeds the item's bounding rectangle in the given `column`."]
        pub fn set_text_overrun_behavior(&mut self, column: i32, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::OverrunBehavior,);
            let args = (column, overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the clipping behavior when the text exceeds the item's bounding rectangle in the given `column`. By default it is [`OverrunBehavior::TRIM_ELLIPSIS`][`crate::classes::text_server::OverrunBehavior::TRIM_ELLIPSIS`]."]
        pub fn get_text_overrun_behavior(&self, column: i32,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set BiDi algorithm override for the structured text. Has effect for cells that display text."]
        pub fn set_structured_text_bidi_override(&mut self, column: i32, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::StructuredTextParser,);
            let args = (column, parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the BiDi algorithm override set for this cell."]
        pub fn get_structured_text_bidi_override(&self, column: i32,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set additional options for BiDi override. Has effect for cells that display text."]
        pub fn set_structured_text_bidi_override_options(&mut self, column: i32, args: &AnyArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, AnyArray >,);
            let args = (column, RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the additional BiDi options set for this cell."]
        pub fn get_structured_text_bidi_override_options(&self, column: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the language code of the given `column`'s text to `language`. This is used for line-breaking and text shaping algorithms. If `language` is empty, the current locale is used."]
        pub fn set_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns item's text language code."]
        pub fn get_language(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a string to be shown after a column's value (for example, a unit abbreviation)."]
        pub fn set_suffix(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_suffix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the suffix string shown after the column value."]
        pub fn get_suffix(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_suffix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given cell's icon [`Texture2D`][crate::classes::Texture2D]. If the cell is in [`TreeCellMode::ICON`][`crate::classes::tree_item::TreeCellMode::ICON`] mode, the icon is displayed in the center of the cell. Otherwise, the icon is displayed before the cell's text. [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] does not display an icon."]
        pub fn set_icon(&mut self, column: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's icon [`Texture2D`][crate::classes::Texture2D]. Error if no icon is set."]
        pub fn get_icon(&self, column: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given cell's icon overlay [`Texture2D`][crate::classes::Texture2D]. The cell has to be in [`TreeCellMode::ICON`][`crate::classes::tree_item::TreeCellMode::ICON`] mode, and icon has to be set. Overlay is drawn on top of icon, in the bottom left corner."]
        pub fn set_icon_overlay(&mut self, column: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_overlay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's icon overlay [`Texture2D`][crate::classes::Texture2D]."]
        pub fn get_icon_overlay(&self, column: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_overlay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's icon's texture region."]
        pub fn set_icon_region(&mut self, column: i32, region: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2,);
            let args = (column, region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the icon [`Texture2D`][crate::classes::Texture2D] region as [`Rect2`][crate::builtin::Rect2]."]
        pub fn get_icon_region(&self, column: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum allowed width of the icon in the given `column`. This limit is applied on top of the default size of the icon and on top of [theme_item Tree.icon_max_width]. The height is adjusted according to the icon's ratio."]
        pub fn set_icon_max_width(&mut self, column: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the maximum allowed width of the icon in the given `column`."]
        pub fn get_icon_max_width(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Modulates the given column's icon with `modulate`."]
        pub fn set_icon_modulate(&mut self, column: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (column, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Color`][crate::builtin::Color] modulating the column's icon."]
        pub fn get_icon_modulate(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value of a [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] column."]
        pub fn set_range(&mut self, column: i32, value: f64,) {
            type CallRet = ();
            type CallParams = (i32, f64,);
            let args = (column, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] column."]
        pub fn get_range(&self, column: i32,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the range of accepted values for a column. The column must be in the [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] mode.\n\nIf `expr` is `true`, the edit mode slider will use an exponential scale as with \\[member Range.exp_edit]."]
        pub(crate) fn set_range_config_full(&mut self, column: i32, min: f64, max: f64, step: f64, expr: bool,) {
            type CallRet = ();
            type CallParams = (i32, f64, f64, f64, bool,);
            let args = (column, min, max, step, expr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_range_config", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_range_config_ex`][Self::set_range_config_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the range of accepted values for a column. The column must be in the [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] mode.\n\nIf `expr` is `true`, the edit mode slider will use an exponential scale as with \\[member Range.exp_edit]."]
        #[inline]
        pub fn set_range_config(&mut self, column: i32, min: f64, max: f64, step: f64,) {
            self.set_range_config_ex(column, min, max, step,) . done()
        }
        #[doc = "Sets the range of accepted values for a column. The column must be in the [`TreeCellMode::RANGE`][`crate::classes::tree_item::TreeCellMode::RANGE`] mode.\n\nIf `expr` is `true`, the edit mode slider will use an exponential scale as with \\[member Range.exp_edit]."]
        #[inline]
        pub fn set_range_config_ex < 'ex > (&'ex mut self, column: i32, min: f64, max: f64, step: f64,) -> ExSetRangeConfig < 'ex > {
            ExSetRangeConfig::new(self, column, min, max, step,)
        }
        #[doc = "Returns a dictionary containing the range parameters for a given column. The keys are \"min\", \"max\", \"step\", and \"expr\"."]
        pub fn get_range_config(&self, column: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_range_config", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the metadata value for the given column, which can be retrieved later using [`get_metadata`][`crate::classes::TreeItem::get_metadata`]. This can be used, for example, to store a reference to the original data."]
        pub fn set_metadata(&mut self, column: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (column, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the metadata value that was set for the given column using [`set_metadata`][`crate::classes::TreeItem::set_metadata`]."]
        pub fn get_metadata(&self, column: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's custom draw callback to the `callback` method on `object`.\n\nThe method named `callback` should accept two arguments: the `TreeItem` that is drawn and its position and size as a [`Rect2`][crate::builtin::Rect2]."]
        pub fn set_custom_draw(&mut self, column: i32, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >,);
            let args = (column, object.into_arg(), callback.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's custom draw callback. Use an empty [`Callable`][crate::builtin::Callable] (`Callable()`) to clear the custom callback. The cell has to be in [`TreeCellMode::CUSTOM`][`crate::classes::tree_item::TreeCellMode::CUSTOM`] to use this feature.\n\nThe `callback` should accept two arguments: the `TreeItem` that is drawn and its position and size as a [`Rect2`][crate::builtin::Rect2]."]
        pub fn set_custom_draw_callback(&mut self, column: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (column, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom callback of column `column`."]
        pub fn get_custom_draw_callback(&self, column: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_draw_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's custom [`StyleBox`][crate::classes::StyleBox] used to draw the background.\n\n**Note:** If a custom background color is set, the [`StyleBox`][crate::classes::StyleBox] will be drawn in front of it."]
        pub fn set_custom_stylebox(&mut self, column: i32, stylebox: impl AsArg < Option < Gd < crate::classes::StyleBox >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::StyleBox > > >,);
            let args = (column, stylebox.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's custom [`StyleBox`][crate::classes::StyleBox] used to draw the background."]
        pub fn get_custom_stylebox(&self, column: i32,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collapsed(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collapsed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Collapses or uncollapses this `TreeItem` and all the descendants of this item."]
        pub fn set_collapsed_recursive(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed_recursive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this `TreeItem`, or any of its descendants, is collapsed.\n\nIf `only_visible` is `true` it ignores non-visible `TreeItem`s."]
        pub(crate) fn is_any_collapsed_full(&self, only_visible: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (only_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_any_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`is_any_collapsed_ex`][Self::is_any_collapsed_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns `true` if this `TreeItem`, or any of its descendants, is collapsed.\n\nIf `only_visible` is `true` it ignores non-visible `TreeItem`s."]
        #[inline]
        pub fn is_any_collapsed(&self,) -> bool {
            self.is_any_collapsed_ex() . done()
        }
        #[doc = "Returns `true` if this `TreeItem`, or any of its descendants, is collapsed.\n\nIf `only_visible` is `true` it ignores non-visible `TreeItem`s."]
        #[inline]
        pub fn is_any_collapsed_ex < 'ex > (&'ex self,) -> ExIsAnyCollapsed < 'ex > {
            ExIsAnyCollapsed::new(self,)
        }
        pub fn set_visible(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if \\[member visible] is `true` and all its ancestors are also visible."]
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Uncollapses all `TreeItem`s necessary to reveal this `TreeItem`, i.e. all ancestor `TreeItem`s."]
        pub fn uncollapse_tree(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "uncollapse_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_minimum_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_minimum_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_minimum_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_minimum_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `selectable` is `true`, the given `column` is selectable."]
        pub fn set_selectable(&mut self, column: i32, selectable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_selectable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is selectable."]
        pub fn is_selectable(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_selectable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is selected."]
        pub fn is_selected(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Selects the given `column`."]
        pub fn select(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deselects the given column."]
        pub fn deselect(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, the given `column` is editable."]
        pub fn set_editable(&mut self, column: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given `column` is editable."]
        pub fn is_editable(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's custom color."]
        pub fn set_custom_color(&mut self, column: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (column, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom color of column `column`."]
        pub fn get_custom_color(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resets the color for the given column to default."]
        pub fn clear_custom_color(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets custom font used to draw text in the given `column`."]
        pub fn set_custom_font(&mut self, column: i32, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (column, font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns custom font used to draw text in the column `column`."]
        pub fn get_custom_font(&self, column: i32,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets custom font size used to draw text in the given `column`."]
        pub fn set_custom_font_size(&mut self, column: i32, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns custom font size used to draw text in the column `column`."]
        pub fn get_custom_font_size(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's custom background color and whether to just use it as an outline.\n\n**Note:** If a custom [`StyleBox`][crate::classes::StyleBox] is set, the background color will be drawn behind it."]
        pub(crate) fn set_custom_bg_color_full(&mut self, column: i32, color: Color, just_outline: bool,) {
            type CallRet = ();
            type CallParams = (i32, Color, bool,);
            let args = (column, color, just_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_custom_bg_color_ex`][Self::set_custom_bg_color_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the given column's custom background color and whether to just use it as an outline.\n\n**Note:** If a custom [`StyleBox`][crate::classes::StyleBox] is set, the background color will be drawn behind it."]
        #[inline]
        pub fn set_custom_bg_color(&mut self, column: i32, color: Color,) {
            self.set_custom_bg_color_ex(column, color,) . done()
        }
        #[doc = "Sets the given column's custom background color and whether to just use it as an outline.\n\n**Note:** If a custom [`StyleBox`][crate::classes::StyleBox] is set, the background color will be drawn behind it."]
        #[inline]
        pub fn set_custom_bg_color_ex < 'ex > (&'ex mut self, column: i32, color: Color,) -> ExSetCustomBgColor < 'ex > {
            ExSetCustomBgColor::new(self, column, color,)
        }
        #[doc = "Resets the background color for the given column to default."]
        pub fn clear_custom_bg_color(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom background color of column `column`."]
        pub fn get_custom_bg_color(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes a cell with [`TreeCellMode::CUSTOM`][`crate::classes::tree_item::TreeCellMode::CUSTOM`] display as a non-flat button with a [`StyleBox`][crate::classes::StyleBox]."]
        pub fn set_custom_as_button(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_as_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the cell was made into a button with [`set_custom_as_button`][`crate::classes::TreeItem::set_custom_as_button`]."]
        pub fn is_custom_set_as_button(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_custom_set_as_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all buttons from all columns of this item."]
        pub fn clear_buttons(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_buttons", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a button with [`Texture2D`][crate::classes::Texture2D] `button` to the end of the cell at column `column`. The `id` is used to identify the button in the according `Tree.button_clicked` signal and can be different from the buttons index. If not specified, the next available index is used, which may be retrieved by calling [`get_button_count`][`crate::classes::TreeItem::get_button_count`] immediately before this method. Optionally, the button can be `disabled` and have a `tooltip_text`. `description` is used as the button description for assistive apps."]
        pub(crate) fn add_button_full(&mut self, column: i32, button: CowArg < Option < Gd < crate::classes::Texture2D > > >, id: i32, disabled: bool, tooltip_text: CowArg < GString >, description: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, i32, bool, CowArg < 'a1, GString >, CowArg < 'a2, GString >,);
            let args = (column, button, id, disabled, tooltip_text, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "add_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_button_ex`][Self::add_button_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a button with [`Texture2D`][crate::classes::Texture2D] `button` to the end of the cell at column `column`. The `id` is used to identify the button in the according `Tree.button_clicked` signal and can be different from the buttons index. If not specified, the next available index is used, which may be retrieved by calling [`get_button_count`][`crate::classes::TreeItem::get_button_count`] immediately before this method. Optionally, the button can be `disabled` and have a `tooltip_text`. `description` is used as the button description for assistive apps."]
        #[inline]
        pub fn add_button(&mut self, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.add_button_ex(column, button,) . done()
        }
        #[doc = "Adds a button with [`Texture2D`][crate::classes::Texture2D] `button` to the end of the cell at column `column`. The `id` is used to identify the button in the according `Tree.button_clicked` signal and can be different from the buttons index. If not specified, the next available index is used, which may be retrieved by calling [`get_button_count`][`crate::classes::TreeItem::get_button_count`] immediately before this method. Optionally, the button can be `disabled` and have a `tooltip_text`. `description` is used as the button description for assistive apps."]
        #[inline]
        pub fn add_button_ex < 'ex > (&'ex mut self, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExAddButton < 'ex > {
            ExAddButton::new(self, column, button,)
        }
        #[doc = "Returns the number of buttons in column `column`."]
        pub fn get_button_count(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the tooltip text for the button at index `button_index` in column `column`."]
        pub fn get_button_tooltip_text(&self, column: i32, button_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID for the button at index `button_index` in column `column`."]
        pub fn get_button_id(&self, column: i32, button_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the button index if there is a button with ID `id` in column `column`, otherwise returns -1."]
        pub fn get_button_by_id(&self, column: i32, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_by_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color of the button with ID `id` in column `column`. If the specified button does not exist, returns `Color.BLACK`."]
        pub fn get_button_color(&self, column: i32, id: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Texture2D`][crate::classes::Texture2D] of the button at index `button_index` in column `column`."]
        pub fn get_button(&self, column: i32, button_index: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the tooltip text for the button at index `button_index` in the given `column`."]
        pub fn set_button_tooltip_text(&mut self, column: i32, button_index: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (column, button_index, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's button [`Texture2D`][crate::classes::Texture2D] at index `button_index` to `button`."]
        pub fn set_button(&mut self, column: i32, button_index: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, button_index, button.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the button at index `button_index` in column `column`."]
        pub fn erase_button(&mut self, column: i32, button_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "erase_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's button description at index `button_index` for assistive apps."]
        pub fn set_button_description(&mut self, column: i32, button_index: i32, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (column, button_index, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, disables the button at index `button_index` in the given `column`."]
        pub fn set_button_disabled(&mut self, column: i32, button_index: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (column, button_index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's button color at index `button_index` to `color`."]
        pub fn set_button_color(&mut self, column: i32, button_index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (column, button_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the button at index `button_index` for the given `column` is disabled."]
        pub fn is_button_disabled(&self, column: i32, button_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_button_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's tooltip text."]
        pub fn set_tooltip_text(&mut self, column: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's tooltip text."]
        pub fn get_tooltip_text(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given column's text alignment to `text_alignment`."]
        pub fn set_text_alignment(&mut self, column: i32, text_alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (i32, crate::global::HorizontalAlignment,);
            let args = (column, text_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the given column's text alignment."]
        pub fn get_text_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_alignment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enable` is `true`, the given `column` is expanded to the right."]
        pub fn set_expand_right(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_expand_right", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if `expand_right` is set."]
        pub fn get_expand_right(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_expand_right", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_folding(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_disable_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_folding_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_folding_disabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an item and adds it as a child.\n\nThe new item will be inserted as position `index` (the default value `-1` means the last position), or it will be the last child if `index` is higher than the child count."]
        pub(crate) fn create_child_full(&mut self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "create_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_child_ex`][Self::create_child_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an item and adds it as a child.\n\nThe new item will be inserted as position `index` (the default value `-1` means the last position), or it will be the last child if `index` is higher than the child count."]
        #[inline]
        pub fn create_child(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_child_ex() . done()
        }
        #[doc = "Creates an item and adds it as a child.\n\nThe new item will be inserted as position `index` (the default value `-1` means the last position), or it will be the last child if `index` is higher than the child count."]
        #[inline]
        pub fn create_child_ex < 'ex > (&'ex mut self,) -> ExCreateChild < 'ex > {
            ExCreateChild::new(self,)
        }
        #[doc = "Adds a previously unparented `TreeItem` as a direct child of this one. The `child` item must not be a part of any [`Tree`][crate::classes::Tree] or parented to any `TreeItem`. See also [`remove_child`][`crate::classes::TreeItem::remove_child`]."]
        pub fn add_child(&mut self, child: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (child.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "add_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the given child `TreeItem` and all its children from the [`Tree`][crate::classes::Tree]. Note that it doesn't free the item from memory, so it can be reused later (see [`add_child`][`crate::classes::TreeItem::add_child`]). To completely remove a `TreeItem` use [`free`][`crate::obj::Gd::free`].\n\n**Note:** If you want to move a child from one [`Tree`][crate::classes::Tree] to another, then instead of removing and adding it manually you can use [`move_before`][`crate::classes::TreeItem::move_before`] or [`move_after`][`crate::classes::TreeItem::move_after`]."]
        pub fn remove_child(&mut self, child: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (child.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "remove_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Tree`][crate::classes::Tree] that owns this TreeItem."]
        pub fn get_tree(&self,) -> Option < Gd < crate::classes::Tree > > {
            type CallRet = Option < Gd < crate::classes::Tree > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next sibling TreeItem in the tree or a `null` object if there is none."]
        pub fn get_next(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the previous sibling TreeItem in the tree or a `null` object if there is none."]
        pub fn get_prev(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the parent TreeItem or a `null` object if there is none."]
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the TreeItem's first child."]
        pub fn get_first_child(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_first_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the next TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first element in the tree when called on the last element, otherwise it returns `null`."]
        pub(crate) fn get_next_in_tree_full(&self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_next_in_tree_ex`][Self::get_next_in_tree_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the next TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first element in the tree when called on the last element, otherwise it returns `null`."]
        #[inline]
        pub fn get_next_in_tree(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_in_tree_ex() . done()
        }
        #[doc = "Returns the next TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first element in the tree when called on the last element, otherwise it returns `null`."]
        #[inline]
        pub fn get_next_in_tree_ex < 'ex > (&'ex self,) -> ExGetNextInTree < 'ex > {
            ExGetNextInTree::new(self,)
        }
        #[doc = "Returns the previous TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last element in the tree when called on the first visible element, otherwise it returns `null`."]
        pub(crate) fn get_prev_in_tree_full(&self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_prev_in_tree_ex`][Self::get_prev_in_tree_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the previous TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last element in the tree when called on the first visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_prev_in_tree(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_in_tree_ex() . done()
        }
        #[doc = "Returns the previous TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last element in the tree when called on the first visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_prev_in_tree_ex < 'ex > (&'ex self,) -> ExGetPrevInTree < 'ex > {
            ExGetPrevInTree::new(self,)
        }
        #[doc = "Returns the next visible TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first visible element in the tree when called on the last visible element, otherwise it returns `null`."]
        pub(crate) fn get_next_visible_full(&self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_next_visible_ex`][Self::get_next_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the next visible TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first visible element in the tree when called on the last visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_next_visible(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_visible_ex() . done()
        }
        #[doc = "Returns the next visible TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the first visible element in the tree when called on the last visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_next_visible_ex < 'ex > (&'ex self,) -> ExGetNextVisible < 'ex > {
            ExGetNextVisible::new(self,)
        }
        #[doc = "Returns the previous visible sibling TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last visible element in the tree when called on the first visible element, otherwise it returns `null`."]
        pub(crate) fn get_prev_visible_full(&self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_prev_visible_ex`][Self::get_prev_visible_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the previous visible sibling TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last visible element in the tree when called on the first visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_prev_visible(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_visible_ex() . done()
        }
        #[doc = "Returns the previous visible sibling TreeItem in the tree (in the context of a depth-first search) or a `null` object if there is none.\n\nIf `wrap` is enabled, the method will wrap around to the last visible element in the tree when called on the first visible element, otherwise it returns `null`."]
        #[inline]
        pub fn get_prev_visible_ex < 'ex > (&'ex self,) -> ExGetPrevVisible < 'ex > {
            ExGetPrevVisible::new(self,)
        }
        #[doc = "Returns a child item by its `index` (see [`get_child_count`][`crate::classes::TreeItem::get_child_count`]). This method is often used for iterating all children of an item.\n\nNegative indices access the children from the last one."]
        pub fn get_child(&self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of child items."]
        pub fn get_child_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_child_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of references to the item's children."]
        pub fn get_children(&self,) -> Array < Gd < crate::classes::TreeItem > > {
            type CallRet = Array < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_children", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the node's order in the tree. For example, if called on the first child item the position is `0`."]
        pub fn get_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves this TreeItem right before the given `item`.\n\n**Note:** You can't move to the root or move the root."]
        pub fn move_before(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "move_before", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves this TreeItem right after the given `item`.\n\n**Note:** You can't move to the root or move the root."]
        pub fn move_after(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "move_after", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calls the `method` on the actual TreeItem and its children recursively. Pass parameters as a comma separated list."]
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_call_recursive(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7436usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "TreeItem", "call_recursive", Some(self.__validated_obj()), args, varargs)
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
    impl crate::obj::GodotClass for TreeItem {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TreeItem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TreeItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TreeItem {
        
    }
    impl std::ops::Deref for TreeItem {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TreeItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TreeItem__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TreeItem` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TreeItem::propagate_check_ex`][super::TreeItem::propagate_check_ex]."]
#[must_use]
pub struct ExPropagateCheck < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TreeItem, column: i32, emit_signal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPropagateCheck < 'ex > {
    fn new(surround_object: &'ex mut re_export::TreeItem, column: i32,) -> Self {
        let emit_signal = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, emit_signal: emit_signal,
        }
    }
    #[inline]
    pub fn emit_signal(self, emit_signal: bool) -> Self {
        Self {
            emit_signal: emit_signal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, emit_signal,
        }
        = self;
        re_export::TreeItem::propagate_check_full(surround_object, column, emit_signal,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_range_config_ex`][super::TreeItem::set_range_config_ex]."]
#[must_use]
pub struct ExSetRangeConfig < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64, expr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetRangeConfig < 'ex > {
    fn new(surround_object: &'ex mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64,) -> Self {
        let expr = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, min: min, max: max, step: step, expr: expr,
        }
    }
    #[inline]
    pub fn expr(self, expr: bool) -> Self {
        Self {
            expr: expr, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, min, max, step, expr,
        }
        = self;
        re_export::TreeItem::set_range_config_full(surround_object, column, min, max, step, expr,)
    }
}
#[doc = "Default-param extender for [`TreeItem::is_any_collapsed_ex`][super::TreeItem::is_any_collapsed_ex]."]
#[must_use]
pub struct ExIsAnyCollapsed < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TreeItem, only_visible: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExIsAnyCollapsed < 'ex > {
    fn new(surround_object: &'ex re_export::TreeItem,) -> Self {
        let only_visible = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, only_visible: only_visible,
        }
    }
    #[inline]
    pub fn only_visible(self, only_visible: bool) -> Self {
        Self {
            only_visible: only_visible, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, only_visible,
        }
        = self;
        re_export::TreeItem::is_any_collapsed_full(surround_object, only_visible,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_custom_bg_color_ex`][super::TreeItem::set_custom_bg_color_ex]."]
#[must_use]
pub struct ExSetCustomBgColor < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TreeItem, column: i32, color: Color, just_outline: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetCustomBgColor < 'ex > {
    fn new(surround_object: &'ex mut re_export::TreeItem, column: i32, color: Color,) -> Self {
        let just_outline = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, color: color, just_outline: just_outline,
        }
    }
    #[inline]
    pub fn just_outline(self, just_outline: bool) -> Self {
        Self {
            just_outline: just_outline, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, color, just_outline,
        }
        = self;
        re_export::TreeItem::set_custom_bg_color_full(surround_object, column, color, just_outline,)
    }
}
#[doc = "Default-param extender for [`TreeItem::add_button_ex`][super::TreeItem::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TreeItem, column: i32, button: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, id: i32, disabled: bool, tooltip_text: CowArg < 'ex, GString >, description: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddButton < 'ex > {
    fn new(surround_object: &'ex mut re_export::TreeItem, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let id = - 1i32;
        let disabled = false;
        let tooltip_text = GString::from("");
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, button: button.into_arg(), id: id, disabled: disabled, tooltip_text: CowArg::Owned(tooltip_text), description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn tooltip_text(self, tooltip_text: impl AsArg < GString > + 'ex) -> Self {
        Self {
            tooltip_text: tooltip_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'ex) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, button, id, disabled, tooltip_text, description,
        }
        = self;
        re_export::TreeItem::add_button_full(surround_object, column, button, id, disabled, tooltip_text, description,)
    }
}
#[doc = "Default-param extender for [`TreeItem::create_child_ex`][super::TreeItem::create_child_ex]."]
#[must_use]
pub struct ExCreateChild < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::TreeItem, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateChild < 'ex > {
    fn new(surround_object: &'ex mut re_export::TreeItem,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TreeItem::create_child_full(surround_object, index,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_in_tree_ex`][super::TreeItem::get_next_in_tree_ex]."]
#[must_use]
pub struct ExGetNextInTree < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetNextInTree < 'ex > {
    fn new(surround_object: &'ex re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_in_tree_ex`][super::TreeItem::get_prev_in_tree_ex]."]
#[must_use]
pub struct ExGetPrevInTree < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPrevInTree < 'ex > {
    fn new(surround_object: &'ex re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_visible_ex`][super::TreeItem::get_next_visible_ex]."]
#[must_use]
pub struct ExGetNextVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetNextVisible < 'ex > {
    fn new(surround_object: &'ex re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_visible_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_visible_ex`][super::TreeItem::get_prev_visible_ex]."]
#[must_use]
pub struct ExGetPrevVisible < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetPrevVisible < 'ex > {
    fn new(surround_object: &'ex re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_visible_full(surround_object, wrap,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TreeCellMode {
    ord: i32
}
impl TreeCellMode {
    #[doc(alias = "CELL_MODE_STRING")]
    #[doc = "Godot enumerator name: `CELL_MODE_STRING`"]
    pub const STRING: TreeCellMode = TreeCellMode {
        ord: 0i32
    };
    #[doc(alias = "CELL_MODE_CHECK")]
    #[doc = "Godot enumerator name: `CELL_MODE_CHECK`"]
    pub const CHECK: TreeCellMode = TreeCellMode {
        ord: 1i32
    };
    #[doc(alias = "CELL_MODE_RANGE")]
    #[doc = "Godot enumerator name: `CELL_MODE_RANGE`"]
    pub const RANGE: TreeCellMode = TreeCellMode {
        ord: 2i32
    };
    #[doc(alias = "CELL_MODE_ICON")]
    #[doc = "Godot enumerator name: `CELL_MODE_ICON`"]
    pub const ICON: TreeCellMode = TreeCellMode {
        ord: 3i32
    };
    #[doc(alias = "CELL_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CELL_MODE_CUSTOM`"]
    pub const CUSTOM: TreeCellMode = TreeCellMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TreeCellMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TreeCellMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TreeCellMode {
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
            Self::STRING => "STRING", Self::CHECK => "CHECK", Self::RANGE => "RANGE", Self::ICON => "ICON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TreeCellMode::STRING, TreeCellMode::CHECK, TreeCellMode::RANGE, TreeCellMode::ICON, TreeCellMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TreeCellMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRING", "CELL_MODE_STRING", TreeCellMode::STRING), crate::meta::inspect::EnumConstant::new("CHECK", "CELL_MODE_CHECK", TreeCellMode::CHECK), crate::meta::inspect::EnumConstant::new("RANGE", "CELL_MODE_RANGE", TreeCellMode::RANGE), crate::meta::inspect::EnumConstant::new("ICON", "CELL_MODE_ICON", TreeCellMode::ICON), crate::meta::inspect::EnumConstant::new("CUSTOM", "CELL_MODE_CUSTOM", TreeCellMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for TreeCellMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cell Mode String", 0i64), EnumeratorShape::new_int("Cell Mode Check", 1i64), EnumeratorShape::new_int("Cell Mode Range", 2i64), EnumeratorShape::new_int("Cell Mode Icon", 3i64), EnumeratorShape::new_int("Cell Mode Custom", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("TreeItem.TreeCellMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TreeCellMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TreeCellMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TreeCellMode {
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
impl crate::registry::property::Export for TreeCellMode {
    
}
impl crate::meta::Element for TreeCellMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TreeItem;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TreeItem {
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