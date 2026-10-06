#![doc = "Sidecar module for class [`Time`][crate::classes::Time].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Time` enums](https://docs.godotengine.org/en/stable/classes/class_time.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Time`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`time`][crate::classes::time]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Time`](https://docs.godotengine.org/en/stable/classes/class_time.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nThe Time singleton allows converting time between various formats and also getting time information from the system.\n\nThis class conforms with as many of the ISO 8601 standards as possible. All dates follow the Proleptic Gregorian calendar. As such, the day before `1582-10-15` is `1582-10-14`, not `1582-10-04`. The year before 1 AD (aka 1 BC) is number `0`, with the year before that (2 BC) being `-1`, etc.\n\nConversion methods assume \"the same timezone\", and do not handle timezone conversions or DST automatically. Leap seconds are also not handled, they must be done manually if desired. Suffixes such as \"Z\" are not handled, you need to strip them away manually.\n\nWhen getting time information from the system, the time can either be in the local timezone or UTC depending on the `utc` parameter. However, the [`get_unix_time_from_system`][`crate::classes::Time::get_unix_time_from_system`] method always uses UTC as it returns the seconds passed since the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time).\n\n**Important:** The `_from_system` methods use the system clock that the user can manually set. **Never use** this method for precise time calculation since its results are subject to automatic adjustments by the user or the operating system. **Always use** [`get_ticks_usec`][`crate::classes::Time::get_ticks_usec`] or [`get_ticks_msec`][`crate::classes::Time::get_ticks_msec`] for precise time calculation instead, since they are guaranteed to be monotonic (i.e. never decrease)."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Time {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Time {
        #[doc = "Converts the given Unix timestamp to a dictionary of keys: `year`, `month`, `day`, `weekday`, `hour`, `minute`, and `second`.\n\nThe returned Dictionary's values will be the same as the [`get_datetime_dict_from_system`][`crate::classes::Time::get_datetime_dict_from_system`] if the Unix timestamp is the current time, with the exception of Daylight Savings Time as it cannot be determined from the epoch."]
        pub fn get_datetime_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given Unix timestamp to a dictionary of keys: `year`, `month`, `day`, and `weekday`."]
        pub fn get_date_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given time to a dictionary of keys: `hour`, `minute`, and `second`."]
        pub fn get_time_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given Unix timestamp to an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        pub(crate) fn get_datetime_string_from_unix_time_full(&self, unix_time_val: i64, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (i64, bool,);
            let args = (unix_time_val, use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_datetime_string_from_unix_time_ex`][Self::get_datetime_string_from_unix_time_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Converts the given Unix timestamp to an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        #[inline]
        pub fn get_datetime_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            self.get_datetime_string_from_unix_time_ex(unix_time_val,) . done()
        }
        #[doc = "Converts the given Unix timestamp to an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        #[inline]
        pub fn get_datetime_string_from_unix_time_ex < 'ex > (&'ex self, unix_time_val: i64,) -> ExGetDatetimeStringFromUnixTime < 'ex > {
            ExGetDatetimeStringFromUnixTime::new(self, unix_time_val,)
        }
        #[doc = "Converts the given Unix timestamp to an ISO 8601 date string (YYYY-MM-DD)."]
        pub fn get_date_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given Unix timestamp to an ISO 8601 time string (HH:MM:SS)."]
        pub fn get_time_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS) to a dictionary of keys: `year`, `month`, `day`, `weekday`, `hour`, `minute`, and `second`.\n\nIf `weekday` is `false`, then the `weekday` entry is excluded (the calculation is relatively expensive).\n\n**Note:** Any decimal fraction in the time string will be ignored silently."]
        pub fn get_datetime_dict_from_datetime_string(&self, datetime: impl AsArg < GString >, weekday: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (datetime.into_arg(), weekday,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_datetime_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given dictionary of keys to an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nThe given dictionary can be populated with the following keys: `year`, `month`, `day`, `hour`, `minute`, and `second`. Any other entries (including `dst`) are ignored.\n\nIf the dictionary is empty, `0` is returned. If some keys are omitted, they default to the equivalent values for the Unix epoch timestamp 0 (1970-01-01 at 00:00:00).\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        pub fn get_datetime_string_from_datetime_dict(&self, datetime: &AnyDictionary, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >, bool,);
            let args = (RefArg::new(datetime), use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_datetime_dict", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts a dictionary of time values to a Unix timestamp.\n\nThe given dictionary can be populated with the following keys: `year`, `month`, `day`, `hour`, `minute`, and `second`. Any other entries (including `dst`) are ignored.\n\nIf the dictionary is empty, `0` is returned. If some keys are omitted, they default to the equivalent values for the Unix epoch timestamp 0 (1970-01-01 at 00:00:00).\n\nYou can pass the output from [`get_datetime_dict_from_unix_time`][`crate::classes::Time::get_datetime_dict_from_unix_time`] directly into this function and get the same as what was put in.\n\n**Note:** Unix timestamps are often in UTC. This method does not do any timezone conversion, so the timestamp will be in the same timezone as the given datetime dictionary."]
        pub fn get_unix_time_from_datetime_dict(&self, datetime: &AnyDictionary,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(datetime),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_dict", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given ISO 8601 date and/or time string to a Unix timestamp. The string can contain a date only, a time only, or both.\n\n**Note:** Unix timestamps are often in UTC. This method does not do any timezone conversion, so the timestamp will be in the same timezone as the given datetime string.\n\n**Note:** Any decimal fraction in the time string will be ignored silently."]
        pub fn get_unix_time_from_datetime_string(&self, datetime: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (datetime.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Converts the given timezone offset in minutes to a timezone offset string. For example, -480 returns \"-08:00\", 345 returns \"+05:45\", and 0 returns \"+00:00\"."]
        pub fn get_offset_string_from_offset_minutes(&self, offset_minutes: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (offset_minutes,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_offset_string_from_offset_minutes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, `weekday`, `hour`, `minute`, `second`, and `dst` (Daylight Savings Time)."]
        pub(crate) fn get_datetime_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_datetime_dict_from_system_ex`][Self::get_datetime_dict_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, `weekday`, `hour`, `minute`, `second`, and `dst` (Daylight Savings Time)."]
        #[inline]
        pub fn get_datetime_dict_from_system(&self,) -> VarDictionary {
            self.get_datetime_dict_from_system_ex() . done()
        }
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, `weekday`, `hour`, `minute`, `second`, and `dst` (Daylight Savings Time)."]
        #[inline]
        pub fn get_datetime_dict_from_system_ex < 'ex > (&'ex self,) -> ExGetDatetimeDictFromSystem < 'ex > {
            ExGetDatetimeDictFromSystem::new(self,)
        }
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, and `weekday`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        pub(crate) fn get_date_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_date_dict_from_system_ex`][Self::get_date_dict_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, and `weekday`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_date_dict_from_system(&self,) -> VarDictionary {
            self.get_date_dict_from_system_ex() . done()
        }
        #[doc = "Returns the current date as a dictionary of keys: `year`, `month`, `day`, and `weekday`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_date_dict_from_system_ex < 'ex > (&'ex self,) -> ExGetDateDictFromSystem < 'ex > {
            ExGetDateDictFromSystem::new(self,)
        }
        #[doc = "Returns the current time as a dictionary of keys: `hour`, `minute`, and `second`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        pub(crate) fn get_time_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_time_dict_from_system_ex`][Self::get_time_dict_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current time as a dictionary of keys: `hour`, `minute`, and `second`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_time_dict_from_system(&self,) -> VarDictionary {
            self.get_time_dict_from_system_ex() . done()
        }
        #[doc = "Returns the current time as a dictionary of keys: `hour`, `minute`, and `second`.\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_time_dict_from_system_ex < 'ex > (&'ex self,) -> ExGetTimeDictFromSystem < 'ex > {
            ExGetTimeDictFromSystem::new(self,)
        }
        #[doc = "Returns the current date and time as an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC.\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        pub(crate) fn get_datetime_string_from_system_full(&self, utc: bool, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool, bool,);
            let args = (utc, use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_datetime_string_from_system_ex`][Self::get_datetime_string_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current date and time as an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC.\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        #[inline]
        pub fn get_datetime_string_from_system(&self,) -> GString {
            self.get_datetime_string_from_system_ex() . done()
        }
        #[doc = "Returns the current date and time as an ISO 8601 date and time string (YYYY-MM-DDTHH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC.\n\nIf `use_space` is `true`, the date and time bits are separated by an empty space character instead of the letter T."]
        #[inline]
        pub fn get_datetime_string_from_system_ex < 'ex > (&'ex self,) -> ExGetDatetimeStringFromSystem < 'ex > {
            ExGetDatetimeStringFromSystem::new(self,)
        }
        #[doc = "Returns the current date as an ISO 8601 date string (YYYY-MM-DD).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        pub(crate) fn get_date_string_from_system_full(&self, utc: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_date_string_from_system_ex`][Self::get_date_string_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current date as an ISO 8601 date string (YYYY-MM-DD).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_date_string_from_system(&self,) -> GString {
            self.get_date_string_from_system_ex() . done()
        }
        #[doc = "Returns the current date as an ISO 8601 date string (YYYY-MM-DD).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_date_string_from_system_ex < 'ex > (&'ex self,) -> ExGetDateStringFromSystem < 'ex > {
            ExGetDateStringFromSystem::new(self,)
        }
        #[doc = "Returns the current time as an ISO 8601 time string (HH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        pub(crate) fn get_time_string_from_system_full(&self, utc: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_time_string_from_system_ex`][Self::get_time_string_from_system_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the current time as an ISO 8601 time string (HH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_time_string_from_system(&self,) -> GString {
            self.get_time_string_from_system_ex() . done()
        }
        #[doc = "Returns the current time as an ISO 8601 time string (HH:MM:SS).\n\nThe returned values are in the system's local time when `utc` is `false`, otherwise they are in UTC."]
        #[inline]
        pub fn get_time_string_from_system_ex < 'ex > (&'ex self,) -> ExGetTimeStringFromSystem < 'ex > {
            ExGetTimeStringFromSystem::new(self,)
        }
        #[doc = "Returns the current time zone as a dictionary of keys: `bias` and `name`.\n\n- `bias` is the offset from UTC in minutes, since not all time zones are multiples of an hour from UTC.\n\n- `name` is the localized name of the time zone, according to the OS locale settings of the current user."]
        pub fn get_time_zone_from_system(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_zone_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current Unix timestamp in seconds based on the system time in UTC. This method is implemented by the operating system and always returns the time in UTC. The Unix timestamp is the number of seconds passed since 1970-01-01 at 00:00:00, the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time).\n\n**Note:** Unlike other methods that use integer timestamps, this method returns the timestamp as a `float` for sub-second precision."]
        pub fn get_unix_time_from_system(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of time passed in milliseconds since the engine started.\n\nWill always be positive or 0 and uses a 64-bit value (it will wrap after roughly 500 million years)."]
        pub fn get_ticks_msec(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_ticks_msec", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the amount of time passed in microseconds since the engine started.\n\nWill always be positive or 0 and uses a 64-bit value (it will wrap after roughly half a million years)."]
        pub fn get_ticks_usec(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_ticks_usec", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Time {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Time"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Time {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Time {
        
    }
    impl crate::obj::Singleton for Time {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"Time"))
            }
        }
    }
    impl std::ops::Deref for Time {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Time {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Time__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Time` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_unix_time_ex`][super::Time::get_datetime_string_from_unix_time_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromUnixTime < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, unix_time_val: i64, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDatetimeStringFromUnixTime < 'ex > {
    fn new(surround_object: &'ex re_export::Time, unix_time_val: i64,) -> Self {
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, unix_time_val: unix_time_val, use_space: use_space,
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, unix_time_val, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_unix_time_full(surround_object, unix_time_val, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_dict_from_system_ex`][super::Time::get_datetime_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeDictFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDatetimeDictFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_datetime_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_dict_from_system_ex`][super::Time::get_date_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDateDictFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDateDictFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_dict_from_system_ex`][super::Time::get_time_dict_from_system_ex]."]
#[must_use]
pub struct ExGetTimeDictFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetTimeDictFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_system_ex`][super::Time::get_datetime_string_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDatetimeStringFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc, use_space: use_space,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_system_full(surround_object, utc, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_string_from_system_ex`][super::Time::get_date_string_from_system_ex]."]
#[must_use]
pub struct ExGetDateStringFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetDateStringFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_string_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_string_from_system_ex`][super::Time::get_time_string_from_system_ex]."]
#[must_use]
pub struct ExGetTimeStringFromSystem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetTimeStringFromSystem < 'ex > {
    fn new(surround_object: &'ex re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_string_from_system_full(surround_object, utc,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Month {
    ord: i32
}
impl Month {
    #[doc(alias = "MONTH_JANUARY")]
    #[doc = "Godot enumerator name: `MONTH_JANUARY`"]
    pub const JANUARY: Month = Month {
        ord: 1i32
    };
    #[doc(alias = "MONTH_FEBRUARY")]
    #[doc = "Godot enumerator name: `MONTH_FEBRUARY`"]
    pub const FEBRUARY: Month = Month {
        ord: 2i32
    };
    #[doc(alias = "MONTH_MARCH")]
    #[doc = "Godot enumerator name: `MONTH_MARCH`"]
    pub const MARCH: Month = Month {
        ord: 3i32
    };
    #[doc(alias = "MONTH_APRIL")]
    #[doc = "Godot enumerator name: `MONTH_APRIL`"]
    pub const APRIL: Month = Month {
        ord: 4i32
    };
    #[doc(alias = "MONTH_MAY")]
    #[doc = "Godot enumerator name: `MONTH_MAY`"]
    pub const MAY: Month = Month {
        ord: 5i32
    };
    #[doc(alias = "MONTH_JUNE")]
    #[doc = "Godot enumerator name: `MONTH_JUNE`"]
    pub const JUNE: Month = Month {
        ord: 6i32
    };
    #[doc(alias = "MONTH_JULY")]
    #[doc = "Godot enumerator name: `MONTH_JULY`"]
    pub const JULY: Month = Month {
        ord: 7i32
    };
    #[doc(alias = "MONTH_AUGUST")]
    #[doc = "Godot enumerator name: `MONTH_AUGUST`"]
    pub const AUGUST: Month = Month {
        ord: 8i32
    };
    #[doc(alias = "MONTH_SEPTEMBER")]
    #[doc = "Godot enumerator name: `MONTH_SEPTEMBER`"]
    pub const SEPTEMBER: Month = Month {
        ord: 9i32
    };
    #[doc(alias = "MONTH_OCTOBER")]
    #[doc = "Godot enumerator name: `MONTH_OCTOBER`"]
    pub const OCTOBER: Month = Month {
        ord: 10i32
    };
    #[doc(alias = "MONTH_NOVEMBER")]
    #[doc = "Godot enumerator name: `MONTH_NOVEMBER`"]
    pub const NOVEMBER: Month = Month {
        ord: 11i32
    };
    #[doc(alias = "MONTH_DECEMBER")]
    #[doc = "Godot enumerator name: `MONTH_DECEMBER`"]
    pub const DECEMBER: Month = Month {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Month") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Month {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::JANUARY => "JANUARY", Self::FEBRUARY => "FEBRUARY", Self::MARCH => "MARCH", Self::APRIL => "APRIL", Self::MAY => "MAY", Self::JUNE => "JUNE", Self::JULY => "JULY", Self::AUGUST => "AUGUST", Self::SEPTEMBER => "SEPTEMBER", Self::OCTOBER => "OCTOBER", Self::NOVEMBER => "NOVEMBER", Self::DECEMBER => "DECEMBER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Month::JANUARY, Month::FEBRUARY, Month::MARCH, Month::APRIL, Month::MAY, Month::JUNE, Month::JULY, Month::AUGUST, Month::SEPTEMBER, Month::OCTOBER, Month::NOVEMBER, Month::DECEMBER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Month >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("JANUARY", "MONTH_JANUARY", Month::JANUARY), crate::meta::inspect::EnumConstant::new("FEBRUARY", "MONTH_FEBRUARY", Month::FEBRUARY), crate::meta::inspect::EnumConstant::new("MARCH", "MONTH_MARCH", Month::MARCH), crate::meta::inspect::EnumConstant::new("APRIL", "MONTH_APRIL", Month::APRIL), crate::meta::inspect::EnumConstant::new("MAY", "MONTH_MAY", Month::MAY), crate::meta::inspect::EnumConstant::new("JUNE", "MONTH_JUNE", Month::JUNE), crate::meta::inspect::EnumConstant::new("JULY", "MONTH_JULY", Month::JULY), crate::meta::inspect::EnumConstant::new("AUGUST", "MONTH_AUGUST", Month::AUGUST), crate::meta::inspect::EnumConstant::new("SEPTEMBER", "MONTH_SEPTEMBER", Month::SEPTEMBER), crate::meta::inspect::EnumConstant::new("OCTOBER", "MONTH_OCTOBER", Month::OCTOBER), crate::meta::inspect::EnumConstant::new("NOVEMBER", "MONTH_NOVEMBER", Month::NOVEMBER), crate::meta::inspect::EnumConstant::new("DECEMBER", "MONTH_DECEMBER", Month::DECEMBER)]
        }
    }
}
impl crate::meta::GodotConvert for Month {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Month January", 1i64), EnumeratorShape::new_int("Month February", 2i64), EnumeratorShape::new_int("Month March", 3i64), EnumeratorShape::new_int("Month April", 4i64), EnumeratorShape::new_int("Month May", 5i64), EnumeratorShape::new_int("Month June", 6i64), EnumeratorShape::new_int("Month July", 7i64), EnumeratorShape::new_int("Month August", 8i64), EnumeratorShape::new_int("Month September", 9i64), EnumeratorShape::new_int("Month October", 10i64), EnumeratorShape::new_int("Month November", 11i64), EnumeratorShape::new_int("Month December", 12i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Time.Month")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Month {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Month {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Month {
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
impl crate::registry::property::Export for Month {
    
}
impl crate::meta::Element for Month {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Weekday {
    ord: i32
}
impl Weekday {
    #[doc(alias = "WEEKDAY_SUNDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SUNDAY`"]
    pub const SUNDAY: Weekday = Weekday {
        ord: 0i32
    };
    #[doc(alias = "WEEKDAY_MONDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_MONDAY`"]
    pub const MONDAY: Weekday = Weekday {
        ord: 1i32
    };
    #[doc(alias = "WEEKDAY_TUESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_TUESDAY`"]
    pub const TUESDAY: Weekday = Weekday {
        ord: 2i32
    };
    #[doc(alias = "WEEKDAY_WEDNESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_WEDNESDAY`"]
    pub const WEDNESDAY: Weekday = Weekday {
        ord: 3i32
    };
    #[doc(alias = "WEEKDAY_THURSDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_THURSDAY`"]
    pub const THURSDAY: Weekday = Weekday {
        ord: 4i32
    };
    #[doc(alias = "WEEKDAY_FRIDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_FRIDAY`"]
    pub const FRIDAY: Weekday = Weekday {
        ord: 5i32
    };
    #[doc(alias = "WEEKDAY_SATURDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SATURDAY`"]
    pub const SATURDAY: Weekday = Weekday {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Weekday") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Weekday {
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
            Self::SUNDAY => "SUNDAY", Self::MONDAY => "MONDAY", Self::TUESDAY => "TUESDAY", Self::WEDNESDAY => "WEDNESDAY", Self::THURSDAY => "THURSDAY", Self::FRIDAY => "FRIDAY", Self::SATURDAY => "SATURDAY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Weekday::SUNDAY, Weekday::MONDAY, Weekday::TUESDAY, Weekday::WEDNESDAY, Weekday::THURSDAY, Weekday::FRIDAY, Weekday::SATURDAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Weekday >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUNDAY", "WEEKDAY_SUNDAY", Weekday::SUNDAY), crate::meta::inspect::EnumConstant::new("MONDAY", "WEEKDAY_MONDAY", Weekday::MONDAY), crate::meta::inspect::EnumConstant::new("TUESDAY", "WEEKDAY_TUESDAY", Weekday::TUESDAY), crate::meta::inspect::EnumConstant::new("WEDNESDAY", "WEEKDAY_WEDNESDAY", Weekday::WEDNESDAY), crate::meta::inspect::EnumConstant::new("THURSDAY", "WEEKDAY_THURSDAY", Weekday::THURSDAY), crate::meta::inspect::EnumConstant::new("FRIDAY", "WEEKDAY_FRIDAY", Weekday::FRIDAY), crate::meta::inspect::EnumConstant::new("SATURDAY", "WEEKDAY_SATURDAY", Weekday::SATURDAY)]
        }
    }
}
impl crate::meta::GodotConvert for Weekday {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Weekday Sunday", 0i64), EnumeratorShape::new_int("Weekday Monday", 1i64), EnumeratorShape::new_int("Weekday Tuesday", 2i64), EnumeratorShape::new_int("Weekday Wednesday", 3i64), EnumeratorShape::new_int("Weekday Thursday", 4i64), EnumeratorShape::new_int("Weekday Friday", 5i64), EnumeratorShape::new_int("Weekday Saturday", 6i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Time.Weekday")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Weekday {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Weekday {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Weekday {
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
impl crate::registry::property::Export for Weekday {
    
}
impl crate::meta::Element for Weekday {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Time;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Time {
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