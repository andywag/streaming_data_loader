

use crate::tasks::DatasetInfo;



pub mod provider_config;
pub mod provider_util;

pub mod arrow_provider;
pub mod arrow_transfer;

pub mod pile_datasets;
pub mod general_file_provider;
pub mod gzip_file_provider;
pub mod zstd_file_provider;

pub mod cache_writer;
pub mod source_filter;

pub enum ProviderChannel<T> {
    Complete,
    Info(DatasetInfo),
    Data(T)
}


