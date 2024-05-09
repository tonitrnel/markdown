pub(crate) mod entities;
pub(crate) mod percent_encode;
mod puncttable;
mod text_encoding;

pub(crate) use entities::escape_xml;
pub(crate) use entities::lookup_entity;
pub(crate) use entities::unescape_string;
pub(crate) use puncttable::is_punctuation;
