use brotli::{CompressorWriter, DecompressorWriter};
use std::io;
use wasm_bindgen::prelude::*;

/// Compresses binary data using Brotli.
/// Quality must be a number from 0-11.
/// The `lg_window` size should be between
/// 20 and 21 (recommended).
///
/// See [brotli](https://crates.io/crates/brotli).
#[wasm_bindgen(js_name = "compressBrotli")]
pub fn brotli_compress(mut data: &[u8], quality: Option<u32>, lg_window: Option<u32>) -> Vec<u8> {
	let mut writer = CompressorWriter::new(
		Vec::new(),
		4096,
		quality.unwrap_or(8),
		lg_window.unwrap_or(21),
	);
	io::copy(&mut data, &mut writer).unwrap();
	writer.into_inner()
}

/// Decompresses brotli compressed data.
#[wasm_bindgen(js_name = "decompressBrotli")]
pub fn brotli_decompress(mut data: &[u8]) -> Option<Vec<u8>> {
	let mut reader = DecompressorWriter::new(Vec::new(), 4096);
	io::copy(&mut data, &mut reader).unwrap();
	match reader.into_inner() {
		Ok(buffer) => Some(buffer),
		Err(_e) => None,
	}
}

/// Same as `brotli_compress` but for strings.
#[wasm_bindgen(js_name = "compressStringBrotli")]
pub fn brotli_compress_string(
	string: &str,
	quality: Option<u32>,
	lg_window: Option<u32>,
) -> Vec<u8> {
	brotli_compress(string.as_bytes(), quality, lg_window)
}

/// Same as `brotli_decompress` but for strings.
#[wasm_bindgen(js_name = "decompressStringBrotli")]
pub fn gbrotli_decompress_string(data: &[u8]) -> Option<String> {
	brotli_decompress(data).map(|decoded| bytes_to_string(&decoded))
}

#[wasm_bindgen(js_name = "stringToUtf8")]
pub fn string_to_bytes(input: &str) -> Vec<u8> {
	input.as_bytes().to_vec()
}

#[wasm_bindgen(js_name = "utf8ToString")]
pub fn bytes_to_string(input: &[u8]) -> String {
	String::from_utf8_lossy(input).to_string()
}
