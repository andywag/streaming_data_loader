
use async_compression::tokio::bufread::GzipDecoder;
use futures::stream::TryStreamExt;
use tokio::{io::{AsyncBufReadExt, BufReader, Lines}, fs::File};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use tokio::sync::mpsc::Sender;

use super::{ProviderChannel, ProviderLength};
 



pub async fn create_url_lines(file_path:&String) -> Lines<BufReader<GzipDecoder<BufReader<File>>>> {
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let gzip_decoder = GzipDecoder::new(reader);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let lines = buf_reader.lines();
    return lines;
}

// TODO : Add Support for Limiting Iterations
pub async fn load_url(url:&String, length:ProviderLength, tx:Sender<ProviderChannel<String>>) {

    let mut it:Option<usize> = None;
    let mut ep:Option<usize> = None;
    match length {
        crate::provider::ProviderLength::Iterations { iterations } => {it = Some(iterations)},
        crate::provider::ProviderLength::Epochs { epochs } => {ep = Some(epochs)},
    }
    let _result = tx.send(ProviderChannel::Info(crate::tasks::DatasetInfo { name: "wiki".to_string(), length: 1000000000 })).await;

    // Print decompressed txt content

    
    let mut data_count = 0;
    let mut epoch_count = 0;
    loop {
        let response = reqwest::get(url).await.unwrap();

        let stream = response
            .bytes_stream()
            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
            .into_async_read()
            .compat();
        let gzip_decoder = GzipDecoder::new(stream);

        let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);

        let mut lines = buf_reader.lines();
        let mut index = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            if index % 2 == 1 {
                let text = super::provider_util::create_text(line, "\"text\":\"");
                let res = tx.send(ProviderChannel::Data(text));
                let _result = res.await;

                if it.is_some() && data_count == it.unwrap() {
                    let _result = tx.send(ProviderChannel::Complete).await;
                    log::info!("Done Loading Data");
                    return;
                }
                data_count += 1;
            }
            index += 1;   
        }
        log::info!("Finished Epoch {}", epoch_count);
        epoch_count += 1;
        if ep.is_some() && epoch_count == ep.unwrap() {
            let _result = tx.send(ProviderChannel::Complete).await;
            return;
        }
        
        
    }

}

pub async fn create_lines(file_path:&String) -> Lines<BufReader<GzipDecoder<BufReader<File>>>> {
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let gzip_decoder = GzipDecoder::new(reader);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let lines = buf_reader.lines();
    return lines;
}

pub async fn load_data(file_path:&String, length:ProviderLength, tx:Sender<ProviderChannel<String>>) {

    let mut it:Option<usize> = None;
    let mut ep:Option<usize> = None;
    match length {
        crate::provider::ProviderLength::Iterations { iterations } => {it = Some(iterations)},
        crate::provider::ProviderLength::Epochs { epochs } => {ep = Some(epochs)},
    }
    let _result = tx.send(ProviderChannel::Info(crate::tasks::DatasetInfo { name: "wiki".to_string(), length: 1000000000 })).await;

                    
                    

    let mut data_count = 0;
    let mut epoch_count = 0;
    loop {
        let mut lines = create_lines(file_path).await;

        let mut index = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            if index % 2 == 1 {
                let text = super::provider_util::create_text(line, "\"text\":\"");
                let res = tx.send(ProviderChannel::Data(text));
                let _ = res.await;
                if it.is_some() && data_count == it.unwrap() {
                    let _result = tx.send(ProviderChannel::Complete).await;
                    log::info!("Finished Loading Data");
                    return;
                }
                data_count += 1;
            }
            index += 1;   
        }
        log::info!("Finished Epoch {}", epoch_count);
        epoch_count += 1;
        if ep.is_some() && epoch_count == ep.unwrap() {
            let _result = tx.send(ProviderChannel::Complete).await;
            return;
        }
    }

}
