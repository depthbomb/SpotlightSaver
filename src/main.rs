use std::io::{self, Read};
use std::process::ExitCode;

#[cfg(target_os = "windows")]
use std::collections::HashSet;
#[cfg(target_os = "windows")]
use std::ffi::{OsString, c_void};
#[cfg(target_os = "windows")]
use std::fs::{self, File, OpenOptions};
#[cfg(target_os = "windows")]
use std::io::{BufReader, Seek, SeekFrom, Write};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::ptr;

#[cfg(target_os = "windows")]
#[repr(C)]
struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[cfg(target_os = "windows")]
const FOLDER_ID_LOCAL_APP_DATA: Guid = Guid {
    data1: 0xf1b32785,
    data2: 0x6fba,
    data3: 0x4fcf,
    data4: [0x9d, 0x55, 0x7b, 0x8e, 0x7f, 0x15, 0x70, 0x91],
};
#[cfg(target_os = "windows")]
const FOLDER_ID_PICTURES: Guid = Guid {
    data1: 0x33e28130,
    data2: 0x4e1e,
    data3: 0x4676,
    data4: [0x83, 0x5a, 0x98, 0x39, 0x5c, 0x3b, 0xc3, 0xbb],
};

#[cfg(target_os = "windows")]
#[link(name = "shell32")]
// SAFETY: This declaration matches the Windows SDK signature.
unsafe extern "system" {
    fn SHGetKnownFolderPath(
        folder_id: *const Guid,
        flags: u32,
        token: *mut c_void,
        path: *mut *mut u16,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "ole32")]
// SAFETY: This declaration matches the Windows SDK signature.
unsafe extern "system" {
    fn CoTaskMemFree(memory: *mut c_void);
}

fn main() -> ExitCode {
    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("Spotlight Saver is not supported on this system");

        return ExitCode::FAILURE;
    }

    #[cfg(target_os = "windows")]
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");

            ExitCode::FAILURE
        }
    }
}

fn jpeg_dimensions<R: Read>(reader: &mut R) -> io::Result<Option<(u16, u16)>> {
    let mut signature = [0_u8; 2];
    if !read_exact_or_invalid(reader, &mut signature)? || signature != [0xff, 0xd8] {
        return Ok(None);
    }

    loop {
        let mut byte = [0_u8; 1];
        if !read_exact_or_invalid(reader, &mut byte)? || byte[0] != 0xff {
            return Ok(None);
        }

        while byte[0] == 0xff {
            if !read_exact_or_invalid(reader, &mut byte)? {
                return Ok(None);
            }
        }

        let marker = byte[0];
        if marker == 0x00 || marker == 0xd8 || marker == 0xd9 || marker == 0xda {
            return Ok(None);
        }

        if marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            continue;
        }

        let mut length_bytes = [0_u8; 2];
        if !read_exact_or_invalid(reader, &mut length_bytes)? {
            return Ok(None);
        }

        let segment_length = u16::from_be_bytes(length_bytes);
        if segment_length < 2 {
            return Ok(None);
        }

        if is_start_of_frame(marker) {
            if segment_length < 8 {
                return Ok(None);
            }

            let mut frame_header = [0_u8; 6];
            if !read_exact_or_invalid(reader, &mut frame_header)? {
                return Ok(None);
            }

            let height = u16::from_be_bytes([frame_header[1], frame_header[2]]);
            let width = u16::from_be_bytes([frame_header[3], frame_header[4]]);
            let component_count = u16::from(frame_header[5]);
            let expected_length = 8 + 3 * component_count;
            if width == 0
                || height == 0
                || component_count == 0
                || segment_length != expected_length
            {
                return Ok(None);
            }

            return Ok(Some((width, height)));
        }

        if !skip_exact(reader, u64::from(segment_length - 2))? {
            return Ok(None);
        }
    }
}

fn is_start_of_frame(marker: u8) -> bool {
    matches!(
        marker,
        0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf
    )
}

fn read_exact_or_invalid<R: Read>(reader: &mut R, buffer: &mut [u8]) -> io::Result<bool> {
    match reader.read_exact(buffer) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
        Err(error) => Err(error),
    }
}

fn skip_exact<R: Read>(reader: &mut R, length: u64) -> io::Result<bool> {
    let copied = io::copy(&mut reader.take(length), &mut io::sink())?;

    Ok(copied == length)
}

#[cfg(target_os = "windows")]
fn run() -> Result<(), String> {
    let pictures_folder = known_folder_path(&FOLDER_ID_PICTURES)
        .map_err(|error| format!("Unable to locate the Pictures folder: {error}"))?;
    let destination_folder = pictures_folder.join("Windows Spotlight");
    let spotlight_folder = known_folder_path(&FOLDER_ID_LOCAL_APP_DATA)
        .map_err(|error| format!("Unable to locate the local AppData folder: {error}"))?
        .join("Packages")
        .join("Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy")
        .join("LocalState")
        .join("Assets");

    if !spotlight_folder.is_dir() {
        return Err(format!(
            "Windows Spotlight assets folder not found: {:?}",
            spotlight_folder
        ));
    }

    fs::create_dir_all(&destination_folder)
        .map_err(|error| format!("Unable to create {:?}: {error}", destination_folder))?;

    save_spotlights(&spotlight_folder, &destination_folder)
}

#[cfg(target_os = "windows")]
fn known_folder_path(folder_id: &Guid) -> io::Result<PathBuf> {
    let mut raw_path = ptr::null_mut();
    // SAFETY: The folder ID and output pointer are valid, and a null token requests the current user.
    let result = unsafe { SHGetKnownFolderPath(folder_id, 0, ptr::null_mut(), &mut raw_path) };

    if result < 0 {
        if !raw_path.is_null() {
            // SAFETY: Windows allocated this non-null pointer with the COM task allocator.
            unsafe { CoTaskMemFree(raw_path.cast()) };
        }

        return Err(io::Error::other(format!(
            "SHGetKnownFolderPath failed with HRESULT 0x{result:08x}"
        )));
    }

    if raw_path.is_null() {
        return Err(io::Error::other(
            "SHGetKnownFolderPath returned a null path",
        ));
    }

    let mut length = 0;
    // SAFETY: A successful call returns a valid, null-terminated UTF-16 string.
    while unsafe { *raw_path.add(length) } != 0 {
        length += 1;
    }

    // SAFETY: The scan above established that this many initialized code units precede the null.
    let path = PathBuf::from(OsString::from_wide(unsafe {
        std::slice::from_raw_parts(raw_path, length)
    }));

    // SAFETY: Windows allocated this non-null pointer with the COM task allocator.
    unsafe { CoTaskMemFree(raw_path.cast()) };

    Ok(path)
}

#[cfg(target_os = "windows")]
fn save_spotlights(source_folder: &Path, destination_folder: &Path) -> Result<(), String> {
    let entries = fs::read_dir(source_folder)
        .map_err(|error| format!("Unable to read {:?}: {error}", source_folder))?;
    let destination_entries = fs::read_dir(destination_folder)
        .map_err(|error| format!("Unable to read {:?}: {error}", destination_folder))?;
    let mut existing_names = HashSet::new();
    let mut error_count = 0_u32;
    let mut saved_count = 0_u32;
    let mut skipped_count = 0_u32;

    for entry_result in destination_entries {
        match entry_result {
            Ok(entry) => {
                existing_names.insert(entry.file_name());
            }
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to read a destination directory entry: {error}");
            }
        }
    }

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to read an asset directory entry: {error}");

                continue;
            }
        };
        let source_path = entry.path();
        let mut destination_name = entry.file_name();
        destination_name.push(".jpg");
        let destination_path = destination_folder.join(&destination_name);

        if existing_names.contains(&destination_name) {
            skipped_count += 1;

            continue;
        }

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to inspect {:?}: {error}", source_path);

                continue;
            }
        };

        if !file_type.is_file() {
            continue;
        }

        let source_file = match File::open(&source_path) {
            Ok(file) => file,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to open {:?}: {error}", source_path);

                continue;
            }
        };
        let mut source = BufReader::new(source_file);
        let dimensions = match jpeg_dimensions(&mut source) {
            Ok(dimensions) => dimensions,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to inspect {:?}: {error}", source_path);

                continue;
            }
        };

        if !matches!(dimensions, Some((width, height)) if width >= 1920 && height >= 1080) {
            continue;
        }

        let has_end_marker = match jpeg_has_end_marker(&mut source) {
            Ok(has_end_marker) => has_end_marker,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to inspect {:?}: {error}", source_path);

                continue;
            }
        };
        if !has_end_marker {
            continue;
        }

        match copy_new(&mut source, &destination_path) {
            Ok(true) => saved_count += 1,
            Ok(false) => skipped_count += 1,
            Err(error) => {
                error_count += 1;
                eprintln!("Unable to copy {:?}: {error}", source_path);
            }
        }
    }

    if error_count > 0 {
        return Err(format!("Finished with {error_count} asset error(s)"));
    }

    println!("Saved {saved_count} new wallpaper(s); skipped {skipped_count} existing wallpaper(s)");

    Ok(())
}

#[cfg(target_os = "windows")]
fn jpeg_has_end_marker(source: &mut BufReader<File>) -> io::Result<bool> {
    let length = source.seek(SeekFrom::End(0))?;
    if length < 2 {
        return Ok(false);
    }

    source.seek(SeekFrom::End(-2))?;

    let mut end_marker = [0_u8; 2];
    if !read_exact_or_invalid(source, &mut end_marker)? {
        return Ok(false);
    }

    Ok(end_marker == [0xff, 0xd9])
}

#[cfg(target_os = "windows")]
fn copy_new(source: &mut BufReader<File>, destination_path: &Path) -> io::Result<bool> {
    source.seek(SeekFrom::Start(0))?;

    let mut destination = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination_path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => return Ok(false),
        Err(error) => return Err(error),
    };
    let copy_result = io::copy(source, &mut destination).and_then(|_| destination.flush());

    if let Err(error) = copy_result {
        drop(destination);
        let _ = fs::remove_file(destination_path);

        return Err(error);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::jpeg_dimensions;
    use std::io::{self, Cursor, Read};

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("test failure"))
        }
    }

    #[test]
    fn reads_baseline_jpeg_dimensions_without_decoding_pixels() {
        let jpeg = [
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x04, 0x12, 0x34, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x04,
            0x38, 0x07, 0x80, 0x03,
        ];

        assert_eq!(
            jpeg_dimensions(&mut Cursor::new(jpeg)).unwrap(),
            Some((1920, 1080))
        );
    }

    #[test]
    fn reads_progressive_jpeg_after_fill_and_standalone_markers() {
        let jpeg = [
            0xff, 0xd8, 0xff, 0xff, 0x01, 0xff, 0xc2, 0x00, 0x11, 0x08, 0x08, 0x70, 0x0f, 0x00,
            0x03,
        ];

        assert_eq!(
            jpeg_dimensions(&mut Cursor::new(jpeg)).unwrap(),
            Some((3840, 2160))
        );
    }

    #[test]
    fn rejects_non_jpeg_and_truncated_jpeg_data() {
        assert_eq!(
            jpeg_dimensions(&mut Cursor::new(b"not a jpeg")).unwrap(),
            None
        );
        assert_eq!(
            jpeg_dimensions(&mut Cursor::new([0xff, 0xd8, 0xff, 0xe0, 0x00, 0x08])).unwrap(),
            None
        );
    }

    #[test]
    fn rejects_zero_dimensions_and_scan_data_before_a_frame() {
        let zero_width = [
            0xff, 0xd8, 0xff, 0xc0, 0x00, 0x0b, 0x08, 0x04, 0x38, 0x00, 0x00, 0x01,
        ];
        let early_scan = [0xff, 0xd8, 0xff, 0xda];

        assert_eq!(jpeg_dimensions(&mut Cursor::new(zero_width)).unwrap(), None);
        assert_eq!(jpeg_dimensions(&mut Cursor::new(early_scan)).unwrap(), None);
    }

    #[test]
    fn propagates_io_errors() {
        let error = jpeg_dimensions(&mut FailingReader).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::Other);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn resolves_required_windows_known_folders() {
        assert!(
            super::known_folder_path(&super::FOLDER_ID_LOCAL_APP_DATA)
                .unwrap()
                .is_dir()
        );
        assert!(
            super::known_folder_path(&super::FOLDER_ID_PICTURES)
                .unwrap()
                .is_dir()
        );
    }
}
