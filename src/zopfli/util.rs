use ::libc;
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
/* Initializes options with default values. */
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
pub unsafe extern "C" fn ZopfliInitOptions(mut options: *mut ZopfliOptions) {
    (*options).verbose = 0 as libc::c_int;
    (*options).verbose_more = 0 as libc::c_int;
    (*options).numiterations = 15 as libc::c_int;
    (*options).blocksplitting = 1 as libc::c_int;
    (*options).blocksplittinglast = 0 as libc::c_int;
    (*options).blocksplittingmax = 15 as libc::c_int;
}
