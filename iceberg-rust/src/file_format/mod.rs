/*!
 * Helper functions for different file formats.
*/

use ::parquet::format::FileMetaData;
pub mod parquet;

/// Metadata for a datafile
pub enum DatafileMetadata {
    /// Metadata for a parquet datafile
    Parquet(FileMetaData),
}
