use std::{path::PathBuf, str::FromStr};

use super::{ProviderChannel, ProviderLength, {Dataset}, gzip_file_provider, zstd_file_provider, provider_util::{get_download_type, DownloadType, get_cached_file, is_network}};
use tokio::sync::mpsc::Sender;




pub struct Counter {
    pub iterations:Option<usize>,
    pub epochs:Option<usize>,
    pub count:usize,
    pub count_epoch:usize
}

impl Counter {
    pub fn new(length:ProviderLength) -> Self {
        let mut iterations_:Option<usize> = None;
        let mut epochs_:Option<usize> = None;
        match length {
            crate::provider::ProviderLength::Iterations { iterations } => {iterations_ = Some(iterations)},
            crate::provider::ProviderLength::Epochs { epochs } => {epochs_ = Some(epochs)},
        }
        Self {
            iterations:iterations_,
            epochs:epochs_,
            count:0,
            count_epoch:0

        }
    }

    pub fn done(&mut self) -> bool {
        if self.iterations.is_some() && self.count == self.iterations.unwrap() {
            return true;
        }
        return false;
    }

    pub fn inc_data(&mut self) -> bool {
        self.count += 1;
        if self.iterations.is_some() && self.count == self.iterations.unwrap() {
            return true;
        }
        return false;
    }

    pub fn inc_epoch(&mut self) -> bool {
        self.count_epoch += 1;
        if self.iterations.is_some() && self.count == self.iterations.unwrap() {
            return true;
        }
        if self.epochs.is_some() && self.count_epoch == self.epochs.unwrap() {
            return true;
        }  
        return false;
    }


}


pub async fn save_data_sets(cache_path:String, datasets:Vec<Dataset>) {

    for dataset in datasets {
        get_cached_file(cache_path.clone(), &dataset.location, false);
    }
}

pub async fn load_data_sets(datasets:Vec<Dataset>, length:ProviderLength, tx:Sender<ProviderChannel<String>>) {


    let mut counter = Counter::new(length);

    let _result = tx.send(ProviderChannel::Info(crate::tasks::DatasetInfo { name: "wiki".to_string(), length: 1000000000 })).await;

    loop {

        for dataset in &datasets {
            let typ = get_download_type(&dataset.location);
            
            let location = if !is_network(&dataset.location) {
                 Some(PathBuf::from_str(&dataset.location.as_str()).unwrap())
            }
            else {
                get_cached_file("../../blob2/data".to_string(), &dataset.location, true)
            };

            match (typ,location) {
                (DownloadType::Zstd, None) => zstd_file_provider::load_url(dataset, &mut counter, &tx).await,
                (DownloadType::Zstd, Some(x)) => zstd_file_provider::load_dataset(&x, &mut counter, &tx).await,
                (DownloadType::Gzip, None) => gzip_file_provider::load_url(dataset, &mut counter, &tx).await,
                (DownloadType::Gzip, Some(x)) => gzip_file_provider::load_dataset(&x, &mut counter, &tx).await,
                (DownloadType::Error, _) => log::error!("Dataset Type Not Defined"),
            }

            if counter.done() {
                log::info!("Finished Data Provider");
                let _ = tx.send(ProviderChannel::Complete).await;
                return;
            }
        }
        let finished = counter.inc_epoch();
        if finished {
            log::info!("Finished Data Provider");
            let _result = tx.send(ProviderChannel::Complete).await;
            return;
        }
    }
    
    

}
