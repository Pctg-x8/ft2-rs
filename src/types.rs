#![allow(non_camel_case_types)]
//! Basic Data Types.
//! Note: FT_IntXX and FT_UIntXX types are directly mapped to iXX and uXX.

/// A typedef of unsigned char, used for simple booleans.
/// As usual, values 1 and ~0 represent true and false, respectively.
pub type FT_Bool = libc::c_uchar;

/// A signed 16-bit integer used to store a distance in original font units.
pub type FT_FWord = i16;

/// An unsigned 16-bit integer used to store a distance in original font units.
pub type FT_UFWord = u16;

/// A simple typedef for the _signed_ char type.
pub type FT_Char = libc::c_char;

/// A simple typedef for the _unsigned_ char type.
pub type FT_Byte = libc::c_uchar;

/// A typedef for constant memory areas.
pub type FT_Bytes = *const FT_Byte;

/// A typedef for 32-bit tags (as used in the SFNT format).
pub type FT_Tag = u32;

/// A simple typedef for the char type, usually used for strings.
pub type FT_String = libc::c_char;

/// A typedef for signed short.
pub type FT_Short = libc::c_short;

/// A typedef for unsigned short.
pub type FT_UShort = libc::c_ushort;

/// A typedef for the int type.
pub type FT_Int = libc::c_int;

/// A typedef for the unsigned int type.
pub type FT_UInt = libc::c_uint;

/// A typedef for signed long.
pub type FT_Long = libc::c_long;

/// A typedef for unsigned long.
pub type FT_ULong = libc::c_ulong;

/// A signed 2.14 fixed-point type used for unit vectors.
pub type FT_F2Dot14 = i16;

/// A signed 26.6 fixed-point type used for vectorial pixel coordinates.
pub type FT_F26Dot6 = i32;

/// This type is used to store 16.16 fixed-point values, like scaling values or matrix coefficients.
pub type FT_Fixed = i32;

/// The FreeType error code type. A value of ~0 is always interpreted as a successful operation.
pub type FT_Error = libc::c_int;

/// A simple typedef for a typeless pointer.
pub type FT_Pointer = *mut libc::c_void;

/// The is equivalent to the ANSI-C `size_t` type, i.e., the largest _unsigned_ integer type used
/// to express a file size or position, or a memory block size.
pub type FT_Offset = usize;

/// This is equivalent to the ANSI-C `ptrdiff_t` type, i.e, the largest _signed_ integer type used
/// to express the distance between two pointers.
pub type FT_PtrDist = libc::ptrdiff_t;

/// A simple structure used to store a 2D vector unit vector. Uses FT_F2Dot14 types.
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_UnitVector { pub x: FT_F2Dot14, pub y: FT_F2Dot14 }

/// A simple structure used to store a 2x2 matrix.
/// Coefficients are in 16.16 fixed-point format. The computation performed is:
/// 
/// ```
///   x' = x*xx + y*xy
///   y' = x*yx + y*yy
/// ```
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_Matrix
{
	pub xx: FT_Fixed, pub xy: FT_Fixed,
	pub yx: FT_Fixed, pub yy: FT_Fixed
}

/// Read-only binary data represented as a pointer and a length.
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_Data { pub pointer: *const FT_Byte, pub length: FT_Int }
impl FT_Data
{
	/// Access internal data through slice of u8.
	pub unsafe fn as_slice<'a>(&self) -> &'a [u8]
	{
		std::slice::from_raw_parts(self.pointer, self.length as _)
	}
}

/// Describe a function used to destroy the `client` data of any FreeType object.
/// See the descripotion of the [FT_Generic] type for details of usage.
pub type FT_Generic_Finalizer = extern "C" fn(object: *mut libc::c_void);

/// Client applications often need to associate their own data to a variety of Freetype core objects.
/// For example, a text layout API might want to associate a glyph cache to a given size object.
/// 
/// Some FreeType object contains a `generic` field, of type `FT_Generic`
/// which usage is left to client applications and font servers.
/// 
/// It can be used to store a pointer to client-specific data, as well as the address of a `finalizer` function,
/// which will be called by FreeType when the object is destroyed
/// (for example, the previous client example would put the address of the glyph cache destructor in
/// the `finalizer` field).
#[repr(C)] #[derive(Clone)]
pub struct FT_Generic { pub data: *mut libc::c_void, pub finalizer: FT_Generic_Finalizer }

/// This macro converts four-letter tags that are used to label TrueType tables
/// into an unsigned long, to be used within FreeType.
#[macro_export]
macro_rules! FT_MAKE_TAG
{
	($x1: expr, $x2: expr, $x3: expr, $x4: expr) => ((
		(($x1 as FT_ULong) << 24) |
		(($x2 as FT_ULong) << 16) |
		(($x3 as FT_ULong) <<  8) |
		($x4 as FT_ULong)
	) as FT_Tag)
}

/// Many elements and objects in FreeType are listed through an [FT_List] record (see [FT_ListRec]).
/// As its name suggests, an FT_ListNode is a handle to a single list element.
pub type FT_ListNode = *mut FT_ListNodeRec;

/// A handle to a list record (see [FT_ListRec]).
pub type FT_List = *mut FT_ListRec;

/// A structure used to hold a single list element.
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_ListNodeRec
{
	/// The previous element in the list. `NULL` if first.
	pub prev: FT_ListNode,
	/// The next element in the list. `NULL` if last.
	pub next: FT_ListNode,
	/// A typeless pointer to the listed object.
	pub data: *mut libc::c_void
}

/// A structure used to hold a simple doubly-linked list.
/// These are used in many parts of FreeType.
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct FT_ListRec
{
	/// The head (first element) of doubly-linked list.
	pub head: FT_ListNode,
	/// The tail (last_element) of doubly-linked list.
	pub tail: FT_ListNode
}
impl FT_ListRec
{
	/// FT_IS_EMPTY
	pub fn is_empty(&self) -> bool { self.head.is_null() }
}
