//! The values for static and dynamic compression were found by benchmarking the implementations
//! using a rather simple HTML file. The highest values are definitely NOT the best! The "static"
//! compression functions here are used for cached assets that only need to be compressed rarely.
//! The `*_dyn` ones are of e.g. /authroize, which needs to be dynamically compressed for each
//! single request. The very low compressions levels already provide quite a bit of benefit without
//! needing too many resources.
//!
//! For dynamic compression, lower levels of gzip actually have an advantage.
//!
//! Compression Benchmark:
//! Algorithm  | Level      |  Time (us) |      Ratio
//! --------------------------------------------------
//! Brotli     | 1          |         18 |       3.17x
//! Gzip       | 1          |         25 |       3.15x
//! ----------------------------------------------------
//! Brotli     | 2          |         24 |       3.31x
//! Gzip       | 2          |         19 |       3.45x
//! ----------------------------------------------------
//! Brotli     | 3          |         29 |       3.37x
//! Gzip       | 3          |         21 |       3.52x
//! ----------------------------------------------------
//! Brotli     | 4          |         41 |       3.64x
//! Gzip       | 4          |         24 |       3.57x
//! ----------------------------------------------------
//! Brotli     | 5          |         56 |       3.97x
//! Gzip       | 5          |         27 |       3.58x
//! ----------------------------------------------------
//! Brotli     | 6          |         68 |       3.97x
//! Gzip       | 6          |         38 |       3.59x
//! ----------------------------------------------------
//! Brotli     | 7          |        118 |       3.98x
//! Gzip       | 7          |         47 |       3.59x
//! ----------------------------------------------------
//! Brotli     | 8          |        181 |       3.98x
//! Gzip       | 8          |         59 |       3.60x
//! ----------------------------------------------------
//! Brotli     | 9          |        719 |       3.99x
//! Gzip       | 9          |         71 |       3.60x
//! ----------------------------------------------------
//! Brotli     | 10         |       2093 |       4.23x
//! Gzip       | 10         |         71 |       3.60x
//! ----------------------------------------------------
//! Brotli     | 11         |       5393 |       4.31x
//! Gzip       | 11         |         71 |       3.60x
//! ---------------------------------------------------
//!
//! This benchmark shows that there is basically nothing happening between brotli level 5 and 10,
//! apart from higher resource consumption, and the small improvements at level 10 are tiny compared
//! to the additional time taken. For gzip, level 2 is the best for speed, and it does not make any
//! sense to go higher than 5 for static content.
//! When it's about the best value overall, gzip level 2 wins easily.
use actix_web::body::{BodyStream, BoxBody};
use async_compression::brotli;
use async_compression::tokio::bufread::{BrotliEncoder, GzipEncoder};
use rauthy_error::ErrorResponse;
use std::io::Cursor;
use tokio::io::BufReader;
use tokio_util::io::ReaderStream;

/// Brotli compression with good quality for static content
#[inline]
pub async fn compress_br(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    let level = async_compression::Level::Precise(5);

    let buf_reader = BufReader::new(Cursor::new(input));
    let params = brotli::EncoderParams::default().quality(level);
    let mut compressor = BrotliEncoder::with_params(buf_reader, params);

    let mut bytes = Vec::with_capacity(input.len() / 3);
    tokio::io::copy(&mut compressor, &mut bytes).await?;

    Ok(bytes)
}

/// Brotli compression with quality for dynamic content compression.
/// Provides a balance between compression ratio and CPU usage.
#[inline]
pub fn compress_br_dyn(input: String) -> BoxBody {
    let level = async_compression::Level::Precise(1);

    let buf_reader = BufReader::new(Cursor::new(input));
    let params = brotli::EncoderParams::default().quality(level);
    let compressor = BrotliEncoder::with_params(buf_reader, params);

    BoxBody::new(BodyStream::new(ReaderStream::new(compressor)))
}

#[inline]
pub async fn compress_gzip(input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    let level = async_compression::Level::Precise(5);

    let buf_reader = BufReader::new(Cursor::new(input));
    let mut compressor = GzipEncoder::with_quality(buf_reader, level);

    let mut bytes = Vec::with_capacity(input.len() / 3);
    tokio::io::copy(&mut compressor, &mut bytes).await?;

    Ok(bytes)
}

#[inline]
pub fn compress_gzip_dyn(input: String) -> BoxBody {
    let level = async_compression::Level::Precise(2);

    let buf_reader = BufReader::new(Cursor::new(input));
    let compressor = GzipEncoder::with_quality(buf_reader, level);

    BoxBody::new(BodyStream::new(ReaderStream::new(compressor)))
}
