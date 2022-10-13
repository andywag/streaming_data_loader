pub mod wiki_file_provider;

pub enum ProviderChannel<T> {
    Complete,
    Data(T)
}

pub async fn provider(location:String, network:bool, iterations:u64, tx:tokio::sync::mpsc::Sender<ProviderChannel<String>>)  {
    if network { // URL to web version of file
        wiki_file_provider::load_url(&location,  tx).await;
    }
    else {// Downloaded File
        wiki_file_provider::load_data(&location, iterations, tx).await;
    }
}
