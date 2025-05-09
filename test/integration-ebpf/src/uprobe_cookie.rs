#![no_std]
#![no_main]

use aya_ebpf::{
    EbpfContext as _, helpers,
    macros::{map, uprobe},
    maps::RingBuf,
    programs::ProbeContext,
};
#[cfg(not(test))]
extern crate ebpf_panic;

#[map]
static RING_BUF: RingBuf = RingBuf::with_byte_size(0, 0);

#[uprobe]
pub fn uprobe_cookie(ctx: ProbeContext) {
    let cookie = unsafe { helpers::bpf_get_attach_cookie(ctx.as_ptr()) };
    let cookie_bytes = cookie.to_le_bytes();
    let _res = RING_BUF.output(&cookie_bytes, 0);
}
