use ::libc;
extern "C" {
    #[no_mangle]
    fn ZopfliDeflate(options: *const ZopfliOptions, btype: libc::c_int,
                     final_0: libc::c_int, in_0: *const libc::c_uchar,
                     insize: size_t, bp: *mut libc::c_uchar,
                     out: *mut *mut libc::c_uchar, outsize: *mut size_t);
    /*
Copyright 2013 Google Inc. All Rights Reserved.

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
Functions to compress according to the Gzip specification.
*/
    /*
Compresses according to the gzip specification and append the compressed
result to the output.

options: global program options
out: pointer to the dynamic output array to which the result is appended. Must
  be freed after use.
outsize: pointer to the dynamic output array size.
*/
    #[no_mangle]
    fn ZopfliGzipCompress(options: *const ZopfliOptions,
                          in_0: *const libc::c_uchar, insize: size_t,
                          out: *mut *mut libc::c_uchar, outsize: *mut size_t);
    /*
Copyright 2013 Google Inc. All Rights Reserved.

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
Functions to compress according to the Zlib specification.
*/
    /*
Compresses according to the zlib specification and append the compressed
result to the output.

options: global program options
out: pointer to the dynamic output array to which the result is appended. Must
  be freed after use.
outsize: pointer to the dynamic output array size.
*/
    #[no_mangle]
    fn ZopfliZlibCompress(options: *const ZopfliOptions,
                          in_0: *const libc::c_uchar, insize: size_t,
                          out: *mut *mut libc::c_uchar, outsize: *mut size_t);
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
pub type ZopfliFormat = libc::c_uint;
pub const ZOPFLI_FORMAT_DEFLATE: ZopfliFormat = 2;
pub const ZOPFLI_FORMAT_ZLIB: ZopfliFormat = 1;
pub const ZOPFLI_FORMAT_GZIP: ZopfliFormat = 0;
/*
Compresses according to the given output format and appends the result to the
output.

options: global program options
output_type: the output format to use
out: pointer to the dynamic output array to which the result is appended. Must
  be freed after use
outsize: pointer to the dynamic output array size
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
pub unsafe extern "C" fn ZopfliCompress(mut options: *const ZopfliOptions,
                                        mut output_type: ZopfliFormat,
                                        mut in_0: *const libc::c_uchar,
                                        mut insize: size_t,
                                        mut out: *mut *mut libc::c_uchar,
                                        mut outsize: *mut size_t) {
    if output_type as libc::c_uint ==
           ZOPFLI_FORMAT_GZIP as libc::c_int as libc::c_uint {
        ZopfliGzipCompress(options, in_0, insize, out, outsize);
    } else if output_type as libc::c_uint ==
                  ZOPFLI_FORMAT_ZLIB as libc::c_int as libc::c_uint {
        ZopfliZlibCompress(options, in_0, insize, out, outsize);
    } else if output_type as libc::c_uint ==
                  ZOPFLI_FORMAT_DEFLATE as libc::c_int as libc::c_uint {
        let mut bp: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
        ZopfliDeflate(options, 2 as libc::c_int, 1 as libc::c_int, in_0,
                      insize, &mut bp, out, outsize);
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"src/zopfli/zopfli_lib.c\x00" as *const u8 as
                              *const libc::c_char,
                          40 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 116],
                                                    &[libc::c_char; 116]>(b"void ZopfliCompress(const ZopfliOptions *, ZopfliFormat, const unsigned char *, size_t, unsigned char **, size_t *)\x00")).as_ptr());
        };
    };
}
