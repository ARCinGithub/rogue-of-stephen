#![doc = "Sidecar module for class [`RenderingServer`][crate::classes::RenderingServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingServer` enums](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingServer`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`rendering_server`][crate::classes::rendering_server]: sidecar module with related enum/flag types\n* [`SignalsOfRenderingServer`][crate::classes::rendering_server::SignalsOfRenderingServer]: signal collection\n\n\nSee also [Godot docs for `RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe rendering server is the API backend for everything visible. The whole scene system mounts on it to display. The rendering server is completely opaque: the internals are entirely implementation-specific and cannot be accessed.\n\nThe rendering server can be used to bypass the scene/[`Node`][crate::classes::Node] system entirely. This can improve performance in cases where the scene system is the bottleneck, but won't improve performance otherwise (for instance, if the GPU is already fully utilized).\n\nResources are created using the `*_create` functions. These functions return [`RID`][crate::builtin::Rid]s which are not references to the objects themselves, but opaque _pointers_ towards these objects.\n\nAll objects are drawn to a viewport. You can use the [`Viewport`][crate::classes::Viewport] attached to the [`SceneTree`][crate::classes::SceneTree] or you can create one yourself with [`viewport_create`][`crate::classes::RenderingServer::viewport_create`]. When using a custom scenario or canvas, the scenario or canvas needs to be attached to the viewport using [`viewport_set_scenario`][`crate::classes::RenderingServer::viewport_set_scenario`] or [`viewport_attach_canvas`][`crate::classes::RenderingServer::viewport_attach_canvas`].\n\n**Scenarios:** In 3D, all visual objects must be associated with a scenario. The scenario is a visual representation of the world. If accessing the rendering server from a running game, the scenario can be accessed from the scene tree from any [`Node3D`][crate::classes::Node3D] node with [`get_world_3d`][`crate::classes::Node3D::get_world_3d`]. Otherwise, a scenario can be created with [`scenario_create`][`crate::classes::RenderingServer::scenario_create`].\n\nSimilarly, in 2D, a canvas is needed to draw all canvas items.\n\n**3D:** In 3D, all visible objects are comprised of a resource and an instance. A resource can be a mesh, a particle system, a light, or any other 3D object. In order to be visible resources must be attached to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`]. The instance must also be attached to the scenario using [`instance_set_scenario`][`crate::classes::RenderingServer::instance_set_scenario`] in order to be visible. RenderingServer methods that don't have a prefix are usually 3D-specific (but not always).\n\n**2D:** In 2D, all visible objects are some form of canvas item. In order to be visible, a canvas item needs to be the child of a canvas attached to a viewport, or it needs to be the child of another canvas item that is eventually attached to the canvas. 2D-specific RenderingServer methods generally start with `canvas_*`.\n\n**Headless mode:** Starting the engine with the `--headless` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html) disables all rendering and window management functions. Most functions from `RenderingServer` will return dummy values in this case."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl RenderingServer {
        #[doc = "Creates a 2-dimensional texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `texture_2d_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`Texture2D`][crate::classes::Texture2D].\n\n**Note:** Not to be confused with [`texture_create`][`crate::classes::RenderingDevice::texture_create`], which creates the graphics API's own texture type as opposed to the Godot-specific [`Texture2D`][crate::classes::Texture2D] resource."]
        pub fn texture_2d_create(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (image.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `texture_2d_layered_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`TextureLayered`][crate::classes::TextureLayered]."]
        pub fn texture_2d_layered_create(&mut self, layers: &Array < Gd < crate::classes::Image > >, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Image > > >, crate::classes::rendering_server::TextureLayeredType,);
            let args = (RefArg::new(layers), layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layered_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "**Note:** The equivalent resource is [`Texture3D`][crate::classes::Texture3D]."]
        pub fn texture_3d_create(&mut self, format: crate::classes::image::Format, width: i32, height: i32, depth: i32, mipmaps: bool, data: &Array < Gd < crate::classes::Image > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (crate::classes::image::Format, i32, i32, i32, bool, RefArg < 'a0, Array < Gd < crate::classes::Image > > >,);
            let args = (format, width, height, depth, mipmaps, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing and always returns an invalid [`RID`][crate::builtin::Rid]."]
        pub fn texture_proxy_create(&mut self, base: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_proxy_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a texture based on a native handle that was created outside of Godot's renderer.\n\n**Note:** If using only the rendering device renderer, it's recommend to use [`texture_create_from_extension`][`crate::classes::RenderingDevice::texture_create_from_extension`] together with [`texture_rd_create`][`crate::classes::RenderingServer::texture_rd_create`], rather than this method. This way, the texture's format and usage can be controlled more effectively."]
        pub(crate) fn texture_create_from_native_handle_full(&mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32, layers: i32, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::rendering_server::TextureType, crate::classes::image::Format, u64, i32, i32, i32, i32, crate::classes::rendering_server::TextureLayeredType,);
            let args = (type_, format, native_handle, width, height, depth, layers, layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_create_from_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`texture_create_from_native_handle_ex`][Self::texture_create_from_native_handle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a texture based on a native handle that was created outside of Godot's renderer.\n\n**Note:** If using only the rendering device renderer, it's recommend to use [`texture_create_from_extension`][`crate::classes::RenderingDevice::texture_create_from_extension`] together with [`texture_rd_create`][`crate::classes::RenderingServer::texture_rd_create`], rather than this method. This way, the texture's format and usage can be controlled more effectively."]
        #[inline]
        pub fn texture_create_from_native_handle(&mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> Rid {
            self.texture_create_from_native_handle_ex(type_, format, native_handle, width, height, depth,) . done()
        }
        #[doc = "Creates a texture based on a native handle that was created outside of Godot's renderer.\n\n**Note:** If using only the rendering device renderer, it's recommend to use [`texture_create_from_extension`][`crate::classes::RenderingDevice::texture_create_from_extension`] together with [`texture_rd_create`][`crate::classes::RenderingServer::texture_rd_create`], rather than this method. This way, the texture's format and usage can be controlled more effectively."]
        #[inline]
        pub fn texture_create_from_native_handle_ex < 'ex > (&'ex mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> ExTextureCreateFromNativeHandle < 'ex > {
            ExTextureCreateFromNativeHandle::new(self, type_, format, native_handle, width, height, depth,)
        }
        #[doc = "Updates the texture specified by the `texture` [`RID`][crate::builtin::Rid] with the data in `image`. A `layer` must also be specified, which should be `0` when updating a single-layer texture ([`Texture2D`][crate::classes::Texture2D]).\n\n**Note:** The `image` must have the same width, height and format as the current `texture` data. Otherwise, an error will be printed and the original texture won't be modified. If you need to use different width, height or format, use [`texture_replace`][`crate::classes::RenderingServer::texture_replace`] instead."]
        pub fn texture_2d_update(&mut self, texture: Rid, image: impl AsArg < Option < Gd < crate::classes::Image >> >, layer: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Image > > >, i32,);
            let args = (texture, image.into_arg(), layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the texture specified by the `texture` [`RID`][crate::builtin::Rid]'s data with the data in `data`. All the texture's layers must be replaced at once.\n\n**Note:** The `texture` must have the same width, height, depth and format as the current texture data. Otherwise, an error will be printed and the original texture won't be modified. If you need to use different width, height, depth or format, use [`texture_replace`][`crate::classes::RenderingServer::texture_replace`] instead."]
        pub fn texture_3d_update(&mut self, texture: Rid, data: &Array < Gd < crate::classes::Image > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Gd < crate::classes::Image > > >,);
            let args = (texture, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing."]
        pub fn texture_proxy_update(&mut self, texture: Rid, proxy_to: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (texture, proxy_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(93usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_proxy_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a placeholder for a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `texture_2d_layered_*` RenderingServer functions, although it does nothing when used. See also [`texture_2d_layered_placeholder_create`][`crate::classes::RenderingServer::texture_2d_layered_placeholder_create`].\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`PlaceholderTexture2D`][crate::classes::PlaceholderTexture2D]."]
        pub fn texture_2d_placeholder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a placeholder for a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `texture_2d_layered_*` RenderingServer functions, although it does nothing when used. See also [`texture_2d_placeholder_create`][`crate::classes::RenderingServer::texture_2d_placeholder_create`].\n\n**Note:** The equivalent resource is [`PlaceholderTextureLayered`][crate::classes::PlaceholderTextureLayered]."]
        pub fn texture_2d_layered_placeholder_create(&mut self, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::rendering_server::TextureLayeredType,);
            let args = (layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layered_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a placeholder for a 3-dimensional texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `texture_3d_*` RenderingServer functions, although it does nothing when used.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`PlaceholderTexture3D`][crate::classes::PlaceholderTexture3D]."]
        pub fn texture_3d_placeholder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] instance from the given `texture` [`RID`][crate::builtin::Rid].\n\n**Example:** Get the test texture from [`get_test_texture`][`crate::classes::RenderingServer::get_test_texture`] and apply it to a [`Sprite2D`][crate::classes::Sprite2D] node:\n\n```gdscript\nvar texture_rid = RenderingServer.get_test_texture()\nvar texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))\n$Sprite2D.texture = texture\n```"]
        pub fn texture_2d_get(&self, texture: Rid,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an [`Image`][crate::classes::Image] instance from the given `texture` [`RID`][crate::builtin::Rid] and `layer`."]
        pub fn texture_2d_layer_get(&self, texture: Rid, layer: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, i32,);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layer_get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns 3D texture data as an array of [`Image`][crate::classes::Image]s for the specified texture [`RID`][crate::builtin::Rid]."]
        pub fn texture_3d_get(&self, texture: Rid,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Replaces `texture`'s texture data by the texture specified by the `by_texture` RID, without changing `texture`'s RID."]
        pub fn texture_replace(&mut self, texture: Rid, by_texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (texture, by_texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_replace", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_size_override(&mut self, texture: Rid, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (texture, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_path(&mut self, texture: Rid, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (texture, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_path(&self, texture: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the format for the texture."]
        pub fn texture_get_format(&self, texture: Rid,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_force_redraw_if_visible(&mut self, texture: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (texture, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_force_redraw_if_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new texture object based on a texture created directly on the [`RenderingDevice`][crate::classes::RenderingDevice]. If the texture contains layers, `layer_type` is used to define the layer type.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] won't free the underlying `rd_texture`, you will want to free the `rd_texture` using [`free_rid`][`crate::classes::RenderingDevice::free_rid`]."]
        pub(crate) fn texture_rd_create_full(&mut self, rd_texture: Rid, layer_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, crate::classes::rendering_server::TextureLayeredType,);
            let args = (rd_texture, layer_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_rd_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`texture_rd_create_ex`][Self::texture_rd_create_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new texture object based on a texture created directly on the [`RenderingDevice`][crate::classes::RenderingDevice]. If the texture contains layers, `layer_type` is used to define the layer type.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] won't free the underlying `rd_texture`, you will want to free the `rd_texture` using [`free_rid`][`crate::classes::RenderingDevice::free_rid`]."]
        #[inline]
        pub fn texture_rd_create(&mut self, rd_texture: Rid,) -> Rid {
            self.texture_rd_create_ex(rd_texture,) . done()
        }
        #[doc = "Creates a new texture object based on a texture created directly on the [`RenderingDevice`][crate::classes::RenderingDevice]. If the texture contains layers, `layer_type` is used to define the layer type.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] won't free the underlying `rd_texture`, you will want to free the `rd_texture` using [`free_rid`][`crate::classes::RenderingDevice::free_rid`]."]
        #[inline]
        pub fn texture_rd_create_ex < 'ex > (&'ex mut self, rd_texture: Rid,) -> ExTextureRdCreate < 'ex > {
            ExTextureRdCreate::new(self, rd_texture,)
        }
        #[doc = "Returns a texture [`RID`][crate::builtin::Rid] that can be used with [`RenderingDevice`][crate::classes::RenderingDevice].\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding."]
        pub(crate) fn texture_get_rd_texture_full(&self, texture: Rid, srgb: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, bool,);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_rd_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`texture_get_rd_texture_ex`][Self::texture_get_rd_texture_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a texture [`RID`][crate::builtin::Rid] that can be used with [`RenderingDevice`][crate::classes::RenderingDevice].\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding."]
        #[inline]
        pub fn texture_get_rd_texture(&self, texture: Rid,) -> Rid {
            self.texture_get_rd_texture_ex(texture,) . done()
        }
        #[doc = "Returns a texture [`RID`][crate::builtin::Rid] that can be used with [`RenderingDevice`][crate::classes::RenderingDevice].\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding."]
        #[inline]
        pub fn texture_get_rd_texture_ex < 'ex > (&'ex self, texture: Rid,) -> ExTextureGetRdTexture < 'ex > {
            ExTextureGetRdTexture::new(self, texture,)
        }
        #[doc = "Returns the internal graphics handle for this texture object. For use when communicating with third-party APIs mostly with GDExtension.\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding.\n\n**Note:** This function returns a `uint64_t` which internally maps to a `GLuint` (OpenGL) or `VkImage` (Vulkan)."]
        pub(crate) fn texture_get_native_handle_full(&self, texture: Rid, srgb: bool,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid, bool,);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`texture_get_native_handle_ex`][Self::texture_get_native_handle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the internal graphics handle for this texture object. For use when communicating with third-party APIs mostly with GDExtension.\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding.\n\n**Note:** This function returns a `uint64_t` which internally maps to a `GLuint` (OpenGL) or `VkImage` (Vulkan)."]
        #[inline]
        pub fn texture_get_native_handle(&self, texture: Rid,) -> u64 {
            self.texture_get_native_handle_ex(texture,) . done()
        }
        #[doc = "Returns the internal graphics handle for this texture object. For use when communicating with third-party APIs mostly with GDExtension.\n\n`srgb` should be `true` when the texture uses nonlinear sRGB encoding and `false` when the texture uses linear encoding.\n\n**Note:** This function returns a `uint64_t` which internally maps to a `GLuint` (OpenGL) or `VkImage` (Vulkan)."]
        #[inline]
        pub fn texture_get_native_handle_ex < 'ex > (&'ex self, texture: Rid,) -> ExTextureGetNativeHandle < 'ex > {
            ExTextureGetNativeHandle::new(self, texture,)
        }
        #[doc = "Creates an empty shader and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `shader_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`Shader`][crate::classes::Shader]."]
        pub fn shader_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shader's source code (which triggers recompilation after being changed)."]
        pub fn shader_set_code(&mut self, shader: Rid, code: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shader, code.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the path hint for the specified shader. This should generally match the [`Shader`][crate::classes::Shader] resource's \\[member Resource.resource_path]."]
        pub fn shader_set_path_hint(&mut self, shader: Rid, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shader, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_path_hint", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a shader's source code as a string."]
        pub fn shader_get_code(&self, shader: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the parameters of a shader."]
        pub fn get_shader_parameter_list(&self, shader: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default value for the specified shader uniform. This is usually the value written in the shader source code."]
        pub fn shader_get_parameter_default(&self, shader: Rid, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (shader, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_parameter_default", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a shader's default texture. Overwrites the texture given by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        pub(crate) fn shader_set_default_texture_parameter_full(&mut self, shader: Rid, name: CowArg < StringName >, texture: Rid, index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >, Rid, i32,);
            let args = (shader, name, texture, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shader_set_default_texture_parameter_ex`][Self::shader_set_default_texture_parameter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a shader's default texture. Overwrites the texture given by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn shader_set_default_texture_parameter(&mut self, shader: Rid, name: impl AsArg < StringName >, texture: Rid,) {
            self.shader_set_default_texture_parameter_ex(shader, name, texture,) . done()
        }
        #[doc = "Sets a shader's default texture. Overwrites the texture given by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn shader_set_default_texture_parameter_ex < 'ex > (&'ex mut self, shader: Rid, name: impl AsArg < StringName > + 'ex, texture: Rid,) -> ExShaderSetDefaultTextureParameter < 'ex > {
            ExShaderSetDefaultTextureParameter::new(self, shader, name, texture,)
        }
        #[doc = "Returns a default texture from a shader searched by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        pub(crate) fn shader_get_default_texture_parameter_full(&self, shader: Rid, name: CowArg < StringName >, index: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >, i32,);
            let args = (shader, name, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`shader_get_default_texture_parameter_ex`][Self::shader_get_default_texture_parameter_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a default texture from a shader searched by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn shader_get_default_texture_parameter(&self, shader: Rid, name: impl AsArg < StringName >,) -> Rid {
            self.shader_get_default_texture_parameter_ex(shader, name,) . done()
        }
        #[doc = "Returns a default texture from a shader searched by name.\n\n**Note:** If the sampler array is used use `index` to access the specified texture."]
        #[inline]
        pub fn shader_get_default_texture_parameter_ex < 'ex > (&'ex self, shader: Rid, name: impl AsArg < StringName > + 'ex,) -> ExShaderGetDefaultTextureParameter < 'ex > {
            ExShaderGetDefaultTextureParameter::new(self, shader, name,)
        }
        #[doc = "Creates an empty material and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `material_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`Material`][crate::classes::Material]."]
        pub fn material_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a shader material's shader."]
        pub fn material_set_shader(&mut self, shader_material: Rid, shader: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (shader_material, shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_shader", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a material's parameter."]
        pub fn material_set_param(&mut self, material: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (material, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of a certain material's parameter."]
        pub fn material_get_param(&self, material: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (material, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_get_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a material's render priority."]
        pub fn material_set_render_priority(&mut self, material: Rid, priority: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (material, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an object's next material."]
        pub fn material_set_next_pass(&mut self, material: Rid, next_material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (material, next_material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_next_pass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "When using the Mobile renderer, [`material_set_use_debanding`][`crate::classes::RenderingServer::material_set_use_debanding`] can be used to enable or disable the debanding feature of 3D materials ([`BaseMaterial3D`][crate::classes::BaseMaterial3D] and [`ShaderMaterial`][crate::classes::ShaderMaterial]).\n\n[`material_set_use_debanding`][`crate::classes::RenderingServer::material_set_use_debanding`] has no effect when using the Compatibility or Forward+ renderer. In Forward+, [`Viewport`][crate::classes::Viewport] debanding can be used instead.\n\nSee also \\[member ProjectSettings.rendering/anti_aliasing/quality/use_debanding] and [`viewport_set_use_debanding`][`crate::classes::RenderingServer::viewport_set_use_debanding`]."]
        pub fn material_set_use_debanding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn mesh_create_from_surfaces_full(&mut self, surfaces: RefArg < Array < AnyDictionary > >, blend_shape_count: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < AnyDictionary > >, i32,);
            let args = (surfaces, blend_shape_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_create_from_surfaces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`mesh_create_from_surfaces_ex`][Self::mesh_create_from_surfaces_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn mesh_create_from_surfaces(&mut self, surfaces: &Array < AnyDictionary >,) -> Rid {
            self.mesh_create_from_surfaces_ex(surfaces,) . done()
        }
        #[inline]
        pub fn mesh_create_from_surfaces_ex < 'ex > (&'ex mut self, surfaces: &'ex Array < AnyDictionary >,) -> ExMeshCreateFromSurfaces < 'ex > {
            ExMeshCreateFromSurfaces::new(self, surfaces,)
        }
        #[doc = "Creates a new mesh and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `mesh_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this mesh to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent resource is [`Mesh`][crate::classes::Mesh]."]
        pub fn mesh_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the offset of a given attribute by `array_index` in the start of its respective buffer."]
        pub fn mesh_surface_get_format_offset(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32, array_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32, i32,);
            let args = (format, vertex_count, array_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stride of the vertex positions for a mesh with given `format`. Note importantly that vertex positions are stored consecutively and are not interleaved with the other attributes in the vertex buffer (normals and tangents)."]
        pub fn mesh_surface_get_format_vertex_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_vertex_stride", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stride of the combined normals and tangents for a mesh with given `format`. Note importantly that, while normals and tangents are in the vertex buffer with vertices, they are only interleaved with each other and so have a different stride than vertex positions."]
        pub fn mesh_surface_get_format_normal_tangent_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_normal_tangent_stride", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stride of the attribute buffer for a mesh with given `format`."]
        pub fn mesh_surface_get_format_attribute_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_attribute_stride", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stride of the skin buffer for a mesh with given `format`."]
        pub fn mesh_surface_get_format_skin_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_skin_stride", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stride of the index buffer for a mesh with the given `format`."]
        pub fn mesh_surface_get_format_index_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_index_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_add_surface(&mut self, mesh: Rid, surface: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, AnyDictionary >,);
            let args = (mesh, RefArg::new(surface),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_add_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn mesh_add_surface_from_arrays_full(&mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: RefArg < AnyArray >, blend_shapes: RefArg < AnyArray >, lods: RefArg < AnyDictionary >, compress_format: crate::classes::rendering_server::ArrayFormat,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, crate::classes::rendering_server::PrimitiveType, RefArg < 'a0, AnyArray >, RefArg < 'a1, AnyArray >, RefArg < 'a2, AnyDictionary >, crate::classes::rendering_server::ArrayFormat,);
            let args = (mesh, primitive, arrays, blend_shapes, lods, compress_format,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_add_surface_from_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`mesh_add_surface_from_arrays_ex`][Self::mesh_add_surface_from_arrays_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn mesh_add_surface_from_arrays(&mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &AnyArray,) {
            self.mesh_add_surface_from_arrays_ex(mesh, primitive, arrays,) . done()
        }
        #[inline]
        pub fn mesh_add_surface_from_arrays_ex < 'ex > (&'ex mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &'ex AnyArray,) -> ExMeshAddSurfaceFromArrays < 'ex > {
            ExMeshAddSurfaceFromArrays::new(self, mesh, primitive, arrays,)
        }
        #[doc = "Returns a mesh's blend shape count."]
        pub fn mesh_get_blend_shape_count(&self, mesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a mesh's blend shape mode."]
        pub fn mesh_set_blend_shape_mode(&mut self, mesh: Rid, mode: crate::classes::rendering_server::BlendShapeMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::BlendShapeMode,);
            let args = (mesh, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's blend shape mode."]
        pub fn mesh_get_blend_shape_mode(&self, mesh: Rid,) -> crate::classes::rendering_server::BlendShapeMode {
            type CallRet = crate::classes::rendering_server::BlendShapeMode;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a mesh's surface's material."]
        pub fn mesh_surface_set_material(&mut self, mesh: Rid, surface: i32, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (mesh, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's surface's material."]
        pub fn mesh_surface_get_material(&self, mesh: Rid, surface: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_get_surface(&mut self, mesh: Rid, surface: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_surface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's surface's buffer arrays."]
        pub fn mesh_surface_get_arrays(&self, mesh: Rid, surface: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's surface's arrays for blend shapes."]
        pub fn mesh_surface_get_blend_shape_arrays(&self, mesh: Rid, surface: i32,) -> Array < VarArray > {
            type CallRet = Array < VarArray >;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_blend_shape_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's number of surfaces."]
        pub fn mesh_get_surface_count(&self, mesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_surface_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a mesh's custom aabb."]
        pub fn mesh_set_custom_aabb(&mut self, mesh: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (mesh, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh's custom aabb."]
        pub fn mesh_get_custom_aabb(&self, mesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the surface at the given index from the Mesh, shifting surfaces with higher index down by one."]
        pub fn mesh_surface_remove(&mut self, mesh: Rid, surface: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_remove", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all surfaces from a mesh."]
        pub fn mesh_clear(&mut self, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_vertex_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_vertex_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_attribute_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_attribute_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_skin_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_skin_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the index buffer of the mesh surface with the given `data`. The expected data are 16 or 32-bit unsigned integers, which can be determined with [`mesh_surface_get_format_index_stride`][`crate::classes::RenderingServer::mesh_surface_get_format_index_stride`]."]
        pub fn mesh_surface_update_index_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_index_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_set_shadow_mesh(&mut self, mesh: Rid, shadow_mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (mesh, shadow_mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_shadow_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new multimesh on the RenderingServer and returns an [`RID`][crate::builtin::Rid] handle. This RID will be used in all `multimesh_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this multimesh to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent resource is [`MultiMesh`][crate::classes::MultiMesh]."]
        pub fn multimesh_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn multimesh_allocate_data_full(&mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool, use_indirect: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, crate::classes::rendering_server::MultimeshTransformFormat, bool, bool, bool,);
            let args = (multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`multimesh_allocate_data_ex`][Self::multimesh_allocate_data_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn multimesh_allocate_data(&mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) {
            self.multimesh_allocate_data_ex(multimesh, instances, transform_format,) . done()
        }
        #[inline]
        pub fn multimesh_allocate_data_ex < 'ex > (&'ex mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) -> ExMultimeshAllocateData < 'ex > {
            ExMultimeshAllocateData::new(self, multimesh, instances, transform_format,)
        }
        #[doc = "Returns the number of instances allocated for this multimesh."]
        pub fn multimesh_get_instance_count(&self, multimesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh to be drawn by the multimesh. Equivalent to \\[member MultiMesh.mesh]."]
        pub fn multimesh_set_mesh(&mut self, multimesh: Rid, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (multimesh, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform3D`][crate::builtin::Transform3D] for this instance. Equivalent to [`set_instance_transform`][`crate::classes::MultiMesh::set_instance_transform`]."]
        pub fn multimesh_instance_set_transform(&mut self, multimesh: Rid, index: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform2D`][crate::builtin::Transform2D] for this instance. For use when multimesh is used in 2D. Equivalent to [`set_instance_transform_2d`][`crate::classes::MultiMesh::set_instance_transform_2d`]."]
        pub fn multimesh_instance_set_transform_2d(&mut self, multimesh: Rid, index: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color by which this instance will be modulated. Equivalent to [`set_instance_color`][`crate::classes::MultiMesh::set_instance_color`]."]
        pub fn multimesh_instance_set_color(&mut self, multimesh: Rid, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Color,);
            let args = (multimesh, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the custom data for this instance. Custom data is passed as a [`Color`][crate::builtin::Color], but is interpreted as a `vec4` in the shader. Equivalent to [`set_instance_custom_data`][`crate::classes::MultiMesh::set_instance_custom_data`]."]
        pub fn multimesh_instance_set_custom_data(&mut self, multimesh: Rid, index: i32, custom_data: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Color,);
            let args = (multimesh, index, custom_data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the RID of the mesh that will be used in drawing this multimesh."]
        pub fn multimesh_get_mesh(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calculates and returns the axis-aligned bounding box that encloses all instances within the multimesh."]
        pub fn multimesh_get_aabb(&self, multimesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the custom AABB for this MultiMesh resource."]
        pub fn multimesh_set_custom_aabb(&mut self, multimesh: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (multimesh, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom AABB defined for this MultiMesh resource."]
        pub fn multimesh_get_custom_aabb(&self, multimesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform3D`][crate::builtin::Transform3D] of the specified instance."]
        pub fn multimesh_instance_get_transform(&self, multimesh: Rid, index: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform2D`][crate::builtin::Transform2D] of the specified instance. For use when the multimesh is set to use 2D transforms."]
        pub fn multimesh_instance_get_transform_2d(&self, multimesh: Rid, index: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the color by which the specified instance will be modulated."]
        pub fn multimesh_instance_get_color(&self, multimesh: Rid, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the custom data associated with the specified instance."]
        pub fn multimesh_instance_get_custom_data(&self, multimesh: Rid, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of instances visible at a given time. If -1, all instances that have been allocated are drawn. Equivalent to \\[member MultiMesh.visible_instance_count]."]
        pub fn multimesh_set_visible_instances(&mut self, multimesh: Rid, visible: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (multimesh, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_visible_instances", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of visible instances for this multimesh."]
        pub fn multimesh_get_visible_instances(&self, multimesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_visible_instances", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the entire data to use for drawing the `multimesh` at once to `buffer` (such as instance transforms and colors). `buffer`'s size must match the number of instances multiplied by the per-instance data size (which depends on the enabled MultiMesh fields). Otherwise, an error message is printed and nothing is rendered. See also [`multimesh_get_buffer`][`crate::classes::RenderingServer::multimesh_get_buffer`].\n\nThe per-instance data size and expected data order is:\n\n```text\n2D:\n  - Position: 8 floats (8 floats for Transform2D)\n  - Position + Vertex color: 12 floats (8 floats for Transform2D, 4 floats for Color)\n  - Position + Custom data: 12 floats (8 floats for Transform2D, 4 floats of custom data)\n  - Position + Vertex color + Custom data: 16 floats (8 floats for Transform2D, 4 floats for Color, 4 floats of custom data)\n3D:\n  - Position: 12 floats (12 floats for Transform3D)\n  - Position + Vertex color: 16 floats (12 floats for Transform3D, 4 floats for Color)\n  - Position + Custom data: 16 floats (12 floats for Transform3D, 4 floats of custom data)\n  - Position + Vertex color + Custom data: 20 floats (12 floats for Transform3D, 4 floats for Color, 4 floats of custom data)\n```\n\nInstance transforms are in row-major order. Specifically:\n\n- For [`Transform2D`][crate::builtin::Transform2D] the float-order is: `(x.x, y.x, padding_float, origin.x, x.y, y.y, padding_float, origin.y)`.\n\n- For [`Transform3D`][crate::builtin::Transform3D] the float-order is: `(basis.x.x, basis.y.x, basis.z.x, origin.x, basis.x.y, basis.y.y, basis.z.y, origin.y, basis.x.z, basis.y.z, basis.z.z, origin.z)`."]
        pub fn multimesh_set_buffer(&mut self, multimesh: Rid, buffer: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >,);
            let args = (multimesh, RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RenderingDevice`][crate::classes::RenderingDevice] [`RID`][crate::builtin::Rid] handle of the [`MultiMesh`][crate::classes::MultiMesh] command buffer. This [`RID`][crate::builtin::Rid] is only valid if `use_indirect` is set to `true` when allocating data through [`multimesh_allocate_data`][`crate::classes::RenderingServer::multimesh_allocate_data`]. It can be used to directly modify the instance count via buffer.\n\nThe data structure is dependent on both how many surfaces the mesh contains and whether it is indexed or not, the buffer has 5 integers in it, with the last unused if the mesh is not indexed.\n\nEach of the values in the buffer correspond to these options:\n\n```text\nIndexed:\n  0 - indexCount;\n  1 - instanceCount;\n  2 - firstIndex;\n  3 - vertexOffset;\n  4 - firstInstance;\nNon Indexed:\n  0 - vertexCount;\n  1 - instanceCount;\n  2 - firstVertex;\n  3 - firstInstance;\n  4 - unused;\n```"]
        pub fn multimesh_get_command_buffer_rd_rid(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_command_buffer_rd_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`RenderingDevice`][crate::classes::RenderingDevice] [`RID`][crate::builtin::Rid] handle of the [`MultiMesh`][crate::classes::MultiMesh], which can be used as any other buffer on the Rendering Device."]
        pub fn multimesh_get_buffer_rd_rid(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_buffer_rd_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the MultiMesh data (such as instance transforms, colors, etc.). See [`multimesh_set_buffer`][`crate::classes::RenderingServer::multimesh_set_buffer`] for details on the returned data.\n\n**Note:** If the buffer is in the engine's internal cache, it will have to be fetched from GPU memory and possibly decompressed. This means [`multimesh_get_buffer`][`crate::classes::RenderingServer::multimesh_get_buffer`] is potentially a slow operation and should be avoided whenever possible."]
        pub fn multimesh_get_buffer(&self, multimesh: Rid,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Alternative version of [`multimesh_set_buffer`][`crate::classes::RenderingServer::multimesh_set_buffer`] for use with physics interpolation.\n\nTakes both an array of current data and an array of data for the previous physics tick."]
        pub fn multimesh_set_buffer_interpolated(&mut self, multimesh: Rid, buffer: &PackedFloat32Array, buffer_previous: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedFloat32Array >, RefArg < 'a1, PackedFloat32Array >,);
            let args = (multimesh, RefArg::new(buffer), RefArg::new(buffer_previous),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_buffer_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Turns on and off physics interpolation for this MultiMesh resource."]
        pub fn multimesh_set_physics_interpolated(&mut self, multimesh: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (multimesh, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_physics_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the physics interpolation quality for the [`MultiMesh`][crate::classes::MultiMesh].\n\nA value of [`MultimeshPhysicsInterpolationQuality::FAST`][`crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality::FAST`] gives fast but low quality interpolation, a value of [`MultimeshPhysicsInterpolationQuality::HIGH`][`crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality::HIGH`] gives slower but higher quality interpolation."]
        pub fn multimesh_set_physics_interpolation_quality(&mut self, multimesh: Rid, quality: crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality,);
            let args = (multimesh, quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_physics_interpolation_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents physics interpolation for the specified instance during the current physics tick.\n\nThis is useful when moving an instance to a new location, to give an instantaneous change rather than interpolation from the previous location."]
        pub fn multimesh_instance_reset_physics_interpolation(&mut self, multimesh: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents physics interpolation for all instances during the current physics tick.\n\nThis is useful when moving all instances to new locations, to give instantaneous changes rather than interpolation from the previous locations."]
        pub fn multimesh_instances_reset_physics_interpolation(&mut self, multimesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instances_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a skeleton and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `skeleton_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method."]
        pub fn skeleton_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn skeleton_allocate_data_full(&mut self, skeleton: Rid, bones: i32, is_2d_skeleton: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (skeleton, bones, is_2d_skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`skeleton_allocate_data_ex`][Self::skeleton_allocate_data_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn skeleton_allocate_data(&mut self, skeleton: Rid, bones: i32,) {
            self.skeleton_allocate_data_ex(skeleton, bones,) . done()
        }
        #[inline]
        pub fn skeleton_allocate_data_ex < 'ex > (&'ex mut self, skeleton: Rid, bones: i32,) -> ExSkeletonAllocateData < 'ex > {
            ExSkeletonAllocateData::new(self, skeleton, bones,)
        }
        #[doc = "Returns the number of bones allocated for this skeleton."]
        pub fn skeleton_get_bone_count(&self, skeleton: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_get_bone_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform3D`][crate::builtin::Transform3D] for a specific bone of this skeleton."]
        pub fn skeleton_bone_set_transform(&mut self, skeleton: Rid, bone: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform3D`][crate::builtin::Transform3D] set for a specific bone of this skeleton."]
        pub fn skeleton_bone_get_transform(&self, skeleton: Rid, bone: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform2D`][crate::builtin::Transform2D] for a specific bone of this skeleton."]
        pub fn skeleton_bone_set_transform_2d(&mut self, skeleton: Rid, bone: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_set_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`Transform2D`][crate::builtin::Transform2D] set for a specific bone of this skeleton."]
        pub fn skeleton_bone_get_transform_2d(&self, skeleton: Rid, bone: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_get_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_set_base_transform_2d(&mut self, skeleton: Rid, base_transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (skeleton, base_transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_set_base_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a directional light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most `light_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this directional light to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent node is [`DirectionalLight3D`][crate::classes::DirectionalLight3D]."]
        pub fn directional_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_light_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new omni light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most `light_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this omni light to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent node is [`OmniLight3D`][crate::classes::OmniLight3D]."]
        pub fn omni_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "omni_light_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a spot light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most `light_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this spot light to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID."]
        pub fn spot_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "spot_light_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of the light. Equivalent to \\[member Light3D.light_color]."]
        pub fn light_set_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the specified 3D light parameter. Equivalent to [`set_param`][`crate::classes::Light3D::set_param`]."]
        pub fn light_set_param(&mut self, light: Rid, param: crate::classes::rendering_server::LightParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightParam, f32,);
            let args = (light, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_param", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, light will cast shadows. Equivalent to \\[member Light3D.shadow_enabled]."]
        pub fn light_set_shadow(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the projector texture to use for the specified 3D light. Equivalent to \\[member Light3D.light_projector]."]
        pub fn light_set_projector(&mut self, light: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_projector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the 3D light will subtract light instead of adding light. Equivalent to \\[member Light3D.light_negative]."]
        pub fn light_set_negative(&mut self, light: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_negative", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the cull mask for this 3D light. Lights only affect objects in the selected layers. Equivalent to \\[member Light3D.light_cull_mask]."]
        pub fn light_set_cull_mask(&mut self, light: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the distance fade for this 3D light. This acts as a form of level of detail (LOD) and can be used to improve performance. Equivalent to \\[member Light3D.distance_fade_enabled], \\[member Light3D.distance_fade_begin], \\[member Light3D.distance_fade_shadow], and \\[member Light3D.distance_fade_length]."]
        pub fn light_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, shadow: f32, length: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32,);
            let args = (decal, enabled, begin, shadow, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, reverses the backface culling of the mesh. This can be useful when you have a flat mesh that has a light behind it. If you need to cast a shadow on both sides of the mesh, set the mesh to use double-sided shadows with [`instance_geometry_set_cast_shadows_setting`][`crate::classes::RenderingServer::instance_geometry_set_cast_shadows_setting`]. Equivalent to \\[member Light3D.shadow_reverse_cull_face]."]
        pub fn light_set_reverse_cull_face_mode(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_reverse_cull_face_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shadow caster mask for this 3D light. Shadows will only be cast using objects in the selected layers. Equivalent to \\[member Light3D.shadow_caster_mask]."]
        pub fn light_set_shadow_caster_mask(&mut self, light: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_shadow_caster_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the bake mode to use for the specified 3D light. Equivalent to \\[member Light3D.light_bake_mode]."]
        pub fn light_set_bake_mode(&mut self, light: Rid, bake_mode: crate::classes::rendering_server::LightBakeMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightBakeMode,);
            let args = (light, bake_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_bake_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum SDFGI cascade in which the 3D light's indirect lighting is rendered. Higher values allow the light to be rendered in SDFGI further away from the camera."]
        pub fn light_set_max_sdfgi_cascade(&mut self, light: Rid, cascade: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, cascade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_max_sdfgi_cascade", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether to use a dual paraboloid or a cubemap for the shadow map. Dual paraboloid is faster but may suffer from artifacts. Equivalent to \\[member OmniLight3D.omni_shadow_mode]."]
        pub fn light_omni_set_shadow_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightOmniShadowMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightOmniShadowMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_omni_set_shadow_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shadow mode for this directional light. Equivalent to \\[member DirectionalLight3D.directional_shadow_mode]."]
        pub fn light_directional_set_shadow_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightDirectionalShadowMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightDirectionalShadowMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_shadow_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, this directional light will blend between shadow map splits resulting in a smoother transition between them. Equivalent to \\[member DirectionalLight3D.directional_shadow_blend_splits]."]
        pub fn light_directional_set_blend_splits(&mut self, light: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_blend_splits", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, this light will not be used for anything except sky shaders. Use this for lights that impact your sky shader that you may want to hide from affecting the rest of the scene. For example, you may want to enable this when the sun in your sky shader falls below the horizon."]
        pub fn light_directional_set_sky_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightDirectionalSkyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightDirectionalSkyMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_sky_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the texture filter mode to use when rendering light projectors. This parameter is global and cannot be set on a per-light basis."]
        pub fn light_projectors_set_filter(&mut self, filter: crate::classes::rendering_server::LightProjectorFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::LightProjectorFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_projectors_set_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Toggles whether a bicubic filter should be used when lightmaps are sampled. This smoothens their appearance at a performance cost."]
        pub fn lightmaps_set_bicubic_filter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmaps_set_bicubic_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the filter quality for omni and spot light shadows in 3D. See also \\[member ProjectSettings.rendering/lights_and_shadows/positional_shadow/soft_shadow_filter_quality]. This parameter is global and cannot be set on a per-viewport basis."]
        pub fn positional_soft_shadow_filter_set_quality(&mut self, quality: crate::classes::rendering_server::ShadowQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ShadowQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "positional_soft_shadow_filter_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the filter `quality` for directional light shadows in 3D. See also \\[member ProjectSettings.rendering/lights_and_shadows/directional_shadow/soft_shadow_filter_quality]. This parameter is global and cannot be set on a per-viewport basis."]
        pub fn directional_soft_shadow_filter_set_quality(&mut self, quality: crate::classes::rendering_server::ShadowQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ShadowQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_soft_shadow_filter_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `size` of the directional light shadows in 3D. See also \\[member ProjectSettings.rendering/lights_and_shadows/directional_shadow/size]. This parameter is global and cannot be set on a per-viewport basis."]
        pub fn directional_shadow_atlas_set_size(&mut self, size: i32, is_16bits: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (size, is_16bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_shadow_atlas_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a reflection probe and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `reflection_probe_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this reflection probe to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent node is [`ReflectionProbe`][crate::classes::ReflectionProbe]."]
        pub fn reflection_probe_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets how often the reflection probe updates. Can either be once or every frame."]
        pub fn reflection_probe_set_update_mode(&mut self, probe: Rid, mode: crate::classes::rendering_server::ReflectionProbeUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ReflectionProbeUpdateMode,);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the intensity of the reflection probe. Intensity modulates the strength of the reflection. Equivalent to \\[member ReflectionProbe.intensity]."]
        pub fn reflection_probe_set_intensity(&mut self, probe: Rid, intensity: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, intensity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_intensity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the distance in meters over which a probe blends into the scene."]
        pub fn reflection_probe_set_blend_distance(&mut self, probe: Rid, blend_distance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, blend_distance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the reflection probe's ambient light mode. Equivalent to \\[member ReflectionProbe.ambient_mode]."]
        pub fn reflection_probe_set_ambient_mode(&mut self, probe: Rid, mode: crate::classes::rendering_server::ReflectionProbeAmbientMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ReflectionProbeAmbientMode,);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the reflection probe's custom ambient light color. Equivalent to \\[member ReflectionProbe.ambient_color]."]
        pub fn reflection_probe_set_ambient_color(&mut self, probe: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (probe, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the reflection probe's custom ambient light energy. Equivalent to \\[member ReflectionProbe.ambient_color_energy]."]
        pub fn reflection_probe_set_ambient_energy(&mut self, probe: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_energy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the max distance away from the probe an object can be before it is culled. Equivalent to \\[member ReflectionProbe.max_distance]."]
        pub fn reflection_probe_set_max_distance(&mut self, probe: Rid, distance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, distance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the size of the area that the reflection probe will capture. Equivalent to \\[member ReflectionProbe.size]."]
        pub fn reflection_probe_set_size(&mut self, probe: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (probe, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the origin offset to be used when this reflection probe is in box project mode. Equivalent to \\[member ReflectionProbe.origin_offset]."]
        pub fn reflection_probe_set_origin_offset(&mut self, probe: Rid, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (probe, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, reflections will ignore sky contribution. Equivalent to \\[member ReflectionProbe.interior]."]
        pub fn reflection_probe_set_as_interior(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, uses box projection. This can make reflections look more correct in certain situations. Equivalent to \\[member ReflectionProbe.box_projection]."]
        pub fn reflection_probe_set_enable_box_projection(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_enable_box_projection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, computes shadows in the reflection probe. This makes the reflection much slower to compute. Equivalent to \\[member ReflectionProbe.enable_shadows]."]
        pub fn reflection_probe_set_enable_shadows(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_enable_shadows", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render cull mask for this reflection probe. Only instances with a matching layer will be reflected by this probe. Equivalent to \\[member ReflectionProbe.cull_mask]."]
        pub fn reflection_probe_set_cull_mask(&mut self, probe: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (probe, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render reflection mask for this reflection probe. Only instances with a matching layer will have reflections applied from this probe. Equivalent to \\[member ReflectionProbe.reflection_mask]."]
        pub fn reflection_probe_set_reflection_mask(&mut self, probe: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (probe, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Deprecated. This method does nothing."]
        pub fn reflection_probe_set_resolution(&mut self, probe: Rid, resolution: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (probe, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_resolution", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh level of detail to use in the reflection probe rendering. Higher values will use less detailed versions of meshes that have LOD variations generated, which can improve performance. Equivalent to \\[member ReflectionProbe.mesh_lod_threshold]."]
        pub fn reflection_probe_set_mesh_lod_threshold(&mut self, probe: Rid, pixels: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, pixels,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a decal and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `decal_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this decal to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent node is [`Decal`][crate::classes::Decal]."]
        pub fn decal_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `size` of the decal specified by the `decal` RID. Equivalent to \\[member Decal.size]."]
        pub fn decal_set_size(&mut self, decal: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (decal, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `texture` in the given texture `type` slot for the specified decal. Equivalent to [`set_texture`][`crate::classes::Decal::set_texture`]."]
        pub fn decal_set_texture(&mut self, decal: Rid, type_: crate::classes::rendering_server::DecalTexture, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::DecalTexture, Rid,);
            let args = (decal, type_, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the emission `energy` in the decal specified by the `decal` RID. Equivalent to \\[member Decal.emission_energy]."]
        pub fn decal_set_emission_energy(&mut self, decal: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `albedo_mix` in the decal specified by the `decal` RID. Equivalent to \\[member Decal.albedo_mix]."]
        pub fn decal_set_albedo_mix(&mut self, decal: Rid, albedo_mix: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, albedo_mix,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_albedo_mix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color multiplier in the decal specified by the `decal` RID to `color`. Equivalent to \\[member Decal.modulate]."]
        pub fn decal_set_modulate(&mut self, decal: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (decal, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the cull `mask` in the decal specified by the `decal` RID. Equivalent to \\[member Decal.cull_mask]."]
        pub fn decal_set_cull_mask(&mut self, decal: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (decal, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the distance fade parameters in the decal specified by the `decal` RID. Equivalent to \\[member Decal.distance_fade_enabled], \\[member Decal.distance_fade_begin] and \\[member Decal.distance_fade_length]."]
        pub fn decal_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, length: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32,);
            let args = (decal, enabled, begin, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the upper fade (`above`) and lower fade (`below`) in the decal specified by the `decal` RID. Equivalent to \\[member Decal.upper_fade] and \\[member Decal.lower_fade]."]
        pub fn decal_set_fade(&mut self, decal: Rid, above: f32, below: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (decal, above, below,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_fade", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the normal `fade` in the decal specified by the `decal` RID. Equivalent to \\[member Decal.normal_fade]."]
        pub fn decal_set_normal_fade(&mut self, decal: Rid, fade: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, fade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_normal_fade", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the texture `filter` mode to use when rendering decals. This parameter is global and cannot be set on a per-decal basis."]
        pub fn decals_set_filter(&mut self, filter: crate::classes::rendering_server::DecalFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DecalFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decals_set_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `half_resolution` is `true`, renders [`VoxelGI`][crate::classes::VoxelGi] and SDFGI (\\[member Environment.sdfgi_enabled]) buffers at halved resolution on each axis (e.g. 960×540 when the viewport size is 1920×1080). This improves performance significantly when VoxelGI or SDFGI is enabled, at the cost of artifacts that may be visible on polygon edges. The loss in quality becomes less noticeable as the viewport resolution increases. [`LightmapGI`][crate::classes::LightmapGi] rendering is not affected by this setting. Equivalent to \\[member ProjectSettings.rendering/global_illumination/gi/use_half_resolution]."]
        pub fn gi_set_use_half_resolution(&mut self, half_resolution: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (half_resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "gi_set_use_half_resolution", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new voxel-based global illumination object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `voxel_gi_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`VoxelGI`][crate::classes::VoxelGi]."]
        pub fn voxel_gi_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_allocate_data(&mut self, voxel_gi: Rid, to_cell_xform: Transform3D, aabb: Aabb, octree_size: Vector3i, octree_cells: &PackedByteArray, data_cells: &PackedByteArray, distance_field: &PackedByteArray, level_counts: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, Transform3D, Aabb, Vector3i, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >, RefArg < 'a3, PackedInt32Array >,);
            let args = (voxel_gi, to_cell_xform, aabb, octree_size, RefArg::new(octree_cells), RefArg::new(data_cells), RefArg::new(distance_field), RefArg::new(level_counts),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_octree_size(&self, voxel_gi: Rid,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_octree_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_octree_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_octree_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_data_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_data_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_distance_field(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_level_counts(&self, voxel_gi: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_level_counts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_to_cell_xform(&self, voxel_gi: Rid,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_to_cell_xform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.dynamic_range] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_dynamic_range(&mut self, voxel_gi: Rid, range: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, range,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_dynamic_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.propagation] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_propagation(&mut self, voxel_gi: Rid, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_propagation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.energy] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_energy(&mut self, voxel_gi: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_energy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Used to inform the renderer what exposure normalization value was used while baking the voxel gi. This value will be used and modulated at run time to ensure that the voxel gi maintains a consistent level of exposure even if the scene-wide exposure normalization is changed at run time. For more information see [`camera_attributes_set_exposure`][`crate::classes::RenderingServer::camera_attributes_set_exposure`]."]
        pub fn voxel_gi_set_baked_exposure_normalization(&mut self, voxel_gi: Rid, baked_exposure: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_baked_exposure_normalization", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.bias] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_bias", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.normal_bias] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_normal_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.interior] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_interior(&mut self, voxel_gi: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_interior", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member VoxelGIData.use_two_bounces] value to use on the specified `voxel_gi`'s [`RID`][crate::builtin::Rid]."]
        pub fn voxel_gi_set_use_two_bounces(&mut self, voxel_gi: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_use_two_bounces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member ProjectSettings.rendering/global_illumination/voxel_gi/quality] value to use when rendering. This parameter is global and cannot be set on a per-VoxelGI basis."]
        pub fn voxel_gi_set_quality(&mut self, quality: crate::classes::rendering_server::VoxelGiQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::VoxelGiQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new lightmap global illumination instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `lightmap_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`LightmapGI`][crate::classes::LightmapGi]."]
        pub fn lightmap_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the textures on the given `lightmap` GI instance to the texture array pointed to by the `light` RID. If the lightmap texture was baked with \\[member LightmapGI.directional] set to `true`, then `uses_sh` must also be `true`."]
        pub fn lightmap_set_textures(&mut self, lightmap: Rid, light: Rid, uses_sh: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, bool,);
            let args = (lightmap, light, uses_sh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_bounds(&mut self, lightmap: Rid, bounds: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (lightmap, bounds,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_bounds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_interior(&mut self, lightmap: Rid, interior: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (lightmap, interior,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_capture_data(&mut self, lightmap: Rid, points: &PackedVector3Array, point_sh: &PackedColorArray, tetrahedra: &PackedInt32Array, bsp_tree: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedInt32Array >, RefArg < 'a3, PackedInt32Array >,);
            let args = (lightmap, RefArg::new(points), RefArg::new(point_sh), RefArg::new(tetrahedra), RefArg::new(bsp_tree),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_capture_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_points(&self, lightmap: Rid,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_sh(&self, lightmap: Rid,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_sh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_tetrahedra(&self, lightmap: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_tetrahedra", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_bsp_tree(&self, lightmap: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_bsp_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Used to inform the renderer what exposure normalization value was used while baking the lightmap. This value will be used and modulated at run time to ensure that the lightmap maintains a consistent level of exposure even if the scene-wide exposure normalization is changed at run time. For more information see [`camera_attributes_set_exposure`][`crate::classes::RenderingServer::camera_attributes_set_exposure`]."]
        pub fn lightmap_set_baked_exposure_normalization(&mut self, lightmap: Rid, baked_exposure: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (lightmap, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_baked_exposure_normalization", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_capture_update_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_capture_update_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a GPU-based particle system and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `particles_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach these particles to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent nodes are [`GPUParticles2D`][crate::classes::GpuParticles2D] and [`GPUParticles3D`][crate::classes::GpuParticles3D].\n\n**Note:** All `particles_*` methods only apply to GPU-based particles, not CPU-based particles. [`CPUParticles2D`][crate::classes::CpuParticles2D] and [`CPUParticles3D`][crate::classes::CpuParticles3D] do not have equivalent RenderingServer functions available, as these use [`MultiMeshInstance2D`][crate::classes::MultiMeshInstance2D] and [`MultiMeshInstance3D`][crate::classes::MultiMeshInstance3D] under the hood (see `multimesh_*` methods)."]
        pub fn particles_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether the GPU particles specified by the `particles` RID should be rendered in 2D or 3D according to `mode`."]
        pub fn particles_set_mode(&mut self, particles: Rid, mode: crate::classes::rendering_server::ParticlesMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesMode,);
            let args = (particles, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, particles will emit over time. Setting to `false` does not reset the particles, but only stops their emission. Equivalent to \\[member GPUParticles3D.emitting]."]
        pub fn particles_set_emitting(&mut self, particles: Rid, emitting: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, emitting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emitting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if particles are currently set to emitting."]
        pub fn particles_get_emitting(&mut self, particles: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_get_emitting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of particles to be drawn and allocates the memory for them. Equivalent to \\[member GPUParticles3D.amount]."]
        pub fn particles_set_amount(&mut self, particles: Rid, amount: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_amount", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the amount ratio for particles to be emitted. Equivalent to \\[member GPUParticles3D.amount_ratio]."]
        pub fn particles_set_amount_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_amount_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the lifetime of each particle in the system. Equivalent to \\[member GPUParticles3D.lifetime]."]
        pub fn particles_set_lifetime(&mut self, particles: Rid, lifetime: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, lifetime,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, particles will emit once and then stop. Equivalent to \\[member GPUParticles3D.one_shot]."]
        pub fn particles_set_one_shot(&mut self, particles: Rid, one_shot: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, one_shot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the preprocess time for the particles' animation. This lets you delay starting an animation until after the particles have begun emitting. Equivalent to \\[member GPUParticles3D.preprocess]."]
        pub fn particles_set_pre_process_time(&mut self, particles: Rid, time: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, time,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests particles to process for extra process time during a single frame."]
        pub fn particles_request_process_time(&mut self, particles: Rid, time: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, time,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_request_process_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the explosiveness ratio. Equivalent to \\[member GPUParticles3D.explosiveness]."]
        pub fn particles_set_explosiveness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the emission randomness ratio. This randomizes the emission of particles within their phase. Equivalent to \\[member GPUParticles3D.randomness]."]
        pub fn particles_set_randomness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the value that informs a [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial] to rush all particles towards the end of their lifetime."]
        pub fn particles_set_interp_to_end(&mut self, particles: Rid, factor: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, factor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_interp_to_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the velocity of a particle node, that will be used by \\[member ParticleProcessMaterial.inherit_velocity_ratio]."]
        pub fn particles_set_emitter_velocity(&mut self, particles: Rid, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (particles, velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emitter_velocity", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom axis-aligned bounding box for the particle system. Equivalent to \\[member GPUParticles3D.visibility_aabb]."]
        pub fn particles_set_custom_aabb(&mut self, particles: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (particles, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the speed scale of the particle system. Equivalent to \\[member GPUParticles3D.speed_scale]."]
        pub fn particles_set_speed_scale(&mut self, particles: Rid, scale: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, particles use local coordinates. If `false` they use global coordinates. Equivalent to \\[member GPUParticles3D.local_coords]."]
        pub fn particles_set_use_local_coordinates(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the material for processing the particles.\n\n**Note:** This is not the material used to draw the materials. Equivalent to \\[member GPUParticles3D.process_material]."]
        pub fn particles_set_process_material(&mut self, particles: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_process_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the frame rate that the particle system rendering will be fixed to. Equivalent to \\[member GPUParticles3D.fixed_fps]."]
        pub fn particles_set_fixed_fps(&mut self, particles: Rid, fps: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, fps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_interpolate(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, uses fractional delta which smooths the movement of the particles. Equivalent to \\[member GPUParticles3D.fract_delta]."]
        pub fn particles_set_fractional_delta(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_collision_base_size(&mut self, particles: Rid, size: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_collision_base_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_transform_align(&mut self, particles: Rid, align: crate::classes::rendering_server::ParticlesTransformAlign,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesTransformAlign,);
            let args = (particles, align,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_transform_align", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enable` is `true`, enables trails for the `particles` with the specified `length_sec` in seconds. Equivalent to \\[member GPUParticles3D.trail_enabled] and \\[member GPUParticles3D.trail_lifetime]."]
        pub fn particles_set_trails(&mut self, particles: Rid, enable: bool, length_sec: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32,);
            let args = (particles, enable, length_sec,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_trails", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_trail_bind_poses(&mut self, particles: Rid, bind_poses: &Array < Transform3D >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Transform3D > >,);
            let args = (particles, RefArg::new(bind_poses),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_trail_bind_poses", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if particles are not emitting and particles are set to inactive."]
        pub fn particles_is_inactive(&mut self, particles: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_is_inactive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Add particle system to list of particle systems that need to be updated. Update will take place on the next frame, or on the next call to [`instances_cull_aabb`][`crate::classes::RenderingServer::instances_cull_aabb`], [`instances_cull_convex`][`crate::classes::RenderingServer::instances_cull_convex`], or [`instances_cull_ray`][`crate::classes::RenderingServer::instances_cull_ray`]."]
        pub fn particles_request_process(&mut self, particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_request_process", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Reset the particles on the next update. Equivalent to [`restart`][`crate::classes::GpuParticles3D::restart`]."]
        pub fn particles_restart(&mut self, particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_restart", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_subemitter(&mut self, particles: Rid, subemitter_particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles, subemitter_particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_subemitter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Manually emits particles from the `particles` instance."]
        pub fn particles_emit(&mut self, particles: Rid, transform: Transform3D, velocity: Vector3, color: Color, custom: Color, emit_flags: u32,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D, Vector3, Color, Color, u32,);
            let args = (particles, transform, velocity, color, custom, emit_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_emit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the draw order of the particles. Equivalent to \\[member GPUParticles3D.draw_order]."]
        pub fn particles_set_draw_order(&mut self, particles: Rid, order: crate::classes::rendering_server::ParticlesDrawOrder,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesDrawOrder,);
            let args = (particles, order,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of draw passes to use. Equivalent to \\[member GPUParticles3D.draw_passes]."]
        pub fn particles_set_draw_passes(&mut self, particles: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_passes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh to be used for the specified draw pass. Equivalent to \\[member GPUParticles3D.draw_pass_1], \\[member GPUParticles3D.draw_pass_2], \\[member GPUParticles3D.draw_pass_3], and \\[member GPUParticles3D.draw_pass_4]."]
        pub fn particles_set_draw_pass_mesh(&mut self, particles: Rid, pass: i32, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (particles, pass, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_pass_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calculates and returns the axis-aligned bounding box that contains all the particles. Equivalent to [`capture_aabb`][`crate::classes::GpuParticles3D::capture_aabb`]."]
        pub fn particles_get_current_aabb(&mut self, particles: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_get_current_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Transform3D`][crate::builtin::Transform3D] that will be used by the particles when they first emit."]
        pub fn particles_set_emission_transform(&mut self, particles: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (particles, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emission_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new 3D GPU particle collision or attractor and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most `particles_collision_*` RenderingServer functions.\n\n**Note:** The equivalent nodes are [`GPUParticlesCollision3D`][crate::classes::GpuParticlesCollision3D] and [`GPUParticlesAttractor3D`][crate::classes::GpuParticlesAttractor3D]."]
        pub fn particles_collision_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the collision or attractor shape `type` for the 3D GPU particles collision or attractor specified by the `particles_collision` RID."]
        pub fn particles_collision_set_collision_type(&mut self, particles_collision: Rid, type_: crate::classes::rendering_server::ParticlesCollisionType,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesCollisionType,);
            let args = (particles_collision, type_,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_collision_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the cull `mask` for the 3D GPU particles collision or attractor specified by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollision3D.cull_mask] or \\[member GPUParticlesAttractor3D.cull_mask] depending on the `particles_collision` type."]
        pub fn particles_collision_set_cull_mask(&mut self, particles_collision: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (particles_collision, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `radius` for the 3D GPU particles sphere collision or attractor specified by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollisionSphere3D.radius] or \\[member GPUParticlesAttractorSphere3D.radius] depending on the `particles_collision` type."]
        pub fn particles_collision_set_sphere_radius(&mut self, particles_collision: Rid, radius: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `extents` for the 3D GPU particles collision by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollisionBox3D.size], \\[member GPUParticlesCollisionSDF3D.size], \\[member GPUParticlesCollisionHeightField3D.size], \\[member GPUParticlesAttractorBox3D.size] or \\[member GPUParticlesAttractorVectorField3D.size] depending on the `particles_collision` type."]
        pub fn particles_collision_set_box_extents(&mut self, particles_collision: Rid, extents: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (particles_collision, extents,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_box_extents", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `strength` for the 3D GPU particles attractor specified by the `particles_collision` RID. Only used for attractors, not colliders. Equivalent to \\[member GPUParticlesAttractor3D.strength]."]
        pub fn particles_collision_set_attractor_strength(&mut self, particles_collision: Rid, strength: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, strength,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the directionality `amount` for the 3D GPU particles attractor specified by the `particles_collision` RID. Only used for attractors, not colliders. Equivalent to \\[member GPUParticlesAttractor3D.directionality]."]
        pub fn particles_collision_set_attractor_directionality(&mut self, particles_collision: Rid, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_directionality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the attenuation `curve` for the 3D GPU particles attractor specified by the `particles_collision` RID. Only used for attractors, not colliders. Equivalent to \\[member GPUParticlesAttractor3D.attenuation]."]
        pub fn particles_collision_set_attractor_attenuation(&mut self, particles_collision: Rid, curve: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, curve,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_attenuation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the signed distance field `texture` for the 3D GPU particles collision specified by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollisionSDF3D.texture] or \\[member GPUParticlesAttractorVectorField3D.texture] depending on the `particles_collision` type."]
        pub fn particles_collision_set_field_texture(&mut self, particles_collision: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles_collision, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_field_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Requests an update for the 3D GPU particle collision heightfield. This may be automatically called by the 3D GPU particle collision heightfield depending on its \\[member GPUParticlesCollisionHeightField3D.update_mode]."]
        pub fn particles_collision_height_field_update(&mut self, particles_collision: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles_collision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_height_field_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the heightmap `resolution` for the 3D GPU particles heightfield collision specified by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollisionHeightField3D.resolution]."]
        pub fn particles_collision_set_height_field_resolution(&mut self, particles_collision: Rid, resolution: crate::classes::rendering_server::ParticlesCollisionHeightfieldResolution,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesCollisionHeightfieldResolution,);
            let args = (particles_collision, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_height_field_resolution", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the heightfield `mask` for the 3D GPU particles heightfield collision specified by the `particles_collision` RID. Equivalent to \\[member GPUParticlesCollisionHeightField3D.heightfield_mask]."]
        pub fn particles_collision_set_height_field_mask(&mut self, particles_collision: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (particles_collision, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_height_field_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new fog volume and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `fog_volume_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`FogVolume`][crate::classes::FogVolume]."]
        pub fn fog_volume_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shape of the fog volume to either [`FogVolumeShape::ELLIPSOID`][`crate::classes::rendering_server::FogVolumeShape::ELLIPSOID`], [`FogVolumeShape::CONE`][`crate::classes::rendering_server::FogVolumeShape::CONE`], [`FogVolumeShape::CYLINDER`][`crate::classes::rendering_server::FogVolumeShape::CYLINDER`], [`FogVolumeShape::BOX`][`crate::classes::rendering_server::FogVolumeShape::BOX`] or [`FogVolumeShape::WORLD`][`crate::classes::rendering_server::FogVolumeShape::WORLD`]."]
        pub fn fog_volume_set_shape(&mut self, fog_volume: Rid, shape: crate::classes::rendering_server::FogVolumeShape,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::FogVolumeShape,);
            let args = (fog_volume, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the size of the fog volume when shape is [`FogVolumeShape::ELLIPSOID`][`crate::classes::rendering_server::FogVolumeShape::ELLIPSOID`], [`FogVolumeShape::CONE`][`crate::classes::rendering_server::FogVolumeShape::CONE`], [`FogVolumeShape::CYLINDER`][`crate::classes::rendering_server::FogVolumeShape::CYLINDER`] or [`FogVolumeShape::BOX`][`crate::classes::rendering_server::FogVolumeShape::BOX`]."]
        pub fn fog_volume_set_size(&mut self, fog_volume: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (fog_volume, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Material`][crate::classes::Material] of the fog volume. Can be either a [`FogMaterial`][crate::classes::FogMaterial] or a custom [`ShaderMaterial`][crate::classes::ShaderMaterial]."]
        pub fn fog_volume_set_material(&mut self, fog_volume: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (fog_volume, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new 3D visibility notifier object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `visibility_notifier_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nTo place in a scene, attach this notifier to an instance using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`] using the returned RID.\n\n**Note:** The equivalent node is [`VisibleOnScreenNotifier3D`][crate::classes::VisibleOnScreenNotifier3D]."]
        pub fn visibility_notifier_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn visibility_notifier_set_aabb(&mut self, notifier: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (notifier, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_set_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn visibility_notifier_set_callbacks(&mut self, notifier: Rid, enter_callable: &Callable, exit_callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (notifier, RefArg::new(enter_callable), RefArg::new(exit_callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_set_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an occluder instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `occluder_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`Occluder3D`][crate::classes::Occluder3D] (not to be confused with the [`OccluderInstance3D`][crate::classes::OccluderInstance3D] node)."]
        pub fn occluder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "occluder_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mesh data for the given occluder RID, which controls the shape of the occlusion culling that will be performed."]
        pub fn occluder_set_mesh(&mut self, occluder: Rid, vertices: &PackedVector3Array, indices: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedInt32Array >,);
            let args = (occluder, RefArg::new(vertices), RefArg::new(indices),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "occluder_set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a 3D camera and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `camera_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`Camera3D`][crate::classes::Camera3D]."]
        pub fn camera_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets camera to use perspective projection. Objects on the screen becomes smaller when they are far away."]
        pub fn camera_set_perspective(&mut self, camera: Rid, fovy_degrees: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (camera, fovy_degrees, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_perspective", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets camera to use orthogonal projection, also known as orthographic projection. Objects remain the same size on the screen no matter how far away they are."]
        pub fn camera_set_orthogonal(&mut self, camera: Rid, size: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (camera, size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_orthogonal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets camera to use frustum projection. This mode allows adjusting the `offset` argument to create \"tilted frustum\" effects."]
        pub fn camera_set_frustum(&mut self, camera: Rid, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, Vector2, f32, f32,);
            let args = (camera, size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_frustum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets [`Transform3D`][crate::builtin::Transform3D] of camera."]
        pub fn camera_set_transform(&mut self, camera: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (camera, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the cull mask associated with this camera. The cull mask describes which 3D layers are rendered by this camera. Equivalent to \\[member Camera3D.cull_mask]."]
        pub fn camera_set_cull_mask(&mut self, camera: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (camera, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the environment used by this camera. Equivalent to \\[member Camera3D.environment]."]
        pub fn camera_set_environment(&mut self, camera: Rid, env: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, env,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera_attributes created with [`camera_attributes_create`][`crate::classes::RenderingServer::camera_attributes_create`] to the given camera."]
        pub fn camera_set_camera_attributes(&mut self, camera: Rid, effects: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the compositor used by this camera. Equivalent to \\[member Camera3D.compositor]."]
        pub fn camera_set_compositor(&mut self, camera: Rid, compositor: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, compositor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_compositor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, preserves the horizontal aspect ratio which is equivalent to [`KeepAspect::WIDTH`][`crate::classes::camera_3d::KeepAspect::WIDTH`]. If `false`, preserves the vertical aspect ratio which is equivalent to [`KeepAspect::HEIGHT`][`crate::classes::camera_3d::KeepAspect::HEIGHT`]."]
        pub fn camera_set_use_vertical_aspect(&mut self, camera: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (camera, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_use_vertical_aspect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an empty viewport and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `viewport_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`Viewport`][crate::classes::Viewport]."]
        pub fn viewport_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the viewport uses augmented or virtual reality technologies. See [`XRInterface`][crate::classes::XrInterface]."]
        pub fn viewport_set_use_xr(&mut self, viewport: Rid, use_xr: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, use_xr,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_xr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's width and height in pixels."]
        pub fn viewport_set_size(&mut self, viewport: Rid, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (viewport, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, sets the viewport active, else sets it inactive."]
        pub fn viewport_set_active(&mut self, viewport: Rid, active: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's parent to the viewport specified by the `parent_viewport` RID."]
        pub fn viewport_set_parent_viewport(&mut self, viewport: Rid, parent_viewport: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, parent_viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_parent_viewport", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Copies the viewport to a region of the screen specified by `rect`. If [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`] is `true`, then the viewport does not use a framebuffer and the contents of the viewport are rendered directly to screen. However, note that the root viewport is drawn last, therefore it will draw over the screen. Accordingly, you must set the root viewport to an area that does not cover the area that you have attached this viewport to.\n\nFor example, you can set the root viewport to not render at all with the following code:\n\n\n```gdscript\nfunc _ready():\n\tRenderingServer.viewport_attach_to_screen(get_viewport().get_viewport_rid(), Rect2())\n\tRenderingServer.viewport_attach_to_screen($Viewport.get_viewport_rid(), Rect2(0, 0, 600, 600))\n```\n\n\nUsing this can result in significant optimization, especially on lower-end devices. However, it comes at the cost of having to manage your viewports manually. For further optimization, see [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`]."]
        pub(crate) fn viewport_attach_to_screen_full(&mut self, viewport: Rid, rect: Rect2, screen: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, i32,);
            let args = (viewport, rect, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_to_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`viewport_attach_to_screen_ex`][Self::viewport_attach_to_screen_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Copies the viewport to a region of the screen specified by `rect`. If [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`] is `true`, then the viewport does not use a framebuffer and the contents of the viewport are rendered directly to screen. However, note that the root viewport is drawn last, therefore it will draw over the screen. Accordingly, you must set the root viewport to an area that does not cover the area that you have attached this viewport to.\n\nFor example, you can set the root viewport to not render at all with the following code:\n\n\n```gdscript\nfunc _ready():\n\tRenderingServer.viewport_attach_to_screen(get_viewport().get_viewport_rid(), Rect2())\n\tRenderingServer.viewport_attach_to_screen($Viewport.get_viewport_rid(), Rect2(0, 0, 600, 600))\n```\n\n\nUsing this can result in significant optimization, especially on lower-end devices. However, it comes at the cost of having to manage your viewports manually. For further optimization, see [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`]."]
        #[inline]
        pub fn viewport_attach_to_screen(&mut self, viewport: Rid,) {
            self.viewport_attach_to_screen_ex(viewport,) . done()
        }
        #[doc = "Copies the viewport to a region of the screen specified by `rect`. If [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`] is `true`, then the viewport does not use a framebuffer and the contents of the viewport are rendered directly to screen. However, note that the root viewport is drawn last, therefore it will draw over the screen. Accordingly, you must set the root viewport to an area that does not cover the area that you have attached this viewport to.\n\nFor example, you can set the root viewport to not render at all with the following code:\n\n\n```gdscript\nfunc _ready():\n\tRenderingServer.viewport_attach_to_screen(get_viewport().get_viewport_rid(), Rect2())\n\tRenderingServer.viewport_attach_to_screen($Viewport.get_viewport_rid(), Rect2(0, 0, 600, 600))\n```\n\n\nUsing this can result in significant optimization, especially on lower-end devices. However, it comes at the cost of having to manage your viewports manually. For further optimization, see [`viewport_set_render_direct_to_screen`][`crate::classes::RenderingServer::viewport_set_render_direct_to_screen`]."]
        #[inline]
        pub fn viewport_attach_to_screen_ex < 'ex > (&'ex mut self, viewport: Rid,) -> ExViewportAttachToScreen < 'ex > {
            ExViewportAttachToScreen::new(self, viewport,)
        }
        #[doc = "If `true`, render the contents of the viewport directly to screen. This allows a low-level optimization where you can skip drawing a viewport to the root viewport. While this optimization can result in a significant increase in speed (especially on older devices), it comes at a cost of usability. When this is enabled, you cannot read from the viewport or from the screen_texture. You also lose the benefit of certain window settings, such as the various stretch modes. Another consequence to be aware of is that in 2D the rendering happens in window coordinates, so if you have a viewport that is double the size of the window, and you set this, then only the portion that fits within the window will be drawn, no automatic scaling is possible, even if your game scene is significantly larger than the window size."]
        pub fn viewport_set_render_direct_to_screen(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_render_direct_to_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the rendering mask associated with this [`Viewport`][crate::classes::Viewport]. Only [`CanvasItem`][crate::classes::CanvasItem] nodes with a matching rendering visibility layer will be rendered by this [`Viewport`][crate::classes::Viewport]."]
        pub fn viewport_set_canvas_cull_mask(&mut self, viewport: Rid, canvas_cull_mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (viewport, canvas_cull_mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the 3D resolution scaling mode. Bilinear scaling renders at different resolution to either undersample or supersample the viewport. FidelityFX Super Resolution 1.0, abbreviated to FSR, is an upscaling technology that produces high quality images at fast framerates by using a spatially aware upscaling algorithm. FSR is slightly more expensive than bilinear, but it produces significantly higher image quality. FSR should be used where possible."]
        pub fn viewport_set_scaling_3d_mode(&mut self, viewport: Rid, scaling_3d_mode: crate::classes::rendering_server::ViewportScaling3DMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportScaling3DMode,);
            let args = (viewport, scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Scales the 3D render buffer based on the viewport size uses an image filter specified in \\[enum ViewportScaling3DMode] to scale the output image to the full viewport size. Values lower than `1.0` can be used to speed up 3D rendering at the cost of quality (undersampling). Values greater than `1.0` are only valid for bilinear mode and can be used to improve 3D rendering quality at a high performance cost (supersampling). See also \\[enum ViewportMSAA] for multi-sample antialiasing, which is significantly cheaper but only smoothens the edges of polygons.\n\nWhen using FSR upscaling, AMD recommends exposing the following values as preset options to users \"Ultra Quality: 0.77\", \"Quality: 0.67\", \"Balanced: 0.59\", \"Performance: 0.5\" instead of exposing the entire scale."]
        pub fn viewport_set_scaling_3d_scale(&mut self, viewport: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scaling_3d_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Determines how sharp the upscaled image will be when using the FSR upscaling mode. Sharpness halves with every whole number. Values go from 0.0 (sharpest) to 2.0. Values above 2.0 won't make a visible difference."]
        pub fn viewport_set_fsr_sharpness(&mut self, viewport: Rid, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, sharpness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Affects the final texture sharpness by reading from a lower or higher mipmap (also called \"texture LOD bias\"). Negative values make mipmapped textures sharper but grainier when viewed at a distance, while positive values make mipmapped textures blurrier (even when up close). To get sharper textures at a distance without introducing too much graininess, set this between `-0.75` and `0.0`. Enabling temporal antialiasing (\\[member ProjectSettings.rendering/anti_aliasing/quality/use_taa]) can help reduce the graininess visible when using negative mipmap bias.\n\n**Note:** When the 3D scaling mode is set to FSR 1.0, this value is used to adjust the automatic mipmap bias which is calculated internally based on the scale factor. The formula for this is `-log2(1.0 / scale) + mipmap_bias`."]
        pub fn viewport_set_texture_mipmap_bias(&mut self, viewport: Rid, mipmap_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, mipmap_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum number of samples to take when using anisotropic filtering on textures (as a power of two). A higher sample count will result in sharper textures at oblique angles, but is more expensive to compute. A value of `0` forcibly disables anisotropic filtering, even on materials where it is enabled.\n\nThe anisotropic filtering level also affects decals and light projectors if they are configured to use anisotropic filtering. See \\[member ProjectSettings.rendering/textures/decals/filter] and \\[member ProjectSettings.rendering/textures/light_projectors/filter].\n\n**Note:** In 3D, for this setting to have an effect, set \\[member BaseMaterial3D.texture_filter] to [`TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC`][`crate::classes::base_material_3d::TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC`] or [`TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC`][`crate::classes::base_material_3d::TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC`] on materials.\n\n**Note:** In 2D, for this setting to have an effect, set \\[member CanvasItem.texture_filter] to [`TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC`][`crate::classes::canvas_item::TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC`] or [`TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC`][`crate::classes::canvas_item::TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC`] on the [`CanvasItem`][crate::classes::CanvasItem] node displaying the texture (or in [`CanvasTexture`][crate::classes::CanvasTexture]). However, anisotropic filtering is rarely useful in 2D, so only enable it for textures in 2D if it makes a meaningful visual difference."]
        pub fn viewport_set_anisotropic_filtering_level(&mut self, viewport: Rid, anisotropic_filtering_level: crate::classes::rendering_server::ViewportAnisotropicFiltering,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportAnisotropicFiltering,);
            let args = (viewport, anisotropic_filtering_level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets when the viewport should be updated."]
        pub fn viewport_set_update_mode(&mut self, viewport: Rid, update_mode: crate::classes::rendering_server::ViewportUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportUpdateMode,);
            let args = (viewport, update_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the viewport's update mode.\n\n**Warning:** Calling this from any thread other than the rendering thread will be detrimental to performance."]
        pub fn viewport_get_update_mode(&self, viewport: Rid,) -> crate::classes::rendering_server::ViewportUpdateMode {
            type CallRet = crate::classes::rendering_server::ViewportUpdateMode;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the clear mode of a viewport."]
        pub fn viewport_set_clear_mode(&mut self, viewport: Rid, clear_mode: crate::classes::rendering_server::ViewportClearMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportClearMode,);
            let args = (viewport, clear_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_clear_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the render target for the viewport."]
        pub fn viewport_get_render_target(&self, viewport: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_render_target", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the viewport's last rendered frame."]
        pub fn viewport_get_texture(&self, viewport: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the viewport's 3D elements are not rendered."]
        pub fn viewport_set_disable_3d(&mut self, viewport: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_disable_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the viewport's canvas (i.e. 2D and GUI elements) is not rendered."]
        pub fn viewport_set_disable_2d(&mut self, viewport: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_disable_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's environment mode which allows enabling or disabling rendering of 3D environment over 2D canvas. When disabled, 2D will not be affected by the environment. When enabled, 2D will be affected by the environment if the environment background mode is [`EnvironmentBg::CANVAS`][`crate::classes::rendering_server::EnvironmentBg::CANVAS`]. The default behavior is to inherit the setting from the viewport's parent. If the topmost parent is also set to [`ViewportEnvironmentMode::INHERIT`][`crate::classes::rendering_server::ViewportEnvironmentMode::INHERIT`], then the behavior will be the same as if it was set to [`ViewportEnvironmentMode::ENABLED`][`crate::classes::rendering_server::ViewportEnvironmentMode::ENABLED`]."]
        pub fn viewport_set_environment_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportEnvironmentMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportEnvironmentMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_environment_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a viewport's camera."]
        pub fn viewport_attach_camera(&mut self, viewport: Rid, camera: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, camera,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_camera", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a viewport's scenario. The scenario contains information about environment information, reflection atlas, etc."]
        pub fn viewport_set_scenario(&mut self, viewport: Rid, scenario: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scenario", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a viewport's canvas."]
        pub fn viewport_attach_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Detaches a viewport from a canvas."]
        pub fn viewport_remove_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_remove_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, canvas item transforms (i.e. origin position) are snapped to the nearest pixel when rendering. This can lead to a crisper appearance at the cost of less smooth movement, especially when [`Camera2D`][crate::classes::Camera2D] smoothing is enabled. Equivalent to \\[member ProjectSettings.rendering/2d/snap/snap_2d_transforms_to_pixel]."]
        pub fn viewport_set_snap_2d_transforms_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_snap_2d_transforms_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, canvas item vertices (i.e. polygon points) are snapped to the nearest pixel when rendering. This can lead to a crisper appearance at the cost of less smooth movement, especially when [`Camera2D`][crate::classes::Camera2D] smoothing is enabled. Equivalent to \\[member ProjectSettings.rendering/2d/snap/snap_2d_vertices_to_pixel]."]
        pub fn viewport_set_snap_2d_vertices_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_snap_2d_vertices_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default texture filtering mode for the specified `viewport` RID."]
        pub fn viewport_set_default_canvas_item_texture_filter(&mut self, viewport: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (viewport, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_default_canvas_item_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default texture repeat mode for the specified `viewport` RID."]
        pub fn viewport_set_default_canvas_item_texture_repeat(&mut self, viewport: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (viewport, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_default_canvas_item_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transformation of a viewport's canvas."]
        pub fn viewport_set_canvas_transform(&mut self, viewport: Rid, canvas: Rid, offset: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D,);
            let args = (viewport, canvas, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the stacking order for a viewport's canvas.\n\n`layer` is the actual canvas layer, while `sublayer` specifies the stacking order of the canvas among those in the same layer.\n\n**Note:** `layer` should be between `CANVAS_LAYER_MIN` and `CANVAS_LAYER_MAX` (inclusive). Any other value will wrap around."]
        pub fn viewport_set_canvas_stacking(&mut self, viewport: Rid, canvas: Rid, layer: i32, sublayer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i32, i32,);
            let args = (viewport, canvas, layer, sublayer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_stacking", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the viewport renders its background as transparent."]
        pub fn viewport_set_transparent_background(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_transparent_background", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's global transformation matrix."]
        pub fn viewport_set_global_canvas_transform(&mut self, viewport: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (viewport, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_global_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's 2D signed distance field \\[member ProjectSettings.rendering/2d/sdf/oversize] and \\[member ProjectSettings.rendering/2d/sdf/scale]. This is used when sampling the signed distance field in [`CanvasItem`][crate::classes::CanvasItem] shaders as well as [`GPUParticles2D`][crate::classes::GpuParticles2D] collision. This is _not_ used by SDFGI in 3D rendering."]
        pub fn viewport_set_sdf_oversize_and_scale(&mut self, viewport: Rid, oversize: crate::classes::rendering_server::ViewportSdfOversize, scale: crate::classes::rendering_server::ViewportSdfScale,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportSdfOversize, crate::classes::rendering_server::ViewportSdfScale,);
            let args = (viewport, oversize, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_sdf_oversize_and_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `size` of the shadow atlas's images (used for omni and spot lights) on the viewport specified by the `viewport` RID. The value is rounded up to the nearest power of 2. If `use_16_bits` is `true`, use 16 bits for the omni/spot shadow depth map. Enabling this results in shadows having less precision and may result in shadow acne, but can lead to performance improvements on some devices.\n\n**Note:** If this is set to `0`, no positional shadows will be visible at all. This can improve performance significantly on low-end systems by reducing both the CPU and GPU load (as fewer draw calls are needed to draw the scene without shadows)."]
        pub(crate) fn viewport_set_positional_shadow_atlas_size_full(&mut self, viewport: Rid, size: i32, use_16_bits: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (viewport, size, use_16_bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_positional_shadow_atlas_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`viewport_set_positional_shadow_atlas_size_ex`][Self::viewport_set_positional_shadow_atlas_size_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `size` of the shadow atlas's images (used for omni and spot lights) on the viewport specified by the `viewport` RID. The value is rounded up to the nearest power of 2. If `use_16_bits` is `true`, use 16 bits for the omni/spot shadow depth map. Enabling this results in shadows having less precision and may result in shadow acne, but can lead to performance improvements on some devices.\n\n**Note:** If this is set to `0`, no positional shadows will be visible at all. This can improve performance significantly on low-end systems by reducing both the CPU and GPU load (as fewer draw calls are needed to draw the scene without shadows)."]
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size(&mut self, viewport: Rid, size: i32,) {
            self.viewport_set_positional_shadow_atlas_size_ex(viewport, size,) . done()
        }
        #[doc = "Sets the `size` of the shadow atlas's images (used for omni and spot lights) on the viewport specified by the `viewport` RID. The value is rounded up to the nearest power of 2. If `use_16_bits` is `true`, use 16 bits for the omni/spot shadow depth map. Enabling this results in shadows having less precision and may result in shadow acne, but can lead to performance improvements on some devices.\n\n**Note:** If this is set to `0`, no positional shadows will be visible at all. This can improve performance significantly on low-end systems by reducing both the CPU and GPU load (as fewer draw calls are needed to draw the scene without shadows)."]
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size_ex < 'ex > (&'ex mut self, viewport: Rid, size: i32,) -> ExViewportSetPositionalShadowAtlasSize < 'ex > {
            ExViewportSetPositionalShadowAtlasSize::new(self, viewport, size,)
        }
        #[doc = "Sets the number of subdivisions to use in the specified shadow atlas `quadrant` for omni and spot shadows. See also [`set_positional_shadow_atlas_quadrant_subdiv`][`crate::classes::Viewport::set_positional_shadow_atlas_quadrant_subdiv`]."]
        pub fn viewport_set_positional_shadow_atlas_quadrant_subdivision(&mut self, viewport: Rid, quadrant: i32, subdivision: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (viewport, quadrant, subdivision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_positional_shadow_atlas_quadrant_subdivision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the multisample antialiasing mode for 3D on the specified `viewport` RID. Equivalent to \\[member ProjectSettings.rendering/anti_aliasing/quality/msaa_3d] or \\[member Viewport.msaa_3d]."]
        pub fn viewport_set_msaa_3d(&mut self, viewport: Rid, msaa: crate::classes::rendering_server::ViewportMsaa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportMsaa,);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the multisample antialiasing mode for 2D/Canvas on the specified `viewport` RID. Equivalent to \\[member ProjectSettings.rendering/anti_aliasing/quality/msaa_2d] or \\[member Viewport.msaa_2d]."]
        pub fn viewport_set_msaa_2d(&mut self, viewport: Rid, msaa: crate::classes::rendering_server::ViewportMsaa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportMsaa,);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_msaa_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, 2D rendering will use a high dynamic range (HDR) `RGBA16` format framebuffer. Additionally, 2D rendering will be performed on linear values and will be converted using the appropriate transfer function immediately before blitting to the screen (if the Viewport is attached to the screen).\n\nPractically speaking, this means that the end result of the Viewport will not be clamped to the `0-1` range and can be used in 3D rendering without color encoding adjustments. This allows 2D rendering to take advantage of effects requiring high dynamic range (e.g. 2D glow) as well as substantially improves the appearance of effects requiring highly detailed gradients. This setting has the same effect as \\[member Viewport.use_hdr_2d]."]
        pub fn viewport_set_use_hdr_2d(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_hdr_2d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the viewport's screen-space antialiasing mode. Equivalent to \\[member ProjectSettings.rendering/anti_aliasing/quality/screen_space_aa] or \\[member Viewport.screen_space_aa]."]
        pub fn viewport_set_screen_space_aa(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportScreenSpaceAa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportScreenSpaceAa,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, use temporal antialiasing. Equivalent to \\[member ProjectSettings.rendering/anti_aliasing/quality/use_taa] or \\[member Viewport.use_taa]."]
        pub fn viewport_set_use_taa(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_taa", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Equivalent to \\[member Viewport.use_debanding]. See also \\[member ProjectSettings.rendering/anti_aliasing/quality/use_debanding]."]
        pub fn viewport_set_use_debanding(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, enables occlusion culling on the specified viewport. Equivalent to \\[member ProjectSettings.rendering/occlusion_culling/use_occlusion_culling]."]
        pub fn viewport_set_use_occlusion_culling(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member ProjectSettings.rendering/occlusion_culling/occlusion_rays_per_thread] to use for occlusion culling. This parameter is global and cannot be set on a per-viewport basis."]
        pub fn viewport_set_occlusion_rays_per_thread(&mut self, rays_per_thread: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (rays_per_thread,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_occlusion_rays_per_thread", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member ProjectSettings.rendering/occlusion_culling/bvh_build_quality] to use for occlusion culling. This parameter is global and cannot be set on a per-viewport basis."]
        pub fn viewport_set_occlusion_culling_build_quality(&mut self, quality: crate::classes::rendering_server::ViewportOcclusionCullingBuildQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportOcclusionCullingBuildQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_occlusion_culling_build_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a statistic about the rendering engine which can be used for performance profiling. This is separated into render pass `type`s, each of them having the same `info`s you can query (different passes will return different values).\n\nSee also [`get_rendering_info`][`crate::classes::RenderingServer::get_rendering_info`], which returns global information across all viewports.\n\n**Note:** Viewport rendering information is not available until at least 2 frames have been rendered by the engine. If rendering information is not available, [`viewport_get_render_info`][`crate::classes::RenderingServer::viewport_get_render_info`] returns `0`. To print rendering information in `_ready()` successfully, use the following:\n\n```gdscript\nfunc _ready():\n\tfor _i in 2:\n\t\tawait get_tree().process_frame\n\n\tprint(\n\t\t\tRenderingServer.viewport_get_render_info(get_viewport().get_viewport_rid(),\n\t\t\tRenderingServer.VIEWPORT_RENDER_INFO_TYPE_VISIBLE,\n\t\t\tRenderingServer.VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME)\n\t)\n```"]
        pub fn viewport_get_render_info(&mut self, viewport: Rid, type_: crate::classes::rendering_server::ViewportRenderInfoType, info: crate::classes::rendering_server::ViewportRenderInfo,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, crate::classes::rendering_server::ViewportRenderInfoType, crate::classes::rendering_server::ViewportRenderInfo,);
            let args = (viewport, type_, info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_render_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the debug draw mode of a viewport."]
        pub fn viewport_set_debug_draw(&mut self, viewport: Rid, draw: crate::classes::rendering_server::ViewportDebugDraw,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportDebugDraw,);
            let args = (viewport, draw,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_debug_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the measurement for the given `viewport` RID (obtained using [`get_viewport_rid`][`crate::classes::Viewport::get_viewport_rid`]). Once enabled, [`viewport_get_measured_render_time_cpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_cpu`] and [`viewport_get_measured_render_time_gpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_gpu`] will return values greater than `0.0` when queried with the given `viewport`."]
        pub fn viewport_set_measure_render_time(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_measure_render_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the CPU time taken to render the last frame in milliseconds. This _only_ includes time spent in rendering-related operations; scripts' `_process` functions and other engine subsystems are not included in this readout. To get a complete readout of CPU time spent to render the scene, sum the render times of all viewports that are drawn every frame plus [`get_frame_setup_time_cpu`][`crate::classes::RenderingServer::get_frame_setup_time_cpu`]. Unlike [`get_frames_per_second`][`crate::classes::Engine::get_frames_per_second`], this method will accurately reflect CPU utilization even if framerate is capped via V-Sync or \\[member Engine.max_fps]. See also [`viewport_get_measured_render_time_gpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_gpu`].\n\n**Note:** Requires measurements to be enabled on the specified `viewport` using [`viewport_set_measure_render_time`][`crate::classes::RenderingServer::viewport_set_measure_render_time`]. Otherwise, this method returns `0.0`."]
        pub fn viewport_get_measured_render_time_cpu(&self, viewport: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_measured_render_time_cpu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the GPU time taken to render the last frame in milliseconds. To get a complete readout of GPU time spent to render the scene, sum the render times of all viewports that are drawn every frame. Unlike [`get_frames_per_second`][`crate::classes::Engine::get_frames_per_second`], this method accurately reflects GPU utilization even if framerate is capped via V-Sync or \\[member Engine.max_fps]. See also [`viewport_get_measured_render_time_cpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_cpu`].\n\n**Note:** Requires measurements to be enabled on the specified `viewport` using [`viewport_set_measure_render_time`][`crate::classes::RenderingServer::viewport_set_measure_render_time`]. Otherwise, this method returns `0.0`.\n\n**Note:** When GPU utilization is low enough during a certain period of time, GPUs will decrease their power state (which in turn decreases core and memory clock speeds). This can cause the reported GPU time to increase if GPU utilization is kept low enough by a framerate cap (compared to what it would be at the GPU's highest power state). Keep this in mind when benchmarking using [`viewport_get_measured_render_time_gpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_gpu`]. This behavior can be overridden in the graphics driver settings at the cost of higher power usage."]
        pub fn viewport_get_measured_render_time_gpu(&self, viewport: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_measured_render_time_gpu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the Variable Rate Shading (VRS) mode for the viewport. If the GPU does not support VRS, this property is ignored. Equivalent to \\[member ProjectSettings.rendering/vrs/mode]."]
        pub fn viewport_set_vrs_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportVrsMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportVrsMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the update mode for Variable Rate Shading (VRS) for the viewport. VRS requires the input texture to be converted to the format usable by the VRS method supported by the hardware. The update mode defines how often this happens. If the GPU does not support VRS, or VRS is not enabled, this property is ignored.\n\nIf set to [`ViewportVrsUpdateMode::ONCE`][`crate::classes::rendering_server::ViewportVrsUpdateMode::ONCE`], the input texture is copied once and the mode is changed to [`ViewportVrsUpdateMode::DISABLED`][`crate::classes::rendering_server::ViewportVrsUpdateMode::DISABLED`]."]
        pub fn viewport_set_vrs_update_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportVrsUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportVrsUpdateMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The texture to use when the VRS mode is set to [`ViewportVrsMode::TEXTURE`][`crate::classes::rendering_server::ViewportVrsMode::TEXTURE`]. Equivalent to \\[member ProjectSettings.rendering/vrs/texture]."]
        pub fn viewport_set_vrs_texture(&mut self, viewport: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an empty sky and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `sky_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method."]
        pub fn sky_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `radiance_size` of the sky specified by the `sky` RID (in pixels). Equivalent to \\[member Sky.radiance_size]."]
        pub fn sky_set_radiance_size(&mut self, sky: Rid, radiance_size: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (sky, radiance_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_radiance_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the process `mode` of the sky specified by the `sky` RID. Equivalent to \\[member Sky.process_mode]."]
        pub fn sky_set_mode(&mut self, sky: Rid, mode: crate::classes::rendering_server::SkyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::SkyMode,);
            let args = (sky, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the material that the sky uses to render the background, ambient and reflection maps."]
        pub fn sky_set_material(&mut self, sky: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (sky, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates and returns an [`Image`][crate::classes::Image] containing the radiance map for the specified `sky` RID. This supports built-in sky material and custom sky shaders. If `bake_irradiance` is `true`, the irradiance map is saved instead of the radiance map. The radiance map is used to render reflected light, while the irradiance map is used to render ambient light. See also [`environment_bake_panorama`][`crate::classes::RenderingServer::environment_bake_panorama`].\n\n**Note:** The image is saved using linear encoding without any tonemapping performed, which means it will look too dark if viewed directly in an image editor. `energy` values above `1.0` can be used to brighten the resulting image.\n\n**Note:** `size` should be a 2:1 aspect ratio for the generated panorama to have square pixels. For radiance maps, there is no point in using a height greater than \\[member Sky.radiance_size], as it won't increase detail. Irradiance maps only contain low-frequency data, so there is usually no point in going past a size of 128×64 pixels when saving an irradiance map."]
        pub fn sky_bake_panorama(&mut self, sky: Rid, energy: f32, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, f32, bool, Vector2i,);
            let args = (sky, energy, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_bake_panorama", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new rendering effect and adds it to the RenderingServer. It can be accessed with the RID that is returned.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method."]
        pub fn compositor_effect_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables/disables this rendering effect."]
        pub fn compositor_effect_set_enabled(&mut self, effect: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (effect, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the callback type (`callback_type`) and callback method(`callback`) for this rendering effect."]
        pub fn compositor_effect_set_callback(&mut self, effect: Rid, callback_type: crate::classes::rendering_server::CompositorEffectCallbackType, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::rendering_server::CompositorEffectCallbackType, RefArg < 'a0, Callable >,);
            let args = (effect, callback_type, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the flag (`flag`) for this rendering effect to `true` or `false` (`set`)."]
        pub fn compositor_effect_set_flag(&mut self, effect: Rid, flag: crate::classes::rendering_server::CompositorEffectFlags, set: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CompositorEffectFlags, bool,);
            let args = (effect, flag, set,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new compositor and adds it to the RenderingServer. It can be accessed with the RID that is returned.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method."]
        pub fn compositor_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the compositor effects for the specified compositor RID. `effects` should be an array containing RIDs created with [`compositor_effect_create`][`crate::classes::RenderingServer::compositor_effect_create`]."]
        pub fn compositor_set_compositor_effects(&mut self, compositor: Rid, effects: &Array < Rid >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Rid > >,);
            let args = (compositor, RefArg::new(effects),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_set_compositor_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates an environment and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `environment_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`Environment`][crate::classes::Environment]."]
        pub fn environment_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the environment's background mode. Equivalent to \\[member Environment.background_mode]."]
        pub fn environment_set_background(&mut self, env: Rid, bg: crate::classes::rendering_server::EnvironmentBg,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::EnvironmentBg,);
            let args = (env, bg,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_background", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera ID to be used as environment background."]
        pub fn environment_set_camera_id(&mut self, env: Rid, id: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (env, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_camera_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`Sky`][crate::classes::Sky] to be used as the environment's background when using _BGMode_ sky. Equivalent to \\[member Environment.sky]."]
        pub fn environment_set_sky(&mut self, env: Rid, sky: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (env, sky,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom field of view for the background [`Sky`][crate::classes::Sky]. Equivalent to \\[member Environment.sky_custom_fov]."]
        pub fn environment_set_sky_custom_fov(&mut self, env: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (env, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky_custom_fov", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the rotation of the background [`Sky`][crate::classes::Sky] expressed as a [`Basis`][crate::builtin::Basis]. Equivalent to \\[member Environment.sky_rotation], where the rotation vector is used to construct the [`Basis`][crate::builtin::Basis]."]
        pub fn environment_set_sky_orientation(&mut self, env: Rid, orientation: Basis,) {
            type CallRet = ();
            type CallParams = (Rid, Basis,);
            let args = (env, orientation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Color displayed for clear areas of the scene. Only effective if using the [`EnvironmentBg::COLOR`][`crate::classes::rendering_server::EnvironmentBg::COLOR`] background mode."]
        pub fn environment_set_bg_color(&mut self, env: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (env, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the intensity of the background color."]
        pub fn environment_set_bg_energy(&mut self, env: Rid, multiplier: f32, exposure_value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (env, multiplier, exposure_value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_bg_energy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the maximum layer to use if using Canvas background mode."]
        pub fn environment_set_canvas_max_layer(&mut self, env: Rid, max_layer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (env, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_canvas_max_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the values to be used for ambient light rendering. See [`Environment`][crate::classes::Environment] for more details."]
        pub(crate) fn environment_set_ambient_light_full(&mut self, env: Rid, color: Color, ambient: crate::classes::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contribution: f32, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource,) {
            type CallRet = ();
            type CallParams = (Rid, Color, crate::classes::rendering_server::EnvironmentAmbientSource, f32, f32, crate::classes::rendering_server::EnvironmentReflectionSource,);
            let args = (env, color, ambient, energy, sky_contribution, reflection_source,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ambient_light", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`environment_set_ambient_light_ex`][Self::environment_set_ambient_light_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the values to be used for ambient light rendering. See [`Environment`][crate::classes::Environment] for more details."]
        #[inline]
        pub fn environment_set_ambient_light(&mut self, env: Rid, color: Color,) {
            self.environment_set_ambient_light_ex(env, color,) . done()
        }
        #[doc = "Sets the values to be used for ambient light rendering. See [`Environment`][crate::classes::Environment] for more details."]
        #[inline]
        pub fn environment_set_ambient_light_ex < 'ex > (&'ex mut self, env: Rid, color: Color,) -> ExEnvironmentSetAmbientLight < 'ex > {
            ExEnvironmentSetAmbientLight::new(self, env, color,)
        }
        #[doc = "Configures glow for the specified environment RID. See `glow_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        pub fn environment_set_glow(&mut self, env: Rid, enable: bool, levels: &PackedFloat32Array, intensity: f32, strength: f32, mix: f32, bloom_threshold: f32, blend_mode: crate::classes::rendering_server::EnvironmentGlowBlendMode, hdr_bleed_threshold: f32, hdr_bleed_scale: f32, hdr_luminance_cap: f32, glow_map_strength: f32, glow_map: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, bool, RefArg < 'a0, PackedFloat32Array >, f32, f32, f32, f32, crate::classes::rendering_server::EnvironmentGlowBlendMode, f32, f32, f32, f32, Rid,);
            let args = (env, enable, RefArg::new(levels), intensity, strength, mix, bloom_threshold, blend_mode, hdr_bleed_threshold, hdr_bleed_scale, hdr_luminance_cap, glow_map_strength, glow_map,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_glow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the variables to be used with the \"tonemap\" post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_tonemap(&mut self, env: Rid, tone_mapper: crate::classes::rendering_server::EnvironmentToneMapper, exposure: f32, white: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::EnvironmentToneMapper, f32, f32,);
            let args = (env, tone_mapper, exposure, white,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_tonemap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "See \\[member Environment.tonemap_agx_contrast] for more details."]
        pub fn environment_set_tonemap_agx_contrast(&mut self, env: Rid, agx_contrast: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (env, agx_contrast,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_tonemap_agx_contrast", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the values to be used with the \"adjustments\" post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_adjustment(&mut self, env: Rid, enable: bool, brightness: f32, contrast: f32, saturation: f32, use_1d_color_correction: bool, color_correction: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, bool, Rid,);
            let args = (env, enable, brightness, contrast, saturation, use_1d_color_correction, color_correction,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_adjustment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the variables to be used with the screen-space reflections (SSR) post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_ssr(&mut self, env: Rid, enable: bool, max_steps: i32, fade_in: f32, fade_out: f32, depth_tolerance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, i32, f32, f32, f32,);
            let args = (env, enable, max_steps, fade_in, fade_out, depth_tolerance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the variables to be used with the screen-space ambient occlusion (SSAO) post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_ssao(&mut self, env: Rid, enable: bool, radius: f32, intensity: f32, power: f32, detail: f32, horizon: f32, sharpness: f32, light_affect: f32, ao_channel_affect: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, f32, f32, f32, f32, f32,);
            let args = (env, enable, radius, intensity, power, detail, horizon, sharpness, light_affect, ao_channel_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssao", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Configures fog for the specified environment RID. See `fog_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        pub(crate) fn environment_set_fog_full(&mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32, fog_mode: crate::classes::rendering_server::EnvironmentFogMode,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Color, f32, f32, f32, f32, f32, f32, f32, crate::classes::rendering_server::EnvironmentFogMode,);
            let args = (env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_fog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`environment_set_fog_ex`][Self::environment_set_fog_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Configures fog for the specified environment RID. See `fog_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        #[inline]
        pub fn environment_set_fog(&mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) {
            self.environment_set_fog_ex(env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect,) . done()
        }
        #[doc = "Configures fog for the specified environment RID. See `fog_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        #[inline]
        pub fn environment_set_fog_ex < 'ex > (&'ex mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) -> ExEnvironmentSetFog < 'ex > {
            ExEnvironmentSetFog::new(self, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect,)
        }
        #[doc = "Configures fog depth for the specified environment RID. Only has an effect when the fog mode of the environment is [`EnvironmentFogMode::DEPTH`][`crate::classes::rendering_server::EnvironmentFogMode::DEPTH`]. See `fog_depth_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        pub fn environment_set_fog_depth(&mut self, env: Rid, curve: f32, begin: f32, end: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (env, curve, begin, end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_fog_depth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Configures signed distance field global illumination for the specified environment RID. See `sdfgi_*` properties in [`Environment`][crate::classes::Environment] for more information."]
        pub fn environment_set_sdfgi(&mut self, env: Rid, enable: bool, cascades: i32, min_cell_size: f32, y_scale: crate::classes::rendering_server::EnvironmentSdfgiYScale, use_occlusion: bool, bounce_feedback: f32, read_sky: bool, energy: f32, normal_bias: f32, probe_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, i32, f32, crate::classes::rendering_server::EnvironmentSdfgiYScale, bool, f32, bool, f32, f32, f32,);
            let args = (env, enable, cascades, min_cell_size, y_scale, use_occlusion, bounce_feedback, read_sky, energy, normal_bias, probe_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the variables to be used with the volumetric fog post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_volumetric_fog(&mut self, env: Rid, enable: bool, density: f32, albedo: Color, emission: Color, emission_energy: f32, anisotropy: f32, length: f32, p_detail_spread: f32, gi_inject: f32, temporal_reprojection: bool, temporal_reprojection_amount: f32, ambient_inject: f32, sky_affect: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, Color, Color, f32, f32, f32, f32, f32, bool, f32, f32, f32,);
            let args = (env, enable, density, albedo, emission, emission_energy, anisotropy, length, p_detail_spread, gi_inject, temporal_reprojection, temporal_reprojection_amount, ambient_inject, sky_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enable` is `true`, enables bicubic upscaling for glow which improves quality at the cost of performance. Equivalent to \\[member ProjectSettings.rendering/environment/glow/upscale_mode].\n\n**Note:** This setting is only effective when using the Forward+ or Mobile rendering methods, as Compatibility uses a different glow implementation."]
        pub fn environment_glow_set_use_bicubic_upscale(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_glow_set_use_bicubic_upscale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether screen-space reflections will be rendered at full or half size. Half size is faster, but may look pixelated or cause flickering."]
        pub fn environment_set_ssr_half_size(&mut self, half_size: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (half_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssr_half_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssr_roughness_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsrRoughnessQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsrRoughnessQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssr_roughness_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the quality level of the screen-space ambient occlusion (SSAO) post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_ssao_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsaoQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsaoQuality, bool, f32, i32, f32, f32,);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssao_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the quality level of the screen-space indirect lighting (SSIL) post-process effect. See [`Environment`][crate::classes::Environment] for more details."]
        pub fn environment_set_ssil_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsilQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsilQuality, bool, f32, i32, f32, f32,);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssil_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of rays to throw per frame when computing signed distance field global illumination. Equivalent to \\[member ProjectSettings.rendering/global_illumination/sdfgi/probe_ray_count]."]
        pub fn environment_set_sdfgi_ray_count(&mut self, ray_count: crate::classes::rendering_server::EnvironmentSdfgiRayCount,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiRayCount,);
            let args = (ray_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_ray_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the number of frames to use for converging signed distance field global illumination. Equivalent to \\[member ProjectSettings.rendering/global_illumination/sdfgi/frames_to_converge]."]
        pub fn environment_set_sdfgi_frames_to_converge(&mut self, frames: crate::classes::rendering_server::EnvironmentSdfgiFramesToConverge,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiFramesToConverge,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_frames_to_converge", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the update speed for dynamic lights' indirect lighting when computing signed distance field global illumination. Equivalent to \\[member ProjectSettings.rendering/global_illumination/sdfgi/frames_to_update_lights]."]
        pub fn environment_set_sdfgi_frames_to_update_light(&mut self, frames: crate::classes::rendering_server::EnvironmentSdfgiFramesToUpdateLight,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiFramesToUpdateLight,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_frames_to_update_light", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the resolution of the volumetric fog's froxel buffer. `size` is modified by the screen's aspect ratio and then used to set the width and height of the buffer. While `depth` is directly used to set the depth of the buffer."]
        pub fn environment_set_volumetric_fog_volume_size(&mut self, size: i32, depth: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (size, depth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog_volume_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables filtering of the volumetric fog scattering buffer. This results in much smoother volumes with very few under-sampling artifacts."]
        pub fn environment_set_volumetric_fog_filter_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog_filter_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates and returns an [`Image`][crate::classes::Image] containing the radiance map for the specified `environment` RID's sky. This supports built-in sky material and custom sky shaders. If `bake_irradiance` is `true`, the irradiance map is saved instead of the radiance map. The radiance map is used to render reflected light, while the irradiance map is used to render ambient light. See also [`sky_bake_panorama`][`crate::classes::RenderingServer::sky_bake_panorama`].\n\n**Note:** The image is saved using linear encoding without any tonemapping performed, which means it will look too dark if viewed directly in an image editor.\n\n**Note:** `size` should be a 2:1 aspect ratio for the generated panorama to have square pixels. For radiance maps, there is no point in using a height greater than \\[member Sky.radiance_size], as it won't increase detail. Irradiance maps only contain low-frequency data, so there is usually no point in going past a size of 128×64 pixels when saving an irradiance map."]
        pub fn environment_bake_panorama(&mut self, environment: Rid, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, bool, Vector2i,);
            let args = (environment, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_bake_panorama", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the screen-space roughness limiter parameters, such as whether it should be enabled and its thresholds. Equivalent to \\[member ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/enabled], \\[member ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/amount] and \\[member ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/limit]."]
        pub fn screen_space_roughness_limiter_set_active(&mut self, enable: bool, amount: f32, limit: f32,) {
            type CallRet = ();
            type CallParams = (bool, f32, f32,);
            let args = (enable, amount, limit,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "screen_space_roughness_limiter_set_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets \\[member ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_quality] to use when rendering materials that have subsurface scattering enabled."]
        pub fn sub_surface_scattering_set_quality(&mut self, quality: crate::classes::rendering_server::SubSurfaceScatteringQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::SubSurfaceScatteringQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sub_surface_scattering_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_scale] and \\[member ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_depth_scale] to use when rendering materials that have subsurface scattering enabled."]
        pub fn sub_surface_scattering_set_scale(&mut self, scale: f32, depth_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32,);
            let args = (scale, depth_scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sub_surface_scattering_set_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a camera attributes object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `camera_attributes_` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`CameraAttributes`][crate::classes::CameraAttributes]."]
        pub fn camera_attributes_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the quality level of the DOF blur effect to `quality`. `use_jitter` can be used to jitter samples taken during the blur pass to hide artifacts at the cost of looking more fuzzy."]
        pub fn camera_attributes_set_dof_blur_quality(&mut self, quality: crate::classes::rendering_server::DofBlurQuality, use_jitter: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DofBlurQuality, bool,);
            let args = (quality, use_jitter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur_quality", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shape of the DOF bokeh pattern to `shape`. Different shapes may be used to achieve artistic effect, or to meet performance targets."]
        pub fn camera_attributes_set_dof_blur_bokeh_shape(&mut self, shape: crate::classes::rendering_server::DofBokehShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DofBokehShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur_bokeh_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the parameters to use with the DOF blur effect. These parameters take on the same meaning as their counterparts in [`CameraAttributesPractical`][crate::classes::CameraAttributesPractical]."]
        pub fn camera_attributes_set_dof_blur(&mut self, camera_attributes: Rid, far_enable: bool, far_distance: f32, far_transition: f32, near_enable: bool, near_distance: f32, near_transition: f32, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, bool, f32, f32, f32,);
            let args = (camera_attributes, far_enable, far_distance, far_transition, near_enable, near_distance, near_transition, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the exposure values that will be used by the renderers. The normalization amount is used to bake a given Exposure Value (EV) into rendering calculations to reduce the dynamic range of the scene.\n\nThe normalization factor can be calculated from exposure value (EV100) as follows:\n\n```gdscript\nfunc get_exposure_normalization(ev100: float):\n\treturn 1.0 / (pow(2.0, ev100) * 1.2)\n```\n\nThe exposure value can be calculated from aperture (in f-stops), shutter speed (in seconds), and sensitivity (in ISO) as follows:\n\n```gdscript\nfunc get_exposure(aperture: float, shutter_speed: float, sensitivity: float):\n\treturn log((aperture * aperture) / shutter_speed * (100.0 / sensitivity)) / log(2)\n```"]
        pub fn camera_attributes_set_exposure(&mut self, camera_attributes: Rid, multiplier: f32, normalization: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (camera_attributes, multiplier, normalization,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_exposure", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the parameters to use with the auto-exposure effect. These parameters take on the same meaning as their counterparts in [`CameraAttributes`][crate::classes::CameraAttributes] and [`CameraAttributesPractical`][crate::classes::CameraAttributesPractical]."]
        pub fn camera_attributes_set_auto_exposure(&mut self, camera_attributes: Rid, enable: bool, min_sensitivity: f32, max_sensitivity: f32, speed: f32, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, f32,);
            let args = (camera_attributes, enable, min_sensitivity, max_sensitivity, speed, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_auto_exposure", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a scenario and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `scenario_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nThe scenario is the 3D world that all the visual instances exist in."]
        pub fn scenario_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the environment that will be used with this scenario. See also [`Environment`][crate::classes::Environment]."]
        pub fn scenario_set_environment(&mut self, scenario: Rid, environment: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the fallback environment to be used by this scenario. The fallback environment is used if no environment is set. Internally, this is used by the editor to provide a default environment."]
        pub fn scenario_set_fallback_environment(&mut self, scenario: Rid, environment: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_fallback_environment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the camera attributes (`effects`) that will be used with this scenario. See also [`CameraAttributes`][crate::classes::CameraAttributes]."]
        pub fn scenario_set_camera_attributes(&mut self, scenario: Rid, effects: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the compositor (`compositor`) that will be used with this scenario. See also `Compositor`."]
        pub fn scenario_set_compositor(&mut self, scenario: Rid, compositor: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, compositor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_compositor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a visual instance, adds it to the RenderingServer, and sets both base and scenario. It can be accessed with the RID that is returned. This RID will be used in all `instance_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method. This is a shorthand for using [`instance_create`][`crate::classes::RenderingServer::instance_create`] and setting the base and scenario manually."]
        pub fn instance_create2(&mut self, base: Rid, scenario: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Rid,);
            let args = (base, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_create2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a visual instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `instance_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nAn instance is a way of placing a 3D object in the scenario. Objects like particles, meshes, reflection probes and decals need to be associated with an instance to be visible in the scenario using [`instance_set_base`][`crate::classes::RenderingServer::instance_set_base`].\n\n**Note:** The equivalent node is [`VisualInstance3D`][crate::classes::VisualInstance3D]."]
        pub fn instance_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the base of the instance. A base can be any of the 3D objects that are created in the RenderingServer that can be displayed. For example, any of the light types, mesh, multimesh, particle system, reflection probe, decal, lightmap, voxel GI and visibility notifiers are all types that can be set as the base of an instance in order to be displayed in the scenario."]
        pub fn instance_set_base(&mut self, instance: Rid, base: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_base", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the scenario that the instance is in. The scenario is the 3D world that the objects will be displayed in."]
        pub fn instance_set_scenario(&mut self, instance: Rid, scenario: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_scenario", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the render layers that this instance will be drawn to. Equivalent to \\[member VisualInstance3D.layers]."]
        pub fn instance_set_layer_mask(&mut self, instance: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (instance, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_layer_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the sorting offset and switches between using the bounding box or instance origin for depth sorting."]
        pub fn instance_set_pivot_data(&mut self, instance: Rid, sorting_offset: f32, use_aabb_center: bool,) {
            type CallRet = ();
            type CallParams = (Rid, f32, bool,);
            let args = (instance, sorting_offset, use_aabb_center,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_pivot_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the world space transform of the instance. Equivalent to \\[member Node3D.global_transform]."]
        pub fn instance_set_transform(&mut self, instance: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches a unique Object ID to instance. Object ID must be attached to instance for proper culling with [`instances_cull_aabb`][`crate::classes::RenderingServer::instances_cull_aabb`], [`instances_cull_convex`][`crate::classes::RenderingServer::instances_cull_convex`], and [`instances_cull_ray`][`crate::classes::RenderingServer::instances_cull_ray`]."]
        pub fn instance_attach_object_instance_id(&mut self, instance: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (instance, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the weight for a given blend shape associated with this instance."]
        pub fn instance_set_blend_shape_weight(&mut self, instance: Rid, shape: i32, weight: f32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, f32,);
            let args = (instance, shape, weight,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_blend_shape_weight", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the override material of a specific surface. Equivalent to [`set_surface_override_material`][`crate::classes::MeshInstance3D::set_surface_override_material`]."]
        pub fn instance_set_surface_override_material(&mut self, instance: Rid, surface: i32, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (instance, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_surface_override_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets whether an instance is drawn or not. Equivalent to \\[member Node3D.visible]."]
        pub fn instance_set_visible(&mut self, instance: Rid, visible: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (instance, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the transparency for the given geometry instance. Equivalent to \\[member GeometryInstance3D.transparency].\n\nA transparency of `0.0` is fully opaque, while `1.0` is fully transparent. Values greater than `0.0` (exclusive) will force the geometry's materials to go through the transparent pipeline, which is slower to render and can exhibit rendering issues due to incorrect transparency sorting. However, unlike using a transparent material, setting `transparency` to a value greater than `0.0` (exclusive) will _not_ disable shadow rendering.\n\nIn spatial shaders, `1.0 - transparency` is set as the default value of the `ALPHA` built-in.\n\n**Note:** `transparency` is clamped between `0.0` and `1.0`, so this property cannot be used to make transparent materials more opaque than they originally are."]
        pub fn instance_geometry_set_transparency(&mut self, instance: Rid, transparency: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, transparency,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_transparency", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resets motion vectors and other interpolated values. Use this _after_ teleporting a mesh from one position to another to avoid ghosting artifacts."]
        pub fn instance_teleport(&mut self, instance: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_teleport", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a custom AABB to use when culling objects from the view frustum. Equivalent to setting \\[member GeometryInstance3D.custom_aabb]."]
        pub fn instance_set_custom_aabb(&mut self, instance: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (instance, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches a skeleton to an instance. Removes the previous skeleton from the instance."]
        pub fn instance_attach_skeleton(&mut self, instance: Rid, skeleton: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_attach_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a margin to increase the size of the AABB when culling objects from the view frustum. This allows you to avoid culling objects that fall outside the view frustum. Equivalent to \\[member GeometryInstance3D.extra_cull_margin]."]
        pub fn instance_set_extra_visibility_margin(&mut self, instance: Rid, margin: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_extra_visibility_margin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the visibility parent for the given instance. Equivalent to \\[member Node3D.visibility_parent]."]
        pub fn instance_set_visibility_parent(&mut self, instance: Rid, parent: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_visibility_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, ignores both frustum and occlusion culling on the specified 3D geometry instance. This is not the same as \\[member GeometryInstance3D.ignore_occlusion_culling], which only ignores occlusion culling and leaves frustum culling intact."]
        pub fn instance_set_ignore_culling(&mut self, instance: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (instance, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_ignore_culling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `flag` for a given `instance` to `enabled`."]
        pub fn instance_geometry_set_flag(&mut self, instance: Rid, flag: crate::classes::rendering_server::InstanceFlags, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::InstanceFlags, bool,);
            let args = (instance, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shadow casting setting. Equivalent to \\[member GeometryInstance3D.cast_shadow]."]
        pub fn instance_geometry_set_cast_shadows_setting(&mut self, instance: Rid, shadow_casting_setting: crate::classes::rendering_server::ShadowCastingSetting,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ShadowCastingSetting,);
            let args = (instance, shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a material that will override the material for all surfaces on the mesh associated with this instance. Equivalent to \\[member GeometryInstance3D.material_override]."]
        pub fn instance_geometry_set_material_override(&mut self, instance: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_material_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a material that will be rendered for all surfaces on top of active materials for the mesh associated with this instance. Equivalent to \\[member GeometryInstance3D.material_overlay]."]
        pub fn instance_geometry_set_material_overlay(&mut self, instance: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the visibility range values for the given geometry instance. Equivalent to \\[member GeometryInstance3D.visibility_range_begin] and related properties."]
        pub fn instance_geometry_set_visibility_range(&mut self, instance: Rid, min: f32, max: f32, min_margin: f32, max_margin: f32, fade_mode: crate::classes::rendering_server::VisibilityRangeFadeMode,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32, f32, crate::classes::rendering_server::VisibilityRangeFadeMode,);
            let args = (instance, min, max, min_margin, max_margin, fade_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_visibility_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the lightmap GI instance to use for the specified 3D geometry instance. The lightmap UV scale for the specified instance (equivalent to \\[member GeometryInstance3D.gi_lightmap_scale]) and lightmap atlas slice must also be specified."]
        pub fn instance_geometry_set_lightmap(&mut self, instance: Rid, lightmap: Rid, lightmap_uv_scale: Rect2, lightmap_slice: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rect2, i32,);
            let args = (instance, lightmap, lightmap_uv_scale, lightmap_slice,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_lightmap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the level of detail bias to use when rendering the specified 3D geometry instance. Higher values result in higher detail from further away. Equivalent to \\[member GeometryInstance3D.lod_bias]."]
        pub fn instance_geometry_set_lod_bias(&mut self, instance: Rid, lod_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, lod_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the per-instance shader uniform on the specified 3D geometry instance. Equivalent to [`set_instance_shader_parameter`][`crate::classes::GeometryInstance3D::set_instance_shader_parameter`]."]
        pub fn instance_geometry_set_shader_parameter(&mut self, instance: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (instance, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the per-instance shader uniform from the specified 3D geometry instance. Equivalent to [`get_instance_shader_parameter`][`crate::classes::GeometryInstance3D::get_instance_shader_parameter`].\n\n**Note:** Per-instance shader parameter names are case-sensitive."]
        pub fn instance_geometry_get_shader_parameter(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default value of the per-instance shader uniform from the specified 3D geometry instance. Equivalent to [`get_instance_shader_parameter`][`crate::classes::GeometryInstance3D::get_instance_shader_parameter`]."]
        pub fn instance_geometry_get_shader_parameter_default_value(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter_default_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a dictionary of per-instance shader uniform names of the per-instance shader uniform from the specified 3D geometry instance. The returned dictionary is in PropertyInfo format, with the keys `name`, `class_name`, `type`, `hint`, `hint_string` and `usage`. Equivalent to [`get_instance_shader_parameter`][`crate::classes::GeometryInstance3D::get_instance_shader_parameter`]."]
        pub fn instance_geometry_get_shader_parameter_list(&self, instance: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array of object IDs intersecting with the provided AABB. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        pub(crate) fn instances_cull_aabb_full(&self, aabb: Aabb, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (Aabb, Rid,);
            let args = (aabb, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`instances_cull_aabb_ex`][Self::instances_cull_aabb_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of object IDs intersecting with the provided AABB. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_aabb(&self, aabb: Aabb,) -> PackedInt64Array {
            self.instances_cull_aabb_ex(aabb,) . done()
        }
        #[doc = "Returns an array of object IDs intersecting with the provided AABB. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_aabb_ex < 'ex > (&'ex self, aabb: Aabb,) -> ExInstancesCullAabb < 'ex > {
            ExInstancesCullAabb::new(self, aabb,)
        }
        #[doc = "Returns an array of object IDs intersecting with the provided 3D ray. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        pub(crate) fn instances_cull_ray_full(&self, from: Vector3, to: Vector3, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (Vector3, Vector3, Rid,);
            let args = (from, to, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_ray", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`instances_cull_ray_ex`][Self::instances_cull_ray_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of object IDs intersecting with the provided 3D ray. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_ray(&self, from: Vector3, to: Vector3,) -> PackedInt64Array {
            self.instances_cull_ray_ex(from, to,) . done()
        }
        #[doc = "Returns an array of object IDs intersecting with the provided 3D ray. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_ray_ex < 'ex > (&'ex self, from: Vector3, to: Vector3,) -> ExInstancesCullRay < 'ex > {
            ExInstancesCullRay::new(self, from, to,)
        }
        #[doc = "Returns an array of object IDs intersecting with the provided convex shape. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        pub(crate) fn instances_cull_convex_full(&self, convex: RefArg < Array < Plane > >, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Plane > >, Rid,);
            let args = (convex, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_convex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`instances_cull_convex_ex`][Self::instances_cull_convex_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns an array of object IDs intersecting with the provided convex shape. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_convex(&self, convex: &Array < Plane >,) -> PackedInt64Array {
            self.instances_cull_convex_ex(convex,) . done()
        }
        #[doc = "Returns an array of object IDs intersecting with the provided convex shape. Only 3D nodes that inherit from [`VisualInstance3D`][crate::classes::VisualInstance3D] are considered, such as [`MeshInstance3D`][crate::classes::MeshInstance3D] or [`DirectionalLight3D`][crate::classes::DirectionalLight3D]. Use [`from_instance_id`][`crate::obj::Gd::from_instance_id`] to obtain the actual nodes. A scenario RID must be provided, which is available in the [`World3D`][crate::classes::World3D] you want to query. This forces an update for all resources queued to update.\n\n**Warning:** This function is primarily intended for editor usage. For in-game use cases, prefer physics collision."]
        #[inline]
        pub fn instances_cull_convex_ex < 'ex > (&'ex self, convex: &'ex Array < Plane >,) -> ExInstancesCullConvex < 'ex > {
            ExInstancesCullConvex::new(self, convex,)
        }
        #[doc = "Bakes the material data of the Mesh passed in the `base` parameter with optional `material_overrides` to a set of [`Image`][crate::classes::Image]s of size `image_size`. Returns an array of [`Image`][crate::classes::Image]s containing material properties as specified in \\[enum BakeChannels]."]
        pub fn bake_render_uv2(&mut self, base: Rid, material_overrides: &Array < Rid >, image_size: Vector2i,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Rid > >, Vector2i,);
            let args = (base, RefArg::new(material_overrides), image_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "bake_render_uv2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a canvas and returns the assigned [`RID`][crate::builtin::Rid]. It can be accessed with the RID that is returned. This RID will be used in all `canvas_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\nCanvas has no [`Resource`][crate::classes::Resource] or [`Node`][crate::classes::Node] equivalent."]
        pub fn canvas_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "A copy of the canvas item will be drawn with a local offset of the `mirroring`.\n\n**Note:** This is equivalent to calling [`canvas_set_item_repeat`][`crate::classes::RenderingServer::canvas_set_item_repeat`] like `canvas_set_item_repeat(item, mirroring, 1)`, with an additional check ensuring `canvas` is a parent of `item`."]
        pub fn canvas_set_item_mirroring(&mut self, canvas: Rid, item: Rid, mirroring: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2,);
            let args = (canvas, item, mirroring,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_item_mirroring", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "A copy of the canvas item will be drawn with a local offset of the `repeat_size` by the number of times of the `repeat_times`. As the `repeat_times` increases, the copies will spread away from the origin texture."]
        pub fn canvas_set_item_repeat(&mut self, item: Rid, repeat_size: Vector2, repeat_times: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32,);
            let args = (item, repeat_size, repeat_times,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_item_repeat", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Modulates all colors in the given canvas."]
        pub fn canvas_set_modulate(&mut self, canvas: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (canvas, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_disable_scale(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_disable_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a canvas texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `canvas_texture_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method. See also [`texture_2d_create`][`crate::classes::RenderingServer::texture_2d_create`].\n\n**Note:** The equivalent resource is [`CanvasTexture`][crate::classes::CanvasTexture] and is only meant to be used in 2D rendering, not 3D."]
        pub fn canvas_texture_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `channel`'s `texture` for the canvas texture specified by the `canvas_texture` RID. Equivalent to \\[member CanvasTexture.diffuse_texture], \\[member CanvasTexture.normal_texture] and \\[member CanvasTexture.specular_texture]."]
        pub fn canvas_texture_set_channel(&mut self, canvas_texture: Rid, channel: crate::classes::rendering_server::CanvasTextureChannel, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasTextureChannel, Rid,);
            let args = (canvas_texture, channel, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_channel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `base_color` and `shininess` to use for the canvas texture specified by the `canvas_texture` RID. Equivalent to \\[member CanvasTexture.specular_color] and \\[member CanvasTexture.specular_shininess]."]
        pub fn canvas_texture_set_shading_parameters(&mut self, canvas_texture: Rid, base_color: Color, shininess: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Color, f32,);
            let args = (canvas_texture, base_color, shininess,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_shading_parameters", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the texture `filter` mode to use for the canvas texture specified by the `canvas_texture` RID."]
        pub fn canvas_texture_set_texture_filter(&mut self, canvas_texture: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (canvas_texture, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the texture `repeat` mode to use for the canvas texture specified by the `canvas_texture` RID."]
        pub fn canvas_texture_set_texture_repeat(&mut self, canvas_texture: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (canvas_texture, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new CanvasItem instance and returns its [`RID`][crate::builtin::Rid]. It can be accessed with the RID that is returned. This RID will be used in all `canvas_item_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`CanvasItem`][crate::classes::CanvasItem]."]
        pub fn canvas_item_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a parent [`CanvasItem`][crate::classes::CanvasItem] to the [`CanvasItem`][crate::classes::CanvasItem]. The item will inherit transform, modulation and visibility from its parent, like [`CanvasItem`][crate::classes::CanvasItem] nodes in the scene tree."]
        pub fn canvas_item_set_parent(&mut self, item: Rid, parent: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default texture filter mode for the canvas item specified by the `item` RID. Equivalent to \\[member CanvasItem.texture_filter]."]
        pub fn canvas_item_set_default_texture_filter(&mut self, item: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (item, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_default_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default texture repeat mode for the canvas item specified by the `item` RID. Equivalent to \\[member CanvasItem.texture_repeat]."]
        pub fn canvas_item_set_default_texture_repeat(&mut self, item: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (item, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_default_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the visibility of the [`CanvasItem`][crate::classes::CanvasItem]."]
        pub fn canvas_item_set_visible(&mut self, item: Rid, visible: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the light `mask` for the canvas item specified by the `item` RID. Equivalent to \\[member CanvasItem.light_mask]."]
        pub fn canvas_item_set_light_mask(&mut self, item: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the rendering visibility layer associated with this [`CanvasItem`][crate::classes::CanvasItem]. Only [`Viewport`][crate::classes::Viewport] nodes with a matching rendering mask will render this [`CanvasItem`][crate::classes::CanvasItem]."]
        pub fn canvas_item_set_visibility_layer(&mut self, item: Rid, visibility_layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (item, visibility_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the `transform` of the canvas item specified by the `item` RID. This affects where and how the item will be drawn. Child canvas items' transforms are multiplied by their parent's transform. Equivalent to \\[member Node2D.transform]."]
        pub fn canvas_item_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `clip` is `true`, makes the canvas item specified by the `item` RID not draw anything outside of its rect's coordinates. This clipping is fast, but works only with axis-aligned rectangles. This means that rotation is ignored by the clipping rectangle. For more advanced clipping shapes, use [`canvas_item_set_canvas_group_mode`][`crate::classes::RenderingServer::canvas_item_set_canvas_group_mode`] instead.\n\n**Note:** The equivalent node functionality is found in \\[member Label.clip_text], [`RichTextLabel`][crate::classes::RichTextLabel] (always enabled) and more."]
        pub fn canvas_item_set_clip(&mut self, item: Rid, clip: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, clip,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_clip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, enables multichannel signed distance field rendering mode for the canvas item specified by the `item` RID. This is meant to be used for font rendering, or with specially generated images using [msdfgen](https://github.com/Chlumsky/msdfgen)."]
        pub fn canvas_item_set_distance_field_mode(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_distance_field_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `use_custom_rect` is `true`, sets the custom visibility rectangle (used for culling) to `rect` for the canvas item specified by `item`. Setting a custom visibility rect can reduce CPU load when drawing lots of 2D instances. If `use_custom_rect` is `false`, automatically computes a visibility rectangle based on the canvas item's draw commands."]
        pub(crate) fn canvas_item_set_custom_rect_full(&mut self, item: Rid, use_custom_rect: bool, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Rect2,);
            let args = (item, use_custom_rect, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_custom_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_set_custom_rect_ex`][Self::canvas_item_set_custom_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If `use_custom_rect` is `true`, sets the custom visibility rectangle (used for culling) to `rect` for the canvas item specified by `item`. Setting a custom visibility rect can reduce CPU load when drawing lots of 2D instances. If `use_custom_rect` is `false`, automatically computes a visibility rectangle based on the canvas item's draw commands."]
        #[inline]
        pub fn canvas_item_set_custom_rect(&mut self, item: Rid, use_custom_rect: bool,) {
            self.canvas_item_set_custom_rect_ex(item, use_custom_rect,) . done()
        }
        #[doc = "If `use_custom_rect` is `true`, sets the custom visibility rectangle (used for culling) to `rect` for the canvas item specified by `item`. Setting a custom visibility rect can reduce CPU load when drawing lots of 2D instances. If `use_custom_rect` is `false`, automatically computes a visibility rectangle based on the canvas item's draw commands."]
        #[inline]
        pub fn canvas_item_set_custom_rect_ex < 'ex > (&'ex mut self, item: Rid, use_custom_rect: bool,) -> ExCanvasItemSetCustomRect < 'ex > {
            ExCanvasItemSetCustomRect::new(self, item, use_custom_rect,)
        }
        #[doc = "Multiplies the color of the canvas item specified by the `item` RID, while affecting its children. See also [`canvas_item_set_self_modulate`][`crate::classes::RenderingServer::canvas_item_set_self_modulate`]. Equivalent to \\[member CanvasItem.modulate]."]
        pub fn canvas_item_set_modulate(&mut self, item: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Multiplies the color of the canvas item specified by the `item` RID, without affecting its children. See also [`canvas_item_set_modulate`][`crate::classes::RenderingServer::canvas_item_set_modulate`]. Equivalent to \\[member CanvasItem.self_modulate]."]
        pub fn canvas_item_set_self_modulate(&mut self, item: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, draws the canvas item specified by the `item` RID behind its parent. Equivalent to \\[member CanvasItem.show_behind_parent]."]
        pub fn canvas_item_set_draw_behind_parent(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_draw_behind_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `interpolated` is `true`, turns on physics interpolation for the canvas item."]
        pub fn canvas_item_set_interpolated(&mut self, item: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents physics interpolation for the current physics tick.\n\nThis is useful when moving a canvas item to a new location, to give an instantaneous change rather than interpolation from the previous location."]
        pub fn canvas_item_reset_physics_interpolation(&mut self, item: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms both the current and previous stored transform for a canvas item.\n\nThis allows transforming a canvas item without creating a \"glitch\" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin."]
        pub fn canvas_item_transform_physics_interpolation(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws a line on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`]."]
        pub(crate) fn canvas_item_add_line_full(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2, Color, f32, bool,);
            let args = (item, from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_line_ex`][Self::canvas_item_add_line_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a line on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`]."]
        #[inline]
        pub fn canvas_item_add_line(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) {
            self.canvas_item_add_line_ex(item, from, to, color,) . done()
        }
        #[doc = "Draws a line on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_line`][`crate::classes::CanvasItem::draw_line`]."]
        #[inline]
        pub fn canvas_item_add_line_ex < 'ex > (&'ex mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) -> ExCanvasItemAddLine < 'ex > {
            ExCanvasItemAddLine::new(self, item, from, to, color,)
        }
        #[doc = "Draws a 2D polyline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]."]
        pub(crate) fn canvas_item_add_polyline_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (item, points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_polyline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_polyline_ex`][Self::canvas_item_add_polyline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a 2D polyline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]."]
        #[inline]
        pub fn canvas_item_add_polyline(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_polyline_ex(item, points, colors,) . done()
        }
        #[doc = "Draws a 2D polyline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_polyline`][`crate::classes::CanvasItem::draw_polyline`] and [`draw_polyline_colors`][`crate::classes::CanvasItem::draw_polyline_colors`]."]
        #[inline]
        pub fn canvas_item_add_polyline_ex < 'ex > (&'ex mut self, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExCanvasItemAddPolyline < 'ex > {
            ExCanvasItemAddPolyline::new(self, item, points, colors,)
        }
        #[doc = "Draws a 2D multiline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] and [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`]."]
        pub(crate) fn canvas_item_add_multiline_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (item, points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_multiline_ex`][Self::canvas_item_add_multiline_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a 2D multiline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] and [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`]."]
        #[inline]
        pub fn canvas_item_add_multiline(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_multiline_ex(item, points, colors,) . done()
        }
        #[doc = "Draws a 2D multiline on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multiline`][`crate::classes::CanvasItem::draw_multiline`] and [`draw_multiline_colors`][`crate::classes::CanvasItem::draw_multiline_colors`]."]
        #[inline]
        pub fn canvas_item_add_multiline_ex < 'ex > (&'ex mut self, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExCanvasItemAddMultiline < 'ex > {
            ExCanvasItemAddMultiline::new(self, item, points, colors,)
        }
        #[doc = "Draws a rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`]."]
        pub(crate) fn canvas_item_add_rect_full(&mut self, item: Rid, rect: Rect2, color: Color, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Color, bool,);
            let args = (item, rect, color, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_rect_ex`][Self::canvas_item_add_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`]."]
        #[inline]
        pub fn canvas_item_add_rect(&mut self, item: Rid, rect: Rect2, color: Color,) {
            self.canvas_item_add_rect_ex(item, rect, color,) . done()
        }
        #[doc = "Draws a rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_rect`][`crate::classes::CanvasItem::draw_rect`]."]
        #[inline]
        pub fn canvas_item_add_rect_ex < 'ex > (&'ex mut self, item: Rid, rect: Rect2, color: Color,) -> ExCanvasItemAddRect < 'ex > {
            ExCanvasItemAddRect::new(self, item, rect, color,)
        }
        #[doc = "Draws a circle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`]."]
        pub(crate) fn canvas_item_add_circle_full(&mut self, item: Rid, pos: Vector2, radius: f32, color: Color, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, f32, Color, bool,);
            let args = (item, pos, radius, color, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_circle_ex`][Self::canvas_item_add_circle_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a circle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`]."]
        #[inline]
        pub fn canvas_item_add_circle(&mut self, item: Rid, pos: Vector2, radius: f32, color: Color,) {
            self.canvas_item_add_circle_ex(item, pos, radius, color,) . done()
        }
        #[doc = "Draws a circle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_circle`][`crate::classes::CanvasItem::draw_circle`]."]
        #[inline]
        pub fn canvas_item_add_circle_ex < 'ex > (&'ex mut self, item: Rid, pos: Vector2, radius: f32, color: Color,) -> ExCanvasItemAddCircle < 'ex > {
            ExCanvasItemAddCircle::new(self, item, pos, radius, color,)
        }
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor` on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`]."]
        pub(crate) fn canvas_item_add_ellipse_full(&mut self, item: Rid, pos: Vector2, major: f32, minor: f32, color: Color, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, f32, f32, Color, bool,);
            let args = (item, pos, major, minor, color, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_ellipse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_ellipse_ex`][Self::canvas_item_add_ellipse_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor` on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`]."]
        #[inline]
        pub fn canvas_item_add_ellipse(&mut self, item: Rid, pos: Vector2, major: f32, minor: f32, color: Color,) {
            self.canvas_item_add_ellipse_ex(item, pos, major, minor, color,) . done()
        }
        #[doc = "Draws an ellipse with semi-major axis `major` and semi-minor axis `minor` on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_ellipse`][`crate::classes::CanvasItem::draw_ellipse`]."]
        #[inline]
        pub fn canvas_item_add_ellipse_ex < 'ex > (&'ex mut self, item: Rid, pos: Vector2, major: f32, minor: f32, color: Color,) -> ExCanvasItemAddEllipse < 'ex > {
            ExCanvasItemAddEllipse::new(self, item, pos, major, minor, color,)
        }
        #[doc = "Draws a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`] and [`draw_rect`][`crate::classes::Texture2D::draw_rect`]."]
        pub(crate) fn canvas_item_add_texture_rect_full(&mut self, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, bool, Color, bool,);
            let args = (item, rect, texture, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_texture_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_texture_rect_ex`][Self::canvas_item_add_texture_rect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`] and [`draw_rect`][`crate::classes::Texture2D::draw_rect`]."]
        #[inline]
        pub fn canvas_item_add_texture_rect(&mut self, item: Rid, rect: Rect2, texture: Rid,) {
            self.canvas_item_add_texture_rect_ex(item, rect, texture,) . done()
        }
        #[doc = "Draws a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect`][`crate::classes::CanvasItem::draw_texture_rect`] and [`draw_rect`][`crate::classes::Texture2D::draw_rect`]."]
        #[inline]
        pub fn canvas_item_add_texture_rect_ex < 'ex > (&'ex mut self, item: Rid, rect: Rect2, texture: Rid,) -> ExCanvasItemAddTextureRect < 'ex > {
            ExCanvasItemAddTextureRect::new(self, item, rect, texture,)
        }
        #[doc = "See also [`draw_msdf_texture_rect_region`][`crate::classes::CanvasItem::draw_msdf_texture_rect_region`]."]
        pub(crate) fn canvas_item_add_msdf_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color, i32, f32, f32,);
            let args = (item, rect, texture, src_rect, modulate, outline_size, px_range, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_msdf_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_msdf_texture_rect_region_ex`][Self::canvas_item_add_msdf_texture_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "See also [`draw_msdf_texture_rect_region`][`crate::classes::CanvasItem::draw_msdf_texture_rect_region`]."]
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_msdf_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[doc = "See also [`draw_msdf_texture_rect_region`][`crate::classes::CanvasItem::draw_msdf_texture_rect_region`]."]
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region_ex < 'ex > (&'ex mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddMsdfTextureRectRegion < 'ex > {
            ExCanvasItemAddMsdfTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        #[doc = "See also [`draw_lcd_texture_rect_region`][`crate::classes::CanvasItem::draw_lcd_texture_rect_region`]."]
        pub fn canvas_item_add_lcd_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color,);
            let args = (item, rect, texture, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_lcd_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws the specified region of a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`] and [`draw_rect_region`][`crate::classes::Texture2D::draw_rect_region`]."]
        pub(crate) fn canvas_item_add_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color, bool, bool,);
            let args = (item, rect, texture, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_texture_rect_region_ex`][Self::canvas_item_add_texture_rect_region_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws the specified region of a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`] and [`draw_rect_region`][`crate::classes::Texture2D::draw_rect_region`]."]
        #[inline]
        pub fn canvas_item_add_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[doc = "Draws the specified region of a 2D textured rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_texture_rect_region`][`crate::classes::CanvasItem::draw_texture_rect_region`] and [`draw_rect_region`][`crate::classes::Texture2D::draw_rect_region`]."]
        #[inline]
        pub fn canvas_item_add_texture_rect_region_ex < 'ex > (&'ex mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddTextureRectRegion < 'ex > {
            ExCanvasItemAddTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        #[doc = "Draws a nine-patch rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]."]
        pub(crate) fn canvas_item_add_nine_patch_full(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rect2, Rid, Vector2, Vector2, crate::classes::rendering_server::NinePatchAxisMode, crate::classes::rendering_server::NinePatchAxisMode, bool, Color,);
            let args = (item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_nine_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_nine_patch_ex`][Self::canvas_item_add_nine_patch_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a nine-patch rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]."]
        #[inline]
        pub fn canvas_item_add_nine_patch(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) {
            self.canvas_item_add_nine_patch_ex(item, rect, source, texture, topleft, bottomright,) . done()
        }
        #[doc = "Draws a nine-patch rectangle on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]."]
        #[inline]
        pub fn canvas_item_add_nine_patch_ex < 'ex > (&'ex mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> ExCanvasItemAddNinePatch < 'ex > {
            ExCanvasItemAddNinePatch::new(self, item, rect, source, texture, topleft, bottomright,)
        }
        #[doc = "Draws a 2D primitive on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_primitive`][`crate::classes::CanvasItem::draw_primitive`]."]
        pub fn canvas_item_add_primitive(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray, uvs: &PackedVector2Array, texture: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, Rid,);
            let args = (item, RefArg::new(points), RefArg::new(colors), RefArg::new(uvs), texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_primitive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Draws a 2D polygon on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`]."]
        pub(crate) fn canvas_item_add_polygon_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, Rid,);
            let args = (item, points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_polygon_ex`][Self::canvas_item_add_polygon_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a 2D polygon on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`]."]
        #[inline]
        pub fn canvas_item_add_polygon(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_polygon_ex(item, points, colors,) . done()
        }
        #[doc = "Draws a 2D polygon on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. If you need more flexibility (such as being able to use bones), use [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] instead. See also [`draw_polygon`][`crate::classes::CanvasItem::draw_polygon`].\n\n**Note:** If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with [`triangulate_polygon`][`crate::classes::Geometry2D::triangulate_polygon`] and using [`draw_mesh`][`crate::classes::CanvasItem::draw_mesh`], [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`], or [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`]."]
        #[inline]
        pub fn canvas_item_add_polygon_ex < 'ex > (&'ex mut self, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExCanvasItemAddPolygon < 'ex > {
            ExCanvasItemAddPolygon::new(self, item, points, colors,)
        }
        #[doc = "Draws a triangle array on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. This is internally used by [`Line2D`][crate::classes::Line2D] and [`StyleBoxFlat`][crate::classes::StyleBoxFlat] for rendering. [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] is highly flexible, but more complex to use than [`canvas_item_add_polygon`][`crate::classes::RenderingServer::canvas_item_add_polygon`].\n\n**Note:** If `count` is set to a non-negative value, only the first `count * 3` indices (corresponding to `count` triangles) will be drawn. Otherwise, all indices are drawn."]
        pub(crate) fn canvas_item_add_triangle_array_full(&mut self, item: Rid, indices: RefArg < PackedInt32Array >, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, bones: RefArg < PackedInt32Array >, weights: RefArg < PackedFloat32Array >, texture: Rid, count: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (Rid, RefArg < 'a0, PackedInt32Array >, RefArg < 'a1, PackedVector2Array >, RefArg < 'a2, PackedColorArray >, RefArg < 'a3, PackedVector2Array >, RefArg < 'a4, PackedInt32Array >, RefArg < 'a5, PackedFloat32Array >, Rid, i32,);
            let args = (item, indices, points, colors, uvs, bones, weights, texture, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_triangle_array", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_triangle_array_ex`][Self::canvas_item_add_triangle_array_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a triangle array on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. This is internally used by [`Line2D`][crate::classes::Line2D] and [`StyleBoxFlat`][crate::classes::StyleBoxFlat] for rendering. [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] is highly flexible, but more complex to use than [`canvas_item_add_polygon`][`crate::classes::RenderingServer::canvas_item_add_polygon`].\n\n**Note:** If `count` is set to a non-negative value, only the first `count * 3` indices (corresponding to `count` triangles) will be drawn. Otherwise, all indices are drawn."]
        #[inline]
        pub fn canvas_item_add_triangle_array(&mut self, item: Rid, indices: &PackedInt32Array, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_triangle_array_ex(item, indices, points, colors,) . done()
        }
        #[doc = "Draws a triangle array on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. This is internally used by [`Line2D`][crate::classes::Line2D] and [`StyleBoxFlat`][crate::classes::StyleBoxFlat] for rendering. [`canvas_item_add_triangle_array`][`crate::classes::RenderingServer::canvas_item_add_triangle_array`] is highly flexible, but more complex to use than [`canvas_item_add_polygon`][`crate::classes::RenderingServer::canvas_item_add_polygon`].\n\n**Note:** If `count` is set to a non-negative value, only the first `count * 3` indices (corresponding to `count` triangles) will be drawn. Otherwise, all indices are drawn."]
        #[inline]
        pub fn canvas_item_add_triangle_array_ex < 'ex > (&'ex mut self, item: Rid, indices: &'ex PackedInt32Array, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> ExCanvasItemAddTriangleArray < 'ex > {
            ExCanvasItemAddTriangleArray::new(self, item, indices, points, colors,)
        }
        #[doc = "Draws a mesh created with [`mesh_create`][`crate::classes::RenderingServer::mesh_create`] with given `transform`, `modulate` color, and `texture`. This is used internally by [`MeshInstance2D`][crate::classes::MeshInstance2D]."]
        pub(crate) fn canvas_item_add_mesh_full(&mut self, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D, Color, Rid,);
            let args = (item, mesh, transform, modulate, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_mesh_ex`][Self::canvas_item_add_mesh_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a mesh created with [`mesh_create`][`crate::classes::RenderingServer::mesh_create`] with given `transform`, `modulate` color, and `texture`. This is used internally by [`MeshInstance2D`][crate::classes::MeshInstance2D]."]
        #[inline]
        pub fn canvas_item_add_mesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_mesh_ex(item, mesh,) . done()
        }
        #[doc = "Draws a mesh created with [`mesh_create`][`crate::classes::RenderingServer::mesh_create`] with given `transform`, `modulate` color, and `texture`. This is used internally by [`MeshInstance2D`][crate::classes::MeshInstance2D]."]
        #[inline]
        pub fn canvas_item_add_mesh_ex < 'ex > (&'ex mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMesh < 'ex > {
            ExCanvasItemAddMesh::new(self, item, mesh,)
        }
        #[doc = "Draws a 2D [`MultiMesh`][crate::classes::MultiMesh] on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`]."]
        pub(crate) fn canvas_item_add_multimesh_full(&mut self, item: Rid, mesh: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rid,);
            let args = (item, mesh, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_multimesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_multimesh_ex`][Self::canvas_item_add_multimesh_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Draws a 2D [`MultiMesh`][crate::classes::MultiMesh] on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`]."]
        #[inline]
        pub fn canvas_item_add_multimesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_multimesh_ex(item, mesh,) . done()
        }
        #[doc = "Draws a 2D [`MultiMesh`][crate::classes::MultiMesh] on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]. See also [`draw_multimesh`][`crate::classes::CanvasItem::draw_multimesh`]."]
        #[inline]
        pub fn canvas_item_add_multimesh_ex < 'ex > (&'ex mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMultimesh < 'ex > {
            ExCanvasItemAddMultimesh::new(self, item, mesh,)
        }
        #[doc = "Draws particles on the [`CanvasItem`][crate::classes::CanvasItem] pointed to by the `item` [`RID`][crate::builtin::Rid]."]
        pub fn canvas_item_add_particles(&mut self, item: Rid, particles: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rid,);
            let args = (item, particles, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_particles", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a [`Transform2D`][crate::builtin::Transform2D] that will be used to transform subsequent canvas item commands."]
        pub fn canvas_item_add_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `ignore` is `true`, ignore clipping on items drawn with this canvas item until this is called again with `ignore` set to `false`."]
        pub fn canvas_item_add_clip_ignore(&mut self, item: Rid, ignore: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, ignore,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_clip_ignore", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        pub(crate) fn canvas_item_add_animation_slice_full(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64, f64, f64,);
            let args = (item, animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_animation_slice", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_add_animation_slice_ex`][Self::canvas_item_add_animation_slice_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        #[inline]
        pub fn canvas_item_add_animation_slice(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.canvas_item_add_animation_slice_ex(item, animation_length, slice_begin, slice_end,) . done()
        }
        #[doc = "Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly."]
        #[inline]
        pub fn canvas_item_add_animation_slice_ex < 'ex > (&'ex mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExCanvasItemAddAnimationSlice < 'ex > {
            ExCanvasItemAddAnimationSlice::new(self, item, animation_length, slice_begin, slice_end,)
        }
        #[doc = "If `enabled` is `true`, child nodes with the lowest Y position are drawn before those with a higher Y position. Y-sorting only affects children that inherit from the canvas item specified by the `item` RID, not the canvas item itself. Equivalent to \\[member CanvasItem.y_sort_enabled]."]
        pub fn canvas_item_set_sort_children_by_y(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_sort_children_by_y", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`CanvasItem`][crate::classes::CanvasItem]'s Z index, i.e. its draw order (lower indexes are drawn first)."]
        pub fn canvas_item_set_z_index(&mut self, item: Rid, z_index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, z_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_z_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If this is enabled, the Z index of the parent will be added to the children's Z index."]
        pub fn canvas_item_set_z_as_relative_to_parent(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_z_as_relative_to_parent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the [`CanvasItem`][crate::classes::CanvasItem] to copy a rect to the backbuffer."]
        pub fn canvas_item_set_copy_to_backbuffer(&mut self, item: Rid, enabled: bool, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Rect2,);
            let args = (item, enabled, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_copy_to_backbuffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches a skeleton to the [`CanvasItem`][crate::classes::CanvasItem]. Removes the previous skeleton."]
        pub fn canvas_item_attach_skeleton(&mut self, item: Rid, skeleton: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_attach_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the [`CanvasItem`][crate::classes::CanvasItem] and removes all commands in it."]
        pub fn canvas_item_clear(&mut self, item: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the index for the [`CanvasItem`][crate::classes::CanvasItem]."]
        pub fn canvas_item_set_draw_index(&mut self, item: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_draw_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a new `material` to the canvas item specified by the `item` RID. Equivalent to \\[member CanvasItem.material]."]
        pub fn canvas_item_set_material(&mut self, item: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets if the [`CanvasItem`][crate::classes::CanvasItem] uses its parent's material."]
        pub fn canvas_item_set_use_parent_material(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the per-instance shader uniform on the specified canvas item instance. Equivalent to [`set_instance_shader_parameter`][`crate::classes::CanvasItem::set_instance_shader_parameter`]."]
        pub fn canvas_item_set_instance_shader_parameter(&mut self, instance: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (instance, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the per-instance shader uniform from the specified canvas item instance. Equivalent to [`get_instance_shader_parameter`][`crate::classes::CanvasItem::get_instance_shader_parameter`]."]
        pub fn canvas_item_get_instance_shader_parameter(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default value of the per-instance shader uniform from the specified canvas item instance. Equivalent to [`get_instance_shader_parameter`][`crate::classes::CanvasItem::get_instance_shader_parameter`]."]
        pub fn canvas_item_get_instance_shader_parameter_default_value(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter_default_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a dictionary of per-instance shader uniform names of the per-instance shader uniform from the specified canvas item instance.\n\nThe returned dictionary is in PropertyInfo format, with the keys `name`, `class_name`, `type`, `hint`, `hint_string`, and `usage`."]
        pub fn canvas_item_get_instance_shader_parameter_list(&self, instance: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the given [`CanvasItem`][crate::classes::CanvasItem] as visibility notifier. `area` defines the area of detecting visibility. `enter_callable` is called when the [`CanvasItem`][crate::classes::CanvasItem] enters the screen, `exit_callable` is called when the [`CanvasItem`][crate::classes::CanvasItem] exits the screen. If `enable` is `false`, the item will no longer function as notifier.\n\nThis method can be used to manually mimic [`VisibleOnScreenNotifier2D`][crate::classes::VisibleOnScreenNotifier2D]."]
        pub fn canvas_item_set_visibility_notifier(&mut self, item: Rid, enable: bool, area: Rect2, enter_callable: &Callable, exit_callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, bool, Rect2, RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (item, enable, area, RefArg::new(enter_callable), RefArg::new(exit_callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visibility_notifier", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the canvas group mode used during 2D rendering for the canvas item specified by the `item` RID. For faster but more limited clipping, use [`canvas_item_set_clip`][`crate::classes::RenderingServer::canvas_item_set_clip`] instead.\n\n**Note:** The equivalent node functionality is found in [`CanvasGroup`][crate::classes::CanvasGroup] and \\[member CanvasItem.clip_children]."]
        pub(crate) fn canvas_item_set_canvas_group_mode_full(&mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasGroupMode, f32, bool, f32, bool,);
            let args = (item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_canvas_group_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`canvas_item_set_canvas_group_mode_ex`][Self::canvas_item_set_canvas_group_mode_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the canvas group mode used during 2D rendering for the canvas item specified by the `item` RID. For faster but more limited clipping, use [`canvas_item_set_clip`][`crate::classes::RenderingServer::canvas_item_set_clip`] instead.\n\n**Note:** The equivalent node functionality is found in [`CanvasGroup`][crate::classes::CanvasGroup] and \\[member CanvasItem.clip_children]."]
        #[inline]
        pub fn canvas_item_set_canvas_group_mode(&mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) {
            self.canvas_item_set_canvas_group_mode_ex(item, mode,) . done()
        }
        #[doc = "Sets the canvas group mode used during 2D rendering for the canvas item specified by the `item` RID. For faster but more limited clipping, use [`canvas_item_set_clip`][`crate::classes::RenderingServer::canvas_item_set_clip`] instead.\n\n**Note:** The equivalent node functionality is found in [`CanvasGroup`][crate::classes::CanvasGroup] and \\[member CanvasItem.clip_children]."]
        #[inline]
        pub fn canvas_item_set_canvas_group_mode_ex < 'ex > (&'ex mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) -> ExCanvasItemSetCanvasGroupMode < 'ex > {
            ExCanvasItemSetCanvasGroupMode::new(self, item, mode,)
        }
        #[doc = "Returns the bounding rectangle for a canvas item in local space, as calculated by the renderer. This bound is used internally for culling.\n\n**Warning:** This function is intended for debugging in the editor, and will pass through and return a zero [`Rect2`][crate::builtin::Rect2] in exported projects."]
        pub fn debug_canvas_item_get_rect(&mut self, item: Rid,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "debug_canvas_item_get_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a canvas light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `canvas_light_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`Light2D`][crate::classes::Light2D]."]
        pub fn canvas_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches the canvas light to the canvas. Removes it from its previous canvas."]
        pub fn canvas_light_attach_to_canvas(&mut self, light: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_attach_to_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables a canvas light."]
        pub fn canvas_light_set_enabled(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the scale factor of a [`PointLight2D`][crate::classes::PointLight2D]'s texture. Equivalent to \\[member PointLight2D.texture_scale]."]
        pub fn canvas_light_set_texture_scale(&mut self, light: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the canvas light's [`Transform2D`][crate::builtin::Transform2D]."]
        pub fn canvas_light_set_transform(&mut self, light: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (light, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the texture to be used by a [`PointLight2D`][crate::classes::PointLight2D]. Equivalent to \\[member PointLight2D.texture]."]
        pub fn canvas_light_set_texture(&mut self, light: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the offset of a [`PointLight2D`][crate::classes::PointLight2D]'s texture. Equivalent to \\[member PointLight2D.offset]."]
        pub fn canvas_light_set_texture_offset(&mut self, light: Rid, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (light, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture_offset", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color for a light."]
        pub fn canvas_light_set_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a canvas light's height."]
        pub fn canvas_light_set_height(&mut self, light: Rid, height: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a canvas light's energy."]
        pub fn canvas_light_set_energy(&mut self, light: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_energy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the Z range of objects that will be affected by this light. Equivalent to \\[member Light2D.range_z_min] and \\[member Light2D.range_z_max]."]
        pub fn canvas_light_set_z_range(&mut self, light: Rid, min_z: i32, max_z: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (light, min_z, max_z,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_z_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The layer range that gets rendered with this light."]
        pub fn canvas_light_set_layer_range(&mut self, light: Rid, min_layer: i32, max_layer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (light, min_layer, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_layer_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The light mask. See [`LightOccluder2D`][crate::classes::LightOccluder2D] for more information on light masks."]
        pub fn canvas_light_set_item_cull_mask(&mut self, light: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_item_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The binary mask used to determine which layers this canvas light's shadows affects. See [`LightOccluder2D`][crate::classes::LightOccluder2D] for more information on light masks."]
        pub fn canvas_light_set_item_shadow_cull_mask(&mut self, light: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_item_shadow_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the mode of the canvas light."]
        pub fn canvas_light_set_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::CanvasLightMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables the canvas light's shadow."]
        pub fn canvas_light_set_shadow_enabled(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the canvas light's shadow's filter."]
        pub fn canvas_light_set_shadow_filter(&mut self, light: Rid, filter: crate::classes::rendering_server::CanvasLightShadowFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightShadowFilter,);
            let args = (light, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the color of the canvas light's shadow."]
        pub fn canvas_light_set_shadow_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Smoothens the shadow. The lower, the smoother."]
        pub fn canvas_light_set_shadow_smooth(&mut self, light: Rid, smooth: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, smooth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_smooth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the blend mode for the given canvas light to `mode`. Equivalent to \\[member Light2D.blend_mode]."]
        pub fn canvas_light_set_blend_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::CanvasLightBlendMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightBlendMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `interpolated` is `true`, turns on physics interpolation for the canvas light."]
        pub fn canvas_light_set_interpolated(&mut self, light: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents physics interpolation for the current physics tick.\n\nThis is useful when moving a canvas item to a new location, to give an instantaneous change rather than interpolation from the previous location."]
        pub fn canvas_light_reset_physics_interpolation(&mut self, light: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (light,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms both the current and previous stored transform for a canvas light.\n\nThis allows transforming a light without creating a \"glitch\" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin."]
        pub fn canvas_light_transform_physics_interpolation(&mut self, light: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (light, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a light occluder and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `canvas_light_occluder_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent node is [`LightOccluder2D`][crate::classes::LightOccluder2D]."]
        pub fn canvas_light_occluder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Attaches a light occluder to the canvas. Removes it from its previous canvas."]
        pub fn canvas_light_occluder_attach_to_canvas(&mut self, occluder: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (occluder, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_attach_to_canvas", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enables or disables light occluder."]
        pub fn canvas_light_occluder_set_enabled(&mut self, occluder: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a light occluder's polygon."]
        pub fn canvas_light_occluder_set_polygon(&mut self, occluder: Rid, polygon: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (occluder, polygon,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_as_sdf_collision(&mut self, occluder: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_as_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a light occluder's [`Transform2D`][crate::builtin::Transform2D]."]
        pub fn canvas_light_occluder_set_transform(&mut self, occluder: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (occluder, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The light mask. See [`LightOccluder2D`][crate::classes::LightOccluder2D] for more information on light masks."]
        pub fn canvas_light_occluder_set_light_mask(&mut self, occluder: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (occluder, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `interpolated` is `true`, turns on physics interpolation for the light occluder."]
        pub fn canvas_light_occluder_set_interpolated(&mut self, occluder: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Prevents physics interpolation for the current physics tick.\n\nThis is useful when moving an occluder to a new location, to give an instantaneous change rather than interpolation from the previous location."]
        pub fn canvas_light_occluder_reset_physics_interpolation(&mut self, occluder: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (occluder,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Transforms both the current and previous stored transform for a light occluder.\n\nThis allows transforming an occluder without creating a \"glitch\" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin."]
        pub fn canvas_light_occluder_transform_physics_interpolation(&mut self, occluder: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (occluder, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new light occluder polygon and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all `canvas_occluder_polygon_*` RenderingServer functions.\n\nOnce finished with your RID, you will want to free the RID using the RenderingServer's [`free_rid`][`crate::classes::RenderingServer::free_rid`] method.\n\n**Note:** The equivalent resource is [`OccluderPolygon2D`][crate::classes::OccluderPolygon2D]."]
        pub fn canvas_occluder_polygon_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the shape of the occluder polygon."]
        pub fn canvas_occluder_polygon_set_shape(&mut self, occluder_polygon: Rid, shape: &PackedVector2Array, closed: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedVector2Array >, bool,);
            let args = (occluder_polygon, RefArg::new(shape), closed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets an occluder polygon's cull mode."]
        pub fn canvas_occluder_polygon_set_cull_mode(&mut self, occluder_polygon: Rid, mode: crate::classes::rendering_server::CanvasOccluderPolygonCullMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasOccluderPolygonCullMode,);
            let args = (occluder_polygon, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_set_cull_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the \\[member ProjectSettings.rendering/2d/shadow_atlas/size] to use for [`Light2D`][crate::classes::Light2D] shadow rendering (in pixels). The value is rounded up to the nearest power of 2."]
        pub fn canvas_set_shadow_texture_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_shadow_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a new global shader uniform.\n\n**Note:** Global shader parameter names are case-sensitive."]
        pub fn global_shader_parameter_add(&mut self, name: impl AsArg < StringName >, type_: crate::classes::rendering_server::GlobalShaderParameterType, default_value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, crate::classes::rendering_server::GlobalShaderParameterType, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), type_, RefArg::new(default_value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_add", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the global shader uniform specified by `name`."]
        pub fn global_shader_parameter_remove(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_remove", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the list of global shader uniform names.\n\n**Note:** [`global_shader_parameter_get`][`crate::classes::RenderingServer::global_shader_parameter_get`] has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters."]
        pub fn global_shader_parameter_get_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the global shader uniform `name` to `value`."]
        pub fn global_shader_parameter_set(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overrides the global shader uniform `name` with `value`. Equivalent to the [`ShaderGlobalsOverride`][crate::classes::ShaderGlobalsOverride] node."]
        pub fn global_shader_parameter_set_override(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_set_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the value of the global shader uniform specified by `name`.\n\n**Note:** [`global_shader_parameter_get`][`crate::classes::RenderingServer::global_shader_parameter_get`] has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters."]
        pub fn global_shader_parameter_get(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type associated to the global shader uniform specified by `name`.\n\n**Note:** [`global_shader_parameter_get`][`crate::classes::RenderingServer::global_shader_parameter_get`] has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters."]
        pub fn global_shader_parameter_get_type(&self, name: impl AsArg < StringName >,) -> crate::classes::rendering_server::GlobalShaderParameterType {
            type CallRet = crate::classes::rendering_server::GlobalShaderParameterType;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Tries to free an object in the RenderingServer. To avoid memory leaks, this should be called after using an object as memory management does not occur automatically when using RenderingServer directly."]
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Schedules a callback to the given callable after a frame has been drawn."]
        pub fn request_frame_drawn_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "request_frame_drawn_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if changes have been made to the RenderingServer's data. [`force_draw`][`crate::classes::RenderingServer::force_draw`] is usually called if this happens."]
        pub fn has_changed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_changed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a statistic about the rendering engine which can be used for performance profiling. See also [`viewport_get_render_info`][`crate::classes::RenderingServer::viewport_get_render_info`], which returns information specific to a viewport.\n\n**Note:** Only 3D rendering is currently taken into account by some of these values, such as the number of draw calls.\n\n**Note:** Rendering information is not available until at least 2 frames have been rendered by the engine. If rendering information is not available, [`get_rendering_info`][`crate::classes::RenderingServer::get_rendering_info`] returns `0`. To print rendering information in `_ready()` successfully, use the following:\n\n```gdscript\nfunc _ready():\n\tfor _i in 2:\n\t\tawait get_tree().process_frame\n\n\tprint(RenderingServer.get_rendering_info(RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME))\n```"]
        pub fn get_rendering_info(&self, info: crate::classes::rendering_server::RenderingInfo,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::rendering_server::RenderingInfo,);
            let args = (info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_rendering_info", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the video adapter (e.g. \"GeForce GTX 1080/PCIe/SSE2\").\n\n**Note:** When running a headless or server binary, this function returns an empty string.\n\n**Note:** On the web platform, some browsers such as Firefox may report a different, fixed GPU name such as \"GeForce GTX 980\" (regardless of the user's actual GPU model). This is done to make fingerprinting more difficult."]
        pub fn get_video_adapter_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the vendor of the video adapter (e.g. \"NVIDIA Corporation\").\n\n**Note:** When running a headless or server binary, this function returns an empty string."]
        pub fn get_video_adapter_vendor(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_vendor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the type of the video adapter. Since dedicated graphics cards from a given generation will _usually_ be significantly faster than integrated graphics made in the same generation, the device type can be used as a basis for automatic graphics settings adjustment. However, this is not always true, so make sure to provide users with a way to manually override graphics settings.\n\n**Note:** When using the OpenGL rendering driver or when running in headless mode, this function always returns [`DeviceType::OTHER`][`crate::classes::rendering_device::DeviceType::OTHER`]."]
        pub fn get_video_adapter_type(&self,) -> crate::classes::rendering_device::DeviceType {
            type CallRet = crate::classes::rendering_device::DeviceType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_type", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the version of the graphics video adapter _currently in use_ (e.g. \"1.2.189\" for Vulkan, \"3.3.0 NVIDIA 510.60.02\" for OpenGL). This version may be different from the actual latest version supported by the hardware, as Godot may not always request the latest version. See also [`get_video_adapter_driver_info`][`crate::classes::Os::get_video_adapter_driver_info`].\n\n**Note:** When running a headless or server binary, this function returns an empty string."]
        pub fn get_video_adapter_api_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_api_version", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the current rendering driver. This can be `vulkan`, `d3d12`, `metal`, `opengl3`, `opengl3_es`, or `opengl3_angle`. See also [`get_current_rendering_method`][`crate::classes::RenderingServer::get_current_rendering_method`].\n\nWhen \\[member ProjectSettings.rendering/renderer/rendering_method] is `forward_plus` or `mobile`, the rendering driver is determined by \\[member ProjectSettings.rendering/rendering_device/driver].\n\nWhen \\[member ProjectSettings.rendering/renderer/rendering_method] is `gl_compatibility`, the rendering driver is determined by \\[member ProjectSettings.rendering/gl_compatibility/driver].\n\nThe rendering driver is also determined by the `--rendering-driver` command line argument that overrides this project setting, or an automatic fallback that is applied depending on the hardware."]
        pub fn get_current_rendering_driver_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_current_rendering_driver_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the current rendering method. This can be `forward_plus`, `mobile`, or `gl_compatibility`. See also [`get_current_rendering_driver_name`][`crate::classes::RenderingServer::get_current_rendering_driver_name`].\n\nThe rendering method is determined by \\[member ProjectSettings.rendering/renderer/rendering_method], the `--rendering-method` command line argument that overrides this project setting, or an automatic fallback that is applied depending on the hardware."]
        pub fn get_current_rendering_method(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_current_rendering_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a mesh of a sphere with the given number of horizontal subdivisions, vertical subdivisions and radius. See also [`get_test_cube`][`crate::classes::RenderingServer::get_test_cube`]."]
        pub fn make_sphere_mesh(&mut self, latitudes: i32, longitudes: i32, radius: f32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32, i32, f32,);
            let args = (latitudes, longitudes, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "make_sphere_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the RID of the test cube. This mesh will be created and returned on the first call to [`get_test_cube`][`crate::classes::RenderingServer::get_test_cube`], then it will be cached for subsequent calls. See also [`make_sphere_mesh`][`crate::classes::RenderingServer::make_sphere_mesh`]."]
        pub fn get_test_cube(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_test_cube", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the RID of a 256×256 texture with a testing pattern on it (in [`Format::RGB8`][`crate::classes::image::Format::RGB8`] format). This texture will be created and returned on the first call to [`get_test_texture`][`crate::classes::RenderingServer::get_test_texture`], then it will be cached for subsequent calls. See also [`get_white_texture`][`crate::classes::RenderingServer::get_white_texture`].\n\n**Example:** Get the test texture and apply it to a [`Sprite2D`][crate::classes::Sprite2D] node:\n\n```gdscript\nvar texture_rid = RenderingServer.get_test_texture()\nvar texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))\n$Sprite2D.texture = texture\n```"]
        pub fn get_test_texture(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_test_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of a 4×4 white texture (in [`Format::RGB8`][`crate::classes::image::Format::RGB8`] format). This texture will be created and returned on the first call to [`get_white_texture`][`crate::classes::RenderingServer::get_white_texture`], then it will be cached for subsequent calls. See also [`get_test_texture`][`crate::classes::RenderingServer::get_test_texture`].\n\n**Example:** Get the white texture and apply it to a [`Sprite2D`][crate::classes::Sprite2D] node:\n\n```gdscript\nvar texture_rid = RenderingServer.get_white_texture()\nvar texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))\n$Sprite2D.texture = texture\n```"]
        pub fn get_white_texture(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_white_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `stretch_mode` indicates how the image will be stretched (see \\[enum SplashStretchMode] for possible values). If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        pub(crate) fn set_boot_image_with_stretch_full(&mut self, image: CowArg < Option < Gd < crate::classes::Image > > >, color: Color, stretch_mode: crate::classes::rendering_server::SplashStretchMode, use_filter: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Color, crate::classes::rendering_server::SplashStretchMode, bool,);
            let args = (image, color, stretch_mode, use_filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_boot_image_with_stretch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_boot_image_with_stretch_ex`][Self::set_boot_image_with_stretch_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `stretch_mode` indicates how the image will be stretched (see \\[enum SplashStretchMode] for possible values). If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        #[inline]
        pub fn set_boot_image_with_stretch(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, color: Color, stretch_mode: crate::classes::rendering_server::SplashStretchMode,) {
            self.set_boot_image_with_stretch_ex(image, color, stretch_mode,) . done()
        }
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `stretch_mode` indicates how the image will be stretched (see \\[enum SplashStretchMode] for possible values). If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        #[inline]
        pub fn set_boot_image_with_stretch_ex < 'ex > (&'ex mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'ex, color: Color, stretch_mode: crate::classes::rendering_server::SplashStretchMode,) -> ExSetBootImageWithStretch < 'ex > {
            ExSetBootImageWithStretch::new(self, image, color, stretch_mode,)
        }
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `scale` indicates if the image will be scaled to fit the screen size. If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        pub(crate) fn set_boot_image_full(&mut self, image: CowArg < Option < Gd < crate::classes::Image > > >, color: Color, scale: bool, use_filter: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Color, bool, bool,);
            let args = (image, color, scale, use_filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_boot_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_boot_image_ex`][Self::set_boot_image_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `scale` indicates if the image will be scaled to fit the screen size. If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        #[inline]
        pub fn set_boot_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, color: Color, scale: bool,) {
            self.set_boot_image_ex(image, color, scale,) . done()
        }
        #[doc = "Sets a boot image. The `color` defines the background color. The value of `scale` indicates if the image will be scaled to fit the screen size. If `use_filter` is `true`, the image will be scaled with linear interpolation. If `use_filter` is `false`, the image will be scaled with nearest-neighbor interpolation."]
        #[inline]
        pub fn set_boot_image_ex < 'ex > (&'ex mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'ex, color: Color, scale: bool,) -> ExSetBootImage < 'ex > {
            ExSetBootImage::new(self, image, color, scale,)
        }
        #[doc = "Returns the default clear color which is used when a specific clear color has not been selected. See also [`set_default_clear_color`][`crate::classes::RenderingServer::set_default_clear_color`]."]
        pub fn get_default_clear_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_default_clear_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default clear color which is used when a specific clear color has not been selected. See also [`get_default_clear_color`][`crate::classes::RenderingServer::get_default_clear_color`]."]
        pub fn set_default_clear_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_default_clear_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the OS supports a certain `feature`. Features might be `s3tc`, `etc`, and `etc2`."]
        pub fn has_os_feature(&self, feature: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (feature.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_os_feature", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `generate` is `true`, generates debug wireframes for all meshes that are loaded when using the Compatibility renderer. By default, the engine does not generate debug wireframes at runtime, since they slow down loading of assets and take up VRAM.\n\n**Note:** You must call this method before loading any meshes when using the Compatibility renderer, otherwise wireframes will not be used."]
        pub fn set_debug_generate_wireframes(&mut self, generate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (generate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_debug_generate_wireframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_render_loop_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "is_render_loop_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_loop_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_render_loop_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the time taken to setup rendering on the CPU in milliseconds. This value is shared across all viewports and does _not_ require [`viewport_set_measure_render_time`][`crate::classes::RenderingServer::viewport_set_measure_render_time`] to be enabled on a viewport to be queried. See also [`viewport_get_measured_render_time_cpu`][`crate::classes::RenderingServer::viewport_get_measured_render_time_cpu`]."]
        pub fn get_frame_setup_time_cpu(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_frame_setup_time_cpu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces a synchronization between the CPU and GPU, which may be required in certain cases. Only call this when needed, as CPU-GPU synchronization has a performance cost."]
        pub fn force_sync(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "force_sync", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces redrawing of all viewports at once. Must be called from the main thread."]
        pub(crate) fn force_draw_full(&mut self, swap_buffers: bool, frame_step: f64,) {
            type CallRet = ();
            type CallParams = (bool, f64,);
            let args = (swap_buffers, frame_step,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "force_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`force_draw_ex`][Self::force_draw_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Forces redrawing of all viewports at once. Must be called from the main thread."]
        #[inline]
        pub fn force_draw(&mut self,) {
            self.force_draw_ex() . done()
        }
        #[doc = "Forces redrawing of all viewports at once. Must be called from the main thread."]
        #[inline]
        pub fn force_draw_ex < 'ex > (&'ex mut self,) -> ExForceDraw < 'ex > {
            ExForceDraw::new(self,)
        }
        #[doc = "Returns the global RenderingDevice.\n\n**Note:** When using the OpenGL rendering driver or when running in headless mode, this function always returns `null`."]
        pub fn get_rendering_device(&self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallRet = Option < Gd < crate::classes::RenderingDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_rendering_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates a RenderingDevice that can be used to do draw and compute operations on a separate thread. Cannot draw to the screen nor share data with the global RenderingDevice.\n\n**Note:** When using the OpenGL rendering driver or when running in headless mode, this function always returns `null`."]
        pub fn create_local_rendering_device(&self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallRet = Option < Gd < crate::classes::RenderingDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "create_local_rendering_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if our code is currently executing on the rendering thread."]
        pub fn is_on_render_thread(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "is_on_render_thread", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "As the RenderingServer actual logic may run on a separate thread, accessing its internals from the main (or any other) thread will result in errors. To make it easier to run code that can safely access the rendering internals (such as [`RenderingDevice`][crate::classes::RenderingDevice] and similar RD classes), push a callable via this function so it will be executed on the render thread."]
        pub fn call_on_render_thread(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "call_on_render_thread", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method does nothing and always returns `false`."]
        pub fn has_feature(&self, feature: crate::classes::rendering_server::Features,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::rendering_server::Features,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_feature", Some(self.__validated_obj()), args,)
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
        pub const NO_INDEX_ARRAY: i32 = - 1i32;
        pub const ARRAY_WEIGHTS_SIZE: i32 = 4i32;
        pub const CANVAS_ITEM_Z_MIN: i32 = - 4096i32;
        pub const CANVAS_ITEM_Z_MAX: i32 = 4096i32;
        pub const CANVAS_LAYER_MIN: i32 = - 2147483648i32;
        pub const CANVAS_LAYER_MAX: i32 = 2147483647i32;
        pub const MAX_GLOW_LEVELS: i32 = 7i32;
        pub const MAX_CURSORS: i32 = 8i32;
        pub const MAX_2D_DIRECTIONAL_LIGHTS: i32 = 8i32;
        pub const MAX_MESH_SURFACES: i32 = 256i32;
        pub const MATERIAL_RENDER_PRIORITY_MIN: i32 = - 128i32;
        pub const MATERIAL_RENDER_PRIORITY_MAX: i32 = 127i32;
        pub const ARRAY_CUSTOM_COUNT: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_POSITION: i32 = 1i32;
        pub const PARTICLES_EMIT_FLAG_ROTATION_SCALE: i32 = 2i32;
        pub const PARTICLES_EMIT_FLAG_VELOCITY: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_COLOR: i32 = 8i32;
        pub const PARTICLES_EMIT_FLAG_CUSTOM: i32 = 16i32;
        
    }
    impl crate::obj::GodotClass for RenderingServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderingServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for RenderingServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderingServer {
        
    }
    impl crate::obj::Singleton for RenderingServer {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"RenderingServer"))
            }
        }
    }
    impl std::ops::Deref for RenderingServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderingServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `RenderingServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_create_from_native_handle_ex`][super::RenderingServer::texture_create_from_native_handle_ex]."]
#[must_use]
pub struct ExTextureCreateFromNativeHandle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32, layers: i32, layered_type: crate::classes::rendering_server::TextureLayeredType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTextureCreateFromNativeHandle < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> Self {
        let layers = 1i32;
        let layered_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, format: format, native_handle: native_handle, width: width, height: height, depth: depth, layers: layers, layered_type: layered_type,
        }
    }
    #[inline]
    pub fn layers(self, layers: i32) -> Self {
        Self {
            layers: layers, .. self
        }
    }
    #[inline]
    pub fn layered_type(self, layered_type: crate::classes::rendering_server::TextureLayeredType) -> Self {
        Self {
            layered_type: layered_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, type_, format, native_handle, width, height, depth, layers, layered_type,
        }
        = self;
        re_export::RenderingServer::texture_create_from_native_handle_full(surround_object, type_, format, native_handle, width, height, depth, layers, layered_type,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_rd_create_ex`][super::RenderingServer::texture_rd_create_ex]."]
#[must_use]
pub struct ExTextureRdCreate < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, rd_texture: Rid, layer_type: crate::classes::rendering_server::TextureLayeredType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTextureRdCreate < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, rd_texture: Rid,) -> Self {
        let layer_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rd_texture: rd_texture, layer_type: layer_type,
        }
    }
    #[inline]
    pub fn layer_type(self, layer_type: crate::classes::rendering_server::TextureLayeredType) -> Self {
        Self {
            layer_type: layer_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, rd_texture, layer_type,
        }
        = self;
        re_export::RenderingServer::texture_rd_create_full(surround_object, rd_texture, layer_type,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_rd_texture_ex`][super::RenderingServer::texture_get_rd_texture_ex]."]
#[must_use]
pub struct ExTextureGetRdTexture < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTextureGetRdTexture < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, texture: Rid,) -> Self {
        let srgb = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture, srgb: srgb,
        }
    }
    #[inline]
    pub fn srgb(self, srgb: bool) -> Self {
        Self {
            srgb: srgb, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, texture, srgb,
        }
        = self;
        re_export::RenderingServer::texture_get_rd_texture_full(surround_object, texture, srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_native_handle_ex`][super::RenderingServer::texture_get_native_handle_ex]."]
#[must_use]
pub struct ExTextureGetNativeHandle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExTextureGetNativeHandle < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, texture: Rid,) -> Self {
        let srgb = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture, srgb: srgb,
        }
    }
    #[inline]
    pub fn srgb(self, srgb: bool) -> Self {
        Self {
            srgb: srgb, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, texture, srgb,
        }
        = self;
        re_export::RenderingServer::texture_get_native_handle_full(surround_object, texture, srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_set_default_texture_parameter_ex`][super::RenderingServer::shader_set_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderSetDefaultTextureParameter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, shader: Rid, name: CowArg < 'ex, StringName >, texture: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShaderSetDefaultTextureParameter < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, shader: Rid, name: impl AsArg < StringName > + 'ex, texture: Rid,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, name: name.into_arg(), texture: texture, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shader, name, texture, index,
        }
        = self;
        re_export::RenderingServer::shader_set_default_texture_parameter_full(surround_object, shader, name, texture, index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_get_default_texture_parameter_ex`][super::RenderingServer::shader_get_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderGetDefaultTextureParameter < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, shader: Rid, name: CowArg < 'ex, StringName >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExShaderGetDefaultTextureParameter < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, shader: Rid, name: impl AsArg < StringName > + 'ex,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, name: name.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, name, index,
        }
        = self;
        re_export::RenderingServer::shader_get_default_texture_parameter_full(surround_object, shader, name, index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_create_from_surfaces_ex`][super::RenderingServer::mesh_create_from_surfaces_ex]."]
#[must_use]
pub struct ExMeshCreateFromSurfaces < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, surfaces: CowArg < 'ex, Array < AnyDictionary > >, blend_shape_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMeshCreateFromSurfaces < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, surfaces: &'ex Array < AnyDictionary >,) -> Self {
        let blend_shape_count = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, surfaces: CowArg::Borrowed(surfaces), blend_shape_count: blend_shape_count,
        }
    }
    #[inline]
    pub fn blend_shape_count(self, blend_shape_count: i32) -> Self {
        Self {
            blend_shape_count: blend_shape_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, surfaces, blend_shape_count,
        }
        = self;
        re_export::RenderingServer::mesh_create_from_surfaces_full(surround_object, surfaces.cow_as_arg(), blend_shape_count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_add_surface_from_arrays_ex`][super::RenderingServer::mesh_add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExMeshAddSurfaceFromArrays < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: CowArg < 'ex, AnyArray >, blend_shapes: CowArg < 'ex, AnyArray >, lods: CowArg < 'ex, AnyDictionary >, compress_format: crate::classes::rendering_server::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMeshAddSurfaceFromArrays < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &'ex AnyArray,) -> Self {
        let blend_shapes = AnyArray::new_untyped();
        let lods = AnyDictionary::new_untyped();
        let compress_format = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), compress_format: compress_format,
        }
    }
    #[inline]
    pub fn blend_shapes(self, blend_shapes: &'ex AnyArray) -> Self {
        Self {
            blend_shapes: CowArg::Borrowed(blend_shapes), .. self
        }
    }
    #[inline]
    pub fn lods(self, lods: &'ex AnyDictionary) -> Self {
        Self {
            lods: CowArg::Borrowed(lods), .. self
        }
    }
    #[inline]
    pub fn compress_format(self, compress_format: crate::classes::rendering_server::ArrayFormat) -> Self {
        Self {
            compress_format: compress_format, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mesh, primitive, arrays, blend_shapes, lods, compress_format,
        }
        = self;
        re_export::RenderingServer::mesh_add_surface_from_arrays_full(surround_object, mesh, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), compress_format,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::multimesh_allocate_data_ex`][super::RenderingServer::multimesh_allocate_data_ex]."]
#[must_use]
pub struct ExMultimeshAllocateData < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool, use_indirect: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMultimeshAllocateData < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) -> Self {
        let color_format = false;
        let custom_data_format = false;
        let use_indirect = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, multimesh: multimesh, instances: instances, transform_format: transform_format, color_format: color_format, custom_data_format: custom_data_format, use_indirect: use_indirect,
        }
    }
    #[inline]
    pub fn color_format(self, color_format: bool) -> Self {
        Self {
            color_format: color_format, .. self
        }
    }
    #[inline]
    pub fn custom_data_format(self, custom_data_format: bool) -> Self {
        Self {
            custom_data_format: custom_data_format, .. self
        }
    }
    #[inline]
    pub fn use_indirect(self, use_indirect: bool) -> Self {
        Self {
            use_indirect: use_indirect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,
        }
        = self;
        re_export::RenderingServer::multimesh_allocate_data_full(surround_object, multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::skeleton_allocate_data_ex`][super::RenderingServer::skeleton_allocate_data_ex]."]
#[must_use]
pub struct ExSkeletonAllocateData < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, skeleton: Rid, bones: i32, is_2d_skeleton: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSkeletonAllocateData < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, skeleton: Rid, bones: i32,) -> Self {
        let is_2d_skeleton = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, skeleton: skeleton, bones: bones, is_2d_skeleton: is_2d_skeleton,
        }
    }
    #[inline]
    pub fn is_2d_skeleton(self, is_2d_skeleton: bool) -> Self {
        Self {
            is_2d_skeleton: is_2d_skeleton, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, skeleton, bones, is_2d_skeleton,
        }
        = self;
        re_export::RenderingServer::skeleton_allocate_data_full(surround_object, skeleton, bones, is_2d_skeleton,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_attach_to_screen_ex`][super::RenderingServer::viewport_attach_to_screen_ex]."]
#[must_use]
pub struct ExViewportAttachToScreen < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, viewport: Rid, rect: Rect2, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExViewportAttachToScreen < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, viewport: Rid,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, viewport: viewport, rect: rect, screen: screen,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2) -> Self {
        Self {
            rect: rect, .. self
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
            _phantom, surround_object, viewport, rect, screen,
        }
        = self;
        re_export::RenderingServer::viewport_attach_to_screen_full(surround_object, viewport, rect, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_set_positional_shadow_atlas_size_ex`][super::RenderingServer::viewport_set_positional_shadow_atlas_size_ex]."]
#[must_use]
pub struct ExViewportSetPositionalShadowAtlasSize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, viewport: Rid, size: i32, use_16_bits: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExViewportSetPositionalShadowAtlasSize < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, viewport: Rid, size: i32,) -> Self {
        let use_16_bits = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, viewport: viewport, size: size, use_16_bits: use_16_bits,
        }
    }
    #[inline]
    pub fn use_16_bits(self, use_16_bits: bool) -> Self {
        Self {
            use_16_bits: use_16_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, viewport, size, use_16_bits,
        }
        = self;
        re_export::RenderingServer::viewport_set_positional_shadow_atlas_size_full(surround_object, viewport, size, use_16_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::environment_set_ambient_light_ex`][super::RenderingServer::environment_set_ambient_light_ex]."]
#[must_use]
pub struct ExEnvironmentSetAmbientLight < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, env: Rid, color: Color, ambient: crate::classes::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contribution: f32, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEnvironmentSetAmbientLight < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, env: Rid, color: Color,) -> Self {
        let ambient = crate::obj::EngineEnum::from_ord(0);
        let energy = 1f32;
        let sky_contribution = 0f32;
        let reflection_source = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, env: env, color: color, ambient: ambient, energy: energy, sky_contribution: sky_contribution, reflection_source: reflection_source,
        }
    }
    #[inline]
    pub fn ambient(self, ambient: crate::classes::rendering_server::EnvironmentAmbientSource) -> Self {
        Self {
            ambient: ambient, .. self
        }
    }
    #[inline]
    pub fn energy(self, energy: f32) -> Self {
        Self {
            energy: energy, .. self
        }
    }
    #[inline]
    pub fn sky_contribution(self, sky_contribution: f32) -> Self {
        Self {
            sky_contribution: sky_contribution, .. self
        }
    }
    #[inline]
    pub fn reflection_source(self, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource) -> Self {
        Self {
            reflection_source: reflection_source, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, env, color, ambient, energy, sky_contribution, reflection_source,
        }
        = self;
        re_export::RenderingServer::environment_set_ambient_light_full(surround_object, env, color, ambient, energy, sky_contribution, reflection_source,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::environment_set_fog_ex`][super::RenderingServer::environment_set_fog_ex]."]
#[must_use]
pub struct ExEnvironmentSetFog < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32, fog_mode: crate::classes::rendering_server::EnvironmentFogMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExEnvironmentSetFog < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) -> Self {
        let fog_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, env: env, enable: enable, light_color: light_color, light_energy: light_energy, sun_scatter: sun_scatter, density: density, height: height, height_density: height_density, aerial_perspective: aerial_perspective, sky_affect: sky_affect, fog_mode: fog_mode,
        }
    }
    #[inline]
    pub fn fog_mode(self, fog_mode: crate::classes::rendering_server::EnvironmentFogMode) -> Self {
        Self {
            fog_mode: fog_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,
        }
        = self;
        re_export::RenderingServer::environment_set_fog_full(surround_object, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_aabb_ex`][super::RenderingServer::instances_cull_aabb_ex]."]
#[must_use]
pub struct ExInstancesCullAabb < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, aabb: Aabb, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInstancesCullAabb < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, aabb: Aabb,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, aabb: aabb, scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, aabb, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_aabb_full(surround_object, aabb, scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_ray_ex`][super::RenderingServer::instances_cull_ray_ex]."]
#[must_use]
pub struct ExInstancesCullRay < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, from: Vector3, to: Vector3, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInstancesCullRay < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, from: Vector3, to: Vector3,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, from, to, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_ray_full(surround_object, from, to, scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_convex_ex`][super::RenderingServer::instances_cull_convex_ex]."]
#[must_use]
pub struct ExInstancesCullConvex < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::RenderingServer, convex: CowArg < 'ex, Array < Plane > >, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInstancesCullConvex < 'ex > {
    fn new(surround_object: &'ex re_export::RenderingServer, convex: &'ex Array < Plane >,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, convex: CowArg::Borrowed(convex), scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, convex, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_convex_full(surround_object, convex.cow_as_arg(), scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_custom_rect_ex`][super::RenderingServer::canvas_item_set_custom_rect_ex]."]
#[must_use]
pub struct ExCanvasItemSetCustomRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, use_custom_rect: bool, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemSetCustomRect < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, use_custom_rect: bool,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, use_custom_rect: use_custom_rect, rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, use_custom_rect, rect,
        }
        = self;
        re_export::RenderingServer::canvas_item_set_custom_rect_full(surround_object, item, use_custom_rect, rect,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_line_ex`][super::RenderingServer::canvas_item_add_line_ex]."]
#[must_use]
pub struct ExCanvasItemAddLine < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddLine < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, from: from, to: to, color: color, width: width, antialiased: antialiased,
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
            _phantom, surround_object, item, from, to, color, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_line_full(surround_object, item, from, to, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polyline_ex`][super::RenderingServer::canvas_item_add_polyline_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolyline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddPolyline < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
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
            _phantom, surround_object, item, points, colors, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_polyline_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multiline_ex`][super::RenderingServer::canvas_item_add_multiline_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultiline < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddMultiline < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
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
            _phantom, surround_object, item, points, colors, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_multiline_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_rect_ex`][super::RenderingServer::canvas_item_add_rect_ex]."]
#[must_use]
pub struct ExCanvasItemAddRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, color: Color, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddRect < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, color: Color,) -> Self {
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, color: color, antialiased: antialiased,
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
            _phantom, surround_object, item, rect, color, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_rect_full(surround_object, item, rect, color, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_circle_ex`][super::RenderingServer::canvas_item_add_circle_ex]."]
#[must_use]
pub struct ExCanvasItemAddCircle < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, pos: Vector2, radius: f32, color: Color, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddCircle < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, pos: Vector2, radius: f32, color: Color,) -> Self {
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, pos: pos, radius: radius, color: color, antialiased: antialiased,
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
            _phantom, surround_object, item, pos, radius, color, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_circle_full(surround_object, item, pos, radius, color, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_ellipse_ex`][super::RenderingServer::canvas_item_add_ellipse_ex]."]
#[must_use]
pub struct ExCanvasItemAddEllipse < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, pos: Vector2, major: f32, minor: f32, color: Color, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddEllipse < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, pos: Vector2, major: f32, minor: f32, color: Color,) -> Self {
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, pos: pos, major: major, minor: minor, color: color, antialiased: antialiased,
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
            _phantom, surround_object, item, pos, major, minor, color, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_ellipse_full(surround_object, item, pos, major, minor, color, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_ex`][super::RenderingServer::canvas_item_add_texture_rect_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddTextureRect < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid,) -> Self {
        let tile = false;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, tile: tile, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn tile(self, tile: bool) -> Self {
        Self {
            tile: tile, .. self
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
            _phantom, surround_object, item, rect, texture, tile, modulate, transpose,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_texture_rect_full(surround_object, item, rect, texture, tile, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_msdf_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddMsdfTextureRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddMsdfTextureRectRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline_size = 0i32;
        let px_range = 1f32;
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, src_rect: src_rect, modulate: modulate, outline_size: outline_size, px_range: px_range, scale: scale,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn px_range(self, px_range: f32) -> Self {
        Self {
            px_range: px_range, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, texture, src_rect, modulate, outline_size, px_range, scale,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_msdf_texture_rect_region_full(surround_object, item, rect, texture, src_rect, modulate, outline_size, px_range, scale,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRectRegion < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddTextureRectRegion < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        let clip_uv = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, src_rect: src_rect, modulate: modulate, transpose: transpose, clip_uv: clip_uv,
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
            _phantom, surround_object, item, rect, texture, src_rect, modulate, transpose, clip_uv,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_texture_rect_region_full(surround_object, item, rect, texture, src_rect, modulate, transpose, clip_uv,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_nine_patch_ex`][super::RenderingServer::canvas_item_add_nine_patch_ex]."]
#[must_use]
pub struct ExCanvasItemAddNinePatch < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddNinePatch < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> Self {
        let x_axis_mode = crate::obj::EngineEnum::from_ord(0);
        let y_axis_mode = crate::obj::EngineEnum::from_ord(0);
        let draw_center = true;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, source: source, texture: texture, topleft: topleft, bottomright: bottomright, x_axis_mode: x_axis_mode, y_axis_mode: y_axis_mode, draw_center: draw_center, modulate: modulate,
        }
    }
    #[inline]
    pub fn x_axis_mode(self, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            x_axis_mode: x_axis_mode, .. self
        }
    }
    #[inline]
    pub fn y_axis_mode(self, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            y_axis_mode: y_axis_mode, .. self
        }
    }
    #[inline]
    pub fn draw_center(self, draw_center: bool) -> Self {
        Self {
            draw_center: draw_center, .. self
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
            _phantom, surround_object, item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_nine_patch_full(surround_object, item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polygon_ex`][super::RenderingServer::canvas_item_add_polygon_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolygon < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, uvs: CowArg < 'ex, PackedVector2Array >, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddPolygon < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), texture: texture,
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'ex PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, points, colors, uvs, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_polygon_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_triangle_array_ex`][super::RenderingServer::canvas_item_add_triangle_array_ex]."]
#[must_use]
pub struct ExCanvasItemAddTriangleArray < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, indices: CowArg < 'ex, PackedInt32Array >, points: CowArg < 'ex, PackedVector2Array >, colors: CowArg < 'ex, PackedColorArray >, uvs: CowArg < 'ex, PackedVector2Array >, bones: CowArg < 'ex, PackedInt32Array >, weights: CowArg < 'ex, PackedFloat32Array >, texture: Rid, count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddTriangleArray < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, indices: &'ex PackedInt32Array, points: &'ex PackedVector2Array, colors: &'ex PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let bones = PackedInt32Array::new();
        let weights = PackedFloat32Array::new();
        let texture = Rid::Invalid;
        let count = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, indices: CowArg::Borrowed(indices), points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), bones: CowArg::Owned(bones), weights: CowArg::Owned(weights), texture: texture, count: count,
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'ex PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn bones(self, bones: &'ex PackedInt32Array) -> Self {
        Self {
            bones: CowArg::Borrowed(bones), .. self
        }
    }
    #[inline]
    pub fn weights(self, weights: &'ex PackedFloat32Array) -> Self {
        Self {
            weights: CowArg::Borrowed(weights), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn count(self, count: i32) -> Self {
        Self {
            count: count, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, indices, points, colors, uvs, bones, weights, texture, count,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_triangle_array_full(surround_object, item, indices.cow_as_arg(), points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), bones.cow_as_arg(), weights.cow_as_arg(), texture, count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_mesh_ex`][super::RenderingServer::canvas_item_add_mesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMesh < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddMesh < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mesh: mesh, transform: transform, modulate: modulate, texture: texture,
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
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mesh, transform, modulate, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_mesh_full(surround_object, item, mesh, transform, modulate, texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multimesh_ex`][super::RenderingServer::canvas_item_add_multimesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultimesh < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, mesh: Rid, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddMultimesh < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mesh: mesh, texture: texture,
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mesh, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_multimesh_full(surround_object, item, mesh, texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_animation_slice_ex`][super::RenderingServer::canvas_item_add_animation_slice_ex]."]
#[must_use]
pub struct ExCanvasItemAddAnimationSlice < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemAddAnimationSlice < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        let offset = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, animation_length: animation_length, slice_begin: slice_begin, slice_end: slice_end, offset: offset,
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
            _phantom, surround_object, item, animation_length, slice_begin, slice_end, offset,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_animation_slice_full(surround_object, item, animation_length, slice_begin, slice_end, offset,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_canvas_group_mode_ex`][super::RenderingServer::canvas_item_set_canvas_group_mode_ex]."]
#[must_use]
pub struct ExCanvasItemSetCanvasGroupMode < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCanvasItemSetCanvasGroupMode < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) -> Self {
        let clear_margin = 5f32;
        let fit_empty = false;
        let fit_margin = 0f32;
        let blur_mipmaps = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mode: mode, clear_margin: clear_margin, fit_empty: fit_empty, fit_margin: fit_margin, blur_mipmaps: blur_mipmaps,
        }
    }
    #[inline]
    pub fn clear_margin(self, clear_margin: f32) -> Self {
        Self {
            clear_margin: clear_margin, .. self
        }
    }
    #[inline]
    pub fn fit_empty(self, fit_empty: bool) -> Self {
        Self {
            fit_empty: fit_empty, .. self
        }
    }
    #[inline]
    pub fn fit_margin(self, fit_margin: f32) -> Self {
        Self {
            fit_margin: fit_margin, .. self
        }
    }
    #[inline]
    pub fn blur_mipmaps(self, blur_mipmaps: bool) -> Self {
        Self {
            blur_mipmaps: blur_mipmaps, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,
        }
        = self;
        re_export::RenderingServer::canvas_item_set_canvas_group_mode_full(surround_object, item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::set_boot_image_with_stretch_ex`][super::RenderingServer::set_boot_image_with_stretch_ex]."]
#[must_use]
pub struct ExSetBootImageWithStretch < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, image: CowArg < 'ex, Option < Gd < crate::classes::Image > > >, color: Color, stretch_mode: crate::classes::rendering_server::SplashStretchMode, use_filter: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetBootImageWithStretch < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'ex, color: Color, stretch_mode: crate::classes::rendering_server::SplashStretchMode,) -> Self {
        let use_filter = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), color: color, stretch_mode: stretch_mode, use_filter: use_filter,
        }
    }
    #[inline]
    pub fn use_filter(self, use_filter: bool) -> Self {
        Self {
            use_filter: use_filter, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, color, stretch_mode, use_filter,
        }
        = self;
        re_export::RenderingServer::set_boot_image_with_stretch_full(surround_object, image, color, stretch_mode, use_filter,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::set_boot_image_ex`][super::RenderingServer::set_boot_image_ex]."]
#[must_use]
pub struct ExSetBootImage < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, image: CowArg < 'ex, Option < Gd < crate::classes::Image > > >, color: Color, scale: bool, use_filter: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetBootImage < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'ex, color: Color, scale: bool,) -> Self {
        let use_filter = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), color: color, scale: scale, use_filter: use_filter,
        }
    }
    #[inline]
    pub fn use_filter(self, use_filter: bool) -> Self {
        Self {
            use_filter: use_filter, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, color, scale, use_filter,
        }
        = self;
        re_export::RenderingServer::set_boot_image_full(surround_object, image, color, scale, use_filter,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::force_draw_ex`][super::RenderingServer::force_draw_ex]."]
#[must_use]
pub struct ExForceDraw < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::RenderingServer, swap_buffers: bool, frame_step: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExForceDraw < 'ex > {
    fn new(surround_object: &'ex mut re_export::RenderingServer,) -> Self {
        let swap_buffers = true;
        let frame_step = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, swap_buffers: swap_buffers, frame_step: frame_step,
        }
    }
    #[inline]
    pub fn swap_buffers(self, swap_buffers: bool) -> Self {
        Self {
            swap_buffers: swap_buffers, .. self
        }
    }
    #[inline]
    pub fn frame_step(self, frame_step: f64) -> Self {
        Self {
            frame_step: frame_step, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, swap_buffers, frame_step,
        }
        = self;
        re_export::RenderingServer::force_draw_full(surround_object, swap_buffers, frame_step,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TEXTURE_TYPE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D`"]
    pub const TYPE_2D: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_TYPE_LAYERED")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_LAYERED`"]
    pub const LAYERED: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_TYPE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_3D`"]
    pub const TYPE_3D: TextureType = TextureType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureType {
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
            Self::TYPE_2D => "TYPE_2D", Self::LAYERED => "LAYERED", Self::TYPE_3D => "TYPE_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureType::TYPE_2D, TextureType::LAYERED, TextureType::TYPE_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TYPE_2D", "TEXTURE_TYPE_2D", TextureType::TYPE_2D), crate::meta::inspect::EnumConstant::new("LAYERED", "TEXTURE_TYPE_LAYERED", TextureType::LAYERED), crate::meta::inspect::EnumConstant::new("TYPE_3D", "TEXTURE_TYPE_3D", TextureType::TYPE_3D)]
        }
    }
}
impl crate::meta::GodotConvert for TextureType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Texture Type 2d", 0i64), EnumeratorShape::new_int("Texture Type Layered", 1i64), EnumeratorShape::new_int("Texture Type 3d", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.TextureType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TextureType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextureType {
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
impl crate::registry::property::Export for TextureType {
    
}
impl crate::meta::Element for TextureType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureLayeredType {
    ord: i32
}
impl TextureLayeredType {
    #[doc(alias = "TEXTURE_LAYERED_2D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_2D_ARRAY`"]
    pub const LAYERED_2D_ARRAY: TextureLayeredType = TextureLayeredType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_LAYERED_CUBEMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_CUBEMAP`"]
    pub const CUBEMAP: TextureLayeredType = TextureLayeredType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_LAYERED_CUBEMAP_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_CUBEMAP_ARRAY`"]
    pub const CUBEMAP_ARRAY: TextureLayeredType = TextureLayeredType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextureLayeredType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureLayeredType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureLayeredType {
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
            Self::LAYERED_2D_ARRAY => "LAYERED_2D_ARRAY", Self::CUBEMAP => "CUBEMAP", Self::CUBEMAP_ARRAY => "CUBEMAP_ARRAY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureLayeredType::LAYERED_2D_ARRAY, TextureLayeredType::CUBEMAP, TextureLayeredType::CUBEMAP_ARRAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureLayeredType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LAYERED_2D_ARRAY", "TEXTURE_LAYERED_2D_ARRAY", TextureLayeredType::LAYERED_2D_ARRAY), crate::meta::inspect::EnumConstant::new("CUBEMAP", "TEXTURE_LAYERED_CUBEMAP", TextureLayeredType::CUBEMAP), crate::meta::inspect::EnumConstant::new("CUBEMAP_ARRAY", "TEXTURE_LAYERED_CUBEMAP_ARRAY", TextureLayeredType::CUBEMAP_ARRAY)]
        }
    }
}
impl crate::meta::GodotConvert for TextureLayeredType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Texture Layered 2d Array", 0i64), EnumeratorShape::new_int("Texture Layered Cubemap", 1i64), EnumeratorShape::new_int("Texture Layered Cubemap Array", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.TextureLayeredType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TextureLayeredType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureLayeredType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TextureLayeredType {
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
impl crate::registry::property::Export for TextureLayeredType {
    
}
impl crate::meta::Element for TextureLayeredType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CubeMapLayer {
    ord: i32
}
impl CubeMapLayer {
    #[doc(alias = "CUBEMAP_LAYER_LEFT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_LEFT`"]
    pub const LEFT: CubeMapLayer = CubeMapLayer {
        ord: 0i32
    };
    #[doc(alias = "CUBEMAP_LAYER_RIGHT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_RIGHT`"]
    pub const RIGHT: CubeMapLayer = CubeMapLayer {
        ord: 1i32
    };
    #[doc(alias = "CUBEMAP_LAYER_BOTTOM")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_BOTTOM`"]
    pub const BOTTOM: CubeMapLayer = CubeMapLayer {
        ord: 2i32
    };
    #[doc(alias = "CUBEMAP_LAYER_TOP")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_TOP`"]
    pub const TOP: CubeMapLayer = CubeMapLayer {
        ord: 3i32
    };
    #[doc(alias = "CUBEMAP_LAYER_FRONT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_FRONT`"]
    pub const FRONT: CubeMapLayer = CubeMapLayer {
        ord: 4i32
    };
    #[doc(alias = "CUBEMAP_LAYER_BACK")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_BACK`"]
    pub const BACK: CubeMapLayer = CubeMapLayer {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for CubeMapLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CubeMapLayer") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CubeMapLayer {
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
            Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::BOTTOM => "BOTTOM", Self::TOP => "TOP", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CubeMapLayer::LEFT, CubeMapLayer::RIGHT, CubeMapLayer::BOTTOM, CubeMapLayer::TOP, CubeMapLayer::FRONT, CubeMapLayer::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CubeMapLayer >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "CUBEMAP_LAYER_LEFT", CubeMapLayer::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "CUBEMAP_LAYER_RIGHT", CubeMapLayer::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM", "CUBEMAP_LAYER_BOTTOM", CubeMapLayer::BOTTOM), crate::meta::inspect::EnumConstant::new("TOP", "CUBEMAP_LAYER_TOP", CubeMapLayer::TOP), crate::meta::inspect::EnumConstant::new("FRONT", "CUBEMAP_LAYER_FRONT", CubeMapLayer::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "CUBEMAP_LAYER_BACK", CubeMapLayer::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for CubeMapLayer {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Cubemap Layer Left", 0i64), EnumeratorShape::new_int("Cubemap Layer Right", 1i64), EnumeratorShape::new_int("Cubemap Layer Bottom", 2i64), EnumeratorShape::new_int("Cubemap Layer Top", 3i64), EnumeratorShape::new_int("Cubemap Layer Front", 4i64), EnumeratorShape::new_int("Cubemap Layer Back", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CubeMapLayer")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CubeMapLayer {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CubeMapLayer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CubeMapLayer {
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
impl crate::registry::property::Export for CubeMapLayer {
    
}
impl crate::meta::Element for CubeMapLayer {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderMode {
    ord: i32
}
impl ShaderMode {
    #[doc(alias = "SHADER_SPATIAL")]
    #[doc = "Godot enumerator name: `SHADER_SPATIAL`"]
    pub const SPATIAL: ShaderMode = ShaderMode {
        ord: 0i32
    };
    #[doc(alias = "SHADER_CANVAS_ITEM")]
    #[doc = "Godot enumerator name: `SHADER_CANVAS_ITEM`"]
    pub const CANVAS_ITEM: ShaderMode = ShaderMode {
        ord: 1i32
    };
    #[doc(alias = "SHADER_PARTICLES")]
    #[doc = "Godot enumerator name: `SHADER_PARTICLES`"]
    pub const PARTICLES: ShaderMode = ShaderMode {
        ord: 2i32
    };
    #[doc(alias = "SHADER_SKY")]
    #[doc = "Godot enumerator name: `SHADER_SKY`"]
    pub const SKY: ShaderMode = ShaderMode {
        ord: 3i32
    };
    #[doc(alias = "SHADER_FOG")]
    #[doc = "Godot enumerator name: `SHADER_FOG`"]
    pub const FOG: ShaderMode = ShaderMode {
        ord: 4i32
    };
    #[doc(alias = "SHADER_MAX")]
    #[doc = "Godot enumerator name: `SHADER_MAX`"]
    pub const MAX: ShaderMode = ShaderMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ShaderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderMode {
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
            Self::SPATIAL => "SPATIAL", Self::CANVAS_ITEM => "CANVAS_ITEM", Self::PARTICLES => "PARTICLES", Self::SKY => "SKY", Self::FOG => "FOG", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShaderMode::SPATIAL, ShaderMode::CANVAS_ITEM, ShaderMode::PARTICLES, ShaderMode::SKY, ShaderMode::FOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShaderMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SPATIAL", "SHADER_SPATIAL", ShaderMode::SPATIAL), crate::meta::inspect::EnumConstant::new("CANVAS_ITEM", "SHADER_CANVAS_ITEM", ShaderMode::CANVAS_ITEM), crate::meta::inspect::EnumConstant::new("PARTICLES", "SHADER_PARTICLES", ShaderMode::PARTICLES), crate::meta::inspect::EnumConstant::new("SKY", "SHADER_SKY", ShaderMode::SKY), crate::meta::inspect::EnumConstant::new("FOG", "SHADER_FOG", ShaderMode::FOG), crate::meta::inspect::EnumConstant::new("MAX", "SHADER_MAX", ShaderMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ShaderMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ShaderMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shader Spatial", 0i64), EnumeratorShape::new_int("Shader Canvas Item", 1i64), EnumeratorShape::new_int("Shader Particles", 2i64), EnumeratorShape::new_int("Shader Sky", 3i64), EnumeratorShape::new_int("Shader Fog", 4i64), EnumeratorShape::new_int("Shader Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ShaderMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ShaderMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ShaderMode {
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
impl crate::registry::property::Export for ShaderMode {
    
}
impl crate::meta::Element for ShaderMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    ord: i32
}
impl ArrayType {
    #[doc(alias = "ARRAY_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_VERTEX`"]
    pub const VERTEX: ArrayType = ArrayType {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_NORMAL`"]
    pub const NORMAL: ArrayType = ArrayType {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_TANGENT`"]
    pub const TANGENT: ArrayType = ArrayType {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_COLOR`"]
    pub const COLOR: ArrayType = ArrayType {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV`"]
    pub const TEX_UV: ArrayType = ArrayType {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV2`"]
    pub const TEX_UV2: ArrayType = ArrayType {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM0`"]
    pub const CUSTOM0: ArrayType = ArrayType {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM1`"]
    pub const CUSTOM1: ArrayType = ArrayType {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM2`"]
    pub const CUSTOM2: ArrayType = ArrayType {
        ord: 8i32
    };
    #[doc(alias = "ARRAY_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM3`"]
    pub const CUSTOM3: ArrayType = ArrayType {
        ord: 9i32
    };
    #[doc(alias = "ARRAY_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_BONES`"]
    pub const BONES: ArrayType = ArrayType {
        ord: 10i32
    };
    #[doc(alias = "ARRAY_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_WEIGHTS`"]
    pub const WEIGHTS: ArrayType = ArrayType {
        ord: 11i32
    };
    #[doc(alias = "ARRAY_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_INDEX`"]
    pub const INDEX: ArrayType = ArrayType {
        ord: 12i32
    };
    #[doc(alias = "ARRAY_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_MAX`"]
    pub const MAX: ArrayType = ArrayType {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayType {
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
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayType::VERTEX, ArrayType::NORMAL, ArrayType::TANGENT, ArrayType::COLOR, ArrayType::TEX_UV, ArrayType::TEX_UV2, ArrayType::CUSTOM0, ArrayType::CUSTOM1, ArrayType::CUSTOM2, ArrayType::CUSTOM3, ArrayType::BONES, ArrayType::WEIGHTS, ArrayType::INDEX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_VERTEX", ArrayType::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_NORMAL", ArrayType::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_TANGENT", ArrayType::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_COLOR", ArrayType::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_TEX_UV", ArrayType::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_TEX_UV2", ArrayType::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_CUSTOM0", ArrayType::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_CUSTOM1", ArrayType::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_CUSTOM2", ArrayType::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_CUSTOM3", ArrayType::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_BONES", ArrayType::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_WEIGHTS", ArrayType::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_INDEX", ArrayType::INDEX), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_MAX", ArrayType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayType {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for ArrayType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Vertex", 0i64), EnumeratorShape::new_int("Array Normal", 1i64), EnumeratorShape::new_int("Array Tangent", 2i64), EnumeratorShape::new_int("Array Color", 3i64), EnumeratorShape::new_int("Array Tex Uv", 4i64), EnumeratorShape::new_int("Array Tex Uv2", 5i64), EnumeratorShape::new_int("Array Custom0", 6i64), EnumeratorShape::new_int("Array Custom1", 7i64), EnumeratorShape::new_int("Array Custom2", 8i64), EnumeratorShape::new_int("Array Custom3", 9i64), EnumeratorShape::new_int("Array Bones", 10i64), EnumeratorShape::new_int("Array Weights", 11i64), EnumeratorShape::new_int("Array Index", 12i64), EnumeratorShape::new_int("Array Max", 13i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ArrayType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ArrayType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayType {
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
impl crate::registry::property::Export for ArrayType {
    
}
impl crate::meta::Element for ArrayType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayCustomFormat {
    ord: i32
}
impl ArrayCustomFormat {
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_HALF`"]
    pub const RG_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_MAX`"]
    pub const MAX: ArrayCustomFormat = ArrayCustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ArrayCustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayCustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayCustomFormat::RGBA8_UNORM, ArrayCustomFormat::RGBA8_SNORM, ArrayCustomFormat::RG_HALF, ArrayCustomFormat::RGBA_HALF, ArrayCustomFormat::R_FLOAT, ArrayCustomFormat::RG_FLOAT, ArrayCustomFormat::RGB_FLOAT, ArrayCustomFormat::RGBA_FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayCustomFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA8_UNORM", "ARRAY_CUSTOM_RGBA8_UNORM", ArrayCustomFormat::RGBA8_UNORM), crate::meta::inspect::EnumConstant::new("RGBA8_SNORM", "ARRAY_CUSTOM_RGBA8_SNORM", ArrayCustomFormat::RGBA8_SNORM), crate::meta::inspect::EnumConstant::new("RG_HALF", "ARRAY_CUSTOM_RG_HALF", ArrayCustomFormat::RG_HALF), crate::meta::inspect::EnumConstant::new("RGBA_HALF", "ARRAY_CUSTOM_RGBA_HALF", ArrayCustomFormat::RGBA_HALF), crate::meta::inspect::EnumConstant::new("R_FLOAT", "ARRAY_CUSTOM_R_FLOAT", ArrayCustomFormat::R_FLOAT), crate::meta::inspect::EnumConstant::new("RG_FLOAT", "ARRAY_CUSTOM_RG_FLOAT", ArrayCustomFormat::RG_FLOAT), crate::meta::inspect::EnumConstant::new("RGB_FLOAT", "ARRAY_CUSTOM_RGB_FLOAT", ArrayCustomFormat::RGB_FLOAT), crate::meta::inspect::EnumConstant::new("RGBA_FLOAT", "ARRAY_CUSTOM_RGBA_FLOAT", ArrayCustomFormat::RGBA_FLOAT), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_CUSTOM_MAX", ArrayCustomFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayCustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for ArrayCustomFormat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Custom Rgba8 Unorm", 0i64), EnumeratorShape::new_int("Array Custom Rgba8 Snorm", 1i64), EnumeratorShape::new_int("Array Custom Rg Half", 2i64), EnumeratorShape::new_int("Array Custom Rgba Half", 3i64), EnumeratorShape::new_int("Array Custom R Float", 4i64), EnumeratorShape::new_int("Array Custom Rg Float", 5i64), EnumeratorShape::new_int("Array Custom Rgb Float", 6i64), EnumeratorShape::new_int("Array Custom Rgba Float", 7i64), EnumeratorShape::new_int("Array Custom Max", 8i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ArrayCustomFormat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ArrayCustomFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayCustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayCustomFormat {
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
impl crate::registry::property::Export for ArrayCustomFormat {
    
}
impl crate::meta::Element for ArrayCustomFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ArrayFormat {
    ord: u64
}
impl ArrayFormat {
    #[doc(alias = "ARRAY_FORMAT_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_VERTEX`"]
    pub const VERTEX: ArrayFormat = ArrayFormat {
        ord: 1u64
    };
    #[doc(alias = "ARRAY_FORMAT_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_NORMAL`"]
    pub const NORMAL: ArrayFormat = ArrayFormat {
        ord: 2u64
    };
    #[doc(alias = "ARRAY_FORMAT_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TANGENT`"]
    pub const TANGENT: ArrayFormat = ArrayFormat {
        ord: 4u64
    };
    #[doc(alias = "ARRAY_FORMAT_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_COLOR`"]
    pub const COLOR: ArrayFormat = ArrayFormat {
        ord: 8u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV`"]
    pub const TEX_UV: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV2`"]
    pub const TEX_UV2: ArrayFormat = ArrayFormat {
        ord: 32u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0`"]
    pub const CUSTOM0: ArrayFormat = ArrayFormat {
        ord: 64u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1`"]
    pub const CUSTOM1: ArrayFormat = ArrayFormat {
        ord: 128u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2`"]
    pub const CUSTOM2: ArrayFormat = ArrayFormat {
        ord: 256u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3`"]
    pub const CUSTOM3: ArrayFormat = ArrayFormat {
        ord: 512u64
    };
    #[doc(alias = "ARRAY_FORMAT_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BONES`"]
    pub const BONES: ArrayFormat = ArrayFormat {
        ord: 1024u64
    };
    #[doc(alias = "ARRAY_FORMAT_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_WEIGHTS`"]
    pub const WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 2048u64
    };
    #[doc(alias = "ARRAY_FORMAT_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_INDEX`"]
    pub const INDEX: ArrayFormat = ArrayFormat {
        ord: 4096u64
    };
    #[doc(alias = "ARRAY_FORMAT_BLEND_SHAPE_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BLEND_SHAPE_MASK`"]
    pub const BLEND_SHAPE_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BASE`"]
    pub const CUSTOM_BASE: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BITS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BITS`"]
    pub const CUSTOM_BITS: ArrayFormat = ArrayFormat {
        ord: 3u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0_SHIFT`"]
    pub const CUSTOM0_SHIFT: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1_SHIFT`"]
    pub const CUSTOM1_SHIFT: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2_SHIFT`"]
    pub const CUSTOM2_SHIFT: ArrayFormat = ArrayFormat {
        ord: 19u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3_SHIFT`"]
    pub const CUSTOM3_SHIFT: ArrayFormat = ArrayFormat {
        ord: 22u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_MASK`"]
    pub const CUSTOM_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_COMPRESS_FLAGS_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_COMPRESS_FLAGS_BASE`"]
    pub const COMPRESS_FLAGS_BASE: ArrayFormat = ArrayFormat {
        ord: 25u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_2D_VERTICES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_2D_VERTICES`"]
    pub const FLAG_USE_2D_VERTICES: ArrayFormat = ArrayFormat {
        ord: 33554432u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_DYNAMIC_UPDATE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_DYNAMIC_UPDATE`"]
    pub const FLAG_USE_DYNAMIC_UPDATE: ArrayFormat = ArrayFormat {
        ord: 67108864u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_8_BONE_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_8_BONE_WEIGHTS`"]
    pub const FLAG_USE_8_BONE_WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 134217728u64
    };
    #[doc(alias = "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY`"]
    pub const FLAG_USES_EMPTY_VERTEX_ARRAY: ArrayFormat = ArrayFormat {
        ord: 268435456u64
    };
    #[doc(alias = "ARRAY_FLAG_COMPRESS_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_COMPRESS_ATTRIBUTES`"]
    pub const FLAG_COMPRESS_ATTRIBUTES: ArrayFormat = ArrayFormat {
        ord: 536870912u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_BASE`"]
    pub const FLAG_FORMAT_VERSION_BASE: ArrayFormat = ArrayFormat {
        ord: 35u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_SHIFT`"]
    pub const FLAG_FORMAT_VERSION_SHIFT: ArrayFormat = ArrayFormat {
        ord: 35u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_1")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_1`"]
    pub const FLAG_FORMAT_VERSION_1: ArrayFormat = ArrayFormat {
        ord: 0u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_2")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_2`"]
    pub const FLAG_FORMAT_VERSION_2: ArrayFormat = ArrayFormat {
        ord: 34359738368u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_CURRENT_VERSION")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_CURRENT_VERSION`"]
    pub const FLAG_FORMAT_CURRENT_VERSION: ArrayFormat = ArrayFormat {
        ord: 34359738368u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_MASK`"]
    pub const FLAG_FORMAT_VERSION_MASK: ArrayFormat = ArrayFormat {
        ord: 255u64
    };
    
}
impl std::fmt::Debug for ArrayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        crate::classes::debug_bitfield(* self, f)
    }
}
impl crate::obj::EngineBitfield for ArrayFormat {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_FORMAT_VERTEX", ArrayFormat::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_FORMAT_NORMAL", ArrayFormat::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_FORMAT_TANGENT", ArrayFormat::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_FORMAT_COLOR", ArrayFormat::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_FORMAT_TEX_UV", ArrayFormat::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_FORMAT_TEX_UV2", ArrayFormat::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_FORMAT_CUSTOM0", ArrayFormat::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_FORMAT_CUSTOM1", ArrayFormat::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_FORMAT_CUSTOM2", ArrayFormat::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_FORMAT_CUSTOM3", ArrayFormat::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_FORMAT_BONES", ArrayFormat::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_FORMAT_WEIGHTS", ArrayFormat::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_FORMAT_INDEX", ArrayFormat::INDEX), crate::meta::inspect::EnumConstant::new("BLEND_SHAPE_MASK", "ARRAY_FORMAT_BLEND_SHAPE_MASK", ArrayFormat::BLEND_SHAPE_MASK), crate::meta::inspect::EnumConstant::new("CUSTOM_BASE", "ARRAY_FORMAT_CUSTOM_BASE", ArrayFormat::CUSTOM_BASE), crate::meta::inspect::EnumConstant::new("CUSTOM_BITS", "ARRAY_FORMAT_CUSTOM_BITS", ArrayFormat::CUSTOM_BITS), crate::meta::inspect::EnumConstant::new("CUSTOM0_SHIFT", "ARRAY_FORMAT_CUSTOM0_SHIFT", ArrayFormat::CUSTOM0_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM1_SHIFT", "ARRAY_FORMAT_CUSTOM1_SHIFT", ArrayFormat::CUSTOM1_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM2_SHIFT", "ARRAY_FORMAT_CUSTOM2_SHIFT", ArrayFormat::CUSTOM2_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM3_SHIFT", "ARRAY_FORMAT_CUSTOM3_SHIFT", ArrayFormat::CUSTOM3_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM_MASK", "ARRAY_FORMAT_CUSTOM_MASK", ArrayFormat::CUSTOM_MASK), crate::meta::inspect::EnumConstant::new("COMPRESS_FLAGS_BASE", "ARRAY_COMPRESS_FLAGS_BASE", ArrayFormat::COMPRESS_FLAGS_BASE), crate::meta::inspect::EnumConstant::new("FLAG_USE_2D_VERTICES", "ARRAY_FLAG_USE_2D_VERTICES", ArrayFormat::FLAG_USE_2D_VERTICES), crate::meta::inspect::EnumConstant::new("FLAG_USE_DYNAMIC_UPDATE", "ARRAY_FLAG_USE_DYNAMIC_UPDATE", ArrayFormat::FLAG_USE_DYNAMIC_UPDATE), crate::meta::inspect::EnumConstant::new("FLAG_USE_8_BONE_WEIGHTS", "ARRAY_FLAG_USE_8_BONE_WEIGHTS", ArrayFormat::FLAG_USE_8_BONE_WEIGHTS), crate::meta::inspect::EnumConstant::new("FLAG_USES_EMPTY_VERTEX_ARRAY", "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY", ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY), crate::meta::inspect::EnumConstant::new("FLAG_COMPRESS_ATTRIBUTES", "ARRAY_FLAG_COMPRESS_ATTRIBUTES", ArrayFormat::FLAG_COMPRESS_ATTRIBUTES), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_BASE", "ARRAY_FLAG_FORMAT_VERSION_BASE", ArrayFormat::FLAG_FORMAT_VERSION_BASE), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_SHIFT", "ARRAY_FLAG_FORMAT_VERSION_SHIFT", ArrayFormat::FLAG_FORMAT_VERSION_SHIFT), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_1", "ARRAY_FLAG_FORMAT_VERSION_1", ArrayFormat::FLAG_FORMAT_VERSION_1), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_2", "ARRAY_FLAG_FORMAT_VERSION_2", ArrayFormat::FLAG_FORMAT_VERSION_2), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_CURRENT_VERSION", "ARRAY_FLAG_FORMAT_CURRENT_VERSION", ArrayFormat::FLAG_FORMAT_CURRENT_VERSION), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_MASK", "ARRAY_FLAG_FORMAT_VERSION_MASK", ArrayFormat::FLAG_FORMAT_VERSION_MASK)]
        }
    }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ArrayFormat {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ArrayFormat {
    type Via = u64;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Array Format Vertex", 1i64), EnumeratorShape::new_int("Array Format Normal", 2i64), EnumeratorShape::new_int("Array Format Tangent", 4i64), EnumeratorShape::new_int("Array Format Color", 8i64), EnumeratorShape::new_int("Array Format Tex Uv", 16i64), EnumeratorShape::new_int("Array Format Tex Uv2", 32i64), EnumeratorShape::new_int("Array Format Custom0", 64i64), EnumeratorShape::new_int("Array Format Custom1", 128i64), EnumeratorShape::new_int("Array Format Custom2", 256i64), EnumeratorShape::new_int("Array Format Custom3", 512i64), EnumeratorShape::new_int("Array Format Bones", 1024i64), EnumeratorShape::new_int("Array Format Weights", 2048i64), EnumeratorShape::new_int("Array Format Index", 4096i64), EnumeratorShape::new_int("Array Format Blend Shape Mask", 7i64), EnumeratorShape::new_int("Array Format Custom Base", 13i64), EnumeratorShape::new_int("Array Format Custom Bits", 3i64), EnumeratorShape::new_int("Array Format Custom0 Shift", 13i64), EnumeratorShape::new_int("Array Format Custom1 Shift", 16i64), EnumeratorShape::new_int("Array Format Custom2 Shift", 19i64), EnumeratorShape::new_int("Array Format Custom3 Shift", 22i64), EnumeratorShape::new_int("Array Format Custom Mask", 7i64), EnumeratorShape::new_int("Array Compress Flags Base", 25i64), EnumeratorShape::new_int("Array Flag Use 2d Vertices", 33554432i64), EnumeratorShape::new_int("Array Flag Use Dynamic Update", 67108864i64), EnumeratorShape::new_int("Array Flag Use 8 Bone Weights", 134217728i64), EnumeratorShape::new_int("Array Flag Uses Empty Vertex Array", 268435456i64), EnumeratorShape::new_int("Array Flag Compress Attributes", 536870912i64), EnumeratorShape::new_int("Array Flag Format Version Base", 35i64), EnumeratorShape::new_int("Array Flag Format Version Shift", 35i64), EnumeratorShape::new_int("Array Flag Format Version 1", 0i64), EnumeratorShape::new_int("Array Flag Format Version 2", 34359738368i64), EnumeratorShape::new_int("Array Flag Format Current Version", 34359738368i64), EnumeratorShape::new_int("Array Flag Format Version Mask", 255i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ArrayFormat")), is_bitfield: true,
        }
    }
}
impl crate::meta::ToGodot for ArrayFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ArrayFormat {
    type PubType = Self;
    fn var_get(field: &Self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* field)
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
impl crate::registry::property::Export for ArrayFormat {
    
}
impl crate::meta::Element for ArrayFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrimitiveType {
    ord: i32
}
impl PrimitiveType {
    #[doc(alias = "PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `PRIMITIVE_POINTS`"]
    pub const POINTS: PrimitiveType = PrimitiveType {
        ord: 0i32
    };
    #[doc(alias = "PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINES`"]
    pub const LINES: PrimitiveType = PrimitiveType {
        ord: 1i32
    };
    #[doc(alias = "PRIMITIVE_LINE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINE_STRIP`"]
    pub const LINE_STRIP: PrimitiveType = PrimitiveType {
        ord: 2i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: PrimitiveType = PrimitiveType {
        ord: 3i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLE_STRIP`"]
    pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType {
        ord: 4i32
    };
    #[doc(alias = "PRIMITIVE_MAX")]
    #[doc = "Godot enumerator name: `PRIMITIVE_MAX`"]
    pub const MAX: PrimitiveType = PrimitiveType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PrimitiveType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PrimitiveType {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINE_STRIP => "LINE_STRIP", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLE_STRIP => "TRIANGLE_STRIP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PrimitiveType::POINTS, PrimitiveType::LINES, PrimitiveType::LINE_STRIP, PrimitiveType::TRIANGLES, PrimitiveType::TRIANGLE_STRIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PrimitiveType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINTS", "PRIMITIVE_POINTS", PrimitiveType::POINTS), crate::meta::inspect::EnumConstant::new("LINES", "PRIMITIVE_LINES", PrimitiveType::LINES), crate::meta::inspect::EnumConstant::new("LINE_STRIP", "PRIMITIVE_LINE_STRIP", PrimitiveType::LINE_STRIP), crate::meta::inspect::EnumConstant::new("TRIANGLES", "PRIMITIVE_TRIANGLES", PrimitiveType::TRIANGLES), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIP", "PRIMITIVE_TRIANGLE_STRIP", PrimitiveType::TRIANGLE_STRIP), crate::meta::inspect::EnumConstant::new("MAX", "PRIMITIVE_MAX", PrimitiveType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PrimitiveType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for PrimitiveType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Primitive Points", 0i64), EnumeratorShape::new_int("Primitive Lines", 1i64), EnumeratorShape::new_int("Primitive Line Strip", 2i64), EnumeratorShape::new_int("Primitive Triangles", 3i64), EnumeratorShape::new_int("Primitive Triangle Strip", 4i64), EnumeratorShape::new_int("Primitive Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.PrimitiveType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PrimitiveType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PrimitiveType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PrimitiveType {
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
impl crate::registry::property::Export for PrimitiveType {
    
}
impl crate::meta::Element for PrimitiveType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendShapeMode {
    ord: i32
}
impl BlendShapeMode {
    #[doc(alias = "BLEND_SHAPE_MODE_NORMALIZED")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_NORMALIZED`"]
    pub const NORMALIZED: BlendShapeMode = BlendShapeMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_SHAPE_MODE_RELATIVE")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_RELATIVE`"]
    pub const RELATIVE: BlendShapeMode = BlendShapeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BlendShapeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendShapeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendShapeMode {
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
            Self::NORMALIZED => "NORMALIZED", Self::RELATIVE => "RELATIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendShapeMode::NORMALIZED, BlendShapeMode::RELATIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendShapeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMALIZED", "BLEND_SHAPE_MODE_NORMALIZED", BlendShapeMode::NORMALIZED), crate::meta::inspect::EnumConstant::new("RELATIVE", "BLEND_SHAPE_MODE_RELATIVE", BlendShapeMode::RELATIVE)]
        }
    }
}
impl crate::meta::GodotConvert for BlendShapeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Blend Shape Mode Normalized", 0i64), EnumeratorShape::new_int("Blend Shape Mode Relative", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.BlendShapeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BlendShapeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendShapeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BlendShapeMode {
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
impl crate::registry::property::Export for BlendShapeMode {
    
}
impl crate::meta::Element for BlendShapeMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MultimeshTransformFormat {
    ord: i32
}
impl MultimeshTransformFormat {
    #[doc(alias = "MULTIMESH_TRANSFORM_2D")]
    #[doc = "Godot enumerator name: `MULTIMESH_TRANSFORM_2D`"]
    pub const TRANSFORM_2D: MultimeshTransformFormat = MultimeshTransformFormat {
        ord: 0i32
    };
    #[doc(alias = "MULTIMESH_TRANSFORM_3D")]
    #[doc = "Godot enumerator name: `MULTIMESH_TRANSFORM_3D`"]
    pub const TRANSFORM_3D: MultimeshTransformFormat = MultimeshTransformFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MultimeshTransformFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MultimeshTransformFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MultimeshTransformFormat {
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
            Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM_3D => "TRANSFORM_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MultimeshTransformFormat::TRANSFORM_2D, MultimeshTransformFormat::TRANSFORM_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MultimeshTransformFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TRANSFORM_2D", "MULTIMESH_TRANSFORM_2D", MultimeshTransformFormat::TRANSFORM_2D), crate::meta::inspect::EnumConstant::new("TRANSFORM_3D", "MULTIMESH_TRANSFORM_3D", MultimeshTransformFormat::TRANSFORM_3D)]
        }
    }
}
impl crate::meta::GodotConvert for MultimeshTransformFormat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Multimesh Transform 2d", 0i64), EnumeratorShape::new_int("Multimesh Transform 3d", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.MultimeshTransformFormat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MultimeshTransformFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MultimeshTransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MultimeshTransformFormat {
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
impl crate::registry::property::Export for MultimeshTransformFormat {
    
}
impl crate::meta::Element for MultimeshTransformFormat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MultimeshPhysicsInterpolationQuality {
    ord: i32
}
impl MultimeshPhysicsInterpolationQuality {
    #[doc(alias = "MULTIMESH_INTERP_QUALITY_FAST")]
    #[doc = "Godot enumerator name: `MULTIMESH_INTERP_QUALITY_FAST`"]
    pub const FAST: MultimeshPhysicsInterpolationQuality = MultimeshPhysicsInterpolationQuality {
        ord: 0i32
    };
    #[doc(alias = "MULTIMESH_INTERP_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `MULTIMESH_INTERP_QUALITY_HIGH`"]
    pub const HIGH: MultimeshPhysicsInterpolationQuality = MultimeshPhysicsInterpolationQuality {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MultimeshPhysicsInterpolationQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MultimeshPhysicsInterpolationQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MultimeshPhysicsInterpolationQuality {
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
            Self::FAST => "FAST", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MultimeshPhysicsInterpolationQuality::FAST, MultimeshPhysicsInterpolationQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MultimeshPhysicsInterpolationQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FAST", "MULTIMESH_INTERP_QUALITY_FAST", MultimeshPhysicsInterpolationQuality::FAST), crate::meta::inspect::EnumConstant::new("HIGH", "MULTIMESH_INTERP_QUALITY_HIGH", MultimeshPhysicsInterpolationQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for MultimeshPhysicsInterpolationQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Multimesh Interp Quality Fast", 0i64), EnumeratorShape::new_int("Multimesh Interp Quality High", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.MultimeshPhysicsInterpolationQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for MultimeshPhysicsInterpolationQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MultimeshPhysicsInterpolationQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for MultimeshPhysicsInterpolationQuality {
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
impl crate::registry::property::Export for MultimeshPhysicsInterpolationQuality {
    
}
impl crate::meta::Element for MultimeshPhysicsInterpolationQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightProjectorFilter {
    ord: i32
}
impl LightProjectorFilter {
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST`"]
    pub const NEAREST: LightProjectorFilter = LightProjectorFilter {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR`"]
    pub const LINEAR: LightProjectorFilter = LightProjectorFilter {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS`"]
    pub const NEAREST_MIPMAPS: LightProjectorFilter = LightProjectorFilter {
        ord: 2i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS`"]
    pub const LINEAR_MIPMAPS: LightProjectorFilter = LightProjectorFilter {
        ord: 3i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_MIPMAPS_ANISOTROPIC: LightProjectorFilter = LightProjectorFilter {
        ord: 4i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_MIPMAPS_ANISOTROPIC: LightProjectorFilter = LightProjectorFilter {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for LightProjectorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightProjectorFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightProjectorFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAPS => "NEAREST_MIPMAPS", Self::LINEAR_MIPMAPS => "LINEAR_MIPMAPS", Self::NEAREST_MIPMAPS_ANISOTROPIC => "NEAREST_MIPMAPS_ANISOTROPIC", Self::LINEAR_MIPMAPS_ANISOTROPIC => "LINEAR_MIPMAPS_ANISOTROPIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightProjectorFilter::NEAREST, LightProjectorFilter::LINEAR, LightProjectorFilter::NEAREST_MIPMAPS, LightProjectorFilter::LINEAR_MIPMAPS, LightProjectorFilter::NEAREST_MIPMAPS_ANISOTROPIC, LightProjectorFilter::LINEAR_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightProjectorFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "LIGHT_PROJECTOR_FILTER_NEAREST", LightProjectorFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "LIGHT_PROJECTOR_FILTER_LINEAR", LightProjectorFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS", "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS", LightProjectorFilter::NEAREST_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS", "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS", LightProjectorFilter::LINEAR_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS_ANISOTROPIC", "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC", LightProjectorFilter::NEAREST_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS_ANISOTROPIC", "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC", LightProjectorFilter::LINEAR_MIPMAPS_ANISOTROPIC)]
        }
    }
}
impl crate::meta::GodotConvert for LightProjectorFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Projector Filter Nearest", 0i64), EnumeratorShape::new_int("Light Projector Filter Linear", 1i64), EnumeratorShape::new_int("Light Projector Filter Nearest Mipmaps", 2i64), EnumeratorShape::new_int("Light Projector Filter Linear Mipmaps", 3i64), EnumeratorShape::new_int("Light Projector Filter Nearest Mipmaps Anisotropic", 4i64), EnumeratorShape::new_int("Light Projector Filter Linear Mipmaps Anisotropic", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightProjectorFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightProjectorFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightProjectorFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightProjectorFilter {
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
impl crate::registry::property::Export for LightProjectorFilter {
    
}
impl crate::meta::Element for LightProjectorFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightType {
    ord: i32
}
impl LightType {
    #[doc(alias = "LIGHT_DIRECTIONAL")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL`"]
    pub const DIRECTIONAL: LightType = LightType {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_OMNI")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI`"]
    pub const OMNI: LightType = LightType {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_SPOT")]
    #[doc = "Godot enumerator name: `LIGHT_SPOT`"]
    pub const SPOT: LightType = LightType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightType {
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
            Self::DIRECTIONAL => "DIRECTIONAL", Self::OMNI => "OMNI", Self::SPOT => "SPOT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightType::DIRECTIONAL, LightType::OMNI, LightType::SPOT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DIRECTIONAL", "LIGHT_DIRECTIONAL", LightType::DIRECTIONAL), crate::meta::inspect::EnumConstant::new("OMNI", "LIGHT_OMNI", LightType::OMNI), crate::meta::inspect::EnumConstant::new("SPOT", "LIGHT_SPOT", LightType::SPOT)]
        }
    }
}
impl crate::meta::GodotConvert for LightType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Directional", 0i64), EnumeratorShape::new_int("Light Omni", 1i64), EnumeratorShape::new_int("Light Spot", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightType {
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
impl crate::registry::property::Export for LightType {
    
}
impl crate::meta::Element for LightType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightParam {
    ord: i32
}
impl LightParam {
    #[doc(alias = "LIGHT_PARAM_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_ENERGY`"]
    pub const ENERGY: LightParam = LightParam {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_PARAM_INDIRECT_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_INDIRECT_ENERGY`"]
    pub const INDIRECT_ENERGY: LightParam = LightParam {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY`"]
    pub const VOLUMETRIC_FOG_ENERGY: LightParam = LightParam {
        ord: 2i32
    };
    #[doc(alias = "LIGHT_PARAM_SPECULAR")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPECULAR`"]
    pub const SPECULAR: LightParam = LightParam {
        ord: 3i32
    };
    #[doc(alias = "LIGHT_PARAM_RANGE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_RANGE`"]
    pub const RANGE: LightParam = LightParam {
        ord: 4i32
    };
    #[doc(alias = "LIGHT_PARAM_SIZE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SIZE`"]
    pub const SIZE: LightParam = LightParam {
        ord: 5i32
    };
    #[doc(alias = "LIGHT_PARAM_ATTENUATION")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_ATTENUATION`"]
    pub const ATTENUATION: LightParam = LightParam {
        ord: 6i32
    };
    #[doc(alias = "LIGHT_PARAM_SPOT_ANGLE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPOT_ANGLE`"]
    pub const SPOT_ANGLE: LightParam = LightParam {
        ord: 7i32
    };
    #[doc(alias = "LIGHT_PARAM_SPOT_ATTENUATION")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPOT_ATTENUATION`"]
    pub const SPOT_ATTENUATION: LightParam = LightParam {
        ord: 8i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_MAX_DISTANCE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_MAX_DISTANCE`"]
    pub const SHADOW_MAX_DISTANCE: LightParam = LightParam {
        ord: 9i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET`"]
    pub const SHADOW_SPLIT_1_OFFSET: LightParam = LightParam {
        ord: 10i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET`"]
    pub const SHADOW_SPLIT_2_OFFSET: LightParam = LightParam {
        ord: 11i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET`"]
    pub const SHADOW_SPLIT_3_OFFSET: LightParam = LightParam {
        ord: 12i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_FADE_START")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_FADE_START`"]
    pub const SHADOW_FADE_START: LightParam = LightParam {
        ord: 13i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_NORMAL_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_NORMAL_BIAS`"]
    pub const SHADOW_NORMAL_BIAS: LightParam = LightParam {
        ord: 14i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_BIAS`"]
    pub const SHADOW_BIAS: LightParam = LightParam {
        ord: 15i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_PANCAKE_SIZE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_PANCAKE_SIZE`"]
    pub const SHADOW_PANCAKE_SIZE: LightParam = LightParam {
        ord: 16i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_OPACITY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_OPACITY`"]
    pub const SHADOW_OPACITY: LightParam = LightParam {
        ord: 17i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_BLUR")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_BLUR`"]
    pub const SHADOW_BLUR: LightParam = LightParam {
        ord: 18i32
    };
    #[doc(alias = "LIGHT_PARAM_TRANSMITTANCE_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_TRANSMITTANCE_BIAS`"]
    pub const TRANSMITTANCE_BIAS: LightParam = LightParam {
        ord: 19i32
    };
    #[doc(alias = "LIGHT_PARAM_INTENSITY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_INTENSITY`"]
    pub const INTENSITY: LightParam = LightParam {
        ord: 20i32
    };
    #[doc(alias = "LIGHT_PARAM_MAX")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_MAX`"]
    pub const MAX: LightParam = LightParam {
        ord: 21i32
    };
    
}
impl std::fmt::Debug for LightParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
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
            Self::ENERGY => "ENERGY", Self::INDIRECT_ENERGY => "INDIRECT_ENERGY", Self::VOLUMETRIC_FOG_ENERGY => "VOLUMETRIC_FOG_ENERGY", Self::SPECULAR => "SPECULAR", Self::RANGE => "RANGE", Self::SIZE => "SIZE", Self::ATTENUATION => "ATTENUATION", Self::SPOT_ANGLE => "SPOT_ANGLE", Self::SPOT_ATTENUATION => "SPOT_ATTENUATION", Self::SHADOW_MAX_DISTANCE => "SHADOW_MAX_DISTANCE", Self::SHADOW_SPLIT_1_OFFSET => "SHADOW_SPLIT_1_OFFSET", Self::SHADOW_SPLIT_2_OFFSET => "SHADOW_SPLIT_2_OFFSET", Self::SHADOW_SPLIT_3_OFFSET => "SHADOW_SPLIT_3_OFFSET", Self::SHADOW_FADE_START => "SHADOW_FADE_START", Self::SHADOW_NORMAL_BIAS => "SHADOW_NORMAL_BIAS", Self::SHADOW_BIAS => "SHADOW_BIAS", Self::SHADOW_PANCAKE_SIZE => "SHADOW_PANCAKE_SIZE", Self::SHADOW_OPACITY => "SHADOW_OPACITY", Self::SHADOW_BLUR => "SHADOW_BLUR", Self::TRANSMITTANCE_BIAS => "TRANSMITTANCE_BIAS", Self::INTENSITY => "INTENSITY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightParam::ENERGY, LightParam::INDIRECT_ENERGY, LightParam::VOLUMETRIC_FOG_ENERGY, LightParam::SPECULAR, LightParam::RANGE, LightParam::SIZE, LightParam::ATTENUATION, LightParam::SPOT_ANGLE, LightParam::SPOT_ATTENUATION, LightParam::SHADOW_MAX_DISTANCE, LightParam::SHADOW_SPLIT_1_OFFSET, LightParam::SHADOW_SPLIT_2_OFFSET, LightParam::SHADOW_SPLIT_3_OFFSET, LightParam::SHADOW_FADE_START, LightParam::SHADOW_NORMAL_BIAS, LightParam::SHADOW_BIAS, LightParam::SHADOW_PANCAKE_SIZE, LightParam::SHADOW_OPACITY, LightParam::SHADOW_BLUR, LightParam::TRANSMITTANCE_BIAS, LightParam::INTENSITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ENERGY", "LIGHT_PARAM_ENERGY", LightParam::ENERGY), crate::meta::inspect::EnumConstant::new("INDIRECT_ENERGY", "LIGHT_PARAM_INDIRECT_ENERGY", LightParam::INDIRECT_ENERGY), crate::meta::inspect::EnumConstant::new("VOLUMETRIC_FOG_ENERGY", "LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY", LightParam::VOLUMETRIC_FOG_ENERGY), crate::meta::inspect::EnumConstant::new("SPECULAR", "LIGHT_PARAM_SPECULAR", LightParam::SPECULAR), crate::meta::inspect::EnumConstant::new("RANGE", "LIGHT_PARAM_RANGE", LightParam::RANGE), crate::meta::inspect::EnumConstant::new("SIZE", "LIGHT_PARAM_SIZE", LightParam::SIZE), crate::meta::inspect::EnumConstant::new("ATTENUATION", "LIGHT_PARAM_ATTENUATION", LightParam::ATTENUATION), crate::meta::inspect::EnumConstant::new("SPOT_ANGLE", "LIGHT_PARAM_SPOT_ANGLE", LightParam::SPOT_ANGLE), crate::meta::inspect::EnumConstant::new("SPOT_ATTENUATION", "LIGHT_PARAM_SPOT_ATTENUATION", LightParam::SPOT_ATTENUATION), crate::meta::inspect::EnumConstant::new("SHADOW_MAX_DISTANCE", "LIGHT_PARAM_SHADOW_MAX_DISTANCE", LightParam::SHADOW_MAX_DISTANCE), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_1_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET", LightParam::SHADOW_SPLIT_1_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_2_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET", LightParam::SHADOW_SPLIT_2_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_3_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET", LightParam::SHADOW_SPLIT_3_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_FADE_START", "LIGHT_PARAM_SHADOW_FADE_START", LightParam::SHADOW_FADE_START), crate::meta::inspect::EnumConstant::new("SHADOW_NORMAL_BIAS", "LIGHT_PARAM_SHADOW_NORMAL_BIAS", LightParam::SHADOW_NORMAL_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_BIAS", "LIGHT_PARAM_SHADOW_BIAS", LightParam::SHADOW_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_PANCAKE_SIZE", "LIGHT_PARAM_SHADOW_PANCAKE_SIZE", LightParam::SHADOW_PANCAKE_SIZE), crate::meta::inspect::EnumConstant::new("SHADOW_OPACITY", "LIGHT_PARAM_SHADOW_OPACITY", LightParam::SHADOW_OPACITY), crate::meta::inspect::EnumConstant::new("SHADOW_BLUR", "LIGHT_PARAM_SHADOW_BLUR", LightParam::SHADOW_BLUR), crate::meta::inspect::EnumConstant::new("TRANSMITTANCE_BIAS", "LIGHT_PARAM_TRANSMITTANCE_BIAS", LightParam::TRANSMITTANCE_BIAS), crate::meta::inspect::EnumConstant::new("INTENSITY", "LIGHT_PARAM_INTENSITY", LightParam::INTENSITY), crate::meta::inspect::EnumConstant::new("MAX", "LIGHT_PARAM_MAX", LightParam::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LightParam {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::meta::GodotConvert for LightParam {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Param Energy", 0i64), EnumeratorShape::new_int("Light Param Indirect Energy", 1i64), EnumeratorShape::new_int("Light Param Volumetric Fog Energy", 2i64), EnumeratorShape::new_int("Light Param Specular", 3i64), EnumeratorShape::new_int("Light Param Range", 4i64), EnumeratorShape::new_int("Light Param Size", 5i64), EnumeratorShape::new_int("Light Param Attenuation", 6i64), EnumeratorShape::new_int("Light Param Spot Angle", 7i64), EnumeratorShape::new_int("Light Param Spot Attenuation", 8i64), EnumeratorShape::new_int("Light Param Shadow Max Distance", 9i64), EnumeratorShape::new_int("Light Param Shadow Split 1 Offset", 10i64), EnumeratorShape::new_int("Light Param Shadow Split 2 Offset", 11i64), EnumeratorShape::new_int("Light Param Shadow Split 3 Offset", 12i64), EnumeratorShape::new_int("Light Param Shadow Fade Start", 13i64), EnumeratorShape::new_int("Light Param Shadow Normal Bias", 14i64), EnumeratorShape::new_int("Light Param Shadow Bias", 15i64), EnumeratorShape::new_int("Light Param Shadow Pancake Size", 16i64), EnumeratorShape::new_int("Light Param Shadow Opacity", 17i64), EnumeratorShape::new_int("Light Param Shadow Blur", 18i64), EnumeratorShape::new_int("Light Param Transmittance Bias", 19i64), EnumeratorShape::new_int("Light Param Intensity", 20i64), EnumeratorShape::new_int("Light Param Max", 21i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightParam")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightParam {
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
impl crate::registry::property::Export for LightParam {
    
}
impl crate::meta::Element for LightParam {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightBakeMode {
    ord: i32
}
impl LightBakeMode {
    #[doc(alias = "LIGHT_BAKE_DISABLED")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_DISABLED`"]
    pub const DISABLED: LightBakeMode = LightBakeMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_BAKE_STATIC")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_STATIC`"]
    pub const STATIC: LightBakeMode = LightBakeMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_BAKE_DYNAMIC")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_DYNAMIC`"]
    pub const DYNAMIC: LightBakeMode = LightBakeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightBakeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightBakeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightBakeMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightBakeMode::DISABLED, LightBakeMode::STATIC, LightBakeMode::DYNAMIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightBakeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "LIGHT_BAKE_DISABLED", LightBakeMode::DISABLED), crate::meta::inspect::EnumConstant::new("STATIC", "LIGHT_BAKE_STATIC", LightBakeMode::STATIC), crate::meta::inspect::EnumConstant::new("DYNAMIC", "LIGHT_BAKE_DYNAMIC", LightBakeMode::DYNAMIC)]
        }
    }
}
impl crate::meta::GodotConvert for LightBakeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Bake Disabled", 0i64), EnumeratorShape::new_int("Light Bake Static", 1i64), EnumeratorShape::new_int("Light Bake Dynamic", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightBakeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightBakeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightBakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightBakeMode {
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
impl crate::registry::property::Export for LightBakeMode {
    
}
impl crate::meta::Element for LightBakeMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightOmniShadowMode {
    ord: i32
}
impl LightOmniShadowMode {
    #[doc(alias = "LIGHT_OMNI_SHADOW_DUAL_PARABOLOID")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI_SHADOW_DUAL_PARABOLOID`"]
    pub const DUAL_PARABOLOID: LightOmniShadowMode = LightOmniShadowMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_OMNI_SHADOW_CUBE")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI_SHADOW_CUBE`"]
    pub const CUBE: LightOmniShadowMode = LightOmniShadowMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LightOmniShadowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightOmniShadowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightOmniShadowMode {
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
            Self::DUAL_PARABOLOID => "DUAL_PARABOLOID", Self::CUBE => "CUBE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightOmniShadowMode::DUAL_PARABOLOID, LightOmniShadowMode::CUBE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightOmniShadowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DUAL_PARABOLOID", "LIGHT_OMNI_SHADOW_DUAL_PARABOLOID", LightOmniShadowMode::DUAL_PARABOLOID), crate::meta::inspect::EnumConstant::new("CUBE", "LIGHT_OMNI_SHADOW_CUBE", LightOmniShadowMode::CUBE)]
        }
    }
}
impl crate::meta::GodotConvert for LightOmniShadowMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Omni Shadow Dual Paraboloid", 0i64), EnumeratorShape::new_int("Light Omni Shadow Cube", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightOmniShadowMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightOmniShadowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightOmniShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightOmniShadowMode {
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
impl crate::registry::property::Export for LightOmniShadowMode {
    
}
impl crate::meta::Element for LightOmniShadowMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightDirectionalShadowMode {
    ord: i32
}
impl LightDirectionalShadowMode {
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL`"]
    pub const ORTHOGONAL: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS`"]
    pub const PARALLEL_2_SPLITS: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS`"]
    pub const PARALLEL_4_SPLITS: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightDirectionalShadowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightDirectionalShadowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightDirectionalShadowMode {
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
            Self::ORTHOGONAL => "ORTHOGONAL", Self::PARALLEL_2_SPLITS => "PARALLEL_2_SPLITS", Self::PARALLEL_4_SPLITS => "PARALLEL_4_SPLITS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightDirectionalShadowMode::ORTHOGONAL, LightDirectionalShadowMode::PARALLEL_2_SPLITS, LightDirectionalShadowMode::PARALLEL_4_SPLITS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightDirectionalShadowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ORTHOGONAL", "LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL", LightDirectionalShadowMode::ORTHOGONAL), crate::meta::inspect::EnumConstant::new("PARALLEL_2_SPLITS", "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS", LightDirectionalShadowMode::PARALLEL_2_SPLITS), crate::meta::inspect::EnumConstant::new("PARALLEL_4_SPLITS", "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS", LightDirectionalShadowMode::PARALLEL_4_SPLITS)]
        }
    }
}
impl crate::meta::GodotConvert for LightDirectionalShadowMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Directional Shadow Orthogonal", 0i64), EnumeratorShape::new_int("Light Directional Shadow Parallel 2 Splits", 1i64), EnumeratorShape::new_int("Light Directional Shadow Parallel 4 Splits", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightDirectionalShadowMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightDirectionalShadowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightDirectionalShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightDirectionalShadowMode {
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
impl crate::registry::property::Export for LightDirectionalShadowMode {
    
}
impl crate::meta::Element for LightDirectionalShadowMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightDirectionalSkyMode {
    ord: i32
}
impl LightDirectionalSkyMode {
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY`"]
    pub const LIGHT_AND_SKY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY`"]
    pub const LIGHT_ONLY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY`"]
    pub const SKY_ONLY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightDirectionalSkyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightDirectionalSkyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightDirectionalSkyMode {
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
            Self::LIGHT_AND_SKY => "LIGHT_AND_SKY", Self::LIGHT_ONLY => "LIGHT_ONLY", Self::SKY_ONLY => "SKY_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightDirectionalSkyMode::LIGHT_AND_SKY, LightDirectionalSkyMode::LIGHT_ONLY, LightDirectionalSkyMode::SKY_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightDirectionalSkyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LIGHT_AND_SKY", "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY", LightDirectionalSkyMode::LIGHT_AND_SKY), crate::meta::inspect::EnumConstant::new("LIGHT_ONLY", "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY", LightDirectionalSkyMode::LIGHT_ONLY), crate::meta::inspect::EnumConstant::new("SKY_ONLY", "LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY", LightDirectionalSkyMode::SKY_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for LightDirectionalSkyMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Light Directional Sky Mode Light And Sky", 0i64), EnumeratorShape::new_int("Light Directional Sky Mode Light Only", 1i64), EnumeratorShape::new_int("Light Directional Sky Mode Sky Only", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.LightDirectionalSkyMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LightDirectionalSkyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightDirectionalSkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LightDirectionalSkyMode {
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
impl crate::registry::property::Export for LightDirectionalSkyMode {
    
}
impl crate::meta::Element for LightDirectionalSkyMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowQuality {
    ord: i32
}
impl ShadowQuality {
    #[doc(alias = "SHADOW_QUALITY_HARD")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_HARD`"]
    pub const HARD: ShadowQuality = ShadowQuality {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_VERY_LOW")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_VERY_LOW`"]
    pub const SOFT_VERY_LOW: ShadowQuality = ShadowQuality {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_LOW")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_LOW`"]
    pub const SOFT_LOW: ShadowQuality = ShadowQuality {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_MEDIUM")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_MEDIUM`"]
    pub const SOFT_MEDIUM: ShadowQuality = ShadowQuality {
        ord: 3i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_HIGH")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_HIGH`"]
    pub const SOFT_HIGH: ShadowQuality = ShadowQuality {
        ord: 4i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_ULTRA")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_ULTRA`"]
    pub const SOFT_ULTRA: ShadowQuality = ShadowQuality {
        ord: 5i32
    };
    #[doc(alias = "SHADOW_QUALITY_MAX")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_MAX`"]
    pub const MAX: ShadowQuality = ShadowQuality {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ShadowQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowQuality {
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
            Self::HARD => "HARD", Self::SOFT_VERY_LOW => "SOFT_VERY_LOW", Self::SOFT_LOW => "SOFT_LOW", Self::SOFT_MEDIUM => "SOFT_MEDIUM", Self::SOFT_HIGH => "SOFT_HIGH", Self::SOFT_ULTRA => "SOFT_ULTRA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowQuality::HARD, ShadowQuality::SOFT_VERY_LOW, ShadowQuality::SOFT_LOW, ShadowQuality::SOFT_MEDIUM, ShadowQuality::SOFT_HIGH, ShadowQuality::SOFT_ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HARD", "SHADOW_QUALITY_HARD", ShadowQuality::HARD), crate::meta::inspect::EnumConstant::new("SOFT_VERY_LOW", "SHADOW_QUALITY_SOFT_VERY_LOW", ShadowQuality::SOFT_VERY_LOW), crate::meta::inspect::EnumConstant::new("SOFT_LOW", "SHADOW_QUALITY_SOFT_LOW", ShadowQuality::SOFT_LOW), crate::meta::inspect::EnumConstant::new("SOFT_MEDIUM", "SHADOW_QUALITY_SOFT_MEDIUM", ShadowQuality::SOFT_MEDIUM), crate::meta::inspect::EnumConstant::new("SOFT_HIGH", "SHADOW_QUALITY_SOFT_HIGH", ShadowQuality::SOFT_HIGH), crate::meta::inspect::EnumConstant::new("SOFT_ULTRA", "SHADOW_QUALITY_SOFT_ULTRA", ShadowQuality::SOFT_ULTRA), crate::meta::inspect::EnumConstant::new("MAX", "SHADOW_QUALITY_MAX", ShadowQuality::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ShadowQuality {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for ShadowQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shadow Quality Hard", 0i64), EnumeratorShape::new_int("Shadow Quality Soft Very Low", 1i64), EnumeratorShape::new_int("Shadow Quality Soft Low", 2i64), EnumeratorShape::new_int("Shadow Quality Soft Medium", 3i64), EnumeratorShape::new_int("Shadow Quality Soft High", 4i64), EnumeratorShape::new_int("Shadow Quality Soft Ultra", 5i64), EnumeratorShape::new_int("Shadow Quality Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ShadowQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ShadowQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ShadowQuality {
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
impl crate::registry::property::Export for ShadowQuality {
    
}
impl crate::meta::Element for ShadowQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReflectionProbeUpdateMode {
    ord: i32
}
impl ReflectionProbeUpdateMode {
    #[doc(alias = "REFLECTION_PROBE_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_UPDATE_ONCE`"]
    pub const ONCE: ReflectionProbeUpdateMode = ReflectionProbeUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "REFLECTION_PROBE_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_UPDATE_ALWAYS`"]
    pub const ALWAYS: ReflectionProbeUpdateMode = ReflectionProbeUpdateMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ReflectionProbeUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReflectionProbeUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReflectionProbeUpdateMode {
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
            Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReflectionProbeUpdateMode::ONCE, ReflectionProbeUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReflectionProbeUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ONCE", "REFLECTION_PROBE_UPDATE_ONCE", ReflectionProbeUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "REFLECTION_PROBE_UPDATE_ALWAYS", ReflectionProbeUpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for ReflectionProbeUpdateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Reflection Probe Update Once", 0i64), EnumeratorShape::new_int("Reflection Probe Update Always", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ReflectionProbeUpdateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ReflectionProbeUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReflectionProbeUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ReflectionProbeUpdateMode {
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
impl crate::registry::property::Export for ReflectionProbeUpdateMode {
    
}
impl crate::meta::Element for ReflectionProbeUpdateMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReflectionProbeAmbientMode {
    ord: i32
}
impl ReflectionProbeAmbientMode {
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_DISABLED")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_DISABLED`"]
    pub const DISABLED: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 0i32
    };
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_ENVIRONMENT")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_ENVIRONMENT`"]
    pub const ENVIRONMENT: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 1i32
    };
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_COLOR")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_COLOR`"]
    pub const COLOR: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ReflectionProbeAmbientMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReflectionProbeAmbientMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReflectionProbeAmbientMode {
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
            Self::DISABLED => "DISABLED", Self::ENVIRONMENT => "ENVIRONMENT", Self::COLOR => "COLOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReflectionProbeAmbientMode::DISABLED, ReflectionProbeAmbientMode::ENVIRONMENT, ReflectionProbeAmbientMode::COLOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReflectionProbeAmbientMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "REFLECTION_PROBE_AMBIENT_DISABLED", ReflectionProbeAmbientMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENVIRONMENT", "REFLECTION_PROBE_AMBIENT_ENVIRONMENT", ReflectionProbeAmbientMode::ENVIRONMENT), crate::meta::inspect::EnumConstant::new("COLOR", "REFLECTION_PROBE_AMBIENT_COLOR", ReflectionProbeAmbientMode::COLOR)]
        }
    }
}
impl crate::meta::GodotConvert for ReflectionProbeAmbientMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Reflection Probe Ambient Disabled", 0i64), EnumeratorShape::new_int("Reflection Probe Ambient Environment", 1i64), EnumeratorShape::new_int("Reflection Probe Ambient Color", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ReflectionProbeAmbientMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ReflectionProbeAmbientMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReflectionProbeAmbientMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ReflectionProbeAmbientMode {
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
impl crate::registry::property::Export for ReflectionProbeAmbientMode {
    
}
impl crate::meta::Element for ReflectionProbeAmbientMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DecalTexture {
    ord: i32
}
impl DecalTexture {
    #[doc(alias = "DECAL_TEXTURE_ALBEDO")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_ALBEDO`"]
    pub const ALBEDO: DecalTexture = DecalTexture {
        ord: 0i32
    };
    #[doc(alias = "DECAL_TEXTURE_NORMAL")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_NORMAL`"]
    pub const NORMAL: DecalTexture = DecalTexture {
        ord: 1i32
    };
    #[doc(alias = "DECAL_TEXTURE_ORM")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_ORM`"]
    pub const ORM: DecalTexture = DecalTexture {
        ord: 2i32
    };
    #[doc(alias = "DECAL_TEXTURE_EMISSION")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_EMISSION`"]
    pub const EMISSION: DecalTexture = DecalTexture {
        ord: 3i32
    };
    #[doc(alias = "DECAL_TEXTURE_MAX")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_MAX`"]
    pub const MAX: DecalTexture = DecalTexture {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DecalTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DecalTexture") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DecalTexture {
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
            Self::ALBEDO => "ALBEDO", Self::NORMAL => "NORMAL", Self::ORM => "ORM", Self::EMISSION => "EMISSION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DecalTexture::ALBEDO, DecalTexture::NORMAL, DecalTexture::ORM, DecalTexture::EMISSION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DecalTexture >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO", "DECAL_TEXTURE_ALBEDO", DecalTexture::ALBEDO), crate::meta::inspect::EnumConstant::new("NORMAL", "DECAL_TEXTURE_NORMAL", DecalTexture::NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "DECAL_TEXTURE_ORM", DecalTexture::ORM), crate::meta::inspect::EnumConstant::new("EMISSION", "DECAL_TEXTURE_EMISSION", DecalTexture::EMISSION), crate::meta::inspect::EnumConstant::new("MAX", "DECAL_TEXTURE_MAX", DecalTexture::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DecalTexture {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DecalTexture {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Decal Texture Albedo", 0i64), EnumeratorShape::new_int("Decal Texture Normal", 1i64), EnumeratorShape::new_int("Decal Texture Orm", 2i64), EnumeratorShape::new_int("Decal Texture Emission", 3i64), EnumeratorShape::new_int("Decal Texture Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.DecalTexture")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DecalTexture {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DecalTexture {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DecalTexture {
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
impl crate::registry::property::Export for DecalTexture {
    
}
impl crate::meta::Element for DecalTexture {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DecalFilter {
    ord: i32
}
impl DecalFilter {
    #[doc(alias = "DECAL_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST`"]
    pub const NEAREST: DecalFilter = DecalFilter {
        ord: 0i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR`"]
    pub const LINEAR: DecalFilter = DecalFilter {
        ord: 1i32
    };
    #[doc(alias = "DECAL_FILTER_NEAREST_MIPMAPS")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST_MIPMAPS`"]
    pub const NEAREST_MIPMAPS: DecalFilter = DecalFilter {
        ord: 2i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR_MIPMAPS")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR_MIPMAPS`"]
    pub const LINEAR_MIPMAPS: DecalFilter = DecalFilter {
        ord: 3i32
    };
    #[doc(alias = "DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_MIPMAPS_ANISOTROPIC: DecalFilter = DecalFilter {
        ord: 4i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_MIPMAPS_ANISOTROPIC: DecalFilter = DecalFilter {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DecalFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DecalFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DecalFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAPS => "NEAREST_MIPMAPS", Self::LINEAR_MIPMAPS => "LINEAR_MIPMAPS", Self::NEAREST_MIPMAPS_ANISOTROPIC => "NEAREST_MIPMAPS_ANISOTROPIC", Self::LINEAR_MIPMAPS_ANISOTROPIC => "LINEAR_MIPMAPS_ANISOTROPIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DecalFilter::NEAREST, DecalFilter::LINEAR, DecalFilter::NEAREST_MIPMAPS, DecalFilter::LINEAR_MIPMAPS, DecalFilter::NEAREST_MIPMAPS_ANISOTROPIC, DecalFilter::LINEAR_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DecalFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "DECAL_FILTER_NEAREST", DecalFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "DECAL_FILTER_LINEAR", DecalFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS", "DECAL_FILTER_NEAREST_MIPMAPS", DecalFilter::NEAREST_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS", "DECAL_FILTER_LINEAR_MIPMAPS", DecalFilter::LINEAR_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS_ANISOTROPIC", "DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC", DecalFilter::NEAREST_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS_ANISOTROPIC", "DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC", DecalFilter::LINEAR_MIPMAPS_ANISOTROPIC)]
        }
    }
}
impl crate::meta::GodotConvert for DecalFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Decal Filter Nearest", 0i64), EnumeratorShape::new_int("Decal Filter Linear", 1i64), EnumeratorShape::new_int("Decal Filter Nearest Mipmaps", 2i64), EnumeratorShape::new_int("Decal Filter Linear Mipmaps", 3i64), EnumeratorShape::new_int("Decal Filter Nearest Mipmaps Anisotropic", 4i64), EnumeratorShape::new_int("Decal Filter Linear Mipmaps Anisotropic", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.DecalFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DecalFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DecalFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DecalFilter {
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
impl crate::registry::property::Export for DecalFilter {
    
}
impl crate::meta::Element for DecalFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VoxelGIQuality`."]
pub struct VoxelGiQuality {
    ord: i32
}
impl VoxelGiQuality {
    #[doc(alias = "VOXEL_GI_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `VOXEL_GI_QUALITY_LOW`"]
    pub const LOW: VoxelGiQuality = VoxelGiQuality {
        ord: 0i32
    };
    #[doc(alias = "VOXEL_GI_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `VOXEL_GI_QUALITY_HIGH`"]
    pub const HIGH: VoxelGiQuality = VoxelGiQuality {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VoxelGiQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VoxelGiQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VoxelGiQuality {
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
            Self::LOW => "LOW", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VoxelGiQuality::LOW, VoxelGiQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VoxelGiQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOW", "VOXEL_GI_QUALITY_LOW", VoxelGiQuality::LOW), crate::meta::inspect::EnumConstant::new("HIGH", "VOXEL_GI_QUALITY_HIGH", VoxelGiQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for VoxelGiQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Voxel Gi Quality Low", 0i64), EnumeratorShape::new_int("Voxel Gi Quality High", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.VoxelGIQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VoxelGiQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VoxelGiQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VoxelGiQuality {
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
impl crate::registry::property::Export for VoxelGiQuality {
    
}
impl crate::meta::Element for VoxelGiQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesMode {
    ord: i32
}
impl ParticlesMode {
    #[doc(alias = "PARTICLES_MODE_2D")]
    #[doc = "Godot enumerator name: `PARTICLES_MODE_2D`"]
    pub const MODE_2D: ParticlesMode = ParticlesMode {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_MODE_3D")]
    #[doc = "Godot enumerator name: `PARTICLES_MODE_3D`"]
    pub const MODE_3D: ParticlesMode = ParticlesMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ParticlesMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesMode {
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
            Self::MODE_2D => "MODE_2D", Self::MODE_3D => "MODE_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesMode::MODE_2D, ParticlesMode::MODE_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MODE_2D", "PARTICLES_MODE_2D", ParticlesMode::MODE_2D), crate::meta::inspect::EnumConstant::new("MODE_3D", "PARTICLES_MODE_3D", ParticlesMode::MODE_3D)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particles Mode 2d", 0i64), EnumeratorShape::new_int("Particles Mode 3d", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ParticlesMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticlesMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticlesMode {
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
impl crate::registry::property::Export for ParticlesMode {
    
}
impl crate::meta::Element for ParticlesMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesTransformAlign {
    ord: i32
}
impl ParticlesTransformAlign {
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_DISABLED")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_DISABLED`"]
    pub const DISABLED: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD`"]
    pub const Z_BILLBOARD: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY`"]
    pub const Y_TO_VELOCITY: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY`"]
    pub const Z_BILLBOARD_Y_TO_VELOCITY: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticlesTransformAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesTransformAlign") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesTransformAlign {
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
            Self::DISABLED => "DISABLED", Self::Z_BILLBOARD => "Z_BILLBOARD", Self::Y_TO_VELOCITY => "Y_TO_VELOCITY", Self::Z_BILLBOARD_Y_TO_VELOCITY => "Z_BILLBOARD_Y_TO_VELOCITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesTransformAlign::DISABLED, ParticlesTransformAlign::Z_BILLBOARD, ParticlesTransformAlign::Y_TO_VELOCITY, ParticlesTransformAlign::Z_BILLBOARD_Y_TO_VELOCITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesTransformAlign >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "PARTICLES_TRANSFORM_ALIGN_DISABLED", ParticlesTransformAlign::DISABLED), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD", "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD", ParticlesTransformAlign::Z_BILLBOARD), crate::meta::inspect::EnumConstant::new("Y_TO_VELOCITY", "PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY", ParticlesTransformAlign::Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD_Y_TO_VELOCITY", "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY", ParticlesTransformAlign::Z_BILLBOARD_Y_TO_VELOCITY)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesTransformAlign {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particles Transform Align Disabled", 0i64), EnumeratorShape::new_int("Particles Transform Align Z Billboard", 1i64), EnumeratorShape::new_int("Particles Transform Align Y To Velocity", 2i64), EnumeratorShape::new_int("Particles Transform Align Z Billboard Y To Velocity", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ParticlesTransformAlign")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticlesTransformAlign {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesTransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticlesTransformAlign {
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
impl crate::registry::property::Export for ParticlesTransformAlign {
    
}
impl crate::meta::Element for ParticlesTransformAlign {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesDrawOrder {
    ord: i32
}
impl ParticlesDrawOrder {
    #[doc(alias = "PARTICLES_DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_INDEX`"]
    pub const INDEX: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_REVERSE_LIFETIME")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_REVERSE_LIFETIME`"]
    pub const REVERSE_LIFETIME: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_VIEW_DEPTH")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_VIEW_DEPTH`"]
    pub const VIEW_DEPTH: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticlesDrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesDrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesDrawOrder {
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::REVERSE_LIFETIME => "REVERSE_LIFETIME", Self::VIEW_DEPTH => "VIEW_DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesDrawOrder::INDEX, ParticlesDrawOrder::LIFETIME, ParticlesDrawOrder::REVERSE_LIFETIME, ParticlesDrawOrder::VIEW_DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesDrawOrder >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INDEX", "PARTICLES_DRAW_ORDER_INDEX", ParticlesDrawOrder::INDEX), crate::meta::inspect::EnumConstant::new("LIFETIME", "PARTICLES_DRAW_ORDER_LIFETIME", ParticlesDrawOrder::LIFETIME), crate::meta::inspect::EnumConstant::new("REVERSE_LIFETIME", "PARTICLES_DRAW_ORDER_REVERSE_LIFETIME", ParticlesDrawOrder::REVERSE_LIFETIME), crate::meta::inspect::EnumConstant::new("VIEW_DEPTH", "PARTICLES_DRAW_ORDER_VIEW_DEPTH", ParticlesDrawOrder::VIEW_DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesDrawOrder {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particles Draw Order Index", 0i64), EnumeratorShape::new_int("Particles Draw Order Lifetime", 1i64), EnumeratorShape::new_int("Particles Draw Order Reverse Lifetime", 2i64), EnumeratorShape::new_int("Particles Draw Order View Depth", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ParticlesDrawOrder")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticlesDrawOrder {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesDrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticlesDrawOrder {
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
impl crate::registry::property::Export for ParticlesDrawOrder {
    
}
impl crate::meta::Element for ParticlesDrawOrder {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesCollisionType {
    ord: i32
}
impl ParticlesCollisionType {
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT`"]
    pub const SPHERE_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_BOX_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_BOX_ATTRACT`"]
    pub const BOX_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT`"]
    pub const VECTOR_FIELD_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE`"]
    pub const SPHERE_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 3i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_BOX_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_BOX_COLLIDE`"]
    pub const BOX_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 4i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SDF_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SDF_COLLIDE`"]
    pub const SDF_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 5i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE`"]
    pub const HEIGHTFIELD_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ParticlesCollisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesCollisionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesCollisionType {
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
            Self::SPHERE_ATTRACT => "SPHERE_ATTRACT", Self::BOX_ATTRACT => "BOX_ATTRACT", Self::VECTOR_FIELD_ATTRACT => "VECTOR_FIELD_ATTRACT", Self::SPHERE_COLLIDE => "SPHERE_COLLIDE", Self::BOX_COLLIDE => "BOX_COLLIDE", Self::SDF_COLLIDE => "SDF_COLLIDE", Self::HEIGHTFIELD_COLLIDE => "HEIGHTFIELD_COLLIDE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesCollisionType::SPHERE_ATTRACT, ParticlesCollisionType::BOX_ATTRACT, ParticlesCollisionType::VECTOR_FIELD_ATTRACT, ParticlesCollisionType::SPHERE_COLLIDE, ParticlesCollisionType::BOX_COLLIDE, ParticlesCollisionType::SDF_COLLIDE, ParticlesCollisionType::HEIGHTFIELD_COLLIDE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesCollisionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SPHERE_ATTRACT", "PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT", ParticlesCollisionType::SPHERE_ATTRACT), crate::meta::inspect::EnumConstant::new("BOX_ATTRACT", "PARTICLES_COLLISION_TYPE_BOX_ATTRACT", ParticlesCollisionType::BOX_ATTRACT), crate::meta::inspect::EnumConstant::new("VECTOR_FIELD_ATTRACT", "PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT", ParticlesCollisionType::VECTOR_FIELD_ATTRACT), crate::meta::inspect::EnumConstant::new("SPHERE_COLLIDE", "PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE", ParticlesCollisionType::SPHERE_COLLIDE), crate::meta::inspect::EnumConstant::new("BOX_COLLIDE", "PARTICLES_COLLISION_TYPE_BOX_COLLIDE", ParticlesCollisionType::BOX_COLLIDE), crate::meta::inspect::EnumConstant::new("SDF_COLLIDE", "PARTICLES_COLLISION_TYPE_SDF_COLLIDE", ParticlesCollisionType::SDF_COLLIDE), crate::meta::inspect::EnumConstant::new("HEIGHTFIELD_COLLIDE", "PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE", ParticlesCollisionType::HEIGHTFIELD_COLLIDE)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesCollisionType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particles Collision Type Sphere Attract", 0i64), EnumeratorShape::new_int("Particles Collision Type Box Attract", 1i64), EnumeratorShape::new_int("Particles Collision Type Vector Field Attract", 2i64), EnumeratorShape::new_int("Particles Collision Type Sphere Collide", 3i64), EnumeratorShape::new_int("Particles Collision Type Box Collide", 4i64), EnumeratorShape::new_int("Particles Collision Type Sdf Collide", 5i64), EnumeratorShape::new_int("Particles Collision Type Heightfield Collide", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ParticlesCollisionType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticlesCollisionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesCollisionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticlesCollisionType {
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
impl crate::registry::property::Export for ParticlesCollisionType {
    
}
impl crate::meta::Element for ParticlesCollisionType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesCollisionHeightfieldResolution {
    ord: i32
}
impl ParticlesCollisionHeightfieldResolution {
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256`"]
    pub const RESOLUTION_256: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512`"]
    pub const RESOLUTION_512: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024`"]
    pub const RESOLUTION_1024: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048`"]
    pub const RESOLUTION_2048: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 3i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096`"]
    pub const RESOLUTION_4096: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 4i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192`"]
    pub const RESOLUTION_8192: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 5i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX`"]
    pub const MAX: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ParticlesCollisionHeightfieldResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesCollisionHeightfieldResolution") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesCollisionHeightfieldResolution {
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
            Self::RESOLUTION_256 => "RESOLUTION_256", Self::RESOLUTION_512 => "RESOLUTION_512", Self::RESOLUTION_1024 => "RESOLUTION_1024", Self::RESOLUTION_2048 => "RESOLUTION_2048", Self::RESOLUTION_4096 => "RESOLUTION_4096", Self::RESOLUTION_8192 => "RESOLUTION_8192", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesCollisionHeightfieldResolution::RESOLUTION_256, ParticlesCollisionHeightfieldResolution::RESOLUTION_512, ParticlesCollisionHeightfieldResolution::RESOLUTION_1024, ParticlesCollisionHeightfieldResolution::RESOLUTION_2048, ParticlesCollisionHeightfieldResolution::RESOLUTION_4096, ParticlesCollisionHeightfieldResolution::RESOLUTION_8192]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesCollisionHeightfieldResolution >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESOLUTION_256", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256", ParticlesCollisionHeightfieldResolution::RESOLUTION_256), crate::meta::inspect::EnumConstant::new("RESOLUTION_512", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512", ParticlesCollisionHeightfieldResolution::RESOLUTION_512), crate::meta::inspect::EnumConstant::new("RESOLUTION_1024", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024", ParticlesCollisionHeightfieldResolution::RESOLUTION_1024), crate::meta::inspect::EnumConstant::new("RESOLUTION_2048", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048", ParticlesCollisionHeightfieldResolution::RESOLUTION_2048), crate::meta::inspect::EnumConstant::new("RESOLUTION_4096", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096", ParticlesCollisionHeightfieldResolution::RESOLUTION_4096), crate::meta::inspect::EnumConstant::new("RESOLUTION_8192", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192", ParticlesCollisionHeightfieldResolution::RESOLUTION_8192), crate::meta::inspect::EnumConstant::new("MAX", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX", ParticlesCollisionHeightfieldResolution::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ParticlesCollisionHeightfieldResolution {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for ParticlesCollisionHeightfieldResolution {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Particles Collision Heightfield Resolution 256", 0i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution 512", 1i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution 1024", 2i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution 2048", 3i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution 4096", 4i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution 8192", 5i64), EnumeratorShape::new_int("Particles Collision Heightfield Resolution Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ParticlesCollisionHeightfieldResolution")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ParticlesCollisionHeightfieldResolution {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesCollisionHeightfieldResolution {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ParticlesCollisionHeightfieldResolution {
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
impl crate::registry::property::Export for ParticlesCollisionHeightfieldResolution {
    
}
impl crate::meta::Element for ParticlesCollisionHeightfieldResolution {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FogVolumeShape {
    ord: i32
}
impl FogVolumeShape {
    #[doc(alias = "FOG_VOLUME_SHAPE_ELLIPSOID")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_ELLIPSOID`"]
    pub const ELLIPSOID: FogVolumeShape = FogVolumeShape {
        ord: 0i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_CONE")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_CONE`"]
    pub const CONE: FogVolumeShape = FogVolumeShape {
        ord: 1i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_CYLINDER")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_CYLINDER`"]
    pub const CYLINDER: FogVolumeShape = FogVolumeShape {
        ord: 2i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_BOX")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_BOX`"]
    pub const BOX: FogVolumeShape = FogVolumeShape {
        ord: 3i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_WORLD")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_WORLD`"]
    pub const WORLD: FogVolumeShape = FogVolumeShape {
        ord: 4i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_MAX`"]
    pub const MAX: FogVolumeShape = FogVolumeShape {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FogVolumeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FogVolumeShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FogVolumeShape {
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
            Self::ELLIPSOID => "ELLIPSOID", Self::CONE => "CONE", Self::CYLINDER => "CYLINDER", Self::BOX => "BOX", Self::WORLD => "WORLD", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FogVolumeShape::ELLIPSOID, FogVolumeShape::CONE, FogVolumeShape::CYLINDER, FogVolumeShape::BOX, FogVolumeShape::WORLD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FogVolumeShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ELLIPSOID", "FOG_VOLUME_SHAPE_ELLIPSOID", FogVolumeShape::ELLIPSOID), crate::meta::inspect::EnumConstant::new("CONE", "FOG_VOLUME_SHAPE_CONE", FogVolumeShape::CONE), crate::meta::inspect::EnumConstant::new("CYLINDER", "FOG_VOLUME_SHAPE_CYLINDER", FogVolumeShape::CYLINDER), crate::meta::inspect::EnumConstant::new("BOX", "FOG_VOLUME_SHAPE_BOX", FogVolumeShape::BOX), crate::meta::inspect::EnumConstant::new("WORLD", "FOG_VOLUME_SHAPE_WORLD", FogVolumeShape::WORLD), crate::meta::inspect::EnumConstant::new("MAX", "FOG_VOLUME_SHAPE_MAX", FogVolumeShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for FogVolumeShape {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FogVolumeShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Fog Volume Shape Ellipsoid", 0i64), EnumeratorShape::new_int("Fog Volume Shape Cone", 1i64), EnumeratorShape::new_int("Fog Volume Shape Cylinder", 2i64), EnumeratorShape::new_int("Fog Volume Shape Box", 3i64), EnumeratorShape::new_int("Fog Volume Shape World", 4i64), EnumeratorShape::new_int("Fog Volume Shape Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.FogVolumeShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for FogVolumeShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FogVolumeShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for FogVolumeShape {
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
impl crate::registry::property::Export for FogVolumeShape {
    
}
impl crate::meta::Element for FogVolumeShape {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportScaling3DMode {
    ord: i32
}
impl ViewportScaling3DMode {
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_BILINEAR")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_BILINEAR`"]
    pub const BILINEAR: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_FSR")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_FSR`"]
    pub const FSR: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_FSR2")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_FSR2`"]
    pub const FSR2: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL`"]
    pub const METALFX_SPATIAL: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL`"]
    pub const METALFX_TEMPORAL: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_MAX`"]
    pub const MAX: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ViewportScaling3DMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportScaling3DMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportScaling3DMode {
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
            Self::BILINEAR => "BILINEAR", Self::FSR => "FSR", Self::FSR2 => "FSR2", Self::METALFX_SPATIAL => "METALFX_SPATIAL", Self::METALFX_TEMPORAL => "METALFX_TEMPORAL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportScaling3DMode::BILINEAR, ViewportScaling3DMode::FSR, ViewportScaling3DMode::FSR2, ViewportScaling3DMode::METALFX_SPATIAL, ViewportScaling3DMode::METALFX_TEMPORAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportScaling3DMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BILINEAR", "VIEWPORT_SCALING_3D_MODE_BILINEAR", ViewportScaling3DMode::BILINEAR), crate::meta::inspect::EnumConstant::new("FSR", "VIEWPORT_SCALING_3D_MODE_FSR", ViewportScaling3DMode::FSR), crate::meta::inspect::EnumConstant::new("FSR2", "VIEWPORT_SCALING_3D_MODE_FSR2", ViewportScaling3DMode::FSR2), crate::meta::inspect::EnumConstant::new("METALFX_SPATIAL", "VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL", ViewportScaling3DMode::METALFX_SPATIAL), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL", "VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL", ViewportScaling3DMode::METALFX_TEMPORAL), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SCALING_3D_MODE_MAX", ViewportScaling3DMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportScaling3DMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ViewportScaling3DMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Scaling 3d Mode Bilinear", 0i64), EnumeratorShape::new_int("Viewport Scaling 3d Mode Fsr", 1i64), EnumeratorShape::new_int("Viewport Scaling 3d Mode Fsr2", 2i64), EnumeratorShape::new_int("Viewport Scaling 3d Mode Metalfx Spatial", 3i64), EnumeratorShape::new_int("Viewport Scaling 3d Mode Metalfx Temporal", 4i64), EnumeratorShape::new_int("Viewport Scaling 3d Mode Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportScaling3DMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportScaling3DMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportScaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportScaling3DMode {
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
impl crate::registry::property::Export for ViewportScaling3DMode {
    
}
impl crate::meta::Element for ViewportScaling3DMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportUpdateMode {
    ord: i32
}
impl ViewportUpdateMode {
    #[doc(alias = "VIEWPORT_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_DISABLED`"]
    pub const DISABLED: ViewportUpdateMode = ViewportUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_ONCE`"]
    pub const ONCE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_WHEN_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_WHEN_VISIBLE`"]
    pub const WHEN_VISIBLE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE`"]
    pub const WHEN_PARENT_VISIBLE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_ALWAYS`"]
    pub const ALWAYS: ViewportUpdateMode = ViewportUpdateMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportUpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::WHEN_VISIBLE => "WHEN_VISIBLE", Self::WHEN_PARENT_VISIBLE => "WHEN_PARENT_VISIBLE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportUpdateMode::DISABLED, ViewportUpdateMode::ONCE, ViewportUpdateMode::WHEN_VISIBLE, ViewportUpdateMode::WHEN_PARENT_VISIBLE, ViewportUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_UPDATE_DISABLED", ViewportUpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "VIEWPORT_UPDATE_ONCE", ViewportUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("WHEN_VISIBLE", "VIEWPORT_UPDATE_WHEN_VISIBLE", ViewportUpdateMode::WHEN_VISIBLE), crate::meta::inspect::EnumConstant::new("WHEN_PARENT_VISIBLE", "VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE", ViewportUpdateMode::WHEN_PARENT_VISIBLE), crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_UPDATE_ALWAYS", ViewportUpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportUpdateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Update Disabled", 0i64), EnumeratorShape::new_int("Viewport Update Once", 1i64), EnumeratorShape::new_int("Viewport Update When Visible", 2i64), EnumeratorShape::new_int("Viewport Update When Parent Visible", 3i64), EnumeratorShape::new_int("Viewport Update Always", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportUpdateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportUpdateMode {
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
impl crate::registry::property::Export for ViewportUpdateMode {
    
}
impl crate::meta::Element for ViewportUpdateMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportClearMode {
    ord: i32
}
impl ViewportClearMode {
    #[doc(alias = "VIEWPORT_CLEAR_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_ALWAYS`"]
    pub const ALWAYS: ViewportClearMode = ViewportClearMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_CLEAR_NEVER")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_NEVER`"]
    pub const NEVER: ViewportClearMode = ViewportClearMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_CLEAR_ONLY_NEXT_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_ONLY_NEXT_FRAME`"]
    pub const ONLY_NEXT_FRAME: ViewportClearMode = ViewportClearMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ViewportClearMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportClearMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportClearMode {
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
            Self::ALWAYS => "ALWAYS", Self::NEVER => "NEVER", Self::ONLY_NEXT_FRAME => "ONLY_NEXT_FRAME", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportClearMode::ALWAYS, ViewportClearMode::NEVER, ViewportClearMode::ONLY_NEXT_FRAME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportClearMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_CLEAR_ALWAYS", ViewportClearMode::ALWAYS), crate::meta::inspect::EnumConstant::new("NEVER", "VIEWPORT_CLEAR_NEVER", ViewportClearMode::NEVER), crate::meta::inspect::EnumConstant::new("ONLY_NEXT_FRAME", "VIEWPORT_CLEAR_ONLY_NEXT_FRAME", ViewportClearMode::ONLY_NEXT_FRAME)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportClearMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Clear Always", 0i64), EnumeratorShape::new_int("Viewport Clear Never", 1i64), EnumeratorShape::new_int("Viewport Clear Only Next Frame", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportClearMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportClearMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportClearMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportClearMode {
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
impl crate::registry::property::Export for ViewportClearMode {
    
}
impl crate::meta::Element for ViewportClearMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportEnvironmentMode {
    ord: i32
}
impl ViewportEnvironmentMode {
    #[doc(alias = "VIEWPORT_ENVIRONMENT_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_DISABLED`"]
    pub const DISABLED: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_ENABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_ENABLED`"]
    pub const ENABLED: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_INHERIT")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_INHERIT`"]
    pub const INHERIT: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_MAX`"]
    pub const MAX: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportEnvironmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportEnvironmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportEnvironmentMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::INHERIT => "INHERIT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportEnvironmentMode::DISABLED, ViewportEnvironmentMode::ENABLED, ViewportEnvironmentMode::INHERIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportEnvironmentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_ENVIRONMENT_DISABLED", ViewportEnvironmentMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "VIEWPORT_ENVIRONMENT_ENABLED", ViewportEnvironmentMode::ENABLED), crate::meta::inspect::EnumConstant::new("INHERIT", "VIEWPORT_ENVIRONMENT_INHERIT", ViewportEnvironmentMode::INHERIT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_ENVIRONMENT_MAX", ViewportEnvironmentMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportEnvironmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportEnvironmentMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Environment Disabled", 0i64), EnumeratorShape::new_int("Viewport Environment Enabled", 1i64), EnumeratorShape::new_int("Viewport Environment Inherit", 2i64), EnumeratorShape::new_int("Viewport Environment Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportEnvironmentMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportEnvironmentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportEnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportEnvironmentMode {
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
impl crate::registry::property::Export for ViewportEnvironmentMode {
    
}
impl crate::meta::Element for ViewportEnvironmentMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportSDFOversize`."]
pub struct ViewportSdfOversize {
    ord: i32
}
impl ViewportSdfOversize {
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_100_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_100_PERCENT`"]
    pub const OVERSIZE_100_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_120_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_120_PERCENT`"]
    pub const OVERSIZE_120_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_150_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_150_PERCENT`"]
    pub const OVERSIZE_150_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_200_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_200_PERCENT`"]
    pub const OVERSIZE_200_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_MAX`"]
    pub const MAX: ViewportSdfOversize = ViewportSdfOversize {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportSdfOversize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportSdfOversize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportSdfOversize {
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
            Self::OVERSIZE_100_PERCENT => "OVERSIZE_100_PERCENT", Self::OVERSIZE_120_PERCENT => "OVERSIZE_120_PERCENT", Self::OVERSIZE_150_PERCENT => "OVERSIZE_150_PERCENT", Self::OVERSIZE_200_PERCENT => "OVERSIZE_200_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportSdfOversize::OVERSIZE_100_PERCENT, ViewportSdfOversize::OVERSIZE_120_PERCENT, ViewportSdfOversize::OVERSIZE_150_PERCENT, ViewportSdfOversize::OVERSIZE_200_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportSdfOversize >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OVERSIZE_100_PERCENT", "VIEWPORT_SDF_OVERSIZE_100_PERCENT", ViewportSdfOversize::OVERSIZE_100_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_120_PERCENT", "VIEWPORT_SDF_OVERSIZE_120_PERCENT", ViewportSdfOversize::OVERSIZE_120_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_150_PERCENT", "VIEWPORT_SDF_OVERSIZE_150_PERCENT", ViewportSdfOversize::OVERSIZE_150_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_200_PERCENT", "VIEWPORT_SDF_OVERSIZE_200_PERCENT", ViewportSdfOversize::OVERSIZE_200_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SDF_OVERSIZE_MAX", ViewportSdfOversize::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportSdfOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for ViewportSdfOversize {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Sdf Oversize 100 Percent", 0i64), EnumeratorShape::new_int("Viewport Sdf Oversize 120 Percent", 1i64), EnumeratorShape::new_int("Viewport Sdf Oversize 150 Percent", 2i64), EnumeratorShape::new_int("Viewport Sdf Oversize 200 Percent", 3i64), EnumeratorShape::new_int("Viewport Sdf Oversize Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportSDFOversize")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportSdfOversize {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportSdfOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportSdfOversize {
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
impl crate::registry::property::Export for ViewportSdfOversize {
    
}
impl crate::meta::Element for ViewportSdfOversize {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportSDFScale`."]
pub struct ViewportSdfScale {
    ord: i32
}
impl ViewportSdfScale {
    #[doc(alias = "VIEWPORT_SDF_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_25_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_25_PERCENT`"]
    pub const SCALE_25_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_MAX`"]
    pub const MAX: ViewportSdfScale = ViewportSdfScale {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportSdfScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportSdfScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportSdfScale {
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
            Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_25_PERCENT => "SCALE_25_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportSdfScale::SCALE_100_PERCENT, ViewportSdfScale::SCALE_50_PERCENT, ViewportSdfScale::SCALE_25_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportSdfScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "VIEWPORT_SDF_SCALE_100_PERCENT", ViewportSdfScale::SCALE_100_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "VIEWPORT_SDF_SCALE_50_PERCENT", ViewportSdfScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_25_PERCENT", "VIEWPORT_SDF_SCALE_25_PERCENT", ViewportSdfScale::SCALE_25_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SDF_SCALE_MAX", ViewportSdfScale::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportSdfScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportSdfScale {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Sdf Scale 100 Percent", 0i64), EnumeratorShape::new_int("Viewport Sdf Scale 50 Percent", 1i64), EnumeratorShape::new_int("Viewport Sdf Scale 25 Percent", 2i64), EnumeratorShape::new_int("Viewport Sdf Scale Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportSDFScale")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportSdfScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportSdfScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportSdfScale {
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
impl crate::registry::property::Export for ViewportSdfScale {
    
}
impl crate::meta::Element for ViewportSdfScale {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportMSAA`."]
pub struct ViewportMsaa {
    ord: i32
}
impl ViewportMsaa {
    #[doc(alias = "VIEWPORT_MSAA_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_DISABLED`"]
    pub const DISABLED: ViewportMsaa = ViewportMsaa {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_MSAA_2X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_2X`"]
    pub const MSAA_2X: ViewportMsaa = ViewportMsaa {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_MSAA_4X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_4X`"]
    pub const MSAA_4X: ViewportMsaa = ViewportMsaa {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_MSAA_8X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_8X`"]
    pub const MSAA_8X: ViewportMsaa = ViewportMsaa {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_MSAA_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_MAX`"]
    pub const MAX: ViewportMsaa = ViewportMsaa {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportMsaa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportMsaa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportMsaa {
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
            Self::DISABLED => "DISABLED", Self::MSAA_2X => "MSAA_2X", Self::MSAA_4X => "MSAA_4X", Self::MSAA_8X => "MSAA_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportMsaa::DISABLED, ViewportMsaa::MSAA_2X, ViewportMsaa::MSAA_4X, ViewportMsaa::MSAA_8X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportMsaa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_MSAA_DISABLED", ViewportMsaa::DISABLED), crate::meta::inspect::EnumConstant::new("MSAA_2X", "VIEWPORT_MSAA_2X", ViewportMsaa::MSAA_2X), crate::meta::inspect::EnumConstant::new("MSAA_4X", "VIEWPORT_MSAA_4X", ViewportMsaa::MSAA_4X), crate::meta::inspect::EnumConstant::new("MSAA_8X", "VIEWPORT_MSAA_8X", ViewportMsaa::MSAA_8X), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_MSAA_MAX", ViewportMsaa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportMsaa {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for ViewportMsaa {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Msaa Disabled", 0i64), EnumeratorShape::new_int("Viewport Msaa 2x", 1i64), EnumeratorShape::new_int("Viewport Msaa 4x", 2i64), EnumeratorShape::new_int("Viewport Msaa 8x", 3i64), EnumeratorShape::new_int("Viewport Msaa Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportMSAA")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportMsaa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportMsaa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportMsaa {
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
impl crate::registry::property::Export for ViewportMsaa {
    
}
impl crate::meta::Element for ViewportMsaa {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportAnisotropicFiltering {
    ord: i32
}
impl ViewportAnisotropicFiltering {
    #[doc(alias = "VIEWPORT_ANISOTROPY_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_DISABLED`"]
    pub const DISABLED: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_2X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_2X`"]
    pub const ANISOTROPY_2X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_4X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_4X`"]
    pub const ANISOTROPY_4X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_8X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_8X`"]
    pub const ANISOTROPY_8X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_16X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_16X`"]
    pub const ANISOTROPY_16X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_MAX`"]
    pub const MAX: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ViewportAnisotropicFiltering {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportAnisotropicFiltering") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportAnisotropicFiltering {
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
            Self::DISABLED => "DISABLED", Self::ANISOTROPY_2X => "ANISOTROPY_2X", Self::ANISOTROPY_4X => "ANISOTROPY_4X", Self::ANISOTROPY_8X => "ANISOTROPY_8X", Self::ANISOTROPY_16X => "ANISOTROPY_16X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportAnisotropicFiltering::DISABLED, ViewportAnisotropicFiltering::ANISOTROPY_2X, ViewportAnisotropicFiltering::ANISOTROPY_4X, ViewportAnisotropicFiltering::ANISOTROPY_8X, ViewportAnisotropicFiltering::ANISOTROPY_16X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportAnisotropicFiltering >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_ANISOTROPY_DISABLED", ViewportAnisotropicFiltering::DISABLED), crate::meta::inspect::EnumConstant::new("ANISOTROPY_2X", "VIEWPORT_ANISOTROPY_2X", ViewportAnisotropicFiltering::ANISOTROPY_2X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_4X", "VIEWPORT_ANISOTROPY_4X", ViewportAnisotropicFiltering::ANISOTROPY_4X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_8X", "VIEWPORT_ANISOTROPY_8X", ViewportAnisotropicFiltering::ANISOTROPY_8X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_16X", "VIEWPORT_ANISOTROPY_16X", ViewportAnisotropicFiltering::ANISOTROPY_16X), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_ANISOTROPY_MAX", ViewportAnisotropicFiltering::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportAnisotropicFiltering {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ViewportAnisotropicFiltering {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Anisotropy Disabled", 0i64), EnumeratorShape::new_int("Viewport Anisotropy 2x", 1i64), EnumeratorShape::new_int("Viewport Anisotropy 4x", 2i64), EnumeratorShape::new_int("Viewport Anisotropy 8x", 3i64), EnumeratorShape::new_int("Viewport Anisotropy 16x", 4i64), EnumeratorShape::new_int("Viewport Anisotropy Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportAnisotropicFiltering")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportAnisotropicFiltering {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportAnisotropicFiltering {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportAnisotropicFiltering {
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
impl crate::registry::property::Export for ViewportAnisotropicFiltering {
    
}
impl crate::meta::Element for ViewportAnisotropicFiltering {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportScreenSpaceAA`."]
pub struct ViewportScreenSpaceAa {
    ord: i32
}
impl ViewportScreenSpaceAa {
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_DISABLED`"]
    pub const DISABLED: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_FXAA")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_FXAA`"]
    pub const FXAA: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_SMAA")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_SMAA`"]
    pub const SMAA: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_MAX`"]
    pub const MAX: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportScreenSpaceAa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportScreenSpaceAa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportScreenSpaceAa {
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
            Self::DISABLED => "DISABLED", Self::FXAA => "FXAA", Self::SMAA => "SMAA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportScreenSpaceAa::DISABLED, ViewportScreenSpaceAa::FXAA, ViewportScreenSpaceAa::SMAA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportScreenSpaceAa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_SCREEN_SPACE_AA_DISABLED", ViewportScreenSpaceAa::DISABLED), crate::meta::inspect::EnumConstant::new("FXAA", "VIEWPORT_SCREEN_SPACE_AA_FXAA", ViewportScreenSpaceAa::FXAA), crate::meta::inspect::EnumConstant::new("SMAA", "VIEWPORT_SCREEN_SPACE_AA_SMAA", ViewportScreenSpaceAa::SMAA), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SCREEN_SPACE_AA_MAX", ViewportScreenSpaceAa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportScreenSpaceAa {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportScreenSpaceAa {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Screen Space Aa Disabled", 0i64), EnumeratorShape::new_int("Viewport Screen Space Aa Fxaa", 1i64), EnumeratorShape::new_int("Viewport Screen Space Aa Smaa", 2i64), EnumeratorShape::new_int("Viewport Screen Space Aa Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportScreenSpaceAA")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportScreenSpaceAa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportScreenSpaceAa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportScreenSpaceAa {
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
impl crate::registry::property::Export for ViewportScreenSpaceAa {
    
}
impl crate::meta::Element for ViewportScreenSpaceAa {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportOcclusionCullingBuildQuality {
    ord: i32
}
impl ViewportOcclusionCullingBuildQuality {
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW`"]
    pub const LOW: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM`"]
    pub const MEDIUM: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH`"]
    pub const HIGH: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ViewportOcclusionCullingBuildQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportOcclusionCullingBuildQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportOcclusionCullingBuildQuality {
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
            Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportOcclusionCullingBuildQuality::LOW, ViewportOcclusionCullingBuildQuality::MEDIUM, ViewportOcclusionCullingBuildQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportOcclusionCullingBuildQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOW", "VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW", ViewportOcclusionCullingBuildQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM", ViewportOcclusionCullingBuildQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH", ViewportOcclusionCullingBuildQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportOcclusionCullingBuildQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Occlusion Build Quality Low", 0i64), EnumeratorShape::new_int("Viewport Occlusion Build Quality Medium", 1i64), EnumeratorShape::new_int("Viewport Occlusion Build Quality High", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportOcclusionCullingBuildQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportOcclusionCullingBuildQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportOcclusionCullingBuildQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportOcclusionCullingBuildQuality {
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
impl crate::registry::property::Export for ViewportOcclusionCullingBuildQuality {
    
}
impl crate::meta::Element for ViewportOcclusionCullingBuildQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportRenderInfo {
    ord: i32
}
impl ViewportRenderInfo {
    #[doc(alias = "VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME`"]
    pub const OBJECTS_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME`"]
    pub const PRIMITIVES_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME`"]
    pub const DRAW_CALLS_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_MAX`"]
    pub const MAX: ViewportRenderInfo = ViewportRenderInfo {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportRenderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportRenderInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportRenderInfo {
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
            Self::OBJECTS_IN_FRAME => "OBJECTS_IN_FRAME", Self::PRIMITIVES_IN_FRAME => "PRIMITIVES_IN_FRAME", Self::DRAW_CALLS_IN_FRAME => "DRAW_CALLS_IN_FRAME", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportRenderInfo::OBJECTS_IN_FRAME, ViewportRenderInfo::PRIMITIVES_IN_FRAME, ViewportRenderInfo::DRAW_CALLS_IN_FRAME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportRenderInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OBJECTS_IN_FRAME", "VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME", ViewportRenderInfo::OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("PRIMITIVES_IN_FRAME", "VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME", ViewportRenderInfo::PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("DRAW_CALLS_IN_FRAME", "VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME", ViewportRenderInfo::DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_RENDER_INFO_MAX", ViewportRenderInfo::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportRenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportRenderInfo {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Render Info Objects In Frame", 0i64), EnumeratorShape::new_int("Viewport Render Info Primitives In Frame", 1i64), EnumeratorShape::new_int("Viewport Render Info Draw Calls In Frame", 2i64), EnumeratorShape::new_int("Viewport Render Info Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportRenderInfo")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportRenderInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportRenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportRenderInfo {
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
impl crate::registry::property::Export for ViewportRenderInfo {
    
}
impl crate::meta::Element for ViewportRenderInfo {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportRenderInfoType {
    ord: i32
}
impl ViewportRenderInfoType {
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_VISIBLE`"]
    pub const VISIBLE: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_SHADOW")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_SHADOW`"]
    pub const SHADOW: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_CANVAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_CANVAS`"]
    pub const CANVAS: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_MAX`"]
    pub const MAX: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportRenderInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportRenderInfoType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportRenderInfoType {
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
            Self::VISIBLE => "VISIBLE", Self::SHADOW => "SHADOW", Self::CANVAS => "CANVAS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportRenderInfoType::VISIBLE, ViewportRenderInfoType::SHADOW, ViewportRenderInfoType::CANVAS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportRenderInfoType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "VIEWPORT_RENDER_INFO_TYPE_VISIBLE", ViewportRenderInfoType::VISIBLE), crate::meta::inspect::EnumConstant::new("SHADOW", "VIEWPORT_RENDER_INFO_TYPE_SHADOW", ViewportRenderInfoType::SHADOW), crate::meta::inspect::EnumConstant::new("CANVAS", "VIEWPORT_RENDER_INFO_TYPE_CANVAS", ViewportRenderInfoType::CANVAS), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_RENDER_INFO_TYPE_MAX", ViewportRenderInfoType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportRenderInfoType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportRenderInfoType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Render Info Type Visible", 0i64), EnumeratorShape::new_int("Viewport Render Info Type Shadow", 1i64), EnumeratorShape::new_int("Viewport Render Info Type Canvas", 2i64), EnumeratorShape::new_int("Viewport Render Info Type Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportRenderInfoType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportRenderInfoType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportRenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportRenderInfoType {
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
impl crate::registry::property::Export for ViewportRenderInfoType {
    
}
impl crate::meta::Element for ViewportRenderInfoType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportDebugDraw {
    ord: i32
}
impl ViewportDebugDraw {
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DISABLED`"]
    pub const DISABLED: ViewportDebugDraw = ViewportDebugDraw {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_UNSHADED")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_UNSHADED`"]
    pub const UNSHADED: ViewportDebugDraw = ViewportDebugDraw {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_LIGHTING")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_LIGHTING`"]
    pub const LIGHTING: ViewportDebugDraw = ViewportDebugDraw {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_OVERDRAW")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_OVERDRAW`"]
    pub const OVERDRAW: ViewportDebugDraw = ViewportDebugDraw {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_WIREFRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_WIREFRAME`"]
    pub const WIREFRAME: ViewportDebugDraw = ViewportDebugDraw {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER`"]
    pub const NORMAL_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 5i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO`"]
    pub const VOXEL_GI_ALBEDO: ViewportDebugDraw = ViewportDebugDraw {
        ord: 6i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING`"]
    pub const VOXEL_GI_LIGHTING: ViewportDebugDraw = ViewportDebugDraw {
        ord: 7i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION`"]
    pub const VOXEL_GI_EMISSION: ViewportDebugDraw = ViewportDebugDraw {
        ord: 8i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS`"]
    pub const SHADOW_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 9i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS`"]
    pub const DIRECTIONAL_SHADOW_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 10i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE`"]
    pub const SCENE_LUMINANCE: ViewportDebugDraw = ViewportDebugDraw {
        ord: 11i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SSAO")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SSAO`"]
    pub const SSAO: ViewportDebugDraw = ViewportDebugDraw {
        ord: 12i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SSIL")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SSIL`"]
    pub const SSIL: ViewportDebugDraw = ViewportDebugDraw {
        ord: 13i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_PSSM_SPLITS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_PSSM_SPLITS`"]
    pub const PSSM_SPLITS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 14i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DECAL_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DECAL_ATLAS`"]
    pub const DECAL_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 15i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SDFGI")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SDFGI`"]
    pub const SDFGI: ViewportDebugDraw = ViewportDebugDraw {
        ord: 16i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SDFGI_PROBES")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SDFGI_PROBES`"]
    pub const SDFGI_PROBES: ViewportDebugDraw = ViewportDebugDraw {
        ord: 17i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_GI_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_GI_BUFFER`"]
    pub const GI_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 18i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DISABLE_LOD")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DISABLE_LOD`"]
    pub const DISABLE_LOD: ViewportDebugDraw = ViewportDebugDraw {
        ord: 19i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS`"]
    pub const CLUSTER_OMNI_LIGHTS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 20i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS`"]
    pub const CLUSTER_SPOT_LIGHTS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 21i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS`"]
    pub const CLUSTER_DECALS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 22i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES`"]
    pub const CLUSTER_REFLECTION_PROBES: ViewportDebugDraw = ViewportDebugDraw {
        ord: 23i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_OCCLUDERS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_OCCLUDERS`"]
    pub const OCCLUDERS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 24i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_MOTION_VECTORS`"]
    pub const MOTION_VECTORS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 25i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER`"]
    pub const INTERNAL_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for ViewportDebugDraw {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportDebugDraw") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportDebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::UNSHADED => "UNSHADED", Self::LIGHTING => "LIGHTING", Self::OVERDRAW => "OVERDRAW", Self::WIREFRAME => "WIREFRAME", Self::NORMAL_BUFFER => "NORMAL_BUFFER", Self::VOXEL_GI_ALBEDO => "VOXEL_GI_ALBEDO", Self::VOXEL_GI_LIGHTING => "VOXEL_GI_LIGHTING", Self::VOXEL_GI_EMISSION => "VOXEL_GI_EMISSION", Self::SHADOW_ATLAS => "SHADOW_ATLAS", Self::DIRECTIONAL_SHADOW_ATLAS => "DIRECTIONAL_SHADOW_ATLAS", Self::SCENE_LUMINANCE => "SCENE_LUMINANCE", Self::SSAO => "SSAO", Self::SSIL => "SSIL", Self::PSSM_SPLITS => "PSSM_SPLITS", Self::DECAL_ATLAS => "DECAL_ATLAS", Self::SDFGI => "SDFGI", Self::SDFGI_PROBES => "SDFGI_PROBES", Self::GI_BUFFER => "GI_BUFFER", Self::DISABLE_LOD => "DISABLE_LOD", Self::CLUSTER_OMNI_LIGHTS => "CLUSTER_OMNI_LIGHTS", Self::CLUSTER_SPOT_LIGHTS => "CLUSTER_SPOT_LIGHTS", Self::CLUSTER_DECALS => "CLUSTER_DECALS", Self::CLUSTER_REFLECTION_PROBES => "CLUSTER_REFLECTION_PROBES", Self::OCCLUDERS => "OCCLUDERS", Self::MOTION_VECTORS => "MOTION_VECTORS", Self::INTERNAL_BUFFER => "INTERNAL_BUFFER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportDebugDraw::DISABLED, ViewportDebugDraw::UNSHADED, ViewportDebugDraw::LIGHTING, ViewportDebugDraw::OVERDRAW, ViewportDebugDraw::WIREFRAME, ViewportDebugDraw::NORMAL_BUFFER, ViewportDebugDraw::VOXEL_GI_ALBEDO, ViewportDebugDraw::VOXEL_GI_LIGHTING, ViewportDebugDraw::VOXEL_GI_EMISSION, ViewportDebugDraw::SHADOW_ATLAS, ViewportDebugDraw::DIRECTIONAL_SHADOW_ATLAS, ViewportDebugDraw::SCENE_LUMINANCE, ViewportDebugDraw::SSAO, ViewportDebugDraw::SSIL, ViewportDebugDraw::PSSM_SPLITS, ViewportDebugDraw::DECAL_ATLAS, ViewportDebugDraw::SDFGI, ViewportDebugDraw::SDFGI_PROBES, ViewportDebugDraw::GI_BUFFER, ViewportDebugDraw::DISABLE_LOD, ViewportDebugDraw::CLUSTER_OMNI_LIGHTS, ViewportDebugDraw::CLUSTER_SPOT_LIGHTS, ViewportDebugDraw::CLUSTER_DECALS, ViewportDebugDraw::CLUSTER_REFLECTION_PROBES, ViewportDebugDraw::OCCLUDERS, ViewportDebugDraw::MOTION_VECTORS, ViewportDebugDraw::INTERNAL_BUFFER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportDebugDraw >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_DEBUG_DRAW_DISABLED", ViewportDebugDraw::DISABLED), crate::meta::inspect::EnumConstant::new("UNSHADED", "VIEWPORT_DEBUG_DRAW_UNSHADED", ViewportDebugDraw::UNSHADED), crate::meta::inspect::EnumConstant::new("LIGHTING", "VIEWPORT_DEBUG_DRAW_LIGHTING", ViewportDebugDraw::LIGHTING), crate::meta::inspect::EnumConstant::new("OVERDRAW", "VIEWPORT_DEBUG_DRAW_OVERDRAW", ViewportDebugDraw::OVERDRAW), crate::meta::inspect::EnumConstant::new("WIREFRAME", "VIEWPORT_DEBUG_DRAW_WIREFRAME", ViewportDebugDraw::WIREFRAME), crate::meta::inspect::EnumConstant::new("NORMAL_BUFFER", "VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER", ViewportDebugDraw::NORMAL_BUFFER), crate::meta::inspect::EnumConstant::new("VOXEL_GI_ALBEDO", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO", ViewportDebugDraw::VOXEL_GI_ALBEDO), crate::meta::inspect::EnumConstant::new("VOXEL_GI_LIGHTING", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING", ViewportDebugDraw::VOXEL_GI_LIGHTING), crate::meta::inspect::EnumConstant::new("VOXEL_GI_EMISSION", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION", ViewportDebugDraw::VOXEL_GI_EMISSION), crate::meta::inspect::EnumConstant::new("SHADOW_ATLAS", "VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS", ViewportDebugDraw::SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("DIRECTIONAL_SHADOW_ATLAS", "VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS", ViewportDebugDraw::DIRECTIONAL_SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("SCENE_LUMINANCE", "VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE", ViewportDebugDraw::SCENE_LUMINANCE), crate::meta::inspect::EnumConstant::new("SSAO", "VIEWPORT_DEBUG_DRAW_SSAO", ViewportDebugDraw::SSAO), crate::meta::inspect::EnumConstant::new("SSIL", "VIEWPORT_DEBUG_DRAW_SSIL", ViewportDebugDraw::SSIL), crate::meta::inspect::EnumConstant::new("PSSM_SPLITS", "VIEWPORT_DEBUG_DRAW_PSSM_SPLITS", ViewportDebugDraw::PSSM_SPLITS), crate::meta::inspect::EnumConstant::new("DECAL_ATLAS", "VIEWPORT_DEBUG_DRAW_DECAL_ATLAS", ViewportDebugDraw::DECAL_ATLAS), crate::meta::inspect::EnumConstant::new("SDFGI", "VIEWPORT_DEBUG_DRAW_SDFGI", ViewportDebugDraw::SDFGI), crate::meta::inspect::EnumConstant::new("SDFGI_PROBES", "VIEWPORT_DEBUG_DRAW_SDFGI_PROBES", ViewportDebugDraw::SDFGI_PROBES), crate::meta::inspect::EnumConstant::new("GI_BUFFER", "VIEWPORT_DEBUG_DRAW_GI_BUFFER", ViewportDebugDraw::GI_BUFFER), crate::meta::inspect::EnumConstant::new("DISABLE_LOD", "VIEWPORT_DEBUG_DRAW_DISABLE_LOD", ViewportDebugDraw::DISABLE_LOD), crate::meta::inspect::EnumConstant::new("CLUSTER_OMNI_LIGHTS", "VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS", ViewportDebugDraw::CLUSTER_OMNI_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_SPOT_LIGHTS", "VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS", ViewportDebugDraw::CLUSTER_SPOT_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_DECALS", "VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS", ViewportDebugDraw::CLUSTER_DECALS), crate::meta::inspect::EnumConstant::new("CLUSTER_REFLECTION_PROBES", "VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES", ViewportDebugDraw::CLUSTER_REFLECTION_PROBES), crate::meta::inspect::EnumConstant::new("OCCLUDERS", "VIEWPORT_DEBUG_DRAW_OCCLUDERS", ViewportDebugDraw::OCCLUDERS), crate::meta::inspect::EnumConstant::new("MOTION_VECTORS", "VIEWPORT_DEBUG_DRAW_MOTION_VECTORS", ViewportDebugDraw::MOTION_VECTORS), crate::meta::inspect::EnumConstant::new("INTERNAL_BUFFER", "VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER", ViewportDebugDraw::INTERNAL_BUFFER)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportDebugDraw {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Debug Draw Disabled", 0i64), EnumeratorShape::new_int("Viewport Debug Draw Unshaded", 1i64), EnumeratorShape::new_int("Viewport Debug Draw Lighting", 2i64), EnumeratorShape::new_int("Viewport Debug Draw Overdraw", 3i64), EnumeratorShape::new_int("Viewport Debug Draw Wireframe", 4i64), EnumeratorShape::new_int("Viewport Debug Draw Normal Buffer", 5i64), EnumeratorShape::new_int("Viewport Debug Draw Voxel Gi Albedo", 6i64), EnumeratorShape::new_int("Viewport Debug Draw Voxel Gi Lighting", 7i64), EnumeratorShape::new_int("Viewport Debug Draw Voxel Gi Emission", 8i64), EnumeratorShape::new_int("Viewport Debug Draw Shadow Atlas", 9i64), EnumeratorShape::new_int("Viewport Debug Draw Directional Shadow Atlas", 10i64), EnumeratorShape::new_int("Viewport Debug Draw Scene Luminance", 11i64), EnumeratorShape::new_int("Viewport Debug Draw Ssao", 12i64), EnumeratorShape::new_int("Viewport Debug Draw Ssil", 13i64), EnumeratorShape::new_int("Viewport Debug Draw Pssm Splits", 14i64), EnumeratorShape::new_int("Viewport Debug Draw Decal Atlas", 15i64), EnumeratorShape::new_int("Viewport Debug Draw Sdfgi", 16i64), EnumeratorShape::new_int("Viewport Debug Draw Sdfgi Probes", 17i64), EnumeratorShape::new_int("Viewport Debug Draw Gi Buffer", 18i64), EnumeratorShape::new_int("Viewport Debug Draw Disable Lod", 19i64), EnumeratorShape::new_int("Viewport Debug Draw Cluster Omni Lights", 20i64), EnumeratorShape::new_int("Viewport Debug Draw Cluster Spot Lights", 21i64), EnumeratorShape::new_int("Viewport Debug Draw Cluster Decals", 22i64), EnumeratorShape::new_int("Viewport Debug Draw Cluster Reflection Probes", 23i64), EnumeratorShape::new_int("Viewport Debug Draw Occluders", 24i64), EnumeratorShape::new_int("Viewport Debug Draw Motion Vectors", 25i64), EnumeratorShape::new_int("Viewport Debug Draw Internal Buffer", 26i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportDebugDraw")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportDebugDraw {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportDebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportDebugDraw {
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
impl crate::registry::property::Export for ViewportDebugDraw {
    
}
impl crate::meta::Element for ViewportDebugDraw {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportVRSMode`."]
pub struct ViewportVrsMode {
    ord: i32
}
impl ViewportVrsMode {
    #[doc(alias = "VIEWPORT_VRS_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_DISABLED`"]
    pub const DISABLED: ViewportVrsMode = ViewportVrsMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_VRS_TEXTURE")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_TEXTURE`"]
    pub const TEXTURE: ViewportVrsMode = ViewportVrsMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_VRS_XR")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_XR`"]
    pub const XR: ViewportVrsMode = ViewportVrsMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_VRS_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_MAX`"]
    pub const MAX: ViewportVrsMode = ViewportVrsMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportVrsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportVrsMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportVrsMode {
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
            Self::DISABLED => "DISABLED", Self::TEXTURE => "TEXTURE", Self::XR => "XR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportVrsMode::DISABLED, ViewportVrsMode::TEXTURE, ViewportVrsMode::XR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportVrsMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_VRS_DISABLED", ViewportVrsMode::DISABLED), crate::meta::inspect::EnumConstant::new("TEXTURE", "VIEWPORT_VRS_TEXTURE", ViewportVrsMode::TEXTURE), crate::meta::inspect::EnumConstant::new("XR", "VIEWPORT_VRS_XR", ViewportVrsMode::XR), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_VRS_MAX", ViewportVrsMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportVrsMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportVrsMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Vrs Disabled", 0i64), EnumeratorShape::new_int("Viewport Vrs Texture", 1i64), EnumeratorShape::new_int("Viewport Vrs Xr", 2i64), EnumeratorShape::new_int("Viewport Vrs Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportVRSMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportVrsMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportVrsMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportVrsMode {
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
impl crate::registry::property::Export for ViewportVrsMode {
    
}
impl crate::meta::Element for ViewportVrsMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportVRSUpdateMode`."]
pub struct ViewportVrsUpdateMode {
    ord: i32
}
impl ViewportVrsUpdateMode {
    #[doc(alias = "VIEWPORT_VRS_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_DISABLED`"]
    pub const DISABLED: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_ONCE`"]
    pub const ONCE: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_ALWAYS`"]
    pub const ALWAYS: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_MAX`"]
    pub const MAX: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportVrsUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportVrsUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportVrsUpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportVrsUpdateMode::DISABLED, ViewportVrsUpdateMode::ONCE, ViewportVrsUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportVrsUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_VRS_UPDATE_DISABLED", ViewportVrsUpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "VIEWPORT_VRS_UPDATE_ONCE", ViewportVrsUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_VRS_UPDATE_ALWAYS", ViewportVrsUpdateMode::ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_VRS_UPDATE_MAX", ViewportVrsUpdateMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportVrsUpdateMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportVrsUpdateMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Viewport Vrs Update Disabled", 0i64), EnumeratorShape::new_int("Viewport Vrs Update Once", 1i64), EnumeratorShape::new_int("Viewport Vrs Update Always", 2i64), EnumeratorShape::new_int("Viewport Vrs Update Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ViewportVRSUpdateMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ViewportVrsUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportVrsUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ViewportVrsUpdateMode {
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
impl crate::registry::property::Export for ViewportVrsUpdateMode {
    
}
impl crate::meta::Element for ViewportVrsUpdateMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkyMode {
    ord: i32
}
impl SkyMode {
    #[doc(alias = "SKY_MODE_AUTOMATIC")]
    #[doc = "Godot enumerator name: `SKY_MODE_AUTOMATIC`"]
    pub const AUTOMATIC: SkyMode = SkyMode {
        ord: 0i32
    };
    #[doc(alias = "SKY_MODE_QUALITY")]
    #[doc = "Godot enumerator name: `SKY_MODE_QUALITY`"]
    pub const QUALITY: SkyMode = SkyMode {
        ord: 1i32
    };
    #[doc(alias = "SKY_MODE_INCREMENTAL")]
    #[doc = "Godot enumerator name: `SKY_MODE_INCREMENTAL`"]
    pub const INCREMENTAL: SkyMode = SkyMode {
        ord: 2i32
    };
    #[doc(alias = "SKY_MODE_REALTIME")]
    #[doc = "Godot enumerator name: `SKY_MODE_REALTIME`"]
    pub const REALTIME: SkyMode = SkyMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SkyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkyMode {
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
            Self::AUTOMATIC => "AUTOMATIC", Self::QUALITY => "QUALITY", Self::INCREMENTAL => "INCREMENTAL", Self::REALTIME => "REALTIME", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SkyMode::AUTOMATIC, SkyMode::QUALITY, SkyMode::INCREMENTAL, SkyMode::REALTIME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SkyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTOMATIC", "SKY_MODE_AUTOMATIC", SkyMode::AUTOMATIC), crate::meta::inspect::EnumConstant::new("QUALITY", "SKY_MODE_QUALITY", SkyMode::QUALITY), crate::meta::inspect::EnumConstant::new("INCREMENTAL", "SKY_MODE_INCREMENTAL", SkyMode::INCREMENTAL), crate::meta::inspect::EnumConstant::new("REALTIME", "SKY_MODE_REALTIME", SkyMode::REALTIME)]
        }
    }
}
impl crate::meta::GodotConvert for SkyMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Sky Mode Automatic", 0i64), EnumeratorShape::new_int("Sky Mode Quality", 1i64), EnumeratorShape::new_int("Sky Mode Incremental", 2i64), EnumeratorShape::new_int("Sky Mode Realtime", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.SkyMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SkyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SkyMode {
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
impl crate::registry::property::Export for SkyMode {
    
}
impl crate::meta::Element for SkyMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompositorEffectFlags {
    ord: i32
}
impl CompositorEffectFlags {
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR`"]
    pub const ACCESS_RESOLVED_COLOR: CompositorEffectFlags = CompositorEffectFlags {
        ord: 1i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH`"]
    pub const ACCESS_RESOLVED_DEPTH: CompositorEffectFlags = CompositorEffectFlags {
        ord: 2i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS`"]
    pub const NEEDS_MOTION_VECTORS: CompositorEffectFlags = CompositorEffectFlags {
        ord: 4i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS`"]
    pub const NEEDS_ROUGHNESS: CompositorEffectFlags = CompositorEffectFlags {
        ord: 8i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR`"]
    pub const NEEDS_SEPARATE_SPECULAR: CompositorEffectFlags = CompositorEffectFlags {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CompositorEffectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompositorEffectFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompositorEffectFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
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
            Self::ACCESS_RESOLVED_COLOR => "ACCESS_RESOLVED_COLOR", Self::ACCESS_RESOLVED_DEPTH => "ACCESS_RESOLVED_DEPTH", Self::NEEDS_MOTION_VECTORS => "NEEDS_MOTION_VECTORS", Self::NEEDS_ROUGHNESS => "NEEDS_ROUGHNESS", Self::NEEDS_SEPARATE_SPECULAR => "NEEDS_SEPARATE_SPECULAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompositorEffectFlags::ACCESS_RESOLVED_COLOR, CompositorEffectFlags::ACCESS_RESOLVED_DEPTH, CompositorEffectFlags::NEEDS_MOTION_VECTORS, CompositorEffectFlags::NEEDS_ROUGHNESS, CompositorEffectFlags::NEEDS_SEPARATE_SPECULAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompositorEffectFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ACCESS_RESOLVED_COLOR", "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR", CompositorEffectFlags::ACCESS_RESOLVED_COLOR), crate::meta::inspect::EnumConstant::new("ACCESS_RESOLVED_DEPTH", "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH", CompositorEffectFlags::ACCESS_RESOLVED_DEPTH), crate::meta::inspect::EnumConstant::new("NEEDS_MOTION_VECTORS", "COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS", CompositorEffectFlags::NEEDS_MOTION_VECTORS), crate::meta::inspect::EnumConstant::new("NEEDS_ROUGHNESS", "COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS", CompositorEffectFlags::NEEDS_ROUGHNESS), crate::meta::inspect::EnumConstant::new("NEEDS_SEPARATE_SPECULAR", "COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR", CompositorEffectFlags::NEEDS_SEPARATE_SPECULAR)]
        }
    }
}
impl crate::meta::GodotConvert for CompositorEffectFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Compositor Effect Flag Access Resolved Color", 1i64), EnumeratorShape::new_int("Compositor Effect Flag Access Resolved Depth", 2i64), EnumeratorShape::new_int("Compositor Effect Flag Needs Motion Vectors", 4i64), EnumeratorShape::new_int("Compositor Effect Flag Needs Roughness", 8i64), EnumeratorShape::new_int("Compositor Effect Flag Needs Separate Specular", 16i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CompositorEffectFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CompositorEffectFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompositorEffectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CompositorEffectFlags {
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
impl crate::registry::property::Export for CompositorEffectFlags {
    
}
impl crate::meta::Element for CompositorEffectFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompositorEffectCallbackType {
    ord: i32
}
impl CompositorEffectCallbackType {
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE`"]
    pub const PRE_OPAQUE: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 0i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE`"]
    pub const POST_OPAQUE: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 1i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY`"]
    pub const POST_SKY: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 2i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT`"]
    pub const PRE_TRANSPARENT: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 3i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT`"]
    pub const POST_TRANSPARENT: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 4i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY`"]
    pub const ANY: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: - 1i32
    };
    
}
impl std::fmt::Debug for CompositorEffectCallbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompositorEffectCallbackType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompositorEffectCallbackType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::PRE_OPAQUE => "PRE_OPAQUE", Self::POST_OPAQUE => "POST_OPAQUE", Self::POST_SKY => "POST_SKY", Self::PRE_TRANSPARENT => "PRE_TRANSPARENT", Self::POST_TRANSPARENT => "POST_TRANSPARENT", Self::ANY => "ANY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompositorEffectCallbackType::PRE_OPAQUE, CompositorEffectCallbackType::POST_OPAQUE, CompositorEffectCallbackType::POST_SKY, CompositorEffectCallbackType::PRE_TRANSPARENT, CompositorEffectCallbackType::POST_TRANSPARENT, CompositorEffectCallbackType::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompositorEffectCallbackType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PRE_OPAQUE", "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE", CompositorEffectCallbackType::PRE_OPAQUE), crate::meta::inspect::EnumConstant::new("POST_OPAQUE", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE", CompositorEffectCallbackType::POST_OPAQUE), crate::meta::inspect::EnumConstant::new("POST_SKY", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY", CompositorEffectCallbackType::POST_SKY), crate::meta::inspect::EnumConstant::new("PRE_TRANSPARENT", "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT", CompositorEffectCallbackType::PRE_TRANSPARENT), crate::meta::inspect::EnumConstant::new("POST_TRANSPARENT", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT", CompositorEffectCallbackType::POST_TRANSPARENT), crate::meta::inspect::EnumConstant::new("ANY", "COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY", CompositorEffectCallbackType::ANY)]
        }
    }
}
impl crate::meta::GodotConvert for CompositorEffectCallbackType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Compositor Effect Callback Type Pre Opaque", 0i64), EnumeratorShape::new_int("Compositor Effect Callback Type Post Opaque", 1i64), EnumeratorShape::new_int("Compositor Effect Callback Type Post Sky", 2i64), EnumeratorShape::new_int("Compositor Effect Callback Type Pre Transparent", 3i64), EnumeratorShape::new_int("Compositor Effect Callback Type Post Transparent", 4i64), EnumeratorShape::new_int("Compositor Effect Callback Type Any", - 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CompositorEffectCallbackType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CompositorEffectCallbackType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompositorEffectCallbackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CompositorEffectCallbackType {
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
impl crate::registry::property::Export for CompositorEffectCallbackType {
    
}
impl crate::meta::Element for CompositorEffectCallbackType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentBG`."]
pub struct EnvironmentBg {
    ord: i32
}
impl EnvironmentBg {
    #[doc(alias = "ENV_BG_CLEAR_COLOR")]
    #[doc = "Godot enumerator name: `ENV_BG_CLEAR_COLOR`"]
    pub const CLEAR_COLOR: EnvironmentBg = EnvironmentBg {
        ord: 0i32
    };
    #[doc(alias = "ENV_BG_COLOR")]
    #[doc = "Godot enumerator name: `ENV_BG_COLOR`"]
    pub const COLOR: EnvironmentBg = EnvironmentBg {
        ord: 1i32
    };
    #[doc(alias = "ENV_BG_SKY")]
    #[doc = "Godot enumerator name: `ENV_BG_SKY`"]
    pub const SKY: EnvironmentBg = EnvironmentBg {
        ord: 2i32
    };
    #[doc(alias = "ENV_BG_CANVAS")]
    #[doc = "Godot enumerator name: `ENV_BG_CANVAS`"]
    pub const CANVAS: EnvironmentBg = EnvironmentBg {
        ord: 3i32
    };
    #[doc(alias = "ENV_BG_KEEP")]
    #[doc = "Godot enumerator name: `ENV_BG_KEEP`"]
    pub const KEEP: EnvironmentBg = EnvironmentBg {
        ord: 4i32
    };
    #[doc(alias = "ENV_BG_CAMERA_FEED")]
    #[doc = "Godot enumerator name: `ENV_BG_CAMERA_FEED`"]
    pub const CAMERA_FEED: EnvironmentBg = EnvironmentBg {
        ord: 5i32
    };
    #[doc(alias = "ENV_BG_MAX")]
    #[doc = "Godot enumerator name: `ENV_BG_MAX`"]
    pub const MAX: EnvironmentBg = EnvironmentBg {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for EnvironmentBg {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentBg") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentBg {
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
            Self::CLEAR_COLOR => "CLEAR_COLOR", Self::COLOR => "COLOR", Self::SKY => "SKY", Self::CANVAS => "CANVAS", Self::KEEP => "KEEP", Self::CAMERA_FEED => "CAMERA_FEED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentBg::CLEAR_COLOR, EnvironmentBg::COLOR, EnvironmentBg::SKY, EnvironmentBg::CANVAS, EnvironmentBg::KEEP, EnvironmentBg::CAMERA_FEED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentBg >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLEAR_COLOR", "ENV_BG_CLEAR_COLOR", EnvironmentBg::CLEAR_COLOR), crate::meta::inspect::EnumConstant::new("COLOR", "ENV_BG_COLOR", EnvironmentBg::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "ENV_BG_SKY", EnvironmentBg::SKY), crate::meta::inspect::EnumConstant::new("CANVAS", "ENV_BG_CANVAS", EnvironmentBg::CANVAS), crate::meta::inspect::EnumConstant::new("KEEP", "ENV_BG_KEEP", EnvironmentBg::KEEP), crate::meta::inspect::EnumConstant::new("CAMERA_FEED", "ENV_BG_CAMERA_FEED", EnvironmentBg::CAMERA_FEED), crate::meta::inspect::EnumConstant::new("MAX", "ENV_BG_MAX", EnvironmentBg::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentBg {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for EnvironmentBg {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Bg Clear Color", 0i64), EnumeratorShape::new_int("Env Bg Color", 1i64), EnumeratorShape::new_int("Env Bg Sky", 2i64), EnumeratorShape::new_int("Env Bg Canvas", 3i64), EnumeratorShape::new_int("Env Bg Keep", 4i64), EnumeratorShape::new_int("Env Bg Camera Feed", 5i64), EnumeratorShape::new_int("Env Bg Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentBG")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentBg {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentBg {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentBg {
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
impl crate::registry::property::Export for EnvironmentBg {
    
}
impl crate::meta::Element for EnvironmentBg {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentAmbientSource {
    ord: i32
}
impl EnvironmentAmbientSource {
    #[doc(alias = "ENV_AMBIENT_SOURCE_BG")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_BG`"]
    pub const BG: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 0i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_DISABLED`"]
    pub const DISABLED: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 1i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_COLOR")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_COLOR`"]
    pub const COLOR: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 2i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_SKY`"]
    pub const SKY: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentAmbientSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentAmbientSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentAmbientSource {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::COLOR => "COLOR", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentAmbientSource::BG, EnvironmentAmbientSource::DISABLED, EnvironmentAmbientSource::COLOR, EnvironmentAmbientSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentAmbientSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "ENV_AMBIENT_SOURCE_BG", EnvironmentAmbientSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_AMBIENT_SOURCE_DISABLED", EnvironmentAmbientSource::DISABLED), crate::meta::inspect::EnumConstant::new("COLOR", "ENV_AMBIENT_SOURCE_COLOR", EnvironmentAmbientSource::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "ENV_AMBIENT_SOURCE_SKY", EnvironmentAmbientSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentAmbientSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Ambient Source Bg", 0i64), EnumeratorShape::new_int("Env Ambient Source Disabled", 1i64), EnumeratorShape::new_int("Env Ambient Source Color", 2i64), EnumeratorShape::new_int("Env Ambient Source Sky", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentAmbientSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentAmbientSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentAmbientSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentAmbientSource {
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
impl crate::registry::property::Export for EnvironmentAmbientSource {
    
}
impl crate::meta::Element for EnvironmentAmbientSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentReflectionSource {
    ord: i32
}
impl EnvironmentReflectionSource {
    #[doc(alias = "ENV_REFLECTION_SOURCE_BG")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_BG`"]
    pub const BG: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 0i32
    };
    #[doc(alias = "ENV_REFLECTION_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_DISABLED`"]
    pub const DISABLED: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 1i32
    };
    #[doc(alias = "ENV_REFLECTION_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_SKY`"]
    pub const SKY: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentReflectionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentReflectionSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentReflectionSource {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentReflectionSource::BG, EnvironmentReflectionSource::DISABLED, EnvironmentReflectionSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentReflectionSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "ENV_REFLECTION_SOURCE_BG", EnvironmentReflectionSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_REFLECTION_SOURCE_DISABLED", EnvironmentReflectionSource::DISABLED), crate::meta::inspect::EnumConstant::new("SKY", "ENV_REFLECTION_SOURCE_SKY", EnvironmentReflectionSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentReflectionSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Reflection Source Bg", 0i64), EnumeratorShape::new_int("Env Reflection Source Disabled", 1i64), EnumeratorShape::new_int("Env Reflection Source Sky", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentReflectionSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentReflectionSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentReflectionSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentReflectionSource {
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
impl crate::registry::property::Export for EnvironmentReflectionSource {
    
}
impl crate::meta::Element for EnvironmentReflectionSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentGlowBlendMode {
    ord: i32
}
impl EnvironmentGlowBlendMode {
    #[doc(alias = "ENV_GLOW_BLEND_MODE_ADDITIVE")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_ADDITIVE`"]
    pub const ADDITIVE: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 0i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_SCREEN`"]
    pub const SCREEN: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 1i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_SOFTLIGHT")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_SOFTLIGHT`"]
    pub const SOFTLIGHT: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 2i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_REPLACE`"]
    pub const REPLACE: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 3i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_MIX`"]
    pub const MIX: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentGlowBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentGlowBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentGlowBlendMode {
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
            Self::ADDITIVE => "ADDITIVE", Self::SCREEN => "SCREEN", Self::SOFTLIGHT => "SOFTLIGHT", Self::REPLACE => "REPLACE", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentGlowBlendMode::ADDITIVE, EnvironmentGlowBlendMode::SCREEN, EnvironmentGlowBlendMode::SOFTLIGHT, EnvironmentGlowBlendMode::REPLACE, EnvironmentGlowBlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentGlowBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADDITIVE", "ENV_GLOW_BLEND_MODE_ADDITIVE", EnvironmentGlowBlendMode::ADDITIVE), crate::meta::inspect::EnumConstant::new("SCREEN", "ENV_GLOW_BLEND_MODE_SCREEN", EnvironmentGlowBlendMode::SCREEN), crate::meta::inspect::EnumConstant::new("SOFTLIGHT", "ENV_GLOW_BLEND_MODE_SOFTLIGHT", EnvironmentGlowBlendMode::SOFTLIGHT), crate::meta::inspect::EnumConstant::new("REPLACE", "ENV_GLOW_BLEND_MODE_REPLACE", EnvironmentGlowBlendMode::REPLACE), crate::meta::inspect::EnumConstant::new("MIX", "ENV_GLOW_BLEND_MODE_MIX", EnvironmentGlowBlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentGlowBlendMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Glow Blend Mode Additive", 0i64), EnumeratorShape::new_int("Env Glow Blend Mode Screen", 1i64), EnumeratorShape::new_int("Env Glow Blend Mode Softlight", 2i64), EnumeratorShape::new_int("Env Glow Blend Mode Replace", 3i64), EnumeratorShape::new_int("Env Glow Blend Mode Mix", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentGlowBlendMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentGlowBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentGlowBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentGlowBlendMode {
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
impl crate::registry::property::Export for EnvironmentGlowBlendMode {
    
}
impl crate::meta::Element for EnvironmentGlowBlendMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentFogMode {
    ord: i32
}
impl EnvironmentFogMode {
    #[doc(alias = "ENV_FOG_MODE_EXPONENTIAL")]
    #[doc = "Godot enumerator name: `ENV_FOG_MODE_EXPONENTIAL`"]
    pub const EXPONENTIAL: EnvironmentFogMode = EnvironmentFogMode {
        ord: 0i32
    };
    #[doc(alias = "ENV_FOG_MODE_DEPTH")]
    #[doc = "Godot enumerator name: `ENV_FOG_MODE_DEPTH`"]
    pub const DEPTH: EnvironmentFogMode = EnvironmentFogMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for EnvironmentFogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentFogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentFogMode {
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
            Self::EXPONENTIAL => "EXPONENTIAL", Self::DEPTH => "DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentFogMode::EXPONENTIAL, EnvironmentFogMode::DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentFogMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EXPONENTIAL", "ENV_FOG_MODE_EXPONENTIAL", EnvironmentFogMode::EXPONENTIAL), crate::meta::inspect::EnumConstant::new("DEPTH", "ENV_FOG_MODE_DEPTH", EnvironmentFogMode::DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentFogMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Fog Mode Exponential", 0i64), EnumeratorShape::new_int("Env Fog Mode Depth", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentFogMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentFogMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentFogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentFogMode {
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
impl crate::registry::property::Export for EnvironmentFogMode {
    
}
impl crate::meta::Element for EnvironmentFogMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentToneMapper {
    ord: i32
}
impl EnvironmentToneMapper {
    #[doc(alias = "ENV_TONE_MAPPER_LINEAR")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_LINEAR`"]
    pub const LINEAR: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 0i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_REINHARD")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_REINHARD`"]
    pub const REINHARD: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 1i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_FILMIC")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_FILMIC`"]
    pub const FILMIC: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 2i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_ACES")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_ACES`"]
    pub const ACES: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 3i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_AGX")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_AGX`"]
    pub const AGX: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentToneMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentToneMapper") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentToneMapper {
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
            Self::LINEAR => "LINEAR", Self::REINHARD => "REINHARD", Self::FILMIC => "FILMIC", Self::ACES => "ACES", Self::AGX => "AGX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentToneMapper::LINEAR, EnvironmentToneMapper::REINHARD, EnvironmentToneMapper::FILMIC, EnvironmentToneMapper::ACES, EnvironmentToneMapper::AGX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentToneMapper >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "ENV_TONE_MAPPER_LINEAR", EnvironmentToneMapper::LINEAR), crate::meta::inspect::EnumConstant::new("REINHARD", "ENV_TONE_MAPPER_REINHARD", EnvironmentToneMapper::REINHARD), crate::meta::inspect::EnumConstant::new("FILMIC", "ENV_TONE_MAPPER_FILMIC", EnvironmentToneMapper::FILMIC), crate::meta::inspect::EnumConstant::new("ACES", "ENV_TONE_MAPPER_ACES", EnvironmentToneMapper::ACES), crate::meta::inspect::EnumConstant::new("AGX", "ENV_TONE_MAPPER_AGX", EnvironmentToneMapper::AGX)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentToneMapper {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Tone Mapper Linear", 0i64), EnumeratorShape::new_int("Env Tone Mapper Reinhard", 1i64), EnumeratorShape::new_int("Env Tone Mapper Filmic", 2i64), EnumeratorShape::new_int("Env Tone Mapper Aces", 3i64), EnumeratorShape::new_int("Env Tone Mapper Agx", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentToneMapper")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentToneMapper {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentToneMapper {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentToneMapper {
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
impl crate::registry::property::Export for EnvironmentToneMapper {
    
}
impl crate::meta::Element for EnvironmentToneMapper {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSRRoughnessQuality`."]
pub struct EnvironmentSsrRoughnessQuality {
    ord: i32
}
impl EnvironmentSsrRoughnessQuality {
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_DISABLED`"]
    pub const DISABLED: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsrRoughnessQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsrRoughnessQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsrRoughnessQuality {
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
            Self::DISABLED => "DISABLED", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsrRoughnessQuality::DISABLED, EnvironmentSsrRoughnessQuality::LOW, EnvironmentSsrRoughnessQuality::MEDIUM, EnvironmentSsrRoughnessQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsrRoughnessQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_SSR_ROUGHNESS_QUALITY_DISABLED", EnvironmentSsrRoughnessQuality::DISABLED), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSR_ROUGHNESS_QUALITY_LOW", EnvironmentSsrRoughnessQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSR_ROUGHNESS_QUALITY_MEDIUM", EnvironmentSsrRoughnessQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSR_ROUGHNESS_QUALITY_HIGH", EnvironmentSsrRoughnessQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsrRoughnessQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Ssr Roughness Quality Disabled", 0i64), EnumeratorShape::new_int("Env Ssr Roughness Quality Low", 1i64), EnumeratorShape::new_int("Env Ssr Roughness Quality Medium", 2i64), EnumeratorShape::new_int("Env Ssr Roughness Quality High", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSSRRoughnessQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSsrRoughnessQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsrRoughnessQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSsrRoughnessQuality {
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
impl crate::registry::property::Export for EnvironmentSsrRoughnessQuality {
    
}
impl crate::meta::Element for EnvironmentSsrRoughnessQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSAOQuality`."]
pub struct EnvironmentSsaoQuality {
    ord: i32
}
impl EnvironmentSsaoQuality {
    #[doc(alias = "ENV_SSAO_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 3i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_ULTRA`"]
    pub const ULTRA: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsaoQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsaoQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsaoQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsaoQuality::VERY_LOW, EnvironmentSsaoQuality::LOW, EnvironmentSsaoQuality::MEDIUM, EnvironmentSsaoQuality::HIGH, EnvironmentSsaoQuality::ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsaoQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "ENV_SSAO_QUALITY_VERY_LOW", EnvironmentSsaoQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSAO_QUALITY_LOW", EnvironmentSsaoQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSAO_QUALITY_MEDIUM", EnvironmentSsaoQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSAO_QUALITY_HIGH", EnvironmentSsaoQuality::HIGH), crate::meta::inspect::EnumConstant::new("ULTRA", "ENV_SSAO_QUALITY_ULTRA", EnvironmentSsaoQuality::ULTRA)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsaoQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Ssao Quality Very Low", 0i64), EnumeratorShape::new_int("Env Ssao Quality Low", 1i64), EnumeratorShape::new_int("Env Ssao Quality Medium", 2i64), EnumeratorShape::new_int("Env Ssao Quality High", 3i64), EnumeratorShape::new_int("Env Ssao Quality Ultra", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSSAOQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSsaoQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsaoQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSsaoQuality {
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
impl crate::registry::property::Export for EnvironmentSsaoQuality {
    
}
impl crate::meta::Element for EnvironmentSsaoQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSILQuality`."]
pub struct EnvironmentSsilQuality {
    ord: i32
}
impl EnvironmentSsilQuality {
    #[doc(alias = "ENV_SSIL_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 3i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_ULTRA`"]
    pub const ULTRA: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsilQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsilQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsilQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsilQuality::VERY_LOW, EnvironmentSsilQuality::LOW, EnvironmentSsilQuality::MEDIUM, EnvironmentSsilQuality::HIGH, EnvironmentSsilQuality::ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsilQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "ENV_SSIL_QUALITY_VERY_LOW", EnvironmentSsilQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSIL_QUALITY_LOW", EnvironmentSsilQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSIL_QUALITY_MEDIUM", EnvironmentSsilQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSIL_QUALITY_HIGH", EnvironmentSsilQuality::HIGH), crate::meta::inspect::EnumConstant::new("ULTRA", "ENV_SSIL_QUALITY_ULTRA", EnvironmentSsilQuality::ULTRA)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsilQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Ssil Quality Very Low", 0i64), EnumeratorShape::new_int("Env Ssil Quality Low", 1i64), EnumeratorShape::new_int("Env Ssil Quality Medium", 2i64), EnumeratorShape::new_int("Env Ssil Quality High", 3i64), EnumeratorShape::new_int("Env Ssil Quality Ultra", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSSILQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSsilQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsilQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSsilQuality {
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
impl crate::registry::property::Export for EnvironmentSsilQuality {
    
}
impl crate::meta::Element for EnvironmentSsilQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIYScale`."]
pub struct EnvironmentSdfgiYScale {
    ord: i32
}
impl EnvironmentSdfgiYScale {
    #[doc(alias = "ENV_SDFGI_Y_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_Y_SCALE_75_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_75_PERCENT`"]
    pub const SCALE_75_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_Y_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiYScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiYScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiYScale {
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
            Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_75_PERCENT => "SCALE_75_PERCENT", Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiYScale::SCALE_50_PERCENT, EnvironmentSdfgiYScale::SCALE_75_PERCENT, EnvironmentSdfgiYScale::SCALE_100_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiYScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "ENV_SDFGI_Y_SCALE_50_PERCENT", EnvironmentSdfgiYScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_75_PERCENT", "ENV_SDFGI_Y_SCALE_75_PERCENT", EnvironmentSdfgiYScale::SCALE_75_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "ENV_SDFGI_Y_SCALE_100_PERCENT", EnvironmentSdfgiYScale::SCALE_100_PERCENT)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSdfgiYScale {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Sdfgi Y Scale 50 Percent", 0i64), EnumeratorShape::new_int("Env Sdfgi Y Scale 75 Percent", 1i64), EnumeratorShape::new_int("Env Sdfgi Y Scale 100 Percent", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSDFGIYScale")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSdfgiYScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiYScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSdfgiYScale {
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
impl crate::registry::property::Export for EnvironmentSdfgiYScale {
    
}
impl crate::meta::Element for EnvironmentSdfgiYScale {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIRayCount`."]
pub struct EnvironmentSdfgiRayCount {
    ord: i32
}
impl EnvironmentSdfgiRayCount {
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_4")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_4`"]
    pub const COUNT_4: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_8")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_8`"]
    pub const COUNT_8: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_16")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_16`"]
    pub const COUNT_16: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_32")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_32`"]
    pub const COUNT_32: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_64")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_64`"]
    pub const COUNT_64: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_96")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_96`"]
    pub const COUNT_96: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 5i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_128")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_128`"]
    pub const COUNT_128: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 6i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_MAX`"]
    pub const MAX: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiRayCount {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiRayCount") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiRayCount {
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
            Self::COUNT_4 => "COUNT_4", Self::COUNT_8 => "COUNT_8", Self::COUNT_16 => "COUNT_16", Self::COUNT_32 => "COUNT_32", Self::COUNT_64 => "COUNT_64", Self::COUNT_96 => "COUNT_96", Self::COUNT_128 => "COUNT_128", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiRayCount::COUNT_4, EnvironmentSdfgiRayCount::COUNT_8, EnvironmentSdfgiRayCount::COUNT_16, EnvironmentSdfgiRayCount::COUNT_32, EnvironmentSdfgiRayCount::COUNT_64, EnvironmentSdfgiRayCount::COUNT_96, EnvironmentSdfgiRayCount::COUNT_128]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiRayCount >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COUNT_4", "ENV_SDFGI_RAY_COUNT_4", EnvironmentSdfgiRayCount::COUNT_4), crate::meta::inspect::EnumConstant::new("COUNT_8", "ENV_SDFGI_RAY_COUNT_8", EnvironmentSdfgiRayCount::COUNT_8), crate::meta::inspect::EnumConstant::new("COUNT_16", "ENV_SDFGI_RAY_COUNT_16", EnvironmentSdfgiRayCount::COUNT_16), crate::meta::inspect::EnumConstant::new("COUNT_32", "ENV_SDFGI_RAY_COUNT_32", EnvironmentSdfgiRayCount::COUNT_32), crate::meta::inspect::EnumConstant::new("COUNT_64", "ENV_SDFGI_RAY_COUNT_64", EnvironmentSdfgiRayCount::COUNT_64), crate::meta::inspect::EnumConstant::new("COUNT_96", "ENV_SDFGI_RAY_COUNT_96", EnvironmentSdfgiRayCount::COUNT_96), crate::meta::inspect::EnumConstant::new("COUNT_128", "ENV_SDFGI_RAY_COUNT_128", EnvironmentSdfgiRayCount::COUNT_128), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_RAY_COUNT_MAX", EnvironmentSdfgiRayCount::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiRayCount {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiRayCount {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Sdfgi Ray Count 4", 0i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 8", 1i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 16", 2i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 32", 3i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 64", 4i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 96", 5i64), EnumeratorShape::new_int("Env Sdfgi Ray Count 128", 6i64), EnumeratorShape::new_int("Env Sdfgi Ray Count Max", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSDFGIRayCount")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSdfgiRayCount {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiRayCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSdfgiRayCount {
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
impl crate::registry::property::Export for EnvironmentSdfgiRayCount {
    
}
impl crate::meta::Element for EnvironmentSdfgiRayCount {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIFramesToConverge`."]
pub struct EnvironmentSdfgiFramesToConverge {
    ord: i32
}
impl EnvironmentSdfgiFramesToConverge {
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_5_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_5_FRAMES`"]
    pub const IN_5_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_10_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_10_FRAMES`"]
    pub const IN_10_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_15_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_15_FRAMES`"]
    pub const IN_15_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_20_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_20_FRAMES`"]
    pub const IN_20_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_25_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_25_FRAMES`"]
    pub const IN_25_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_30_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_30_FRAMES`"]
    pub const IN_30_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 5i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_MAX`"]
    pub const MAX: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiFramesToConverge {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiFramesToConverge") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiFramesToConverge {
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
            Self::IN_5_FRAMES => "IN_5_FRAMES", Self::IN_10_FRAMES => "IN_10_FRAMES", Self::IN_15_FRAMES => "IN_15_FRAMES", Self::IN_20_FRAMES => "IN_20_FRAMES", Self::IN_25_FRAMES => "IN_25_FRAMES", Self::IN_30_FRAMES => "IN_30_FRAMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiFramesToConverge::IN_5_FRAMES, EnvironmentSdfgiFramesToConverge::IN_10_FRAMES, EnvironmentSdfgiFramesToConverge::IN_15_FRAMES, EnvironmentSdfgiFramesToConverge::IN_20_FRAMES, EnvironmentSdfgiFramesToConverge::IN_25_FRAMES, EnvironmentSdfgiFramesToConverge::IN_30_FRAMES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiFramesToConverge >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN_5_FRAMES", "ENV_SDFGI_CONVERGE_IN_5_FRAMES", EnvironmentSdfgiFramesToConverge::IN_5_FRAMES), crate::meta::inspect::EnumConstant::new("IN_10_FRAMES", "ENV_SDFGI_CONVERGE_IN_10_FRAMES", EnvironmentSdfgiFramesToConverge::IN_10_FRAMES), crate::meta::inspect::EnumConstant::new("IN_15_FRAMES", "ENV_SDFGI_CONVERGE_IN_15_FRAMES", EnvironmentSdfgiFramesToConverge::IN_15_FRAMES), crate::meta::inspect::EnumConstant::new("IN_20_FRAMES", "ENV_SDFGI_CONVERGE_IN_20_FRAMES", EnvironmentSdfgiFramesToConverge::IN_20_FRAMES), crate::meta::inspect::EnumConstant::new("IN_25_FRAMES", "ENV_SDFGI_CONVERGE_IN_25_FRAMES", EnvironmentSdfgiFramesToConverge::IN_25_FRAMES), crate::meta::inspect::EnumConstant::new("IN_30_FRAMES", "ENV_SDFGI_CONVERGE_IN_30_FRAMES", EnvironmentSdfgiFramesToConverge::IN_30_FRAMES), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_CONVERGE_MAX", EnvironmentSdfgiFramesToConverge::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiFramesToConverge {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiFramesToConverge {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Sdfgi Converge In 5 Frames", 0i64), EnumeratorShape::new_int("Env Sdfgi Converge In 10 Frames", 1i64), EnumeratorShape::new_int("Env Sdfgi Converge In 15 Frames", 2i64), EnumeratorShape::new_int("Env Sdfgi Converge In 20 Frames", 3i64), EnumeratorShape::new_int("Env Sdfgi Converge In 25 Frames", 4i64), EnumeratorShape::new_int("Env Sdfgi Converge In 30 Frames", 5i64), EnumeratorShape::new_int("Env Sdfgi Converge Max", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSDFGIFramesToConverge")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSdfgiFramesToConverge {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiFramesToConverge {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSdfgiFramesToConverge {
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
impl crate::registry::property::Export for EnvironmentSdfgiFramesToConverge {
    
}
impl crate::meta::Element for EnvironmentSdfgiFramesToConverge {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIFramesToUpdateLight`."]
pub struct EnvironmentSdfgiFramesToUpdateLight {
    ord: i32
}
impl EnvironmentSdfgiFramesToUpdateLight {
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME`"]
    pub const IN_1_FRAME: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES`"]
    pub const IN_2_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES`"]
    pub const IN_4_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES`"]
    pub const IN_8_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES`"]
    pub const IN_16_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_MAX`"]
    pub const MAX: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiFramesToUpdateLight {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiFramesToUpdateLight") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiFramesToUpdateLight {
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
            Self::IN_1_FRAME => "IN_1_FRAME", Self::IN_2_FRAMES => "IN_2_FRAMES", Self::IN_4_FRAMES => "IN_4_FRAMES", Self::IN_8_FRAMES => "IN_8_FRAMES", Self::IN_16_FRAMES => "IN_16_FRAMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiFramesToUpdateLight::IN_1_FRAME, EnvironmentSdfgiFramesToUpdateLight::IN_2_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_4_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_8_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_16_FRAMES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiFramesToUpdateLight >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN_1_FRAME", "ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME", EnvironmentSdfgiFramesToUpdateLight::IN_1_FRAME), crate::meta::inspect::EnumConstant::new("IN_2_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_2_FRAMES), crate::meta::inspect::EnumConstant::new("IN_4_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_4_FRAMES), crate::meta::inspect::EnumConstant::new("IN_8_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_8_FRAMES), crate::meta::inspect::EnumConstant::new("IN_16_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_16_FRAMES), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_UPDATE_LIGHT_MAX", EnvironmentSdfgiFramesToUpdateLight::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiFramesToUpdateLight {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiFramesToUpdateLight {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Env Sdfgi Update Light In 1 Frame", 0i64), EnumeratorShape::new_int("Env Sdfgi Update Light In 2 Frames", 1i64), EnumeratorShape::new_int("Env Sdfgi Update Light In 4 Frames", 2i64), EnumeratorShape::new_int("Env Sdfgi Update Light In 8 Frames", 3i64), EnumeratorShape::new_int("Env Sdfgi Update Light In 16 Frames", 4i64), EnumeratorShape::new_int("Env Sdfgi Update Light Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.EnvironmentSDFGIFramesToUpdateLight")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EnvironmentSdfgiFramesToUpdateLight {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiFramesToUpdateLight {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EnvironmentSdfgiFramesToUpdateLight {
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
impl crate::registry::property::Export for EnvironmentSdfgiFramesToUpdateLight {
    
}
impl crate::meta::Element for EnvironmentSdfgiFramesToUpdateLight {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubSurfaceScatteringQuality {
    ord: i32
}
impl SubSurfaceScatteringQuality {
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_DISABLED")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_DISABLED`"]
    pub const DISABLED: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 0i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_LOW`"]
    pub const LOW: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 1i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_MEDIUM`"]
    pub const MEDIUM: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 2i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_HIGH`"]
    pub const HIGH: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SubSurfaceScatteringQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubSurfaceScatteringQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubSurfaceScatteringQuality {
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
            Self::DISABLED => "DISABLED", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SubSurfaceScatteringQuality::DISABLED, SubSurfaceScatteringQuality::LOW, SubSurfaceScatteringQuality::MEDIUM, SubSurfaceScatteringQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SubSurfaceScatteringQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SUB_SURFACE_SCATTERING_QUALITY_DISABLED", SubSurfaceScatteringQuality::DISABLED), crate::meta::inspect::EnumConstant::new("LOW", "SUB_SURFACE_SCATTERING_QUALITY_LOW", SubSurfaceScatteringQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "SUB_SURFACE_SCATTERING_QUALITY_MEDIUM", SubSurfaceScatteringQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "SUB_SURFACE_SCATTERING_QUALITY_HIGH", SubSurfaceScatteringQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for SubSurfaceScatteringQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Sub Surface Scattering Quality Disabled", 0i64), EnumeratorShape::new_int("Sub Surface Scattering Quality Low", 1i64), EnumeratorShape::new_int("Sub Surface Scattering Quality Medium", 2i64), EnumeratorShape::new_int("Sub Surface Scattering Quality High", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.SubSurfaceScatteringQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SubSurfaceScatteringQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubSurfaceScatteringQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SubSurfaceScatteringQuality {
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
impl crate::registry::property::Export for SubSurfaceScatteringQuality {
    
}
impl crate::meta::Element for SubSurfaceScatteringQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DOFBokehShape`."]
pub struct DofBokehShape {
    ord: i32
}
impl DofBokehShape {
    #[doc(alias = "DOF_BOKEH_BOX")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_BOX`"]
    pub const BOX: DofBokehShape = DofBokehShape {
        ord: 0i32
    };
    #[doc(alias = "DOF_BOKEH_HEXAGON")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_HEXAGON`"]
    pub const HEXAGON: DofBokehShape = DofBokehShape {
        ord: 1i32
    };
    #[doc(alias = "DOF_BOKEH_CIRCLE")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_CIRCLE`"]
    pub const CIRCLE: DofBokehShape = DofBokehShape {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DofBokehShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DofBokehShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DofBokehShape {
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
            Self::BOX => "BOX", Self::HEXAGON => "HEXAGON", Self::CIRCLE => "CIRCLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DofBokehShape::BOX, DofBokehShape::HEXAGON, DofBokehShape::CIRCLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DofBokehShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOX", "DOF_BOKEH_BOX", DofBokehShape::BOX), crate::meta::inspect::EnumConstant::new("HEXAGON", "DOF_BOKEH_HEXAGON", DofBokehShape::HEXAGON), crate::meta::inspect::EnumConstant::new("CIRCLE", "DOF_BOKEH_CIRCLE", DofBokehShape::CIRCLE)]
        }
    }
}
impl crate::meta::GodotConvert for DofBokehShape {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Dof Bokeh Box", 0i64), EnumeratorShape::new_int("Dof Bokeh Hexagon", 1i64), EnumeratorShape::new_int("Dof Bokeh Circle", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.DOFBokehShape")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DofBokehShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DofBokehShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DofBokehShape {
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
impl crate::registry::property::Export for DofBokehShape {
    
}
impl crate::meta::Element for DofBokehShape {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DOFBlurQuality`."]
pub struct DofBlurQuality {
    ord: i32
}
impl DofBlurQuality {
    #[doc(alias = "DOF_BLUR_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: DofBlurQuality = DofBlurQuality {
        ord: 0i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_LOW`"]
    pub const LOW: DofBlurQuality = DofBlurQuality {
        ord: 1i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_MEDIUM`"]
    pub const MEDIUM: DofBlurQuality = DofBlurQuality {
        ord: 2i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_HIGH`"]
    pub const HIGH: DofBlurQuality = DofBlurQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DofBlurQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DofBlurQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DofBlurQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DofBlurQuality::VERY_LOW, DofBlurQuality::LOW, DofBlurQuality::MEDIUM, DofBlurQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DofBlurQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "DOF_BLUR_QUALITY_VERY_LOW", DofBlurQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "DOF_BLUR_QUALITY_LOW", DofBlurQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "DOF_BLUR_QUALITY_MEDIUM", DofBlurQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "DOF_BLUR_QUALITY_HIGH", DofBlurQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for DofBlurQuality {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Dof Blur Quality Very Low", 0i64), EnumeratorShape::new_int("Dof Blur Quality Low", 1i64), EnumeratorShape::new_int("Dof Blur Quality Medium", 2i64), EnumeratorShape::new_int("Dof Blur Quality High", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.DOFBlurQuality")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for DofBlurQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DofBlurQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for DofBlurQuality {
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
impl crate::registry::property::Export for DofBlurQuality {
    
}
impl crate::meta::Element for DofBlurQuality {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceType {
    ord: i32
}
impl InstanceType {
    #[doc(alias = "INSTANCE_NONE")]
    #[doc = "Godot enumerator name: `INSTANCE_NONE`"]
    pub const NONE: InstanceType = InstanceType {
        ord: 0i32
    };
    #[doc(alias = "INSTANCE_MESH")]
    #[doc = "Godot enumerator name: `INSTANCE_MESH`"]
    pub const MESH: InstanceType = InstanceType {
        ord: 1i32
    };
    #[doc(alias = "INSTANCE_MULTIMESH")]
    #[doc = "Godot enumerator name: `INSTANCE_MULTIMESH`"]
    pub const MULTIMESH: InstanceType = InstanceType {
        ord: 2i32
    };
    #[doc(alias = "INSTANCE_PARTICLES")]
    #[doc = "Godot enumerator name: `INSTANCE_PARTICLES`"]
    pub const PARTICLES: InstanceType = InstanceType {
        ord: 3i32
    };
    #[doc(alias = "INSTANCE_PARTICLES_COLLISION")]
    #[doc = "Godot enumerator name: `INSTANCE_PARTICLES_COLLISION`"]
    pub const PARTICLES_COLLISION: InstanceType = InstanceType {
        ord: 4i32
    };
    #[doc(alias = "INSTANCE_LIGHT")]
    #[doc = "Godot enumerator name: `INSTANCE_LIGHT`"]
    pub const LIGHT: InstanceType = InstanceType {
        ord: 5i32
    };
    #[doc(alias = "INSTANCE_REFLECTION_PROBE")]
    #[doc = "Godot enumerator name: `INSTANCE_REFLECTION_PROBE`"]
    pub const REFLECTION_PROBE: InstanceType = InstanceType {
        ord: 6i32
    };
    #[doc(alias = "INSTANCE_DECAL")]
    #[doc = "Godot enumerator name: `INSTANCE_DECAL`"]
    pub const DECAL: InstanceType = InstanceType {
        ord: 7i32
    };
    #[doc(alias = "INSTANCE_VOXEL_GI")]
    #[doc = "Godot enumerator name: `INSTANCE_VOXEL_GI`"]
    pub const VOXEL_GI: InstanceType = InstanceType {
        ord: 8i32
    };
    #[doc(alias = "INSTANCE_LIGHTMAP")]
    #[doc = "Godot enumerator name: `INSTANCE_LIGHTMAP`"]
    pub const LIGHTMAP: InstanceType = InstanceType {
        ord: 9i32
    };
    #[doc(alias = "INSTANCE_OCCLUDER")]
    #[doc = "Godot enumerator name: `INSTANCE_OCCLUDER`"]
    pub const OCCLUDER: InstanceType = InstanceType {
        ord: 10i32
    };
    #[doc(alias = "INSTANCE_VISIBLITY_NOTIFIER")]
    #[doc = "Godot enumerator name: `INSTANCE_VISIBLITY_NOTIFIER`"]
    pub const VISIBLITY_NOTIFIER: InstanceType = InstanceType {
        ord: 11i32
    };
    #[doc(alias = "INSTANCE_FOG_VOLUME")]
    #[doc = "Godot enumerator name: `INSTANCE_FOG_VOLUME`"]
    pub const FOG_VOLUME: InstanceType = InstanceType {
        ord: 12i32
    };
    #[doc(alias = "INSTANCE_MAX")]
    #[doc = "Godot enumerator name: `INSTANCE_MAX`"]
    pub const MAX: InstanceType = InstanceType {
        ord: 13i32
    };
    #[doc(alias = "INSTANCE_GEOMETRY_MASK")]
    #[doc = "Godot enumerator name: `INSTANCE_GEOMETRY_MASK`"]
    pub const GEOMETRY_MASK: InstanceType = InstanceType {
        ord: 14i32
    };
    
}
impl std::fmt::Debug for InstanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InstanceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InstanceType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 => Some(Self {
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
            Self::NONE => "NONE", Self::MESH => "MESH", Self::MULTIMESH => "MULTIMESH", Self::PARTICLES => "PARTICLES", Self::PARTICLES_COLLISION => "PARTICLES_COLLISION", Self::LIGHT => "LIGHT", Self::REFLECTION_PROBE => "REFLECTION_PROBE", Self::DECAL => "DECAL", Self::VOXEL_GI => "VOXEL_GI", Self::LIGHTMAP => "LIGHTMAP", Self::OCCLUDER => "OCCLUDER", Self::VISIBLITY_NOTIFIER => "VISIBLITY_NOTIFIER", Self::FOG_VOLUME => "FOG_VOLUME", Self::MAX => "MAX", Self::GEOMETRY_MASK => "GEOMETRY_MASK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InstanceType::NONE, InstanceType::MESH, InstanceType::MULTIMESH, InstanceType::PARTICLES, InstanceType::PARTICLES_COLLISION, InstanceType::LIGHT, InstanceType::REFLECTION_PROBE, InstanceType::DECAL, InstanceType::VOXEL_GI, InstanceType::LIGHTMAP, InstanceType::OCCLUDER, InstanceType::VISIBLITY_NOTIFIER, InstanceType::FOG_VOLUME, InstanceType::MAX, InstanceType::GEOMETRY_MASK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InstanceType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "INSTANCE_NONE", InstanceType::NONE), crate::meta::inspect::EnumConstant::new("MESH", "INSTANCE_MESH", InstanceType::MESH), crate::meta::inspect::EnumConstant::new("MULTIMESH", "INSTANCE_MULTIMESH", InstanceType::MULTIMESH), crate::meta::inspect::EnumConstant::new("PARTICLES", "INSTANCE_PARTICLES", InstanceType::PARTICLES), crate::meta::inspect::EnumConstant::new("PARTICLES_COLLISION", "INSTANCE_PARTICLES_COLLISION", InstanceType::PARTICLES_COLLISION), crate::meta::inspect::EnumConstant::new("LIGHT", "INSTANCE_LIGHT", InstanceType::LIGHT), crate::meta::inspect::EnumConstant::new("REFLECTION_PROBE", "INSTANCE_REFLECTION_PROBE", InstanceType::REFLECTION_PROBE), crate::meta::inspect::EnumConstant::new("DECAL", "INSTANCE_DECAL", InstanceType::DECAL), crate::meta::inspect::EnumConstant::new("VOXEL_GI", "INSTANCE_VOXEL_GI", InstanceType::VOXEL_GI), crate::meta::inspect::EnumConstant::new("LIGHTMAP", "INSTANCE_LIGHTMAP", InstanceType::LIGHTMAP), crate::meta::inspect::EnumConstant::new("OCCLUDER", "INSTANCE_OCCLUDER", InstanceType::OCCLUDER), crate::meta::inspect::EnumConstant::new("VISIBLITY_NOTIFIER", "INSTANCE_VISIBLITY_NOTIFIER", InstanceType::VISIBLITY_NOTIFIER), crate::meta::inspect::EnumConstant::new("FOG_VOLUME", "INSTANCE_FOG_VOLUME", InstanceType::FOG_VOLUME), crate::meta::inspect::EnumConstant::new("MAX", "INSTANCE_MAX", InstanceType::MAX), crate::meta::inspect::EnumConstant::new("GEOMETRY_MASK", "INSTANCE_GEOMETRY_MASK", InstanceType::GEOMETRY_MASK)]
        }
    }
}
impl crate::meta::GodotConvert for InstanceType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Instance None", 0i64), EnumeratorShape::new_int("Instance Mesh", 1i64), EnumeratorShape::new_int("Instance Multimesh", 2i64), EnumeratorShape::new_int("Instance Particles", 3i64), EnumeratorShape::new_int("Instance Particles Collision", 4i64), EnumeratorShape::new_int("Instance Light", 5i64), EnumeratorShape::new_int("Instance Reflection Probe", 6i64), EnumeratorShape::new_int("Instance Decal", 7i64), EnumeratorShape::new_int("Instance Voxel Gi", 8i64), EnumeratorShape::new_int("Instance Lightmap", 9i64), EnumeratorShape::new_int("Instance Occluder", 10i64), EnumeratorShape::new_int("Instance Visiblity Notifier", 11i64), EnumeratorShape::new_int("Instance Fog Volume", 12i64), EnumeratorShape::new_int("Instance Max", 13i64), EnumeratorShape::new_int("Instance Geometry Mask", 14i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.InstanceType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for InstanceType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InstanceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for InstanceType {
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
impl crate::registry::property::Export for InstanceType {
    
}
impl crate::meta::Element for InstanceType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceFlags {
    ord: i32
}
impl InstanceFlags {
    #[doc(alias = "INSTANCE_FLAG_USE_BAKED_LIGHT")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_USE_BAKED_LIGHT`"]
    pub const USE_BAKED_LIGHT: InstanceFlags = InstanceFlags {
        ord: 0i32
    };
    #[doc(alias = "INSTANCE_FLAG_USE_DYNAMIC_GI")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_USE_DYNAMIC_GI`"]
    pub const USE_DYNAMIC_GI: InstanceFlags = InstanceFlags {
        ord: 1i32
    };
    #[doc(alias = "INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE`"]
    pub const DRAW_NEXT_FRAME_IF_VISIBLE: InstanceFlags = InstanceFlags {
        ord: 2i32
    };
    #[doc(alias = "INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING`"]
    pub const IGNORE_OCCLUSION_CULLING: InstanceFlags = InstanceFlags {
        ord: 3i32
    };
    #[doc(alias = "INSTANCE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_MAX`"]
    pub const MAX: InstanceFlags = InstanceFlags {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for InstanceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InstanceFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InstanceFlags {
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
            Self::USE_BAKED_LIGHT => "USE_BAKED_LIGHT", Self::USE_DYNAMIC_GI => "USE_DYNAMIC_GI", Self::DRAW_NEXT_FRAME_IF_VISIBLE => "DRAW_NEXT_FRAME_IF_VISIBLE", Self::IGNORE_OCCLUSION_CULLING => "IGNORE_OCCLUSION_CULLING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InstanceFlags::USE_BAKED_LIGHT, InstanceFlags::USE_DYNAMIC_GI, InstanceFlags::DRAW_NEXT_FRAME_IF_VISIBLE, InstanceFlags::IGNORE_OCCLUSION_CULLING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InstanceFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("USE_BAKED_LIGHT", "INSTANCE_FLAG_USE_BAKED_LIGHT", InstanceFlags::USE_BAKED_LIGHT), crate::meta::inspect::EnumConstant::new("USE_DYNAMIC_GI", "INSTANCE_FLAG_USE_DYNAMIC_GI", InstanceFlags::USE_DYNAMIC_GI), crate::meta::inspect::EnumConstant::new("DRAW_NEXT_FRAME_IF_VISIBLE", "INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE", InstanceFlags::DRAW_NEXT_FRAME_IF_VISIBLE), crate::meta::inspect::EnumConstant::new("IGNORE_OCCLUSION_CULLING", "INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING", InstanceFlags::IGNORE_OCCLUSION_CULLING), crate::meta::inspect::EnumConstant::new("MAX", "INSTANCE_FLAG_MAX", InstanceFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for InstanceFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for InstanceFlags {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Instance Flag Use Baked Light", 0i64), EnumeratorShape::new_int("Instance Flag Use Dynamic Gi", 1i64), EnumeratorShape::new_int("Instance Flag Draw Next Frame If Visible", 2i64), EnumeratorShape::new_int("Instance Flag Ignore Occlusion Culling", 3i64), EnumeratorShape::new_int("Instance Flag Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.InstanceFlags")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for InstanceFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InstanceFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for InstanceFlags {
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
impl crate::registry::property::Export for InstanceFlags {
    
}
impl crate::meta::Element for InstanceFlags {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    #[doc(alias = "SHADOW_CASTING_SETTING_OFF")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_OFF`"]
    pub const OFF: ShadowCastingSetting = ShadowCastingSetting {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_ON")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_ON`"]
    pub const ON: ShadowCastingSetting = ShadowCastingSetting {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: ShadowCastingSetting = ShadowCastingSetting {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_SHADOWS_ONLY")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_SHADOWS_ONLY`"]
    pub const SHADOWS_ONLY: ShadowCastingSetting = ShadowCastingSetting {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadowCastingSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowCastingSetting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
            Self::OFF => "OFF", Self::ON => "ON", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOWS_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowCastingSetting::OFF, ShadowCastingSetting::ON, ShadowCastingSetting::DOUBLE_SIDED, ShadowCastingSetting::SHADOWS_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowCastingSetting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "SHADOW_CASTING_SETTING_OFF", ShadowCastingSetting::OFF), crate::meta::inspect::EnumConstant::new("ON", "SHADOW_CASTING_SETTING_ON", ShadowCastingSetting::ON), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "SHADOW_CASTING_SETTING_DOUBLE_SIDED", ShadowCastingSetting::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("SHADOWS_ONLY", "SHADOW_CASTING_SETTING_SHADOWS_ONLY", ShadowCastingSetting::SHADOWS_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Shadow Casting Setting Off", 0i64), EnumeratorShape::new_int("Shadow Casting Setting On", 1i64), EnumeratorShape::new_int("Shadow Casting Setting Double Sided", 2i64), EnumeratorShape::new_int("Shadow Casting Setting Shadows Only", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.ShadowCastingSetting")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ShadowCastingSetting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ShadowCastingSetting {
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
impl crate::registry::property::Export for ShadowCastingSetting {
    
}
impl crate::meta::Element for ShadowCastingSetting {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    #[doc(alias = "VISIBILITY_RANGE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DISABLED`"]
    pub const DISABLED: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_SELF")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_SELF`"]
    pub const SELF: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_DEPENDENCIES")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DEPENDENCIES`"]
    pub const DEPENDENCIES: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityRangeFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityRangeFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
            Self::DISABLED => "DISABLED", Self::SELF => "SELF", Self::DEPENDENCIES => "DEPENDENCIES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityRangeFadeMode::DISABLED, VisibilityRangeFadeMode::SELF, VisibilityRangeFadeMode::DEPENDENCIES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityRangeFadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VISIBILITY_RANGE_FADE_DISABLED", VisibilityRangeFadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("SELF", "VISIBILITY_RANGE_FADE_SELF", VisibilityRangeFadeMode::SELF), crate::meta::inspect::EnumConstant::new("DEPENDENCIES", "VISIBILITY_RANGE_FADE_DEPENDENCIES", VisibilityRangeFadeMode::DEPENDENCIES)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Visibility Range Fade Disabled", 0i64), EnumeratorShape::new_int("Visibility Range Fade Self", 1i64), EnumeratorShape::new_int("Visibility Range Fade Dependencies", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.VisibilityRangeFadeMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for VisibilityRangeFadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for VisibilityRangeFadeMode {
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
impl crate::registry::property::Export for VisibilityRangeFadeMode {
    
}
impl crate::meta::Element for VisibilityRangeFadeMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeChannels {
    ord: i32
}
impl BakeChannels {
    #[doc(alias = "BAKE_CHANNEL_ALBEDO_ALPHA")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_ALBEDO_ALPHA`"]
    pub const ALBEDO_ALPHA: BakeChannels = BakeChannels {
        ord: 0i32
    };
    #[doc(alias = "BAKE_CHANNEL_NORMAL")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_NORMAL`"]
    pub const NORMAL: BakeChannels = BakeChannels {
        ord: 1i32
    };
    #[doc(alias = "BAKE_CHANNEL_ORM")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_ORM`"]
    pub const ORM: BakeChannels = BakeChannels {
        ord: 2i32
    };
    #[doc(alias = "BAKE_CHANNEL_EMISSION")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_EMISSION`"]
    pub const EMISSION: BakeChannels = BakeChannels {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BakeChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeChannels") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeChannels {
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
            Self::ALBEDO_ALPHA => "ALBEDO_ALPHA", Self::NORMAL => "NORMAL", Self::ORM => "ORM", Self::EMISSION => "EMISSION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BakeChannels::ALBEDO_ALPHA, BakeChannels::NORMAL, BakeChannels::ORM, BakeChannels::EMISSION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BakeChannels >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO_ALPHA", "BAKE_CHANNEL_ALBEDO_ALPHA", BakeChannels::ALBEDO_ALPHA), crate::meta::inspect::EnumConstant::new("NORMAL", "BAKE_CHANNEL_NORMAL", BakeChannels::NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "BAKE_CHANNEL_ORM", BakeChannels::ORM), crate::meta::inspect::EnumConstant::new("EMISSION", "BAKE_CHANNEL_EMISSION", BakeChannels::EMISSION)]
        }
    }
}
impl crate::meta::GodotConvert for BakeChannels {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Bake Channel Albedo Alpha", 0i64), EnumeratorShape::new_int("Bake Channel Normal", 1i64), EnumeratorShape::new_int("Bake Channel Orm", 2i64), EnumeratorShape::new_int("Bake Channel Emission", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.BakeChannels")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for BakeChannels {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for BakeChannels {
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
impl crate::registry::property::Export for BakeChannels {
    
}
impl crate::meta::Element for BakeChannels {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasTextureChannel {
    ord: i32
}
impl CanvasTextureChannel {
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_DIFFUSE")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_DIFFUSE`"]
    pub const DIFFUSE: CanvasTextureChannel = CanvasTextureChannel {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_NORMAL")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_NORMAL`"]
    pub const NORMAL: CanvasTextureChannel = CanvasTextureChannel {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_SPECULAR")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_SPECULAR`"]
    pub const SPECULAR: CanvasTextureChannel = CanvasTextureChannel {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasTextureChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasTextureChannel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasTextureChannel {
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
            Self::DIFFUSE => "DIFFUSE", Self::NORMAL => "NORMAL", Self::SPECULAR => "SPECULAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasTextureChannel::DIFFUSE, CanvasTextureChannel::NORMAL, CanvasTextureChannel::SPECULAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasTextureChannel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DIFFUSE", "CANVAS_TEXTURE_CHANNEL_DIFFUSE", CanvasTextureChannel::DIFFUSE), crate::meta::inspect::EnumConstant::new("NORMAL", "CANVAS_TEXTURE_CHANNEL_NORMAL", CanvasTextureChannel::NORMAL), crate::meta::inspect::EnumConstant::new("SPECULAR", "CANVAS_TEXTURE_CHANNEL_SPECULAR", CanvasTextureChannel::SPECULAR)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasTextureChannel {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Texture Channel Diffuse", 0i64), EnumeratorShape::new_int("Canvas Texture Channel Normal", 1i64), EnumeratorShape::new_int("Canvas Texture Channel Specular", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasTextureChannel")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasTextureChannel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasTextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasTextureChannel {
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
impl crate::registry::property::Export for CanvasTextureChannel {
    
}
impl crate::meta::Element for CanvasTextureChannel {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NinePatchAxisMode {
    ord: i32
}
impl NinePatchAxisMode {
    #[doc(alias = "NINE_PATCH_STRETCH")]
    #[doc = "Godot enumerator name: `NINE_PATCH_STRETCH`"]
    pub const STRETCH: NinePatchAxisMode = NinePatchAxisMode {
        ord: 0i32
    };
    #[doc(alias = "NINE_PATCH_TILE")]
    #[doc = "Godot enumerator name: `NINE_PATCH_TILE`"]
    pub const TILE: NinePatchAxisMode = NinePatchAxisMode {
        ord: 1i32
    };
    #[doc(alias = "NINE_PATCH_TILE_FIT")]
    #[doc = "Godot enumerator name: `NINE_PATCH_TILE_FIT`"]
    pub const TILE_FIT: NinePatchAxisMode = NinePatchAxisMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for NinePatchAxisMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("NinePatchAxisMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for NinePatchAxisMode {
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
            Self::STRETCH => "STRETCH", Self::TILE => "TILE", Self::TILE_FIT => "TILE_FIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[NinePatchAxisMode::STRETCH, NinePatchAxisMode::TILE, NinePatchAxisMode::TILE_FIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < NinePatchAxisMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRETCH", "NINE_PATCH_STRETCH", NinePatchAxisMode::STRETCH), crate::meta::inspect::EnumConstant::new("TILE", "NINE_PATCH_TILE", NinePatchAxisMode::TILE), crate::meta::inspect::EnumConstant::new("TILE_FIT", "NINE_PATCH_TILE_FIT", NinePatchAxisMode::TILE_FIT)]
        }
    }
}
impl crate::meta::GodotConvert for NinePatchAxisMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Nine Patch Stretch", 0i64), EnumeratorShape::new_int("Nine Patch Tile", 1i64), EnumeratorShape::new_int("Nine Patch Tile Fit", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.NinePatchAxisMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for NinePatchAxisMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for NinePatchAxisMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for NinePatchAxisMode {
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
impl crate::registry::property::Export for NinePatchAxisMode {
    
}
impl crate::meta::Element for NinePatchAxisMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasItemTextureFilter {
    ord: i32
}
impl CanvasItemTextureFilter {
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_DEFAULT")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_DEFAULT`"]
    pub const DEFAULT: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 3i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 4i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 5i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 6i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_MAX`"]
    pub const MAX: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for CanvasItemTextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasItemTextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasItemTextureFilter {
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
            Self::DEFAULT => "DEFAULT", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasItemTextureFilter::DEFAULT, CanvasItemTextureFilter::NEAREST, CanvasItemTextureFilter::LINEAR, CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS, CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS, CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC, CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasItemTextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "CANVAS_ITEM_TEXTURE_FILTER_DEFAULT", CanvasItemTextureFilter::DEFAULT), crate::meta::inspect::EnumConstant::new("NEAREST", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST", CanvasItemTextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR", CanvasItemTextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS_ANISOTROPIC", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS_ANISOTROPIC", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_ITEM_TEXTURE_FILTER_MAX", CanvasItemTextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for CanvasItemTextureFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Item Texture Filter Default", 0i64), EnumeratorShape::new_int("Canvas Item Texture Filter Nearest", 1i64), EnumeratorShape::new_int("Canvas Item Texture Filter Linear", 2i64), EnumeratorShape::new_int("Canvas Item Texture Filter Nearest With Mipmaps", 3i64), EnumeratorShape::new_int("Canvas Item Texture Filter Linear With Mipmaps", 4i64), EnumeratorShape::new_int("Canvas Item Texture Filter Nearest With Mipmaps Anisotropic", 5i64), EnumeratorShape::new_int("Canvas Item Texture Filter Linear With Mipmaps Anisotropic", 6i64), EnumeratorShape::new_int("Canvas Item Texture Filter Max", 7i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasItemTextureFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasItemTextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasItemTextureFilter {
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
impl crate::registry::property::Export for CanvasItemTextureFilter {
    
}
impl crate::meta::Element for CanvasItemTextureFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasItemTextureRepeat {
    ord: i32
}
impl CanvasItemTextureRepeat {
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT`"]
    pub const DEFAULT: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 3i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_MAX`"]
    pub const MAX: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CanvasItemTextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasItemTextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasItemTextureRepeat {
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
            Self::DEFAULT => "DEFAULT", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasItemTextureRepeat::DEFAULT, CanvasItemTextureRepeat::DISABLED, CanvasItemTextureRepeat::ENABLED, CanvasItemTextureRepeat::MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasItemTextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT", CanvasItemTextureRepeat::DEFAULT), crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_ITEM_TEXTURE_REPEAT_DISABLED", CanvasItemTextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "CANVAS_ITEM_TEXTURE_REPEAT_ENABLED", CanvasItemTextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("MIRROR", "CANVAS_ITEM_TEXTURE_REPEAT_MIRROR", CanvasItemTextureRepeat::MIRROR), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_ITEM_TEXTURE_REPEAT_MAX", CanvasItemTextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for CanvasItemTextureRepeat {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Item Texture Repeat Default", 0i64), EnumeratorShape::new_int("Canvas Item Texture Repeat Disabled", 1i64), EnumeratorShape::new_int("Canvas Item Texture Repeat Enabled", 2i64), EnumeratorShape::new_int("Canvas Item Texture Repeat Mirror", 3i64), EnumeratorShape::new_int("Canvas Item Texture Repeat Max", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasItemTextureRepeat")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasItemTextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasItemTextureRepeat {
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
impl crate::registry::property::Export for CanvasItemTextureRepeat {
    
}
impl crate::meta::Element for CanvasItemTextureRepeat {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasGroupMode {
    ord: i32
}
impl CanvasGroupMode {
    #[doc(alias = "CANVAS_GROUP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_DISABLED`"]
    pub const DISABLED: CanvasGroupMode = CanvasGroupMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_CLIP_ONLY")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_CLIP_ONLY`"]
    pub const CLIP_ONLY: CanvasGroupMode = CanvasGroupMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_CLIP_AND_DRAW")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_CLIP_AND_DRAW`"]
    pub const CLIP_AND_DRAW: CanvasGroupMode = CanvasGroupMode {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_TRANSPARENT")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_TRANSPARENT`"]
    pub const TRANSPARENT: CanvasGroupMode = CanvasGroupMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CanvasGroupMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasGroupMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasGroupMode {
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
            Self::DISABLED => "DISABLED", Self::CLIP_ONLY => "CLIP_ONLY", Self::CLIP_AND_DRAW => "CLIP_AND_DRAW", Self::TRANSPARENT => "TRANSPARENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasGroupMode::DISABLED, CanvasGroupMode::CLIP_ONLY, CanvasGroupMode::CLIP_AND_DRAW, CanvasGroupMode::TRANSPARENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasGroupMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_GROUP_MODE_DISABLED", CanvasGroupMode::DISABLED), crate::meta::inspect::EnumConstant::new("CLIP_ONLY", "CANVAS_GROUP_MODE_CLIP_ONLY", CanvasGroupMode::CLIP_ONLY), crate::meta::inspect::EnumConstant::new("CLIP_AND_DRAW", "CANVAS_GROUP_MODE_CLIP_AND_DRAW", CanvasGroupMode::CLIP_AND_DRAW), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "CANVAS_GROUP_MODE_TRANSPARENT", CanvasGroupMode::TRANSPARENT)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasGroupMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Group Mode Disabled", 0i64), EnumeratorShape::new_int("Canvas Group Mode Clip Only", 1i64), EnumeratorShape::new_int("Canvas Group Mode Clip And Draw", 2i64), EnumeratorShape::new_int("Canvas Group Mode Transparent", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasGroupMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasGroupMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasGroupMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasGroupMode {
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
impl crate::registry::property::Export for CanvasGroupMode {
    
}
impl crate::meta::Element for CanvasGroupMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightMode {
    ord: i32
}
impl CanvasLightMode {
    #[doc(alias = "CANVAS_LIGHT_MODE_POINT")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_MODE_POINT`"]
    pub const POINT: CanvasLightMode = CanvasLightMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_MODE_DIRECTIONAL")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_MODE_DIRECTIONAL`"]
    pub const DIRECTIONAL: CanvasLightMode = CanvasLightMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CanvasLightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightMode {
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
            Self::POINT => "POINT", Self::DIRECTIONAL => "DIRECTIONAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightMode::POINT, CanvasLightMode::DIRECTIONAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINT", "CANVAS_LIGHT_MODE_POINT", CanvasLightMode::POINT), crate::meta::inspect::EnumConstant::new("DIRECTIONAL", "CANVAS_LIGHT_MODE_DIRECTIONAL", CanvasLightMode::DIRECTIONAL)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasLightMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Light Mode Point", 0i64), EnumeratorShape::new_int("Canvas Light Mode Directional", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasLightMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasLightMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasLightMode {
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
impl crate::registry::property::Export for CanvasLightMode {
    
}
impl crate::meta::Element for CanvasLightMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightBlendMode {
    ord: i32
}
impl CanvasLightBlendMode {
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_ADD`"]
    pub const ADD: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_SUB`"]
    pub const SUB: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_MIX`"]
    pub const MIX: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasLightBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightBlendMode {
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
            Self::ADD => "ADD", Self::SUB => "SUB", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightBlendMode::ADD, CanvasLightBlendMode::SUB, CanvasLightBlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD", "CANVAS_LIGHT_BLEND_MODE_ADD", CanvasLightBlendMode::ADD), crate::meta::inspect::EnumConstant::new("SUB", "CANVAS_LIGHT_BLEND_MODE_SUB", CanvasLightBlendMode::SUB), crate::meta::inspect::EnumConstant::new("MIX", "CANVAS_LIGHT_BLEND_MODE_MIX", CanvasLightBlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasLightBlendMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Light Blend Mode Add", 0i64), EnumeratorShape::new_int("Canvas Light Blend Mode Sub", 1i64), EnumeratorShape::new_int("Canvas Light Blend Mode Mix", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasLightBlendMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasLightBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasLightBlendMode {
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
impl crate::registry::property::Export for CanvasLightBlendMode {
    
}
impl crate::meta::Element for CanvasLightBlendMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightShadowFilter {
    ord: i32
}
impl CanvasLightShadowFilter {
    #[doc(alias = "CANVAS_LIGHT_FILTER_NONE")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_NONE`"]
    pub const NONE: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_PCF5")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_PCF5`"]
    pub const PCF5: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_PCF13")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_PCF13`"]
    pub const PCF13: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_MAX`"]
    pub const MAX: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CanvasLightShadowFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightShadowFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightShadowFilter {
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
            Self::NONE => "NONE", Self::PCF5 => "PCF5", Self::PCF13 => "PCF13", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightShadowFilter::NONE, CanvasLightShadowFilter::PCF5, CanvasLightShadowFilter::PCF13]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightShadowFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "CANVAS_LIGHT_FILTER_NONE", CanvasLightShadowFilter::NONE), crate::meta::inspect::EnumConstant::new("PCF5", "CANVAS_LIGHT_FILTER_PCF5", CanvasLightShadowFilter::PCF5), crate::meta::inspect::EnumConstant::new("PCF13", "CANVAS_LIGHT_FILTER_PCF13", CanvasLightShadowFilter::PCF13), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_LIGHT_FILTER_MAX", CanvasLightShadowFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasLightShadowFilter {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CanvasLightShadowFilter {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Light Filter None", 0i64), EnumeratorShape::new_int("Canvas Light Filter Pcf5", 1i64), EnumeratorShape::new_int("Canvas Light Filter Pcf13", 2i64), EnumeratorShape::new_int("Canvas Light Filter Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasLightShadowFilter")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasLightShadowFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightShadowFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasLightShadowFilter {
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
impl crate::registry::property::Export for CanvasLightShadowFilter {
    
}
impl crate::meta::Element for CanvasLightShadowFilter {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasOccluderPolygonCullMode {
    ord: i32
}
impl CanvasOccluderPolygonCullMode {
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_DISABLED`"]
    pub const DISABLED: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE`"]
    pub const CLOCKWISE: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE`"]
    pub const COUNTER_CLOCKWISE: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasOccluderPolygonCullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasOccluderPolygonCullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasOccluderPolygonCullMode {
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
            Self::DISABLED => "DISABLED", Self::CLOCKWISE => "CLOCKWISE", Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasOccluderPolygonCullMode::DISABLED, CanvasOccluderPolygonCullMode::CLOCKWISE, CanvasOccluderPolygonCullMode::COUNTER_CLOCKWISE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasOccluderPolygonCullMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_OCCLUDER_POLYGON_CULL_DISABLED", CanvasOccluderPolygonCullMode::DISABLED), crate::meta::inspect::EnumConstant::new("CLOCKWISE", "CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE", CanvasOccluderPolygonCullMode::CLOCKWISE), crate::meta::inspect::EnumConstant::new("COUNTER_CLOCKWISE", "CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE", CanvasOccluderPolygonCullMode::COUNTER_CLOCKWISE)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasOccluderPolygonCullMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Canvas Occluder Polygon Cull Disabled", 0i64), EnumeratorShape::new_int("Canvas Occluder Polygon Cull Clockwise", 1i64), EnumeratorShape::new_int("Canvas Occluder Polygon Cull Counter Clockwise", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.CanvasOccluderPolygonCullMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for CanvasOccluderPolygonCullMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasOccluderPolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for CanvasOccluderPolygonCullMode {
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
impl crate::registry::property::Export for CanvasOccluderPolygonCullMode {
    
}
impl crate::meta::Element for CanvasOccluderPolygonCullMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlobalShaderParameterType {
    ord: i32
}
impl GlobalShaderParameterType {
    #[doc(alias = "GLOBAL_VAR_TYPE_BOOL")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BOOL`"]
    pub const BOOL: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 0i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC2`"]
    pub const BVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 1i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC3`"]
    pub const BVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 2i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC4`"]
    pub const BVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 3i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_INT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_INT`"]
    pub const INT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 4i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC2`"]
    pub const IVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 5i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC3`"]
    pub const IVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 6i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC4`"]
    pub const IVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 7i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_RECT2I")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_RECT2I`"]
    pub const RECT2I: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 8i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UINT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UINT`"]
    pub const UINT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 9i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC2`"]
    pub const UVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 10i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC3`"]
    pub const UVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 11i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC4`"]
    pub const UVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 12i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_FLOAT`"]
    pub const FLOAT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 13i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC2`"]
    pub const VEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 14i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC3`"]
    pub const VEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 15i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC4`"]
    pub const VEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 16i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_COLOR")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_COLOR`"]
    pub const COLOR: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 17i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_RECT2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_RECT2`"]
    pub const RECT2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 18i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT2`"]
    pub const MAT2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 19i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT3`"]
    pub const MAT3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 20i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT4`"]
    pub const MAT4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 21i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_TRANSFORM_2D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_TRANSFORM_2D`"]
    pub const TRANSFORM_2D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 22i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_TRANSFORM`"]
    pub const TRANSFORM: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 23i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER2D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER2D`"]
    pub const SAMPLER2D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 24i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER2DARRAY")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER2DARRAY`"]
    pub const SAMPLER2DARRAY: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 25i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER3D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER3D`"]
    pub const SAMPLER3D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 26i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLERCUBE")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLERCUBE`"]
    pub const SAMPLERCUBE: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 27i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLEREXT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLEREXT`"]
    pub const SAMPLEREXT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 28i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAX")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAX`"]
    pub const MAX: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 29i32
    };
    
}
impl std::fmt::Debug for GlobalShaderParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GlobalShaderParameterType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GlobalShaderParameterType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 => Some(Self {
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
            Self::BOOL => "BOOL", Self::BVEC2 => "BVEC2", Self::BVEC3 => "BVEC3", Self::BVEC4 => "BVEC4", Self::INT => "INT", Self::IVEC2 => "IVEC2", Self::IVEC3 => "IVEC3", Self::IVEC4 => "IVEC4", Self::RECT2I => "RECT2I", Self::UINT => "UINT", Self::UVEC2 => "UVEC2", Self::UVEC3 => "UVEC3", Self::UVEC4 => "UVEC4", Self::FLOAT => "FLOAT", Self::VEC2 => "VEC2", Self::VEC3 => "VEC3", Self::VEC4 => "VEC4", Self::COLOR => "COLOR", Self::RECT2 => "RECT2", Self::MAT2 => "MAT2", Self::MAT3 => "MAT3", Self::MAT4 => "MAT4", Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM => "TRANSFORM", Self::SAMPLER2D => "SAMPLER2D", Self::SAMPLER2DARRAY => "SAMPLER2DARRAY", Self::SAMPLER3D => "SAMPLER3D", Self::SAMPLERCUBE => "SAMPLERCUBE", Self::SAMPLEREXT => "SAMPLEREXT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GlobalShaderParameterType::BOOL, GlobalShaderParameterType::BVEC2, GlobalShaderParameterType::BVEC3, GlobalShaderParameterType::BVEC4, GlobalShaderParameterType::INT, GlobalShaderParameterType::IVEC2, GlobalShaderParameterType::IVEC3, GlobalShaderParameterType::IVEC4, GlobalShaderParameterType::RECT2I, GlobalShaderParameterType::UINT, GlobalShaderParameterType::UVEC2, GlobalShaderParameterType::UVEC3, GlobalShaderParameterType::UVEC4, GlobalShaderParameterType::FLOAT, GlobalShaderParameterType::VEC2, GlobalShaderParameterType::VEC3, GlobalShaderParameterType::VEC4, GlobalShaderParameterType::COLOR, GlobalShaderParameterType::RECT2, GlobalShaderParameterType::MAT2, GlobalShaderParameterType::MAT3, GlobalShaderParameterType::MAT4, GlobalShaderParameterType::TRANSFORM_2D, GlobalShaderParameterType::TRANSFORM, GlobalShaderParameterType::SAMPLER2D, GlobalShaderParameterType::SAMPLER2DARRAY, GlobalShaderParameterType::SAMPLER3D, GlobalShaderParameterType::SAMPLERCUBE, GlobalShaderParameterType::SAMPLEREXT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GlobalShaderParameterType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOOL", "GLOBAL_VAR_TYPE_BOOL", GlobalShaderParameterType::BOOL), crate::meta::inspect::EnumConstant::new("BVEC2", "GLOBAL_VAR_TYPE_BVEC2", GlobalShaderParameterType::BVEC2), crate::meta::inspect::EnumConstant::new("BVEC3", "GLOBAL_VAR_TYPE_BVEC3", GlobalShaderParameterType::BVEC3), crate::meta::inspect::EnumConstant::new("BVEC4", "GLOBAL_VAR_TYPE_BVEC4", GlobalShaderParameterType::BVEC4), crate::meta::inspect::EnumConstant::new("INT", "GLOBAL_VAR_TYPE_INT", GlobalShaderParameterType::INT), crate::meta::inspect::EnumConstant::new("IVEC2", "GLOBAL_VAR_TYPE_IVEC2", GlobalShaderParameterType::IVEC2), crate::meta::inspect::EnumConstant::new("IVEC3", "GLOBAL_VAR_TYPE_IVEC3", GlobalShaderParameterType::IVEC3), crate::meta::inspect::EnumConstant::new("IVEC4", "GLOBAL_VAR_TYPE_IVEC4", GlobalShaderParameterType::IVEC4), crate::meta::inspect::EnumConstant::new("RECT2I", "GLOBAL_VAR_TYPE_RECT2I", GlobalShaderParameterType::RECT2I), crate::meta::inspect::EnumConstant::new("UINT", "GLOBAL_VAR_TYPE_UINT", GlobalShaderParameterType::UINT), crate::meta::inspect::EnumConstant::new("UVEC2", "GLOBAL_VAR_TYPE_UVEC2", GlobalShaderParameterType::UVEC2), crate::meta::inspect::EnumConstant::new("UVEC3", "GLOBAL_VAR_TYPE_UVEC3", GlobalShaderParameterType::UVEC3), crate::meta::inspect::EnumConstant::new("UVEC4", "GLOBAL_VAR_TYPE_UVEC4", GlobalShaderParameterType::UVEC4), crate::meta::inspect::EnumConstant::new("FLOAT", "GLOBAL_VAR_TYPE_FLOAT", GlobalShaderParameterType::FLOAT), crate::meta::inspect::EnumConstant::new("VEC2", "GLOBAL_VAR_TYPE_VEC2", GlobalShaderParameterType::VEC2), crate::meta::inspect::EnumConstant::new("VEC3", "GLOBAL_VAR_TYPE_VEC3", GlobalShaderParameterType::VEC3), crate::meta::inspect::EnumConstant::new("VEC4", "GLOBAL_VAR_TYPE_VEC4", GlobalShaderParameterType::VEC4), crate::meta::inspect::EnumConstant::new("COLOR", "GLOBAL_VAR_TYPE_COLOR", GlobalShaderParameterType::COLOR), crate::meta::inspect::EnumConstant::new("RECT2", "GLOBAL_VAR_TYPE_RECT2", GlobalShaderParameterType::RECT2), crate::meta::inspect::EnumConstant::new("MAT2", "GLOBAL_VAR_TYPE_MAT2", GlobalShaderParameterType::MAT2), crate::meta::inspect::EnumConstant::new("MAT3", "GLOBAL_VAR_TYPE_MAT3", GlobalShaderParameterType::MAT3), crate::meta::inspect::EnumConstant::new("MAT4", "GLOBAL_VAR_TYPE_MAT4", GlobalShaderParameterType::MAT4), crate::meta::inspect::EnumConstant::new("TRANSFORM_2D", "GLOBAL_VAR_TYPE_TRANSFORM_2D", GlobalShaderParameterType::TRANSFORM_2D), crate::meta::inspect::EnumConstant::new("TRANSFORM", "GLOBAL_VAR_TYPE_TRANSFORM", GlobalShaderParameterType::TRANSFORM), crate::meta::inspect::EnumConstant::new("SAMPLER2D", "GLOBAL_VAR_TYPE_SAMPLER2D", GlobalShaderParameterType::SAMPLER2D), crate::meta::inspect::EnumConstant::new("SAMPLER2DARRAY", "GLOBAL_VAR_TYPE_SAMPLER2DARRAY", GlobalShaderParameterType::SAMPLER2DARRAY), crate::meta::inspect::EnumConstant::new("SAMPLER3D", "GLOBAL_VAR_TYPE_SAMPLER3D", GlobalShaderParameterType::SAMPLER3D), crate::meta::inspect::EnumConstant::new("SAMPLERCUBE", "GLOBAL_VAR_TYPE_SAMPLERCUBE", GlobalShaderParameterType::SAMPLERCUBE), crate::meta::inspect::EnumConstant::new("SAMPLEREXT", "GLOBAL_VAR_TYPE_SAMPLEREXT", GlobalShaderParameterType::SAMPLEREXT), crate::meta::inspect::EnumConstant::new("MAX", "GLOBAL_VAR_TYPE_MAX", GlobalShaderParameterType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for GlobalShaderParameterType {
    const ENUMERATOR_COUNT: usize = 29usize;
    
}
impl crate::meta::GodotConvert for GlobalShaderParameterType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Global Var Type Bool", 0i64), EnumeratorShape::new_int("Global Var Type Bvec2", 1i64), EnumeratorShape::new_int("Global Var Type Bvec3", 2i64), EnumeratorShape::new_int("Global Var Type Bvec4", 3i64), EnumeratorShape::new_int("Global Var Type Int", 4i64), EnumeratorShape::new_int("Global Var Type Ivec2", 5i64), EnumeratorShape::new_int("Global Var Type Ivec3", 6i64), EnumeratorShape::new_int("Global Var Type Ivec4", 7i64), EnumeratorShape::new_int("Global Var Type Rect2i", 8i64), EnumeratorShape::new_int("Global Var Type Uint", 9i64), EnumeratorShape::new_int("Global Var Type Uvec2", 10i64), EnumeratorShape::new_int("Global Var Type Uvec3", 11i64), EnumeratorShape::new_int("Global Var Type Uvec4", 12i64), EnumeratorShape::new_int("Global Var Type Float", 13i64), EnumeratorShape::new_int("Global Var Type Vec2", 14i64), EnumeratorShape::new_int("Global Var Type Vec3", 15i64), EnumeratorShape::new_int("Global Var Type Vec4", 16i64), EnumeratorShape::new_int("Global Var Type Color", 17i64), EnumeratorShape::new_int("Global Var Type Rect2", 18i64), EnumeratorShape::new_int("Global Var Type Mat2", 19i64), EnumeratorShape::new_int("Global Var Type Mat3", 20i64), EnumeratorShape::new_int("Global Var Type Mat4", 21i64), EnumeratorShape::new_int("Global Var Type Transform 2d", 22i64), EnumeratorShape::new_int("Global Var Type Transform", 23i64), EnumeratorShape::new_int("Global Var Type Sampler2d", 24i64), EnumeratorShape::new_int("Global Var Type Sampler2darray", 25i64), EnumeratorShape::new_int("Global Var Type Sampler3d", 26i64), EnumeratorShape::new_int("Global Var Type Samplercube", 27i64), EnumeratorShape::new_int("Global Var Type Samplerext", 28i64), EnumeratorShape::new_int("Global Var Type Max", 29i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.GlobalShaderParameterType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GlobalShaderParameterType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GlobalShaderParameterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GlobalShaderParameterType {
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
impl crate::registry::property::Export for GlobalShaderParameterType {
    
}
impl crate::meta::Element for GlobalShaderParameterType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderingInfo {
    ord: i32
}
impl RenderingInfo {
    #[doc(alias = "RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME`"]
    pub const TOTAL_OBJECTS_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 0i32
    };
    #[doc(alias = "RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME`"]
    pub const TOTAL_PRIMITIVES_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 1i32
    };
    #[doc(alias = "RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME`"]
    pub const TOTAL_DRAW_CALLS_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 2i32
    };
    #[doc(alias = "RENDERING_INFO_TEXTURE_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TEXTURE_MEM_USED`"]
    pub const TEXTURE_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 3i32
    };
    #[doc(alias = "RENDERING_INFO_BUFFER_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_BUFFER_MEM_USED`"]
    pub const BUFFER_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 4i32
    };
    #[doc(alias = "RENDERING_INFO_VIDEO_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_VIDEO_MEM_USED`"]
    pub const VIDEO_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 5i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS`"]
    pub const PIPELINE_COMPILATIONS_CANVAS: RenderingInfo = RenderingInfo {
        ord: 6i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_MESH")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_MESH`"]
    pub const PIPELINE_COMPILATIONS_MESH: RenderingInfo = RenderingInfo {
        ord: 7i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE`"]
    pub const PIPELINE_COMPILATIONS_SURFACE: RenderingInfo = RenderingInfo {
        ord: 8i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW`"]
    pub const PIPELINE_COMPILATIONS_DRAW: RenderingInfo = RenderingInfo {
        ord: 9i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION`"]
    pub const PIPELINE_COMPILATIONS_SPECIALIZATION: RenderingInfo = RenderingInfo {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for RenderingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderingInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderingInfo {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::TOTAL_OBJECTS_IN_FRAME => "TOTAL_OBJECTS_IN_FRAME", Self::TOTAL_PRIMITIVES_IN_FRAME => "TOTAL_PRIMITIVES_IN_FRAME", Self::TOTAL_DRAW_CALLS_IN_FRAME => "TOTAL_DRAW_CALLS_IN_FRAME", Self::TEXTURE_MEM_USED => "TEXTURE_MEM_USED", Self::BUFFER_MEM_USED => "BUFFER_MEM_USED", Self::VIDEO_MEM_USED => "VIDEO_MEM_USED", Self::PIPELINE_COMPILATIONS_CANVAS => "PIPELINE_COMPILATIONS_CANVAS", Self::PIPELINE_COMPILATIONS_MESH => "PIPELINE_COMPILATIONS_MESH", Self::PIPELINE_COMPILATIONS_SURFACE => "PIPELINE_COMPILATIONS_SURFACE", Self::PIPELINE_COMPILATIONS_DRAW => "PIPELINE_COMPILATIONS_DRAW", Self::PIPELINE_COMPILATIONS_SPECIALIZATION => "PIPELINE_COMPILATIONS_SPECIALIZATION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderingInfo::TOTAL_OBJECTS_IN_FRAME, RenderingInfo::TOTAL_PRIMITIVES_IN_FRAME, RenderingInfo::TOTAL_DRAW_CALLS_IN_FRAME, RenderingInfo::TEXTURE_MEM_USED, RenderingInfo::BUFFER_MEM_USED, RenderingInfo::VIDEO_MEM_USED, RenderingInfo::PIPELINE_COMPILATIONS_CANVAS, RenderingInfo::PIPELINE_COMPILATIONS_MESH, RenderingInfo::PIPELINE_COMPILATIONS_SURFACE, RenderingInfo::PIPELINE_COMPILATIONS_DRAW, RenderingInfo::PIPELINE_COMPILATIONS_SPECIALIZATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderingInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOTAL_OBJECTS_IN_FRAME", "RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME", RenderingInfo::TOTAL_OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("TOTAL_PRIMITIVES_IN_FRAME", "RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME", RenderingInfo::TOTAL_PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("TOTAL_DRAW_CALLS_IN_FRAME", "RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME", RenderingInfo::TOTAL_DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("TEXTURE_MEM_USED", "RENDERING_INFO_TEXTURE_MEM_USED", RenderingInfo::TEXTURE_MEM_USED), crate::meta::inspect::EnumConstant::new("BUFFER_MEM_USED", "RENDERING_INFO_BUFFER_MEM_USED", RenderingInfo::BUFFER_MEM_USED), crate::meta::inspect::EnumConstant::new("VIDEO_MEM_USED", "RENDERING_INFO_VIDEO_MEM_USED", RenderingInfo::VIDEO_MEM_USED), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_CANVAS", "RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS", RenderingInfo::PIPELINE_COMPILATIONS_CANVAS), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_MESH", "RENDERING_INFO_PIPELINE_COMPILATIONS_MESH", RenderingInfo::PIPELINE_COMPILATIONS_MESH), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SURFACE", "RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE", RenderingInfo::PIPELINE_COMPILATIONS_SURFACE), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_DRAW", "RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW", RenderingInfo::PIPELINE_COMPILATIONS_DRAW), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SPECIALIZATION", "RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION", RenderingInfo::PIPELINE_COMPILATIONS_SPECIALIZATION)]
        }
    }
}
impl crate::meta::GodotConvert for RenderingInfo {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Rendering Info Total Objects In Frame", 0i64), EnumeratorShape::new_int("Rendering Info Total Primitives In Frame", 1i64), EnumeratorShape::new_int("Rendering Info Total Draw Calls In Frame", 2i64), EnumeratorShape::new_int("Rendering Info Texture Mem Used", 3i64), EnumeratorShape::new_int("Rendering Info Buffer Mem Used", 4i64), EnumeratorShape::new_int("Rendering Info Video Mem Used", 5i64), EnumeratorShape::new_int("Rendering Info Pipeline Compilations Canvas", 6i64), EnumeratorShape::new_int("Rendering Info Pipeline Compilations Mesh", 7i64), EnumeratorShape::new_int("Rendering Info Pipeline Compilations Surface", 8i64), EnumeratorShape::new_int("Rendering Info Pipeline Compilations Draw", 9i64), EnumeratorShape::new_int("Rendering Info Pipeline Compilations Specialization", 10i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.RenderingInfo")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for RenderingInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderingInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for RenderingInfo {
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
impl crate::registry::property::Export for RenderingInfo {
    
}
impl crate::meta::Element for RenderingInfo {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PipelineSource {
    ord: i32
}
impl PipelineSource {
    #[doc(alias = "PIPELINE_SOURCE_CANVAS")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_CANVAS`"]
    pub const CANVAS: PipelineSource = PipelineSource {
        ord: 0i32
    };
    #[doc(alias = "PIPELINE_SOURCE_MESH")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_MESH`"]
    pub const MESH: PipelineSource = PipelineSource {
        ord: 1i32
    };
    #[doc(alias = "PIPELINE_SOURCE_SURFACE")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_SURFACE`"]
    pub const SURFACE: PipelineSource = PipelineSource {
        ord: 2i32
    };
    #[doc(alias = "PIPELINE_SOURCE_DRAW")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_DRAW`"]
    pub const DRAW: PipelineSource = PipelineSource {
        ord: 3i32
    };
    #[doc(alias = "PIPELINE_SOURCE_SPECIALIZATION")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_SPECIALIZATION`"]
    pub const SPECIALIZATION: PipelineSource = PipelineSource {
        ord: 4i32
    };
    #[doc(alias = "PIPELINE_SOURCE_MAX")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_MAX`"]
    pub const MAX: PipelineSource = PipelineSource {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for PipelineSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PipelineSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PipelineSource {
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
            Self::CANVAS => "CANVAS", Self::MESH => "MESH", Self::SURFACE => "SURFACE", Self::DRAW => "DRAW", Self::SPECIALIZATION => "SPECIALIZATION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PipelineSource::CANVAS, PipelineSource::MESH, PipelineSource::SURFACE, PipelineSource::DRAW, PipelineSource::SPECIALIZATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PipelineSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CANVAS", "PIPELINE_SOURCE_CANVAS", PipelineSource::CANVAS), crate::meta::inspect::EnumConstant::new("MESH", "PIPELINE_SOURCE_MESH", PipelineSource::MESH), crate::meta::inspect::EnumConstant::new("SURFACE", "PIPELINE_SOURCE_SURFACE", PipelineSource::SURFACE), crate::meta::inspect::EnumConstant::new("DRAW", "PIPELINE_SOURCE_DRAW", PipelineSource::DRAW), crate::meta::inspect::EnumConstant::new("SPECIALIZATION", "PIPELINE_SOURCE_SPECIALIZATION", PipelineSource::SPECIALIZATION), crate::meta::inspect::EnumConstant::new("MAX", "PIPELINE_SOURCE_MAX", PipelineSource::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PipelineSource {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for PipelineSource {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Pipeline Source Canvas", 0i64), EnumeratorShape::new_int("Pipeline Source Mesh", 1i64), EnumeratorShape::new_int("Pipeline Source Surface", 2i64), EnumeratorShape::new_int("Pipeline Source Draw", 3i64), EnumeratorShape::new_int("Pipeline Source Specialization", 4i64), EnumeratorShape::new_int("Pipeline Source Max", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.PipelineSource")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PipelineSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PipelineSource {
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
impl crate::registry::property::Export for PipelineSource {
    
}
impl crate::meta::Element for PipelineSource {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SplashStretchMode {
    ord: i32
}
impl SplashStretchMode {
    #[doc(alias = "SPLASH_STRETCH_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_DISABLED`"]
    pub const DISABLED: SplashStretchMode = SplashStretchMode {
        ord: 0i32
    };
    #[doc(alias = "SPLASH_STRETCH_MODE_KEEP")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_KEEP`"]
    pub const KEEP: SplashStretchMode = SplashStretchMode {
        ord: 1i32
    };
    #[doc(alias = "SPLASH_STRETCH_MODE_KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_KEEP_WIDTH`"]
    pub const KEEP_WIDTH: SplashStretchMode = SplashStretchMode {
        ord: 2i32
    };
    #[doc(alias = "SPLASH_STRETCH_MODE_KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_KEEP_HEIGHT`"]
    pub const KEEP_HEIGHT: SplashStretchMode = SplashStretchMode {
        ord: 3i32
    };
    #[doc(alias = "SPLASH_STRETCH_MODE_COVER")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_COVER`"]
    pub const COVER: SplashStretchMode = SplashStretchMode {
        ord: 4i32
    };
    #[doc(alias = "SPLASH_STRETCH_MODE_IGNORE")]
    #[doc = "Godot enumerator name: `SPLASH_STRETCH_MODE_IGNORE`"]
    pub const IGNORE: SplashStretchMode = SplashStretchMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SplashStretchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SplashStretchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SplashStretchMode {
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
            Self::DISABLED => "DISABLED", Self::KEEP => "KEEP", Self::KEEP_WIDTH => "KEEP_WIDTH", Self::KEEP_HEIGHT => "KEEP_HEIGHT", Self::COVER => "COVER", Self::IGNORE => "IGNORE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SplashStretchMode::DISABLED, SplashStretchMode::KEEP, SplashStretchMode::KEEP_WIDTH, SplashStretchMode::KEEP_HEIGHT, SplashStretchMode::COVER, SplashStretchMode::IGNORE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SplashStretchMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SPLASH_STRETCH_MODE_DISABLED", SplashStretchMode::DISABLED), crate::meta::inspect::EnumConstant::new("KEEP", "SPLASH_STRETCH_MODE_KEEP", SplashStretchMode::KEEP), crate::meta::inspect::EnumConstant::new("KEEP_WIDTH", "SPLASH_STRETCH_MODE_KEEP_WIDTH", SplashStretchMode::KEEP_WIDTH), crate::meta::inspect::EnumConstant::new("KEEP_HEIGHT", "SPLASH_STRETCH_MODE_KEEP_HEIGHT", SplashStretchMode::KEEP_HEIGHT), crate::meta::inspect::EnumConstant::new("COVER", "SPLASH_STRETCH_MODE_COVER", SplashStretchMode::COVER), crate::meta::inspect::EnumConstant::new("IGNORE", "SPLASH_STRETCH_MODE_IGNORE", SplashStretchMode::IGNORE)]
        }
    }
}
impl crate::meta::GodotConvert for SplashStretchMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Splash Stretch Mode Disabled", 0i64), EnumeratorShape::new_int("Splash Stretch Mode Keep", 1i64), EnumeratorShape::new_int("Splash Stretch Mode Keep Width", 2i64), EnumeratorShape::new_int("Splash Stretch Mode Keep Height", 3i64), EnumeratorShape::new_int("Splash Stretch Mode Cover", 4i64), EnumeratorShape::new_int("Splash Stretch Mode Ignore", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.SplashStretchMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SplashStretchMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SplashStretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SplashStretchMode {
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
impl crate::registry::property::Export for SplashStretchMode {
    
}
impl crate::meta::Element for SplashStretchMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Features {
    ord: i32
}
impl Features {
    #[doc(alias = "FEATURE_SHADERS")]
    #[doc = "Godot enumerator name: `FEATURE_SHADERS`"]
    pub const SHADERS: Features = Features {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_MULTITHREADED")]
    #[doc = "Godot enumerator name: `FEATURE_MULTITHREADED`"]
    pub const MULTITHREADED: Features = Features {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Features") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Features {
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
            Self::SHADERS => "SHADERS", Self::MULTITHREADED => "MULTITHREADED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Features::SHADERS, Features::MULTITHREADED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Features >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHADERS", "FEATURE_SHADERS", Features::SHADERS), crate::meta::inspect::EnumConstant::new("MULTITHREADED", "FEATURE_MULTITHREADED", Features::MULTITHREADED)]
        }
    }
}
impl crate::meta::GodotConvert for Features {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Feature Shaders", 0i64), EnumeratorShape::new_int("Feature Multithreaded", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("RenderingServer.Features")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Features {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Features {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Features {
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
impl crate::registry::property::Export for Features {
    
}
impl crate::meta::Element for Features {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderingServer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RenderingServer`][crate::classes::RenderingServer] class."]
    pub struct SignalsOfRenderingServer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRenderingServer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn frame_pre_draw(&mut self) -> SigFramePreDraw < 'c, C > {
            SigFramePreDraw {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "frame_pre_draw")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn frame_post_draw(&mut self) -> SigFramePostDraw < 'c, C > {
            SigFramePostDraw {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "frame_post_draw")
            }
        }
    }
    type TypedSigFramePreDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFramePreDraw < 'c, C: WithSignals > {
        typed: TypedSigFramePreDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFramePreDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFramePreDraw < 'c, C > {
        type Target = TypedSigFramePreDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFramePreDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFramePostDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFramePostDraw < 'c, C: WithSignals > {
        typed: TypedSigFramePostDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFramePostDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFramePostDraw < 'c, C > {
        type Target = TypedSigFramePostDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFramePostDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RenderingServer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRenderingServer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRenderingServer < 'c, C > {
        type Target = < < RenderingServer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RenderingServer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRenderingServer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RenderingServer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}