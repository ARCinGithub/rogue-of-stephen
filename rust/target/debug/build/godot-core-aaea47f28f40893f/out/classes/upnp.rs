#![doc = "Sidecar module for class [`Upnp`][crate::classes::Upnp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNP` enums](https://docs.godotengine.org/en/stable/classes/class_upnp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UPNP`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`upnp`][crate::classes::upnp]: sidecar module with related enum/flag types\n* [`IUpnp`][crate::classes::IUpnp]: virtual methods\n\n\nSee also [Godot docs for `UPNP`](https://docs.godotengine.org/en/stable/classes/class_upnp.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Upnp::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class can be used to discover compatible [`UPNPDevice`][crate::classes::UpnpDevice]s on the local network and execute commands on them, like managing port mappings (for port forwarding/NAT traversal) and querying the local and remote network IP address. Note that methods on this class are synchronous and block the calling thread.\n\nTo forward a specific port (here `7777`, note both [`discover`][`crate::classes::Upnp::discover`] and [`add_port_mapping`][`crate::classes::Upnp::add_port_mapping`] can return errors that should be checked):\n\n```gdscript\nvar upnp = UPNP.new()\nupnp.discover()\nupnp.add_port_mapping(7777)\n```\n\nTo close a specific port (e.g. after you have finished using it):\n\n```gdscript\nupnp.delete_port_mapping(port)\n```\n\n**Note:** UPnP discovery blocks the current thread. To perform discovery without blocking the main thread, use `Thread`s like this:\n\n```gdscript\n# Emitted when UPnP port mapping setup is completed (regardless of success or failure).\nsignal upnp_completed(error)\n\n# Replace this with your own server port number between 1024 and 65535.\nconst SERVER_PORT = 3928\nvar thread = null\n\nfunc _upnp_setup(server_port):\n\t# UPNP queries take some time.\n\tvar upnp = UPNP.new()\n\tvar err = upnp.discover()\n\n\tif err != OK:\n\t\tpush_error(str(err))\n\t\tupnp_completed.emit(err)\n\t\treturn\n\n\tif upnp.get_gateway() and upnp.get_gateway().is_valid_gateway():\n\t\tupnp.add_port_mapping(server_port, server_port, ProjectSettings.get_setting(\"application/config/name\"), \"UDP\")\n\t\tupnp.add_port_mapping(server_port, server_port, ProjectSettings.get_setting(\"application/config/name\"), \"TCP\")\n\t\tupnp_completed.emit(OK)\n\nfunc _ready():\n\tthread = Thread.new()\n\tthread.start(_upnp_setup.bind(SERVER_PORT))\n\nfunc _exit_tree():\n\t# Wait for thread finish here to handle game exit while the thread is running.\n\tthread.wait_to_finish()\n```\n\n**Terminology:** In the context of UPnP networking, \"gateway\" (or \"internet gateway device\", short IGD) refers to network devices that allow computers in the local network to access the internet (\"wide area network\", WAN). These gateways are often also called \"routers\".\n\n**Pitfalls:**\n\n- As explained above, these calls are blocking and shouldn't be run on the main thread, especially as they can block for multiple seconds at a time. Use threading!\n\n- Networking is physical and messy. Packets get lost in transit or get filtered, addresses, free ports and assigned mappings change, and devices may leave or join the network at any time. Be mindful of this, be diligent when checking and handling errors, and handle these gracefully if you can: add clear error UI, timeouts and re-try handling.\n\n- Port mappings may change (and be removed) at any time, and the remote/external IP address of the gateway can change likewise. You should consider re-querying the external IP and try to update/refresh the port mapping periodically (for example, every 5 minutes and on networking failures).\n\n- Not all devices support UPnP, and some users disable UPnP support. You need to handle this (e.g. documenting and requiring the user to manually forward ports, or adding alternative methods of NAT traversal, like a relay/mirror server, or NAT hole punching, STUN/TURN, etc.).\n\n- Consider what happens on mapping conflicts. Maybe multiple users on the same network would like to play your game at the same time, or maybe another application uses the same port. Make the port configurable, and optimally choose a port automatically (re-trying with a different port on failure).\n\n**Further reading:** If you want to know more about UPnP (and the Internet Gateway Device (IGD) and Port Control Protocol (PCP) specifically), [Wikipedia](https://en.wikipedia.org/wiki/Universal_Plug_and_Play) is a good first stop, the specification can be found at the [Open Connectivity Foundation](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/) and Godot's implementation is based on the [MiniUPnP client](https://github.com/miniupnp/miniupnp)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Upnp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Upnp`][crate::classes::Upnp].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `UPNP` methods](https://docs.godotengine.org/en/stable/classes/class_upnp.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUpnp: crate::obj::GodotClass < Base = Upnp > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl Upnp {
        #[doc = "Returns the number of discovered [`UPNPDevice`][crate::classes::UpnpDevice]s."]
        pub fn get_device_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_device_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`UPNPDevice`][crate::classes::UpnpDevice] at the given `index`."]
        pub fn get_device(&self, index: i32,) -> Option < Gd < crate::classes::UpnpDevice > > {
            type CallRet = Option < Gd < crate::classes::UpnpDevice > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds the given [`UPNPDevice`][crate::classes::UpnpDevice] to the list of discovered devices."]
        pub fn add_device(&mut self, device: impl AsArg < Option < Gd < crate::classes::UpnpDevice >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::UpnpDevice > > >,);
            let args = (device.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "add_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the device at `index` from the list of discovered devices to `device`."]
        pub fn set_device(&mut self, index: i32, device: impl AsArg < Option < Gd < crate::classes::UpnpDevice >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::UpnpDevice > > >,);
            let args = (index, device.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the device at `index` from the list of discovered devices."]
        pub fn remove_device(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "remove_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the list of discovered devices."]
        pub fn clear_devices(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "clear_devices", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the default gateway. That is the first discovered [`UPNPDevice`][crate::classes::UpnpDevice] that is also a valid IGD (InternetGatewayDevice)."]
        pub fn get_gateway(&self,) -> Option < Gd < crate::classes::UpnpDevice > > {
            type CallRet = Option < Gd < crate::classes::UpnpDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_gateway", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Discovers local [`UPNPDevice`][crate::classes::UpnpDevice]s. Clears the list of previously discovered devices.\n\nFilters for IGD (InternetGatewayDevice) type devices by default, as those manage port forwarding. `timeout` is the time to wait for responses in milliseconds. `ttl` is the time-to-live; only touch this if you know what you're doing.\n\nSee \\[enum UPNPResult] for possible return values."]
        pub(crate) fn discover_full(&mut self, timeout: i32, ttl: i32, device_filter: CowArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (timeout, ttl, device_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "discover", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`discover_ex`][Self::discover_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Discovers local [`UPNPDevice`][crate::classes::UpnpDevice]s. Clears the list of previously discovered devices.\n\nFilters for IGD (InternetGatewayDevice) type devices by default, as those manage port forwarding. `timeout` is the time to wait for responses in milliseconds. `ttl` is the time-to-live; only touch this if you know what you're doing.\n\nSee \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn discover(&mut self,) -> i32 {
            self.discover_ex() . done()
        }
        #[doc = "Discovers local [`UPNPDevice`][crate::classes::UpnpDevice]s. Clears the list of previously discovered devices.\n\nFilters for IGD (InternetGatewayDevice) type devices by default, as those manage port forwarding. `timeout` is the time to wait for responses in milliseconds. `ttl` is the time-to-live; only touch this if you know what you're doing.\n\nSee \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn discover_ex < 'ex > (&'ex mut self,) -> ExDiscover < 'ex > {
            ExDiscover::new(self,)
        }
        #[doc = "Returns the external [`IP`][crate::classes::Ip] address of the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) as string. Returns an empty string on error."]
        pub fn query_external_address(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "query_external_address", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a mapping to forward the external `port` (between 1 and 65535, although recommended to use port 1024 or above) on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) to the `port_internal` on the local machine for the given protocol `proto` (either `\"TCP\"` or `\"UDP\"`, with UDP being the default). If a port mapping for the given port and protocol combination already exists on that gateway device, this method tries to overwrite it. If that is not desired, you can retrieve the gateway manually with [`get_gateway`][`crate::classes::Upnp::get_gateway`] and call [`add_port_mapping`][`crate::classes::Upnp::add_port_mapping`] on it, if any. Note that forwarding a well-known port (below 1024) with UPnP may fail depending on the device.\n\nDepending on the gateway device, if a mapping for that port already exists, it will either be updated or it will refuse this command due to that conflict, especially if the existing mapping for that port wasn't created via UPnP or points to a different network address (or device) than this one.\n\nIf `port_internal` is `0` (the default), the same port number is used for both the external and the internal port (the `port` value).\n\nThe description (`desc`) is shown in some routers management UIs and can be used to point out which application added the mapping.\n\nThe mapping's lease `duration` can be limited by specifying a duration in seconds. The default of `0` means no duration, i.e. a permanent lease and notably some devices only support these permanent leases. Note that whether permanent or not, this is only a request and the gateway may still decide at any point to remove the mapping (which usually happens on a reboot of the gateway, when its external IP address changes, or on some models when it detects a port mapping has become inactive, i.e. had no traffic for multiple minutes). If not `0` (permanent), the allowed range according to spec is between `120` (2 minutes) and `86400` seconds (24 hours).\n\nSee \\[enum UPNPResult] for possible return values."]
        pub(crate) fn add_port_mapping_full(&self, port: i32, port_internal: i32, desc: CowArg < GString >, proto: CowArg < GString >, duration: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (i32, i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (port, port_internal, desc, proto, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "add_port_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_port_mapping_ex`][Self::add_port_mapping_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a mapping to forward the external `port` (between 1 and 65535, although recommended to use port 1024 or above) on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) to the `port_internal` on the local machine for the given protocol `proto` (either `\"TCP\"` or `\"UDP\"`, with UDP being the default). If a port mapping for the given port and protocol combination already exists on that gateway device, this method tries to overwrite it. If that is not desired, you can retrieve the gateway manually with [`get_gateway`][`crate::classes::Upnp::get_gateway`] and call [`add_port_mapping`][`crate::classes::Upnp::add_port_mapping`] on it, if any. Note that forwarding a well-known port (below 1024) with UPnP may fail depending on the device.\n\nDepending on the gateway device, if a mapping for that port already exists, it will either be updated or it will refuse this command due to that conflict, especially if the existing mapping for that port wasn't created via UPnP or points to a different network address (or device) than this one.\n\nIf `port_internal` is `0` (the default), the same port number is used for both the external and the internal port (the `port` value).\n\nThe description (`desc`) is shown in some routers management UIs and can be used to point out which application added the mapping.\n\nThe mapping's lease `duration` can be limited by specifying a duration in seconds. The default of `0` means no duration, i.e. a permanent lease and notably some devices only support these permanent leases. Note that whether permanent or not, this is only a request and the gateway may still decide at any point to remove the mapping (which usually happens on a reboot of the gateway, when its external IP address changes, or on some models when it detects a port mapping has become inactive, i.e. had no traffic for multiple minutes). If not `0` (permanent), the allowed range according to spec is between `120` (2 minutes) and `86400` seconds (24 hours).\n\nSee \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn add_port_mapping(&self, port: i32,) -> i32 {
            self.add_port_mapping_ex(port,) . done()
        }
        #[doc = "Adds a mapping to forward the external `port` (between 1 and 65535, although recommended to use port 1024 or above) on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) to the `port_internal` on the local machine for the given protocol `proto` (either `\"TCP\"` or `\"UDP\"`, with UDP being the default). If a port mapping for the given port and protocol combination already exists on that gateway device, this method tries to overwrite it. If that is not desired, you can retrieve the gateway manually with [`get_gateway`][`crate::classes::Upnp::get_gateway`] and call [`add_port_mapping`][`crate::classes::Upnp::add_port_mapping`] on it, if any. Note that forwarding a well-known port (below 1024) with UPnP may fail depending on the device.\n\nDepending on the gateway device, if a mapping for that port already exists, it will either be updated or it will refuse this command due to that conflict, especially if the existing mapping for that port wasn't created via UPnP or points to a different network address (or device) than this one.\n\nIf `port_internal` is `0` (the default), the same port number is used for both the external and the internal port (the `port` value).\n\nThe description (`desc`) is shown in some routers management UIs and can be used to point out which application added the mapping.\n\nThe mapping's lease `duration` can be limited by specifying a duration in seconds. The default of `0` means no duration, i.e. a permanent lease and notably some devices only support these permanent leases. Note that whether permanent or not, this is only a request and the gateway may still decide at any point to remove the mapping (which usually happens on a reboot of the gateway, when its external IP address changes, or on some models when it detects a port mapping has become inactive, i.e. had no traffic for multiple minutes). If not `0` (permanent), the allowed range according to spec is between `120` (2 minutes) and `86400` seconds (24 hours).\n\nSee \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn add_port_mapping_ex < 'ex > (&'ex self, port: i32,) -> ExAddPortMapping < 'ex > {
            ExAddPortMapping::new(self, port,)
        }
        #[doc = "Deletes the port mapping for the given port and protocol combination on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) if one exists. `port` must be a valid port between 1 and 65535, `proto` can be either `\"TCP\"` or `\"UDP\"`. May be refused for mappings pointing to addresses other than this one, for well-known ports (below 1024), or for mappings not added via UPnP. See \\[enum UPNPResult] for possible return values."]
        pub(crate) fn delete_port_mapping_full(&self, port: i32, proto: CowArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (port, proto,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "delete_port_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`delete_port_mapping_ex`][Self::delete_port_mapping_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Deletes the port mapping for the given port and protocol combination on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) if one exists. `port` must be a valid port between 1 and 65535, `proto` can be either `\"TCP\"` or `\"UDP\"`. May be refused for mappings pointing to addresses other than this one, for well-known ports (below 1024), or for mappings not added via UPnP. See \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn delete_port_mapping(&self, port: i32,) -> i32 {
            self.delete_port_mapping_ex(port,) . done()
        }
        #[doc = "Deletes the port mapping for the given port and protocol combination on the default gateway (see [`get_gateway`][`crate::classes::Upnp::get_gateway`]) if one exists. `port` must be a valid port between 1 and 65535, `proto` can be either `\"TCP\"` or `\"UDP\"`. May be refused for mappings pointing to addresses other than this one, for well-known ports (below 1024), or for mappings not added via UPnP. See \\[enum UPNPResult] for possible return values."]
        #[inline]
        pub fn delete_port_mapping_ex < 'ex > (&'ex self, port: i32,) -> ExDeletePortMapping < 'ex > {
            ExDeletePortMapping::new(self, port,)
        }
        pub fn set_discover_multicast_if(&mut self, m_if: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (m_if.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_multicast_if", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_discover_multicast_if(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_discover_multicast_if", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_discover_local_port(&mut self, port: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_local_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_discover_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_discover_local_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_discover_ipv6(&mut self, ipv6: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ipv6,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_ipv6", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_discover_ipv6(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "is_discover_ipv6", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Upnp {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("UPNP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Upnp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Upnp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Upnp {
        
    }
    impl crate::obj::cap::GodotDefault for Upnp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Upnp {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Upnp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Upnp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Upnp__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Upnp > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Upnp::discover_ex`][super::Upnp::discover_ex]."]
#[must_use]
pub struct ExDiscover < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Upnp, timeout: i32, ttl: i32, device_filter: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDiscover < 'ex > {
    fn new(surround_object: &'ex mut re_export::Upnp,) -> Self {
        let timeout = 2000i32;
        let ttl = 2i32;
        let device_filter = GString::from("InternetGatewayDevice");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, timeout: timeout, ttl: ttl, device_filter: CowArg::Owned(device_filter),
        }
    }
    #[inline]
    pub fn timeout(self, timeout: i32) -> Self {
        Self {
            timeout: timeout, .. self
        }
    }
    #[inline]
    pub fn ttl(self, ttl: i32) -> Self {
        Self {
            ttl: ttl, .. self
        }
    }
    #[inline]
    pub fn device_filter(self, device_filter: impl AsArg < GString > + 'ex) -> Self {
        Self {
            device_filter: device_filter.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, timeout, ttl, device_filter,
        }
        = self;
        re_export::Upnp::discover_full(surround_object, timeout, ttl, device_filter,)
    }
}
#[doc = "Default-param extender for [`Upnp::add_port_mapping_ex`][super::Upnp::add_port_mapping_ex]."]
#[must_use]
pub struct ExAddPortMapping < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Upnp, port: i32, port_internal: i32, desc: CowArg < 'ex, GString >, proto: CowArg < 'ex, GString >, duration: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddPortMapping < 'ex > {
    fn new(surround_object: &'ex re_export::Upnp, port: i32,) -> Self {
        let port_internal = 0i32;
        let desc = GString::from("");
        let proto = GString::from("UDP");
        let duration = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, port_internal: port_internal, desc: CowArg::Owned(desc), proto: CowArg::Owned(proto), duration: duration,
        }
    }
    #[inline]
    pub fn port_internal(self, port_internal: i32) -> Self {
        Self {
            port_internal: port_internal, .. self
        }
    }
    #[inline]
    pub fn desc(self, desc: impl AsArg < GString > + 'ex) -> Self {
        Self {
            desc: desc.into_arg(), .. self
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'ex) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn duration(self, duration: i32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, port_internal, desc, proto, duration,
        }
        = self;
        re_export::Upnp::add_port_mapping_full(surround_object, port, port_internal, desc, proto, duration,)
    }
}
#[doc = "Default-param extender for [`Upnp::delete_port_mapping_ex`][super::Upnp::delete_port_mapping_ex]."]
#[must_use]
pub struct ExDeletePortMapping < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Upnp, port: i32, proto: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDeletePortMapping < 'ex > {
    fn new(surround_object: &'ex re_export::Upnp, port: i32,) -> Self {
        let proto = GString::from("UDP");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, proto: CowArg::Owned(proto),
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'ex) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, proto,
        }
        = self;
        re_export::Upnp::delete_port_mapping_full(surround_object, port, proto,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `UPNPResult`."]
pub struct UpnpResult {
    ord: i32
}
impl UpnpResult {
    #[doc(alias = "UPNP_RESULT_SUCCESS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SUCCESS`"]
    pub const SUCCESS: UpnpResult = UpnpResult {
        ord: 0i32
    };
    #[doc(alias = "UPNP_RESULT_NOT_AUTHORIZED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NOT_AUTHORIZED`"]
    pub const NOT_AUTHORIZED: UpnpResult = UpnpResult {
        ord: 1i32
    };
    #[doc(alias = "UPNP_RESULT_PORT_MAPPING_NOT_FOUND")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_PORT_MAPPING_NOT_FOUND`"]
    pub const PORT_MAPPING_NOT_FOUND: UpnpResult = UpnpResult {
        ord: 2i32
    };
    #[doc(alias = "UPNP_RESULT_INCONSISTENT_PARAMETERS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INCONSISTENT_PARAMETERS`"]
    pub const INCONSISTENT_PARAMETERS: UpnpResult = UpnpResult {
        ord: 3i32
    };
    #[doc(alias = "UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY`"]
    pub const NO_SUCH_ENTRY_IN_ARRAY: UpnpResult = UpnpResult {
        ord: 4i32
    };
    #[doc(alias = "UPNP_RESULT_ACTION_FAILED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_ACTION_FAILED`"]
    pub const ACTION_FAILED: UpnpResult = UpnpResult {
        ord: 5i32
    };
    #[doc(alias = "UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED`"]
    pub const SRC_IP_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 6i32
    };
    #[doc(alias = "UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED`"]
    pub const EXT_PORT_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 7i32
    };
    #[doc(alias = "UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED`"]
    pub const INT_PORT_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 8i32
    };
    #[doc(alias = "UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD`"]
    pub const REMOTE_HOST_MUST_BE_WILDCARD: UpnpResult = UpnpResult {
        ord: 9i32
    };
    #[doc(alias = "UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD`"]
    pub const EXT_PORT_MUST_BE_WILDCARD: UpnpResult = UpnpResult {
        ord: 10i32
    };
    #[doc(alias = "UPNP_RESULT_NO_PORT_MAPS_AVAILABLE")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_PORT_MAPS_AVAILABLE`"]
    pub const NO_PORT_MAPS_AVAILABLE: UpnpResult = UpnpResult {
        ord: 11i32
    };
    #[doc(alias = "UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM`"]
    pub const CONFLICT_WITH_OTHER_MECHANISM: UpnpResult = UpnpResult {
        ord: 12i32
    };
    #[doc(alias = "UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING`"]
    pub const CONFLICT_WITH_OTHER_MAPPING: UpnpResult = UpnpResult {
        ord: 13i32
    };
    #[doc(alias = "UPNP_RESULT_SAME_PORT_VALUES_REQUIRED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SAME_PORT_VALUES_REQUIRED`"]
    pub const SAME_PORT_VALUES_REQUIRED: UpnpResult = UpnpResult {
        ord: 14i32
    };
    #[doc(alias = "UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED`"]
    pub const ONLY_PERMANENT_LEASE_SUPPORTED: UpnpResult = UpnpResult {
        ord: 15i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_GATEWAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_GATEWAY`"]
    pub const INVALID_GATEWAY: UpnpResult = UpnpResult {
        ord: 16i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PORT")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PORT`"]
    pub const INVALID_PORT: UpnpResult = UpnpResult {
        ord: 17i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PROTOCOL")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PROTOCOL`"]
    pub const INVALID_PROTOCOL: UpnpResult = UpnpResult {
        ord: 18i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_DURATION")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_DURATION`"]
    pub const INVALID_DURATION: UpnpResult = UpnpResult {
        ord: 19i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_ARGS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_ARGS`"]
    pub const INVALID_ARGS: UpnpResult = UpnpResult {
        ord: 20i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_RESPONSE")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_RESPONSE`"]
    pub const INVALID_RESPONSE: UpnpResult = UpnpResult {
        ord: 21i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PARAM")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PARAM`"]
    pub const INVALID_PARAM: UpnpResult = UpnpResult {
        ord: 22i32
    };
    #[doc(alias = "UPNP_RESULT_HTTP_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_HTTP_ERROR`"]
    pub const HTTP_ERROR: UpnpResult = UpnpResult {
        ord: 23i32
    };
    #[doc(alias = "UPNP_RESULT_SOCKET_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SOCKET_ERROR`"]
    pub const SOCKET_ERROR: UpnpResult = UpnpResult {
        ord: 24i32
    };
    #[doc(alias = "UPNP_RESULT_MEM_ALLOC_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_MEM_ALLOC_ERROR`"]
    pub const MEM_ALLOC_ERROR: UpnpResult = UpnpResult {
        ord: 25i32
    };
    #[doc(alias = "UPNP_RESULT_NO_GATEWAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_GATEWAY`"]
    pub const NO_GATEWAY: UpnpResult = UpnpResult {
        ord: 26i32
    };
    #[doc(alias = "UPNP_RESULT_NO_DEVICES")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_DEVICES`"]
    pub const NO_DEVICES: UpnpResult = UpnpResult {
        ord: 27i32
    };
    #[doc(alias = "UPNP_RESULT_UNKNOWN_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_UNKNOWN_ERROR`"]
    pub const UNKNOWN_ERROR: UpnpResult = UpnpResult {
        ord: 28i32
    };
    
}
impl std::fmt::Debug for UpnpResult {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpnpResult") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpnpResult {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 => Some(Self {
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
            Self::SUCCESS => "SUCCESS", Self::NOT_AUTHORIZED => "NOT_AUTHORIZED", Self::PORT_MAPPING_NOT_FOUND => "PORT_MAPPING_NOT_FOUND", Self::INCONSISTENT_PARAMETERS => "INCONSISTENT_PARAMETERS", Self::NO_SUCH_ENTRY_IN_ARRAY => "NO_SUCH_ENTRY_IN_ARRAY", Self::ACTION_FAILED => "ACTION_FAILED", Self::SRC_IP_WILDCARD_NOT_PERMITTED => "SRC_IP_WILDCARD_NOT_PERMITTED", Self::EXT_PORT_WILDCARD_NOT_PERMITTED => "EXT_PORT_WILDCARD_NOT_PERMITTED", Self::INT_PORT_WILDCARD_NOT_PERMITTED => "INT_PORT_WILDCARD_NOT_PERMITTED", Self::REMOTE_HOST_MUST_BE_WILDCARD => "REMOTE_HOST_MUST_BE_WILDCARD", Self::EXT_PORT_MUST_BE_WILDCARD => "EXT_PORT_MUST_BE_WILDCARD", Self::NO_PORT_MAPS_AVAILABLE => "NO_PORT_MAPS_AVAILABLE", Self::CONFLICT_WITH_OTHER_MECHANISM => "CONFLICT_WITH_OTHER_MECHANISM", Self::CONFLICT_WITH_OTHER_MAPPING => "CONFLICT_WITH_OTHER_MAPPING", Self::SAME_PORT_VALUES_REQUIRED => "SAME_PORT_VALUES_REQUIRED", Self::ONLY_PERMANENT_LEASE_SUPPORTED => "ONLY_PERMANENT_LEASE_SUPPORTED", Self::INVALID_GATEWAY => "INVALID_GATEWAY", Self::INVALID_PORT => "INVALID_PORT", Self::INVALID_PROTOCOL => "INVALID_PROTOCOL", Self::INVALID_DURATION => "INVALID_DURATION", Self::INVALID_ARGS => "INVALID_ARGS", Self::INVALID_RESPONSE => "INVALID_RESPONSE", Self::INVALID_PARAM => "INVALID_PARAM", Self::HTTP_ERROR => "HTTP_ERROR", Self::SOCKET_ERROR => "SOCKET_ERROR", Self::MEM_ALLOC_ERROR => "MEM_ALLOC_ERROR", Self::NO_GATEWAY => "NO_GATEWAY", Self::NO_DEVICES => "NO_DEVICES", Self::UNKNOWN_ERROR => "UNKNOWN_ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpnpResult::SUCCESS, UpnpResult::NOT_AUTHORIZED, UpnpResult::PORT_MAPPING_NOT_FOUND, UpnpResult::INCONSISTENT_PARAMETERS, UpnpResult::NO_SUCH_ENTRY_IN_ARRAY, UpnpResult::ACTION_FAILED, UpnpResult::SRC_IP_WILDCARD_NOT_PERMITTED, UpnpResult::EXT_PORT_WILDCARD_NOT_PERMITTED, UpnpResult::INT_PORT_WILDCARD_NOT_PERMITTED, UpnpResult::REMOTE_HOST_MUST_BE_WILDCARD, UpnpResult::EXT_PORT_MUST_BE_WILDCARD, UpnpResult::NO_PORT_MAPS_AVAILABLE, UpnpResult::CONFLICT_WITH_OTHER_MECHANISM, UpnpResult::CONFLICT_WITH_OTHER_MAPPING, UpnpResult::SAME_PORT_VALUES_REQUIRED, UpnpResult::ONLY_PERMANENT_LEASE_SUPPORTED, UpnpResult::INVALID_GATEWAY, UpnpResult::INVALID_PORT, UpnpResult::INVALID_PROTOCOL, UpnpResult::INVALID_DURATION, UpnpResult::INVALID_ARGS, UpnpResult::INVALID_RESPONSE, UpnpResult::INVALID_PARAM, UpnpResult::HTTP_ERROR, UpnpResult::SOCKET_ERROR, UpnpResult::MEM_ALLOC_ERROR, UpnpResult::NO_GATEWAY, UpnpResult::NO_DEVICES, UpnpResult::UNKNOWN_ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpnpResult >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUCCESS", "UPNP_RESULT_SUCCESS", UpnpResult::SUCCESS), crate::meta::inspect::EnumConstant::new("NOT_AUTHORIZED", "UPNP_RESULT_NOT_AUTHORIZED", UpnpResult::NOT_AUTHORIZED), crate::meta::inspect::EnumConstant::new("PORT_MAPPING_NOT_FOUND", "UPNP_RESULT_PORT_MAPPING_NOT_FOUND", UpnpResult::PORT_MAPPING_NOT_FOUND), crate::meta::inspect::EnumConstant::new("INCONSISTENT_PARAMETERS", "UPNP_RESULT_INCONSISTENT_PARAMETERS", UpnpResult::INCONSISTENT_PARAMETERS), crate::meta::inspect::EnumConstant::new("NO_SUCH_ENTRY_IN_ARRAY", "UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY", UpnpResult::NO_SUCH_ENTRY_IN_ARRAY), crate::meta::inspect::EnumConstant::new("ACTION_FAILED", "UPNP_RESULT_ACTION_FAILED", UpnpResult::ACTION_FAILED), crate::meta::inspect::EnumConstant::new("SRC_IP_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED", UpnpResult::SRC_IP_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("EXT_PORT_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED", UpnpResult::EXT_PORT_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("INT_PORT_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED", UpnpResult::INT_PORT_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("REMOTE_HOST_MUST_BE_WILDCARD", "UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD", UpnpResult::REMOTE_HOST_MUST_BE_WILDCARD), crate::meta::inspect::EnumConstant::new("EXT_PORT_MUST_BE_WILDCARD", "UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD", UpnpResult::EXT_PORT_MUST_BE_WILDCARD), crate::meta::inspect::EnumConstant::new("NO_PORT_MAPS_AVAILABLE", "UPNP_RESULT_NO_PORT_MAPS_AVAILABLE", UpnpResult::NO_PORT_MAPS_AVAILABLE), crate::meta::inspect::EnumConstant::new("CONFLICT_WITH_OTHER_MECHANISM", "UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM", UpnpResult::CONFLICT_WITH_OTHER_MECHANISM), crate::meta::inspect::EnumConstant::new("CONFLICT_WITH_OTHER_MAPPING", "UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING", UpnpResult::CONFLICT_WITH_OTHER_MAPPING), crate::meta::inspect::EnumConstant::new("SAME_PORT_VALUES_REQUIRED", "UPNP_RESULT_SAME_PORT_VALUES_REQUIRED", UpnpResult::SAME_PORT_VALUES_REQUIRED), crate::meta::inspect::EnumConstant::new("ONLY_PERMANENT_LEASE_SUPPORTED", "UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED", UpnpResult::ONLY_PERMANENT_LEASE_SUPPORTED), crate::meta::inspect::EnumConstant::new("INVALID_GATEWAY", "UPNP_RESULT_INVALID_GATEWAY", UpnpResult::INVALID_GATEWAY), crate::meta::inspect::EnumConstant::new("INVALID_PORT", "UPNP_RESULT_INVALID_PORT", UpnpResult::INVALID_PORT), crate::meta::inspect::EnumConstant::new("INVALID_PROTOCOL", "UPNP_RESULT_INVALID_PROTOCOL", UpnpResult::INVALID_PROTOCOL), crate::meta::inspect::EnumConstant::new("INVALID_DURATION", "UPNP_RESULT_INVALID_DURATION", UpnpResult::INVALID_DURATION), crate::meta::inspect::EnumConstant::new("INVALID_ARGS", "UPNP_RESULT_INVALID_ARGS", UpnpResult::INVALID_ARGS), crate::meta::inspect::EnumConstant::new("INVALID_RESPONSE", "UPNP_RESULT_INVALID_RESPONSE", UpnpResult::INVALID_RESPONSE), crate::meta::inspect::EnumConstant::new("INVALID_PARAM", "UPNP_RESULT_INVALID_PARAM", UpnpResult::INVALID_PARAM), crate::meta::inspect::EnumConstant::new("HTTP_ERROR", "UPNP_RESULT_HTTP_ERROR", UpnpResult::HTTP_ERROR), crate::meta::inspect::EnumConstant::new("SOCKET_ERROR", "UPNP_RESULT_SOCKET_ERROR", UpnpResult::SOCKET_ERROR), crate::meta::inspect::EnumConstant::new("MEM_ALLOC_ERROR", "UPNP_RESULT_MEM_ALLOC_ERROR", UpnpResult::MEM_ALLOC_ERROR), crate::meta::inspect::EnumConstant::new("NO_GATEWAY", "UPNP_RESULT_NO_GATEWAY", UpnpResult::NO_GATEWAY), crate::meta::inspect::EnumConstant::new("NO_DEVICES", "UPNP_RESULT_NO_DEVICES", UpnpResult::NO_DEVICES), crate::meta::inspect::EnumConstant::new("UNKNOWN_ERROR", "UPNP_RESULT_UNKNOWN_ERROR", UpnpResult::UNKNOWN_ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for UpnpResult {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Upnp Result Success", 0i64), EnumeratorShape::new_int("Upnp Result Not Authorized", 1i64), EnumeratorShape::new_int("Upnp Result Port Mapping Not Found", 2i64), EnumeratorShape::new_int("Upnp Result Inconsistent Parameters", 3i64), EnumeratorShape::new_int("Upnp Result No Such Entry In Array", 4i64), EnumeratorShape::new_int("Upnp Result Action Failed", 5i64), EnumeratorShape::new_int("Upnp Result Src Ip Wildcard Not Permitted", 6i64), EnumeratorShape::new_int("Upnp Result Ext Port Wildcard Not Permitted", 7i64), EnumeratorShape::new_int("Upnp Result Int Port Wildcard Not Permitted", 8i64), EnumeratorShape::new_int("Upnp Result Remote Host Must Be Wildcard", 9i64), EnumeratorShape::new_int("Upnp Result Ext Port Must Be Wildcard", 10i64), EnumeratorShape::new_int("Upnp Result No Port Maps Available", 11i64), EnumeratorShape::new_int("Upnp Result Conflict With Other Mechanism", 12i64), EnumeratorShape::new_int("Upnp Result Conflict With Other Mapping", 13i64), EnumeratorShape::new_int("Upnp Result Same Port Values Required", 14i64), EnumeratorShape::new_int("Upnp Result Only Permanent Lease Supported", 15i64), EnumeratorShape::new_int("Upnp Result Invalid Gateway", 16i64), EnumeratorShape::new_int("Upnp Result Invalid Port", 17i64), EnumeratorShape::new_int("Upnp Result Invalid Protocol", 18i64), EnumeratorShape::new_int("Upnp Result Invalid Duration", 19i64), EnumeratorShape::new_int("Upnp Result Invalid Args", 20i64), EnumeratorShape::new_int("Upnp Result Invalid Response", 21i64), EnumeratorShape::new_int("Upnp Result Invalid Param", 22i64), EnumeratorShape::new_int("Upnp Result Http Error", 23i64), EnumeratorShape::new_int("Upnp Result Socket Error", 24i64), EnumeratorShape::new_int("Upnp Result Mem Alloc Error", 25i64), EnumeratorShape::new_int("Upnp Result No Gateway", 26i64), EnumeratorShape::new_int("Upnp Result No Devices", 27i64), EnumeratorShape::new_int("Upnp Result Unknown Error", 28i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("UPNP.UPNPResult")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for UpnpResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpnpResult {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for UpnpResult {
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
impl crate::registry::property::Export for UpnpResult {
    
}
impl crate::meta::Element for UpnpResult {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Upnp;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Upnp {
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