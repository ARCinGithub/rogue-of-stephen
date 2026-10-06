#![doc = "Sidecar module for class [`CanvasItem`][crate::classes::CanvasItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItem` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CanvasItem`.\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`canvas_item`][crate::classes::canvas_item]: sidecar module with related enum/flag types\n* [`SignalsOfCanvasItem`][crate::classes::canvas_item::SignalsOfCanvasItem]: signal collection\n* [`CanvasItemNotification`][crate::classes::notify::CanvasItemNotification]: notification type\n\n\nSee also [Godot docs for `CanvasItem`](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CanvasItem>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nAbstract base class for everything in 2D space. Canvas items are laid out in a tree; children inherit and extend their parent's transform. `CanvasItem` is extended by [`Control`][crate::classes::Control] for GUI-related nodes, and by [`Node2D`][crate::classes::Node2D] for 2D game objects.\n\nAny `CanvasItem` can draw. For this, [`queue_redraw`][`crate::classes::CanvasItem::queue_redraw`] is called by the engine, then [`CanvasItemNotification::DRAW`][`crate::classes::notify::CanvasItemNotification::DRAW`] will be received on idle time to request a redraw. Because of this, canvas items don't need to be redrawn on every frame, improving the performance significantly. Several functions for drawing on the `CanvasItem` are provided (see `draw_*` functions). However, they can only be used inside \\[method _draw], its corresponding [`on_notification`][`crate::classes::IObject::on_notification`] or methods connected to the `draw` signal.\n\nCanvas items are drawn in tree order on their canvas layer. By default, children are on top of their parents, so a root `CanvasItem` will be drawn behind everything. This behavior can be changed on a per-item basis.\n\nA `CanvasItem` can be hidden, which will also hide its children. By adjusting various other properties of a `CanvasItem`, you can also modulate its color (via \\[member modulate] or \\[member self_modulate]), change its Z-index, blend mode, and more.\n\nNote that properties like transform, modulation, and visibility are only propagated to _direct_ `CanvasItem` child nodes. If there is a non-`CanvasItem` node in between, like [`Node`][crate::classes::Node] or [`AnimationPlayer`][crate::classes::AnimationPlayer], the `CanvasItem` nodes below will have an independent position and \\[member modulate] chain. See also \\[member top_level]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CanvasItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Notification type for class [`CanvasItem`][crate::classes::CanvasItem]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum CanvasItemNotification {
        TRANSFORM_CHANGED = 2000i32, LOCAL_TRANSFORM_CHANGED = 35i32, DRAW = 30i32, VISIBILITY_CHANGED = 31i32, ENTER_CANVAS = 32i32, EXIT_CANVAS = 33i32, WORLD_2D_CHANGED = 36i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for CanvasItemNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TRANSFORM_CHANGED, 35i32 => Self::LOCAL_TRANSFORM_CHANGED, 30i32 => Self::DRAW, 31i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::ENTER_CANVAS, 33i32 => Self::EXIT_CANVAS, 36i32 => Self::WORLD_2D_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < CanvasItemNotification > for i32 {
        fn from(notification: CanvasItemNotification) -> i32 {
            match notification {
                CanvasItemNotification::TRANSFORM_CHANGED => 2000i32, CanvasItemNotification::LOCAL_TRANSFORM_CHANGED => 35i32, CanvasItemNotification::DRAW => 30i32, CanvasItemNotification::VISIBILITY_CHANGED => 31i32, CanvasItemNotification::ENTER_CANVAS => 32i32, CanvasItemNotification::EXIT_CANVAS => 33i32, CanvasItemNotification::WORLD_2D_CHANGED => 36i32, CanvasItemNotification::ENTER_TREE => 10i32, CanvasItemNotification::EXIT_TREE => 11i32, CanvasItemNotification::MOVED_IN_PARENT => 12i32, CanvasItemNotification::READY => 13i32, CanvasItemNotification::PAUSED => 14i32, CanvasItemNotification::UNPAUSED => 15i32, CanvasItemNotification::PHYSICS_PROCESS => 16i32, CanvasItemNotification::PROCESS => 17i32, CanvasItemNotification::PARENTED => 18i32, CanvasItemNotification::UNPARENTED => 19i32, CanvasItemNotification::SCENE_INSTANTIATED => 20i32, CanvasItemNotification::DRAG_BEGIN => 21i32, CanvasItemNotification::DRAG_END => 22i32, CanvasItemNotification::PATH_RENAMED => 23i32, CanvasItemNotification::CHILD_ORDER_CHANGED => 24i32, CanvasItemNotification::INTERNAL_PROCESS => 25i32, CanvasItemNotification::INTERNAL_PHYSICS_PROCESS => 26i32, CanvasItemNotification::POST_ENTER_TREE => 27i32, CanvasItemNotification::DISABLED => 28i32, CanvasItemNotification::ENABLED => 29i32, CanvasItemNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, CanvasItemNotification::EDITOR_PRE_SAVE => 9001i32, CanvasItemNotification::EDITOR_POST_SAVE => 9002i32, CanvasItemNotification::WM_MOUSE_ENTER => 1002i32, CanvasItemNotification::WM_MOUSE_EXIT => 1003i32, CanvasItemNotification::WM_WINDOW_FOCUS_IN => 1004i32, CanvasItemNotification::WM_WINDOW_FOCUS_OUT => 1005i32, CanvasItemNotification::WM_CLOSE_REQUEST => 1006i32, CanvasItemNotification::WM_GO_BACK_REQUEST => 1007i32, CanvasItemNotification::WM_SIZE_CHANGED => 1008i32, CanvasItemNotification::WM_DPI_CHANGE => 1009i32, CanvasItemNotification::VP_MOUSE_ENTER => 1010i32, CanvasItemNotification::VP_MOUSE_EXIT => 1011i32, CanvasItemNotification::WM_POSITION_CHANGED => 1012i32, CanvasItemNotification::OS_MEMORY_WARNING => 2009i32, CanvasItemNotification::TRANSLATION_CHANGED => 2010i32, CanvasItemNotification::WM_ABOUT => 2011i32, CanvasItemNotification::CRASH => 2012i32, CanvasItemNotification::OS_IME_UPDATE => 2013i32, CanvasItemNotification::APPLICATION_RESUMED => 2014i32, CanvasItemNotification::APPLICATION_PAUSED => 2015i32, CanvasItemNotification::APPLICATION_FOCUS_IN => 2016i32, CanvasItemNotification::APPLICATION_FOCUS_OUT => 2017i32, CanvasItemNotification::TEXT_SERVER_CHANGED => 2018i32, CanvasItemNotification::ACCESSIBILITY_UPDATE => 3000i32, CanvasItemNotification::ACCESSIBILITY_INVALIDATE => 3001i32, CanvasItemNotification::POSTINITIALIZE => 0i32, CanvasItemNotification::PREDELETE => 1i32, CanvasItemNotification::EXTENSION_RELOADED => 2i32, CanvasItemNotification::Unknown(int) => int,
            }
        }
    }
    impl CanvasItem {
        #[doc = "Returns the internal canvas item [`RID`][crate::builtin::Rid] used by the [`RenderingServer`][crate::classes::RenderingServer] for this node."]
        pub fn get_canvas_item(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node is present in the [`SceneTree`][crate::classes::SceneTree], its \\[member visible] property is `true` and all its ancestors are also visible. If any ancestor is hidden, this node will not be visible in the scene tree, and is therefore not drawn (see \\[method _draw]).\n\nVisibility is checked only in parent nodes that inherit from `CanvasItem`, [`CanvasLayer`][crate::classes::CanvasLayer], and [`Window`][crate::classes::Window]. If the parent is of any other type (such as [`Node`][crate::classes::Node], [`AnimationPlayer`][crate::classes::AnimationPlayer], or [`Node3D`][crate::classes::Node3D]), it is assumed to be visible.\n\n**Note:** This method does not take \\[member visibility_layer] into account, so even if this method returns `true`, the node might end up not being rendered."]
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_visible_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Show the `CanvasItem` if it's currently hidden. This is equivalent to setting \\[member visible] to `true`.\n\n**Note:** For controls that inherit [`Popup`][crate::classes::Popup], the correct way to make them visible is to call one of the multiple `popup*()` functions instead."]
        pub fn show(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "show", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Hide the `CanvasItem` if it's currently visible. This is equivalent to setting \\[member visible] to `false`."]
        pub fn hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "hide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Queues the `CanvasItem` to redraw. During idle time, if `CanvasItem` is visible, [`CanvasItemNotification::DRAW`][`crate::classes::notify::CanvasItemNotification::DRAW`] is sent and \\[method _draw] is called. This only occurs **once** per frame, even if this method has been called multiple times."]
        pub fn queue_redraw(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "queue_redraw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Moves this node below its siblings, usually causing the node to draw on top of its siblings. Does nothing if this node does not have a parent. See also [`move_child`][`crate::classes::Node::move_child`]."]
        pub fn move_to_front(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "move_to_front", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_light_mask(&mut self, light_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_light_mask(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_self_modulate(&mut self, self_modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (self_modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_self_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_index(&mut self, z_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_as_relative(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_z_as_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_z_relative(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_z_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_y_sort_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_y_sort_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_behind_parent(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_draw_behind_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_behind_parent_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_draw_behind_parent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws a line from a 2D point to another, with a given color and width. It can be optionally antialiased. The `from` and `to` positions are defined in local space. See also [`draw_dashed_line`][`crate::classes::CanvasItem::draw_dashed_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitive will be drawn instead of a four-point one. This means that when the CanvasItem is scaled, the line will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        pub(crate) fn draw_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2, Color, f32, bool,);
            let args = (from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_line_ex`][Self::draw_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a line from a 2D point to another, with a given color and width. It can be optionally antialiased. The `from` and `to` positions are defined in local space. See also [`draw_dashed_line`][`crate::classes::CanvasItem::draw_dashed_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitive will be drawn instead of a four-point one. This means that when the CanvasItem is scaled, the line will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_line_ex(from, to, color,) . done()
        }
        #[doc = "Draws a line from a 2D point to another, with a given color and width. It can be optionally antialiased. The `from` and `to` positions are defined in local space. See also [`draw_dashed_line`][`crate::classes::CanvasItem::draw_dashed_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitive will be drawn instead of a four-point one. This means that when the CanvasItem is scaled, the line will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_line_ex < 'ex > (&'ex mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawLine < 'ex > {
            ExDrawLine::new(self, from, to, color,)
        }
        #[doc = "Draws a dashed line from a 2D point to another, with a given color and width. The `from` and `to` positions are defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the line parts will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n`dash` is the length of each dash in pixels, with the gap between each dash being the same length. If `aligned` is `true`, the length of the first and last dashes may be shortened or lengthened to allow the line to begin and end at the precise points defined by `from` and `to`. Both ends are always symmetrical when `aligned` is `true`. If `aligned` is `false`, all dashes will have the same length, but the line may appear incomplete at the end due to the dash length not dividing evenly into the line length. Only full dashes are drawn when `aligned` is `false`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        pub(crate) fn draw_dashed_line_full(&mut self, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2, Color, f32, f32, bool, bool,);
            let args = (from, to, color, width, dash, aligned, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_dashed_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_dashed_line_ex`][Self::draw_dashed_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a dashed line from a 2D point to another, with a given color and width. The `from` and `to` positions are defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the line parts will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n`dash` is the length of each dash in pixels, with the gap between each dash being the same length. If `aligned` is `true`, the length of the first and last dashes may be shortened or lengthened to allow the line to begin and end at the precise points defined by `from` and `to`. Both ends are always symmetrical when `aligned` is `true`. If `aligned` is `false`, all dashes will have the same length, but the line may appear incomplete at the end due to the dash length not dividing evenly into the line length. Only full dashes are drawn when `aligned` is `false`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_dashed_line(&mut self, from: Vector2, to: Vector2, color: Color,) {
            self.draw_dashed_line_ex(from, to, color,) . done()
        }
        #[doc = "Draws a dashed line from a 2D point to another, with a given color and width. The `from` and `to` positions are defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`], and [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`].\n\nIf `width` is negative, then a two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the line parts will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n`dash` is the length of each dash in pixels, with the gap between each dash being the same length. If `aligned` is `true`, the length of the first and last dashes may be shortened or lengthened to allow the line to begin and end at the precise points defined by `from` and `to`. Both ends are always symmetrical when `aligned` is `true`. If `aligned` is `false`, all dashes will have the same length, but the line may appear incomplete at the end due to the dash length not dividing evenly into the line length. Only full dashes are drawn when `aligned` is `false`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_dashed_line_ex < 'ex > (&'ex mut self, from: Vector2, to: Vector2, color: Color,) -> ExDrawDashedLine < 'ex > {
            ExDrawDashedLine::new(self, from, to, color,)
        }
        #[doc = "Draws interconnected line segments with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        pub(crate) fn draw_polyline_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, Color, f32, bool,);
            let args = (points, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polyline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_polyline_ex`][Self::draw_polyline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws interconnected line segments with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_polyline(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_polyline_ex(points, color,) . done()
        }
        #[doc = "Draws interconnected line segments with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_polyline_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, color: Color,) -> ExDrawPolyline < 'ex > {
            ExDrawPolyline::new(self, points, color,)
        }
        #[doc = "Draws interconnected line segments with a uniform `width`, point-by-point coloring, and optional antialiasing (supported only for positive `width`). Colors assigned to line points match by index between `points` and `colors`, i.e. each line segment is filled with a gradient between the colors of the endpoints. The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        pub(crate) fn draw_polyline_colors_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polyline_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_polyline_colors_ex`][Self::draw_polyline_colors_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws interconnected line segments with a uniform `width`, point-by-point coloring, and optional antialiasing (supported only for positive `width`). Colors assigned to line points match by index between `points` and `colors`, i.e. each line segment is filled with a gradient between the colors of the endpoints. The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_polyline_colors(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_polyline_colors_ex(points, colors,) . done()
        }
        #[doc = "Draws interconnected line segments with a uniform `width`, point-by-point coloring, and optional antialiasing (supported only for positive `width`). Colors assigned to line points match by index between `points` and `colors`, i.e. each line segment is filled with a gradient between the colors of the endpoints. The `points` array is defined in local space. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw disconnected lines, use [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `width` is negative, it will be ignored and the polyline will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the polyline will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`."]
        #[inline]
        pub fn draw_polyline_colors_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExDrawPolylineColors < 'ex > {
            ExDrawPolylineColors::new(self, points, colors,)
        }
        #[doc = "Draws an unfilled elliptical arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. For circular arcs, see [`draw_arc`][`crate::classes::CanvasItem::draw_arc`]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full ellipse is drawn (i.e. arc will not overlap itself)."]
        pub(crate) fn draw_ellipse_arc_full(&mut self, center: Vector2, major: f32, minor: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, f32, f32, f32, i32, Color, f32, bool,);
            let args = (center, major, minor, start_angle, end_angle, point_count, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_ellipse_arc", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_ellipse_arc_ex`][Self::draw_ellipse_arc_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws an unfilled elliptical arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. For circular arcs, see [`draw_arc`][`crate::classes::CanvasItem::draw_arc`]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full ellipse is drawn (i.e. arc will not overlap itself)."]
        #[inline]
        pub fn draw_ellipse_arc(&mut self, center: Vector2, major: f32, minor: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) {
            self.draw_ellipse_arc_ex(center, major, minor, start_angle, end_angle, point_count, color,) . done()
        }
        #[doc = "Draws an unfilled elliptical arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. For circular arcs, see [`draw_arc`][`crate::classes::CanvasItem::draw_arc`]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full ellipse is drawn (i.e. arc will not overlap itself)."]
        #[inline]
        pub fn draw_ellipse_arc_ex < 'ex > (&'ex mut self, center: Vector2, major: f32, minor: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> ExDrawEllipseArc < 'ex > {
            ExDrawEllipseArc::new(self, center, major, minor, start_angle, end_angle, point_count, color,)
        }
        #[doc = "Draws an unfilled arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. `center` is defined in local space. For elliptical arcs, see [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full circle arc is drawn (i.e. arc will not overlap itself)."]
        pub(crate) fn draw_arc_full(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, f32, f32, i32, Color, f32, bool,);
            let args = (center, radius, start_angle, end_angle, point_count, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_arc", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_arc_ex`][Self::draw_arc_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws an unfilled arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. `center` is defined in local space. For elliptical arcs, see [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full circle arc is drawn (i.e. arc will not overlap itself)."]
        #[inline]
        pub fn draw_arc(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) {
            self.draw_arc_ex(center, radius, start_angle, end_angle, point_count, color,) . done()
        }
        #[doc = "Draws an unfilled arc between the given angles with a uniform `color` and `width` and optional antialiasing (supported only for positive `width`). The larger the value of `point_count`, the smoother the curve. `center` is defined in local space. For elliptical arcs, see [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`].\n\nIf `width` is negative, it will be ignored and the arc will be drawn using [`PrimitiveType::LINE_STRIP`][`crate::classes::rendering_server::PrimitiveType::LINE_STRIP`]. This means that when the CanvasItem is scaled, the arc will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nThe arc is drawn from `start_angle` towards the value of `end_angle` so in clockwise direction if `start_angle < end_angle` and counter-clockwise otherwise. Passing the same angles but in reversed order will produce the same arc. If absolute difference of `start_angle` and `end_angle` is greater than `@GDScript.TAU` radians, then a full circle arc is drawn (i.e. arc will not overlap itself)."]
        #[inline]
        pub fn draw_arc_ex < 'ex > (&'ex mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> ExDrawArc < 'ex > {
            ExDrawArc::new(self, center, radius, start_angle, end_angle, point_count, color,)
        }
        #[doc = "Draws multiple disconnected lines with a uniform `width` and `color`. Each line is defined by two consecutive points from `points` array in local space, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        pub(crate) fn draw_multiline_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >, Color, f32, bool,);
            let args = (points, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_ex`][Self::draw_multiline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws multiple disconnected lines with a uniform `width` and `color`. Each line is defined by two consecutive points from `points` array in local space, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_multiline(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_multiline_ex(points, color,) . done()
        }
        #[doc = "Draws multiple disconnected lines with a uniform `width` and `color`. Each line is defined by two consecutive points from `points` array in local space, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_multiline_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, color: Color,) -> ExDrawMultiline < 'ex > {
            ExDrawMultiline::new(self, points, color,)
        }
        #[doc = "Draws multiple disconnected lines with a uniform `width` and segment-by-segment coloring. Each segment is defined by two consecutive points from `points` array in local space and a corresponding color from `colors` array, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints and has `colors[i]` color. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        pub(crate) fn draw_multiline_colors_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_colors", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_colors_ex`][Self::draw_multiline_colors_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws multiple disconnected lines with a uniform `width` and segment-by-segment coloring. Each segment is defined by two consecutive points from `points` array in local space and a corresponding color from `colors` array, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints and has `colors[i]` color. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_multiline_colors(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_multiline_colors_ex(points, colors,) . done()
        }
        #[doc = "Draws multiple disconnected lines with a uniform `width` and segment-by-segment coloring. Each segment is defined by two consecutive points from `points` array in local space and a corresponding color from `colors` array, i.e. i-th segment consists of `points[2 * i]`, `points[2 * i + 1]` endpoints and has `colors[i]` color. When drawing large amounts of lines, this is faster than using individual [`draw_line`][`crate::classes::CanvasItem::draw_line`] calls. To draw interconnected lines, use [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`] instead.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\n**Note:** `antialiased` is only effective if `width` is greater than `0.0`."]
        #[inline]
        pub fn draw_multiline_colors_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExDrawMultilineColors < 'ex > {
            ExDrawMultilineColors::new(self, points, colors,)
        }
        #[doc = "Draws a rectangle. If `filled` is `true`, the rectangle will be filled with the `color` specified. If `filled` is `false`, the rectangle will be drawn as a stroke with the `color` and `width` specified. The `rect` is specified in local space. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`.\n\n**Note:** Unfilled rectangles drawn with a negative `width` may not display perfectly. For example, corners may be missing or brighter due to overlapping lines (for a translucent `color`)."]
        pub(crate) fn draw_rect_full(&mut self, rect: Rect2, color: Color, filled: bool, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rect2, Color, bool, f32, bool,);
            let args = (rect, color, filled, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_rect_ex`][Self::draw_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a rectangle. If `filled` is `true`, the rectangle will be filled with the `color` specified. If `filled` is `false`, the rectangle will be drawn as a stroke with the `color` and `width` specified. The `rect` is specified in local space. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`.\n\n**Note:** Unfilled rectangles drawn with a negative `width` may not display perfectly. For example, corners may be missing or brighter due to overlapping lines (for a translucent `color`)."]
        #[inline]
        pub fn draw_rect(&mut self, rect: Rect2, color: Color,) {
            self.draw_rect_ex(rect, color,) . done()
        }
        #[doc = "Draws a rectangle. If `filled` is `true`, the rectangle will be filled with the `color` specified. If `filled` is `false`, the rectangle will be drawn as a stroke with the `color` and `width` specified. The `rect` is specified in local space. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`.\n\n**Note:** Unfilled rectangles drawn with a negative `width` may not display perfectly. For example, corners may be missing or brighter due to overlapping lines (for a translucent `color`)."]
        #[inline]
        pub fn draw_rect_ex < 'ex > (&'ex mut self, rect: Rect2, color: Color,) -> ExDrawRect < 'ex > {
            ExDrawRect::new(self, rect, color,)
        }
        #[doc = "Draws a circle, with `position` defined in local space. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`], [`draw_arc`][`crate::classes::CanvasItem::draw_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the circle will be filled with the `color` specified. If `filled` is `false`, the circle will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        pub(crate) fn draw_circle_full(&mut self, position: Vector2, radius: f32, color: Color, filled: bool, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, Color, bool, f32, bool,);
            let args = (position, radius, color, filled, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_circle_ex`][Self::draw_circle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a circle, with `position` defined in local space. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`], [`draw_arc`][`crate::classes::CanvasItem::draw_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the circle will be filled with the `color` specified. If `filled` is `false`, the circle will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        #[inline]
        pub fn draw_circle(&mut self, position: Vector2, radius: f32, color: Color,) {
            self.draw_circle_ex(position, radius, color,) . done()
        }
        #[doc = "Draws a circle, with `position` defined in local space. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`], [`draw_arc`][`crate::classes::CanvasItem::draw_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the circle will be filled with the `color` specified. If `filled` is `false`, the circle will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of a four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        #[inline]
        pub fn draw_circle_ex < 'ex > (&'ex mut self, position: Vector2, radius: f32, color: Color,) -> ExDrawCircle < 'ex > {
            ExDrawCircle::new(self, position, radius, color,)
        }
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor`. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`], [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the ellipse will be filled with the `color` specified. If `filled` is `false`, the ellipse will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        pub(crate) fn draw_ellipse_full(&mut self, position: Vector2, major: f32, minor: f32, color: Color, filled: bool, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, f32, Color, bool, f32, bool,);
            let args = (position, major, minor, color, filled, width, antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_ellipse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_ellipse_ex`][Self::draw_ellipse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor`. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`], [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the ellipse will be filled with the `color` specified. If `filled` is `false`, the ellipse will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        #[inline]
        pub fn draw_ellipse(&mut self, position: Vector2, major: f32, minor: f32, color: Color,) {
            self.draw_ellipse_ex(position, major, minor, color,) . done()
        }
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor`. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`], [`draw_ellipse_arc`][`crate::classes::CanvasItem::draw_ellipse_arc`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], and [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\nIf `filled` is `true`, the ellipse will be filled with the `color` specified. If `filled` is `false`, the ellipse will be drawn as a stroke with the `color` and `width` specified.\n\nIf `width` is negative, then two-point primitives will be drawn instead of four-point ones. This means that when the CanvasItem is scaled, the lines will remain thin. If this behavior is not desired, then pass a positive `width` like `1.0`.\n\nIf `antialiased` is `true`, half transparent \"feathers\" will be attached to the boundary, making outlines smooth.\n\n**Note:** `width` is only effective if `filled` is `false`."]
        #[inline]
        pub fn draw_ellipse_ex < 'ex > (&'ex mut self, position: Vector2, major: f32, minor: f32, color: Color,) -> ExDrawEllipse < 'ex > {
            ExDrawEllipse::new(self, position, major, minor, color,)
        }
        #[doc = "Draws a texture at a given position. The `position` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_texture_full(&mut self, texture: CowArg < Gd < crate::classes::Texture2D > >, position: Vector2, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Texture2D > >, Vector2, Color,);
            let args = (texture, position, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_texture_ex`][Self::draw_texture_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a texture at a given position. The `position` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture(&mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >>, position: Vector2,) {
            self.draw_texture_ex(texture, position,) . done()
        }
        #[doc = "Draws a texture at a given position. The `position` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture_ex < 'ex > (&'ex mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, position: Vector2,) -> ExDrawTexture < 'ex > {
            ExDrawTexture::new(self, texture, position,)
        }
        #[doc = "Draws a textured rectangle at a given position, optionally modulated by a color. The `rect` is defined in local space. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`] and [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_texture_rect_full(&mut self, texture: CowArg < Gd < crate::classes::Texture2D > >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Texture2D > >, Rect2, bool, Color, bool,);
            let args = (texture, rect, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_texture_rect_ex`][Self::draw_texture_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a textured rectangle at a given position, optionally modulated by a color. The `rect` is defined in local space. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`] and [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture_rect(&mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >>, rect: Rect2, tile: bool,) {
            self.draw_texture_rect_ex(texture, rect, tile,) . done()
        }
        #[doc = "Draws a textured rectangle at a given position, optionally modulated by a color. The `rect` is defined in local space. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`] and [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture_rect_ex < 'ex > (&'ex mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, tile: bool,) -> ExDrawTextureRect < 'ex > {
            ExDrawTextureRect::new(self, texture, rect, tile,)
        }
        #[doc = "Draws a textured rectangle from a texture's region (specified by `src_rect`) at a given position in local space, optionally modulated by a color. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_texture_rect_region_full(&mut self, texture: CowArg < Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Texture2D > >, Rect2, Rect2, Color, bool, bool,);
            let args = (texture, rect, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_texture_rect_region_ex`][Self::draw_texture_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a textured rectangle from a texture's region (specified by `src_rect`) at a given position in local space, optionally modulated by a color. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture_rect_region(&mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >>, rect: Rect2, src_rect: Rect2,) {
            self.draw_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[doc = "Draws a textured rectangle from a texture's region (specified by `src_rect`) at a given position in local space, optionally modulated by a color. If `transpose` is `true`, the texture will have its X and Y coordinates swapped. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_texture_rect_region_ex < 'ex > (&'ex mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> ExDrawTextureRectRegion < 'ex > {
            ExDrawTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        #[doc = "Draws a textured rectangle region of the multichannel signed distance field texture at a given position, optionally modulated by a color. The `rect` is defined in local space. See \\[member FontFile.multichannel_signed_distance_field] for more information and caveats about MSDF font rendering.\n\nIf `outline` is positive, each alpha channel value of pixel in region is set to maximum value of true distance in the `outline` radius.\n\nValue of the `pixel_range` should the same that was used during distance field texture generation.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_msdf_texture_rect_region_full(&mut self, texture: CowArg < Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Texture2D > >, Rect2, Rect2, Color, f64, f64, f64,);
            let args = (texture, rect, src_rect, modulate, outline, pixel_range, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_msdf_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_msdf_texture_rect_region_ex`][Self::draw_msdf_texture_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a textured rectangle region of the multichannel signed distance field texture at a given position, optionally modulated by a color. The `rect` is defined in local space. See \\[member FontFile.multichannel_signed_distance_field] for more information and caveats about MSDF font rendering.\n\nIf `outline` is positive, each alpha channel value of pixel in region is set to maximum value of true distance in the `outline` radius.\n\nValue of the `pixel_range` should the same that was used during distance field texture generation.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_msdf_texture_rect_region(&mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >>, rect: Rect2, src_rect: Rect2,) {
            self.draw_msdf_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[doc = "Draws a textured rectangle region of the multichannel signed distance field texture at a given position, optionally modulated by a color. The `rect` is defined in local space. See \\[member FontFile.multichannel_signed_distance_field] for more information and caveats about MSDF font rendering.\n\nIf `outline` is positive, each alpha channel value of pixel in region is set to maximum value of true distance in the `outline` radius.\n\nValue of the `pixel_range` should the same that was used during distance field texture generation.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_msdf_texture_rect_region_ex < 'ex > (&'ex mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> ExDrawMsdfTextureRectRegion < 'ex > {
            ExDrawMsdfTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        #[doc = "Draws a textured rectangle region of the font texture with LCD subpixel anti-aliasing at a given position, optionally modulated by a color. The `rect` is defined in local space.\n\nTexture is drawn using the following blend operation, blend mode of the [`CanvasItemMaterial`][crate::classes::CanvasItemMaterial] is ignored:\n\n```gdscript\ndst.r = texture.r * modulate.r * modulate.a + dst.r * (1.0 - texture.r * modulate.a);\ndst.g = texture.g * modulate.g * modulate.a + dst.g * (1.0 - texture.g * modulate.a);\ndst.b = texture.b * modulate.b * modulate.a + dst.b * (1.0 - texture.b * modulate.a);\ndst.a = modulate.a + dst.a * (1.0 - modulate.a);\n```\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_lcd_texture_rect_region_full(&mut self, texture: CowArg < Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Texture2D > >, Rect2, Rect2, Color,);
            let args = (texture, rect, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_lcd_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_lcd_texture_rect_region_ex`][Self::draw_lcd_texture_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a textured rectangle region of the font texture with LCD subpixel anti-aliasing at a given position, optionally modulated by a color. The `rect` is defined in local space.\n\nTexture is drawn using the following blend operation, blend mode of the [`CanvasItemMaterial`][crate::classes::CanvasItemMaterial] is ignored:\n\n```gdscript\ndst.r = texture.r * modulate.r * modulate.a + dst.r * (1.0 - texture.r * modulate.a);\ndst.g = texture.g * modulate.g * modulate.a + dst.g * (1.0 - texture.g * modulate.a);\ndst.b = texture.b * modulate.b * modulate.a + dst.b * (1.0 - texture.b * modulate.a);\ndst.a = modulate.a + dst.a * (1.0 - modulate.a);\n```\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_lcd_texture_rect_region(&mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >>, rect: Rect2, src_rect: Rect2,) {
            self.draw_lcd_texture_rect_region_ex(texture, rect, src_rect,) . done()
        }
        #[doc = "Draws a textured rectangle region of the font texture with LCD subpixel anti-aliasing at a given position, optionally modulated by a color. The `rect` is defined in local space.\n\nTexture is drawn using the following blend operation, blend mode of the [`CanvasItemMaterial`][crate::classes::CanvasItemMaterial] is ignored:\n\n```gdscript\ndst.r = texture.r * modulate.r * modulate.a + dst.r * (1.0 - texture.r * modulate.a);\ndst.g = texture.g * modulate.g * modulate.a + dst.g * (1.0 - texture.g * modulate.a);\ndst.b = texture.b * modulate.b * modulate.a + dst.b * (1.0 - texture.b * modulate.a);\ndst.a = modulate.a + dst.a * (1.0 - modulate.a);\n```\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_lcd_texture_rect_region_ex < 'ex > (&'ex mut self, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> ExDrawLcdTextureRectRegion < 'ex > {
            ExDrawLcdTextureRectRegion::new(self, texture, rect, src_rect,)
        }
        #[doc = "Draws a styled rectangle. The `rect` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub fn draw_style_box(&mut self, style_box: impl AsArg < Gd < crate::classes::StyleBox >>, rect: Rect2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::StyleBox > >, Rect2,);
            let args = (style_box.into_arg(), rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_style_box", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws a custom primitive. 1 point for a point, 2 points for a line, 3 points for a triangle, and 4 points for a quad. If 0 points or more than 4 points are specified, nothing will be drawn and an error message will be printed. The `points` array is defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], and [`draw_rect`][`crate::classes::CanvasItem::draw_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_primitive_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_primitive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_primitive_ex`][Self::draw_primitive_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a custom primitive. 1 point for a point, 2 points for a line, 3 points for a triangle, and 4 points for a quad. If 0 points or more than 4 points are specified, nothing will be drawn and an error message will be printed. The `points` array is defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], and [`draw_rect`][`crate::classes::CanvasItem::draw_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_primitive(&mut self, points: &PackedVector2Array, colors: &PackedColorArray, uvs: &PackedVector2Array,) {
            self.draw_primitive_ex(points, colors, uvs,) . done()
        }
        #[doc = "Draws a custom primitive. 1 point for a point, 2 points for a line, 3 points for a triangle, and 4 points for a quad. If 0 points or more than 4 points are specified, nothing will be drawn and an error message will be printed. The `points` array is defined in local space. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`], [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`], [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], and [`draw_rect`][`crate::classes::CanvasItem::draw_rect`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_primitive_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray, uvs: &'ex PackedVector2Array,) -> ExDrawPrimitive < 'ex > {
            ExDrawPrimitive::new(self, points, colors, uvs,)
        }
        #[doc = "Draws a solid polygon of any number of points, convex or concave. Unlike [`draw_colored_polygon`][`crate::classes::CanvasItem::draw_colored_polygon`], each point's color can be changed individually. The `points` array is defined in local space. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_polygon_full(&mut self, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_polygon_ex`][Self::draw_polygon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a solid polygon of any number of points, convex or concave. Unlike [`draw_colored_polygon`][`crate::classes::CanvasItem::draw_colored_polygon`], each point's color can be changed individually. The `points` array is defined in local space. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_polygon(&mut self, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.draw_polygon_ex(points, colors,) . done()
        }
        #[doc = "Draws a solid polygon of any number of points, convex or concave. Unlike [`draw_colored_polygon`][`crate::classes::CanvasItem::draw_colored_polygon`], each point's color can be changed individually. The `points` array is defined in local space. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_polygon_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExDrawPolygon < 'ex > {
            ExDrawPolygon::new(self, points, colors,)
        }
        #[doc = "Draws a colored polygon of any number of points, convex or concave. The points in the `points` array are defined in local space. Unlike [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], a single color must be specified for the whole polygon.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_colored_polygon_full(&mut self, points: RefArg < PackedVector2Array >, color: Color, uvs: RefArg < PackedVector2Array >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, PackedVector2Array >, Color, RefArg < 'a1, PackedVector2Array >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (points, color, uvs, texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_colored_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_colored_polygon_ex`][Self::draw_colored_polygon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a colored polygon of any number of points, convex or concave. The points in the `points` array are defined in local space. Unlike [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], a single color must be specified for the whole polygon.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_colored_polygon(&mut self, points: &PackedVector2Array, color: Color,) {
            self.draw_colored_polygon_ex(points, color,) . done()
        }
        #[doc = "Draws a colored polygon of any number of points, convex or concave. The points in the `points` array are defined in local space. Unlike [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`], a single color must be specified for the whole polygon.\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`].\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_colored_polygon_ex < 'ex > (&'ex mut self, points: &'ex PackedVector2Array, color: Color,) -> ExDrawColoredPolygon < 'ex > {
            ExDrawColoredPolygon::new(self, points, color,)
        }
        #[doc = "Draws `text` using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Example:** Draw \"Hello world\", using the project's default font:\n\n\n```gdscript\ndraw_string(ThemeDB.fallback_font, Vector2(64, 64), \"Hello world\", HORIZONTAL_ALIGNMENT_LEFT, -1, ThemeDB.fallback_font_size)\n```\n\n\nSee also [`draw_string`][`crate::classes::Font::draw_string`]."]
        pub(crate) fn draw_string_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_string_ex`][Self::draw_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws `text` using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Example:** Draw \"Hello world\", using the project's default font:\n\n\n```gdscript\ndraw_string(ThemeDB.fallback_font, Vector2(64, 64), \"Hello world\", HORIZONTAL_ALIGNMENT_LEFT, -1, ThemeDB.fallback_font_size)\n```\n\n\nSee also [`draw_string`][`crate::classes::Font::draw_string`]."]
        #[inline]
        pub fn draw_string(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_ex(font, pos, text,) . done()
        }
        #[doc = "Draws `text` using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.\n\n**Example:** Draw \"Hello world\", using the project's default font:\n\n\n```gdscript\ndraw_string(ThemeDB.fallback_font, Vector2(64, 64), \"Hello world\", HORIZONTAL_ALIGNMENT_LEFT, -1, ThemeDB.fallback_font_size)\n```\n\n\nSee also [`draw_string`][`crate::classes::Font::draw_string`]."]
        #[inline]
        pub fn draw_string_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawString < 'ex > {
            ExDrawString::new(self, font, pos, text,)
        }
        #[doc = "Breaks `text` into lines and draws it using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_multiline_string_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_string_ex`][Self::draw_multiline_string_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks `text` into lines and draws it using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_multiline_string(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_ex(font, pos, text,) . done()
        }
        #[doc = "Breaks `text` into lines and draws it using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_multiline_string_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawMultilineString < 'ex > {
            ExDrawMultilineString::new(self, font, pos, text,)
        }
        #[doc = "Draws `text` outline using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_string_outline_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, Color, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_string_outline_ex`][Self::draw_string_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws `text` outline using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_string_outline(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_string_outline_ex(font, pos, text,) . done()
        }
        #[doc = "Draws `text` outline using the specified `font` at the `pos` in local space (bottom-left corner using the baseline of the font). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_string_outline_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawStringOutline < 'ex > {
            ExDrawStringOutline::new(self, font, pos, text,)
        }
        #[doc = "Breaks `text` to the lines and draws text outline using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        pub(crate) fn draw_multiline_string_outline_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, crate::global::HorizontalAlignment, f32, i32, i32, i32, Color, crate::classes::text_server::LineBreakFlag, crate::classes::text_server::JustificationFlag, crate::classes::text_server::Direction, crate::classes::text_server::Orientation, f32,);
            let args = (font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multiline_string_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_multiline_string_outline_ex`][Self::draw_multiline_string_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Breaks `text` to the lines and draws text outline using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_multiline_string_outline(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, text: impl AsArg < GString >,) {
            self.draw_multiline_string_outline_ex(font, pos, text,) . done()
        }
        #[doc = "Breaks `text` to the lines and draws text outline using the specified `font` at the `pos` in local space (top-left corner). The text will have its color multiplied by `modulate`. If `width` is greater than or equal to 0, the text will be clipped if it exceeds the specified width. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used."]
        #[inline]
        pub fn draw_multiline_string_outline_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> ExDrawMultilineStringOutline < 'ex > {
            ExDrawMultilineStringOutline::new(self, font, pos, text,)
        }
        #[doc = "Draws a string first character using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        pub(crate) fn draw_char_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, char: CowArg < GString >, font_size: i32, modulate: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, i32, Color, f32,);
            let args = (font, pos, char, font_size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_char", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_char_ex`][Self::draw_char_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a string first character using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        #[inline]
        pub fn draw_char(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, char: impl AsArg < GString >,) {
            self.draw_char_ex(font, pos, char,) . done()
        }
        #[doc = "Draws a string first character using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        #[inline]
        pub fn draw_char_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, char: impl AsArg < GString > + 'ex,) -> ExDrawChar < 'ex > {
            ExDrawChar::new(self, font, pos, char,)
        }
        #[doc = "Draws a string first character outline using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        pub(crate) fn draw_char_outline_full(&self, font: CowArg < Gd < crate::classes::Font > >, pos: Vector2, char: CowArg < GString >, font_size: i32, size: i32, modulate: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Font > >, Vector2, CowArg < 'a1, GString >, i32, i32, Color, f32,);
            let args = (font, pos, char, font_size, size, modulate, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_char_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_char_outline_ex`][Self::draw_char_outline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a string first character outline using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        #[inline]
        pub fn draw_char_outline(&self, font: impl AsArg < Gd < crate::classes::Font >>, pos: Vector2, char: impl AsArg < GString >,) {
            self.draw_char_outline_ex(font, pos, char,) . done()
        }
        #[doc = "Draws a string first character outline using a custom font. If `oversampling` is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used. `pos` is defined in local space."]
        #[inline]
        pub fn draw_char_outline_ex < 'ex > (&'ex self, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, char: impl AsArg < GString > + 'ex,) -> ExDrawCharOutline < 'ex > {
            ExDrawCharOutline::new(self, font, pos, char,)
        }
        #[doc = "Draws a [`Mesh`][crate::classes::Mesh] in 2D, using the provided texture. See [`MeshInstance2D`][crate::classes::MeshInstance2D] for related documentation. The `transform` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub(crate) fn draw_mesh_full(&mut self, mesh: CowArg < Gd < crate::classes::Mesh > >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, transform: Transform2D, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::Mesh > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, Transform2D, Color,);
            let args = (mesh, texture, transform, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_mesh_ex`][Self::draw_mesh_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a [`Mesh`][crate::classes::Mesh] in 2D, using the provided texture. See [`MeshInstance2D`][crate::classes::MeshInstance2D] for related documentation. The `transform` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_mesh(&mut self, mesh: impl AsArg < Gd < crate::classes::Mesh >>, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.draw_mesh_ex(mesh, texture,) . done()
        }
        #[doc = "Draws a [`Mesh`][crate::classes::Mesh] in 2D, using the provided texture. See [`MeshInstance2D`][crate::classes::MeshInstance2D] for related documentation. The `transform` is defined in local space.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        #[inline]
        pub fn draw_mesh_ex < 'ex > (&'ex mut self, mesh: impl AsArg < Gd < crate::classes::Mesh >> + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExDrawMesh < 'ex > {
            ExDrawMesh::new(self, mesh, texture,)
        }
        #[doc = "Draws a [`MultiMesh`][crate::classes::MultiMesh] in 2D with the provided texture. See [`MultiMeshInstance2D`][crate::classes::MultiMeshInstance2D] for related documentation.\n\n**Note:** Styleboxes, textures, and meshes stored only inside local variables should **not** be used with this method in GDScript, because the drawing operation doesn't begin immediately once this method is called. In GDScript, when the function with the local variables ends, the local variables get destroyed before the rendering takes place."]
        pub fn draw_multimesh(&mut self, multimesh: impl AsArg < Gd < crate::classes::MultiMesh >>, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Gd < crate::classes::MultiMesh > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (multimesh.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_multimesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom local transform for drawing via components. Anything drawn afterwards will be transformed by this.\n\n**Note:** \\[member FontFile.oversampling] does _not_ take `scale` into account. This means that scaling up/down will cause bitmap fonts and rasterized (non-MSDF) dynamic fonts to appear blurry or pixelated. To ensure text remains crisp regardless of scale, you can enable MSDF font rendering by enabling \\[member ProjectSettings.gui/theme/default_font_multichannel_signed_distance_field] (applies to the default project font only), or enabling **Multichannel Signed Distance Field** in the import options of a DynamicFont for custom fonts. On system fonts, \\[member SystemFont.multichannel_signed_distance_field] can be enabled in the inspector."]
        pub(crate) fn draw_set_transform_full(&mut self, position: Vector2, rotation: f32, scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, f32, Vector2,);
            let args = (position, rotation, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_set_transform_ex`][Self::draw_set_transform_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a custom local transform for drawing via components. Anything drawn afterwards will be transformed by this.\n\n**Note:** \\[member FontFile.oversampling] does _not_ take `scale` into account. This means that scaling up/down will cause bitmap fonts and rasterized (non-MSDF) dynamic fonts to appear blurry or pixelated. To ensure text remains crisp regardless of scale, you can enable MSDF font rendering by enabling \\[member ProjectSettings.gui/theme/default_font_multichannel_signed_distance_field] (applies to the default project font only), or enabling **Multichannel Signed Distance Field** in the import options of a DynamicFont for custom fonts. On system fonts, \\[member SystemFont.multichannel_signed_distance_field] can be enabled in the inspector."]
        #[inline]
        pub fn draw_set_transform(&mut self, position: Vector2,) {
            self.draw_set_transform_ex(position,) . done()
        }
        #[doc = "Sets a custom local transform for drawing via components. Anything drawn afterwards will be transformed by this.\n\n**Note:** \\[member FontFile.oversampling] does _not_ take `scale` into account. This means that scaling up/down will cause bitmap fonts and rasterized (non-MSDF) dynamic fonts to appear blurry or pixelated. To ensure text remains crisp regardless of scale, you can enable MSDF font rendering by enabling \\[member ProjectSettings.gui/theme/default_font_multichannel_signed_distance_field] (applies to the default project font only), or enabling **Multichannel Signed Distance Field** in the import options of a DynamicFont for custom fonts. On system fonts, \\[member SystemFont.multichannel_signed_distance_field] can be enabled in the inspector."]
        #[inline]
        pub fn draw_set_transform_ex < 'ex > (&'ex mut self, position: Vector2,) -> ExDrawSetTransform < 'ex > {
            ExDrawSetTransform::new(self, position,)
        }
        #[doc = "Sets a custom local transform for drawing via matrix. Anything drawn afterwards will be transformed by this."]
        pub fn draw_set_transform_matrix(&mut self, xform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_set_transform_matrix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        pub(crate) fn draw_animation_slice_full(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type CallRet = ();
            type CallParams = (f64, f64, f64, f64,);
            let args = (animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_animation_slice", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`draw_animation_slice_ex`][Self::draw_animation_slice_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        #[inline]
        pub fn draw_animation_slice(&mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.draw_animation_slice_ex(animation_length, slice_begin, slice_end,) . done()
        }
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        #[inline]
        pub fn draw_animation_slice_ex < 'ex > (&'ex mut self, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExDrawAnimationSlice < 'ex > {
            ExDrawAnimationSlice::new(self, animation_length, slice_begin, slice_end,)
        }
        #[doc = "After submitting all animations slices via [`draw_animation_slice`][`crate::classes::CanvasItem::draw_animation_slice`], this function can be used to revert drawing to its default state (all subsequent drawing commands will be visible). If you don't care about this particular use case, usage of this function after submitting the slices is not required."]
        pub fn draw_end_animation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "draw_end_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform matrix of this `CanvasItem`."]
        pub fn get_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the global transform matrix of this item, i.e. the combined transform up to the topmost `CanvasItem` node. The topmost item is a `CanvasItem` that either has no parent, has non-`CanvasItem` parent or it has \\[member top_level] enabled."]
        pub fn get_global_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform from the local coordinate system of this `CanvasItem` to the [`Viewport`][crate::classes::Viewport]s coordinate system."]
        pub fn get_global_transform_with_canvas(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_transform_with_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform of this node, converted from its registered canvas's coordinate system to its viewport embedder's coordinate system. See also [`get_final_transform`][`crate::classes::Viewport::get_final_transform`] and [`get_viewport`][`crate::classes::Node::get_viewport`]."]
        pub fn get_viewport_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_viewport_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns this node's viewport boundaries as a [`Rect2`][crate::builtin::Rect2]. See also [`get_viewport`][`crate::classes::Node::get_viewport`]."]
        pub fn get_viewport_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_viewport_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform of this node, converted from its registered canvas's coordinate system to its viewport's coordinate system. See also [`get_viewport`][`crate::classes::Node::get_viewport`]."]
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transform of this `CanvasItem` in global screen coordinates (i.e. taking window position into account). Mostly useful for editor plugins.\n\nEquivalent to [`get_global_transform_with_canvas`][`crate::classes::CanvasItem::get_global_transform_with_canvas`] if the window is embedded (see \\[member Viewport.gui_embed_subwindows])."]
        pub fn get_screen_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_screen_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the mouse's position in this `CanvasItem` using the local coordinate system of this `CanvasItem`."]
        pub fn get_local_mouse_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_local_mouse_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns mouse cursor's global position relative to the [`CanvasLayer`][crate::classes::CanvasLayer] that contains this node.\n\n**Note:** For screen-space coordinates (e.g. when using a non-embedded [`Popup`][crate::classes::Popup]), you can use [`mouse_get_position`][`crate::classes::DisplayServer::mouse_get_position`]."]
        pub fn get_global_mouse_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_global_mouse_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RID`][crate::builtin::Rid] of the [`World2D`][crate::classes::World2D] canvas where this node is registered to, used by the [`RenderingServer`][crate::classes::RenderingServer]."]
        pub fn get_canvas(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`CanvasLayer`][crate::classes::CanvasLayer] that contains this node, or `null` if the node is not in any [`CanvasLayer`][crate::classes::CanvasLayer]."]
        pub fn get_canvas_layer_node(&self,) -> Option < Gd < crate::classes::CanvasLayer > > {
            type CallRet = Option < Gd < crate::classes::CanvasLayer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_canvas_layer_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`World2D`][crate::classes::World2D] this node is registered to.\n\nUsually, this is the same as this node's viewport (see [`get_viewport`][`crate::classes::Node::get_viewport`] and [`find_world_2d`][`crate::classes::Viewport::find_world_2d`])."]
        pub fn get_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallRet = Option < Gd < crate::classes::World2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_world_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the value of a shader uniform for this instance only ([per-instance uniform]($DOCS_URL/tutorials/shaders/shader_reference/shading_language.html#per-instance-uniforms)). See also [`set_shader_parameter`][`crate::classes::ShaderMaterial::set_shader_parameter`] to assign a uniform on all instances using the same [`ShaderMaterial`][crate::classes::ShaderMaterial].\n\n**Note:** For a shader uniform to be assignable on a per-instance basis, it _must_ be defined with `instance uniform ...` rather than `uniform ...` in the shader code.\n\n**Note:** `name` is case-sensitive and must match the name of the uniform in the code exactly (not the capitalized name in the inspector)."]
        pub fn set_instance_shader_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Get the value of a shader parameter as set on this instance."]
        pub fn get_instance_shader_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_parent_material(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_parent_material(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the node will receive [`CanvasItemNotification::LOCAL_TRANSFORM_CHANGED`][`crate::classes::notify::CanvasItemNotification::LOCAL_TRANSFORM_CHANGED`] whenever its local transform changes.\n\n**Note:** Many canvas items such as [`Bone2D`][crate::classes::Bone2D] or [`CollisionShape2D`][crate::classes::CollisionShape2D] automatically enable this in order to function correctly."]
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_notify_local_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node receives [`CanvasItemNotification::LOCAL_TRANSFORM_CHANGED`][`crate::classes::notify::CanvasItemNotification::LOCAL_TRANSFORM_CHANGED`] whenever its local transform changes. This is enabled with [`set_notify_local_transform`][`crate::classes::CanvasItem::set_notify_local_transform`]."]
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_local_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the node will receive [`CanvasItemNotification::TRANSFORM_CHANGED`][`crate::classes::notify::CanvasItemNotification::TRANSFORM_CHANGED`] whenever its global transform changes.\n\n**Note:** Many canvas items such as [`Camera2D`][crate::classes::Camera2D] or [`Light2D`][crate::classes::Light2D] automatically enable this in order to function correctly."]
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_notify_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the node receives [`CanvasItemNotification::TRANSFORM_CHANGED`][`crate::classes::notify::CanvasItemNotification::TRANSFORM_CHANGED`] whenever its global transform changes. This is enabled with [`set_notify_transform`][`crate::classes::CanvasItem::set_notify_transform`]."]
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "is_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces the node's transform to update. Fails if the node is not inside the tree. See also [`get_transform`][`crate::classes::CanvasItem::get_transform`].\n\n**Note:** For performance reasons, transform changes are usually accumulated and applied _once_ at the end of the frame. The update propagates through `CanvasItem` children, as well. Therefore, use this method only when you need an up-to-date transform (such as during physics operations)."]
        pub fn force_update_transform(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "force_update_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms `viewport_point` from the viewport's coordinates to this node's local coordinates.\n\nFor the opposite operation, use [`get_global_transform_with_canvas`][`crate::classes::CanvasItem::get_global_transform_with_canvas`].\n\n```gdscript\nvar viewport_point = get_global_transform_with_canvas() * local_point\n```"]
        pub fn make_canvas_position_local(&self, viewport_point: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (viewport_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "make_canvas_position_local", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a copy of the given `event` with its coordinates converted from global space to this `CanvasItem`'s local space. If not possible, returns the same [`InputEvent`][crate::classes::InputEvent] unchanged."]
        pub fn make_input_local(&self, event: impl AsArg < Gd < crate::classes::InputEvent >>,) -> Gd < crate::classes::InputEvent > {
            type CallRet = Gd < crate::classes::InputEvent >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::InputEvent > >,);
            let args = (event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "make_input_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set/clear individual bits on the rendering visibility layer. This simplifies editing this `CanvasItem`'s visibility layer."]
        pub fn set_visibility_layer_bit(&mut self, layer: u32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (u32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_visibility_layer_bit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the layer at the given index is set in \\[member visibility_layer]."]
        pub fn get_visibility_layer_bit(&self, layer: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_visibility_layer_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::canvas_item::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::TextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::canvas_item::TextureFilter {
            type CallRet = crate::classes::canvas_item::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, mode: crate::classes::canvas_item::TextureRepeat,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::TextureRepeat,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::classes::canvas_item::TextureRepeat {
            type CallRet = crate::classes::canvas_item::TextureRepeat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_children_mode(&mut self, mode: crate::classes::canvas_item::ClipChildrenMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item::ClipChildrenMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "set_clip_children_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_children_mode(&self,) -> crate::classes::canvas_item::ClipChildrenMode {
            type CallRet = crate::classes::canvas_item::ClipChildrenMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItem", "get_clip_children_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 35i32;
        pub(crate) const NOTIFICATION_DRAW: i32 = 30i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 31i32;
        pub(crate) const NOTIFICATION_ENTER_CANVAS: i32 = 32i32;
        pub(crate) const NOTIFICATION_EXIT_CANVAS: i32 = 33i32;
        pub(crate) const NOTIFICATION_WORLD_2D_CHANGED: i32 = 36i32;
        
    }
    impl crate::obj::GodotClass for CanvasItem {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CanvasItem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CanvasItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CanvasItem {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CanvasItem {
        
    }
    impl std::ops::Deref for CanvasItem {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CanvasItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CanvasItem__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CanvasItem` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_line_ex`][super::CanvasItem::draw_line_ex]."]
#[must_use]
pub struct ExDrawLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_line_full(surround_object, from, to, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_dashed_line_ex`][super::CanvasItem::draw_dashed_line_ex]."]
#[must_use]
pub struct ExDrawDashedLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color, width: f32, dash: f32, aligned: bool, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawDashedLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let dash = 2f32;
        let aligned = true;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, color: color, width: width, dash: dash, aligned: aligned, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn dash(self, dash: f32) -> Self {
        Self {
            dash: dash, .. self
        }
    }
    #[inline]
    pub fn aligned(self, aligned: bool) -> Self {
        Self {
            aligned: aligned, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to, color, width, dash, aligned, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_dashed_line_full(surround_object, from, to, color, width, dash, aligned, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_ex`][super::CanvasItem::draw_polyline_ex]."]
#[must_use]
pub struct ExDrawPolyline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawPolyline < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_polyline_full(surround_object, points.cow_as_arg(), color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polyline_colors_ex`][super::CanvasItem::draw_polyline_colors_ex]."]
#[must_use]
pub struct ExDrawPolylineColors < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawPolylineColors < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_polyline_colors_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_ellipse_arc_ex`][super::CanvasItem::draw_ellipse_arc_ex]."]
#[must_use]
pub struct ExDrawEllipseArc < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, center: Vector2, major: f32, minor: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawEllipseArc < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, center: Vector2, major: f32, minor: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, center: center, major: major, minor: minor, start_angle: start_angle, end_angle: end_angle, point_count: point_count, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, center, major, minor, start_angle, end_angle, point_count, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_ellipse_arc_full(surround_object, center, major, minor, start_angle, end_angle, point_count, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_arc_ex`][super::CanvasItem::draw_arc_ex]."]
#[must_use]
pub struct ExDrawArc < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawArc < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, point_count: i32, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, center: center, radius: radius, start_angle: start_angle, end_angle: end_angle, point_count: point_count, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, center, radius, start_angle, end_angle, point_count, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_arc_full(surround_object, center, radius, start_angle, end_angle, point_count, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_ex`][super::CanvasItem::draw_multiline_ex]."]
#[must_use]
pub struct ExDrawMultiline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultiline < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_multiline_full(surround_object, points.cow_as_arg(), color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_colors_ex`][super::CanvasItem::draw_multiline_colors_ex]."]
#[must_use]
pub struct ExDrawMultilineColors < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultilineColors < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_multiline_colors_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_rect_ex`][super::CanvasItem::draw_rect_ex]."]
#[must_use]
pub struct ExDrawRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, rect: Rect2, color: Color, filled: bool, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawRect < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, rect: Rect2, color: Color,) -> Self {
        let filled = true;
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rect: rect, color: color, filled: filled, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled: filled, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, rect, color, filled, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_rect_full(surround_object, rect, color, filled, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_circle_ex`][super::CanvasItem::draw_circle_ex]."]
#[must_use]
pub struct ExDrawCircle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, position: Vector2, radius: f32, color: Color, filled: bool, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawCircle < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, position: Vector2, radius: f32, color: Color,) -> Self {
        let filled = true;
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, radius: radius, color: color, filled: filled, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled: filled, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, radius, color, filled, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_circle_full(surround_object, position, radius, color, filled, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_ellipse_ex`][super::CanvasItem::draw_ellipse_ex]."]
#[must_use]
pub struct ExDrawEllipse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, position: Vector2, major: f32, minor: f32, color: Color, filled: bool, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawEllipse < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, position: Vector2, major: f32, minor: f32, color: Color,) -> Self {
        let filled = true;
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, major: major, minor: minor, color: color, filled: filled, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn filled(self, filled: bool) -> Self {
        Self {
            filled: filled, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, major, minor, color, filled, width, antialiased,
        }
        = self;
        re_export::CanvasItem::draw_ellipse_full(surround_object, position, major, minor, color, filled, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_ex`][super::CanvasItem::draw_texture_ex]."]
#[must_use]
pub struct ExDrawTexture < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, texture: CowArg < 'ex, Gd < crate::classes::Texture2D > >, position: Vector2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawTexture < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, position: Vector2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), position: position, modulate: modulate,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, position, modulate,
        }
        = self;
        re_export::CanvasItem::draw_texture_full(surround_object, texture, position, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_ex`][super::CanvasItem::draw_texture_rect_ex]."]
#[must_use]
pub struct ExDrawTextureRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, texture: CowArg < 'ex, Gd < crate::classes::Texture2D > >, rect: Rect2, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawTextureRect < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, tile: bool,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, tile: tile, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, tile, modulate, transpose,
        }
        = self;
        re_export::CanvasItem::draw_texture_rect_full(surround_object, texture, rect, tile, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_texture_rect_region_ex`][super::CanvasItem::draw_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawTextureRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, texture: CowArg < 'ex, Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawTextureRectRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        let clip_uv = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate, transpose: transpose, clip_uv: clip_uv,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, clip_uv: bool) -> Self {
        Self {
            clip_uv: clip_uv, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate, transpose, clip_uv,
        }
        = self;
        re_export::CanvasItem::draw_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate, transpose, clip_uv,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_msdf_texture_rect_region_ex`][super::CanvasItem::draw_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawMsdfTextureRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, texture: CowArg < 'ex, Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color, outline: f64, pixel_range: f64, scale: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMsdfTextureRectRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline = 0f64;
        let pixel_range = 4f64;
        let scale = 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate, outline: outline, pixel_range: pixel_range, scale: scale,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn outline(self, outline: f64) -> Self {
        Self {
            outline: outline, .. self
        }
    }
    #[inline]
    pub fn pixel_range(self, pixel_range: f64) -> Self {
        Self {
            pixel_range: pixel_range, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: f64) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate, outline, pixel_range, scale,
        }
        = self;
        re_export::CanvasItem::draw_msdf_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate, outline, pixel_range, scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_lcd_texture_rect_region_ex`][super::CanvasItem::draw_lcd_texture_rect_region_ex]."]
#[must_use]
pub struct ExDrawLcdTextureRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, texture: CowArg < 'ex, Gd < crate::classes::Texture2D > >, rect: Rect2, src_rect: Rect2, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawLcdTextureRectRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, texture: impl AsArg < Gd < crate::classes::Texture2D >> + 'ex, rect: Rect2, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), rect: rect, src_rect: src_rect, modulate: modulate,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, rect, src_rect, modulate,
        }
        = self;
        re_export::CanvasItem::draw_lcd_texture_rect_region_full(surround_object, texture, rect, src_rect, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_primitive_ex`][super::CanvasItem::draw_primitive_ex]."]
#[must_use]
pub struct ExDrawPrimitive < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, uvs: CowArg < 'ex, PackedVector2Array >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawPrimitive < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray, uvs: &'ex PackedVector2Array,) -> Self {
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Borrowed(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_primitive_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_polygon_ex`][super::CanvasItem::draw_polygon_ex]."]
#[must_use]
pub struct ExDrawPolygon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, uvs: CowArg < 'ex, PackedVector2Array >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawPolygon < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'ex PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, colors, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_polygon_full(surround_object, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_colored_polygon_ex`][super::CanvasItem::draw_colored_polygon_ex]."]
#[must_use]
pub struct ExDrawColoredPolygon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, points: CowArg < 'ex, PackedVector2Array >, color: Color, uvs: CowArg < 'ex, PackedVector2Array >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawColoredPolygon < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, points: &'ex PackedVector2Array, color: Color,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, points: CowArg::Borrowed(points), color: color, uvs: CowArg::Owned(uvs), texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'ex PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, points, color, uvs, texture,
        }
        = self;
        re_export::CanvasItem::draw_colored_polygon_full(surround_object, points.cow_as_arg(), color, uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_ex`][super::CanvasItem::draw_string_ex]."]
#[must_use]
pub struct ExDrawString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawString < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_string_full(surround_object, font, pos, text, alignment, width, font_size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_ex`][super::CanvasItem::draw_multiline_string_ex]."]
#[must_use]
pub struct ExDrawMultilineString < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultilineString < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_multiline_string_full(surround_object, font, pos, text, alignment, width, font_size, max_lines, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_string_outline_ex`][super::CanvasItem::draw_string_outline_ex]."]
#[must_use]
pub struct ExDrawStringOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, size: i32, modulate: Color, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawStringOutline < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, size: size, modulate: modulate, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_string_outline_full(surround_object, font, pos, text, alignment, width, font_size, size, modulate, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_multiline_string_outline_ex`][super::CanvasItem::draw_multiline_string_outline_ex]."]
#[must_use]
pub struct ExDrawMultilineStringOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, text: CowArg < 'ex, GString >, alignment: crate::global::HorizontalAlignment, width: f32, font_size: i32, max_lines: i32, size: i32, modulate: Color, brk_flags: crate::classes::text_server::LineBreakFlag, justification_flags: crate::classes::text_server::JustificationFlag, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMultilineStringOutline < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, text: impl AsArg < GString > + 'ex,) -> Self {
        let alignment = crate::obj::EngineEnum::from_ord(0);
        let width = - 1f32;
        let font_size = 16i32;
        let max_lines = - 1i32;
        let size = 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let brk_flags = crate::obj::EngineBitfield::from_ord(3);
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, text: text.into_arg(), alignment: alignment, width: width, font_size: font_size, max_lines: max_lines, size: size, modulate: modulate, brk_flags: brk_flags, justification_flags: justification_flags, direction: direction, orientation: orientation, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn max_lines(self, max_lines: i32) -> Self {
        Self {
            max_lines: max_lines, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn brk_flags(self, brk_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            brk_flags: brk_flags, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_multiline_string_outline_full(surround_object, font, pos, text, alignment, width, font_size, max_lines, size, modulate, brk_flags, justification_flags, direction, orientation, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_ex`][super::CanvasItem::draw_char_ex]."]
#[must_use]
pub struct ExDrawChar < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, char: CowArg < 'ex, GString >, font_size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawChar < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, char: impl AsArg < GString > + 'ex,) -> Self {
        let font_size = 16i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, char: char.into_arg(), font_size: font_size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, char, font_size, modulate, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_char_full(surround_object, font, pos, char, font_size, modulate, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_char_outline_ex`][super::CanvasItem::draw_char_outline_ex]."]
#[must_use]
pub struct ExDrawCharOutline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::CanvasItem, font: CowArg < 'ex, Gd < crate::classes::Font > >, pos: Vector2, char: CowArg < 'ex, GString >, font_size: i32, size: i32, modulate: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawCharOutline < 'ex > {
    fn new(surround_object: &'ex re_export::CanvasItem, font: impl AsArg < Gd < crate::classes::Font >> + 'ex, pos: Vector2, char: impl AsArg < GString > + 'ex,) -> Self {
        let font_size = 16i32;
        let size = - 1i32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), pos: pos, char: char.into_arg(), font_size: font_size, size: size, modulate: modulate, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: size, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, pos, char, font_size, size, modulate, oversampling,
        }
        = self;
        re_export::CanvasItem::draw_char_outline_full(surround_object, font, pos, char, font_size, size, modulate, oversampling,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_mesh_ex`][super::CanvasItem::draw_mesh_ex]."]
#[must_use]
pub struct ExDrawMesh < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, mesh: CowArg < 'ex, Gd < crate::classes::Mesh > >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, transform: Transform2D, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawMesh < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, mesh: impl AsArg < Gd < crate::classes::Mesh >> + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.into_arg(), texture: texture.into_arg(), transform: transform, modulate: modulate,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mesh, texture, transform, modulate,
        }
        = self;
        re_export::CanvasItem::draw_mesh_full(surround_object, mesh, texture, transform, modulate,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_set_transform_ex`][super::CanvasItem::draw_set_transform_ex]."]
#[must_use]
pub struct ExDrawSetTransform < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, position: Vector2, rotation: f32, scale: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawSetTransform < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, position: Vector2,) -> Self {
        let rotation = 0f32;
        let scale = Vector2::new(1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, rotation: rotation, scale: scale,
        }
    }
    #[inline]
    pub fn rotation(self, rotation: f32) -> Self {
        Self {
            rotation: rotation, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: Vector2) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, rotation, scale,
        }
        = self;
        re_export::CanvasItem::draw_set_transform_full(surround_object, position, rotation, scale,)
    }
}
#[doc = "Default-param extender for [`CanvasItem::draw_animation_slice_ex`][super::CanvasItem::draw_animation_slice_ex]."]
#[must_use]
pub struct ExDrawAnimationSlice < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDrawAnimationSlice < 'ex > {
    fn new(surround_object: &'ex mut re_export::CanvasItem, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        let offset = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, animation_length: animation_length, slice_begin: slice_begin, slice_end: slice_end, offset: offset,
        }
    }
    #[inline]
    pub fn offset(self, offset: f64) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, animation_length, slice_begin, slice_end, offset,
        }
        = self;
        re_export::CanvasItem::draw_animation_slice_full(surround_object, animation_length, slice_begin, slice_end, offset,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    #[doc(alias = "TEXTURE_FILTER_PARENT_NODE")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_PARENT_NODE`"]
    pub const PARENT_NODE: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureFilter {
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
            Self::PARENT_NODE => "PARENT_NODE", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureFilter::PARENT_NODE, TextureFilter::NEAREST, TextureFilter::LINEAR, TextureFilter::NEAREST_WITH_MIPMAPS, TextureFilter::LINEAR_WITH_MIPMAPS, TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC, TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PARENT_NODE", "TEXTURE_FILTER_PARENT_NODE", TextureFilter::PARENT_NODE), crate::meta::inspect::EnumConstant::new("NEAREST", "TEXTURE_FILTER_NEAREST", TextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "TEXTURE_FILTER_LINEAR", TextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", TextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", TextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_FILTER_MAX", TextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Texture Filter Parent Node", 0i64), EnumeratorShape::new_int("Texture Filter Nearest", 1i64), EnumeratorShape::new_int("Texture Filter Linear", 2i64), EnumeratorShape::new_int("Texture Filter Nearest With Mipmaps", 3i64), EnumeratorShape::new_int("Texture Filter Linear With Mipmaps", 4i64), EnumeratorShape::new_int("Texture Filter Nearest With Mipmaps Anisotropic", 5i64), EnumeratorShape::new_int("Texture Filter Linear With Mipmaps Anisotropic", 6i64), EnumeratorShape::new_int("Texture Filter Max", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CanvasItem.TextureFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextureFilter {
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
impl crate::registry::property::Export for TextureFilter {
    
}
impl crate::meta::Element for TextureFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    #[doc(alias = "TEXTURE_REPEAT_PARENT_NODE")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_PARENT_NODE`"]
    pub const PARENT_NODE: TextureRepeat = TextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: TextureRepeat = TextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: TextureRepeat = TextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: TextureRepeat = TextureRepeat {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_REPEAT_MAX`"]
    pub const MAX: TextureRepeat = TextureRepeat {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureRepeat {
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
            Self::PARENT_NODE => "PARENT_NODE", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureRepeat::PARENT_NODE, TextureRepeat::DISABLED, TextureRepeat::ENABLED, TextureRepeat::MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PARENT_NODE", "TEXTURE_REPEAT_PARENT_NODE", TextureRepeat::PARENT_NODE), crate::meta::inspect::EnumConstant::new("DISABLED", "TEXTURE_REPEAT_DISABLED", TextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "TEXTURE_REPEAT_ENABLED", TextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("MIRROR", "TEXTURE_REPEAT_MIRROR", TextureRepeat::MIRROR), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_REPEAT_MAX", TextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Texture Repeat Parent Node", 0i64), EnumeratorShape::new_int("Texture Repeat Disabled", 1i64), EnumeratorShape::new_int("Texture Repeat Enabled", 2i64), EnumeratorShape::new_int("Texture Repeat Mirror", 3i64), EnumeratorShape::new_int("Texture Repeat Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CanvasItem.TextureRepeat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextureRepeat {
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
impl crate::registry::property::Export for TextureRepeat {
    
}
impl crate::meta::Element for TextureRepeat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClipChildrenMode {
    ord: i32
}
impl ClipChildrenMode {
    #[doc(alias = "CLIP_CHILDREN_DISABLED")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_DISABLED`"]
    pub const DISABLED: ClipChildrenMode = ClipChildrenMode {
        ord: 0i32
    };
    #[doc(alias = "CLIP_CHILDREN_ONLY")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_ONLY`"]
    pub const ONLY: ClipChildrenMode = ClipChildrenMode {
        ord: 1i32
    };
    #[doc(alias = "CLIP_CHILDREN_AND_DRAW")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_AND_DRAW`"]
    pub const AND_DRAW: ClipChildrenMode = ClipChildrenMode {
        ord: 2i32
    };
    #[doc(alias = "CLIP_CHILDREN_MAX")]
    #[doc = "Godot enumerator name: `CLIP_CHILDREN_MAX`"]
    pub const MAX: ClipChildrenMode = ClipChildrenMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ClipChildrenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ClipChildrenMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ClipChildrenMode {
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
            Self::DISABLED => "DISABLED", Self::ONLY => "ONLY", Self::AND_DRAW => "AND_DRAW", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ClipChildrenMode::DISABLED, ClipChildrenMode::ONLY, ClipChildrenMode::AND_DRAW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ClipChildrenMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CLIP_CHILDREN_DISABLED", ClipChildrenMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONLY", "CLIP_CHILDREN_ONLY", ClipChildrenMode::ONLY), crate::meta::inspect::EnumConstant::new("AND_DRAW", "CLIP_CHILDREN_AND_DRAW", ClipChildrenMode::AND_DRAW), crate::meta::inspect::EnumConstant::new("MAX", "CLIP_CHILDREN_MAX", ClipChildrenMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ClipChildrenMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ClipChildrenMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Clip Children Disabled", 0i64), EnumeratorShape::new_int("Clip Children Only", 1i64), EnumeratorShape::new_int("Clip Children And Draw", 2i64), EnumeratorShape::new_int("Clip Children Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("CanvasItem.ClipChildrenMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ClipChildrenMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ClipChildrenMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ClipChildrenMode {
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
impl crate::registry::property::Export for ClipChildrenMode {
    
}
impl crate::meta::Element for ClipChildrenMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CanvasItem;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CanvasItem`][crate::classes::CanvasItem] class."]
    pub struct SignalsOfCanvasItem < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCanvasItem < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn draw(&mut self) -> SigDraw < 'c, C > {
            SigDraw {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "draw")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn hidden(&mut self) -> SigHidden < 'c, C > {
            SigHidden {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "hidden")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_rect_changed(&mut self) -> SigItemRectChanged < 'c, C > {
            SigItemRectChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "item_rect_changed")
            }
        }
    }
    type TypedSigDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDraw < 'c, C: WithSignals > {
        typed: TypedSigDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDraw < 'c, C > {
        type Target = TypedSigDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVisibilityChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityChanged < 'c, C > {
        type Target = TypedSigVisibilityChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigHidden < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigHidden < 'c, C: WithSignals > {
        typed: TypedSigHidden < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigHidden < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigHidden < 'c, C > {
        type Target = TypedSigHidden < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigHidden < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemRectChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemRectChanged < 'c, C: WithSignals > {
        typed: TypedSigItemRectChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemRectChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemRectChanged < 'c, C > {
        type Target = TypedSigItemRectChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemRectChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CanvasItem {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCanvasItem < 'c, C > {
        type Target = < < CanvasItem as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CanvasItem;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCanvasItem < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CanvasItem;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}