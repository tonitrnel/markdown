pub(crate) mod entities;
mod is_punctuation_or_symbol;
pub(crate) mod percent_encode;
mod text_encoding;

pub(crate) use entities::escape_xml;
pub(crate) use entities::lookup_entity;
pub(crate) use entities::unescape_string;
pub(crate) use is_punctuation_or_symbol::is_punctuation_or_symbol;

#[cfg(debug_assertions)]
static FUSE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[allow(unused)]
#[cfg(debug_assertions)]
pub(crate) fn fuse(max: usize) {
    if FUSE.load(std::sync::atomic::Ordering::SeqCst) >= max {
        panic!("Emergency fuse")
    }
    FUSE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}
