use brotli::enc::BrotliEncoderParams;
use libflate::gzip::Encoder;
use rauthy_error::ErrorResponse;
use std::io::{Cursor, Write};
use std::sync::LazyLock;

static BROTLI_PARAMS: LazyLock<BrotliEncoderParams> = LazyLock::new(BrotliEncoderParams::default);
static BROTLI_PARAMS_9: LazyLock<BrotliEncoderParams> = LazyLock::new(|| {
    let mut p = BrotliEncoderParams::default();
    p.quality = 9;
    p
});
// Params for dynamic compression which happens often like with every new request.
// We don't want to compress too high in that case because of diminishing returns.
// TODO test compression times for dyn compression and find best compromise
static BROTLI_PARAMS_DYN: LazyLock<BrotliEncoderParams> = LazyLock::new(|| {
    let mut p = BrotliEncoderParams::default();
    p.quality = 5;
    p
});

/// Brotli compression with max quality
#[inline]
pub fn compress_br(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    compress_br_with(input, &BROTLI_PARAMS)
}

/// Brotli compression with quality 9
#[inline]
pub fn compress_br_9(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    compress_br_with(input, &BROTLI_PARAMS_9)
}

/// Brotli compression with quality for dynamic content compression.
/// Provides a balance between compression ratio and CPU usage.
#[inline]
pub fn compress_br_dyn(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    compress_br_with(input, &BROTLI_PARAMS_DYN)
}

#[inline]
fn compress_br_with(input: &[u8], params: &BrotliEncoderParams) -> Result<Vec<u8>, ErrorResponse> {
    let mut writer = Cursor::new(Vec::with_capacity(input.len()));
    let mut reader = Cursor::new(input);
    brotli::BrotliCompress(&mut reader, &mut writer, params)?;
    Ok(writer.into_inner())
}

#[inline]
pub fn compress_gzip(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    let mut encoder = Encoder::new(Vec::new())?;
    encoder.write_all(input)?;
    Ok(encoder.finish().into_result()?)
}
