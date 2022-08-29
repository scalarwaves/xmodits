mod fmt;
use fmt::*;
#[allow(unused, dead_code)]
mod utils;
mod interface;
mod error;

pub use interface::{TrackerDumper, TrackerModule, TrackerSample};
pub use error::XmoditsError;
pub use utils::Error;

pub mod tracker_formats {
    pub use crate::it::ITFile;
    pub use crate::xm::XMFile;
    pub use crate::amig_mod::MODFile;
    pub use crate::s3m::S3MFile;
    pub use crate::umx::UMXFile;
}

pub fn load_module<P: AsRef<std::path::Path>>(path: P) -> Result<TrackerModule, XmoditsError> {
    use tracker_formats::*;

    let hint: String = file_extension(&path);

    match hint.as_str() {
        "it"    => ITFile::load_module(path),
        "xm"    => XMFile::load_module(path),
        "s3m"   => S3MFile::load_module(path),
        "mod"   => MODFile::load_module(path),
        // "umx"   => UMXFile::load_module(path),
        f       => return Err(
            XmoditsError::UnsupportedFormat(
                format!("'{}' is not a supported format.", f)
            )
        ),
    }
}

// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    (match p.as_ref().extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    }).to_string()
}