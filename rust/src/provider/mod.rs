

pub mod wiki_file_provider;
pub mod arrow_provider;
//pub mod provider_config;
pub mod arrow_transfer;

pub enum ProviderChannel<T> {
    Complete,
    Data(T)
}



