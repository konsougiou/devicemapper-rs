// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code, non_camel_case_types)]

// Was once automatically generated by rust-bindgen,
// but has been edited since then.

use std::borrow::Cow;
use std::fmt;

use device::Device;
use util::slice_to_null;

pub type __s8 = ::libc::c_char;
pub type __u8 = ::libc::c_uchar;
pub type __s16 = ::libc::c_short;
pub type __u16 = ::libc::c_ushort;
pub type __s32 = ::libc::c_int;
pub type __u32 = ::libc::c_uint;
pub type __s64 = ::libc::c_longlong;
pub type __u64 = ::libc::c_ulonglong;

#[repr(C)]
pub struct Struct_dm_ioctl {
    pub version: [__u32; 3usize],
    pub data_size: __u32,
    pub data_start: __u32,
    pub target_count: __u32,
    pub open_count: __s32,
    pub flags: __u32,
    pub event_nr: __u32,
    pub padding: __u32,
    pub dev: __u64,
    pub name: [u8; 128usize],
    pub uuid: [u8; 129usize],
    pub data: [u8; 7usize],
}
impl ::std::clone::Clone for Struct_dm_ioctl {
    fn clone(&self) -> Self {
        let mut name: [u8; 128usize] = [0; 128usize];
        name.copy_from_slice(&self.name);

        let mut uuid: [u8; 129usize] = [0; 129usize];
        uuid.copy_from_slice(&self.uuid);

        let mut data: [u8; 7usize] = [0; 7usize];
        data.copy_from_slice(&self.data);

        Struct_dm_ioctl {
            version: self.version,
            data_size: self.data_size,
            data_start: self.data_start,
            target_count: self.target_count,
            open_count: self.open_count,
            flags: self.flags,
            event_nr: self.event_nr,
            padding: self.padding,
            dev: self.dev,
            name,
            uuid,
            data,
        }
    }
}
impl ::std::default::Default for Struct_dm_ioctl {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl fmt::Debug for Struct_dm_ioctl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Struct_dm_ioctl")
            .field("version", &self.version)
            .field("data_size", &self.data_size)
            .field("data_start", &self.data_start)
            .field("target_count", &self.target_count)
            .field("open_count", &self.open_count)
            .field("flags", &self.flags)
            .field("event_nr", &self.event_nr)
            .field("dev", &Device::from_kdev_t(self.dev as u32))
            .field(
                "name",
                &slice_to_null(&self.name)
                    .map(|s| String::from_utf8_lossy(s))
                    .unwrap_or_else(|| Cow::Borrowed("kernel bug: unterminated dm_ioctl.name")),
            )
            .field(
                "uuid",
                &slice_to_null(&self.uuid)
                    .map(|s| String::from_utf8_lossy(s))
                    .unwrap_or_else(|| Cow::Borrowed("kernel bug: unterminated dm_ioctl.uuid")),
            )
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_dm_target_spec {
    pub sector_start: __u64,
    pub length: __u64,
    pub status: __s32,
    pub next: __u32,
    pub target_type: [u8; 16usize],
}
impl ::std::default::Default for Struct_dm_target_spec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Struct_dm_target_deps {
    pub count: __u32,
    pub padding: __u32,
    pub dev: [__u64; 0usize],
}
impl ::std::default::Default for Struct_dm_target_deps {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Struct_dm_name_list {
    pub dev: __u64,
    pub next: __u32,
    pub name: [u8; 0usize],
}
impl ::std::default::Default for Struct_dm_name_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Struct_dm_target_versions {
    pub next: __u32,
    pub version: [__u32; 3usize],
    pub name: [u8; 0usize],
}
impl ::std::default::Default for Struct_dm_target_versions {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Struct_dm_target_msg {
    pub sector: __u64,
    pub message: [u8; 0usize],
}
impl ::std::default::Default for Struct_dm_target_msg {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const DM_VERSION_CMD: ::libc::c_uint = 0;
pub const DM_REMOVE_ALL_CMD: ::libc::c_uint = 1;
pub const DM_LIST_DEVICES_CMD: ::libc::c_uint = 2;
pub const DM_DEV_CREATE_CMD: ::libc::c_uint = 3;
pub const DM_DEV_REMOVE_CMD: ::libc::c_uint = 4;
pub const DM_DEV_RENAME_CMD: ::libc::c_uint = 5;
pub const DM_DEV_SUSPEND_CMD: ::libc::c_uint = 6;
pub const DM_DEV_STATUS_CMD: ::libc::c_uint = 7;
pub const DM_DEV_WAIT_CMD: ::libc::c_uint = 8;
pub const DM_TABLE_LOAD_CMD: ::libc::c_uint = 9;
pub const DM_TABLE_CLEAR_CMD: ::libc::c_uint = 10;
pub const DM_TABLE_DEPS_CMD: ::libc::c_uint = 11;
pub const DM_TABLE_STATUS_CMD: ::libc::c_uint = 12;
pub const DM_LIST_VERSIONS_CMD: ::libc::c_uint = 13;
pub const DM_TARGET_MSG_CMD: ::libc::c_uint = 14;
pub const DM_DEV_SET_GEOMETRY_CMD: ::libc::c_uint = 15;
pub const DM_DEV_ARM_POLL_CMD: ::libc::c_uint = 16;
