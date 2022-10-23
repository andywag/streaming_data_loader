use std::cmp::min;

use tokenizers::{Tokenizer};
use tokio::sync::mpsc::Receiver;
use crate::provider::ProviderChannel;
use crate::transport::ZmqChannel;
use crate::utils;

use super::SquadConfig;
use super::squad_data::{SquadData, SquadGeneral};

fn find_offset(index:usize, types:&[u32], offsets:&[(usize,usize)]) -> usize {
    let mut sp = 0;
    let mut ep = offsets.len();
    while ep > sp {
        let cp = sp + (ep - sp)/2;
        if offsets[cp] == (0,0) {
            if cp == 0 {
                return 0;
            }
            ep = cp;
        }
        else if types[cp] == 0 {
            sp = cp + 1;
        }
        else if offsets[cp].0 <= index && index <= offsets[cp].1 {
            //println!("Found {} {} {:?}", cp, index, offsets[cp]);
            return cp;
        }
        else if offsets[cp].1 < index {
            sp = cp+1;
        } 
        else {
            ep = cp-1;
        }
    }
    //println!("Found {} {} {:?}", sp, ep, offsets[sp]);
    return sp;
}

pub struct SquadTokenizer {
    pub batch_size:u32,
    pub sequence_length:u32,
    tokenizer:Tokenizer
}

impl SquadTokenizer {
    pub fn new(config:&SquadConfig) -> Self {
        let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
            batch_size: config.batch_size,
            sequence_length: config.sequence_length,
            tokenizer: tokenizer,
        }
    }

    fn create_data(&self) -> SquadData {
        return SquadData::new(self.batch_size, self.sequence_length)
    }
     


    pub async fn create_batch(&self, mut rx:Receiver<ProviderChannel<SquadGeneral>>, 
        tx_transport:tokio::sync::mpsc::Sender<ZmqChannel<SquadData>>) {

        let mut batch = self.create_data();
        let mut index = 0;
        
        //let mask = self.tokenizer.token_to_id("[MASK]").unwrap();

        loop {

            let data_option = rx.recv().await;
            // Channel is shutdown if the receive data is None           
            if data_option.is_none() {
               break
            }
            // Match the input to check if the stream is complete and send the complete command forward
            let data:SquadGeneral;
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
            let result = self.tokenizer.encode((data.question, data.context), true).unwrap();
            //println!("Data Length {:?} {:?}", result.get_ids().len(), result.get_offsets());

            let length = min(result.len(), self.sequence_length as usize);
            batch.input_ids[index][0..length].clone_from_slice(&result.get_ids()[0..length]);
            batch.token_type_ids[index][0..length].clone_from_slice(&result.get_type_ids()[0..length]);
            batch.attention_mask[index][0..length].clone_from_slice(&result.get_attention_mask()[0..length]);
            batch.answers[index] = data.answer.clone();
            
            //println!("Offsets {:?}", result.get_offsets());

            let mut start = find_offset(data.sp as usize, result.get_type_ids(), result.get_offsets());
            let mut end = find_offset(data.ep as usize, result.get_type_ids(), result.get_offsets());

            let ans_token = self.tokenizer.encode(data.answer.unwrap(), false);
            let ans_ids = ans_token.unwrap();

            if start > self.sequence_length as usize{
                continue;
            }

            if batch.input_ids[index].len() > start && ans_ids.get_ids()[0] != batch.input_ids[index][start as usize] {
                // TODO : Hacked way of searching for the proper start/end points
                // Doesn't catch all cases which run through the continue loop
                // Misses some mismatch cases because only works on first char
                for _ in 0..10 {
                    start += 1;
                    end += 1;
                    if batch.input_ids[index].len() > start {
                        if ans_ids.get_ids()[0] == batch.input_ids[index][start as usize] {
                            break;
                        }
                    }
                }
                if ans_ids.get_ids()[0] != batch.input_ids[index][start as usize] {
                    //println!("Offsets {start} {} {:?}", data.sp, result.get_offsets());
                    //println!("Mismatched {:?}:{:?} {:?}",batch.input_ids[index][start], ans_ids.get_ids(), batch.input_ids[index]);
                    continue;
                }
                if start >= self.sequence_length as usize || end >= self.sequence_length as usize {
                    continue;
                }
                
            }

            //println!("Here {:?} {:?} {:?} {:?} ", data.sp, data.ep, start, end);
            //let start1 = result.char_to_token(data.sp as usize, 1);
            //let end1 = result.char_to_token(data.ep as usize, 1);

            //println!("AASA {} {:?} {:?}", start, start1, end1); 

            batch.start_positions[index] = start as u32;
            batch.end_positions[index] = end as u32 ;

            index += 1;
            if index == self.batch_size as usize {
            //    println!("Sending Data");
                let _ = tx_transport.send(ZmqChannel::Data(batch)).await;
                batch = self.create_data();
                index = 0;
               
            }


        }
        
    }
    

}

pub async fn create_tokenizer(config:&SquadConfig, rx:tokio::sync::mpsc::Receiver<ProviderChannel<SquadGeneral>>, 
    tx_transport:tokio::sync::mpsc::Sender<ZmqChannel<SquadData>>) {
    let base_tokenizer = SquadTokenizer::new(config);
    
    let result = base_tokenizer.create_batch(rx, tx_transport);
    result.await;
}
