#![feature(libc)]
extern crate libc;

mod elf;
mod libelf;

#[test]
fn test_elf_version() {
    unsafe {
        assert_eq!(libelf::elf_version(elf::EV_CURRENT), elf::EV_CURRENT);
    }
}

#[cfg(test)]
fn get_elf(filename: &str) -> (libelf::Elf, std::fs::File) {
    use std::ptr::null_mut;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;

    let f = File::open(filename).unwrap();
    let fd = f.as_raw_fd();
    unsafe {
        libelf::elf_version(elf::EV_CURRENT);
        let elf = libelf::elf_begin(fd, libelf::ELF_C_READ_MMAP, null_mut());
        assert!(!elf.is_null());
        (elf, f)
    }
}

#[test]
fn test_elf_various() {
    let (elf, _) = get_elf("../tests/y.o");

    unsafe {
        assert_eq!(libelf::elf_kind(elf), libelf::ELF_K_ELF);
        assert_eq!(libelf::elf_next(elf), libelf::ELF_C_NULL);
        let refcount = libelf::elf_end(elf);
        assert_eq!(refcount, 0);
    }
}

#[test]
fn test_elf_errmsg() {
    use std::ptr::null_mut;
    use std::ffi::CStr;

    unsafe {
        libelf::elf_version(elf::EV_CURRENT);
        let elf = libelf::elf_begin(-1, libelf::ELF_C_READ, null_mut());
        assert!(elf.is_null());
        let errno = libelf::elf_errno();
        assert!(errno != 0);
        let errmsg = CStr::from_ptr(libelf::elf_errmsg(errno));
        assert!(std::str::from_utf8_unchecked(errmsg.to_bytes())
                == "invalid file descriptor");
    }
}

#[cfg(test)]
fn check_ident(ident: &[libc::c_uchar], class: libc::c_uchar, data: libc::c_uchar) {
    assert_eq!(ident[elf::EI_MAG0], elf::ELFMAG0);
    assert_eq!(ident[elf::EI_MAG1], elf::ELFMAG1);
    assert_eq!(ident[elf::EI_MAG2], elf::ELFMAG2);
    assert_eq!(ident[elf::EI_MAG3], elf::ELFMAG3);
    assert_eq!(ident[elf::EI_VERSION], elf::EV_CURRENT as libc::c_uchar);
    assert_eq!(ident[elf::EI_OSABI], elf::ELFOSABI_SYSV);
    assert_eq!(ident[elf::EI_CLASS], class);
    assert_eq!(ident[elf::EI_DATA], data);
    assert_eq!(ident[elf::EI_ABIVERSION], 0);
}

#[test]
fn test_elf_ident() {
    use std::slice;

    let (elf, _) = get_elf("../tests/y.o");

    unsafe {
        let mut nbytes = 0;
        let buf = libelf::elf_getident(elf, &mut nbytes);
        let ident = slice::from_raw_parts(buf, nbytes as usize);
        check_ident(ident, elf::ELFCLASS32, elf::ELFDATA2LSB);
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf32_getehdr() {
    let (elf, _) = get_elf("../tests/y.o");

    unsafe {
        let ehdr32 = libelf::elf32_getehdr(elf);
        assert!(!ehdr32.is_null());
        check_ident(&(*ehdr32).e_ident, elf::ELFCLASS32, elf::ELFDATA2LSB);
        assert_eq!((*ehdr32).e_type, elf::ET_REL);
        assert_eq!((*ehdr32).e_machine, elf::EM_ARM);
        assert_eq!((*ehdr32).e_version, elf::EV_CURRENT);
        assert_eq!((*ehdr32).e_ehsize, 52);
        assert_eq!((*ehdr32).e_shoff, 260);
        assert_eq!((*ehdr32).e_shnum, 10);
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf64_getehdr() {
    let (elf, _) = get_elf("../tests/empty");

    unsafe {
        let ehdr64 = libelf::elf64_getehdr(elf);
        assert!(!ehdr64.is_null());
        check_ident(&(*ehdr64).e_ident, elf::ELFCLASS64, elf::ELFDATA2LSB);
        assert_eq!((*ehdr64).e_type, elf::ET_EXEC);
        assert_eq!((*ehdr64).e_machine, elf::EM_X86_64);
        assert_eq!((*ehdr64).e_version, elf::EV_CURRENT);
        assert_eq!((*ehdr64).e_ehsize, 64);
        assert_eq!((*ehdr64).e_shoff, 4744);
        assert_eq!((*ehdr64).e_shnum, 34);
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf_getphdrnum() {
    let (elf, _) = get_elf("../tests/empty");

    unsafe {
        let mut n: libc::size_t = 0;
        assert_eq!(libelf::elf_getphdrnum(elf, &mut n), 0);
        assert_eq!(n, 9);
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf32_getphdr() {
    let (elf, _) = get_elf("../tests/testfile_const_type");

    let phdrs = unsafe {
        let mut n: libc::size_t = 0;
        assert_eq!(libelf::elf_getphdrnum(elf, &mut n), 0);

        let buf = libelf::elf32_getphdr(elf);
        assert!(!buf.is_null());

        std::slice::from_raw_parts(buf, n as usize)
    };

    let types = [elf::PT_PHDR, elf::PT_INTERP, elf::PT_LOAD, elf::PT_LOAD,
                 elf::PT_DYNAMIC, elf::PT_NOTE, elf::PT_GNU_EH_FRAME,
                 elf::PT_GNU_STACK, elf::PT_GNU_RELRO];

    assert_eq!(phdrs.len(), types.len());
    for i in 0..types.len() {
        assert_eq!(phdrs[i].p_type, types[i]);
    }

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf64_getphdr() {
    let (elf, _) = get_elf("../tests/empty");

    let phdrs = unsafe {
        let mut n: libc::size_t = 0;
        assert_eq!(libelf::elf_getphdrnum(elf, &mut n), 0);

        let buf = libelf::elf64_getphdr(elf);
        assert!(!buf.is_null());

        std::slice::from_raw_parts(buf, n as usize)
    };

    let types = [elf::PT_PHDR, elf::PT_INTERP, elf::PT_LOAD, elf::PT_LOAD,
                 elf::PT_DYNAMIC, elf::PT_NOTE, elf::PT_GNU_EH_FRAME,
                 elf::PT_GNU_STACK, elf::PT_GNU_RELRO];

    assert_eq!(phdrs.len(), types.len());
    for i in 0..types.len() {
        assert_eq!(phdrs[i].p_type, types[i]);
    }

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf_getscn_ndxscn() {
    let (elf, _) = get_elf("../tests/empty");
    let ehdr = unsafe{libelf::elf64_getehdr(elf)};
    assert!(!ehdr.is_null());

    for i in 0..(unsafe{&*ehdr}.e_shnum as libc::size_t) {
        let scn = unsafe{libelf::elf_getscn(elf, i)};
        assert!(!scn.is_null());
        assert_eq!(unsafe{libelf::elf_ndxscn(scn)}, i);
    }

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf_nextscn() {
    use std::ptr::null_mut;

    let (elf, _) = get_elf("../tests/empty");
    let ehdr = unsafe{libelf::elf64_getehdr(elf)};
    assert!(!ehdr.is_null());

    let mut scn = null_mut();
    let mut n = 0;
    loop {
        scn = unsafe{libelf::elf_nextscn(elf, scn)};
        n += 1;
        if scn == null_mut() {
            assert_eq!(n, unsafe{&*ehdr}.e_shnum);
            break;
        }
    }

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf_getshdrnum() {
    let (elf, _) = get_elf("../tests/empty");
    unsafe {
        let ehdr = libelf::elf64_getehdr(elf);
        assert!(!ehdr.is_null());
        let mut n: libc::size_t = 0;
        assert_eq!(libelf::elf_getshdrnum(elf, &mut n), 0);
        assert_eq!((*ehdr).e_shnum as libc::size_t, n);
    }

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf_getshdrstrndx() {
    let (elf, _) = get_elf("../tests/empty");
    let mut n: libc::size_t = 0;
    assert_eq!(unsafe{libelf::elf_getshdrstrndx(elf, &mut n)}, 0);
    assert_eq!(n, 31);

    unsafe {
        libelf::elf_end(elf);
    }
}

#[test]
fn test_elf32_getshdr() {
    use std::ptr::null_mut;

    let (elf, _) = get_elf("../tests/testfile_const_type");

    let types = [elf::SHT_PROGBITS, elf::SHT_NOTE, elf::SHT_NOTE,
                 elf::SHT_GNU_HASH, elf::SHT_DYNSYM, elf::SHT_STRTAB,
                 elf::SHT_GNU_versym, elf::SHT_GNU_verneed,
                 elf::SHT_REL, elf::SHT_REL, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_INIT_ARRAY, elf::SHT_FINI_ARRAY,
                 elf::SHT_PROGBITS, elf::SHT_DYNAMIC,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_NOBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_STRTAB, elf::SHT_SYMTAB,
                 elf::SHT_STRTAB];

    let mut scn = null_mut();
    for &t in types.iter() {
        scn = unsafe{libelf::elf_nextscn(elf, scn)};
        assert!(!scn.is_null());
        let shdr = unsafe{libelf::elf32_getshdr(scn)};
        assert!(!shdr.is_null());
        assert_eq!(unsafe{&*shdr}.sh_type, t);
    }
}

#[test]
fn test_elf64_getshdr() {
    use std::ptr::null_mut;

    let (elf, _) = get_elf("../tests/empty");

    let types = [elf::SHT_PROGBITS, elf::SHT_NOTE, elf::SHT_NOTE,
                 elf::SHT_GNU_HASH, elf::SHT_DYNSYM, elf::SHT_STRTAB,
                 elf::SHT_GNU_versym, elf::SHT_GNU_verneed,
                 elf::SHT_RELA, elf::SHT_RELA, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_DYNAMIC,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_NOBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_PROGBITS,
                 elf::SHT_PROGBITS, elf::SHT_STRTAB, elf::SHT_SYMTAB,
                 elf::SHT_STRTAB];

    let mut scn = null_mut();
    for &t in types.iter() {
        scn = unsafe{libelf::elf_nextscn(elf, scn)};
        assert!(!scn.is_null());
        let shdr = unsafe{libelf::elf64_getshdr(scn)};
        assert!(!shdr.is_null());
        assert_eq!(unsafe{&*shdr}.sh_type, t);
    }
}

#[cfg(test)]
fn secnames_empty() -> &'static [&'static str] {
    static RET: [&'static str; 34] =
        ["\0", ".interp\0", ".note.ABI-tag\0", ".note.gnu.build-id\0",
         ".gnu.hash\0", ".dynsym\0", ".dynstr\0", ".gnu.version\0",
         ".gnu.version_r\0", ".rela.dyn\0", ".rela.plt\0", ".init\0",
         ".plt\0", ".text\0", ".fini\0", ".rodata\0",
         ".eh_frame_hdr\0", ".eh_frame\0", ".ctors\0", ".dtors\0",
         ".jcr\0", ".dynamic\0", ".got\0", ".got.plt\0", ".data\0",
         ".bss\0", ".comment\0", ".debug_info\0", ".debug_abbrev\0",
         ".debug_line\0", ".debug_str\0", ".shstrtab\0", ".symtab\0",
         ".strtab\0"];

    &RET
}

#[cfg(test)]
fn check_sections<Cb>(elf: libelf::Elf, ehdr: *mut elf::Elf64_Ehdr, cb: Cb)
    where Cb: Fn(libelf::Elf, *mut elf::Elf64_Ehdr,
                 *mut libelf::Elf_Scn, *mut elf::Elf64_Shdr)
{
    for i in 0..unsafe{&*ehdr}.e_shnum {
        let scn = unsafe{libelf::elf_getscn(elf, i as libc::size_t)};
        assert!(!scn.is_null());
        let shdr = unsafe{libelf::elf64_getshdr(scn)};
        assert!(!shdr.is_null());
        cb(elf, ehdr, scn, shdr);
    }
}

#[cfg(test)]
fn getdata_test<F>(getdata: F)
    where F: Fn(libelf::Elf, *mut libelf::Elf_Scn) -> *mut libelf::Elf_Data
{
    let (elf, _) = get_elf("../tests/empty");
    let ehdr = unsafe{libelf::elf64_getehdr(elf)};
    assert!(!ehdr.is_null());

    let strings = unsafe {
        let str_scn = libelf::elf_getscn(elf, (*ehdr).e_shstrndx as libc::size_t);
        assert!(!str_scn.is_null());

        let str_data = getdata(elf, str_scn);
        assert!(!str_data.is_null());

        let str_slice = std::slice::from_raw_parts((*str_data).d_buf as *mut libc::c_uchar,
                                                   (*str_data).d_size as usize);
        std::str::from_utf8_unchecked(str_slice)
    };

    let sec_names = secnames_empty();

    check_sections(elf, ehdr,
                   |_: libelf::Elf, _: *mut elf::Elf64_Ehdr,
                    scn: *mut libelf::Elf_Scn, shdr: *mut elf::Elf64_Shdr| {
                        let i = unsafe{libelf::elf_ndxscn(scn)} as usize;
                        let ndx = unsafe{&*shdr}.sh_name as usize;
                        let part = unsafe{strings.slice_unchecked(ndx, strings.len())};
                        assert!(part.starts_with(sec_names[i]));
                    });
}

#[test]
fn test_elf_getdata() {
    getdata_test(|_: libelf::Elf, str_scn: *mut libelf::Elf_Scn| {
        use std::ptr::null_mut;

        let str_data = unsafe{libelf::elf_getdata(str_scn, null_mut())};
        assert!(!str_data.is_null());
        assert!(!unsafe{&*str_data}.d_buf.is_null());
        assert!(unsafe{libelf::elf_getdata(str_scn, str_data)}.is_null());

        str_data
    });
}

#[test]
fn test_elf_rawdata() {
    getdata_test(|_: libelf::Elf, str_scn: *mut libelf::Elf_Scn| {
        use std::ptr::null_mut;

        let str_data = unsafe{libelf::elf_rawdata(str_scn, null_mut())};
        assert!(!str_data.is_null());
        assert!(!unsafe{&*str_data}.d_buf.is_null());
        assert!(unsafe{libelf::elf_rawdata(str_scn, str_data)}.is_null());

        str_data
    });
}

#[test]
fn test_elf_getdata_rawchunk() {
    getdata_test(|elf: libelf::Elf, str_scn: *mut libelf::Elf_Scn| {
        let str_shdr = unsafe {
            let str_shdr_p = libelf::elf64_getshdr(str_scn);
            assert!(!str_shdr_p.is_null());
            &*str_shdr_p
        };

        let str_data = unsafe {
            libelf::elf_getdata_rawchunk(elf, str_shdr.sh_offset as i64,
                                         str_shdr.sh_size, libelf::ELF_T_BYTE)
        };
        assert!(!str_data.is_null());

        str_data
    });
}

#[test]
fn test_elf_strptr() {
    use std::ffi::CStr;

    let (elf, _) = get_elf("../tests/empty");
    let ehdr = unsafe{libelf::elf64_getehdr(elf)};
    assert!(!ehdr.is_null());

    let sec_names = secnames_empty();

    check_sections(elf, ehdr,
                   |_: libelf::Elf, ehdr: *mut elf::Elf64_Ehdr,
                    scn: *mut libelf::Elf_Scn, shdr: *mut elf::Elf64_Shdr| {
                        let ndx = unsafe{&*ehdr}.e_shstrndx as libc::size_t;
                        let off = unsafe{&*shdr}.sh_name as libc::size_t;

                        let name = unsafe {
                            let buf = libelf::elf_strptr(elf, ndx, off);
                            let cstr = CStr::from_ptr(buf);
                            std::str::from_utf8_unchecked(cstr.to_bytes_with_nul())
                        };

                        let i = unsafe{libelf::elf_ndxscn(scn)} as usize;
                        assert_eq!(name, sec_names[i]);
                    });
}

#[test]
fn test_archives() {
    use std::os::unix::io::AsRawFd;
    use std::ffi::CStr;

    let (elf, f) = get_elf("../tests/x.ar");
    let fd = f.as_raw_fd();

    let names = ["/", "x.o", "x2.o"];
    let offsets = [8, 72, 1404];

    let mut i = 0;
    let mut cmd = libelf::ELF_C_READ_MMAP;
    loop {
        let subelf = unsafe{libelf::elf_begin(fd, cmd, elf)};
        assert!(!subelf.is_null());

        let arhdr = unsafe{libelf::elf_getarhdr(subelf)};
        assert!(!arhdr.is_null());

        let cname = unsafe{CStr::from_ptr((*arhdr).ar_name)};
        let name = unsafe{std::str::from_utf8_unchecked(cname.to_bytes())};
        assert_eq!(name, names[i]);

        let off = unsafe{libelf::elf_getaroff(subelf)};
        assert_eq!(off, offsets[i]);

        i += 1;
        cmd = unsafe{libelf::elf_next(subelf)};
        if cmd == libelf::ELF_C_NULL {
            break;
        }
    }

    assert_eq!(i, 3);
}

#[test]
fn test_elf_getarsym() {
    use std::ffi::CStr;

    let (elf, _) = get_elf("../tests/x2.ar");
    let syms = unsafe {
        let mut n = 0;
        let buf = libelf::elf_getarsym(elf, &mut n);
        assert_eq!(n, 4);

        // The last one, however, is expected to be a fake member.
        assert!((*buf.offset(3)).as_name.is_null());
        n -= 1;

        std::slice::from_raw_parts(buf, n as usize)
    };
    let names = ["foo", "bar", "baz"];
    for i in 0..names.len() {
        let name = unsafe {
            let buf = syms[i].as_name;
            assert!(!buf.is_null());
            std::str::from_utf8_unchecked(CStr::from_ptr(buf).to_bytes())
        };
        assert_eq!(name, names[i]);
    }
}
