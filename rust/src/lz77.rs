use ::libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* Initializes the ZopfliLongestMatchCache. */
    #[no_mangle]
    fn ZopfliInitCache(blocksize: size_t, lmc: *mut ZopfliLongestMatchCache);
    /* Frees up the memory of the ZopfliLongestMatchCache. */
    #[no_mangle]
    fn ZopfliCleanCache(lmc: *mut ZopfliLongestMatchCache);
    /* Stores sublen array in the cache. */
    #[no_mangle]
    fn ZopfliSublenToCache(
        sublen: *const libc::c_ushort,
        pos: size_t,
        length: size_t,
        lmc: *mut ZopfliLongestMatchCache,
    );
    /* Extracts sublen array from the cache. */
    #[no_mangle]
    fn ZopfliCacheToSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
        sublen: *mut libc::c_ushort,
    );
    /* Returns the length up to which could be stored in the cache. */
    #[no_mangle]
    fn ZopfliMaxCachedSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
    ) -> libc::c_uint;
    /* Resets all fields of ZopfliHash. */
    #[no_mangle]
    fn ZopfliResetHash(window_size: size_t, h: *mut ZopfliHash);
    /*
    Updates the hash values based on the current position in the array. All calls
    to this must be made for consecutive bytes.
    */
    #[no_mangle]
    fn ZopfliUpdateHash(array: *const libc::c_uchar, pos: size_t, end: size_t, h: *mut ZopfliHash);
    /*
    Prepopulates hash:
    Fills in the initial values in the hash, before ZopfliUpdateHash can be used
    correctly.
    */
    #[no_mangle]
    fn ZopfliWarmupHash(array: *const libc::c_uchar, pos: size_t, end: size_t, h: *mut ZopfliHash);
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
}
/* Gets the symbol for the given dist, cfr. the DEFLATE spec. */
unsafe extern "C" fn ZopfliGetDistSymbol(mut dist: libc::c_int) -> libc::c_int {
    if dist < 5 as libc::c_int {
        return dist - 1 as libc::c_int;
    } else {
        let mut l: libc::c_int =
            31 as libc::c_int ^ ((dist - 1 as libc::c_int) as libc::c_uint).leading_zeros() as i32; /* log2(dist - 1) */
        let mut r: libc::c_int = dist - 1 as libc::c_int >> l - 1 as libc::c_int & 1 as libc::c_int;
        return l * 2 as libc::c_int + r;
    };
}
/*
Gets the symbol for the given length, cfr. the DEFLATE spec.
Returns the symbol in the range [257-285] (inclusive)
*/
unsafe extern "C" fn ZopfliGetLengthSymbol(mut l: libc::c_int) -> libc::c_int {
    static mut table: [libc::c_int; 259] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        257 as libc::c_int,
        258 as libc::c_int,
        259 as libc::c_int,
        260 as libc::c_int,
        261 as libc::c_int,
        262 as libc::c_int,
        263 as libc::c_int,
        264 as libc::c_int,
        265 as libc::c_int,
        265 as libc::c_int,
        266 as libc::c_int,
        266 as libc::c_int,
        267 as libc::c_int,
        267 as libc::c_int,
        268 as libc::c_int,
        268 as libc::c_int,
        269 as libc::c_int,
        269 as libc::c_int,
        269 as libc::c_int,
        269 as libc::c_int,
        270 as libc::c_int,
        270 as libc::c_int,
        270 as libc::c_int,
        270 as libc::c_int,
        271 as libc::c_int,
        271 as libc::c_int,
        271 as libc::c_int,
        271 as libc::c_int,
        272 as libc::c_int,
        272 as libc::c_int,
        272 as libc::c_int,
        272 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        273 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        274 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        275 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        276 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        277 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        278 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        279 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        280 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        281 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        282 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        283 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        284 as libc::c_int,
        285 as libc::c_int,
    ];
    return table[l as usize];
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
#[no_mangle]
pub unsafe extern "C" fn ZopfliInitLZ77Store(
    mut data: *const libc::c_uchar,
    mut store: *mut ZopfliLZ77Store,
) {
    (*store).size = 0 as libc::c_int as size_t;
    (*store).litlens = 0 as *mut libc::c_ushort;
    (*store).dists = 0 as *mut libc::c_ushort;
    (*store).pos = 0 as *mut size_t;
    (*store).data = data;
    (*store).ll_symbol = 0 as *mut libc::c_ushort;
    (*store).d_symbol = 0 as *mut libc::c_ushort;
    (*store).ll_counts = 0 as *mut size_t;
    (*store).d_counts = 0 as *mut size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanLZ77Store(mut store: *mut ZopfliLZ77Store) {
    free((*store).litlens as *mut libc::c_void);
    free((*store).dists as *mut libc::c_void);
    free((*store).pos as *mut libc::c_void);
    free((*store).ll_symbol as *mut libc::c_void);
    free((*store).d_symbol as *mut libc::c_void);
    free((*store).ll_counts as *mut libc::c_void);
    free((*store).d_counts as *mut libc::c_void);
}
unsafe extern "C" fn CeilDiv(mut a: size_t, mut b: size_t) -> size_t {
    return a
        .wrapping_add(b)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(b);
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCopyLZ77Store(
    mut source: *const ZopfliLZ77Store,
    mut dest: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut llsize: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(CeilDiv((*source).size, 288 as libc::c_int as size_t));
    let mut dsize: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(CeilDiv((*source).size, 32 as libc::c_int as size_t));
    ZopfliCleanLZ77Store(dest);
    ZopfliInitLZ77Store((*source).data, dest);
    (*dest).litlens = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong).wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest).dists = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong).wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest).pos =
        malloc((::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul((*source).size))
            as *mut size_t;
    (*dest).ll_symbol = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong).wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest).d_symbol = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong).wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest).ll_counts =
        malloc((::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(llsize))
            as *mut size_t;
    (*dest).d_counts =
        malloc((::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(dsize))
            as *mut size_t;
    /* Allocation failed. */
    if (*dest).litlens.is_null() || (*dest).dists.is_null() {
        exit(-(1 as libc::c_int));
    }
    if (*dest).pos.is_null() {
        exit(-(1 as libc::c_int));
    }
    if (*dest).ll_symbol.is_null() || (*dest).d_symbol.is_null() {
        exit(-(1 as libc::c_int));
    }
    if (*dest).ll_counts.is_null() || (*dest).d_counts.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*dest).size = (*source).size;
    i = 0 as libc::c_int as size_t;
    while i < (*source).size {
        *(*dest).litlens.offset(i as isize) = *(*source).litlens.offset(i as isize);
        *(*dest).dists.offset(i as isize) = *(*source).dists.offset(i as isize);
        *(*dest).pos.offset(i as isize) = *(*source).pos.offset(i as isize);
        *(*dest).ll_symbol.offset(i as isize) = *(*source).ll_symbol.offset(i as isize);
        *(*dest).d_symbol.offset(i as isize) = *(*source).d_symbol.offset(i as isize);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < llsize {
        *(*dest).ll_counts.offset(i as isize) = *(*source).ll_counts.offset(i as isize);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < dsize {
        *(*dest).d_counts.offset(i as isize) = *(*source).d_counts.offset(i as isize);
        i = i.wrapping_add(1)
    }
}
/*
Appends the length and distance to the LZ77 arrays of the ZopfliLZ77Store.
context must be a ZopfliLZ77Store*.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliStoreLitLenDist(
    mut length: libc::c_ushort,
    mut dist: libc::c_ushort,
    mut pos: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    /* Needed for using ZOPFLI_APPEND_DATA multiple times. */
    let mut origsize: size_t = (*store).size;
    let mut llstart: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(origsize.wrapping_div(288 as libc::c_int as libc::c_ulong));
    let mut dstart: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(origsize.wrapping_div(32 as libc::c_int as libc::c_ulong));
    /* Everytime the index wraps around, a new cumulative histogram is made: we're
    keeping one histogram value per LZ77 symbol rather than a full histogram for
    each to save memory. */
    if origsize.wrapping_rem(288 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        let mut llsize: size_t = origsize;
        i = 0 as libc::c_int as size_t;
        while i < 288 as libc::c_int as libc::c_ulong {
            if llsize & llsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
                (*store).ll_counts = if llsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                } else {
                    realloc(
                        (*store).ll_counts as *mut libc::c_void,
                        llsize
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
                    )
                } as *mut size_t
            }
            *(*store).ll_counts.offset(llsize as isize) =
                if origsize == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    *(*store).ll_counts.offset(
                        origsize
                            .wrapping_sub(288 as libc::c_int as libc::c_ulong)
                            .wrapping_add(i) as isize,
                    )
                };
            llsize = llsize.wrapping_add(1);
            i = i.wrapping_add(1)
        }
    }
    if origsize.wrapping_rem(32 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        let mut dsize: size_t = origsize;
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as libc::c_ulong {
            if dsize & dsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
                (*store).d_counts = if dsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                } else {
                    realloc(
                        (*store).d_counts as *mut libc::c_void,
                        dsize
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
                    )
                } as *mut size_t
            }
            *(*store).d_counts.offset(dsize as isize) =
                if origsize == 0 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    *(*store).d_counts.offset(
                        origsize
                            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
                            .wrapping_add(i) as isize,
                    )
                };
            dsize = dsize.wrapping_add(1);
            i = i.wrapping_add(1)
        }
    }
    if (*store).size
        & (*store)
            .size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store).litlens = if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
        } else {
            realloc(
                (*store).litlens as *mut libc::c_void,
                (*store)
                    .size
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
            )
        } as *mut libc::c_ushort
    }
    *(*store).litlens.offset((*store).size as isize) = length;
    (*store).size = (*store).size.wrapping_add(1);
    (*store).size = origsize;
    if (*store).size
        & (*store)
            .size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store).dists = if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
        } else {
            realloc(
                (*store).dists as *mut libc::c_void,
                (*store)
                    .size
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
            )
        } as *mut libc::c_ushort
    }
    *(*store).dists.offset((*store).size as isize) = dist;
    (*store).size = (*store).size.wrapping_add(1);
    (*store).size = origsize;
    if (*store).size
        & (*store)
            .size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store).pos = if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
        } else {
            realloc(
                (*store).pos as *mut libc::c_void,
                (*store)
                    .size
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
        } as *mut size_t
    }
    *(*store).pos.offset((*store).size as isize) = pos;
    (*store).size = (*store).size.wrapping_add(1);
    if (length as libc::c_int) < 259 as libc::c_int {
    } else {
        __assert_fail(b"length < 259\x00" as *const u8 as *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      131 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"void ZopfliStoreLitLenDist(unsigned short, unsigned short, size_t, ZopfliLZ77Store *)\x00")).as_ptr());
    };
    if dist as libc::c_int == 0 as libc::c_int {
        (*store).size = origsize;
        if (*store).size
            & (*store)
                .size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0
        {
            (*store).ll_symbol = if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).ll_symbol as *mut libc::c_void,
                    (*store)
                        .size
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
                )
            } as *mut libc::c_ushort
        }
        *(*store).ll_symbol.offset((*store).size as isize) = length;
        (*store).size = (*store).size.wrapping_add(1);
        (*store).size = origsize;
        if (*store).size
            & (*store)
                .size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0
        {
            (*store).d_symbol = if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).d_symbol as *mut libc::c_void,
                    (*store)
                        .size
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
                )
            } as *mut libc::c_ushort
        }
        *(*store).d_symbol.offset((*store).size as isize) = 0 as libc::c_int as libc::c_ushort;
        (*store).size = (*store).size.wrapping_add(1);
        let ref mut fresh0 = *(*store)
            .ll_counts
            .offset(llstart.wrapping_add(length as libc::c_ulong) as isize);
        *fresh0 = (*fresh0).wrapping_add(1)
    } else {
        (*store).size = origsize;
        if (*store).size
            & (*store)
                .size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0
        {
            (*store).ll_symbol = if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).ll_symbol as *mut libc::c_void,
                    (*store)
                        .size
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
                )
            } as *mut libc::c_ushort
        }
        *(*store).ll_symbol.offset((*store).size as isize) =
            ZopfliGetLengthSymbol(length as libc::c_int) as libc::c_ushort;
        (*store).size = (*store).size.wrapping_add(1);
        (*store).size = origsize;
        if (*store).size
            & (*store)
                .size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0
        {
            (*store).d_symbol = if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).d_symbol as *mut libc::c_void,
                    (*store)
                        .size
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
                )
            } as *mut libc::c_ushort
        }
        *(*store).d_symbol.offset((*store).size as isize) =
            ZopfliGetDistSymbol(dist as libc::c_int) as libc::c_ushort;
        (*store).size = (*store).size.wrapping_add(1);
        let ref mut fresh1 = *(*store).ll_counts.offset(
            llstart.wrapping_add(ZopfliGetLengthSymbol(length as libc::c_int) as libc::c_ulong)
                as isize,
        );
        *fresh1 = (*fresh1).wrapping_add(1);
        let ref mut fresh2 = *(*store).d_counts.offset(
            dstart.wrapping_add(ZopfliGetDistSymbol(dist as libc::c_int) as libc::c_ulong) as isize,
        );
        *fresh2 = (*fresh2).wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliAppendLZ77Store(
    mut store: *const ZopfliLZ77Store,
    mut target: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*store).size {
        ZopfliStoreLitLenDist(
            *(*store).litlens.offset(i as isize),
            *(*store).dists.offset(i as isize),
            *(*store).pos.offset(i as isize),
            target,
        );
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77GetByteRange(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let mut l: size_t = lend.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if lstart == lend {
        return 0 as libc::c_int as size_t;
    }
    return (*(*lz77).pos.offset(l as isize))
        .wrapping_add(
            (if *(*lz77).dists.offset(l as isize) as libc::c_int == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                *(*lz77).litlens.offset(l as isize) as libc::c_int
            }) as libc::c_ulong,
        )
        .wrapping_sub(*(*lz77).pos.offset(lstart as isize));
}
unsafe extern "C" fn ZopfliLZ77GetHistogramAt(
    mut lz77: *const ZopfliLZ77Store,
    mut lpos: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    /* The real histogram is created by using the histogram for this chunk, but
    all superfluous values of this chunk subtracted. */
    let mut llpos: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lpos.wrapping_div(288 as libc::c_int as libc::c_ulong));
    let mut dpos: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lpos.wrapping_div(32 as libc::c_int as libc::c_ulong));
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        *ll_counts.offset(i as isize) = *(*lz77).ll_counts.offset(llpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1)
    }
    i = lpos.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while i < llpos.wrapping_add(288 as libc::c_int as libc::c_ulong) && i < (*lz77).size {
        let ref mut fresh3 = *ll_counts.offset(*(*lz77).ll_symbol.offset(i as isize) as isize);
        *fresh3 = (*fresh3).wrapping_sub(1);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        *d_counts.offset(i as isize) = *(*lz77).d_counts.offset(dpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1)
    }
    i = lpos.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while i < dpos.wrapping_add(32 as libc::c_int as libc::c_ulong) && i < (*lz77).size {
        if *(*lz77).dists.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            let ref mut fresh4 = *d_counts.offset(*(*lz77).d_symbol.offset(i as isize) as isize);
            *fresh4 = (*fresh4).wrapping_sub(1)
        }
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77GetHistogram(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let mut i: size_t = 0;
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as libc::c_ulong) > lend {
        memset(
            ll_counts as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(288 as libc::c_int as libc::c_ulong),
        );
        memset(
            d_counts as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(32 as libc::c_int as libc::c_ulong),
        );
        i = lstart;
        while i < lend {
            let ref mut fresh5 = *ll_counts.offset(*(*lz77).ll_symbol.offset(i as isize) as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            if *(*lz77).dists.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                let ref mut fresh6 =
                    *d_counts.offset(*(*lz77).d_symbol.offset(i as isize) as isize);
                *fresh6 = (*fresh6).wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    } else {
        /* Subtract the cumulative histograms at the end and the start to get the
        histogram for this range. */
        ZopfliLZ77GetHistogramAt(
            lz77,
            lend.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ll_counts,
            d_counts,
        );
        if lstart > 0 as libc::c_int as libc::c_ulong {
            let mut ll_counts2: [size_t; 288] = [0; 288];
            let mut d_counts2: [size_t; 32] = [0; 32];
            ZopfliLZ77GetHistogramAt(
                lz77,
                lstart.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ll_counts2.as_mut_ptr(),
                d_counts2.as_mut_ptr(),
            );
            i = 0 as libc::c_int as size_t;
            while i < 288 as libc::c_int as libc::c_ulong {
                let ref mut fresh7 = *ll_counts.offset(i as isize);
                *fresh7 = (*fresh7 as libc::c_ulong).wrapping_sub(ll_counts2[i as usize]) as size_t
                    as size_t;
                i = i.wrapping_add(1)
            }
            i = 0 as libc::c_int as size_t;
            while i < 32 as libc::c_int as libc::c_ulong {
                let ref mut fresh8 = *d_counts.offset(i as isize);
                *fresh8 = (*fresh8 as libc::c_ulong).wrapping_sub(d_counts2[i as usize]) as size_t
                    as size_t;
                i = i.wrapping_add(1)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliInitBlockState(
    mut options: *const ZopfliOptions,
    mut blockstart: size_t,
    mut blockend: size_t,
    mut add_lmc: libc::c_int,
    mut s: *mut ZopfliBlockState,
) {
    (*s).options = options;
    (*s).blockstart = blockstart;
    (*s).blockend = blockend;
    if add_lmc != 0 {
        (*s).lmc = malloc(::std::mem::size_of::<ZopfliLongestMatchCache>() as libc::c_ulong)
            as *mut ZopfliLongestMatchCache;
        ZopfliInitCache(blockend.wrapping_sub(blockstart), (*s).lmc);
    } else {
        (*s).lmc = 0 as *mut ZopfliLongestMatchCache
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliCleanBlockState(mut s: *mut ZopfliBlockState) {
    if !(*s).lmc.is_null() {
        ZopfliCleanCache((*s).lmc);
        free((*s).lmc as *mut libc::c_void);
    };
}
/*
Gets a score of the length given the distance. Typically, the score of the
length is the length itself, but if the distance is very long, decrease the
score of the length a bit to make up for the fact that long distances use large
amounts of extra bits.

This is not an accurate score, it is a heuristic only for the greedy LZ77
implementation. More accurate cost models are employed later. Making this
heuristic more accurate may hurt rather than improve compression.

The two direct uses of this heuristic are:
-avoid using a length of 3 in combination with a long distance. This only has
 an effect if length == 3.
-make a slightly better choice between the two options of the lazy matching.

Indirectly, this affects:
-the block split points if the default of block splitting first is used, in a
 rather unpredictable way
-the first zopfli run, so it affects the chance of the first run being closer
 to the optimal output
*/
unsafe extern "C" fn GetLengthScore(
    mut length: libc::c_int,
    mut distance: libc::c_int,
) -> libc::c_int {
    /*
    At 1024, the distance uses 9+ extra bits and this seems to be the sweet spot
    on tested files.
    */
    return if distance > 1024 as libc::c_int {
        (length) - 1 as libc::c_int
    } else {
        length
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliVerifyLenDist(
    mut data: *const libc::c_uchar,
    mut datasize: size_t,
    mut pos: size_t,
    mut dist: libc::c_ushort,
    mut length: libc::c_ushort,
) {
    /* TODO(lode): make this only run in a debug compile, it's for assert only. */
    let mut i: size_t = 0;
    if pos.wrapping_add(length as libc::c_ulong) <= datasize {
    } else {
        __assert_fail(b"pos + length <= datasize\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      279 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 96],
                                                &[libc::c_char; 96]>(b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\x00")).as_ptr());
    };
    i = 0 as libc::c_int as size_t;
    while i < length as libc::c_ulong {
        if *data.offset(pos.wrapping_sub(dist as libc::c_ulong).wrapping_add(i) as isize)
            as libc::c_int
            != *data.offset(pos.wrapping_add(i) as isize) as libc::c_int
        {
            if *data.offset(pos.wrapping_sub(dist as libc::c_ulong).wrapping_add(i) as isize)
                as libc::c_int
                == *data.offset(pos.wrapping_add(i) as isize) as libc::c_int
            {
            } else {
                __assert_fail(b"data[pos - dist + i] == data[pos + i]\x00" as
                                  *const u8 as *const libc::c_char,
                              b"src/zopfli/lz77.c\x00" as *const u8 as
                                  *const libc::c_char,
                              282 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 96],
                                                        &[libc::c_char; 96]>(b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\x00")).as_ptr());
            };
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
}
/*
Finds how long the match of scan and match is. Can be used to find how many
bytes starting from scan, and from match, are equal. Returns the last byte
after scan, which is still equal to the correspondinb byte after match.
scan is the position to compare
match is the earlier position to compare.
end is the last possible byte, beyond which to stop looking.
safe_end is a few (8) bytes before end, for comparing multiple bytes at once.
*/
unsafe extern "C" fn GetMatch(
    mut scan: *const libc::c_uchar,
    mut match_0: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut safe_end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    if ::std::mem::size_of::<size_t>() as libc::c_ulong == 8 as libc::c_int as libc::c_ulong {
        /* 8 checks at once per array bounds check (size_t is 64-bit). */
        while scan < safe_end && *(scan as *mut size_t) == *(match_0 as *mut size_t) {
            scan = scan.offset(8 as libc::c_int as isize);
            match_0 = match_0.offset(8 as libc::c_int as isize)
        }
    } else if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        /* 4 checks at once per array bounds check (unsigned int is 32-bit). */
        while scan < safe_end && *(scan as *mut libc::c_uint) == *(match_0 as *mut libc::c_uint) {
            scan = scan.offset(4 as libc::c_int as isize);
            match_0 = match_0.offset(4 as libc::c_int as isize)
        }
    } else {
        /* do 8 checks at once per array bounds check. */
        while scan < safe_end
            && *scan as libc::c_int == *match_0 as libc::c_int
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                (*scan as libc::c_int) == *match_0 as libc::c_int
            }
        {
            scan = scan.offset(1);
            match_0 = match_0.offset(1)
        }
    }
    /* The remaining few bytes. */
    while scan != end && *scan as libc::c_int == *match_0 as libc::c_int {
        scan = scan.offset(1);
        match_0 = match_0.offset(1)
    }
    return scan;
}
/*
Gets distance, length and sublen values from the cache if possible.
Returns 1 if it got the values from the cache, 0 if not.
Updates the limit value to a smaller one if possible with more limited
information from the cache.
*/
unsafe extern "C" fn TryGetFromLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: *mut size_t,
    mut sublen: *mut libc::c_ushort,
    mut distance: *mut libc::c_ushort,
    mut length: *mut libc::c_ushort,
) -> libc::c_int {
    /* The LMC cache starts at the beginning of the block rather than the
    beginning of the whole array. */
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    /* Length > 0 and dist 0 is invalid combination, which indicates on purpose
    that this cache value is not filled in yet. */
    let mut cache_available: libc::c_uchar = (!(*s).lmc.is_null()
        && (*(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_int == 0 as libc::c_int
            || *(*(*s).lmc).dist.offset(lmcpos as isize) as libc::c_int != 0 as libc::c_int))
        as libc::c_int as libc::c_uchar;
    let mut limit_ok_for_cache: libc::c_uchar = (cache_available as libc::c_int != 0
        && (*limit == 258 as libc::c_int as libc::c_ulong
            || *(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_ulong <= *limit
            || !sublen.is_null()
                && ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *(*(*s).lmc).length.offset(lmcpos as isize) as size_t,
                ) as libc::c_ulong
                    >= *limit)) as libc::c_int
        as libc::c_uchar;
    if !(*s).lmc.is_null()
        && limit_ok_for_cache as libc::c_int != 0
        && cache_available as libc::c_int != 0
    {
        if sublen.is_null()
            || *(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_uint
                <= ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *(*(*s).lmc).length.offset(lmcpos as isize) as size_t,
                )
        {
            *length = *(*(*s).lmc).length.offset(lmcpos as isize);
            if *length as libc::c_ulong > *limit {
                *length = *limit as libc::c_ushort
            }
            if !sublen.is_null() {
                ZopfliCacheToSublen((*s).lmc, lmcpos, *length as size_t, sublen);
                *distance = *sublen.offset(*length as isize);
                if *limit == 258 as libc::c_int as libc::c_ulong
                    && *length as libc::c_int >= 3 as libc::c_int
                {
                    if *sublen.offset(*length as isize) as libc::c_int
                        == *(*(*s).lmc).dist.offset(lmcpos as isize) as libc::c_int
                    {
                    } else {
                        __assert_fail(b"sublen[*length] == s->lmc->dist[lmcpos]\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"src/zopfli/lz77.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      365 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 124],
                                                                &[libc::c_char; 124]>(b"int TryGetFromLongestMatchCache(ZopfliBlockState *, size_t, size_t *, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
                    };
                }
            } else {
                *distance = *(*(*s).lmc).dist.offset(lmcpos as isize)
            }
            return 1 as libc::c_int;
        }
        /* Can't use much of the cache, since the "sublens" need to be calculated,
        but at  least we already know when to stop. */
        *limit = *(*(*s).lmc).length.offset(lmcpos as isize) as size_t
    }
    return 0 as libc::c_int;
}
/*
Stores the found sublen, distance and length in the longest match cache, if
possible.
*/
unsafe extern "C" fn StoreInLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: size_t,
    mut sublen: *const libc::c_ushort,
    mut distance: libc::c_ushort,
    mut length: libc::c_ushort,
) {
    /* The LMC cache starts at the beginning of the block rather than the
    beginning of the whole array. */
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    /* Length > 0 and dist 0 is invalid combination, which indicates on purpose
    that this cache value is not filled in yet. */
    let mut cache_available: libc::c_uchar = (!(*s).lmc.is_null()
        && (*(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_int == 0 as libc::c_int
            || *(*(*s).lmc).dist.offset(lmcpos as isize) as libc::c_int != 0 as libc::c_int))
        as libc::c_int as libc::c_uchar; /* For quitting early. */
    if !(*s).lmc.is_null()
        && limit == 258 as libc::c_int as libc::c_ulong
        && !sublen.is_null()
        && cache_available == 0
    {
        if *(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_int == 1 as libc::c_int
            && *(*(*s).lmc).dist.offset(lmcpos as isize) as libc::c_int == 0 as libc::c_int
        {
        } else {
            __assert_fail(b"s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          398 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\x00")).as_ptr());
            /* Not unsigned short on purpose. */
        };
        *(*(*s).lmc).dist.offset(lmcpos as isize) = if (length as libc::c_int) < 3 as libc::c_int {
            0 as libc::c_int
        } else {
            distance as libc::c_int
        } as libc::c_ushort;
        *(*(*s).lmc).length.offset(lmcpos as isize) = if (length as libc::c_int) < 3 as libc::c_int
        {
            0 as libc::c_int
        } else {
            length as libc::c_int
        } as libc::c_ushort;
        if !(*(*(*s).lmc).length.offset(lmcpos as isize) as libc::c_int == 1 as libc::c_int
            && *(*(*s).lmc).dist.offset(lmcpos as isize) as libc::c_int == 0 as libc::c_int)
        {
        } else {
            __assert_fail(b"!(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)\x00"
                              as *const u8 as *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          401 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\x00")).as_ptr());
        };
        ZopfliSublenToCache(sublen, lmcpos, length as size_t, (*s).lmc);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliFindLongestMatch(
    mut s: *mut ZopfliBlockState,
    mut h: *const ZopfliHash,
    mut array: *const libc::c_uchar,
    mut pos: size_t,
    mut size: size_t,
    mut limit: size_t,
    mut sublen: *mut libc::c_ushort,
    mut distance: *mut libc::c_ushort,
    mut length: *mut libc::c_ushort,
) {
    let mut hpos: libc::c_ushort =
        (pos & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as libc::c_ushort;
    let mut p: libc::c_ushort = 0;
    let mut pp: libc::c_ushort = 0;
    let mut bestdist: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut bestlength: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
    let mut scan: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut match_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut arrayend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut arrayend_safe: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut chain_counter: libc::c_int = 8192 as libc::c_int;
    let mut dist: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hhead: *mut libc::c_int = (*h).head;
    let mut hprev: *mut libc::c_ushort = (*h).prev;
    let mut hhashval: *mut libc::c_int = (*h).hashval;
    let mut hval: libc::c_int = (*h).val;
    if TryGetFromLongestMatchCache(s, pos, &mut limit, sublen, distance, length) != 0 {
        if pos.wrapping_add(*length as libc::c_ulong) <= size {
        } else {
            __assert_fail(b"pos + *length <= size\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          431 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 169],
                                                    &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
        };
        return;
    }
    if limit <= 258 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"limit <= ZOPFLI_MAX_MATCH\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      436 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    if limit >= 3 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"limit >= ZOPFLI_MIN_MATCH\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      437 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    if pos < size {
    } else {
        __assert_fail(b"pos < size\x00" as *const u8 as *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      438 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    if size.wrapping_sub(pos) < 3 as libc::c_int as libc::c_ulong {
        /* The rest of the code assumes there are at least ZOPFLI_MIN_MATCH bytes to
        try. */
        *length = 0 as libc::c_int as libc::c_ushort; /* During the whole loop, p == hprev[pp]. */
        *distance = 0 as libc::c_int as libc::c_ushort;
        return;
    }
    if pos.wrapping_add(limit) > size {
        limit = size.wrapping_sub(pos)
    }
    arrayend = (&*array.offset(pos as isize) as *const libc::c_uchar).offset(limit as isize);
    arrayend_safe = arrayend.offset(-(8 as libc::c_int as isize));
    if hval < 65536 as libc::c_int {
    } else {
        __assert_fail(b"hval < 65536\x00" as *const u8 as *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      454 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    pp = *hhead.offset(hval as isize) as libc::c_ushort;
    p = *hprev.offset(pp as isize);
    if pp as libc::c_int == hpos as libc::c_int {
    } else {
        __assert_fail(b"pp == hpos\x00" as *const u8 as *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      459 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    dist = if (p as libc::c_int) < pp as libc::c_int {
        (pp as libc::c_int) - p as libc::c_int
    } else {
        (32768 as libc::c_int - p as libc::c_int) + pp as libc::c_int
    } as libc::c_uint;
    /* Go through all distances. */
    while dist < 32768 as libc::c_int as libc::c_uint {
        let mut currentlength: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
        if (p as libc::c_int) < 32768 as libc::c_int {
        } else {
            __assert_fail(b"p < ZOPFLI_WINDOW_SIZE\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          467 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 169],
                                                    &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
        };
        if p as libc::c_int == *hprev.offset(pp as isize) as libc::c_int {
        } else {
            __assert_fail(b"p == hprev[pp]\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          468 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 169],
                                                    &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
        };
        if *hhashval.offset(p as isize) == hval {
        } else {
            __assert_fail(b"hhashval[p] == hval\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/lz77.c\x00" as *const u8 as
                              *const libc::c_char,
                          469 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 169],
                                                    &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
        };
        if dist > 0 as libc::c_int as libc::c_uint {
            if pos < size {
            } else {
                __assert_fail(b"pos < size\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/lz77.c\x00" as *const u8 as
                                  *const libc::c_char,
                              472 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 169],
                                                        &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
            };
            if dist as libc::c_ulong <= pos {
            } else {
                __assert_fail(b"dist <= pos\x00" as *const u8 as
                                  *const libc::c_char,
                              b"src/zopfli/lz77.c\x00" as *const u8 as
                                  *const libc::c_char,
                              473 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 169],
                                                        &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
            };
            scan = &*array.offset(pos as isize) as *const libc::c_uchar;
            match_0 = &*array.offset(pos.wrapping_sub(dist as libc::c_ulong) as isize)
                as *const libc::c_uchar;
            /* Testing the byte at position bestlength first, goes slightly faster. */
            if pos.wrapping_add(bestlength as libc::c_ulong) >= size
                || *scan.offset(bestlength as libc::c_int as isize) as libc::c_int
                    == *match_0.offset(bestlength as libc::c_int as isize) as libc::c_int
            {
                let mut same0: libc::c_ushort = *(*h).same.offset(
                    (pos & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as isize,
                );
                if same0 as libc::c_int > 2 as libc::c_int
                    && *scan as libc::c_int == *match_0 as libc::c_int
                {
                    let mut same1: libc::c_ushort = *(*h).same.offset(
                        (pos.wrapping_sub(dist as libc::c_ulong)
                            & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                            as isize,
                    );
                    let mut same: libc::c_ushort = if (same0 as libc::c_int) < same1 as libc::c_int
                    {
                        same0 as libc::c_int
                    } else {
                        same1 as libc::c_int
                    } as libc::c_ushort;
                    if same as libc::c_ulong > limit {
                        same = limit as libc::c_ushort
                    }
                    scan = scan.offset(same as libc::c_int as isize);
                    match_0 = match_0.offset(same as libc::c_int as isize)
                }
                scan = GetMatch(scan, match_0, arrayend, arrayend_safe);
                currentlength = scan
                    .offset_from(&*array.offset(pos as isize) as *const libc::c_uchar)
                    as libc::c_long as libc::c_ushort
                /* The found length. */
            }
            if currentlength as libc::c_int > bestlength as libc::c_int {
                if !sublen.is_null() {
                    let mut j: libc::c_ushort = 0;
                    j = (bestlength as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
                    while j as libc::c_int <= currentlength as libc::c_int {
                        *sublen.offset(j as isize) = dist as libc::c_ushort;
                        j = j.wrapping_add(1)
                    }
                }
                bestdist = dist as libc::c_ushort;
                bestlength = currentlength;
                if currentlength as libc::c_ulong >= limit {
                    break;
                }
            }
        }
        /* Switch to the other hash once this will be more efficient. */
        if hhead != (*h).head2
            && bestlength as libc::c_int >= *(*h).same.offset(hpos as isize) as libc::c_int
            && (*h).val2 == *(*h).hashval2.offset(p as isize)
        {
            /* Now use the hash that encodes the length and first byte. */
            hhead = (*h).head2; /* Uninited prev value. */
            hprev = (*h).prev2;
            hhashval = (*h).hashval2;
            hval = (*h).val2
        }
        pp = p;
        p = *hprev.offset(p as isize);
        if p as libc::c_int == pp as libc::c_int {
            break;
        }
        dist = dist.wrapping_add(if (p as libc::c_int) < pp as libc::c_int {
            (pp as libc::c_int) - p as libc::c_int
        } else {
            (32768 as libc::c_int - p as libc::c_int) + pp as libc::c_int
        } as libc::c_uint);
        chain_counter -= 1;
        if chain_counter <= 0 as libc::c_int {
            break;
        }
    }
    StoreInLongestMatchCache(s, pos, limit, sublen, bestdist, bestlength);
    if bestlength as libc::c_ulong <= limit {
    } else {
        __assert_fail(b"bestlength <= limit\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      537 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
    *distance = bestdist;
    *length = bestlength;
    if pos.wrapping_add(*length as libc::c_ulong) <= size {
    } else {
        __assert_fail(b"pos + *length <= size\x00" as *const u8 as
                          *const libc::c_char,
                      b"src/zopfli/lz77.c\x00" as *const u8 as
                          *const libc::c_char,
                      541 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 169],
                                                &[libc::c_char; 169]>(b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\x00")).as_ptr());
    };
}
/* Gets the amount of raw bytes that this range of LZ77 symbols spans. */
/* Gets the histogram of lit/len and dist symbols in the given range, using the
cumulative histograms, so faster than adding one by one for large range. Does
not add the one end symbol of value 256. */
/*
Some state information for compressing a block.
This is currently a bit under-used (with mainly only the longest match cache),
but is kept for easy future expansion.
*/
/* Cache for length/distance pairs found so far. */
/* The start (inclusive) and end (not inclusive) of the current block. */
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
/*
Verifies if length and dist are indeed valid, only used for assertion.
*/
/*
Does LZ77 using an algorithm similar to gzip, with lazy matching, rather than
with the slow but better "squeeze" implementation.
The result is placed in the ZopfliLZ77Store.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliLZ77Greedy(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0;
    let mut leng: libc::c_ushort = 0;
    let mut dist: libc::c_ushort = 0;
    let mut lengthscore: libc::c_int = 0;
    let mut windowstart: size_t = if instart > 32768 as libc::c_int as libc::c_ulong {
        instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut dummysublen: [libc::c_ushort; 259] = [0; 259];
    /* Lazy matching. */
    let mut prev_length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prev_match: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prevlengthscore: libc::c_int = 0;
    let mut match_available: libc::c_int = 0 as libc::c_int;
    if instart == inend {
        return;
    }
    ZopfliResetHash(32768 as libc::c_int as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1)
    }
    let mut current_block_44: u64;
    i = instart;
    while i < inend {
        ZopfliUpdateHash(in_0, i, inend, h);
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            258 as libc::c_int as size_t,
            dummysublen.as_mut_ptr(),
            &mut dist,
            &mut leng,
        );
        lengthscore = GetLengthScore(leng as libc::c_int, dist as libc::c_int);
        /* Lazy matching. */
        prevlengthscore = GetLengthScore(prev_length as libc::c_int, prev_match as libc::c_int);
        if match_available != 0 {
            match_available = 0 as libc::c_int;
            if lengthscore > prevlengthscore + 1 as libc::c_int {
                ZopfliStoreLitLenDist(
                    *in_0.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    store,
                );
                if lengthscore >= 3 as libc::c_int && (leng as libc::c_int) < 258 as libc::c_int {
                    match_available = 1 as libc::c_int;
                    prev_length = leng as libc::c_uint;
                    prev_match = dist as libc::c_uint;
                    current_block_44 = 3512920355445576850;
                } else {
                    current_block_44 = 4567019141635105728;
                }
            } else {
                /* Add previous to output. */
                leng = prev_length as libc::c_ushort;
                dist = prev_match as libc::c_ushort;
                lengthscore = prevlengthscore;
                /* Add to output. */
                ZopfliVerifyLenDist(
                    in_0,
                    inend,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    dist,
                    leng,
                );
                ZopfliStoreLitLenDist(
                    leng,
                    dist,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    store,
                );
                j = 2 as libc::c_int as size_t;
                while j < leng as libc::c_ulong {
                    if i < inend {
                    } else {
                        __assert_fail(b"i < inend\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"src/zopfli/lz77.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      600 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 114],
                                                                &[libc::c_char; 114]>(b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\x00")).as_ptr());
                    };
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1)
                }
                current_block_44 = 3512920355445576850;
            }
        } else if lengthscore >= 3 as libc::c_int && (leng as libc::c_int) < 258 as libc::c_int {
            match_available = 1 as libc::c_int;
            prev_length = leng as libc::c_uint;
            prev_match = dist as libc::c_uint;
            current_block_44 = 3512920355445576850;
        } else {
            current_block_44 = 4567019141635105728;
        }
        match current_block_44 {
            4567019141635105728 => {
                /* End of lazy matching. */
                /* Add to output. */
                if lengthscore >= 3 as libc::c_int {
                    ZopfliVerifyLenDist(in_0, inend, i, dist, leng);
                    ZopfliStoreLitLenDist(leng, dist, i, store);
                } else {
                    leng = 1 as libc::c_int as libc::c_ushort;
                    ZopfliStoreLitLenDist(
                        *in_0.offset(i as isize) as libc::c_ushort,
                        0 as libc::c_int as libc::c_ushort,
                        i,
                        store,
                    );
                }
                j = 1 as libc::c_int as size_t;
                while j < leng as libc::c_ulong {
                    if i < inend {
                    } else {
                        __assert_fail(b"i < inend\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"src/zopfli/lz77.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      625 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 114],
                                                                &[libc::c_char; 114]>(b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\x00")).as_ptr());
                    };
                    i = i.wrapping_add(1);
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1)
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1)
    }
}
