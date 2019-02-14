use std::fs;
use std::io::Write;
use std::vec::Vec;

use failure::Error;

use crate::analysis::{AnalysisSet, fn_analysis_note_noop};
use crate::interned_file::InternedFile;
use crate::patch::PatchDirection;
use crate::patch::unified::parser::parse_patch;
use crate::line_interner::LineInterner;


#[cfg(test)]
#[test]
fn all_files() -> Result<(), Error> {
    for entry in fs::read_dir("testdata/patching")? {
        // Skip everything that doesn't end with ".patch"
        let entry = entry?;
        let path = entry.path();
        match path.extension() {
            Some(extension) if extension == "patch" => {},
            _ => continue,
        }

        eprintln!("Testing patch {}", path.display());

        // Load and parse the patch
        let patch_data = fs::read(&path)?;
        let strip = 0;
        let mut patch = parse_patch(&patch_data, strip, true)?;

        // Check that there is exactly one FilePatch
        if patch.file_patches.len() != 1 {
            panic!("Test patch {} is for {} files, expected exactly one!", path.display(), patch.file_patches.len());
        }
        let file_patch = patch.file_patches.pop().unwrap();
        std::mem::drop(patch); // XXX: Not sure why I have to drop manually here, but otherwise I get strange borrow check error.

        // Load the target file
        // Note: In this case we always expect the old_filename to exist, so we
        //       select it directly.
        let file = fs::read(path.with_file_name(file_patch.old_filename().expect("old_filename missing!")))?;

        // Intern the patch and file
        let mut interner = LineInterner::new();
        let file_patch = file_patch.intern(&mut interner);
        let mut interned_file = InternedFile::new(&mut interner, &file, true);

        // Patch it
        let report = file_patch.apply(&mut interned_file, PatchDirection::Forward, 0, &AnalysisSet::default(), &fn_analysis_note_noop);

        // Check if it failed when shouldn't or succeeded when it was expected to fail
        let error_file = path.with_extension("error");
        let should_fail = error_file.exists();
        if should_fail {
            if report.ok() {
                panic!("The {} file exists, so apply failure is expected, but patch applied successfully!", error_file.display());
            }

            // We are done with this patch then
            continue
        }
        if !should_fail && report.failed() {
            panic!("The patch unexpectedly failed to apply!");
        }

        // Write the output to a buffer
        let mut output = Vec::<u8>::new();
        interned_file.write_to(&interner, &mut output)?;

        // Compare with the expected output
        let expected_output = fs::read(path.with_extension("out"))?;

        if output != expected_output {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            writeln!(stderr, "*** EXPECTED ***")?;
            stderr.write(&expected_output)?;
            writeln!(stderr, "*** WROTE ***")?;
            stderr.write(&output)?;

            // Try to save what we thought should be there, but ignore errors
            if let Ok(mut file) = fs::File::create(path.with_extension("out-bad")) {
                let _ = file.write_all(&output);
            }

            panic!("The patched file does not match the expected output!");
        }
    }

    Ok(())
}
