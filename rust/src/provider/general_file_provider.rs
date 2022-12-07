use std::{path::{PathBuf}, str::FromStr};

use super::{ProviderChannel, gzip_file_provider, zstd_file_provider, provider_util::{get_download_type, DownloadType, is_network, get_local_path}, cache_writer, source_filter::SourceFilter, provider_config::{ProviderLength, Dataset}};
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
            ProviderLength::Iterations { iterations } => {iterations_ = Some(iterations)},
            ProviderLength::Epochs { epochs } => {epochs_ = Some(epochs)},
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




pub async fn load_data_sets(datasets:Vec<Dataset>, 
    length:ProviderLength, 
    tx:Sender<ProviderChannel<String>>, 
    cache:Option<String>,
    filter:&SourceFilter) {


    let mut counter = Counter::new(length);

    log::info!("Sending Dataset Info");
    let _result = tx.send(ProviderChannel::Info(crate::tasks::DatasetInfo { name: "wiki".to_string(), length: 1000000000 })).await;

    loop {

        for dataset in &datasets {
            // Download Type
            let typ = get_download_type(&dataset.location);
            
            let location = if !is_network(&dataset.location) { // Local Path
                 (typ, Some(PathBuf::from_str(&dataset.location.as_str()).unwrap()), None)
            }
            else {

                match cache.to_owned() {
                    Some(path) => {

                        let cache_path = PathBuf::from_str(&path).unwrap();
                        if !cache_path.exists() {
                            log::error!("Cache Location : {:?} Doesn't Exist", cache_path);
                        }
                        let base_file_path = cache_path.join(get_local_path(&dataset.location));
                        let zstd_location = cache_writer::existing_cache_file(&base_file_path);

                        if zstd_location.is_some() {
                            log::info!("Found Local Path {:?} ", zstd_location);
                            (DownloadType::Zstd, zstd_location, None) // Local Cache File -- No Writer
                        }
                        else {
                            log::info!("File Path {:?} {:?}", base_file_path, dataset.location);
                            let writer = cache_writer::CacheWriter::new(base_file_path);
                            (typ, None, Some(writer))        
                        }
                    },
                    None => (typ, None, None),
                }
            };


            match location {
                (DownloadType::Zstd, None, z) => zstd_file_provider::load_url(dataset, &mut counter, &tx, z, filter).await,
                (DownloadType::Zstd, Some(x), _) => zstd_file_provider::load_dataset(&x, &mut counter, &tx, filter).await,
                (DownloadType::Gzip, None, z) => gzip_file_provider::load_url(dataset, &mut counter, &tx, z, filter).await,
                (DownloadType::Gzip, Some(x), _) => gzip_file_provider::load_dataset(&x, &mut counter, &tx, filter).await,
                (DownloadType::Error, _, _) => log::error!("Dataset Type Not Defined"),
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
