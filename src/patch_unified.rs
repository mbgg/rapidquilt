    static ref DIFF_GIT: Regex = Regex::new(r"^diff --git +(?P<oldfilename>[^ ]+) +(?P<newfilename>[^ ]+)").unwrap();

        r"^git --diff ", // this is exactly what patch uses to recognize end of hunk-less filepatch
#[allow(unused)]
    GitDiffSeparator = 0,
    Index,
struct FilePatchMetadata<'a> {
    old_filename: Option<&'a [u8]>,
    new_filename: Option<&'a [u8]>,
impl<'a> Default for FilePatchMetadata<'a> {
            old_filename: None,
            new_filename: None,
fn new_filepatch<'a>(filepatch_metadata: &FilePatchMetadata, strip: usize) -> Result<Option<TextFilePatch<'a>>, Error> {
    if let (Some(old_filename), Some(new_filename)) = (filepatch_metadata.old_filename, filepatch_metadata.new_filename) {
        let (kind, filename, other_filename) = if old_filename == NULL_FILENAME {
            (FilePatchKind::Create, new_filename, None)
        } else if new_filename == NULL_FILENAME {
            (FilePatchKind::Delete, old_filename, None)
        } else {
            // TODO: What to do if new_filename and old_filename differ after stripping the beginning?

            (FilePatchKind::Modify, new_filename, Some(old_filename))
        };

        fn strip_filename(filename: &[u8], strip: usize) -> Result<PathBuf, Error> {
            let filename = PathBuf::from(OsStr::from_bytes(filename));
            if !filename.is_relative() {
                return Err(format_err!("Path in patch is not relative: \"{:?}\"", filename));
            }

            let mut components = filename.components();
            for _ in 0..strip { components.next(); }
            Ok(components.as_path().to_path_buf())
        }

        let filename = strip_filename(filename, strip)?;

        if filepatch_metadata.rename_from && filepatch_metadata.rename_to && other_filename.is_some() {
            let other_filename = strip_filename(other_filename.unwrap(), strip)?;
            Ok(Some(FilePatch::new_renamed(kind, other_filename, filename)))
        } else {
            Ok(Some(FilePatch::new(kind, filename)))
        }
    } else {
        Ok(None)
    }
}

    while let Some(line) = lines.peek() {
            lines.next();
        if let Some(capture) = MINUS_FILENAME.captures(line) {
            filepatch_metadata.old_filename = Some(capture.get(1).unwrap().as_bytes());
            lines.next();
            continue;
        }
        if let Some(capture) = PLUS_FILENAME.captures(line) {
            filepatch_metadata.new_filename = Some(capture.get(1).unwrap().as_bytes());
            lines.next();
            continue;
        }
        if let Some(capture) = DIFF_GIT.captures(line) {
            // patch uses "diff --git " as a separator that can mean a filepatch ended even if it had no hunks
            {
                if let Some(file_patch) = new_filepatch(&filepatch_metadata, strip)? {
                    file_patches.push(file_patch);
                filepatch_metadata = FilePatchMetadata::default();
            filepatch_metadata.old_filename = Some(capture.name("oldfilename").unwrap().as_bytes());
            filepatch_metadata.new_filename = Some(capture.name("newfilename").unwrap().as_bytes());
            lines.next();
            continue;
        }

        if !CHUNK.is_match(line) {
            lines.next();
            continue;
        }

        let mut file_patch = match new_filepatch(&filepatch_metadata, strip)? {
            Some(file_patch) => file_patch,
            None => {
                // TODO: Better error reporting...
                return Err(format_err!("Badly formated patch!"));
        filepatch_metadata = FilePatchMetadata::default();
                file_patch.change_kind(FilePatchKind::Create);
                file_patch.change_kind(FilePatchKind::Delete);
            if file_patch.kind() == FilePatchKind::Create {
            if file_patch.kind() == FilePatchKind::Delete {
                if !plus_newline_at_end && file_patch.kind() != FilePatchKind::Delete {
                if !minus_newline_at_end && file_patch.kind() != FilePatchKind::Create {
            file_patch.hunks.push(hunk);
        file_patches.push(file_patch);