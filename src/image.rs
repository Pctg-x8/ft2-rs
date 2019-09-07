#![allow(non_camel_case_types)]

use libc::*;

pub type FT_Pos = c_long;
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FT_Vector { pub x: FT_Pos, pub y: FT_Pos }
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_BBox
{
	pub xMin: FT_Pos,
	pub yMin: FT_Pos,
	pub xMax: FT_Pos,
	pub yMax: FT_Pos
}

pub type FT_Pixel_Mode = c_uchar;
pub const FT_PIXEL_MODE_NONE: FT_Pixel_Mode = 0;
pub const FT_PIXEL_MODE_MONO: FT_Pixel_Mode = 1;
pub const FT_PIXEL_MODE_GRAY: FT_Pixel_Mode = 2;
pub const FT_PIXEL_MODE_GRAY2: FT_Pixel_Mode = 3;
pub const FT_PIXEL_MODE_GRAY4: FT_Pixel_Mode = 4;
pub const FT_PIXEL_MODE_LCD: FT_Pixel_Mode = 5;
pub const FT_PIXEL_MODE_LCD_V: FT_Pixel_Mode = 6;
pub const FT_PIXEL_MODE_BGRA: FT_Pixel_Mode = 7;

#[repr(C)] #[derive(Debug)]
pub struct FT_Bitmap
{
	pub rows: c_uint,
	pub width: c_uint,
	pub pitch: c_int,
	pub buffer: *mut c_uchar,
	pub num_grays: c_ushort,
	pub pixel_mode: c_uchar,
	pub palette_mode: c_uchar,
	pub palette: *mut c_void
}

#[repr(C)] #[derive(Debug)]
pub struct FT_Outline
{
	pub n_contours: c_short,
	pub n_points: c_short,
	
	pub points: *mut FT_Vector,
	pub tags: *mut c_char,
	pub contours: *mut c_short,
	
	pub flags: c_int
}

pub const FT_OUTLINE_NONE: c_int = 0x0;
pub const FT_OUTLINE_OWNER: c_int = 0x1;
pub const FT_OUTLINE_EVEN_ODD_FILL: c_int = 0x2;
pub const FT_OUTLINE_REVERSE_FILL: c_int = 0x0;
pub const FT_OUTLINE_IGNORE_DROPOUTS: c_int = 0x0;
pub const FT_OUTLINE_SMART_DROPOUTS: c_int = 0x10;
pub const FT_OUTLINE_INCLUDE_STUBS: c_int = 0x20;

pub const FT_OUTLINE_HIGH_PRECISION: c_int = 0x100;
pub const FT_OUTLINE_SINGLE_PASS: c_int = 0x200;

#[macro_export]
macro_rules! FT_CURVE_TAG { ($flag: expr) => ($flag & 0x03) }

pub const FT_CURVE_TAG_ON: c_int = 0x01;
pub const FT_CURVE_TAG_CONIC: c_int = 0x00;
pub const FT_CURVE_TAG_CUBIC: c_int = 0x02;

pub const FT_CURVE_TAG_HAS_SCANCODE: c_int = 0x04;

pub const FT_CURVE_TAG_TOUCH_X: c_int = 0x08;
pub const FT_CURVE_TAG_TOUCH_Y: c_int = 0x10;
pub const FT_CURVE_TAG_TOUCH_BOTH: c_int = FT_CURVE_TAG_TOUCH_X | FT_CURVE_TAG_TOUCH_Y;

pub type FT_Outline_MoveToFunc = extern "system" fn(to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_LineToFunc = extern "system" fn(to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_ConicToFunc = extern "system" fn(
	control: *const FT_Vector, to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_CubicToFunc = extern "system" fn(
	control1: *const FT_Vector, onst2: *const FT_Vector, to: *const FT_Vector, user: *mut c_void) -> c_int;
#[repr(C)]
pub struct FT_Outline_Funcs
{
	pub move_to: FT_Outline_MoveToFunc,
	pub line_to: FT_Outline_LineToFunc,
	pub conic_to: FT_Outline_ConicToFunc,
	pub cubic_to: FT_Outline_CubicToFunc,

	pub shift: c_int,
	pub delta: FT_Pos
}

#[macro_export]
macro_rules! FT_IMAGE_TAG
{
	($x1: expr, $x2: expr, $x3: expr, $x4: expr) => (
		(($x1 as c_ulong) << 24) |
		(($x2 as c_ulong) << 16) |
		(($x3 as c_ulong) << 8) |
		($x4 as c_ulong)
	)
}

pub type FT_Glyph_Format = c_ulong;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format = 0;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format = FT_IMAGE_TAG!('c', 'o', 'm', 'p');
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format = FT_IMAGE_TAG!('b', 'i', 't', 's');
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format = FT_IMAGE_TAG!('o', 'u', 't', 'l');
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format = FT_IMAGE_TAG!('p', 'l', 'o', 't');

pub enum FT_RasterRec_ { /* opaque */ }
pub type FT_Raster = *mut FT_RasterRec_;
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_Span
{
	pub x: c_short,
	pub len: c_ushort,
	pub coverage: c_uchar
}
pub type FT_SpanFunc = extern "system" fn(y: c_int, count: c_int, spans: *const FT_Span, user: *mut c_void);

pub type FT_Raster_BitTest_Func = extern "system" fn(y: c_int, x: c_int, user: *mut c_void) -> c_int;
pub type FT_Raster_BitSet_Func = extern "system" fn(y: c_int, x: c_int, user: *mut c_void);

pub const FT_RASTER_FLAG_DEFAULT: c_int = 0x0;
pub const FT_RASTER_FLAG_AA: c_int = 0x1;
pub const FT_RASTER_FLAG_DIRECT: c_int = 0x2;
pub const FT_RASTER_FLAG_CLIP: c_int = 0x4;
#[repr(C)]
pub struct FT_Raster_Params
{
	pub target: *const FT_Bitmap,
	pub source: *const c_void,
	pub flags: c_int,
	pub gray_spans: FT_SpanFunc,
	pub black_spans: FT_SpanFunc,
	pub bit_test: FT_Raster_BitTest_Func,
	pub bit_set: FT_Raster_BitSet_Func,
	pub user: *mut c_void,
	pub clip_box: FT_BBox
}

pub type FT_Raster_NewFunc = extern "system" fn(memory: *mut c_void, raster: *mut FT_Raster) -> c_int;
pub type FT_Raster_DoneFunc = extern "system" fn(raster: FT_Raster);
pub type FT_Raster_ResetFunc = extern "system" fn(raster: FT_Raster, pool_base: *mut c_uchar, pool_size: c_ulong);
pub type FT_Raster_SetModeFunc = extern "system" fn(raster: FT_Raster, mode: c_ulong, args: *mut c_void) -> c_int;
pub type FT_Raster_RenderFunc = extern "system" fn(raster: FT_Raster, params: *const FT_Raster_Params) -> c_int;
#[repr(C)]
pub struct FT_Raster_Funcs
{
	pub glyph_format: FT_Glyph_Format,

	pub raster_new: FT_Raster_NewFunc,
	pub raster_reset: FT_Raster_ResetFunc,
	pub raster_set_mode: FT_Raster_SetModeFunc,
	pub raster_render: FT_Raster_RenderFunc,
	pub raster_done: FT_Raster_DoneFunc
}
