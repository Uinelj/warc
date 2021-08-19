//! A WARC (Web ARChive) library

mod error;
pub use error::Error;

mod warc_reader;
pub use warc_reader::WarcReader;
pub use warc_reader::RawRecordIter;
pub use warc_reader::RecordIter;
mod warc_writer;
pub use warc_writer::WarcWriter;

pub mod header;

pub mod parser;

mod record;
pub use record::{BufferedBody, EmptyBody, RawRecordHeader, Record, RecordBuilder, StreamingBody};

mod record_type;
pub use record_type::RecordType;

mod truncated_type;
pub use truncated_type::TruncatedType;
