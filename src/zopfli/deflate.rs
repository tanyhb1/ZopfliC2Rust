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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn ZopfliInitLZ77Store(data: *const libc::c_uchar,
                           store: *mut ZopfliLZ77Store);
    #[no_mangle]
    fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    #[no_mangle]
    fn ZopfliAppendLZ77Store(store: *const ZopfliLZ77Store,
                             target: *mut ZopfliLZ77Store);
    /* Gets the amount of raw bytes that this range of LZ77 symbols spans. */
    #[no_mangle]
    fn ZopfliLZ77GetByteRange(lz77: *const ZopfliLZ77Store, lstart: size_t,
                              lend: size_t) -> size_t;
    /* Gets the histogram of lit/len and dist symbols in the given range, using the
cumulative histograms, so faster than adding one by one for large range. Does
not add the one end symbol of value 256. */
    #[no_mangle]
    fn ZopfliLZ77GetHistogram(lz77: *const ZopfliLZ77Store, lstart: size_t,
                              lend: size_t, ll_counts: *mut size_t,
                              d_counts: *mut size_t);
    #[no_mangle]
    fn ZopfliInitBlockState(options: *const ZopfliOptions, blockstart: size_t,
                            blockend: size_t, add_lmc: libc::c_int,
                            s: *mut ZopfliBlockState);
    #[no_mangle]
    fn ZopfliCleanBlockState(s: *mut ZopfliBlockState);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
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
Functions to choose good boundaries for block splitting. Deflate allows encoding
the data in multiple blocks, with a separate Huffman tree for each block. The
Huffman tree itself requires some bytes to encode, so by choosing certain
blocks, you can either hurt, or enhance compression. These functions choose good
ones that enhance it.
*/
    /*
Does blocksplitting on LZ77 data.
The output splitpoints are indices in the LZ77 data.
maxblocks: set a limit to the amount of blocks. Set to 0 to mean no limit.
*/
    #[no_mangle]
    fn ZopfliBlockSplitLZ77(options: *const ZopfliOptions,
                            lz77: *const ZopfliLZ77Store, maxblocks: size_t,
                            splitpoints: *mut *mut size_t,
                            npoints: *mut size_t);
    /*
Does blocksplitting on uncompressed data.
The output splitpoints are indices in the uncompressed bytes.

options: general program options.
in: uncompressed input data
instart: where to start splitting
inend: where to end splitting (not inclusive)
maxblocks: maximum amount of blocks to split into, or 0 for no limit
splitpoints: dynamic array to put the resulting split point coordinates into.
  The coordinates are indices in the input array.
npoints: pointer to amount of splitpoints, for the dynamic array. The amount of
  blocks is the amount of splitpoitns + 1.
*/
    #[no_mangle]
    fn ZopfliBlockSplit(options: *const ZopfliOptions,
                        in_0: *const libc::c_uchar, instart: size_t,
                        inend: size_t, maxblocks: size_t,
                        splitpoints: *mut *mut size_t, npoints: *mut size_t);
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
    fn ZopfliLZ77Optimal(s: *mut ZopfliBlockState, in_0: *const libc::c_uchar,
                         instart: size_t, inend: size_t,
                         numiterations: libc::c_int,
                         store: *mut ZopfliLZ77Store);
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
    fn ZopfliLZ77OptimalFixed(s: *mut ZopfliBlockState,
                              in_0: *const libc::c_uchar, instart: size_t,
                              inend: size_t, store: *mut ZopfliLZ77Store);
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
Utilities for creating and using Huffman trees.
*/
    /*
Calculates the bitlengths for the Huffman tree, based on the counts of each
symbol.
*/
    #[no_mangle]
    fn ZopfliCalculateBitLengths(count: *const size_t, n: size_t,
                                 maxbits: libc::c_int,
                                 bitlengths: *mut libc::c_uint);
    /*
Converts a series of Huffman tree bitlengths, to the bit values of the symbols.
*/
    #[no_mangle]
    fn ZopfliLengthsToSymbols(lengths: *const libc::c_uint, n: size_t,
                              maxbits: libc::c_uint,
                              symbols: *mut libc::c_uint);
}
pub type size_t = libc::c_ulong;
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
Functions for basic LZ77 compression and utilities for the "squeeze" LZ77
compression.
*/
/*
Stores lit/length and dist pairs for LZ77.
Parameter litlens: Contains the literal symbols or length values.
Parameter dists: Contains the distances. A value is 0 to indicate that there is
no dist and the corresponding litlens value is a literal instead of a length.
Parameter size: The size of both the litlens and dists arrays.
The memory can best be managed by using ZopfliInitLZ77Store to initialize it,
ZopfliCleanLZ77Store to destroy it, and ZopfliStoreLitLenDist to append values.

*/
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
/*
Some state information for compressing a block.
This is currently a bit under-used (with mainly only the longest match cache),
but is kept for easy future expansion.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
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
/* Gets value of the extra bits for the given dist, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetDistExtraBitsValue(mut dist: libc::c_int)
 -> libc::c_int {
    if dist < 5 as libc::c_int {
        return 0 as libc::c_int
    } else {
        let mut l: libc::c_int =
            31 as libc::c_int ^
                ((dist - 1 as libc::c_int) as libc::c_uint).leading_zeros() as
                    i32; /* log2(dist - 1) */
        return dist - (1 as libc::c_int + ((1 as libc::c_int) << l)) &
                   ((1 as libc::c_int) << l - 1 as libc::c_int) -
                       1 as libc::c_int
    };
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
/* Gets value of the extra bits for the given length, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetLengthExtraBitsValue(mut l: libc::c_int)
 -> libc::c_int {
    static mut table: [libc::c_int; 259] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
         3 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         7 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         7 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int,
         8 as libc::c_int, 9 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 12 as libc::c_int, 13 as libc::c_int,
         14 as libc::c_int, 15 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         7 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int,
         10 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
         13 as libc::c_int, 14 as libc::c_int, 15 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 8 as libc::c_int,
         9 as libc::c_int, 10 as libc::c_int, 11 as libc::c_int,
         12 as libc::c_int, 13 as libc::c_int, 14 as libc::c_int,
         15 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int,
         8 as libc::c_int, 9 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 12 as libc::c_int, 13 as libc::c_int,
         14 as libc::c_int, 15 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         7 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int,
         10 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
         13 as libc::c_int, 14 as libc::c_int, 15 as libc::c_int,
         16 as libc::c_int, 17 as libc::c_int, 18 as libc::c_int,
         19 as libc::c_int, 20 as libc::c_int, 21 as libc::c_int,
         22 as libc::c_int, 23 as libc::c_int, 24 as libc::c_int,
         25 as libc::c_int, 26 as libc::c_int, 27 as libc::c_int,
         28 as libc::c_int, 29 as libc::c_int, 30 as libc::c_int,
         31 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 6 as libc::c_int, 7 as libc::c_int,
         8 as libc::c_int, 9 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 12 as libc::c_int, 13 as libc::c_int,
         14 as libc::c_int, 15 as libc::c_int, 16 as libc::c_int,
         17 as libc::c_int, 18 as libc::c_int, 19 as libc::c_int,
         20 as libc::c_int, 21 as libc::c_int, 22 as libc::c_int,
         23 as libc::c_int, 24 as libc::c_int, 25 as libc::c_int,
         26 as libc::c_int, 27 as libc::c_int, 28 as libc::c_int,
         29 as libc::c_int, 30 as libc::c_int, 31 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 8 as libc::c_int,
         9 as libc::c_int, 10 as libc::c_int, 11 as libc::c_int,
         12 as libc::c_int, 13 as libc::c_int, 14 as libc::c_int,
         15 as libc::c_int, 16 as libc::c_int, 17 as libc::c_int,
         18 as libc::c_int, 19 as libc::c_int, 20 as libc::c_int,
         21 as libc::c_int, 22 as libc::c_int, 23 as libc::c_int,
         24 as libc::c_int, 25 as libc::c_int, 26 as libc::c_int,
         27 as libc::c_int, 28 as libc::c_int, 29 as libc::c_int,
         30 as libc::c_int, 31 as libc::c_int, 0 as libc::c_int,
         1 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         4 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         7 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int,
         10 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
         13 as libc::c_int, 14 as libc::c_int, 15 as libc::c_int,
         16 as libc::c_int, 17 as libc::c_int, 18 as libc::c_int,
         19 as libc::c_int, 20 as libc::c_int, 21 as libc::c_int,
         22 as libc::c_int, 23 as libc::c_int, 24 as libc::c_int,
         25 as libc::c_int, 26 as libc::c_int, 27 as libc::c_int,
         28 as libc::c_int, 29 as libc::c_int, 30 as libc::c_int,
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
/* Gets the amount of extra bits for the given length symbol. */
unsafe extern "C" fn ZopfliGetLengthSymbolExtraBits(mut s: libc::c_int)
 -> libc::c_int {
    static mut table: [libc::c_int; 29] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
         1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 2 as libc::c_int,
         2 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int,
         4 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
         5 as libc::c_int, 0 as libc::c_int];
    return table[(s - 257 as libc::c_int) as usize];
}
/* Gets the amount of extra bits for the given distance symbol. */
unsafe extern "C" fn ZopfliGetDistSymbolExtraBits(mut s: libc::c_int)
 -> libc::c_int {
    static mut table: [libc::c_int; 30] =
        [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
         0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
         2 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int,
         3 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
         5 as libc::c_int, 5 as libc::c_int, 6 as libc::c_int,
         6 as libc::c_int, 7 as libc::c_int, 7 as libc::c_int,
         8 as libc::c_int, 8 as libc::c_int, 9 as libc::c_int,
         9 as libc::c_int, 10 as libc::c_int, 10 as libc::c_int,
         11 as libc::c_int, 11 as libc::c_int, 12 as libc::c_int,
         12 as libc::c_int, 13 as libc::c_int, 13 as libc::c_int];
    return table[s as usize];
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
bp = bitpointer, always in range [0, 7].
The outsize is number of necessary bytes to encode the bits.
Given the value of bp and the amount of bytes, the amount of bits represented
is not simply bytesize * 8 + bp because even representing one bit requires a
whole byte. It is: (bp == 0) ? (bytesize * 8) : ((bytesize - 1) * 8 + bp)
*/
unsafe extern "C" fn AddBit(mut bit: libc::c_int, mut bp: *mut libc::c_uchar,
                            mut out: *mut *mut libc::c_uchar,
                            mut outsize: *mut size_t) {
    if *bp as libc::c_int == 0 as libc::c_int {
        if *outsize &
               (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
           {
            *out =
                if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong)
                } else {
                    realloc(*out as *mut libc::c_void,
                            (*outsize).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong))
                } as *mut libc::c_uchar
        }
        *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1)
    }
    let ref mut fresh0 =
        *(*out).offset((*outsize).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) as isize);
    *fresh0 =
        (*fresh0 as libc::c_int | bit << *bp as libc::c_int) as libc::c_uchar;
    *bp =
        (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int) as
            libc::c_uchar;
}
unsafe extern "C" fn AddBits(mut symbol: libc::c_uint,
                             mut length: libc::c_uint,
                             mut bp: *mut libc::c_uchar,
                             mut out: *mut *mut libc::c_uchar,
                             mut outsize: *mut size_t) {
    /* TODO(lode): make more efficient (add more bits at once). */
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let mut bit: libc::c_uint =
            symbol >> i & 1 as libc::c_int as libc::c_uint;
        if *bp as libc::c_int == 0 as libc::c_int {
            if *outsize &
                   (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                   == 0 {
                *out =
                    if *outsize == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uchar>() as
                                   libc::c_ulong)
                    } else {
                        realloc(*out as *mut libc::c_void,
                                (*outsize).wrapping_mul(2 as libc::c_int as
                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                            as
                                                                                            libc::c_ulong))
                    } as *mut libc::c_uchar
            }
            *(*out).offset(*outsize as isize) =
                0 as libc::c_int as libc::c_uchar;
            *outsize = (*outsize).wrapping_add(1)
        }
        let ref mut fresh1 =
            *(*out).offset((*outsize).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                               isize);
        *fresh1 =
            (*fresh1 as libc::c_uint | bit << *bp as libc::c_int) as
                libc::c_uchar;
        *bp =
            (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int) as
                libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
/*
Adds bits, like AddBits, but the order is inverted. The deflate specification
uses both orders in one standard.
*/
unsafe extern "C" fn AddHuffmanBits(mut symbol: libc::c_uint,
                                    mut length: libc::c_uint,
                                    mut bp: *mut libc::c_uchar,
                                    mut out: *mut *mut libc::c_uchar,
                                    mut outsize: *mut size_t) {
    /* TODO(lode): make more efficient (add more bits at once). */
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let mut bit: libc::c_uint =
            symbol >>
                length.wrapping_sub(i).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) &
                1 as libc::c_int as libc::c_uint;
        if *bp as libc::c_int == 0 as libc::c_int {
            if *outsize &
                   (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                   == 0 {
                *out =
                    if *outsize == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uchar>() as
                                   libc::c_ulong)
                    } else {
                        realloc(*out as *mut libc::c_void,
                                (*outsize).wrapping_mul(2 as libc::c_int as
                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                            as
                                                                                            libc::c_ulong))
                    } as *mut libc::c_uchar
            }
            *(*out).offset(*outsize as isize) =
                0 as libc::c_int as libc::c_uchar;
            *outsize = (*outsize).wrapping_add(1)
        }
        let ref mut fresh2 =
            *(*out).offset((*outsize).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                               isize);
        *fresh2 =
            (*fresh2 as libc::c_uint | bit << *bp as libc::c_int) as
                libc::c_uchar;
        *bp =
            (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int) as
                libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
/*
Ensures there are at least 2 distance codes to support buggy decoders.
Zlib 1.2.1 and below have a bug where it fails if there isn't at least 1
distance code (with length > 0), even though it's valid according to the
deflate spec to have 0 distance codes. On top of that, some mobile phones
require at least two distance codes. To support these decoders too (but
potentially at the cost of a few bytes), add dummy code lengths of 1.
References to this bug can be found in the changelog of
Zlib 1.2.2 and here: http://www.jonof.id.au/forum/index.php?topic=515.0.

d_lengths: the 32 lengths of the distance codes.
*/
unsafe extern "C" fn PatchDistanceCodesForBuggyDecoders(mut d_lengths:
                                                            *mut libc::c_uint) {
    let mut num_dist_codes: libc::c_int =
        0 as libc::c_int; /* Amount of non-zero distance codes */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        if *d_lengths.offset(i as isize) != 0 { num_dist_codes += 1 }
        if num_dist_codes >= 2 as libc::c_int { return }
        i += 1
        /* Two or more codes is fine. */
    }
    if num_dist_codes == 0 as libc::c_int {
        let ref mut fresh3 = *d_lengths.offset(1 as libc::c_int as isize);
        *fresh3 = 1 as libc::c_int as libc::c_uint;
        *d_lengths.offset(0 as libc::c_int as isize) = *fresh3
    } else if num_dist_codes == 1 as libc::c_int {
        *d_lengths.offset(if *d_lengths.offset(0 as libc::c_int as isize) != 0
                             {
                              1 as libc::c_int
                          } else { 0 as libc::c_int } as isize) =
            1 as libc::c_int as libc::c_uint
    };
}
/*
Encodes the Huffman tree and returns how many bits its encoding takes. If out
is a null pointer, only returns the size and runs faster.
*/
unsafe extern "C" fn EncodeTree(mut ll_lengths: *const libc::c_uint,
                                mut d_lengths: *const libc::c_uint,
                                mut use_16: libc::c_int,
                                mut use_17: libc::c_int,
                                mut use_18: libc::c_int,
                                mut bp: *mut libc::c_uchar,
                                mut out: *mut *mut libc::c_uchar,
                                mut outsize: *mut size_t) -> size_t {
    let mut lld_total: libc::c_uint =
        0; /* Total amount of literal, length, distance codes. */
    /* Runlength encoded version of lengths of litlen and dist trees. */
    let mut rle: *mut libc::c_uint =
        0 as *mut libc::c_uint; /* Extra bits for rle values 16, 17 and 18. */
    let mut rle_bits: *mut libc::c_uint =
        0 as *mut libc::c_uint; /* Size of rle array. */
    let mut rle_size: size_t =
        0 as libc::c_int as size_t; /* Should have same value as rle_size. */
    let mut rle_bits_size: size_t =
        0 as libc::c_int as size_t; /* 286 - 257 */
    let mut hlit: libc::c_uint =
        29 as libc::c_int as
            libc::c_uint; /* 32 - 1, but gzip does not like hdist > 29.*/
    let mut hdist: libc::c_uint =
        29 as libc::c_int as libc::c_uint; /* Code length code lengths. */
    let mut hclen: libc::c_uint = 0;
    let mut hlit2: libc::c_uint = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut clcounts: [size_t; 19] = [0; 19];
    let mut clcl: [libc::c_uint; 19] = [0; 19];
    let mut clsymbols: [libc::c_uint; 19] = [0; 19];
    /* The order in which code length code lengths are encoded as per deflate. */
    static mut order: [libc::c_uint; 19] =
        [16 as libc::c_int as libc::c_uint, 17 as libc::c_int as libc::c_uint,
         18 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_uint,
         8 as libc::c_int as libc::c_uint, 7 as libc::c_int as libc::c_uint,
         9 as libc::c_int as libc::c_uint, 6 as libc::c_int as libc::c_uint,
         10 as libc::c_int as libc::c_uint, 5 as libc::c_int as libc::c_uint,
         11 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
         12 as libc::c_int as libc::c_uint, 3 as libc::c_int as libc::c_uint,
         13 as libc::c_int as libc::c_uint, 2 as libc::c_int as libc::c_uint,
         14 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
         15 as libc::c_int as libc::c_uint];
    let mut size_only: libc::c_int = out.is_null() as libc::c_int;
    let mut result_size: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 19 as libc::c_int as libc::c_ulong {
        clcounts[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1)
    }
    /* Trim zeros. */
    while hlit > 0 as libc::c_int as libc::c_uint &&
              *ll_lengths.offset((257 as libc::c_int as
                                      libc::c_uint).wrapping_add(hlit).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                     as isize) ==
                  0 as libc::c_int as libc::c_uint {
        hlit = hlit.wrapping_sub(1)
    }
    while hdist > 0 as libc::c_int as libc::c_uint &&
              *d_lengths.offset((1 as libc::c_int as
                                     libc::c_uint).wrapping_add(hdist).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                    as isize) ==
                  0 as libc::c_int as libc::c_uint {
        hdist = hdist.wrapping_sub(1)
    }
    hlit2 = hlit.wrapping_add(257 as libc::c_int as libc::c_uint);
    lld_total =
        hlit2.wrapping_add(hdist).wrapping_add(1 as libc::c_int as
                                                   libc::c_uint);
    i = 0 as libc::c_int as size_t;
    while i < lld_total as libc::c_ulong {
        /* This is an encoding of a huffman tree, so now the length is a symbol */
        let mut symbol: libc::c_uchar =
            if i < hlit2 as libc::c_ulong {
                *ll_lengths.offset(i as isize)
            } else {
                *d_lengths.offset(i.wrapping_sub(hlit2 as libc::c_ulong) as
                                      isize)
            } as libc::c_uchar;
        let mut count: libc::c_uint = 1 as libc::c_int as libc::c_uint;
        if use_16 != 0 ||
               symbol as libc::c_int == 0 as libc::c_int &&
                   (use_17 != 0 || use_18 != 0) {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < lld_total as libc::c_ulong &&
                      symbol as libc::c_uint ==
                          (if j < hlit2 as libc::c_ulong {
                               *ll_lengths.offset(j as isize)
                           } else {
                               *d_lengths.offset(j.wrapping_sub(hlit2 as
                                                                    libc::c_ulong)
                                                     as isize)
                           }) {
                count = count.wrapping_add(1);
                j = j.wrapping_add(1)
            }
        }
        i =
            (i as
                 libc::c_ulong).wrapping_add(count.wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                                 as libc::c_ulong) as size_t
                as size_t;
        /* Repetitions of zeroes */
        if symbol as libc::c_int == 0 as libc::c_int &&
               count >= 3 as libc::c_int as libc::c_uint {
            if use_18 != 0 {
                while count >= 11 as libc::c_int as libc::c_uint {
                    let mut count2: libc::c_uint =
                        if count > 138 as libc::c_int as libc::c_uint {
                            138 as libc::c_int as libc::c_uint
                        } else { count };
                    if size_only == 0 {
                        if rle_size &
                               rle_size.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) == 0 {
                            rle =
                                if rle_size ==
                                       0 as libc::c_int as libc::c_ulong {
                                    malloc(::std::mem::size_of::<libc::c_uint>()
                                               as libc::c_ulong)
                                } else {
                                    realloc(rle as *mut libc::c_void,
                                            rle_size.wrapping_mul(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                      as
                                                                                                      libc::c_ulong))
                                } as *mut libc::c_uint
                        }
                        *rle.offset(rle_size as isize) =
                            18 as libc::c_int as libc::c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        if rle_bits_size &
                               rle_bits_size.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                               == 0 {
                            rle_bits =
                                if rle_bits_size ==
                                       0 as libc::c_int as libc::c_ulong {
                                    malloc(::std::mem::size_of::<libc::c_uint>()
                                               as libc::c_ulong)
                                } else {
                                    realloc(rle_bits as *mut libc::c_void,
                                            rle_bits_size.wrapping_mul(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                           as
                                                                                                           libc::c_ulong))
                                } as *mut libc::c_uint
                        }
                        *rle_bits.offset(rle_bits_size as isize) =
                            count2.wrapping_sub(11 as libc::c_int as
                                                    libc::c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1)
                    }
                    clcounts[18 as libc::c_int as usize] =
                        clcounts[18 as libc::c_int as usize].wrapping_add(1);
                    count = count.wrapping_sub(count2)
                }
            }
            if use_17 != 0 {
                while count >= 3 as libc::c_int as libc::c_uint {
                    let mut count2_0: libc::c_uint =
                        if count > 10 as libc::c_int as libc::c_uint {
                            10 as libc::c_int as libc::c_uint
                        } else { count };
                    if size_only == 0 {
                        if rle_size &
                               rle_size.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) == 0 {
                            rle =
                                if rle_size ==
                                       0 as libc::c_int as libc::c_ulong {
                                    malloc(::std::mem::size_of::<libc::c_uint>()
                                               as libc::c_ulong)
                                } else {
                                    realloc(rle as *mut libc::c_void,
                                            rle_size.wrapping_mul(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                      as
                                                                                                      libc::c_ulong))
                                } as *mut libc::c_uint
                        }
                        *rle.offset(rle_size as isize) =
                            17 as libc::c_int as libc::c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        if rle_bits_size &
                               rle_bits_size.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                               == 0 {
                            rle_bits =
                                if rle_bits_size ==
                                       0 as libc::c_int as libc::c_ulong {
                                    malloc(::std::mem::size_of::<libc::c_uint>()
                                               as libc::c_ulong)
                                } else {
                                    realloc(rle_bits as *mut libc::c_void,
                                            rle_bits_size.wrapping_mul(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                           as
                                                                                                           libc::c_ulong))
                                } as *mut libc::c_uint
                        }
                        *rle_bits.offset(rle_bits_size as isize) =
                            count2_0.wrapping_sub(3 as libc::c_int as
                                                      libc::c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1)
                    }
                    clcounts[17 as libc::c_int as usize] =
                        clcounts[17 as libc::c_int as usize].wrapping_add(1);
                    count = count.wrapping_sub(count2_0)
                }
            }
        }
        /* Repetitions of any symbol */
        if use_16 != 0 && count >= 4 as libc::c_int as libc::c_uint {
            count =
                count.wrapping_sub(1); /* Since the first one is hardcoded. */
            clcounts[symbol as usize] =
                clcounts[symbol as usize].wrapping_add(1);
            if size_only == 0 {
                if rle_size &
                       rle_size.wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong) == 0 {
                    rle =
                        if rle_size == 0 as libc::c_int as libc::c_ulong {
                            malloc(::std::mem::size_of::<libc::c_uint>() as
                                       libc::c_ulong)
                        } else {
                            realloc(rle as *mut libc::c_void,
                                    rle_size.wrapping_mul(2 as libc::c_int as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                              as
                                                                                              libc::c_ulong))
                        } as *mut libc::c_uint
                }
                *rle.offset(rle_size as isize) = symbol as libc::c_uint;
                rle_size = rle_size.wrapping_add(1);
                if rle_bits_size &
                       rle_bits_size.wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong) == 0 {
                    rle_bits =
                        if rle_bits_size == 0 as libc::c_int as libc::c_ulong
                           {
                            malloc(::std::mem::size_of::<libc::c_uint>() as
                                       libc::c_ulong)
                        } else {
                            realloc(rle_bits as *mut libc::c_void,
                                    rle_bits_size.wrapping_mul(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                   as
                                                                                                   libc::c_ulong))
                        } as *mut libc::c_uint
                }
                *rle_bits.offset(rle_bits_size as isize) =
                    0 as libc::c_int as libc::c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1)
            }
            while count >= 3 as libc::c_int as libc::c_uint {
                let mut count2_1: libc::c_uint =
                    if count > 6 as libc::c_int as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else { count };
                if size_only == 0 {
                    if rle_size &
                           rle_size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) == 0 {
                        rle =
                            if rle_size == 0 as libc::c_int as libc::c_ulong {
                                malloc(::std::mem::size_of::<libc::c_uint>()
                                           as libc::c_ulong)
                            } else {
                                realloc(rle as *mut libc::c_void,
                                        rle_size.wrapping_mul(2 as libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                  as
                                                                                                  libc::c_ulong))
                            } as *mut libc::c_uint
                    }
                    *rle.offset(rle_size as isize) =
                        16 as libc::c_int as libc::c_uint;
                    rle_size = rle_size.wrapping_add(1);
                    if rle_bits_size &
                           rle_bits_size.wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) == 0
                       {
                        rle_bits =
                            if rle_bits_size ==
                                   0 as libc::c_int as libc::c_ulong {
                                malloc(::std::mem::size_of::<libc::c_uint>()
                                           as libc::c_ulong)
                            } else {
                                realloc(rle_bits as *mut libc::c_void,
                                        rle_bits_size.wrapping_mul(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                       as
                                                                                                       libc::c_ulong))
                            } as *mut libc::c_uint
                    }
                    *rle_bits.offset(rle_bits_size as isize) =
                        count2_1.wrapping_sub(3 as libc::c_int as
                                                  libc::c_uint);
                    rle_bits_size = rle_bits_size.wrapping_add(1)
                }
                clcounts[16 as libc::c_int as usize] =
                    clcounts[16 as libc::c_int as usize].wrapping_add(1);
                count = count.wrapping_sub(count2_1)
            }
        }
        /* No or insufficient repetition */
        clcounts[symbol as usize] =
            (clcounts[symbol as usize] as
                 libc::c_ulong).wrapping_add(count as libc::c_ulong) as size_t
                as size_t;
        while count > 0 as libc::c_int as libc::c_uint {
            if size_only == 0 {
                if rle_size &
                       rle_size.wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong) == 0 {
                    rle =
                        if rle_size == 0 as libc::c_int as libc::c_ulong {
                            malloc(::std::mem::size_of::<libc::c_uint>() as
                                       libc::c_ulong)
                        } else {
                            realloc(rle as *mut libc::c_void,
                                    rle_size.wrapping_mul(2 as libc::c_int as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                              as
                                                                                              libc::c_ulong))
                        } as *mut libc::c_uint
                }
                *rle.offset(rle_size as isize) = symbol as libc::c_uint;
                rle_size = rle_size.wrapping_add(1);
                if rle_bits_size &
                       rle_bits_size.wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong) == 0 {
                    rle_bits =
                        if rle_bits_size == 0 as libc::c_int as libc::c_ulong
                           {
                            malloc(::std::mem::size_of::<libc::c_uint>() as
                                       libc::c_ulong)
                        } else {
                            realloc(rle_bits as *mut libc::c_void,
                                    rle_bits_size.wrapping_mul(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                                   as
                                                                                                   libc::c_ulong))
                        } as *mut libc::c_uint
                }
                *rle_bits.offset(rle_bits_size as isize) =
                    0 as libc::c_int as libc::c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1)
            }
            count = count.wrapping_sub(1)
        }
        i = i.wrapping_add(1)
    }
    ZopfliCalculateBitLengths(clcounts.as_mut_ptr(),
                              19 as libc::c_int as size_t, 7 as libc::c_int,
                              clcl.as_mut_ptr());
    if size_only == 0 {
        ZopfliLengthsToSymbols(clcl.as_mut_ptr(), 19 as libc::c_int as size_t,
                               7 as libc::c_int as libc::c_uint,
                               clsymbols.as_mut_ptr());
    }
    hclen = 15 as libc::c_int as libc::c_uint;
    /* Trim zeros. */
    while hclen > 0 as libc::c_int as libc::c_uint &&
              clcounts[order[hclen.wrapping_add(4 as libc::c_int as
                                                    libc::c_uint).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                                 as usize] as usize] ==
                  0 as libc::c_int as libc::c_ulong {
        hclen = hclen.wrapping_sub(1)
    }
    if size_only == 0 {
        AddBits(hlit, 5 as libc::c_int as libc::c_uint, bp, out, outsize);
        AddBits(hdist, 5 as libc::c_int as libc::c_uint, bp, out, outsize);
        AddBits(hclen, 4 as libc::c_int as libc::c_uint, bp, out, outsize);
        i = 0 as libc::c_int as size_t;
        while i <
                  hclen.wrapping_add(4 as libc::c_int as libc::c_uint) as
                      libc::c_ulong {
            AddBits(clcl[order[i as usize] as usize],
                    3 as libc::c_int as libc::c_uint, bp, out, outsize);
            i = i.wrapping_add(1)
        }
        i = 0 as libc::c_int as size_t;
        while i < rle_size {
            let mut symbol_0: libc::c_uint =
                clsymbols[*rle.offset(i as isize) as usize];
            AddHuffmanBits(symbol_0, clcl[*rle.offset(i as isize) as usize],
                           bp, out, outsize);
            /* Extra bits. */
            if *rle.offset(i as isize) == 16 as libc::c_int as libc::c_uint {
                AddBits(*rle_bits.offset(i as isize),
                        2 as libc::c_int as libc::c_uint, bp, out,
                        outsize); /* hlit, hdist, hclen bits */
            } else if *rle.offset(i as isize) ==
                          17 as libc::c_int as libc::c_uint {
                AddBits(*rle_bits.offset(i as isize),
                        3 as libc::c_int as libc::c_uint, bp, out,
                        outsize); /* clcl bits */
            } else if *rle.offset(i as isize) ==
                          18 as libc::c_int as libc::c_uint {
                AddBits(*rle_bits.offset(i as isize),
                        7 as libc::c_int as libc::c_uint, bp, out, outsize);
            }
            i = i.wrapping_add(1)
        }
    }
    result_size =
        (result_size as
             libc::c_ulong).wrapping_add(14 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    result_size =
        (result_size as
             libc::c_ulong).wrapping_add(hclen.wrapping_add(4 as libc::c_int
                                                                as
                                                                libc::c_uint).wrapping_mul(3
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint)
                                             as libc::c_ulong) as size_t as
            size_t;
    i = 0 as libc::c_int as size_t;
    while i < 19 as libc::c_int as libc::c_ulong {
        result_size =
            (result_size as
                 libc::c_ulong).wrapping_add((clcl[i as usize] as
                                                  libc::c_ulong).wrapping_mul(clcounts[i
                                                                                           as
                                                                                           usize]))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    /* Extra bits. */
    result_size =
        (result_size as
             libc::c_ulong).wrapping_add(clcounts[16 as libc::c_int as
                                                      usize].wrapping_mul(2 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    result_size =
        (result_size as
             libc::c_ulong).wrapping_add(clcounts[17 as libc::c_int as
                                                      usize].wrapping_mul(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    result_size =
        (result_size as
             libc::c_ulong).wrapping_add(clcounts[18 as libc::c_int as
                                                      usize].wrapping_mul(7 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    /* Note: in case of "size_only" these are null pointers so no effect. */
    free(rle as *mut libc::c_void);
    free(rle_bits as *mut libc::c_void);
    return result_size;
}
unsafe extern "C" fn AddDynamicTree(mut ll_lengths: *const libc::c_uint,
                                    mut d_lengths: *const libc::c_uint,
                                    mut bp: *mut libc::c_uchar,
                                    mut out: *mut *mut libc::c_uchar,
                                    mut outsize: *mut size_t) {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0 as libc::c_int;
    let mut bestsize: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut size: size_t =
            EncodeTree(ll_lengths, d_lengths, i & 1 as libc::c_int,
                       i & 2 as libc::c_int, i & 4 as libc::c_int,
                       0 as *mut libc::c_uchar, 0 as *mut *mut libc::c_uchar,
                       0 as *mut size_t);
        if bestsize == 0 as libc::c_int as libc::c_ulong || size < bestsize {
            bestsize = size;
            best = i
        }
        i += 1
    }
    EncodeTree(ll_lengths, d_lengths, best & 1 as libc::c_int,
               best & 2 as libc::c_int, best & 4 as libc::c_int, bp, out,
               outsize);
}
/*
Gives the exact size of the tree, in bits, as it will be encoded in DEFLATE.
*/
unsafe extern "C" fn CalculateTreeSize(mut ll_lengths: *const libc::c_uint,
                                       mut d_lengths: *const libc::c_uint)
 -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut size: size_t =
            EncodeTree(ll_lengths, d_lengths, i & 1 as libc::c_int,
                       i & 2 as libc::c_int, i & 4 as libc::c_int,
                       0 as *mut libc::c_uchar, 0 as *mut *mut libc::c_uchar,
                       0 as *mut size_t);
        if result == 0 as libc::c_int as libc::c_ulong || size < result {
            result = size
        }
        i += 1
    }
    return result;
}
/*
Adds all lit/len and dist codes from the lists as huffman symbols. Does not add
end code 256. expected_data_size is the uncompressed block size, used for
assert, but you can set it to 0 to not do the assertion.
*/
unsafe extern "C" fn AddLZ77Data(mut lz77: *const ZopfliLZ77Store,
                                 mut lstart: size_t, mut lend: size_t,
                                 mut expected_data_size: size_t,
                                 mut ll_symbols: *const libc::c_uint,
                                 mut ll_lengths: *const libc::c_uint,
                                 mut d_symbols: *const libc::c_uint,
                                 mut d_lengths: *const libc::c_uint,
                                 mut bp: *mut libc::c_uchar,
                                 mut out: *mut *mut libc::c_uchar,
                                 mut outsize: *mut size_t) {
    let mut testlength: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        let mut dist: libc::c_uint =
            *(*lz77).dists.offset(i as isize) as libc::c_uint;
        let mut litlen: libc::c_uint =
            *(*lz77).litlens.offset(i as isize) as libc::c_uint;
        if dist == 0 as libc::c_int as libc::c_uint {
            if litlen < 256 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"litlen < 256\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/deflate.c\x00" as *const u8 as
                                  *const libc::c_char,
                              311 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 199],
                                                        &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
            };
            if *ll_lengths.offset(litlen as isize) >
                   0 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"ll_lengths[litlen] > 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/deflate.c\x00" as *const u8 as
                                  *const libc::c_char,
                              312 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 199],
                                                        &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
            };
            AddHuffmanBits(*ll_symbols.offset(litlen as isize),
                           *ll_lengths.offset(litlen as isize), bp, out,
                           outsize);
            testlength = testlength.wrapping_add(1)
        } else {
            let mut lls: libc::c_uint =
                ZopfliGetLengthSymbol(litlen as libc::c_int) as libc::c_uint;
            let mut ds: libc::c_uint =
                ZopfliGetDistSymbol(dist as libc::c_int) as libc::c_uint;
            if litlen >= 3 as libc::c_int as libc::c_uint &&
                   litlen <= 288 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"litlen >= 3 && litlen <= 288\x00" as *const u8
                                  as *const libc::c_char,
                              b"src/zopfli/deflate.c\x00" as *const u8 as
                                  *const libc::c_char,
                              318 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 199],
                                                        &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
            };
            if *ll_lengths.offset(lls as isize) >
                   0 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"ll_lengths[lls] > 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/deflate.c\x00" as *const u8 as
                                  *const libc::c_char,
                              319 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 199],
                                                        &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
            };
            if *d_lengths.offset(ds as isize) >
                   0 as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"d_lengths[ds] > 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/deflate.c\x00" as *const u8 as
                                  *const libc::c_char,
                              320 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 199],
                                                        &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
            };
            AddHuffmanBits(*ll_symbols.offset(lls as isize),
                           *ll_lengths.offset(lls as isize), bp, out,
                           outsize);
            AddBits(ZopfliGetLengthExtraBitsValue(litlen as libc::c_int) as
                        libc::c_uint,
                    ZopfliGetLengthExtraBits(litlen as libc::c_int) as
                        libc::c_uint, bp, out, outsize);
            AddHuffmanBits(*d_symbols.offset(ds as isize),
                           *d_lengths.offset(ds as isize), bp, out, outsize);
            AddBits(ZopfliGetDistExtraBitsValue(dist as libc::c_int) as
                        libc::c_uint,
                    ZopfliGetDistExtraBits(dist as libc::c_int) as
                        libc::c_uint, bp, out, outsize);
            testlength =
                (testlength as
                     libc::c_ulong).wrapping_add(litlen as libc::c_ulong) as
                    size_t as size_t
        }
        i = i.wrapping_add(1)
    }
    if expected_data_size == 0 as libc::c_int as libc::c_ulong ||
           testlength == expected_data_size {
    } else {
        __assert_fail(b"expected_data_size == 0 || testlength == expected_data_size\x00"
                          as *const u8 as *const libc::c_char,
                      b"src/zopfli/deflate.c\x00" as *const u8 as
                          *const libc::c_char,
                      332 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 199],
                                                &[libc::c_char; 199]>(b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
    };
}
unsafe extern "C" fn GetFixedTree(mut ll_lengths: *mut libc::c_uint,
                                  mut d_lengths: *mut libc::c_uint) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 144 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 8 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
    i = 144 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 9 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
    i = 256 as libc::c_int as size_t;
    while i < 280 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 7 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
    i = 280 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 8 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        *d_lengths.offset(i as isize) = 5 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    };
}
/*
Same as CalculateBlockSymbolSize, but for block size smaller than histogram
size.
*/
unsafe extern "C" fn CalculateBlockSymbolSizeSmall(mut ll_lengths:
                                                       *const libc::c_uint,
                                                   mut d_lengths:
                                                       *const libc::c_uint,
                                                   mut lz77:
                                                       *const ZopfliLZ77Store,
                                                   mut lstart: size_t,
                                                   mut lend: size_t)
 -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t; /*end symbol*/
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        if i < (*lz77).size {
        } else {
            __assert_fail(b"i < lz77->size\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/deflate.c\x00" as *const u8 as
                              *const libc::c_char,
                          355 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\x00")).as_ptr());
        };
        if (*(*lz77).litlens.offset(i as isize) as libc::c_int) <
               259 as libc::c_int {
        } else {
            __assert_fail(b"lz77->litlens[i] < 259\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/deflate.c\x00" as *const u8 as
                              *const libc::c_char,
                          356 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\x00")).as_ptr());
        };
        if *(*lz77).dists.offset(i as isize) as libc::c_int ==
               0 as libc::c_int {
            result =
                (result as
                     libc::c_ulong).wrapping_add(*ll_lengths.offset(*(*lz77).litlens.offset(i
                                                                                                as
                                                                                                isize)
                                                                        as
                                                                        isize)
                                                     as libc::c_ulong) as
                    size_t as size_t
        } else {
            let mut ll_symbol: libc::c_int =
                ZopfliGetLengthSymbol(*(*lz77).litlens.offset(i as isize) as
                                          libc::c_int);
            let mut d_symbol: libc::c_int =
                ZopfliGetDistSymbol(*(*lz77).dists.offset(i as isize) as
                                        libc::c_int);
            result =
                (result as
                     libc::c_ulong).wrapping_add(*ll_lengths.offset(ll_symbol
                                                                        as
                                                                        isize)
                                                     as libc::c_ulong) as
                    size_t as size_t;
            result =
                (result as
                     libc::c_ulong).wrapping_add(*d_lengths.offset(d_symbol as
                                                                       isize)
                                                     as libc::c_ulong) as
                    size_t as size_t;
            result =
                (result as
                     libc::c_ulong).wrapping_add(ZopfliGetLengthSymbolExtraBits(ll_symbol)
                                                     as libc::c_ulong) as
                    size_t as size_t;
            result =
                (result as
                     libc::c_ulong).wrapping_add(ZopfliGetDistSymbolExtraBits(d_symbol)
                                                     as libc::c_ulong) as
                    size_t as size_t
        }
        i = i.wrapping_add(1)
    }
    result =
        (result as
             libc::c_ulong).wrapping_add(*ll_lengths.offset(256 as libc::c_int
                                                                as isize) as
                                             libc::c_ulong) as size_t as
            size_t;
    return result;
}
/*
Same as CalculateBlockSymbolSize, but with the histogram provided by the caller.
*/
unsafe extern "C" fn CalculateBlockSymbolSizeGivenCounts(mut ll_counts:
                                                             *const size_t,
                                                         mut d_counts:
                                                             *const size_t,
                                                         mut ll_lengths:
                                                             *const libc::c_uint,
                                                         mut d_lengths:
                                                             *const libc::c_uint,
                                                         mut lz77:
                                                             *const ZopfliLZ77Store,
                                                         mut lstart: size_t,
                                                         mut lend: size_t)
 -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t; /*end symbol*/
    let mut i: size_t = 0;
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as
                               libc::c_ulong) > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77,
                                             lstart, lend)
    } else {
        i = 0 as libc::c_int as size_t;
        while i < 256 as libc::c_int as libc::c_ulong {
            result =
                (result as
                     libc::c_ulong).wrapping_add((*ll_lengths.offset(i as
                                                                         isize)
                                                      as
                                                      libc::c_ulong).wrapping_mul(*ll_counts.offset(i
                                                                                                        as
                                                                                                        isize)))
                    as size_t as size_t;
            i = i.wrapping_add(1)
        }
        i = 257 as libc::c_int as size_t;
        while i < 286 as libc::c_int as libc::c_ulong {
            result =
                (result as
                     libc::c_ulong).wrapping_add((*ll_lengths.offset(i as
                                                                         isize)
                                                      as
                                                      libc::c_ulong).wrapping_mul(*ll_counts.offset(i
                                                                                                        as
                                                                                                        isize)))
                    as size_t as size_t;
            result =
                (result as
                     libc::c_ulong).wrapping_add((ZopfliGetLengthSymbolExtraBits(i
                                                                                     as
                                                                                     libc::c_int)
                                                      as
                                                      libc::c_ulong).wrapping_mul(*ll_counts.offset(i
                                                                                                        as
                                                                                                        isize)))
                    as size_t as size_t;
            i = i.wrapping_add(1)
        }
        i = 0 as libc::c_int as size_t;
        while i < 30 as libc::c_int as libc::c_ulong {
            result =
                (result as
                     libc::c_ulong).wrapping_add((*d_lengths.offset(i as
                                                                        isize)
                                                      as
                                                      libc::c_ulong).wrapping_mul(*d_counts.offset(i
                                                                                                       as
                                                                                                       isize)))
                    as size_t as size_t;
            result =
                (result as
                     libc::c_ulong).wrapping_add((ZopfliGetDistSymbolExtraBits(i
                                                                                   as
                                                                                   libc::c_int)
                                                      as
                                                      libc::c_ulong).wrapping_mul(*d_counts.offset(i
                                                                                                       as
                                                                                                       isize)))
                    as size_t as size_t;
            i = i.wrapping_add(1)
        }
        result =
            (result as
                 libc::c_ulong).wrapping_add(*ll_lengths.offset(256 as
                                                                    libc::c_int
                                                                    as isize)
                                                 as libc::c_ulong) as size_t
                as size_t;
        return result
    };
}
/*
Calculates size of the part after the header and tree of an LZ77 block, in bits.
*/
unsafe extern "C" fn CalculateBlockSymbolSize(mut ll_lengths:
                                                  *const libc::c_uint,
                                              mut d_lengths:
                                                  *const libc::c_uint,
                                              mut lz77:
                                                  *const ZopfliLZ77Store,
                                              mut lstart: size_t,
                                              mut lend: size_t) -> size_t {
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as
                               libc::c_ulong) > lend {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77,
                                             lstart, lend)
    } else {
        let mut ll_counts: [size_t; 288] = [0; 288];
        let mut d_counts: [size_t; 32] = [0; 32];
        ZopfliLZ77GetHistogram(lz77, lstart, lend, ll_counts.as_mut_ptr(),
                               d_counts.as_mut_ptr());
        return CalculateBlockSymbolSizeGivenCounts(ll_counts.as_mut_ptr(),
                                                   d_counts.as_mut_ptr(),
                                                   ll_lengths, d_lengths,
                                                   lz77, lstart, lend)
    };
}
unsafe extern "C" fn AbsDiff(mut x: size_t, mut y: size_t) -> size_t {
    if x > y { return x.wrapping_sub(y) } else { return y.wrapping_sub(x) };
}
/*
Changes the population counts in a way that the consequent Huffman tree
compression, especially its rle-part, will be more likely to compress this data
more efficiently. length contains the size of the histogram.
*/
#[no_mangle]
pub unsafe extern "C" fn OptimizeHuffmanForRle(mut length: libc::c_int,
                                               mut counts: *mut size_t) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut symbol: size_t = 0;
    let mut sum: size_t = 0;
    let mut limit: size_t = 0;
    let mut good_for_rle: *mut libc::c_int = 0 as *mut libc::c_int;
    /* 1) We don't want to touch the trailing zeros. We may break the
  rules of the format by adding more data in the distance codes. */
    while length >= 0 as libc::c_int {
        if length == 0 as libc::c_int { return }
        if *counts.offset((length - 1 as libc::c_int) as isize) !=
               0 as libc::c_int as libc::c_ulong {
            break ;
        }
        length -= 1
    }
    /* 2) Let's mark all population counts that already can be encoded
  with an rle code.*/
    good_for_rle =
        malloc((length as libc::c_uint as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        *good_for_rle.offset(i as isize) = 0 as libc::c_int;
        i += 1
    }
    /* Let's not spoil any of the existing good rle codes.
  Mark any seq of 0's that is longer than 5 as a good_for_rle.
  Mark any seq of non-0's that is longer than 7 as a good_for_rle.*/
    symbol = *counts.offset(0 as libc::c_int as isize);
    stride = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length + 1 as libc::c_int {
        if i == length || *counts.offset(i as isize) != symbol {
            if symbol == 0 as libc::c_int as libc::c_ulong &&
                   stride >= 5 as libc::c_int ||
                   symbol != 0 as libc::c_int as libc::c_ulong &&
                       stride >= 7 as libc::c_int {
                k = 0 as libc::c_int;
                while k < stride {
                    *good_for_rle.offset((i - k - 1 as libc::c_int) as isize)
                        = 1 as libc::c_int;
                    k += 1
                }
            }
            stride = 1 as libc::c_int;
            if i != length { symbol = *counts.offset(i as isize) }
        } else { stride += 1 }
        i += 1
    }
    /* 3) Let's replace those population counts that lead to more rle codes. */
    stride = 0 as libc::c_int;
    limit = *counts.offset(0 as libc::c_int as isize);
    sum = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < length + 1 as libc::c_int {
        if i == length || *good_for_rle.offset(i as isize) != 0 ||
               AbsDiff(*counts.offset(i as isize), limit) >=
                   4 as libc::c_int as libc::c_ulong {
            if stride >= 4 as libc::c_int ||
                   stride >= 3 as libc::c_int &&
                       sum == 0 as libc::c_int as libc::c_ulong {
                /* The stride must end, collapse what we have, if we have enough (4). */
                let mut count: libc::c_int =
                    sum.wrapping_add((stride / 2 as libc::c_int) as
                                         libc::c_ulong).wrapping_div(stride as
                                                                         libc::c_ulong)
                        as libc::c_int;
                if count < 1 as libc::c_int { count = 1 as libc::c_int }
                if sum == 0 as libc::c_int as libc::c_ulong {
                    /* Don't make an all zeros stride to be upgraded to ones. */
                    count = 0 as libc::c_int
                }
                k = 0 as libc::c_int;
                while k < stride {
                    /* We don't want to change value at counts[i],
          that is already belonging to the next stride. Thus - 1. */
                    *counts.offset((i - k - 1 as libc::c_int) as isize) =
                        count as size_t;
                    k += 1
                }
            }
            stride = 0 as libc::c_int;
            sum = 0 as libc::c_int as size_t;
            if i < length - 3 as libc::c_int {
                /* All interesting strides have a count of at least 4,
        at least when non-zeros. */
                limit =
                    (*counts.offset(i as
                                        isize)).wrapping_add(*counts.offset((i
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)).wrapping_add(*counts.offset((i
                                                                                                                         +
                                                                                                                         2
                                                                                                                             as
                                                                                                                             libc::c_int)
                                                                                                                        as
                                                                                                                        isize)).wrapping_add(*counts.offset((i
                                                                                                                                                                 +
                                                                                                                                                                 3
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int)
                                                                                                                                                                as
                                                                                                                                                                isize)).wrapping_add(2
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_ulong).wrapping_div(4
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                                                         as
                                                                                                                                                                                                                         libc::c_ulong)
            } else if i < length {
                limit = *counts.offset(i as isize)
            } else { limit = 0 as libc::c_int as size_t }
        }
        stride += 1;
        if i != length {
            sum =
                (sum as
                     libc::c_ulong).wrapping_add(*counts.offset(i as isize))
                    as size_t as size_t
        }
        i += 1
    }
    free(good_for_rle as *mut libc::c_void);
}
/*
Tries out OptimizeHuffmanForRle for this block, if the result is smaller,
uses it, otherwise keeps the original. Returns size of encoded tree and data in
bits, not including the 3-bit block header.
*/
unsafe extern "C" fn TryOptimizeHuffmanForRle(mut lz77:
                                                  *const ZopfliLZ77Store,
                                              mut lstart: size_t,
                                              mut lend: size_t,
                                              mut ll_counts: *const size_t,
                                              mut d_counts: *const size_t,
                                              mut ll_lengths:
                                                  *mut libc::c_uint,
                                              mut d_lengths:
                                                  *mut libc::c_uint)
 -> libc::c_double {
    let mut ll_counts2: [size_t; 288] = [0; 288];
    let mut d_counts2: [size_t; 32] = [0; 32];
    let mut ll_lengths2: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths2: [libc::c_uint; 32] = [0; 32];
    let mut treesize: libc::c_double = 0.;
    let mut datasize: libc::c_double = 0.;
    let mut treesize2: libc::c_double = 0.;
    let mut datasize2: libc::c_double = 0.;
    treesize = CalculateTreeSize(ll_lengths, d_lengths) as libc::c_double;
    datasize =
        CalculateBlockSymbolSizeGivenCounts(ll_counts, d_counts, ll_lengths,
                                            d_lengths, lz77, lstart, lend) as
            libc::c_double;
    memcpy(ll_counts2.as_mut_ptr() as *mut libc::c_void,
           ll_counts as *const libc::c_void,
           ::std::mem::size_of::<[size_t; 288]>() as libc::c_ulong);
    memcpy(d_counts2.as_mut_ptr() as *mut libc::c_void,
           d_counts as *const libc::c_void,
           ::std::mem::size_of::<[size_t; 32]>() as libc::c_ulong);
    OptimizeHuffmanForRle(288 as libc::c_int, ll_counts2.as_mut_ptr());
    OptimizeHuffmanForRle(32 as libc::c_int, d_counts2.as_mut_ptr());
    ZopfliCalculateBitLengths(ll_counts2.as_mut_ptr(),
                              288 as libc::c_int as size_t, 15 as libc::c_int,
                              ll_lengths2.as_mut_ptr());
    ZopfliCalculateBitLengths(d_counts2.as_mut_ptr(),
                              32 as libc::c_int as size_t, 15 as libc::c_int,
                              d_lengths2.as_mut_ptr());
    PatchDistanceCodesForBuggyDecoders(d_lengths2.as_mut_ptr());
    treesize2 =
        CalculateTreeSize(ll_lengths2.as_mut_ptr(), d_lengths2.as_mut_ptr())
            as libc::c_double;
    datasize2 =
        CalculateBlockSymbolSizeGivenCounts(ll_counts, d_counts,
                                            ll_lengths2.as_mut_ptr(),
                                            d_lengths2.as_mut_ptr(), lz77,
                                            lstart, lend) as libc::c_double;
    if treesize2 + datasize2 < treesize + datasize {
        memcpy(ll_lengths as *mut libc::c_void,
               ll_lengths2.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_uint; 288]>() as libc::c_ulong);
        memcpy(d_lengths as *mut libc::c_void,
               d_lengths2.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong);
        return treesize2 + datasize2
    }
    return treesize + datasize;
}
/*
Calculates the bit lengths for the symbols for dynamic blocks. Chooses bit
lengths that give the smallest size of tree encoding + encoding of all the
symbols to have smallest output size. This are not necessarily the ideal Huffman
bit lengths. Returns size of encoded tree and data in bits, not including the
3-bit block header.
*/
unsafe extern "C" fn GetDynamicLengths(mut lz77: *const ZopfliLZ77Store,
                                       mut lstart: size_t, mut lend: size_t,
                                       mut ll_lengths: *mut libc::c_uint,
                                       mut d_lengths: *mut libc::c_uint)
 -> libc::c_double {
    let mut ll_counts: [size_t; 288] = [0; 288]; /* End symbol. */
    let mut d_counts: [size_t; 32] = [0; 32]; /* bfinal and btype bits */
    ZopfliLZ77GetHistogram(lz77, lstart, lend, ll_counts.as_mut_ptr(),
                           d_counts.as_mut_ptr());
    ll_counts[256 as libc::c_int as usize] = 1 as libc::c_int as size_t;
    ZopfliCalculateBitLengths(ll_counts.as_mut_ptr(),
                              288 as libc::c_int as size_t, 15 as libc::c_int,
                              ll_lengths);
    ZopfliCalculateBitLengths(d_counts.as_mut_ptr(),
                              32 as libc::c_int as size_t, 15 as libc::c_int,
                              d_lengths);
    PatchDistanceCodesForBuggyDecoders(d_lengths);
    return TryOptimizeHuffmanForRle(lz77, lstart, lend,
                                    ll_counts.as_mut_ptr(),
                                    d_counts.as_mut_ptr(), ll_lengths,
                                    d_lengths);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateBlockSize(mut lz77:
                                                      *const ZopfliLZ77Store,
                                                  mut lstart: size_t,
                                                  mut lend: size_t,
                                                  mut btype: libc::c_int)
 -> libc::c_double {
    let mut ll_lengths: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths: [libc::c_uint; 32] = [0; 32];
    let mut result: libc::c_double = 3 as libc::c_int as libc::c_double;
    if btype == 0 as libc::c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut rem: size_t =
            length.wrapping_rem(65535 as libc::c_int as libc::c_ulong);
        let mut blocks: size_t =
            length.wrapping_div(65535 as libc::c_int as
                                    libc::c_ulong).wrapping_add((if rem != 0 {
                                                                     1 as
                                                                         libc::c_int
                                                                 } else {
                                                                     0 as
                                                                         libc::c_int
                                                                 }) as
                                                                    libc::c_ulong);
        /* An uncompressed block must actually be split into multiple blocks if it's
       larger than 65535 bytes long. Eeach block header is 5 bytes: 3 bits,
       padding, LEN and NLEN (potential less padding for first one ignored). */
        return blocks.wrapping_mul(5 as libc::c_int as
                                       libc::c_ulong).wrapping_mul(8 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong).wrapping_add(length.wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                   as libc::c_double
    }
    if btype == 1 as libc::c_int {
        GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        result +=
            CalculateBlockSymbolSize(ll_lengths.as_mut_ptr(),
                                     d_lengths.as_mut_ptr(), lz77, lstart,
                                     lend) as libc::c_double
    } else {
        result +=
            GetDynamicLengths(lz77, lstart, lend, ll_lengths.as_mut_ptr(),
                              d_lengths.as_mut_ptr())
    }
    return result;
}
/*
Calculates block size in bits.
litlens: lz77 lit/lengths
dists: ll77 distances
lstart: start of block
lend: end of block (not inclusive)
*/
/*
Calculates block size in bits, automatically using the best btype.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateBlockSizeAutoType(mut lz77:
                                                              *const ZopfliLZ77Store,
                                                          mut lstart: size_t,
                                                          mut lend: size_t)
 -> libc::c_double {
    let mut uncompressedcost: libc::c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 0 as libc::c_int);
    /* Don't do the expensive fixed cost calculation for larger blocks that are
     unlikely to use it. */
    let mut fixedcost: libc::c_double =
        if (*lz77).size > 1000 as libc::c_int as libc::c_ulong {
            uncompressedcost
        } else {
            ZopfliCalculateBlockSize(lz77, lstart, lend, 1 as libc::c_int)
        };
    let mut dyncost: libc::c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 2 as libc::c_int);
    return if uncompressedcost < fixedcost && uncompressedcost < dyncost {
               uncompressedcost
           } else if fixedcost < dyncost { fixedcost } else { dyncost };
}
/* Since an uncompressed block can be max 65535 in size, it actually adds
multible blocks if needed. */
unsafe extern "C" fn AddNonCompressedBlock(mut options: *const ZopfliOptions,
                                           mut final_0: libc::c_int,
                                           mut in_0: *const libc::c_uchar,
                                           mut instart: size_t,
                                           mut inend: size_t,
                                           mut bp: *mut libc::c_uchar,
                                           mut out: *mut *mut libc::c_uchar,
                                           mut outsize: *mut size_t) {
    let mut pos: size_t = instart;
    loop  {
        let mut i: size_t = 0;
        let mut blocksize: libc::c_ushort =
            65535 as libc::c_int as libc::c_ushort;
        let mut nlen: libc::c_ushort = 0;
        let mut currentfinal: libc::c_int = 0;
        if pos.wrapping_add(blocksize as libc::c_ulong) > inend {
            blocksize = inend.wrapping_sub(pos) as libc::c_ushort
        }
        currentfinal =
            (pos.wrapping_add(blocksize as libc::c_ulong) >= inend) as
                libc::c_int;
        nlen = !(blocksize as libc::c_int) as libc::c_ushort;
        AddBit((final_0 != 0 && currentfinal != 0) as libc::c_int, bp, out,
               outsize);
        /* BTYPE 00 */
        AddBit(0 as libc::c_int, bp, out, outsize);
        AddBit(0 as libc::c_int, bp, out, outsize);
        /* Any bits of input up to the next byte boundary are ignored. */
        *bp = 0 as libc::c_int as libc::c_uchar;
        if *outsize &
               (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
           {
            *out =
                if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong)
                } else {
                    realloc(*out as *mut libc::c_void,
                            (*outsize).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong))
                } as *mut libc::c_uchar
        }
        *(*out).offset(*outsize as isize) =
            (blocksize as libc::c_int % 256 as libc::c_int) as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize &
               (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
           {
            *out =
                if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong)
                } else {
                    realloc(*out as *mut libc::c_void,
                            (*outsize).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong))
                } as *mut libc::c_uchar
        }
        *(*out).offset(*outsize as isize) =
            (blocksize as libc::c_int / 256 as libc::c_int %
                 256 as libc::c_int) as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize &
               (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
           {
            *out =
                if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong)
                } else {
                    realloc(*out as *mut libc::c_void,
                            (*outsize).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong))
                } as *mut libc::c_uchar
        }
        *(*out).offset(*outsize as isize) =
            (nlen as libc::c_int % 256 as libc::c_int) as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        if *outsize &
               (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
           {
            *out =
                if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as
                               libc::c_ulong)
                } else {
                    realloc(*out as *mut libc::c_void,
                            (*outsize).wrapping_mul(2 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                        as
                                                                                        libc::c_ulong))
                } as *mut libc::c_uchar
        }
        *(*out).offset(*outsize as isize) =
            (nlen as libc::c_int / 256 as libc::c_int % 256 as libc::c_int) as
                libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        i = 0 as libc::c_int as size_t;
        while i < blocksize as libc::c_ulong {
            if *outsize &
                   (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                   == 0 {
                *out =
                    if *outsize == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uchar>() as
                                   libc::c_ulong)
                    } else {
                        realloc(*out as *mut libc::c_void,
                                (*outsize).wrapping_mul(2 as libc::c_int as
                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uchar>()
                                                                                            as
                                                                                            libc::c_ulong))
                    } as *mut libc::c_uchar
            }
            *(*out).offset(*outsize as isize) =
                *in_0.offset(pos.wrapping_add(i) as isize);
            *outsize = (*outsize).wrapping_add(1);
            i = i.wrapping_add(1)
        }
        if currentfinal != 0 { break ; }
        pos =
            (pos as libc::c_ulong).wrapping_add(blocksize as libc::c_ulong) as
                size_t as size_t
    };
}
/*
Adds a deflate block with the given LZ77 data to the output.
options: global program options
btype: the block type, must be 1 or 2
final: whether to set the "final" bit on this block, must be the last block
litlens: literal/length array of the LZ77 data, in the same format as in
    ZopfliLZ77Store.
dists: distance array of the LZ77 data, in the same format as in
    ZopfliLZ77Store.
lstart: where to start in the LZ77 data
lend: where to end in the LZ77 data (not inclusive)
expected_data_size: the uncompressed block size, used for assert, but you can
  set it to 0 to not do the assertion.
bp: output bit pointer
out: dynamic output array to append to
outsize: dynamic output array size
*/
unsafe extern "C" fn AddLZ77Block(mut options: *const ZopfliOptions,
                                  mut btype: libc::c_int,
                                  mut final_0: libc::c_int,
                                  mut lz77: *const ZopfliLZ77Store,
                                  mut lstart: size_t, mut lend: size_t,
                                  mut expected_data_size: size_t,
                                  mut bp: *mut libc::c_uchar,
                                  mut out: *mut *mut libc::c_uchar,
                                  mut outsize: *mut size_t) {
    let mut ll_lengths: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths: [libc::c_uint; 32] = [0; 32];
    let mut ll_symbols: [libc::c_uint; 288] = [0; 288];
    let mut d_symbols: [libc::c_uint; 32] = [0; 32];
    let mut detect_block_size: size_t = *outsize;
    let mut compressed_size: size_t = 0;
    let mut uncompressed_size: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if btype == 0 as libc::c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut pos: size_t =
            if lstart == lend {
                0 as libc::c_int as libc::c_ulong
            } else { *(*lz77).pos.offset(lstart as isize) };
        let mut end: size_t = pos.wrapping_add(length);
        AddNonCompressedBlock(options, final_0, (*lz77).data, pos, end, bp,
                              out, outsize);
        return
    }
    AddBit(final_0, bp, out, outsize);
    AddBit(btype & 1 as libc::c_int, bp, out, outsize);
    AddBit((btype & 2 as libc::c_int) >> 1 as libc::c_int, bp, out, outsize);
    if btype == 1 as libc::c_int {
        /* Fixed block. */
        GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
    } else {
        /* Dynamic block. */
        let mut detect_tree_size: libc::c_uint = 0;
        if btype == 2 as libc::c_int {
        } else {
            __assert_fail(b"btype == 2\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/deflate.c\x00" as *const u8 as
                              *const libc::c_char,
                          715 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 145],
                                                    &[libc::c_char; 145]>(b"void AddLZ77Block(const ZopfliOptions *, int, int, const ZopfliLZ77Store *, size_t, size_t, size_t, unsigned char *, unsigned char **, size_t *)\x00")).as_ptr());
        };
        GetDynamicLengths(lz77, lstart, lend, ll_lengths.as_mut_ptr(),
                          d_lengths.as_mut_ptr());
        detect_tree_size = *outsize as libc::c_uint;
        AddDynamicTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr(), bp,
                       out, outsize);
        if (*options).verbose != 0 {
            fprintf(stderr,
                    b"treesize: %d\n\x00" as *const u8 as *const libc::c_char,
                    (*outsize).wrapping_sub(detect_tree_size as libc::c_ulong)
                        as libc::c_int);
        }
    }
    ZopfliLengthsToSymbols(ll_lengths.as_mut_ptr(),
                           288 as libc::c_int as size_t,
                           15 as libc::c_int as libc::c_uint,
                           ll_symbols.as_mut_ptr());
    ZopfliLengthsToSymbols(d_lengths.as_mut_ptr(),
                           32 as libc::c_int as size_t,
                           15 as libc::c_int as libc::c_uint,
                           d_symbols.as_mut_ptr());
    detect_block_size = *outsize;
    AddLZ77Data(lz77, lstart, lend, expected_data_size,
                ll_symbols.as_mut_ptr(), ll_lengths.as_mut_ptr(),
                d_symbols.as_mut_ptr(), d_lengths.as_mut_ptr(), bp, out,
                outsize);
    /* End symbol. */
    AddHuffmanBits(ll_symbols[256 as libc::c_int as usize],
                   ll_lengths[256 as libc::c_int as usize], bp, out, outsize);
    i = lstart;
    while i < lend {
        uncompressed_size =
            (uncompressed_size as
                 libc::c_ulong).wrapping_add(if *(*lz77).dists.offset(i as
                                                                          isize)
                                                    as libc::c_int ==
                                                    0 as libc::c_int {
                                                 1 as libc::c_int
                                             } else {
                                                 *(*lz77).litlens.offset(i as
                                                                             isize)
                                                     as libc::c_int
                                             } as libc::c_ulong) as size_t as
                size_t;
        i = i.wrapping_add(1)
    }
    compressed_size = (*outsize).wrapping_sub(detect_block_size);
    if (*options).verbose != 0 {
        fprintf(stderr,
                b"compressed block size: %d (%dk) (unc: %d)\n\x00" as
                    *const u8 as *const libc::c_char,
                compressed_size as libc::c_int,
                compressed_size.wrapping_div(1024 as libc::c_int as
                                                 libc::c_ulong) as
                    libc::c_int, uncompressed_size as libc::c_int);
    };
}
unsafe extern "C" fn AddLZ77BlockAutoType(mut options: *const ZopfliOptions,
                                          mut final_0: libc::c_int,
                                          mut lz77: *const ZopfliLZ77Store,
                                          mut lstart: size_t,
                                          mut lend: size_t,
                                          mut expected_data_size: size_t,
                                          mut bp: *mut libc::c_uchar,
                                          mut out: *mut *mut libc::c_uchar,
                                          mut outsize: *mut size_t) {
    let mut uncompressedcost: libc::c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 0 as libc::c_int);
    let mut fixedcost: libc::c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 1 as libc::c_int);
    let mut dyncost: libc::c_double =
        ZopfliCalculateBlockSize(lz77, lstart, lend, 2 as libc::c_int);
    /* Whether to perform the expensive calculation of creating an optimal block
  with fixed huffman tree to check if smaller. Only do this for small blocks or
  blocks which already are pretty good with fixed huffman tree. */
    let mut expensivefixed: libc::c_int =
        ((*lz77).size < 1000 as libc::c_int as libc::c_ulong ||
             fixedcost <= dyncost * 1.1f64) as libc::c_int;
    let mut fixedstore: ZopfliLZ77Store =
        ZopfliLZ77Store{litlens: 0 as *mut libc::c_ushort,
                        dists: 0 as *mut libc::c_ushort,
                        size: 0,
                        data: 0 as *const libc::c_uchar,
                        pos: 0 as *mut size_t,
                        ll_symbol: 0 as *mut libc::c_ushort,
                        d_symbol: 0 as *mut libc::c_ushort,
                        ll_counts: 0 as *mut size_t,
                        d_counts: 0 as *mut size_t,};
    if lstart == lend {
        /* Smallest empty block is represented by fixed block */
        AddBits(final_0 as libc::c_uint, 1 as libc::c_int as libc::c_uint, bp,
                out, outsize); /* btype 01 */
        AddBits(1 as libc::c_int as libc::c_uint,
                2 as libc::c_int as libc::c_uint, bp, out,
                outsize); /* end symbol has code 0000000 */
        AddBits(0 as libc::c_int as libc::c_uint,
                7 as libc::c_int as libc::c_uint, bp, out, outsize);
        return
    }
    ZopfliInitLZ77Store((*lz77).data, &mut fixedstore);
    if expensivefixed != 0 {
        /* Recalculate the LZ77 with ZopfliLZ77OptimalFixed */
        let mut instart: size_t = *(*lz77).pos.offset(lstart as isize);
        let mut inend: size_t =
            instart.wrapping_add(ZopfliLZ77GetByteRange(lz77, lstart, lend));
        let mut s: ZopfliBlockState =
            ZopfliBlockState{options: 0 as *const ZopfliOptions,
                             lmc: 0 as *mut ZopfliLongestMatchCache,
                             blockstart: 0,
                             blockend: 0,};
        ZopfliInitBlockState(options, instart, inend, 1 as libc::c_int,
                             &mut s);
        ZopfliLZ77OptimalFixed(&mut s, (*lz77).data, instart, inend,
                               &mut fixedstore);
        fixedcost =
            ZopfliCalculateBlockSize(&mut fixedstore,
                                     0 as libc::c_int as size_t,
                                     fixedstore.size, 1 as libc::c_int);
        ZopfliCleanBlockState(&mut s);
    }
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        AddLZ77Block(options, 0 as libc::c_int, final_0, lz77, lstart, lend,
                     expected_data_size, bp, out, outsize);
    } else if fixedcost < dyncost {
        if expensivefixed != 0 {
            AddLZ77Block(options, 1 as libc::c_int, final_0, &mut fixedstore,
                         0 as libc::c_int as size_t, fixedstore.size,
                         expected_data_size, bp, out, outsize);
        } else {
            AddLZ77Block(options, 1 as libc::c_int, final_0, lz77, lstart,
                         lend, expected_data_size, bp, out, outsize);
        }
    } else {
        AddLZ77Block(options, 2 as libc::c_int, final_0, lz77, lstart, lend,
                     expected_data_size, bp, out, outsize);
    }
    ZopfliCleanLZ77Store(&mut fixedstore);
}
/*
Like ZopfliDeflate, but allows to specify start and end byte with instart and
inend. Only that part is compressed, but earlier bytes are still used for the
back window.
*/
/*
Deflate a part, to allow ZopfliDeflate() to use multiple master blocks if
needed.
It is possible to call this function multiple times in a row, shifting
instart and inend to next bytes of the data. If instart is larger than 0, then
previous bytes are used as the initial dictionary for LZ77.
This function will usually output multiple deflate blocks. If final is 1, then
the final bit will be set on the last block.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliDeflatePart(mut options: *const ZopfliOptions,
                                           mut btype: libc::c_int,
                                           mut final_0: libc::c_int,
                                           mut in_0: *const libc::c_uchar,
                                           mut instart: size_t,
                                           mut inend: size_t,
                                           mut bp: *mut libc::c_uchar,
                                           mut out: *mut *mut libc::c_uchar,
                                           mut outsize: *mut size_t) {
    let mut i: size_t = 0;
    /* byte coordinates rather than lz77 index */
    let mut splitpoints_uncompressed: *mut size_t = 0 as *mut size_t;
    let mut npoints: size_t = 0 as libc::c_int as size_t;
    let mut splitpoints: *mut size_t = 0 as *mut size_t;
    let mut totalcost: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lz77: ZopfliLZ77Store =
        ZopfliLZ77Store{litlens: 0 as *mut libc::c_ushort,
                        dists: 0 as *mut libc::c_ushort,
                        size: 0,
                        data: 0 as *const libc::c_uchar,
                        pos: 0 as *mut size_t,
                        ll_symbol: 0 as *mut libc::c_ushort,
                        d_symbol: 0 as *mut libc::c_ushort,
                        ll_counts: 0 as *mut size_t,
                        d_counts: 0 as *mut size_t,};
    /* If btype=2 is specified, it tries all block types. If a lesser btype is
  given, then however it forces that one. Neither of the lesser types needs
  block splitting as they have no dynamic huffman trees. */
    if btype == 0 as libc::c_int {
        AddNonCompressedBlock(options, final_0, in_0, instart, inend, bp, out,
                              outsize);
        return
    } else {
        if btype == 1 as libc::c_int {
            let mut store: ZopfliLZ77Store =
                ZopfliLZ77Store{litlens: 0 as *mut libc::c_ushort,
                                dists: 0 as *mut libc::c_ushort,
                                size: 0,
                                data: 0 as *const libc::c_uchar,
                                pos: 0 as *mut size_t,
                                ll_symbol: 0 as *mut libc::c_ushort,
                                d_symbol: 0 as *mut libc::c_ushort,
                                ll_counts: 0 as *mut size_t,
                                d_counts: 0 as *mut size_t,};
            let mut s: ZopfliBlockState =
                ZopfliBlockState{options: 0 as *const ZopfliOptions,
                                 lmc: 0 as *mut ZopfliLongestMatchCache,
                                 blockstart: 0,
                                 blockend: 0,};
            ZopfliInitLZ77Store(in_0, &mut store);
            ZopfliInitBlockState(options, instart, inend, 1 as libc::c_int,
                                 &mut s);
            ZopfliLZ77OptimalFixed(&mut s, in_0, instart, inend, &mut store);
            AddLZ77Block(options, btype, final_0, &mut store,
                         0 as libc::c_int as size_t, store.size,
                         0 as libc::c_int as size_t, bp, out, outsize);
            ZopfliCleanBlockState(&mut s);
            ZopfliCleanLZ77Store(&mut store);
            return
        }
    }
    if (*options).blocksplitting != 0 {
        ZopfliBlockSplit(options, in_0, instart, inend,
                         (*options).blocksplittingmax as size_t,
                         &mut splitpoints_uncompressed, &mut npoints);
        splitpoints =
            malloc((::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(npoints)) as *mut size_t
    }
    ZopfliInitLZ77Store(in_0, &mut lz77);
    i = 0 as libc::c_int as size_t;
    while i <= npoints {
        let mut start: size_t =
            if i == 0 as libc::c_int as libc::c_ulong {
                instart
            } else {
                *splitpoints_uncompressed.offset(i.wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                                     as isize)
            };
        let mut end: size_t =
            if i == npoints {
                inend
            } else { *splitpoints_uncompressed.offset(i as isize) };
        let mut s_0: ZopfliBlockState =
            ZopfliBlockState{options: 0 as *const ZopfliOptions,
                             lmc: 0 as *mut ZopfliLongestMatchCache,
                             blockstart: 0,
                             blockend: 0,};
        let mut store_0: ZopfliLZ77Store =
            ZopfliLZ77Store{litlens: 0 as *mut libc::c_ushort,
                            dists: 0 as *mut libc::c_ushort,
                            size: 0,
                            data: 0 as *const libc::c_uchar,
                            pos: 0 as *mut size_t,
                            ll_symbol: 0 as *mut libc::c_ushort,
                            d_symbol: 0 as *mut libc::c_ushort,
                            ll_counts: 0 as *mut size_t,
                            d_counts: 0 as *mut size_t,};
        ZopfliInitLZ77Store(in_0, &mut store_0);
        ZopfliInitBlockState(options, start, end, 1 as libc::c_int, &mut s_0);
        ZopfliLZ77Optimal(&mut s_0, in_0, start, end,
                          (*options).numiterations, &mut store_0);
        totalcost +=
            ZopfliCalculateBlockSizeAutoType(&mut store_0,
                                             0 as libc::c_int as size_t,
                                             store_0.size);
        ZopfliAppendLZ77Store(&mut store_0, &mut lz77);
        if i < npoints { *splitpoints.offset(i as isize) = lz77.size }
        ZopfliCleanBlockState(&mut s_0);
        ZopfliCleanLZ77Store(&mut store_0);
        i = i.wrapping_add(1)
    }
    /* Second block splitting attempt */
    if (*options).blocksplitting != 0 &&
           npoints > 1 as libc::c_int as libc::c_ulong {
        let mut splitpoints2: *mut size_t = 0 as *mut size_t;
        let mut npoints2: size_t = 0 as libc::c_int as size_t;
        let mut totalcost2: libc::c_double =
            0 as libc::c_int as libc::c_double;
        ZopfliBlockSplitLZ77(options, &mut lz77,
                             (*options).blocksplittingmax as size_t,
                             &mut splitpoints2, &mut npoints2);
        i = 0 as libc::c_int as size_t;
        while i <= npoints2 {
            let mut start_0: size_t =
                if i == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    *splitpoints2.offset(i.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong) as
                                             isize)
                };
            let mut end_0: size_t =
                if i == npoints2 {
                    lz77.size
                } else { *splitpoints2.offset(i as isize) };
            totalcost2 +=
                ZopfliCalculateBlockSizeAutoType(&mut lz77, start_0, end_0);
            i = i.wrapping_add(1)
        }
        if totalcost2 < totalcost {
            free(splitpoints as *mut libc::c_void);
            splitpoints = splitpoints2;
            npoints = npoints2
        } else { free(splitpoints2 as *mut libc::c_void); }
    }
    i = 0 as libc::c_int as size_t;
    while i <= npoints {
        let mut start_1: size_t =
            if i == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                *splitpoints.offset(i.wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                                        isize)
            };
        let mut end_1: size_t =
            if i == npoints {
                lz77.size
            } else { *splitpoints.offset(i as isize) };
        AddLZ77BlockAutoType(options,
                             (i == npoints && final_0 != 0) as libc::c_int,
                             &mut lz77, start_1, end_1,
                             0 as libc::c_int as size_t, bp, out, outsize);
        i = i.wrapping_add(1)
    }
    ZopfliCleanLZ77Store(&mut lz77);
    free(splitpoints as *mut libc::c_void);
    free(splitpoints_uncompressed as *mut libc::c_void);
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
Functions to compress according to the DEFLATE specification, using the
"squeeze" LZ77 compression backend.
*/
/*
Compresses according to the deflate specification and append the compressed
result to the output.
This function will usually output multiple deflate blocks. If final is 1, then
the final bit will be set on the last block.

options: global program options
btype: the deflate block type. Use 2 for best compression.
  -0: non compressed blocks (00)
  -1: blocks with fixed tree (01)
  -2: blocks with dynamic tree (10)
final: whether this is the last section of the input, sets the final bit to the
  last deflate block.
in: the input bytes
insize: number of input bytes
bp: bit pointer for the output array. This must initially be 0, and for
  consecutive calls must be reused (it can have values from 0-7). This is
  because deflate appends blocks as bit-based data, rather than on byte
  boundaries.
out: pointer to the dynamic output array to which the result is appended. Must
  be freed after use.
outsize: pointer to the dynamic output array size.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliDeflate(mut options: *const ZopfliOptions,
                                       mut btype: libc::c_int,
                                       mut final_0: libc::c_int,
                                       mut in_0: *const libc::c_uchar,
                                       mut insize: size_t,
                                       mut bp: *mut libc::c_uchar,
                                       mut out: *mut *mut libc::c_uchar,
                                       mut outsize: *mut size_t) {
    let mut offset: size_t = *outsize;
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop  {
        let mut masterfinal: libc::c_int =
            (i.wrapping_add(1000000 as libc::c_int as libc::c_ulong) >=
                 insize) as libc::c_int;
        let mut final2: libc::c_int =
            (final_0 != 0 && masterfinal != 0) as libc::c_int;
        let mut size: size_t =
            if masterfinal != 0 {
                insize.wrapping_sub(i)
            } else { 1000000 as libc::c_int as libc::c_ulong };
        ZopfliDeflatePart(options, btype, final2, in_0, i,
                          i.wrapping_add(size), bp, out, outsize);
        i = (i as libc::c_ulong).wrapping_add(size) as size_t as size_t;
        if !(i < insize) { break ; }
    }
    if (*options).verbose != 0 {
        fprintf(stderr,
                b"Original Size: %lu, Deflate: %lu, Compression: %f%% Removed\n\x00"
                    as *const u8 as *const libc::c_char, insize,
                (*outsize).wrapping_sub(offset),
                100.0f64 *
                    insize.wrapping_sub((*outsize).wrapping_sub(offset)) as
                        libc::c_double / insize as libc::c_double);
    };
}
