#![feature(main)]
#![feature(linkage)]
#![feature(extern_types)]
use ::libc;
use rust::src::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /*
    Copyright 2011 Google Inc. All Rights Reserved.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

    Author: lode.vandevenne@gmail.com (Lode Vandevenne)
    Author: jyrki.alakuijala@gmail.com (Jyrki Alakuijala)
    */
    /* for size_t */
    /*
    Options used throughout the program.
    */
    /* Whether to print output */
    /* Whether to print more detailed output */
    /*
    Maximum amount of times to rerun forward and backward pass to optimize LZ77
    compression cost. Good values: 10, 15 for small files, 5 for files over
    several MB in size or it will be too slow.
    */
    /*
    If true, splits the data in multiple deflate blocks with optimal choice
    for the block boundaries. Block splitting gives better compression. Default:
    true (1).
    */
    /*
    No longer used, left for compatibility.
    */
    /*
    Maximum amount of blocks to split into (0 for unlimited, but this can give
    extreme results that hurt compression on some files). Default value: 15.
    */
    /* Initializes options with default values. */
    #[no_mangle]
    fn ZopfliInitOptions(options: *mut ZopfliOptions);
    /*
    Compresses according to the given output format and appends the result to the
    output.

    options: global program options
    output_type: the output format to use
    out: pointer to the dynamic output array to which the result is appended. Must
      be freed after use
    outsize: pointer to the dynamic output array size
    */
    #[no_mangle]
    fn ZopfliCompress(
        options: *const ZopfliOptions,
        output_type: ZopfliFormat,
        in_0: *const libc::c_uchar,
        insize: size_t,
        out: *mut *mut libc::c_uchar,
        outsize: *mut size_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: libc::c_int,
    pub verbose_more: libc::c_int,
    pub numiterations: libc::c_int,
    pub blocksplitting: libc::c_int,
    pub blocksplittinglast: libc::c_int,
    pub blocksplittingmax: libc::c_int,
}
/* Output format */
pub type ZopfliFormat = libc::c_uint;
pub const ZOPFLI_FORMAT_DEFLATE: ZopfliFormat = 2;
pub const ZOPFLI_FORMAT_ZLIB: ZopfliFormat = 1;
pub const ZOPFLI_FORMAT_GZIP: ZopfliFormat = 0;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
/*
Copyright 2011 Google Inc. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Author: lode.vandevenne@gmail.com (Lode Vandevenne)
Author: jyrki.alakuijala@gmail.com (Jyrki Alakuijala)
*/
/*
Zopfli compressor program. It can output gzip-, zlib- or deflate-compatible
data. By default it creates a .gz file. This tool can only compress, not
decompress. Decompression can be done by any standard gzip, zlib or deflate
decompressor.
*/
/* Windows workaround for stdout output. */
/*
Loads a file into a memory array. Returns 1 on success, 0 if file doesn't exist
or couldn't be opened.
*/
unsafe extern "C" fn LoadFile(
    mut filename: *const libc::c_char,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    *out = 0 as *mut libc::c_uchar;
    *outsize = 0 as libc::c_int as size_t;
    file = fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as libc::c_int;
    }
    fseek(file, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    *outsize = ftell(file) as size_t;
    if *outsize > 2147483647 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Files larger than 2GB are not supported.\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    rewind(file);
    *out = malloc(*outsize) as *mut libc::c_uchar;
    if *outsize != 0 && !(*out).is_null() {
        let mut testsize: size_t = fread(
            *out as *mut libc::c_void,
            1 as libc::c_int as size_t,
            *outsize,
            file,
        );
        if testsize != *outsize {
            /* It could be a directory */
            free(*out as *mut libc::c_void); /* If size is not zero, out must be allocated. */
            *out = 0 as *mut libc::c_uchar;
            *outsize = 0 as libc::c_int as size_t;
            fclose(file);
            return 0 as libc::c_int;
        }
    }
    if *outsize == 0 || !out.is_null() {
    } else {
        __assert_fail(
            b"!(*outsize) || out\x00" as *const u8 as *const libc::c_char,
            b"src/zopfli/zopfli_bin.c\x00" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"int LoadFile(const char *, unsigned char **, size_t *)\x00",
            ))
            .as_ptr(),
        );
    };
    fclose(file);
    return 1 as libc::c_int;
}
/*
Saves a file from a memory array, overwriting the file if it existed.
*/
unsafe extern "C" fn SaveFile(
    mut filename: *const libc::c_char,
    mut in_0: *const libc::c_uchar,
    mut insize: size_t,
) {
    let mut file: *mut FILE = fopen(filename, b"wb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"Error: Cannot write to output file, terminating.\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if !file.is_null() {
    } else {
        __assert_fail(
            b"file\x00" as *const u8 as *const libc::c_char,
            b"src/zopfli/zopfli_bin.c\x00" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void SaveFile(const char *, const unsigned char *, size_t)\x00",
            ))
            .as_ptr(),
        );
    };
    fwrite(
        in_0 as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
        insize,
        file,
    );
    fclose(file);
}
/*
outfilename: filename to write output to, or 0 to write to stdout instead
*/
unsafe extern "C" fn CompressFile(
    mut options: *const ZopfliOptions,
    mut output_type: ZopfliFormat,
    mut infilename: *const libc::c_char,
    mut outfilename: *const libc::c_char,
) {
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut insize: size_t = 0;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut outsize: size_t = 0 as libc::c_int as size_t;
    if LoadFile(infilename, &mut in_0, &mut insize) == 0 {
        fprintf(
            stderr,
            b"Invalid filename: %s\n\x00" as *const u8 as *const libc::c_char,
            infilename,
        );
        return;
    }
    ZopfliCompress(options, output_type, in_0, insize, &mut out, &mut outsize);
    if !outfilename.is_null() {
        SaveFile(outfilename, out, outsize);
    } else {
        fwrite(
            out as *const libc::c_void,
            1 as libc::c_int as size_t,
            outsize,
            stdout,
        );
    }
    free(out as *mut libc::c_void);
    free(in_0 as *mut libc::c_void);
}
/*
Add two strings together. Size does not matter. Result must be freed.
*/
unsafe extern "C" fn AddStrings(
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = strlen(str1).wrapping_add(strlen(str2)); /* Allocation failed. */
    let mut result: *mut libc::c_char =
        malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if result.is_null() {
        exit(-(1 as libc::c_int));
    }
    strcpy(result, str1);
    strcat(result, str2);
    return result;
}
unsafe extern "C" fn StringsEqual(
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
) -> libc::c_char {
    return (strcmp(str1, str2) == 0 as libc::c_int) as libc::c_int as libc::c_char;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut options: ZopfliOptions = ZopfliOptions {
        verbose: 0,
        verbose_more: 0,
        numiterations: 0,
        blocksplitting: 0,
        blocksplittinglast: 0,
        blocksplittingmax: 0,
    };
    let mut output_type: ZopfliFormat = ZOPFLI_FORMAT_GZIP;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut output_to_stdout: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    ZopfliInitOptions(&mut options);
    i = 1 as libc::c_int;
    while i < argc {
        let mut arg: *const libc::c_char = *argv.offset(i as isize);
        if StringsEqual(arg, b"-v\x00" as *const u8 as *const libc::c_char) != 0 {
            options.verbose = 1 as libc::c_int
        } else if StringsEqual(arg, b"-c\x00" as *const u8 as *const libc::c_char) != 0 {
            output_to_stdout = 1 as libc::c_int
        } else if StringsEqual(arg, b"--deflate\x00" as *const u8 as *const libc::c_char) != 0 {
            output_type = ZOPFLI_FORMAT_DEFLATE
        } else if StringsEqual(arg, b"--zlib\x00" as *const u8 as *const libc::c_char) != 0 {
            output_type = ZOPFLI_FORMAT_ZLIB
        } else if StringsEqual(arg, b"--gzip\x00" as *const u8 as *const libc::c_char) != 0 {
            output_type = ZOPFLI_FORMAT_GZIP
        } else if !(StringsEqual(arg, b"--splitlast\x00" as *const u8 as *const libc::c_char) != 0)
        {
            if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *arg.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *arg.offset(2 as libc::c_int as isize) as libc::c_int == 'i' as i32
                && *arg.offset(3 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *arg.offset(3 as libc::c_int as isize) as libc::c_int <= '9' as i32
            {
                options.numiterations = atoi(arg.offset(3 as libc::c_int as isize))
            } else if StringsEqual(arg, b"-h\x00" as *const u8 as *const libc::c_char) != 0 {
                fprintf(stderr,
                        b"Usage: zopfli [OPTION]... FILE...\n  -h    gives this help\n  -c    write the result on standard output, instead of disk filename + \'.gz\'\n  -v    verbose mode\n  --i#  perform # iterations (default 15). More gives more compression but is slower. Examples: --i10, --i50, --i1000\n\x00"
                            as *const u8 as *const libc::c_char);
                fprintf(stderr,
                        b"  --gzip        output to gzip format (default)\n  --zlib        output to zlib format instead of gzip\n  --deflate     output to deflate format instead of gzip\n  --splitlast   ignored, left for backwards compatibility\n\x00"
                            as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
        }
        i += 1
    }
    if options.numiterations < 1 as libc::c_int {
        fprintf(
            stderr,
            b"Error: must have 1 or more iterations\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != '-' as i32
        {
            let mut outfilename: *mut libc::c_char = 0 as *mut libc::c_char;
            filename = *argv.offset(i as isize);
            if output_to_stdout != 0 {
                outfilename = 0 as *mut libc::c_char
            } else if output_type as libc::c_uint
                == ZOPFLI_FORMAT_GZIP as libc::c_int as libc::c_uint
            {
                outfilename = AddStrings(filename, b".gz\x00" as *const u8 as *const libc::c_char)
            } else if output_type as libc::c_uint
                == ZOPFLI_FORMAT_ZLIB as libc::c_int as libc::c_uint
            {
                outfilename = AddStrings(filename, b".zlib\x00" as *const u8 as *const libc::c_char)
            } else {
                if output_type as libc::c_uint
                    == ZOPFLI_FORMAT_DEFLATE as libc::c_int as libc::c_uint
                {
                } else {
                    __assert_fail(
                        b"output_type == ZOPFLI_FORMAT_DEFLATE\x00" as *const u8
                            as *const libc::c_char,
                        b"src/zopfli/zopfli_bin.c\x00" as *const u8 as *const libc::c_char,
                        202 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                            b"int main(int, char **)\x00",
                        ))
                        .as_ptr(),
                    );
                };
                outfilename = AddStrings(
                    filename,
                    b".deflate\x00" as *const u8 as *const libc::c_char,
                )
            }
            if options.verbose != 0 && !outfilename.is_null() {
                fprintf(
                    stderr,
                    b"Saving to: %s\n\x00" as *const u8 as *const libc::c_char,
                    outfilename,
                );
            }
            CompressFile(&mut options, output_type, filename, outfilename);
            free(outfilename as *mut libc::c_void);
        }
        i += 1
    }
    if filename.is_null() {
        fprintf(
            stderr,
            b"Please provide filename\nFor help, type: %s -h\n\x00" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
    }
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
