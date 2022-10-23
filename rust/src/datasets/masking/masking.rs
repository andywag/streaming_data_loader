use tokenizers::{Tokenizer};
use tokio::sync::mpsc::Receiver;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::provider::ProviderChannel;
use crate::transport::ZmqChannel;
use crate::utils;
use super::masked_data::MaskedData;
use super::masking_config::MaskingConfig;

pub struct BaseTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    pub mask_length:u32,
    tokenizer:Tokenizer
}

impl BaseTokenizer {
    pub fn new(config:&MaskingConfig) -> Self {
        let mask_length = config.mask_length;
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            mask_length: mask_length,
            tokenizer: tokenizer,
        }
    }

    fn create_data(&self) -> MaskedData {
        return MaskedData::new(self.batch_size, self.sequence_length, self.mask_length)
    }
     
    pub fn mask_sequence(&self, index:usize, positions:&Vec<u32>, batch:&mut MaskedData, l:usize, mask:u32) {
        for x in 0..self.mask_length as usize {
            if positions[x] <= l as u32 {
                batch.masked_lm_positions[index][x] = positions[x];
                let masked_value = batch.input_ids[index][positions[x] as usize];
                batch.input_ids[index][positions[x] as usize] = mask;
                batch.masked_lm_positions[index][x] = positions[x];
                batch.masked_lm_labels[index][x] = masked_value;
            }
            
        }
    }



    pub async fn create_batch(&self, mut rx:Receiver<ProviderChannel<String>>, 
        tx_transport:tokio::sync::mpsc::Sender<ZmqChannel<MaskedData>>) {

        let mut batch = self.create_data();
        let mut index = 0;
        let attention = vec![0;self.sequence_length as usize];
        let mut positions:Vec<u32> = (0..self.sequence_length).collect();
        
        let mask = self.tokenizer.token_to_id("[MASK]").unwrap();

        loop {

            let data_option = rx.recv().await;
            // Channel is shutdown if the receive data is None           
            if data_option.is_none() {
               break
            }
            
            // Match the input to check if the stream is complete and send the complete command forward
            let data:String;
            match data_option.unwrap() {
                ProviderChannel::Complete => {
                    let _ = tx_transport.send(ZmqChannel::Complete).await;
                    println!("Finished Tokenizer");
                    return;
                },
                ProviderChannel::Data(x) => {
                    data = x;
                },
            }
            // Encode the Data
            let result = self.tokenizer.encode(data, true).unwrap();
            let ids = result.get_ids();
            
            let mut current_index = 0;
            while current_index < ids.len() {
                positions.shuffle(&mut thread_rng());
                let mut length:usize = self.sequence_length as usize;
                if (current_index + self.sequence_length as usize) < ids.len() {
                    
                    batch.input_ids[index].clone_from_slice(&ids[current_index..current_index+self.sequence_length as usize]);
                }
                else {
                    length = ids.len() - current_index;
                    batch.input_ids[index][0..length].clone_from_slice(&ids[current_index..current_index+length as usize]);
                    batch.attention_mask[index][(self.sequence_length as usize-length)..self.sequence_length as usize].copy_from_slice(&attention[0..length]);
                }

                self.mask_sequence(index, &positions, &mut batch, length, mask);

                current_index += self.sequence_length as usize;
                index += 1;
                if index == self.batch_size as usize {

                    let _ = tx_transport.send(ZmqChannel::Data(batch)).await;
                    batch = self.create_data();
                    index = 0;
                    //println!("Created Batch");
                }
            }
            
        
            //println!("Result {:?}", ids.len());
            //thread::sleep(Duration::from_millis(1000));
        }
        
    }
    

}

pub async fn create_tokenizer(config:&MaskingConfig, rx:tokio::sync::mpsc::Receiver<ProviderChannel<String>>, 
    tx_transport:tokio::sync::mpsc::Sender<ZmqChannel<MaskedData>>) {
    let base_tokenizer = BaseTokenizer::new(config);
    
    let result = base_tokenizer.create_batch(rx, tx_transport);
    result.await;
}
