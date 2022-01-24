use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
Outputs minimum-redundancy length-limited code bitlengths for symbols with the
given counts. The bitlengths are limited by maxbits.

The output is tailored for DEFLATE: symbols that never occur, get a bit length
of 0, and if only a single symbol occurs at least once, its bitlength will be 1,
and not 0 as would theoretically be needed for a single symbol.

frequencies: The amount of occurrences of each symbol.
n: The amount of symbols.
maxbits: Maximum bit length, inclusive.
bitlengths: Output, the bitlengths for the symbol prefix codes.
return: 0 for OK, non-0 for error.
*/
    #[no_mangle]
    fn ZopfliLengthLimitedCodeLengths(frequencies: *const size_t,
                                      n: libc::c_int, maxbits: libc::c_int,
                                      bitlengths: *mut libc::c_uint)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
/*
Converts a series of Huffman tree bitlengths, to the bit values of the symbols.
*/
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
pub unsafe extern "C" fn ZopfliLengthsToSymbols(mut lengths:
                                                    *const libc::c_uint,
                                                mut n: size_t,
                                                mut maxbits: libc::c_uint,
                                                mut symbols:
                                                    *mut libc::c_uint) {
    let mut bl_count: *mut size_t =
        malloc((::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(maxbits.wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                    as libc::c_ulong)) as
            *mut size_t;
    let mut next_code: *mut size_t =
        malloc((::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(maxbits.wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                    as libc::c_ulong)) as
            *mut size_t;
    let mut bits: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut code: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        *symbols.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
    /* 1) Count the number of codes for each code length. Let bl_count[N] be the
  number of codes of length N, N >= 1. */
    bits = 0 as libc::c_int as libc::c_uint;
    while bits <= maxbits {
        *bl_count.offset(bits as isize) = 0 as libc::c_int as size_t;
        bits = bits.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        if *lengths.offset(i as isize) <= maxbits {
        } else {
            __assert_fail(b"lengths[i] <= maxbits\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/tree.c\x00" as *const u8 as
                              *const libc::c_char,
                          47 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 88],
                                                    &[libc::c_char; 88]>(b"void ZopfliLengthsToSymbols(const unsigned int *, size_t, unsigned int, unsigned int *)\x00")).as_ptr());
        };
        let ref mut fresh0 =
            *bl_count.offset(*lengths.offset(i as isize) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        i = i.wrapping_add(1)
    }
    /* 2) Find the numerical value of the smallest code for each code length. */
    code = 0 as libc::c_int as libc::c_uint;
    *bl_count.offset(0 as libc::c_int as isize) = 0 as libc::c_int as size_t;
    bits = 1 as libc::c_int as libc::c_uint;
    while bits <= maxbits {
        code =
            ((code as
                  libc::c_ulong).wrapping_add(*bl_count.offset(bits.wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                                                   as isize))
                 << 1 as libc::c_int) as libc::c_uint;
        *next_code.offset(bits as isize) = code as size_t;
        bits = bits.wrapping_add(1)
    }
    /* 3) Assign numerical values to all codes, using consecutive values for all
  codes of the same length with the base values determined at step 2. */
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        let mut len: libc::c_uint = *lengths.offset(i as isize);
        if len != 0 as libc::c_int as libc::c_uint {
            *symbols.offset(i as isize) =
                *next_code.offset(len as isize) as libc::c_uint;
            let ref mut fresh1 = *next_code.offset(len as isize);
            *fresh1 = (*fresh1).wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    free(bl_count as *mut libc::c_void);
    free(next_code as *mut libc::c_void);
}
/*
Calculates the entropy of each symbol, based on the counts of each symbol. The
result is similar to the result of ZopfliCalculateBitLengths, but with the
actual theoritical bit lengths according to the entropy. Since the resulting
values are fractional, they cannot be used to encode the tree specified by
DEFLATE.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateEntropy(mut count: *const size_t,
                                                mut n: size_t,
                                                mut bitlengths:
                                                    *mut libc::c_double) {
    static mut kInvLog2: libc::c_double =
        1.4426950408889f64; /* 1.0 / log(2.0) */
    let mut sum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut log2sum: libc::c_double = 0.;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        sum =
            (sum as libc::c_ulong).wrapping_add(*count.offset(i as isize)) as
                libc::c_uint as libc::c_uint;
        i = i.wrapping_add(1)
    }
    log2sum =
        (if sum == 0 as libc::c_int as libc::c_uint {
             log(n as libc::c_double)
         } else { log(sum as libc::c_double) }) * kInvLog2;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        /* When the count of the symbol is 0, but its cost is requested anyway, it
    means the symbol will appear at least once anyway, so give it the cost as if
    its count is 1.*/
        if *count.offset(i as isize) == 0 as libc::c_int as libc::c_ulong {
            *bitlengths.offset(i as isize) = log2sum
        } else {
            *bitlengths.offset(i as isize) =
                log2sum -
                    log(*count.offset(i as isize) as libc::c_double) *
                        kInvLog2
        }
        /* Depending on compiler and architecture, the above subtraction of two
    floating point numbers may give a negative result very close to zero
    instead of zero (e.g. -5.973954e-17 with gcc 4.1.2 on Ubuntu 11.4). Clamp
    it to zero. These floating point imprecisions do not affect the cost model
    significantly so this is ok. */
        if *bitlengths.offset(i as isize) < 0 as libc::c_int as libc::c_double
               && *bitlengths.offset(i as isize) > -1e-5f64 {
            *bitlengths.offset(i as isize) =
                0 as libc::c_int as libc::c_double
        }
        if *bitlengths.offset(i as isize) >=
               0 as libc::c_int as libc::c_double {
        } else {
            __assert_fail(b"bitlengths[i] >= 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"src/zopfli/tree.c\x00" as *const u8 as
                              *const libc::c_char,
                          92 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 62],
                                                    &[libc::c_char; 62]>(b"void ZopfliCalculateEntropy(const size_t *, size_t, double *)\x00")).as_ptr());
        };
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
Utilities for creating and using Huffman trees.
*/
/*
Calculates the bitlengths for the Huffman tree, based on the counts of each
symbol.
*/
#[no_mangle]
pub unsafe extern "C" fn ZopfliCalculateBitLengths(mut count: *const size_t,
                                                   mut n: size_t,
                                                   mut maxbits: libc::c_int,
                                                   mut bitlengths:
                                                       *mut libc::c_uint) {
    let mut error: libc::c_int =
        ZopfliLengthLimitedCodeLengths(count, n as libc::c_int, maxbits,
                                       bitlengths);
    if error == 0 {
    } else {
        __assert_fail(b"!error\x00" as *const u8 as *const libc::c_char,
                      b"src/zopfli/tree.c\x00" as *const u8 as
                          *const libc::c_char,
                      100 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[libc::c_char; 76]>(b"void ZopfliCalculateBitLengths(const size_t *, size_t, int, unsigned int *)\x00")).as_ptr());
    };
}
