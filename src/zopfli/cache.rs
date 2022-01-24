use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut libc::c_ushort,
    pub dist: *mut libc::c_ushort,
    pub sublen: *mut libc::c_uchar,
}
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
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
#[no_mangle]
pub unsafe extern "C" fn ZopfliInitCache(mut blocksize: size_t,
                                         mut lmc:
                                             *mut ZopfliLongestMatchCache) {
    let mut i: size_t = 0;
    (*lmc).length =
        malloc((::std::mem::size_of::<libc::c_ushort>() as
                    libc::c_ulong).wrapping_mul(blocksize)) as
            *mut libc::c_ushort;
    (*lmc).dist =
        malloc((::std::mem::size_of::<libc::c_ushort>() as
                    libc::c_ulong).wrapping_mul(blocksize)) as
            *mut libc::c_ushort;
    /* Rather large amount of memory. */
    (*lmc).sublen =
        malloc(((8 as libc::c_int * 3 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(blocksize)) as
            *mut libc::c_uchar;
    if (*lmc).sublen.is_null() {
        fprintf(stderr,
                b"Error: Out of memory. Tried allocating %lu bytes of memory.\n\x00"
                    as *const u8 as *const libc::c_char,
                (8 as libc::c_int as
                     libc::c_ulong).wrapping_mul(3 as libc::c_int as
                                                     libc::c_ulong).wrapping_mul(blocksize));
        exit(1 as libc::c_int);
    }
    /* length > 0 and dist 0 is invalid combination, which indicates on purpose
  that this cache value is not filled in yet. */
    i = 0 as libc::c_int as size_t;
    while i < blocksize {
        *(*lmc).length.offset(i as isize) =
            1 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < blocksize {
        *(*lmc).dist.offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (8 as libc::c_int as
                   libc::c_ulong).wrapping_mul(blocksize).wrapping_mul(3 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
          {
        *(*lmc).sublen.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1)
    };
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
The cache that speeds up ZopfliFindLongestMatch of lz77.c.
*/
/*
Cache used by ZopfliFindLongestMatch to remember previously found length/dist
values.
This is needed because the squeeze runs will ask these values multiple times for
the same position.
Uses large amounts of memory, since it has to remember the distance belonging
to every possible shorter-than-the-best length (the so called "sublen" array).
*/
/* Initializes the ZopfliLongestMatchCache. */
/* Frees up the memory of the ZopfliLongestMatchCache. */
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanCache(mut lmc:
                                              *mut ZopfliLongestMatchCache) {
    free((*lmc).length as *mut libc::c_void);
    free((*lmc).dist as *mut libc::c_void);
    free((*lmc).sublen as *mut libc::c_void);
}
/* Stores sublen array in the cache. */
#[no_mangle]
pub unsafe extern "C" fn ZopfliSublenToCache(mut sublen:
                                                 *const libc::c_ushort,
                                             mut pos: size_t,
                                             mut length: size_t,
                                             mut lmc:
                                                 *mut ZopfliLongestMatchCache) {
    let mut i: size_t = 0;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut bestlength: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cache: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    cache =
        &mut *(*lmc).sublen.offset((8 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(pos).wrapping_mul(3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                       as isize) as *mut libc::c_uchar;
    if length < 3 as libc::c_int as libc::c_ulong { return }
    i = 3 as libc::c_int as size_t;
    while i <= length {
        if i == length ||
               *sublen.offset(i as isize) as libc::c_int !=
                   *sublen.offset(i.wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                       as libc::c_int {
            *cache.offset(j.wrapping_mul(3 as libc::c_int as libc::c_ulong) as
                              isize) =
                i.wrapping_sub(3 as libc::c_int as libc::c_ulong) as
                    libc::c_uchar;
            *cache.offset(j.wrapping_mul(3 as libc::c_int as
                                             libc::c_ulong).wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                              as isize) =
                (*sublen.offset(i as isize) as libc::c_int %
                     256 as libc::c_int) as libc::c_uchar;
            *cache.offset(j.wrapping_mul(3 as libc::c_int as
                                             libc::c_ulong).wrapping_add(2 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                              as isize) =
                ((*sublen.offset(i as isize) as libc::c_int >>
                      8 as libc::c_int) % 256 as libc::c_int) as
                    libc::c_uchar;
            bestlength = i as libc::c_uint;
            j = j.wrapping_add(1);
            if j >= 8 as libc::c_int as libc::c_ulong { break ; }
        }
        i = i.wrapping_add(1)
    }
    if j < 8 as libc::c_int as libc::c_ulong {
        if bestlength as libc::c_ulong == length {
        } else {
            __assert_fail(b"bestlength == length\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/cache.c\x00" as *const u8 as
                              *const libc::c_char,
                          79 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 92],
                                                    &[libc::c_char; 92]>(b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\x00")).as_ptr());
        };
        *cache.offset(((8 as libc::c_int - 1 as libc::c_int) *
                           3 as libc::c_int) as isize) =
            bestlength.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                libc::c_uchar
    } else {
        if bestlength as libc::c_ulong <= length {
        } else {
            __assert_fail(b"bestlength <= length\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/cache.c\x00" as *const u8 as
                              *const libc::c_char,
                          82 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 92],
                                                    &[libc::c_char; 92]>(b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\x00")).as_ptr());
        };
    }
    if bestlength == ZopfliMaxCachedSublen(lmc, pos, length) {
    } else {
        __assert_fail(b"bestlength == ZopfliMaxCachedSublen(lmc, pos, length)\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/zopfli/cache.c\x00" as *const u8 as
                          *const libc::c_char,
                      84 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 92],
                                                &[libc::c_char; 92]>(b"void ZopfliSublenToCache(const unsigned short *, size_t, size_t, ZopfliLongestMatchCache *)\x00")).as_ptr());
    };
}
/* Extracts sublen array from the cache. */
#[no_mangle]
pub unsafe extern "C" fn ZopfliCacheToSublen(mut lmc:
                                                 *const ZopfliLongestMatchCache,
                                             mut pos: size_t,
                                             mut length: size_t,
                                             mut sublen:
                                                 *mut libc::c_ushort) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut maxlength: libc::c_uint = ZopfliMaxCachedSublen(lmc, pos, length);
    let mut prevlength: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cache: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if length < 3 as libc::c_int as libc::c_ulong { return }
    cache =
        &mut *(*lmc).sublen.offset((8 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(pos).wrapping_mul(3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                       as isize) as *mut libc::c_uchar;
    j = 0 as libc::c_int as size_t;
    while j < 8 as libc::c_int as libc::c_ulong {
        let mut length_0: libc::c_uint =
            (*cache.offset(j.wrapping_mul(3 as libc::c_int as libc::c_ulong)
                               as isize) as libc::c_int + 3 as libc::c_int) as
                libc::c_uint;
        let mut dist: libc::c_uint =
            (*cache.offset(j.wrapping_mul(3 as libc::c_int as
                                              libc::c_ulong).wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                               as isize) as libc::c_int +
                 256 as libc::c_int *
                     *cache.offset(j.wrapping_mul(3 as libc::c_int as
                                                      libc::c_ulong).wrapping_add(2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                                       as isize) as libc::c_int) as
                libc::c_uint;
        i = prevlength as size_t;
        while i <= length_0 as libc::c_ulong {
            *sublen.offset(i as isize) = dist as libc::c_ushort;
            i = i.wrapping_add(1)
        }
        if length_0 == maxlength { break ; }
        prevlength = length_0.wrapping_add(1 as libc::c_int as libc::c_uint);
        j = j.wrapping_add(1)
    };
}
/* Returns the length up to which could be stored in the cache. */
/*
Returns the length up to which could be stored in the cache.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliMaxCachedSublen(mut lmc:
                                                   *const ZopfliLongestMatchCache,
                                               mut pos: size_t,
                                               mut length: size_t)
 -> libc::c_uint {
    let mut cache: *mut libc::c_uchar =
        0 as *mut libc::c_uchar; /* No sublen cached. */
    cache =
        &mut *(*lmc).sublen.offset((8 as libc::c_int as
                                        libc::c_ulong).wrapping_mul(pos).wrapping_mul(3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                       as isize) as *mut libc::c_uchar;
    if *cache.offset(1 as libc::c_int as isize) as libc::c_int ==
           0 as libc::c_int &&
           *cache.offset(2 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint
    }
    return (*cache.offset(((8 as libc::c_int - 1 as libc::c_int) *
                               3 as libc::c_int) as isize) as libc::c_int +
                3 as libc::c_int) as libc::c_uint;
}
/* ZOPFLI_LONGEST_MATCH_CACHE */
