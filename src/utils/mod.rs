pub(crate) mod entities;
mod is_punctuation_or_symbol;
pub(crate) mod percent_encode;
mod text_encoding;

pub(crate) use entities::escape_xml;
pub(crate) use entities::lookup_entity;
pub(crate) use entities::unescape_string;
pub(crate) use is_punctuation_or_symbol::is_punctuation_or_symbol;
