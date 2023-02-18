#![no_std]
#![no_main]

use aya_bpf::{
    macros::uprobe,
    macros::uretprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[uprobe(name="ts_parser_parse")]
pub fn ts_parser_parse(ctx: ProbeContext) -> u32 {
    match try_ts_parser_parse(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_ts_parser_parse(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function ts_parser_parse called by nvim");
    Ok(0)
}

#[uretprobe(name="ts_parser_parse_ret")]
pub fn ts_parser_parse_ret(ctx: ProbeContext) -> u32 {
    match try_ts_parser_parse_ret(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_ts_parser_parse_ret(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function ts_parser_parse returned");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
