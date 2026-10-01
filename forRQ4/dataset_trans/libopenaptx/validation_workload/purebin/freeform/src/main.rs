#![feature(extern_types)]
                                                           
                                                                     
mod libcall {
    use libopenaptx::src::openaptx as lib;
    pub type Ctx = *mut lib::aptx_context;

    pub unsafe fn init(hd: i32) -> Ctx {
        lib::aptx_init(hd)
    }
    pub unsafe fn finish(ctx: Ctx) {
        lib::aptx_finish(ctx)
    }
    pub unsafe fn reset(ctx: Ctx) {
        lib::aptx_reset(ctx)
    }
    pub unsafe fn encode(ctx: Ctx, input: *const u8, in_sz: usize, out: *mut u8, out_sz: usize, w: &mut usize) -> usize {
        lib::aptx_encode(ctx, input, in_sz, out, out_sz, w as *mut usize)
    }
    pub unsafe fn encode_finish(ctx: Ctx, out: *mut u8, out_sz: usize, w: &mut usize) -> i32 {
        lib::aptx_encode_finish(ctx, out, out_sz, w as *mut usize)
    }
    pub unsafe fn decode(ctx: Ctx, input: *const u8, in_sz: usize, out: *mut u8, out_sz: usize, w: &mut usize) -> usize {
        lib::aptx_decode(ctx, input, in_sz, out, out_sz, w as *mut usize)
    }
    pub unsafe fn decode_sync(ctx: Ctx, input: *const u8, in_sz: usize, out: *mut u8, out_sz: usize, w: &mut usize, s: &mut i32, d: &mut usize) -> usize {
        lib::aptx_decode_sync(ctx, input, in_sz, out, out_sz, w as *mut usize, s as *mut i32, d as *mut usize)
    }
    pub unsafe fn decode_sync_finish(ctx: Ctx) -> usize {
        lib::aptx_decode_sync_finish(ctx)
    }
}

include!("../../driver.rs");
