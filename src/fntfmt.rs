
use crate::*;
use libc::*;

extern "system"
{
	#[deprecated(note = "use FT_Get_X11_Font_Format")]
	pub fn FT_Get_Font_Format(face: FT_Face) -> *const c_char;
	pub fn FT_Get_X11_Font_Format(face: FT_Face) -> *const c_char;
}
