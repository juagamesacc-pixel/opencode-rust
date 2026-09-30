// source: src/util/media.ts — exports: isPdfAttachment, isMedia,
// isImageAttachment, sniffAttachmentMime (all verbatim).

/// source: isPdfAttachment — verbatim.
pub fn is_pdf_attachment(mime: &str) -> bool {
    mime == "application/pdf"
}

/// source: isMedia — verbatim.
pub fn is_media(mime: &str) -> bool {
    mime.starts_with("image/") || is_pdf_attachment(mime)
}

/// source: isImageAttachment — verbatim exclusions.
pub fn is_image_attachment(mime: &str) -> bool {
    mime.starts_with("image/") && mime != "image/svg+xml" && mime != "image/vnd.fastbidsheet"
}

/// source: sniffAttachmentMime — verbatim magic-byte table + fallback.
pub fn sniff_attachment_mime(bytes: &[u8], fallback: &str) -> String {
    let starts = |prefix: &[u8]| bytes.len() >= prefix.len() && &bytes[..prefix.len()] == prefix;
    if starts(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]) {
        return "image/png".to_string();
    }
    if starts(&[0xff, 0xd8, 0xff]) {
        return "image/jpeg".to_string();
    }
    if starts(&[0x47, 0x49, 0x46, 0x38]) {
        return "image/gif".to_string();
    }
    if starts(&[0x42, 0x4d]) {
        return "image/bmp".to_string();
    }
    if starts(&[0x25, 0x50, 0x44, 0x46, 0x2d]) {
        return "application/pdf".to_string();
    }
    if starts(&[0x52, 0x49, 0x46, 0x46])
        && bytes.len() >= 12
        && bytes[8..12] == [0x57, 0x45, 0x42, 0x50]
    {
        return "image/webp".to_string();
    }
    fallback.to_string()
}
