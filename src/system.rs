#![allow(non_camel_case_types)]

use libc::*;

/// A handle to a given memory manager object, defined with an [FT_MemoryRec] structure.
pub type FT_Memory = *mut FT_MemoryRec_;

/// A function used to allocate `size` bytes from `memory`.
pub type FT_Alloc_Func = extern "system" fn(memory: FT_Memory, size: c_long) -> *mut c_void;

/// A function used to release a given block of memory.
pub type FT_Free_Func = extern "system" fn(memory: FT_Memory, block: *mut c_void);

/// A function used to re-allocate a given block of memory.
pub type FT_Realloc_Func = extern "system" fn(memory: FT_Memory,
	cur_size: c_long, new_size: c_long, block: *mut c_void) -> *mut c_void;

/// A structure used to describe a given memory manager to FreeType-2.
#[repr(C)]
pub struct FT_MemoryRec_
{
	pub user: *mut c_void,
	pub alloc: FT_Alloc_Func,
	pub free: FT_Free_Func,
	pub realloc: FT_Realloc_Func
}

pub type FT_Stream = *mut FT_StreamRec;
#[repr(C)]
pub union FT_StreamDesc
{
	pub value: c_long,
	pub pointer: *mut c_void
}

pub type FT_Stream_IoFunc = extern "system" fn(stream: FT_Stream,
	offset: c_ulong, buffer: *mut c_uchar, count: c_ulong) -> c_ulong;
pub type FT_Stream_CloseFunc = extern "system" fn(stream: FT_Stream);
#[repr(C)]
pub struct FT_StreamRec
{
	pub base: *mut c_uchar,
	pub size: c_ulong,
	pub pos: c_ulong,

	pub descriptor: FT_StreamDesc,
	pub pathname: FT_StreamDesc,
	pub read: FT_Stream_IoFunc,
	pub close: FT_Stream_CloseFunc,

	pub memory: FT_Memory,
	pub cursor: *mut c_uchar,
	pub limit: *mut c_uchar
}
