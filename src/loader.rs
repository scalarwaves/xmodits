/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::tracker_formats::*;
use crate::TrackerDumper;
use crate::TrackerModule;
use crate::XmoditsError;
use phf::phf_map;

const MAX_FILESIZE_MB: u64 = 1024 * 1024 * 64;

type ModLoaderFunc = fn(Vec<u8>) -> Result<TrackerModule, XmoditsError>;
type ModValidatorFunc = fn(&[u8]) -> Result<(), XmoditsError>;

fn validate<T: TrackerDumper>(buf: &[u8]) -> Result<(), XmoditsError> {
    T::validate(buf)
}

fn load<T: TrackerDumper>(buf: Vec<u8>) -> Result<TrackerModule, XmoditsError> {
    T::load_from_buf_unchecked(buf)
}

pub static LOADERS: phf::Map<&str, (ModValidatorFunc, ModLoaderFunc)> = phf_map! {
    "it" => (validate::<ITFile>, load::<ITFile>),
    "xm" => (validate::<XMFile>, load::<XMFile>),
    "s3m" => (validate::<S3MFile>, load::<S3MFile>),
    "umx" => (validate::<UMXFile>, load::<UMXFile>),
    "mod" => (validate::<MODFile>, load::<MODFile>),
};

/// A more robust method to load a module gven a path.
///
/// Load a module given a file extension.
///
/// If it fails, loop through other module loaders, return if one succeeds.
pub fn load_module<P>(path: P) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let ext = file_extension(&path).to_lowercase();
    load_from_ext(path, &ext)
}

pub fn load_from_ext<P>(path: P, ext: &str) -> Result<TrackerModule, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    let buf: Vec<u8> = load_to_buf(path)?;

    match LOADERS.get(ext) {
        Some((validator, loader)) => {
            if let Err(original_err) = validator(&buf) {
                for (_, (validator_bak, loader_bak)) in
                    LOADERS.entries().filter(|(k, _)| !["mod", ext].contains(k))
                {
                    if validator_bak(&buf).is_ok() {
                        return loader_bak(buf);
                    }
                }
                Err(original_err)
            } else {
                loader(buf)
            }
        }
        None => Err(XmoditsError::UnsupportedFormat(format!(
            "'{}' is not a supported format.",
            ext
        ))),
    }
}

pub fn load_to_buf<P>(path: P) -> Result<Vec<u8>, XmoditsError>
where
    P: AsRef<std::path::Path>,
{
    if std::fs::metadata(&path)?.len() > MAX_FILESIZE_MB {
        return Err(XmoditsError::file(
            "File provided is larger than 64MB. No tracker module should ever be close to that",
        ));
    }

    Ok(std::fs::read(&path)?)
}

/// Function to get file extension from path.
fn file_extension<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
