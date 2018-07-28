pub const IFF_TUN: u32 = 1;
pub const IFF_TAP: u32 = 2;
pub const IFF_NAPI: u32 = 16;
pub const IFF_NAPI_FRAGS: u32 = 32;
pub const IFF_NO_PI: u32 = 4096;
pub const IFF_ONE_QUEUE: u32 = 8192;
pub const IFF_VNET_HDR: u32 = 16384;
pub const IFF_TUN_EXCL: u32 = 32768;
pub const IFF_MULTI_QUEUE: u32 = 256;
pub const IFF_ATTACH_QUEUE: u32 = 512;
pub const IFF_DETACH_QUEUE: u32 = 1024;
pub const IFF_PERSIST: u32 = 2048;
pub const IFF_NOFILTER: u32 = 4096;

pub const IFNAMSIZ: u32 = 16;

/* automatically generated by rust-bindgen */

pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type sa_family_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ifmap {
    pub mem_start: ::std::os::raw::c_ulong,
    pub mem_end: ::std::os::raw::c_ulong,
    pub base_addr: ::std::os::raw::c_ushort,
    pub irq: ::std::os::raw::c_uchar,
    pub dma: ::std::os::raw::c_uchar,
    pub port: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_ifmap() {
    assert_eq!(
        ::std::mem::size_of::<ifmap>(),
        24usize,
        concat!("Size of: ", stringify!(ifmap))
    );
    assert_eq!(
        ::std::mem::align_of::<ifmap>(),
        8usize,
        concat!("Alignment of ", stringify!(ifmap))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).mem_start as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(mem_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).mem_end as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(mem_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).base_addr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(base_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).irq as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(irq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).dma as *const _ as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(dma)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifmap>())).port as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ifmap),
            "::",
            stringify!(port)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_ifrn: ifreq__bindgen_ty_1,
    pub ifr_ifru: ifreq__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq__bindgen_ty_1 {
    pub ifrn_name: [::std::os::raw::c_char; 16usize],
    _bindgen_union_align: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_ifreq__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ifreq__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ifreq__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ifreq__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(ifreq__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_1>())).ifrn_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_1),
            "::",
            stringify!(ifrn_name)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq__bindgen_ty_2 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: ::std::os::raw::c_short,
    pub ifru_ivalue: ::std::os::raw::c_int,
    pub ifru_mtu: ::std::os::raw::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [::std::os::raw::c_char; 16usize],
    pub ifru_newname: [::std::os::raw::c_char; 16usize],
    pub ifru_data: __caddr_t,
    _bindgen_union_align: [u64; 3usize],
}
#[test]
fn bindgen_test_layout_ifreq__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<ifreq__bindgen_ty_2>(),
        24usize,
        concat!("Size of: ", stringify!(ifreq__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<ifreq__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(ifreq__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_dstaddr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_dstaddr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_broadaddr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_broadaddr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_netmask as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_netmask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_hwaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_hwaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_ivalue as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_ivalue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_mtu as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_mtu)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_map as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_map)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_slave as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_slave)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_newname as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_newname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq__bindgen_ty_2>())).ifru_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq__bindgen_ty_2),
            "::",
            stringify!(ifru_data)
        )
    );
}
#[test]
fn bindgen_test_layout_ifreq() {
    assert_eq!(
        ::std::mem::size_of::<ifreq>(),
        40usize,
        concat!("Size of: ", stringify!(ifreq))
    );
    assert_eq!(
        ::std::mem::align_of::<ifreq>(),
        8usize,
        concat!("Alignment of ", stringify!(ifreq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq>())).ifr_ifrn as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq),
            "::",
            stringify!(ifr_ifrn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ifreq>())).ifr_ifru as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ifreq),
            "::",
            stringify!(ifr_ifru)
        )
    );
}