use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn ZopfliDeflate(options: *const ZopfliOptions, btype: libc::c_int,
                     final_0: libc::c_int, in_0: *const libc::c_uchar,
                     insize: size_t, bp: *mut libc::c_uchar,
                     out: *mut *mut libc::c_uchar, outsize: *mut size_t);
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
/* CRC polynomial: 0xedb88320 */
static mut crc32_table: [libc::c_ulong; 256] =
    [0 as libc::c_uint as libc::c_ulong,
     1996959894 as libc::c_uint as libc::c_ulong,
     3993919788 as libc::c_uint as libc::c_ulong,
     2567524794 as libc::c_uint as libc::c_ulong,
     124634137 as libc::c_uint as libc::c_ulong,
     1886057615 as libc::c_uint as libc::c_ulong,
     3915621685 as libc::c_uint as libc::c_ulong,
     2657392035 as libc::c_uint as libc::c_ulong,
     249268274 as libc::c_uint as libc::c_ulong,
     2044508324 as libc::c_uint as libc::c_ulong,
     3772115230 as libc::c_uint as libc::c_ulong,
     2547177864 as libc::c_uint as libc::c_ulong,
     162941995 as libc::c_uint as libc::c_ulong,
     2125561021 as libc::c_uint as libc::c_ulong,
     3887607047 as libc::c_uint as libc::c_ulong,
     2428444049 as libc::c_uint as libc::c_ulong,
     498536548 as libc::c_uint as libc::c_ulong,
     1789927666 as libc::c_uint as libc::c_ulong,
     4089016648 as libc::c_uint as libc::c_ulong,
     2227061214 as libc::c_uint as libc::c_ulong,
     450548861 as libc::c_uint as libc::c_ulong,
     1843258603 as libc::c_uint as libc::c_ulong,
     4107580753 as libc::c_uint as libc::c_ulong,
     2211677639 as libc::c_uint as libc::c_ulong,
     325883990 as libc::c_uint as libc::c_ulong,
     1684777152 as libc::c_uint as libc::c_ulong,
     4251122042 as libc::c_uint as libc::c_ulong,
     2321926636 as libc::c_uint as libc::c_ulong,
     335633487 as libc::c_uint as libc::c_ulong,
     1661365465 as libc::c_uint as libc::c_ulong,
     4195302755 as libc::c_uint as libc::c_ulong,
     2366115317 as libc::c_uint as libc::c_ulong,
     997073096 as libc::c_uint as libc::c_ulong,
     1281953886 as libc::c_uint as libc::c_ulong,
     3579855332 as libc::c_uint as libc::c_ulong,
     2724688242 as libc::c_uint as libc::c_ulong,
     1006888145 as libc::c_uint as libc::c_ulong,
     1258607687 as libc::c_uint as libc::c_ulong,
     3524101629 as libc::c_uint as libc::c_ulong,
     2768942443 as libc::c_uint as libc::c_ulong,
     901097722 as libc::c_uint as libc::c_ulong,
     1119000684 as libc::c_uint as libc::c_ulong,
     3686517206 as libc::c_uint as libc::c_ulong,
     2898065728 as libc::c_uint as libc::c_ulong,
     853044451 as libc::c_uint as libc::c_ulong,
     1172266101 as libc::c_uint as libc::c_ulong,
     3705015759 as libc::c_uint as libc::c_ulong,
     2882616665 as libc::c_uint as libc::c_ulong,
     651767980 as libc::c_uint as libc::c_ulong,
     1373503546 as libc::c_uint as libc::c_ulong,
     3369554304 as libc::c_uint as libc::c_ulong,
     3218104598 as libc::c_uint as libc::c_ulong,
     565507253 as libc::c_uint as libc::c_ulong,
     1454621731 as libc::c_uint as libc::c_ulong,
     3485111705 as libc::c_uint as libc::c_ulong,
     3099436303 as libc::c_uint as libc::c_ulong,
     671266974 as libc::c_uint as libc::c_ulong,
     1594198024 as libc::c_uint as libc::c_ulong,
     3322730930 as libc::c_uint as libc::c_ulong,
     2970347812 as libc::c_uint as libc::c_ulong,
     795835527 as libc::c_uint as libc::c_ulong,
     1483230225 as libc::c_uint as libc::c_ulong,
     3244367275 as libc::c_uint as libc::c_ulong,
     3060149565 as libc::c_uint as libc::c_ulong,
     1994146192 as libc::c_uint as libc::c_ulong,
     31158534 as libc::c_uint as libc::c_ulong,
     2563907772 as libc::c_uint as libc::c_ulong,
     4023717930 as libc::c_uint as libc::c_ulong,
     1907459465 as libc::c_uint as libc::c_ulong,
     112637215 as libc::c_uint as libc::c_ulong,
     2680153253 as libc::c_uint as libc::c_ulong,
     3904427059 as libc::c_uint as libc::c_ulong,
     2013776290 as libc::c_uint as libc::c_ulong,
     251722036 as libc::c_uint as libc::c_ulong,
     2517215374 as libc::c_uint as libc::c_ulong,
     3775830040 as libc::c_uint as libc::c_ulong,
     2137656763 as libc::c_uint as libc::c_ulong,
     141376813 as libc::c_uint as libc::c_ulong,
     2439277719 as libc::c_uint as libc::c_ulong,
     3865271297 as libc::c_uint as libc::c_ulong,
     1802195444 as libc::c_uint as libc::c_ulong,
     476864866 as libc::c_uint as libc::c_ulong,
     2238001368 as libc::c_uint as libc::c_ulong,
     4066508878 as libc::c_uint as libc::c_ulong,
     1812370925 as libc::c_uint as libc::c_ulong,
     453092731 as libc::c_uint as libc::c_ulong,
     2181625025 as libc::c_uint as libc::c_ulong,
     4111451223 as libc::c_uint as libc::c_ulong,
     1706088902 as libc::c_uint as libc::c_ulong,
     314042704 as libc::c_uint as libc::c_ulong,
     2344532202 as libc::c_uint as libc::c_ulong,
     4240017532 as libc::c_uint as libc::c_ulong,
     1658658271 as libc::c_uint as libc::c_ulong,
     366619977 as libc::c_uint as libc::c_ulong,
     2362670323 as libc::c_uint as libc::c_ulong,
     4224994405 as libc::c_uint as libc::c_ulong,
     1303535960 as libc::c_uint as libc::c_ulong,
     984961486 as libc::c_uint as libc::c_ulong,
     2747007092 as libc::c_uint as libc::c_ulong,
     3569037538 as libc::c_uint as libc::c_ulong,
     1256170817 as libc::c_uint as libc::c_ulong,
     1037604311 as libc::c_uint as libc::c_ulong,
     2765210733 as libc::c_uint as libc::c_ulong,
     3554079995 as libc::c_uint as libc::c_ulong,
     1131014506 as libc::c_uint as libc::c_ulong,
     879679996 as libc::c_uint as libc::c_ulong,
     2909243462 as libc::c_uint as libc::c_ulong,
     3663771856 as libc::c_uint as libc::c_ulong,
     1141124467 as libc::c_uint as libc::c_ulong,
     855842277 as libc::c_uint as libc::c_ulong,
     2852801631 as libc::c_uint as libc::c_ulong,
     3708648649 as libc::c_uint as libc::c_ulong,
     1342533948 as libc::c_uint as libc::c_ulong,
     654459306 as libc::c_uint as libc::c_ulong,
     3188396048 as libc::c_uint as libc::c_ulong,
     3373015174 as libc::c_uint as libc::c_ulong,
     1466479909 as libc::c_uint as libc::c_ulong,
     544179635 as libc::c_uint as libc::c_ulong,
     3110523913 as libc::c_uint as libc::c_ulong,
     3462522015 as libc::c_uint as libc::c_ulong,
     1591671054 as libc::c_uint as libc::c_ulong,
     702138776 as libc::c_uint as libc::c_ulong,
     2966460450 as libc::c_uint as libc::c_ulong,
     3352799412 as libc::c_uint as libc::c_ulong,
     1504918807 as libc::c_uint as libc::c_ulong,
     783551873 as libc::c_uint as libc::c_ulong,
     3082640443 as libc::c_uint as libc::c_ulong,
     3233442989 as libc::c_uint as libc::c_ulong,
     3988292384 as libc::c_uint as libc::c_ulong,
     2596254646 as libc::c_uint as libc::c_ulong,
     62317068 as libc::c_uint as libc::c_ulong,
     1957810842 as libc::c_uint as libc::c_ulong,
     3939845945 as libc::c_uint as libc::c_ulong,
     2647816111 as libc::c_uint as libc::c_ulong,
     81470997 as libc::c_uint as libc::c_ulong,
     1943803523 as libc::c_uint as libc::c_ulong,
     3814918930 as libc::c_uint as libc::c_ulong,
     2489596804 as libc::c_uint as libc::c_ulong,
     225274430 as libc::c_uint as libc::c_ulong,
     2053790376 as libc::c_uint as libc::c_ulong,
     3826175755 as libc::c_uint as libc::c_ulong,
     2466906013 as libc::c_uint as libc::c_ulong,
     167816743 as libc::c_uint as libc::c_ulong,
     2097651377 as libc::c_uint as libc::c_ulong,
     4027552580 as libc::c_uint as libc::c_ulong,
     2265490386 as libc::c_uint as libc::c_ulong,
     503444072 as libc::c_uint as libc::c_ulong,
     1762050814 as libc::c_uint as libc::c_ulong,
     4150417245 as libc::c_uint as libc::c_ulong,
     2154129355 as libc::c_uint as libc::c_ulong,
     426522225 as libc::c_uint as libc::c_ulong,
     1852507879 as libc::c_uint as libc::c_ulong,
     4275313526 as libc::c_uint as libc::c_ulong,
     2312317920 as libc::c_uint as libc::c_ulong,
     282753626 as libc::c_uint as libc::c_ulong,
     1742555852 as libc::c_uint as libc::c_ulong,
     4189708143 as libc::c_uint as libc::c_ulong,
     2394877945 as libc::c_uint as libc::c_ulong,
     397917763 as libc::c_uint as libc::c_ulong,
     1622183637 as libc::c_uint as libc::c_ulong,
     3604390888 as libc::c_uint as libc::c_ulong,
     2714866558 as libc::c_uint as libc::c_ulong,
     953729732 as libc::c_uint as libc::c_ulong,
     1340076626 as libc::c_uint as libc::c_ulong,
     3518719985 as libc::c_uint as libc::c_ulong,
     2797360999 as libc::c_uint as libc::c_ulong,
     1068828381 as libc::c_uint as libc::c_ulong,
     1219638859 as libc::c_uint as libc::c_ulong,
     3624741850 as libc::c_uint as libc::c_ulong,
     2936675148 as libc::c_uint as libc::c_ulong,
     906185462 as libc::c_uint as libc::c_ulong,
     1090812512 as libc::c_uint as libc::c_ulong,
     3747672003 as libc::c_uint as libc::c_ulong,
     2825379669 as libc::c_uint as libc::c_ulong,
     829329135 as libc::c_uint as libc::c_ulong,
     1181335161 as libc::c_uint as libc::c_ulong,
     3412177804 as libc::c_uint as libc::c_ulong,
     3160834842 as libc::c_uint as libc::c_ulong,
     628085408 as libc::c_uint as libc::c_ulong,
     1382605366 as libc::c_uint as libc::c_ulong,
     3423369109 as libc::c_uint as libc::c_ulong,
     3138078467 as libc::c_uint as libc::c_ulong,
     570562233 as libc::c_uint as libc::c_ulong,
     1426400815 as libc::c_uint as libc::c_ulong,
     3317316542 as libc::c_uint as libc::c_ulong,
     2998733608 as libc::c_uint as libc::c_ulong,
     733239954 as libc::c_uint as libc::c_ulong,
     1555261956 as libc::c_uint as libc::c_ulong,
     3268935591 as libc::c_uint as libc::c_ulong,
     3050360625 as libc::c_uint as libc::c_ulong,
     752459403 as libc::c_uint as libc::c_ulong,
     1541320221 as libc::c_uint as libc::c_ulong,
     2607071920 as libc::c_uint as libc::c_ulong,
     3965973030 as libc::c_uint as libc::c_ulong,
     1969922972 as libc::c_uint as libc::c_ulong,
     40735498 as libc::c_uint as libc::c_ulong,
     2617837225 as libc::c_uint as libc::c_ulong,
     3943577151 as libc::c_uint as libc::c_ulong,
     1913087877 as libc::c_uint as libc::c_ulong,
     83908371 as libc::c_uint as libc::c_ulong,
     2512341634 as libc::c_uint as libc::c_ulong,
     3803740692 as libc::c_uint as libc::c_ulong,
     2075208622 as libc::c_uint as libc::c_ulong,
     213261112 as libc::c_uint as libc::c_ulong,
     2463272603 as libc::c_uint as libc::c_ulong,
     3855990285 as libc::c_uint as libc::c_ulong,
     2094854071 as libc::c_uint as libc::c_ulong,
     198958881 as libc::c_uint as libc::c_ulong,
     2262029012 as libc::c_uint as libc::c_ulong,
     4057260610 as libc::c_uint as libc::c_ulong,
     1759359992 as libc::c_uint as libc::c_ulong,
     534414190 as libc::c_uint as libc::c_ulong,
     2176718541 as libc::c_uint as libc::c_ulong,
     4139329115 as libc::c_uint as libc::c_ulong,
     1873836001 as libc::c_uint as libc::c_ulong,
     414664567 as libc::c_uint as libc::c_ulong,
     2282248934 as libc::c_uint as libc::c_ulong,
     4279200368 as libc::c_uint as libc::c_ulong,
     1711684554 as libc::c_uint as libc::c_ulong,
     285281116 as libc::c_uint as libc::c_ulong,
     2405801727 as libc::c_uint as libc::c_ulong,
     4167216745 as libc::c_uint as libc::c_ulong,
     1634467795 as libc::c_uint as libc::c_ulong,
     376229701 as libc::c_uint as libc::c_ulong,
     2685067896 as libc::c_uint as libc::c_ulong,
     3608007406 as libc::c_uint as libc::c_ulong,
     1308918612 as libc::c_uint as libc::c_ulong,
     956543938 as libc::c_uint as libc::c_ulong,
     2808555105 as libc::c_uint as libc::c_ulong,
     3495958263 as libc::c_uint as libc::c_ulong,
     1231636301 as libc::c_uint as libc::c_ulong,
     1047427035 as libc::c_uint as libc::c_ulong,
     2932959818 as libc::c_uint as libc::c_ulong,
     3654703836 as libc::c_uint as libc::c_ulong,
     1088359270 as libc::c_uint as libc::c_ulong,
     936918000 as libc::c_uint as libc::c_ulong,
     2847714899 as libc::c_uint as libc::c_ulong,
     3736837829 as libc::c_uint as libc::c_ulong,
     1202900863 as libc::c_uint as libc::c_ulong,
     817233897 as libc::c_uint as libc::c_ulong,
     3183342108 as libc::c_uint as libc::c_ulong,
     3401237130 as libc::c_uint as libc::c_ulong,
     1404277552 as libc::c_uint as libc::c_ulong,
     615818150 as libc::c_uint as libc::c_ulong,
     3134207493 as libc::c_uint as libc::c_ulong,
     3453421203 as libc::c_uint as libc::c_ulong,
     1423857449 as libc::c_uint as libc::c_ulong,
     601450431 as libc::c_uint as libc::c_ulong,
     3009837614 as libc::c_uint as libc::c_ulong,
     3294710456 as libc::c_uint as libc::c_ulong,
     1567103746 as libc::c_uint as libc::c_ulong,
     711928724 as libc::c_uint as libc::c_ulong,
     3020668471 as libc::c_uint as libc::c_ulong,
     3272380065 as libc::c_uint as libc::c_ulong,
     1510334235 as libc::c_uint as libc::c_ulong,
     755167117 as libc::c_uint as libc::c_ulong];
/* Returns the CRC32 */
unsafe extern "C" fn CRC(mut data: *const libc::c_uchar, mut size: size_t)
 -> libc::c_ulong {
    let mut result: libc::c_ulong =
        0xffffffff as libc::c_uint as libc::c_ulong;
    while size > 0 as libc::c_int as libc::c_ulong {
        let fresh0 = data;
        data = data.offset(1);
        result =
            crc32_table[((result ^ *fresh0 as libc::c_ulong) &
                             0xff as libc::c_int as libc::c_ulong) as usize] ^
                result >> 8 as libc::c_int;
        size = size.wrapping_sub(1)
    }
    return result ^ 0xffffffff as libc::c_uint as libc::c_ulong;
}
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
/* Compresses the data according to the gzip specification, RFC 1952. */
#[no_mangle]
pub unsafe extern "C" fn ZopfliGzipCompress(mut options: *const ZopfliOptions,
                                            mut in_0: *const libc::c_uchar,
                                            mut insize: size_t,
                                            mut out: *mut *mut libc::c_uchar,
                                            mut outsize: *mut size_t) {
    let mut crcvalue: libc::c_ulong = CRC(in_0, insize);
    let mut bp: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 31 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* ID1 */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 139 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* ID2 */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 8 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* CM */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* FLG */
    /* MTIME */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 2 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* XFL, 2 indicates best compression. */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
    *(*out).offset(*outsize as isize) = 3 as libc::c_int as libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* OS follows Unix conventions. */
    ZopfliDeflate(options, 2 as libc::c_int, 1 as libc::c_int, in_0, insize,
                  &mut bp, out, outsize);
    /* CRC */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        crcvalue.wrapping_rem(256 as libc::c_int as libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (crcvalue >>
             8 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (crcvalue >>
             16 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (crcvalue >>
             24 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    /* ISIZE */
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        insize.wrapping_rem(256 as libc::c_int as libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (insize >>
             8 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (insize >>
             16 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        (insize >>
             24 as
                 libc::c_int).wrapping_rem(256 as libc::c_int as
                                               libc::c_ulong) as
            libc::c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if (*options).verbose != 0 {
        fprintf(stderr,
                b"Original Size: %d, Gzip: %d, Compression: %f%% Removed\n\x00"
                    as *const u8 as *const libc::c_char,
                insize as libc::c_int, *outsize as libc::c_int,
                100.0f64 * insize.wrapping_sub(*outsize) as libc::c_double /
                    insize as libc::c_double);
    };
}
