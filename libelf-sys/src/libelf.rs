#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate libc;

use elf;

pub type Elf_Type = libc::c_uint;
pub const ELF_T_BYTE: Elf_Type = 0;	/* unsigned char */
pub const ELF_T_ADDR: Elf_Type = 1;	/* Elf32_Addr, Elf64_Addr, ... */
pub const ELF_T_DYN: Elf_Type = 2;	/* Dynamic section record.  */
pub const ELF_T_EHDR: Elf_Type = 3;	/* ELF header.  */
pub const ELF_T_HALF: Elf_Type = 4;	/* Elf32_Half, Elf64_Half, ... */
pub const ELF_T_OFF: Elf_Type = 5;	/* Elf32_Off, Elf64_Off, ... */
pub const ELF_T_PHDR: Elf_Type = 6;	/* Program header.  */
pub const ELF_T_RELA: Elf_Type = 7;	/* Relocation entry with addend.  */
pub const ELF_T_REL: Elf_Type = 8;	/* Relocation entry.	*/
pub const ELF_T_SHDR: Elf_Type = 9;	/* Section header.  */
pub const ELF_T_SWORD: Elf_Type = 10;	/* Elf32_Sword, Elf64_Sword, ... */
pub const ELF_T_SYM: Elf_Type = 11;	/* Symbol record.  */
pub const ELF_T_WORD: Elf_Type = 12;	/* Elf32_Word, Elf64_Word, ... */
pub const ELF_T_XWORD: Elf_Type = 13;	/* Elf32_Xword, Elf64_Xword, ... */
pub const ELF_T_SXWORD: Elf_Type = 14;	/* Elf32_Sxword, Elf64_Sxword, ... */
pub const ELF_T_VDEF: Elf_Type = 15;	/* Elf32_Verdef, Elf64_Verdef, ... */
pub const ELF_T_VDAUX: Elf_Type = 16;	/* Elf32_Verdaux, Elf64_Verdaux, ... */
pub const ELF_T_VNEED: Elf_Type = 17;	/* Elf32_Verneed, Elf64_Verneed, ... */
pub const ELF_T_VNAUX: Elf_Type = 18;	/* Elf32_Vernaux, Elf64_Vernaux, ... */
pub const ELF_T_NHDR: Elf_Type = 19;	/* Elf32_Nhdr, Elf64_Nhdr, ... */
pub const ELF_T_SYMINFO: Elf_Type = 20;	/* Elf32_Syminfo, Elf64_Syminfo, ... */
pub const ELF_T_MOVE: Elf_Type = 21;	/* Elf32_Move, Elf64_Move, ... */
pub const ELF_T_LIB: Elf_Type = 22;	/* Elf32_Lib, Elf64_Lib, ... */
pub const ELF_T_GNUHASH: Elf_Type = 23;	/* GNU-style hash section.  */
pub const ELF_T_AUXV: Elf_Type = 24;	/* Elf32_auxv_t, Elf64_auxv_t, ... */

/* Descriptor for data to be converted to or from memory format.  */
#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut libc::c_void,	/* Pointer to the actual data.  */
    pub d_type: Elf_Type,		/* Type of this piece of data.  */
    pub d_version: libc::c_uint,	/* ELF version.  */
    pub d_size: libc::size_t,		/* Size in bytes.  */
    pub d_off: libc::uint64_t,		/* Offset into section.  */
    pub d_align: libc::size_t,		/* Alignment in section.  */
}


/* Commands for `...'.  */
pub type Elf_Cmd = libc::c_uint;
pub const ELF_C_NULL: Elf_Cmd = 0;	/* Nothing, terminate, or compute only.  */
pub const ELF_C_READ: Elf_Cmd = 1;	/* Read .. */
pub const ELF_C_RDWR: Elf_Cmd = 2;	/* Read and write .. */
pub const ELF_C_WRITE: Elf_Cmd = 3;	/* Write .. */
pub const ELF_C_CLR: Elf_Cmd = 4;	/* Clear flag.  */
pub const ELF_C_SET: Elf_Cmd = 5;	/* Set flag.  */
pub const ELF_C_FDDONE: Elf_Cmd = 6;	/* Signal that file descriptor will not
                                           be used anymore.  */
pub const ELF_C_FDREAD: Elf_Cmd = 7;	/* Read rest of data so that file descriptor
                                           is not used anymore.  */
pub const ELF_C_READ_MMAP: Elf_Cmd = 8;	/* Read, but mmap the file if possible.  */
pub const ELF_C_RDWR_MMAP: Elf_Cmd = 9;	/* Read and write, with mmap.  */
pub const ELF_C_WRITE_MMAP: Elf_Cmd = 10;	/* Write, with mmap.  */
pub const ELF_C_READ_MMAP_PRIVATE: Elf_Cmd = 11; /* Read, but memory is writable, results
                                                    are not written to the file.  */
pub const ELF_C_EMPTY: Elf_Cmd = 12;	/* Copy basic file data but not the content. */


// /* Flags for the ELF structures.  */
// enum
// {
//   ELF_F_DIRTY = 0x1,
// #define ELF_F_DIRTY		ELF_F_DIRTY
//   ELF_F_LAYOUT = 0x4,
// #define ELF_F_LAYOUT		ELF_F_LAYOUT
//   ELF_F_PERMISSIVE = 0x8
// #define ELF_F_PERMISSIVE	ELF_F_PERMISSIVE
// };

pub type Elf_Kind = libc::c_uint;
pub const ELF_K_NONE: Elf_Kind = 0;	/* Unknown.  */
pub const ELF_K_AR: Elf_Kind = 1;	/* Archive.  */
pub const ELF_K_COFF: Elf_Kind = 2;	/* Stupid old COFF.  */
pub const ELF_K_ELF: Elf_Kind = 3;	/* ELF file.  */


pub type loff_t = libc::c_longlong;


/* Archive member header.  */

#[repr(C)]
pub struct Elf_Arhdr {
    pub ar_name: *mut libc::c_char,	/* Name of archive member.  */
    pub ar_date: libc::time_t,		/* File date.  */
    pub ar_uid: libc::uid_t,		/* User ID.  */
    pub ar_gid: libc::gid_t,		/* Group ID.  */
    pub ar_mode: libc::mode_t,		/* File mode.  */
    pub ar_size: loff_t,		/* File size.  */
    pub ar_rawname: *mut libc::c_char,	/* Original name of archive member.  */
}


/* Archive symbol table entry.  */

#[repr(C)]
pub struct Elf_Arsym {
    pub as_name: *mut libc::c_char,	/* Symbol name.  */
    pub as_off: libc::size_t,		/* Offset for this file in the archive.  */
    pub as_hash: libc::c_ulong,		/* Hash value of the name.  */
}


/* Descriptor for the ELF file.  */

#[repr(C)]
struct Elf_impl;
pub type Elf = *mut Elf_impl;

/* Descriptor for ELF file section.  */
#[repr(C)]
pub struct Elf_Scn;

#[link(name = "elf")]
extern "C" {
    /* Return descriptor for ELF file to work according to CMD.  */
    pub fn elf_begin(fildes: libc::c_int, cmd: Elf_Cmd, ref_elf: Elf) -> Elf;

    /* Create a clone of an existing ELF descriptor.  */
    pub fn elf_clone(elf: Elf, cmd: Elf_Cmd) -> Elf;

    /* Create descriptor for memory region.  */
    pub fn elf_memory(image: *mut libc::c_char, size: libc::size_t) -> Elf;

    /* Advance archive descriptor to next element.  */
    pub fn elf_next(elf: Elf) -> Elf_Cmd;

    /* Free resources allocated for ELF.  */
    pub fn elf_end(elf: Elf) -> libc::c_int;

    /* Update ELF descriptor and write file to disk.  */
    pub fn elf_update(elf: Elf, cmd: Elf_Cmd) -> loff_t;

    /* Determine what kind of file is associated with ELF.  */
    pub fn elf_kind(elf: Elf) -> Elf_Kind;

    /* Get the base offset for an object file.  */
    pub fn elf_getbase(elf: Elf) -> loff_t;

    /* Retrieve file identification data.  */
    // N.B.: in C, this returns plain char, not unsigned char, so it
    // could be either signed or unsigned.  But pretend it's actually
    // unsigned char for consistency with Elf{32,64}_Ehdr.
    pub fn elf_getident(elf: Elf, nbytes: *mut libc::size_t) -> *mut libc::c_uchar;

    /* Retrieve class-dependent object file header.  */
    pub fn elf32_getehdr(elf: Elf) -> *mut elf::Elf32_Ehdr;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_getehdr(elf: Elf) -> *mut elf::Elf64_Ehdr;

    /* Create ELF header if none exists.  */
    pub fn elf32_newehdr(elf: Elf) -> *mut elf::Elf32_Ehdr;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_newehdr(elf: Elf) -> *mut elf::Elf64_Ehdr;

    /* Get the number of program headers in the ELF file.  If the file
       uses more headers than can be represented in the e_phnum field
       of the ELF header the information from the sh_info field in the
       zeroth section header is used.  */
    pub fn elf_getphdrnum(elf: Elf, dst: *mut libc::size_t) -> libc::c_int;

    /* Retrieve class-dependent program header table.  */
    pub fn elf32_getphdr(elf: Elf) -> *mut elf::Elf32_Phdr;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_getphdr(elf: Elf) -> *mut elf::Elf64_Phdr;

    /* Create ELF program header.  */
    pub fn elf32_newphdr(elf: Elf, cnt: libc::size_t) -> *mut elf::Elf32_Phdr;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_newphdr(elf: Elf, cnt: libc::size_t) -> *mut elf::Elf64_Phdr;

    /* Get section at INDEX.  */
    pub fn elf_getscn(elf: Elf, index: libc::size_t) -> *mut Elf_Scn;

    /* Get section at OFFSET.  */
    pub fn elf32_offscn(elf: Elf, offset: elf::Elf32_Off) -> *mut Elf_Scn;

    /* Similar bug this time the binary calls is ELFCLASS64.  */
    pub fn elf64_offscn(elf: Elf, offset: elf::Elf64_Off) -> *mut Elf_Scn;

    /* Get index of section.  */
    pub fn elf_ndxscn(scn: *mut Elf_Scn) -> libc::size_t;

    /* Get section with next section index.  */
    pub fn elf_nextscn(elf: Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;

    /* Create a new section and append it at the end of the table.  */
    pub fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn;

    /* Get the section index of the extended section index table for the
       given symbol table.  */
    pub fn elf_scnshndx(scn: *mut Elf_Scn) -> libc::c_int;

    /* Get the number of sections in the ELF file.  If the file uses more
       sections than can be represented in the e_shnum field of the ELF
       header the information from the sh_size field in the zeroth section
       header is used.  */
    pub fn elf_getshdrnum(elf: Elf, dst: *mut libc::size_t) -> libc::c_int;

    /* Get the section index of the section header string table in the ELF
       file.  If the index cannot be represented in the e_shnum field of
       the ELF header the information from the sh_link field in the zeroth
       section header is used.  */
    pub fn elf_getshdrstrndx(elf: Elf, dst: *mut libc::size_t) -> libc::c_int;

    /* Retrieve section header of ELFCLASS32 binary.  */
    pub fn elf32_getshdr(scn: *mut Elf_Scn) -> *mut elf::Elf32_Shdr;

    /* Similar for ELFCLASS64.  */
    pub fn elf64_getshdr(scn: *mut Elf_Scn) -> *mut elf::Elf64_Shdr;

    /* Set or clear flags for ELF file.  */
    pub fn elf_flagelf(elf: Elf, cmd: Elf_Cmd, flags: libc::c_uint) -> libc::c_uint;

    /* Similarly for the ELF header.  */
    pub fn elf_flagehdr(elf: Elf, cmd: Elf_Cmd, flags: libc::c_uint) -> libc::c_uint;

    /* Similarly for the ELF program header.  */
    pub fn elf_flagphdr(elf: Elf, cmd: Elf_Cmd, flags: libc::c_uint) -> libc::c_uint;

    /* Similarly for the given ELF section.  */
    pub fn elf_flagscn(scn: *mut Elf_Scn, cmd: Elf_Cmd,
                       flags: libc::c_uint) -> libc::c_uint;

    /* Similarly for the given ELF data.  */
    pub fn elf_flagdata(data: *mut Elf_Data, cmd: Elf_Cmd,
                        flags: libc::c_uint) -> libc::c_uint;

    /* Similarly for the given ELF section header.  */
    pub fn elf_flagshdr(scn: *mut Elf_Scn, cmd: Elf_Cmd,
                        flags: libc::c_uint) -> libc::c_uint;

    /* Get data from section while translating from file representation
       to memory representation.  */
    pub fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;

    /* Get uninterpreted section content.  */
    pub fn elf_rawdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;

    /* Create new data descriptor for section SCN.  */
    pub fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data;

    /* Get data translated from a chunk of the file contents as section data
       would be for TYPE.  The resulting Elf_Data pointer is valid until
       elf_end (ELF) is called.  */
    pub fn elf_getdata_rawchunk(elf: Elf, offset: loff_t, size: libc::size_t,
                                typ: Elf_Type) -> *mut Elf_Data;

    /* Return pointer to string at OFFSET in section INDEX.  */
    pub fn elf_strptr(elf: Elf, index: libc::size_t,
                      offset: libc::size_t) -> *mut libc::c_char;

    /* Return header of archive.  */
    pub fn elf_getarhdr(elf: Elf) -> *mut Elf_Arhdr;

    /* Return offset in archive for current file ELF.  */
    pub fn elf_getaroff(elf: Elf) -> loff_t;

    /* Select archive element at OFFSET.  */
    pub fn elf_rand(elf: Elf, offset: libc::size_t) -> libc::size_t;

    /* Get symbol table of archive.  */
    pub fn elf_getarsym(elf: Elf, narsyms: *mut libc::size_t) -> *mut Elf_Arsym;

    /* Control ELF descriptor.  */
    pub fn elf_cntl(elf: Elf, cmd: Elf_Cmd) -> libc::c_int;

    /* Retrieve uninterpreted file contents.  */
    pub fn elf_rawfile(elf: Elf, nbytes: *mut libc::size_t) -> *mut libc::c_char;

    /* Return size of array of COUNT elements of the type denoted by TYPE
       in the external representation.  The binary class is taken from ELF.
       The result is based on version VERSION of the ELF standard.  */
    pub fn elf32_fsize(typ: Elf_Type, count: libc::size_t,
                       version: libc::c_uint) -> libc::size_t;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_fsize(typ: Elf_Type, count: libc::size_t,
                       version: libc::c_uint) -> libc::size_t;

    /* Convert data structure from the representation in the file represented
       by ELF to their memory representation.  */
    pub fn elf32_xlatetom(dest: *mut Elf_Data, src: *const Elf_Data,
                          encoding: libc::c_uint) -> *mut Elf_Data;

    /* Same for 64 bit class.  */
    pub fn elf64_xlatetom(dest: *mut Elf_Data, src: *const Elf_Data,
                          encoding: libc::c_uint) -> *mut Elf_Data;

    /* Convert data structure from to the representation in memory
       represented by ELF file representation.  */
    pub fn elf32_xlatetof(dest: *mut Elf_Data, src: *const Elf_Data,
                          encoding: libc::c_uint) -> *mut Elf_Data;

    /* Same for 64 bit class.  */
    pub fn elf64_xlatetof(dest: *mut Elf_Data, src: *const Elf_Data,
                          encoding: libc::c_uint) -> *mut Elf_Data;

    /* Return error code of last failing function call.  This value is
       kept separately for each thread.  */
    pub fn elf_errno() -> libc::c_int;

    /* Return error string for ERROR.  If ERROR is zero, return error string
       for most recent error or NULL is none occurred.  If ERROR is -1 the
       behaviour is similar to the last case except that not NULL but a legal
       string is returned.  */
    pub fn elf_errmsg(error: libc::c_int) -> *const libc::c_char;

    /* Coordinate ELF library and application versions.  */
    pub fn elf_version(version: libc::c_uint) -> libc::c_uint;

    /* Set fill bytes used to fill holes in data structures.  */
    pub fn elf_fill(fill: libc::c_int);

    /* Compute hash value.  */
    pub fn elf_hash(string: *const libc::c_char) -> libc::c_ulong;

    /* Compute hash value using the GNU-specific hash function.  */
    pub fn elf_gnu_hash(string: *const libc::c_char) -> libc::c_ulong;

    /* Compute simple checksum from permanent parts of the ELF file.  */
    pub fn elf32_checksum(elf: Elf) -> libc::c_long;

    /* Similar but this time the binary calls is ELFCLASS64.  */
    pub fn elf64_checksum(elf: Elf) -> libc::c_long;
}
