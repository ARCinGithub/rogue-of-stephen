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
pub(super) mod re_export {
    use super::*;
    #[doc(hidden)]
    #[repr(transparent)]
    pub struct InnerColor < 'inner > {
        pub(super) _outer_lifetime: std::marker::PhantomData < &'inner() >, pub(super) sys_ptr: sys::GDExtensionTypePtr,
    }
}
impl < 'inner > re_export::InnerColor < 'inner > {
    pub fn from_outer(outer: &Color) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    #[doc = "Returns the color converted to a 32-bit integer in ARGB format (each component is 8 bits). ARGB is more compatible with DirectX.\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_argb32()) # Prints 4294934323\n```\n"]
    pub fn to_argb32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(457usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_argb32", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to a 32-bit integer in ABGR format (each component is 8 bits). ABGR is the reversed version of the default RGBA format.\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_abgr32()) # Prints 4281565439\n```\n"]
    pub fn to_abgr32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(458usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_abgr32", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to a 32-bit integer in RGBA format (each component is 8 bits). RGBA is Godot's default format. This method is the inverse of \\[method hex].\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_rgba32()) # Prints 4286526463\n```\n"]
    pub fn to_rgba32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(459usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_rgba32", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to a 64-bit integer in ARGB format (each component is 16 bits). ARGB is more compatible with DirectX.\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_argb64()) # Prints -2147470541\n```\n"]
    pub fn to_argb64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(460usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_argb64", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to a 64-bit integer in ABGR format (each component is 16 bits). ABGR is the reversed version of the default RGBA format.\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_abgr64()) # Prints -225178692812801\n```\n"]
    pub fn to_abgr64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(461usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_abgr64", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to a 64-bit integer in RGBA format (each component is 16 bits). RGBA is Godot's default format. This method is the inverse of \\[method hex64].\n\n\n```gdscript\nvar color = Color(1, 0.5, 0.2)\nprint(color.to_rgba64()) # Prints -140736629309441\n```\n"]
    pub fn to_rgba64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(462usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_rgba64", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color converted to an HTML hexadecimal color [`String`][crate::builtin::GString] in RGBA format, without the hash (`#`) prefix.\n\nSetting `with_alpha` to `false`, excludes alpha from the hexadecimal string, using RGB format instead of RGBA format.\n\n\n```gdscript\nvar white = Color(1, 1, 1, 0.5)\nvar with_alpha = white.to_html() # Returns \"ffffff7f\"\nvar without_alpha = white.to_html(false) # Returns \"ffffff\"\n```\n"]
    pub fn to_html(&self, with_alpha: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool,);
        let args = (with_alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(463usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_html", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new color with all components clamped between the components of `min` and `max`, by running [`clamp`][`crate::global::clamp`] on each component."]
    pub fn clamp(&self, min: Color, max: Color,) -> Color {
        type CallRet = Color;
        type CallParams = (Color, Color,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(464usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "clamp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the color with its \\[member r], \\[member g], and \\[member b] components inverted (`(1 - r, 1 - g, 1 - b, a)`).\n\n\n```gdscript\nvar black = Color.WHITE.inverted()\nvar color = Color(0.3, 0.4, 0.9)\nvar inverted_color = color.inverted() # Equivalent to `Color(0.7, 0.6, 0.1)`\n```\n"]
    pub fn inverted(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(465usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "inverted", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the linear interpolation between this color's components and `to`'s components. The interpolation factor `weight` should be between 0.0 and 1.0 (inclusive). See also [`lerp`][`crate::global::lerp`].\n\n\n```gdscript\nvar red = Color(1.0, 0.0, 0.0)\nvar aqua = Color(0.0, 1.0, 0.8)\n\nred.lerp(aqua, 0.2) # Returns Color(0.8, 0.2, 0.16)\nred.lerp(aqua, 0.5) # Returns Color(0.5, 0.5, 0.4)\nred.lerp(aqua, 1.0) # Returns Color(0.0, 1.0, 0.8)\n```\n"]
    pub fn lerp(&self, to: Color, weight: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (Color, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(466usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "lerp", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new color resulting from making this color lighter by the specified `amount`, which should be a ratio from 0.0 to 1.0. See also \\[method darkened].\n\n\n```gdscript\nvar green = Color(0.0, 1.0, 0.0)\nvar light_green = green.lightened(0.2) # 20% lighter than regular green\n```\n"]
    pub fn lightened(&self, amount: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(467usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "lightened", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new color resulting from making this color darker by the specified `amount` (ratio from 0.0 to 1.0). See also \\[method lightened].\n\n\n```gdscript\nvar green = Color(0.0, 1.0, 0.0)\nvar darkgreen = green.darkened(0.2) # 20% darker than regular green\n```\n"]
    pub fn darkened(&self, amount: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(468usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "darkened", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a new color resulting from overlaying this color over the given color. In a painting program, you can imagine it as the `over` color painted over this color (including alpha).\n\n\n```gdscript\nvar bg = Color(0.0, 1.0, 0.0, 0.5) # Green with alpha of 50%\nvar fg = Color(1.0, 0.0, 0.0, 0.5) # Red with alpha of 50%\nvar blended_color = bg.blend(fg) # Brown with alpha of 75%\n```\n"]
    pub fn blend(&self, over: Color,) -> Color {
        type CallRet = Color;
        type CallParams = (Color,);
        let args = (over,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(469usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "blend", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the light intensity of the color, as a value between 0.0 and 1.0 (inclusive). This is useful when determining light or dark color. Colors with a luminance smaller than 0.5 can be generally considered dark.\n\n**Note:** \\[method get_luminance] relies on the color using linear encoding to return an accurate relative luminance value. If the color uses the default nonlinear sRGB encoding, use \\[method srgb_to_linear] to convert it to linear encoding first."]
    pub fn get_luminance(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(470usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "get_luminance", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the color that uses linear encoding. This method requires the original color to be encoded using the [nonlinear sRGB transfer function](https://en.wikipedia.org/wiki/SRGB). See also \\[method linear_to_srgb] which performs the opposite operation.\n\n**Note:** The color's alpha channel (\\[member a]) is not affected. The alpha channel is always stored with linear encoding, regardless of the color space of the other color channels."]
    pub fn srgb_to_linear(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(471usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "srgb_to_linear", self.sys_ptr, args)
        }
    }
    #[doc = "Returns a copy of the color that is encoded using the [nonlinear sRGB transfer function](https://en.wikipedia.org/wiki/SRGB). This method requires the original color to use linear encoding. See also \\[method srgb_to_linear] which performs the opposite operation.\n\n**Note:** The color's alpha channel (\\[member a]) is not affected. The alpha channel is always stored with linear encoding, regardless of the color space of the other color channels."]
    pub fn linear_to_srgb(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(472usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "linear_to_srgb", self.sys_ptr, args)
        }
    }
    #[doc = "Returns `true` if this color and `to` are approximately equal, by running [`is_equal_approx`][`crate::global::is_equal_approx`] on each component."]
    pub fn is_equal_approx(&self, to: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(473usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "is_equal_approx", self.sys_ptr, args)
        }
    }
    #[doc = "Returns the [`Color`][crate::builtin::Color] associated with the provided `hex` integer in 32-bit RGBA format (8 bits per channel). This method is the inverse of \\[method to_rgba32].\n\nIn GDScript and C#, the `int` is best visualized with hexadecimal notation (`\"0x\"` prefix, making it `\"0xRRGGBBAA\"`).\n\n\n```gdscript\nvar red = Color.hex(0xff0000ff)\nvar dark_cyan = Color.hex(0x008b8bff)\nvar my_color = Color.hex(0xbbefd2a4)\n```\n\n\nIf you want to use hex notation in a constant expression, use the equivalent constructor instead (i.e. `Color(0xRRGGBBAA)`)."]
    pub fn hex(hex: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(474usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "hex", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns the [`Color`][crate::builtin::Color] associated with the provided `hex` integer in 64-bit RGBA format (16 bits per channel). This method is the inverse of \\[method to_rgba64].\n\nIn GDScript and C#, the `int` is best visualized with hexadecimal notation (`\"0x\"` prefix, making it `\"0xRRRRGGGGBBBBAAAA\"`)."]
    pub fn hex64(hex: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(475usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "hex64", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns a new color from `rgba`, an HTML hexadecimal color string. `rgba` is not case-sensitive, and may be prefixed by a hash sign (`#`).\n\n`rgba` must be a valid three-digit or six-digit hexadecimal color string, and may contain an alpha channel value. If `rgba` does not contain an alpha channel value, an alpha channel value of 1.0 is applied. If `rgba` is invalid, returns an empty color.\n\n\n```gdscript\nvar blue = Color.html(\"#0000ff\") # blue is Color(0.0, 0.0, 1.0, 1.0)\nvar green = Color.html(\"#0F0\")   # green is Color(0.0, 1.0, 0.0, 1.0)\nvar col = Color.html(\"663399cc\") # col is Color(0.4, 0.2, 0.6, 0.8)\n```\n"]
    pub fn html(rgba: impl AsArg < GString >,) -> Color {
        type CallRet = Color;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (rgba.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(476usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "html", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns `true` if `color` is a valid HTML hexadecimal color string. The string must be a hexadecimal value (case-insensitive) of either 3, 4, 6 or 8 digits, and may be prefixed by a hash sign (`#`). This method is identical to [`is_valid_html_color`][`crate::builtin::GString::is_valid_html_color`].\n\n\n```gdscript\nColor.html_is_valid(\"#55aaFF\")   # Returns true\nColor.html_is_valid(\"#55AAFF20\") # Returns true\nColor.html_is_valid(\"55AAFF\")    # Returns true\nColor.html_is_valid(\"#F2C\")      # Returns true\n\nColor.html_is_valid(\"#AABBC\")    # Returns false\nColor.html_is_valid(\"#55aaFF5\")  # Returns false\n```\n"]
    pub fn html_is_valid(color: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (color.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(477usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "html_is_valid", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Creates a [`Color`][crate::builtin::Color] from the given string, which can be either an HTML color code or a named color (case-insensitive). Returns `default` if the color cannot be inferred from the string.\n\nIf you want to create a color from String in a constant expression, use the equivalent constructor instead (i.e. `Color(\"color string\")`)."]
    pub fn from_string(str: impl AsArg < GString >, default: Color,) -> Color {
        type CallRet = Color;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
        let args = (str.into_arg(), default,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(478usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_string", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Constructs a color from an [HSV profile](https://en.wikipedia.org/wiki/HSL_and_HSV). The hue (`h`), saturation (`s`), and value (`v`) are typically between 0.0 and 1.0.\n\n\n```gdscript\nvar color = Color.from_hsv(0.58, 0.5, 0.79, 0.8)\n```\n"]
    pub fn from_hsv(h: f64, s: f64, v: f64, alpha: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64, f64, f64, f64,);
        let args = (h, s, v, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(479usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_hsv", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Constructs a color from an [OK HSL profile](https://bottosson.github.io/posts/colorpicker/). The hue (`h`), saturation (`s`), and lightness (`l`) are typically between 0.0 and 1.0.\n\n\n```gdscript\nvar color = Color.from_ok_hsl(0.58, 0.5, 0.79, 0.8)\n```\n"]
    pub fn from_ok_hsl(h: f64, s: f64, l: f64, alpha: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64, f64, f64, f64,);
        let args = (h, s, l, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(480usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_ok_hsl", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Decodes a [`Color`][crate::builtin::Color] from an RGBE9995 format integer. See [`Format::RGBE9995`][`crate::classes::image::Format::RGBE9995`]."]
    pub fn from_rgbe9995(rgbe: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (rgbe,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(481usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_rgbe9995", std::ptr::null_mut(), args)
        }
    }
    #[doc = "Returns a [`Color`][crate::builtin::Color] constructed from red (`r8`), green (`g8`), blue (`b8`), and optionally alpha (`a8`) integer channels, each divided by `255.0` for their final value.\n\n```gdscript\nvar red = Color.from_rgba8(255, 0, 0)             # Same as Color(1, 0, 0).\nvar dark_blue = Color.from_rgba8(0, 0, 51)        # Same as Color(0, 0, 0.2).\nvar my_color = Color.from_rgba8(306, 255, 0, 102) # Same as Color(1.2, 1, 0, 0.4).\n```\n\n**Note:** Due to the lower precision of \\[method from_rgba8] compared to the standard [`Color`][crate::builtin::Color] constructor, a color created with \\[method from_rgba8] will generally not be equal to the same color created with the standard [`Color`][crate::builtin::Color] constructor. Use \\[method is_equal_approx] for comparisons to avoid issues with floating-point precision error."]
    pub fn from_rgba8(r8: i64, g8: i64, b8: i64, a8: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64, i64, i64, i64,);
        let args = (r8, g8, b8, a8,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(482usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_rgba8", std::ptr::null_mut(), args)
        }
    }
}
pub use re_export::InnerColor;
impl Color {
    
}