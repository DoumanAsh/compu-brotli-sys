use compu_brotli_sys::{BrotliEncoderVersion, BrotliDecoderVersion};

#[test]
fn verify_version() {
    let major = env!("CARGO_PKG_VERSION_MAJOR");
    let minor = env!("CARGO_PKG_VERSION_MINOR");
    let patch = env!("CARGO_PKG_VERSION_PATCH");

    let major = major.parse::<u32>().unwrap();
    let minor = minor.parse::<u32>().unwrap();
    let patch = patch.parse::<u32>().unwrap();

    let expected_version = major << 24 | minor << 12 | patch;
    let version = unsafe {
        BrotliEncoderVersion()
    };
    assert_eq!(version, expected_version);
    let version = unsafe {
        BrotliDecoderVersion()
    };
    assert_eq!(version, expected_version);
}
