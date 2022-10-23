

pub mod wiki_file_provider;
pub mod arrow_provider;

pub enum ProviderChannel<T> {
    Complete,
    Data(T)
}



