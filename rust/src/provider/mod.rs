use crate::datasets::DatasetInfo;



pub mod wiki_file_provider;
pub mod arrow_provider;
pub mod arrow_transfer;

pub enum ProviderChannel<T> {
    Complete,
    Info(DatasetInfo),
    Data(T)
}

pub struct ProviderConfigIterations {
    pub iterations:u64
}
pub struct ProviderConfigEpochs {
    pub epochs:u64,
}

pub enum ProviderConfig {
    Iterations(ProviderConfigIterations),
    Epochs(ProviderConfigEpochs)
}
