use std::path::PathBuf;

use super::{Dataset, ProviderChannel, general_file_provider::Counter, cache_writer::CacheWriter, source_filter::SourceFilter};
use async_compression::tokio::bufread::ZstdDecoder;
use tokio::{io::{AsyncBufReadExt, BufReader, Lines}, fs::File};

use tokio::sync::mpsc::Sender;
use futures::stream::TryStreamExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

type Decoder<T> = ZstdDecoder<T>;

pub async fn create_lines(file_path:&PathBuf) -> Lines<BufReader<Decoder<BufReader<File>>>> {
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let gzip_decoder = ZstdDecoder::new(reader);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let lines = buf_reader.lines();
    return lines;
}

pub async fn load_dataset(path:&PathBuf, counter:&mut Counter, tx:&Sender<ProviderChannel<String>>, filter:&SourceFilter) {
    let mut lines = create_lines(path).await;
    while let Some(line) = lines.next_line().await.unwrap() {
        let text = filter.get_text(line);
        match text {
            Some(x) => {
                let _res_ = tx.send(ProviderChannel::Data(x)).await;
                if counter.inc_data() {
                    return;
                }
            },
            None => {
                continue
            },
        }
    }
}

pub async fn load_url(dataset:&Dataset, 
    counter:&mut Counter, 
    tx:&Sender<ProviderChannel<String>>, 
    mut cache_writer:Option<CacheWriter>,
    filter:&SourceFilter) {

    let response = reqwest::get(dataset.location.to_owned()).await.unwrap();
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = Decoder::new(stream);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let mut lines = buf_reader.lines();

    loop {
        let data = lines.next_line().await;
        match data {
            Ok(Some(line)) => {
                //log::info!("Line {:?}", line);
                let text = filter.get_text(line);
                //log::info!("Here {:?}", text);
                match text {
                    Some(x) => {
                        
                        cache_writer.as_mut().map(|s| s.write_line(x.to_owned()));
                        let _res = tx.send(ProviderChannel::Data(x.to_owned())).await;
                        if counter.inc_data() {
                            return;
                        }
                    },
                    None => {
                        continue
                    },
                }
            },
            Ok(None) => {
                log::info!("Line Not Available");
                return;
            },
            Err(e) => {
                log::error!("Error in File Read {:?}", e);
                return;
            },
        }
    }
            
}



