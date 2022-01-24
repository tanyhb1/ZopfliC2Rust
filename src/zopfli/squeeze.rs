use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    /* Allocates ZopfliHash memory. */
    #[no_mangle]
    fn ZopfliAllocHash(window_size: size_t, h: *mut ZopfliHash);
    /* Resets all fields of ZopfliHash. */
    #[no_mangle]
    fn ZopfliResetHash(window_size: size_t, h: *mut ZopfliHash);
    /* Frees ZopfliHash memory. */
    #[no_mangle]
    fn ZopfliCleanHash(h: *mut ZopfliHash);
    /*
Updates the hash values based on the current position in the array. All calls
to this must be made for consecutive bytes.
*/
    #[no_mangle]
    fn ZopfliUpdateHash(array: *const libc::c_uchar, pos: size_t, end: size_t,
                        h: *mut ZopfliHash);
    /*
Prepopulates hash:
Fills in the initial values in the hash, before ZopfliUpdateHash can be used
correctly.
*/
    #[no_mangle]
    fn ZopfliWarmupHash(array: *const libc::c_uchar, pos: size_t, end: size_t,
                        h: *mut ZopfliHash);
    #[no_mangle]
    fn ZopfliInitLZ77Store(data: *const libc::c_uchar,
                           store: *mut ZopfliLZ77Store);
    #[no_mangle]
    fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    #[no_mangle]
    fn ZopfliCopyLZ77Store(source: *const ZopfliLZ77Store,
                           dest: *mut ZopfliLZ77Store);
    #[no_mangle]
    fn ZopfliStoreLitLenDist(length: libc::c_ushort, dist: libc::c_ushort,
                             pos: size_t, store: *mut ZopfliLZ77Store);
    /*
Finds the longest match (length and corresponding distance) for LZ77
compression.
Even when not using "sublen", it can be more efficient to provide an array,
because only then the caching is used.
array: the data
pos: position in the data to find the match for
size: size of the data
limit: limit length to maximum this value (default should be 258). This allows
    finding a shorter dist for that length (= less extra bits). Must be
    in the range [ZOPFLI_MIN_MATCH, ZOPFLI_MAX_MATCH].
sublen: output array of 259 elements, or null. Has, for each length, the
    smallest distance required to reach this length. Only 256 of its 259 values
    are used, the first 3 are ignored (the shortest length is 3. It is purely
    for convenience that the array is made 3 longer).
*/
    #[no_mangle]
    fn ZopfliFindLongestMatch(s: *mut ZopfliBlockState, h: *const ZopfliHash,
                              array: *const libc::c_uchar, pos: size_t,
                              size: size_t, limit: size_t,
                              sublen: *mut libc::c_ushort,
                              distance: *mut libc::c_ushort,
                              length: *mut libc::c_ushort);
    /*
Verifies if length and dist are indeed valid, only used for assertion.
*/
    #[no_mangle]
    fn ZopfliVerifyLenDist(data: *const libc::c_uchar, datasize: size_t,
                           pos: size_t, dist: libc::c_ushort,
                           length: libc::c_ushort);
    /*
Does LZ77 using an algorithm similar to gzip, with lazy matching, rather than
with the slow but better "squeeze" implementation.
The result is placed in the ZopfliLZ77Store.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
    #[no_mangle]
    fn ZopfliLZ77Greedy(s: *mut ZopfliBlockState, in_0: *const libc::c_uchar,
                        instart: size_t, inend: size_t,
                        store: *mut ZopfliLZ77Store, h: *mut ZopfliHash);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    /*
Calculates block size in bits.
litlens: lz77 lit/lengths
dists: ll77 distances
lstart: start of block
lend: end of block (not inclusive)
*/
    #[no_mangle]
    fn ZopfliCalculateBlockSize(lz77: *const ZopfliLZ77Store, lstart: size_t,
                                lend: size_t, btype: libc::c_int)
     -> libc::c_double;
    /*
Calculates the entropy of each symbol, based on the counts of each symbol. The
result is similar to the result of ZopfliCalculateBitLengths, but with the
actual theoritical bit lengths according to the entropy. Since the resulting
values are fractional, they cannot be used to encode the tree specified by
DEFLATE.
*/
    #[no_mangle]
    fn ZopfliCalculateEntropy(count: *const size_t, n: size_t,
                              bitlengths: *mut libc::c_double);
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut libc::c_ushort,
    pub dist: *mut libc::c_ushort,
    pub sublen: *mut libc::c_uchar,
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
The hash for ZopfliFindLongestMatch of lz77.c.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliHash {
    pub head: *mut libc::c_int,
    pub prev: *mut libc::c_ushort,
    pub hashval: *mut libc::c_int,
    pub val: libc::c_int,
    pub head2: *mut libc::c_int,
    pub prev2: *mut libc::c_ushort,
    pub hashval2: *mut libc::c_int,
    pub val2: libc::c_int,
    pub same: *mut libc::c_ushort,
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
/* for size_t */
/*
Options used throughout the program.
*/
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLZ77Store {
    pub litlens: *mut libc::c_ushort,
    pub dists: *mut libc::c_ushort,
    pub size: size_t,
    pub data: *const libc::c_uchar,
    pub pos: *mut size_t,
    pub ll_symbol: *mut libc::c_ushort,
    pub d_symbol: *mut libc::c_ushort,
    pub ll_counts: *mut size_t,
    pub d_counts: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolStats {
    pub litlens: [size_t; 288],
    pub dists: [size_t; 32],
    pub ll_symbols: [libc::c_double; 288],
    pub d_symbols: [libc::c_double; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub m_w: libc::c_uint,
    pub m_z: libc::c_uint,
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
Function that calculates a cost based on a model for the given LZ77 symbol.
litlen: means literal symbol if dist is 0, length otherwise.
*/
pub type CostModelFun
    =
    unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint,
                         _: *mut libc::c_void) -> libc::c_double;
/*
Copyright 2016 Google Inc. All Rights Reserved.

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
Utilities for using the lz77 symbols of the deflate spec.
*/
/* __has_builtin available in clang */
/* __builtin_clz available beginning with GCC 3.4 */
/* Gets the amount of extra bits for the given dist, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetDistExtraBits(mut dist: libc::c_int)
 -> libc::c_int {
    if dist < 5 as libc::c_int { return 0 as libc::c_int }
    return (31 as libc::c_int ^
                ((dist - 1 as libc::c_int) as libc::c_uint).leading_zeros() as
                    i32) - 1 as libc::c_int;
    /* log2(dist - 1) - 1 */
}
/* Gets the symbol for the given dist, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetDistSymbol(mut dist: libc::c_int)
 -> libc::c_int {
    if dist < 5 as libc::c_int {
        return dist - 1 as libc::c_int
    } else {
        let mut l: libc::c_int =
            31 as libc::c_int ^
                ((dist - 1 as libc::c_int) as libc::c_uint).leading_zeros() as
                    i32; /* log2(dist - 1) */
        let mut r: libc::c_int =
            dist - 1 as libc::c_int >> l - 1 as libc::c_int &
                1 as libc::c_int;
        return l * 2 as libc::c_int + r
    };
}
/* Gets the amount of extra bits for the given length, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetLengthExtraBits(mut l: libc::c_int)
 -> libc::c_int {
    static mut table: [libc::c_int; 259] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         0 as libc::c_int];
    return table[l as usize];
}
/*
Gets the symbol for the given length, cfr. the DEFLATE spec.
Returns the symbol in the range [257-285] (inclusive)
*/
unsafe extern "C" fn ZopfliGetLengthSymbol(mut l: libc::c_int)
 -> libc::c_int {
    static mut table: [libc::c_int; 259] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         257 as libc::c_int, 258 as libc::c_int, 259 as libc::c_int,
         260 as libc::c_int, 261 as libc::c_int, 262 as libc::c_int,
         263 as libc::c_int, 264 as libc::c_int, 265 as libc::c_int,
         265 as libc::c_int, 266 as libc::c_int, 266 as libc::c_int,
         267 as libc::c_int, 267 as libc::c_int, 268 as libc::c_int,
         268 as libc::c_int, 269 as libc::c_int, 269 as libc::c_int,
         269 as libc::c_int, 269 as libc::c_int, 270 as libc::c_int,
         270 as libc::c_int, 270 as libc::c_int, 270 as libc::c_int,
         271 as libc::c_int, 271 as libc::c_int, 271 as libc::c_int,
         271 as libc::c_int, 272 as libc::c_int, 272 as libc::c_int,
         272 as libc::c_int, 272 as libc::c_int, 273 as libc::c_int,
         273 as libc::c_int, 273 as libc::c_int, 273 as libc::c_int,
         273 as libc::c_int, 273 as libc::c_int, 273 as libc::c_int,
         273 as libc::c_int, 274 as libc::c_int, 274 as libc::c_int,
         274 as libc::c_int, 274 as libc::c_int, 274 as libc::c_int,
         274 as libc::c_int, 274 as libc::c_int, 274 as libc::c_int,
         275 as libc::c_int, 275 as libc::c_int, 275 as libc::c_int,
         275 as libc::c_int, 275 as libc::c_int, 275 as libc::c_int,
         275 as libc::c_int, 275 as libc::c_int, 276 as libc::c_int,
         276 as libc::c_int, 276 as libc::c_int, 276 as libc::c_int,
         276 as libc::c_int, 276 as libc::c_int, 276 as libc::c_int,
         276 as libc::c_int, 277 as libc::c_int, 277 as libc::c_int,
         277 as libc::c_int, 277 as libc::c_int, 277 as libc::c_int,
         277 as libc::c_int, 277 as libc::c_int, 277 as libc::c_int,
         277 as libc::c_int, 277 as libc::c_int, 277 as libc::c_int,
         277 as libc::c_int, 277 as libc::c_int, 277 as libc::c_int,
         277 as libc::c_int, 277 as libc::c_int, 278 as libc::c_int,
         278 as libc::c_int, 278 as libc::c_int, 278 as libc::c_int,
         278 as libc::c_int, 278 as libc::c_int, 278 as libc::c_int,
         278 as libc::c_int, 278 as libc::c_int, 278 as libc::c_int,
         278 as libc::c_int, 278 as libc::c_int, 278 as libc::c_int,
         278 as libc::c_int, 278 as libc::c_int, 278 as libc::c_int,
         279 as libc::c_int, 279 as libc::c_int, 279 as libc::c_int,
         279 as libc::c_int, 279 as libc::c_int, 279 as libc::c_int,
         279 as libc::c_int, 279 as libc::c_int, 279 as libc::c_int,
         279 as libc::c_int, 279 as libc::c_int, 279 as libc::c_int,
         279 as libc::c_int, 279 as libc::c_int, 279 as libc::c_int,
         279 as libc::c_int, 280 as libc::c_int, 280 as libc::c_int,
         280 as libc::c_int, 280 as libc::c_int, 280 as libc::c_int,
         280 as libc::c_int, 280 as libc::c_int, 280 as libc::c_int,
         280 as libc::c_int, 280 as libc::c_int, 280 as libc::c_int,
         280 as libc::c_int, 280 as libc::c_int, 280 as libc::c_int,
         280 as libc::c_int, 280 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 281 as libc::c_int, 281 as libc::c_int,
         281 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         282 as libc::c_int, 282 as libc::c_int, 282 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 283 as libc::c_int,
         283 as libc::c_int, 283 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         284 as libc::c_int, 284 as libc::c_int, 284 as libc::c_int,
         285 as libc::c_int];
    return table[l as usize];
}
/* Sets everything to 0. */
unsafe extern "C" fn InitStats(mut stats: *mut SymbolStats) {
    memset((*stats).litlens.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (288 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as
                                                libc::c_ulong));
    memset((*stats).dists.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (32 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as
                                                libc::c_ulong));
    memset((*stats).ll_symbols.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (288 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                as libc::c_ulong));
    memset((*stats).d_symbols.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (32 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                as libc::c_ulong));
}
unsafe extern "C" fn CopyStats(mut source: *mut SymbolStats,
                               mut dest: *mut SymbolStats) {
    memcpy((*dest).litlens.as_mut_ptr() as *mut libc::c_void,
           (*source).litlens.as_mut_ptr() as *const libc::c_void,
           (288 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as
                                                libc::c_ulong));
    memcpy((*dest).dists.as_mut_ptr() as *mut libc::c_void,
           (*source).dists.as_mut_ptr() as *const libc::c_void,
           (32 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as
                                                libc::c_ulong));
    memcpy((*dest).ll_symbols.as_mut_ptr() as *mut libc::c_void,
           (*source).ll_symbols.as_mut_ptr() as *const libc::c_void,
           (288 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                as libc::c_ulong));
    memcpy((*dest).d_symbols.as_mut_ptr() as *mut libc::c_void,
           (*source).d_symbols.as_mut_ptr() as *const libc::c_void,
           (32 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                as libc::c_ulong));
}
/* Adds the bit lengths. */
unsafe extern "C" fn AddWeighedStatFreqs(mut stats1: *const SymbolStats,
                                         mut w1: libc::c_double,
                                         mut stats2: *const SymbolStats,
                                         mut w2: libc::c_double,
                                         mut result: *mut SymbolStats) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        (*result).litlens[i as usize] =
            ((*stats1).litlens[i as usize] as libc::c_double * w1 +
                 (*stats2).litlens[i as usize] as libc::c_double * w2) as
                size_t;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*result).dists[i as usize] =
            ((*stats1).dists[i as usize] as libc::c_double * w1 +
                 (*stats2).dists[i as usize] as libc::c_double * w2) as
                size_t;
        i = i.wrapping_add(1)
    }
    (*result).litlens[256 as libc::c_int as usize] =
        1 as libc::c_int as size_t;
    /* End symbol. */
}
unsafe extern "C" fn InitRanState(mut state: *mut RanState) {
    (*state).m_w = 1 as libc::c_int as libc::c_uint;
    (*state).m_z = 2 as libc::c_int as libc::c_uint;
}
/* Get random number: "Multiply-With-Carry" generator of G. Marsaglia */
unsafe extern "C" fn Ran(mut state: *mut RanState) -> libc::c_uint {
    (*state).m_z =
        (36969 as libc::c_int as
             libc::c_uint).wrapping_mul((*state).m_z &
                                            65535 as libc::c_int as
                                                libc::c_uint).wrapping_add((*state).m_z
                                                                               >>
                                                                               16
                                                                                   as
                                                                                   libc::c_int);
    (*state).m_w =
        (18000 as libc::c_int as
             libc::c_uint).wrapping_mul((*state).m_w &
                                            65535 as libc::c_int as
                                                libc::c_uint).wrapping_add((*state).m_w
                                                                               >>
                                                                               16
                                                                                   as
                                                                                   libc::c_int);
    return ((*state).m_z << 16 as libc::c_int).wrapping_add((*state).m_w);
    /* 32-bit result. */
}
unsafe extern "C" fn RandomizeFreqs(mut state: *mut RanState,
                                    mut freqs: *mut size_t,
                                    mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if (Ran(state) >>
                4 as
                    libc::c_int).wrapping_rem(3 as libc::c_int as
                                                  libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
            *freqs.offset(i as isize) =
                *freqs.offset(Ran(state).wrapping_rem(n as libc::c_uint) as
                                  isize)
        }
        i += 1
    };
}
unsafe extern "C" fn RandomizeStatFreqs(mut state: *mut RanState,
                                        mut stats: *mut SymbolStats) {
    RandomizeFreqs(state, (*stats).litlens.as_mut_ptr(), 288 as libc::c_int);
    RandomizeFreqs(state, (*stats).dists.as_mut_ptr(), 32 as libc::c_int);
    (*stats).litlens[256 as libc::c_int as usize] =
        1 as libc::c_int as size_t;
    /* End symbol. */
}
unsafe extern "C" fn ClearStatFreqs(mut stats: *mut SymbolStats) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        (*stats).litlens[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*stats).dists[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1)
    };
}
/*
Cost model which should exactly match fixed tree.
type: CostModelFun
*/
unsafe extern "C" fn GetCostFixed(mut litlen: libc::c_uint,
                                  mut dist: libc::c_uint,
                                  mut unused: *mut libc::c_void)
 -> libc::c_double {
    if dist == 0 as libc::c_int as libc::c_uint {
        if litlen <= 143 as libc::c_int as libc::c_uint {
            return 8 as libc::c_int as libc::c_double
        } else { return 9 as libc::c_int as libc::c_double }
    } else {
        let mut dbits: libc::c_int =
            ZopfliGetDistExtraBits(dist as
                                       libc::c_int); /* Every dist symbol has length 5. */
        let mut lbits: libc::c_int =
            ZopfliGetLengthExtraBits(litlen as libc::c_int);
        let mut lsym: libc::c_int =
            ZopfliGetLengthSymbol(litlen as libc::c_int);
        let mut cost: libc::c_int = 0 as libc::c_int;
        if lsym <= 279 as libc::c_int {
            cost += 7 as libc::c_int
        } else { cost += 8 as libc::c_int }
        cost += 5 as libc::c_int;
        return (cost + dbits + lbits) as libc::c_double
    };
}
/*
Cost model based on symbol statistics.
type: CostModelFun
*/
unsafe extern "C" fn GetCostStat(mut litlen: libc::c_uint,
                                 mut dist: libc::c_uint,
                                 mut context: *mut libc::c_void)
 -> libc::c_double {
    let mut stats: *mut SymbolStats = context as *mut SymbolStats;
    if dist == 0 as libc::c_int as libc::c_uint {
        return (*stats).ll_symbols[litlen as usize]
    } else {
        let mut lsym: libc::c_int =
            ZopfliGetLengthSymbol(litlen as libc::c_int);
        let mut lbits: libc::c_int =
            ZopfliGetLengthExtraBits(litlen as libc::c_int);
        let mut dsym: libc::c_int = ZopfliGetDistSymbol(dist as libc::c_int);
        let mut dbits: libc::c_int =
            ZopfliGetDistExtraBits(dist as libc::c_int);
        return (lbits + dbits) as libc::c_double +
                   (*stats).ll_symbols[lsym as usize] +
                   (*stats).d_symbols[dsym as usize]
    };
}
/*
Finds the minimum possible cost this cost model can return for valid length and
distance symbols.
*/
unsafe extern "C" fn GetCostModelMinCost(mut costmodel: Option<CostModelFun>,
                                         mut costcontext: *mut libc::c_void)
 -> libc::c_double {
    let mut mincost: libc::c_double =
        0.; /* length that has lowest cost in the cost model */
    let mut bestlength: libc::c_int =
        0 as
            libc::c_int; /* distance that has lowest cost in the cost model */
    let mut bestdist: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /*
  Table of distances that have a different distance symbol in the deflate
  specification. Each value is the first distance that has a new symbol. Only
  different symbols affect the cost model so only these need to be checked.
  See RFC 1951 section 3.2.5. Compressed blocks (length and distance codes).
  */
    static mut dsymbols: [libc::c_int; 30] =
        [1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 7 as libc::c_int,
         9 as libc::c_int, 13 as libc::c_int, 17 as libc::c_int,
         25 as libc::c_int, 33 as libc::c_int, 49 as libc::c_int,
         65 as libc::c_int, 97 as libc::c_int, 129 as libc::c_int,
         193 as libc::c_int, 257 as libc::c_int, 385 as libc::c_int,
         513 as libc::c_int, 769 as libc::c_int, 1025 as libc::c_int,
         1537 as libc::c_int, 2049 as libc::c_int, 3073 as libc::c_int,
         4097 as libc::c_int, 6145 as libc::c_int, 8193 as libc::c_int,
         12289 as libc::c_int, 16385 as libc::c_int, 24577 as libc::c_int];
    mincost = 1e30f64;
    i = 3 as libc::c_int;
    while i < 259 as libc::c_int {
        let mut c: libc::c_double =
            costmodel.expect("non-null function pointer")(i as libc::c_uint,
                                                          1 as libc::c_int as
                                                              libc::c_uint,
                                                          costcontext);
        if c < mincost { bestlength = i; mincost = c }
        i += 1
    }
    mincost = 1e30f64;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        let mut c_0: libc::c_double =
            costmodel.expect("non-null function pointer")(3 as libc::c_int as
                                                              libc::c_uint,
                                                          dsymbols[i as usize]
                                                              as libc::c_uint,
                                                          costcontext);
        if c_0 < mincost { bestdist = dsymbols[i as usize]; mincost = c_0 }
        i += 1
    }
    return costmodel.expect("non-null function pointer")(bestlength as
                                                             libc::c_uint,
                                                         bestdist as
                                                             libc::c_uint,
                                                         costcontext);
}
unsafe extern "C" fn zopfli_min(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
/*
Performs the forward pass for "squeeze". Gets the most optimal length to reach
every byte from a previous byte, using cost calculations.
s: the ZopfliBlockState
in: the input data array
instart: where to start
inend: where to stop (not inclusive)
costmodel: function to calculate the cost of some lit/len/dist pair.
costcontext: abstract context for the costmodel function
length_array: output array of size (inend - instart) which will receive the best
    length to reach this byte from a previous byte.
returns the cost that was, according to the costmodel, needed to get to the end.
*/
unsafe extern "C" fn GetBestLengths(mut s: *mut ZopfliBlockState,
                                    mut in_0: *const libc::c_uchar,
                                    mut instart: size_t, mut inend: size_t,
                                    mut costmodel: Option<CostModelFun>,
                                    mut costcontext: *mut libc::c_void,
                                    mut length_array: *mut libc::c_ushort,
                                    mut h: *mut ZopfliHash,
                                    mut costs: *mut libc::c_float)
 -> libc::c_double {
    /* Best cost to get here so far. */
    let mut blocksize: size_t =
        inend.wrapping_sub(instart); /* Because it's the start. */
    let mut i: size_t =
        0 as libc::c_int as
            size_t; /* Index in the costs array and length_array. */
    let mut k: size_t = 0;
    let mut kend: size_t = 0;
    let mut leng: libc::c_ushort = 0;
    let mut dist: libc::c_ushort = 0;
    let mut sublen: [libc::c_ushort; 259] = [0; 259];
    let mut windowstart: size_t =
        if instart > 32768 as libc::c_int as libc::c_ulong {
            instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
        } else { 0 as libc::c_int as libc::c_ulong };
    let mut result: libc::c_double = 0.;
    let mut mincost: libc::c_double =
        GetCostModelMinCost(costmodel, costcontext);
    let mut mincostaddcostj: libc::c_double = 0.;
    if instart == inend { return 0 as libc::c_int as libc::c_double }
    ZopfliResetHash(32768 as libc::c_int as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1)
    }
    i = 1 as libc::c_int as size_t;
    while i < blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *costs.offset(i as isize) = 1e30f64 as libc::c_float;
        i = i.wrapping_add(1)
    }
    *costs.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_float;
    *length_array.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_ushort;
    i = instart;
    while i < inend {
        let mut j: size_t = i.wrapping_sub(instart);
        ZopfliUpdateHash(in_0, i, inend, h);
        /* If we're in a long repetition of the same character and have more than
    ZOPFLI_MAX_MATCH characters before and after our position. */
        if *(*h).same.offset((i &
                                  (32768 as libc::c_int - 1 as libc::c_int) as
                                      libc::c_ulong) as isize) as libc::c_int
               > 258 as libc::c_int * 2 as libc::c_int &&
               i >
                   instart.wrapping_add(258 as libc::c_int as
                                            libc::c_ulong).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
               &&
               i.wrapping_add((258 as libc::c_int * 2 as libc::c_int) as
                                  libc::c_ulong).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                   < inend &&
               *(*h).same.offset((i.wrapping_sub(258 as libc::c_int as
                                                     libc::c_ulong) &
                                      (32768 as libc::c_int -
                                           1 as libc::c_int) as libc::c_ulong)
                                     as isize) as libc::c_int >
                   258 as libc::c_int {
            let mut symbolcost: libc::c_double =
                costmodel.expect("non-null function pointer")(258 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint,
                                                              1 as libc::c_int
                                                                  as
                                                                  libc::c_uint,
                                                              costcontext);
            /* Set the length to reach each one to ZOPFLI_MAX_MATCH, and the cost to
      the cost corresponding to that length. Doing this, we skip
      ZOPFLI_MAX_MATCH values to avoid calling ZopfliFindLongestMatch. */
            k = 0 as libc::c_int as size_t;
            while k < 258 as libc::c_int as libc::c_ulong {
                *costs.offset(j.wrapping_add(258 as libc::c_int as
                                                 libc::c_ulong) as isize) =
                    (*costs.offset(j as isize) as libc::c_double + symbolcost)
                        as libc::c_float;
                *length_array.offset(j.wrapping_add(258 as libc::c_int as
                                                        libc::c_ulong) as
                                         isize) =
                    258 as libc::c_int as libc::c_ushort;
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
                ZopfliUpdateHash(in_0, i, inend, h);
                k = k.wrapping_add(1)
            }
        }
        ZopfliFindLongestMatch(s, h, in_0, i, inend,
                               258 as libc::c_int as size_t,
                               sublen.as_mut_ptr(), &mut dist, &mut leng);
        /* Literal. */
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) <= inend {
            let mut newCost: libc::c_double =
                costmodel.expect("non-null function pointer")(*in_0.offset(i
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_uint,
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_uint,
                                                              costcontext) +
                    *costs.offset(j as isize) as libc::c_double;
            if newCost >= 0 as libc::c_int as libc::c_double {
            } else {
                __assert_fail(b"newCost >= 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/squeeze.c\x00" as *const u8 as
                                  *const libc::c_char,
                              279 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 146],
                                                        &[libc::c_char; 146]>(b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\x00")).as_ptr());
            };
            if newCost <
                   *costs.offset(j.wrapping_add(1 as libc::c_int as
                                                    libc::c_ulong) as isize)
                       as libc::c_double {
                *costs.offset(j.wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as isize) =
                    newCost as libc::c_float;
                *length_array.offset(j.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                                         isize) =
                    1 as libc::c_int as libc::c_ushort
            }
        }
        /* Lengths. */
        kend = zopfli_min(leng as size_t, inend.wrapping_sub(i));
        mincostaddcostj =
            mincost + *costs.offset(j as isize) as libc::c_double;
        k = 3 as libc::c_int as size_t;
        while k <= kend {
            let mut newCost_0: libc::c_double = 0.;
            /* Calling the cost model is expensive, avoid this if we are already at
      the minimum possible cost that it can return. */
            if !(*costs.offset(j.wrapping_add(k) as isize) as libc::c_double
                     <= mincostaddcostj) {
                newCost_0 =
                    costmodel.expect("non-null function pointer")(k as
                                                                      libc::c_uint,
                                                                  sublen[k as
                                                                             usize]
                                                                      as
                                                                      libc::c_uint,
                                                                  costcontext)
                        + *costs.offset(j as isize) as libc::c_double;
                if newCost_0 >= 0 as libc::c_int as libc::c_double {
                } else {
                    __assert_fail(b"newCost >= 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"src/zopfli/squeeze.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  296 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 146],
                                                            &[libc::c_char; 146]>(b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\x00")).as_ptr());
                };
                if newCost_0 <
                       *costs.offset(j.wrapping_add(k) as isize) as
                           libc::c_double {
                    if k <= 258 as libc::c_int as libc::c_ulong {
                    } else {
                        __assert_fail(b"k <= ZOPFLI_MAX_MATCH\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"src/zopfli/squeeze.c\x00" as *const u8
                                          as *const libc::c_char,
                                      298 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 146],
                                                                &[libc::c_char; 146]>(b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\x00")).as_ptr());
                    };
                    *costs.offset(j.wrapping_add(k) as isize) =
                        newCost_0 as libc::c_float;
                    *length_array.offset(j.wrapping_add(k) as isize) =
                        k as libc::c_ushort
                }
            }
            k = k.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if *costs.offset(blocksize as isize) >= 0 as libc::c_int as libc::c_float
       {
    } else {
        __assert_fail(b"costs[blocksize] >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/squeeze.c\x00" as *const u8 as
                          *const libc::c_char,
                      305 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 146],
                                                &[libc::c_char; 146]>(b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\x00")).as_ptr());
    };
    result = *costs.offset(blocksize as isize) as libc::c_double;
    return result;
}
/*
Calculates the optimal path of lz77 lengths to use, from the calculated
length_array. The length_array must contain the optimal length to reach that
byte. The path will be filled with the lengths to use, so its data size will be
the amount of lz77 symbols.
*/
unsafe extern "C" fn TraceBackwards(mut size: size_t,
                                    mut length_array: *const libc::c_ushort,
                                    mut path: *mut *mut libc::c_ushort,
                                    mut pathsize: *mut size_t) {
    let mut index: size_t = size;
    if size == 0 as libc::c_int as libc::c_ulong { return }
    loop  {
        if *pathsize &
               (*pathsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) ==
               0 {
            *path =
                if *pathsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_ushort>() as
                               libc::c_ulong)
                } else {
                    realloc(*path as *mut libc::c_void,
                            (*pathsize).wrapping_mul(2 as libc::c_int as
                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                                         as
                                                                                         libc::c_ulong))
                } as *mut libc::c_ushort
        }
        *(*path).offset(*pathsize as isize) =
            *length_array.offset(index as isize);
        *pathsize = (*pathsize).wrapping_add(1);
        if *length_array.offset(index as isize) as libc::c_ulong <= index {
        } else {
            __assert_fail(b"length_array[index] <= index\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/squeeze.c\x00" as *const u8 as
                              *const libc::c_char,
                          323 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 81],
                                                    &[libc::c_char; 81]>(b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\x00")).as_ptr());
        };
        if *length_array.offset(index as isize) as libc::c_int <=
               258 as libc::c_int {
        } else {
            __assert_fail(b"length_array[index] <= ZOPFLI_MAX_MATCH\x00" as
                              *const u8 as *const libc::c_char,
                          b"src/zopfli/squeeze.c\x00" as *const u8 as
                              *const libc::c_char,
                          324 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 81],
                                                    &[libc::c_char; 81]>(b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\x00")).as_ptr());
        };
        if *length_array.offset(index as isize) as libc::c_int !=
               0 as libc::c_int {
        } else {
            __assert_fail(b"length_array[index] != 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/squeeze.c\x00" as *const u8 as
                              *const libc::c_char,
                          325 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 81],
                                                    &[libc::c_char; 81]>(b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\x00")).as_ptr());
        };
        index =
            (index as
                 libc::c_ulong).wrapping_sub(*length_array.offset(index as
                                                                      isize)
                                                 as libc::c_ulong) as size_t
                as size_t;
        if index == 0 as libc::c_int as libc::c_ulong { break ; }
    }
    /* Mirror result. */
    index = 0 as libc::c_int as size_t;
    while index < (*pathsize).wrapping_div(2 as libc::c_int as libc::c_ulong)
          {
        let mut temp: libc::c_ushort = *(*path).offset(index as isize);
        *(*path).offset(index as isize) =
            *(*path).offset((*pathsize).wrapping_sub(index).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                as isize);
        *(*path).offset((*pathsize).wrapping_sub(index).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                            as isize) = temp;
        index = index.wrapping_add(1)
    };
}
unsafe extern "C" fn FollowPath(mut s: *mut ZopfliBlockState,
                                mut in_0: *const libc::c_uchar,
                                mut instart: size_t, mut inend: size_t,
                                mut path: *mut libc::c_ushort,
                                mut pathsize: size_t,
                                mut store: *mut ZopfliLZ77Store,
                                mut h: *mut ZopfliHash) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut windowstart: size_t =
        if instart > 32768 as libc::c_int as libc::c_ulong {
            instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
        } else { 0 as libc::c_int as libc::c_ulong };
    let mut total_length_test: size_t = 0 as libc::c_int as size_t;
    if instart == inend { return }
    ZopfliResetHash(32768 as libc::c_int as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1)
    }
    pos = instart;
    i = 0 as libc::c_int as size_t;
    while i < pathsize {
        let mut length: libc::c_ushort = *path.offset(i as isize);
        let mut dummy_length: libc::c_ushort = 0;
        let mut dist: libc::c_ushort = 0;
        if pos < inend {
        } else {
            __assert_fail(b"pos < inend\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/squeeze.c\x00" as *const u8 as
                              *const libc::c_char,
                          361 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 134],
                                                    &[libc::c_char; 134]>(b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\x00")).as_ptr());
        };
        ZopfliUpdateHash(in_0, pos, inend, h);
        /* Add to output. */
        if length as libc::c_int >= 3 as libc::c_int {
            /* Get the distance by recalculating longest match. The found length
      should match the length from the path. */
            ZopfliFindLongestMatch(s, h, in_0, pos, inend, length as size_t,
                                   0 as *mut libc::c_ushort, &mut dist,
                                   &mut dummy_length);
            if !(dummy_length as libc::c_int != length as libc::c_int &&
                     length as libc::c_int > 2 as libc::c_int &&
                     dummy_length as libc::c_int > 2 as libc::c_int) {
            } else {
                __assert_fail(b"!(dummy_length != length && length > 2 && dummy_length > 2)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"src/zopfli/squeeze.c\x00" as *const u8 as
                                  *const libc::c_char,
                              371 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 134],
                                                        &[libc::c_char; 134]>(b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\x00")).as_ptr());
            };
            ZopfliVerifyLenDist(in_0, inend, pos, dist, length);
            ZopfliStoreLitLenDist(length, dist, pos, store);
            total_length_test =
                (total_length_test as
                     libc::c_ulong).wrapping_add(length as libc::c_ulong) as
                    size_t as size_t
        } else {
            length = 1 as libc::c_int as libc::c_ushort;
            ZopfliStoreLitLenDist(*in_0.offset(pos as isize) as
                                      libc::c_ushort,
                                  0 as libc::c_int as libc::c_ushort, pos,
                                  store);
            total_length_test = total_length_test.wrapping_add(1)
        }
        if pos.wrapping_add(length as libc::c_ulong) <= inend {
        } else {
            __assert_fail(b"pos + length <= inend\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/squeeze.c\x00" as *const u8 as
                              *const libc::c_char,
                          382 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 134],
                                                    &[libc::c_char; 134]>(b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\x00")).as_ptr());
        };
        j = 1 as libc::c_int as size_t;
        while j < length as libc::c_ulong {
            ZopfliUpdateHash(in_0, pos.wrapping_add(j), inend, h);
            j = j.wrapping_add(1)
        }
        pos =
            (pos as libc::c_ulong).wrapping_add(length as libc::c_ulong) as
                size_t as size_t;
        i = i.wrapping_add(1)
    };
}
/* Calculates the entropy of the statistics */
unsafe extern "C" fn CalculateStatistics(mut stats: *mut SymbolStats) {
    ZopfliCalculateEntropy((*stats).litlens.as_mut_ptr(),
                           288 as libc::c_int as size_t,
                           (*stats).ll_symbols.as_mut_ptr());
    ZopfliCalculateEntropy((*stats).dists.as_mut_ptr(),
                           32 as libc::c_int as size_t,
                           (*stats).d_symbols.as_mut_ptr());
}
/* Appends the symbol statistics from the store. */
unsafe extern "C" fn GetStatistics(mut store: *const ZopfliLZ77Store,
                                   mut stats: *mut SymbolStats) {
    let mut i: size_t = 0; /* End symbol. */
    i = 0 as libc::c_int as size_t;
    while i < (*store).size {
        if *(*store).dists.offset(i as isize) as libc::c_int ==
               0 as libc::c_int {
            (*stats).litlens[*(*store).litlens.offset(i as isize) as usize] =
                (*stats).litlens[*(*store).litlens.offset(i as isize) as
                                     usize].wrapping_add(1)
        } else {
            (*stats).litlens[ZopfliGetLengthSymbol(*(*store).litlens.offset(i
                                                                                as
                                                                                isize)
                                                       as libc::c_int) as
                                 usize] =
                (*stats).litlens[ZopfliGetLengthSymbol(*(*store).litlens.offset(i
                                                                                    as
                                                                                    isize)
                                                           as libc::c_int) as
                                     usize].wrapping_add(1);
            (*stats).dists[ZopfliGetDistSymbol(*(*store).dists.offset(i as
                                                                          isize)
                                                   as libc::c_int) as usize] =
                (*stats).dists[ZopfliGetDistSymbol(*(*store).dists.offset(i as
                                                                              isize)
                                                       as libc::c_int) as
                                   usize].wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    (*stats).litlens[256 as libc::c_int as usize] =
        1 as libc::c_int as size_t;
    CalculateStatistics(stats);
}
/*
Does a single run for ZopfliLZ77Optimal. For good compression, repeated runs
with updated statistics should be performed.
s: the block state
in: the input data array
instart: where to start
inend: where to stop (not inclusive)
path: pointer to dynamically allocated memory to store the path
pathsize: pointer to the size of the dynamic path array
length_array: array of size (inend - instart) used to store lengths
costmodel: function to use as the cost model for this squeeze run
costcontext: abstract context for the costmodel function
store: place to output the LZ77 data
returns the cost that was, according to the costmodel, needed to get to the end.
    This is not the actual cost.
*/
unsafe extern "C" fn LZ77OptimalRun(mut s: *mut ZopfliBlockState,
                                    mut in_0: *const libc::c_uchar,
                                    mut instart: size_t, mut inend: size_t,
                                    mut path: *mut *mut libc::c_ushort,
                                    mut pathsize: *mut size_t,
                                    mut length_array: *mut libc::c_ushort,
                                    mut costmodel: Option<CostModelFun>,
                                    mut costcontext: *mut libc::c_void,
                                    mut store: *mut ZopfliLZ77Store,
                                    mut h: *mut ZopfliHash,
                                    mut costs: *mut libc::c_float)
 -> libc::c_double {
    let mut cost: libc::c_double =
        GetBestLengths(s, in_0, instart, inend, costmodel, costcontext,
                       length_array, h, costs);
    free(*path as *mut libc::c_void);
    *path = 0 as *mut libc::c_ushort;
    *pathsize = 0 as libc::c_int as size_t;
    TraceBackwards(inend.wrapping_sub(instart), length_array, path, pathsize);
    FollowPath(s, in_0, instart, inend, *path, *pathsize, store, h);
    if cost < 1e30f64 {
    } else {
        __assert_fail(b"cost < ZOPFLI_LARGE_FLOAT\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/squeeze.c\x00" as *const u8 as
                          *const libc::c_char,
                      442 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 194],
                                                &[libc::c_char; 194]>(b"double LZ77OptimalRun(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short **, size_t *, unsigned short *, CostModelFun *, void *, ZopfliLZ77Store *, ZopfliHash *, float *)\x00")).as_ptr());
    };
    return cost;
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
The squeeze functions do enhanced LZ77 compression by optimal parsing with a
cost model, rather than greedily choosing the longest length or using a single
step of lazy matching like regular implementations.

Since the cost model is based on the Huffman tree that can only be calculated
after the LZ77 data is generated, there is a chicken and egg problem, and
multiple runs are done with updated cost models to converge to a better
solution.
*/
/*
Calculates lit/len and dist pairs for given data.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77Optimal(mut s: *mut ZopfliBlockState,
                                           mut in_0: *const libc::c_uchar,
                                           mut instart: size_t,
                                           mut inend: size_t,
                                           mut numiterations: libc::c_int,
                                           mut store: *mut ZopfliLZ77Store) {
    /* Dist to get to here with smallest cost. */
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut length_array: *mut libc::c_ushort =
        malloc((::std::mem::size_of::<libc::c_ushort>() as
                    libc::c_ulong).wrapping_mul(blocksize.wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)))
            as *mut libc::c_ushort;
    let mut path: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut pathsize: size_t = 0 as libc::c_int as size_t;
    let mut currentstore: ZopfliLZ77Store =
        ZopfliLZ77Store{litlens: 0 as *mut libc::c_ushort,
                        dists: 0 as *mut libc::c_ushort,
                        size: 0,
                        data: 0 as *const libc::c_uchar,
                        pos: 0 as *mut size_t,
                        ll_symbol: 0 as *mut libc::c_ushort,
                        d_symbol: 0 as *mut libc::c_ushort,
                        ll_counts: 0 as *mut size_t,
                        d_counts: 0 as *mut size_t,};
    let mut hash: ZopfliHash =
        ZopfliHash{head: 0 as *mut libc::c_int,
                   prev: 0 as *mut libc::c_ushort,
                   hashval: 0 as *mut libc::c_int,
                   val: 0,
                   head2: 0 as *mut libc::c_int,
                   prev2: 0 as *mut libc::c_ushort,
                   hashval2: 0 as *mut libc::c_int,
                   val2: 0,
                   same: 0 as *mut libc::c_ushort,};
    let mut h: *mut ZopfliHash = &mut hash;
    let mut stats: SymbolStats =
        SymbolStats{litlens: [0; 288],
                    dists: [0; 32],
                    ll_symbols: [0.; 288],
                    d_symbols: [0.; 32],};
    let mut beststats: SymbolStats =
        SymbolStats{litlens: [0; 288],
                    dists: [0; 32],
                    ll_symbols: [0.; 288],
                    d_symbols: [0.; 32],};
    let mut laststats: SymbolStats =
        SymbolStats{litlens: [0; 288],
                    dists: [0; 32],
                    ll_symbols: [0.; 288],
                    d_symbols: [0.; 32],};
    let mut i: libc::c_int = 0;
    let mut costs: *mut libc::c_float =
        malloc((::std::mem::size_of::<libc::c_float>() as
                    libc::c_ulong).wrapping_mul(blocksize.wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)))
            as *mut libc::c_float;
    let mut cost: libc::c_double = 0.;
    let mut bestcost: libc::c_double = 1e30f64;
    let mut lastcost: libc::c_double = 0 as libc::c_int as libc::c_double;
    /* Try randomizing the costs a bit once the size stabilizes. */
    let mut ran_state: RanState =
        RanState{m_w: 0, m_z: 0,}; /* Allocation failed. */
    let mut lastrandomstep: libc::c_int =
        -(1 as libc::c_int); /* Allocation failed. */
    if costs.is_null() { exit(-(1 as libc::c_int)); }
    if length_array.is_null() { exit(-(1 as libc::c_int)); }
    InitRanState(&mut ran_state);
    InitStats(&mut stats);
    ZopfliInitLZ77Store(in_0, &mut currentstore);
    ZopfliAllocHash(32768 as libc::c_int as size_t, h);
    /* Do regular deflate, then loop multiple shortest path runs, each time using
  the statistics of the previous run. */
    /* Initial run. */
    ZopfliLZ77Greedy(s, in_0, instart, inend, &mut currentstore, h);
    GetStatistics(&mut currentstore, &mut stats);
    /* Repeat statistics with each time the cost model from the previous stat
  run. */
    i = 0 as libc::c_int;
    while i < numiterations {
        ZopfliCleanLZ77Store(&mut currentstore);
        ZopfliInitLZ77Store(in_0, &mut currentstore);
        LZ77OptimalRun(s, in_0, instart, inend, &mut path, &mut pathsize,
                       length_array,
                       Some(GetCostStat as
                                unsafe extern "C" fn(_: libc::c_uint,
                                                     _: libc::c_uint,
                                                     _: *mut libc::c_void)
                                    -> libc::c_double),
                       &mut stats as *mut SymbolStats as *mut libc::c_void,
                       &mut currentstore, h, costs);
        cost =
            ZopfliCalculateBlockSize(&mut currentstore,
                                     0 as libc::c_int as size_t,
                                     currentstore.size, 2 as libc::c_int);
        if (*(*s).options).verbose_more != 0 ||
               (*(*s).options).verbose != 0 && cost < bestcost {
            fprintf(stderr,
                    b"Iteration %d: %d bit\n\x00" as *const u8 as
                        *const libc::c_char, i, cost as libc::c_int);
        }
        if cost < bestcost {
            /* Copy to the output store. */
            ZopfliCopyLZ77Store(&mut currentstore, store);
            CopyStats(&mut stats, &mut beststats);
            bestcost = cost
        }
        CopyStats(&mut stats, &mut laststats);
        ClearStatFreqs(&mut stats);
        GetStatistics(&mut currentstore, &mut stats);
        if lastrandomstep != -(1 as libc::c_int) {
            /* This makes it converge slower but better. Do it only once the
      randomness kicks in so that if the user does few iterations, it gives a
      better result sooner. */
            AddWeighedStatFreqs(&mut stats, 1.0f64, &mut laststats, 0.5f64,
                                &mut stats);
            CalculateStatistics(&mut stats);
        }
        if i > 5 as libc::c_int && cost == lastcost {
            CopyStats(&mut beststats, &mut stats);
            RandomizeStatFreqs(&mut ran_state, &mut stats);
            CalculateStatistics(&mut stats);
            lastrandomstep = i
        }
        lastcost = cost;
        i += 1
    }
    free(length_array as *mut libc::c_void);
    free(path as *mut libc::c_void);
    free(costs as *mut libc::c_void);
    ZopfliCleanLZ77Store(&mut currentstore);
    ZopfliCleanHash(h);
}
/*
Does the same as ZopfliLZ77Optimal, but optimized for the fixed tree of the
deflate standard.
The fixed tree never gives the best compression. But this gives the best
possible LZ77 encoding possible with the fixed tree.
This does not create or output any fixed tree, only LZ77 data optimized for
using with a fixed tree.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77OptimalFixed(mut s: *mut ZopfliBlockState,
                                                mut in_0:
                                                    *const libc::c_uchar,
                                                mut instart: size_t,
                                                mut inend: size_t,
                                                mut store:
                                                    *mut ZopfliLZ77Store) {
    /* Dist to get to here with smallest cost. */
    let mut blocksize: size_t =
        inend.wrapping_sub(instart); /* Allocation failed. */
    let mut length_array: *mut libc::c_ushort =
        malloc((::std::mem::size_of::<libc::c_ushort>() as
                    libc::c_ulong).wrapping_mul(blocksize.wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)))
            as *mut libc::c_ushort; /* Allocation failed. */
    let mut path: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut pathsize: size_t = 0 as libc::c_int as size_t;
    let mut hash: ZopfliHash =
        ZopfliHash{head: 0 as *mut libc::c_int,
                   prev: 0 as *mut libc::c_ushort,
                   hashval: 0 as *mut libc::c_int,
                   val: 0,
                   head2: 0 as *mut libc::c_int,
                   prev2: 0 as *mut libc::c_ushort,
                   hashval2: 0 as *mut libc::c_int,
                   val2: 0,
                   same: 0 as *mut libc::c_ushort,};
    let mut h: *mut ZopfliHash = &mut hash;
    let mut costs: *mut libc::c_float =
        malloc((::std::mem::size_of::<libc::c_float>() as
                    libc::c_ulong).wrapping_mul(blocksize.wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)))
            as *mut libc::c_float;
    if costs.is_null() { exit(-(1 as libc::c_int)); }
    if length_array.is_null() { exit(-(1 as libc::c_int)); }
    ZopfliAllocHash(32768 as libc::c_int as size_t, h);
    (*s).blockstart = instart;
    (*s).blockend = inend;
    /* Shortest path for fixed tree This one should give the shortest possible
  result for fixed tree, no repeated runs are needed since the tree is known. */
    LZ77OptimalRun(s, in_0, instart, inend, &mut path, &mut pathsize,
                   length_array,
                   Some(GetCostFixed as
                            unsafe extern "C" fn(_: libc::c_uint,
                                                 _: libc::c_uint,
                                                 _: *mut libc::c_void)
                                -> libc::c_double), 0 as *mut libc::c_void,
                   store, h, costs);
    free(length_array as *mut libc::c_void);
    free(path as *mut libc::c_void);
    free(costs as *mut libc::c_void);
    ZopfliCleanHash(h);
}
