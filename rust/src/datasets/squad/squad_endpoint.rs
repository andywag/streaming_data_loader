use tokenizers::Tokenizer;

use crate::{transport::ZmqChannel, utils};

use super::{SquadConfig, squad_data::SquadData};

fn check_batch(data:SquadData, tokenizer:&Tokenizer) -> bool{

    for x in 0..data.input_ids.len() {
        //println!("AAA {} {}", data.start_positions[x], data.end_positions[x]);
        let base_answer = tokenizer.decode(data.input_ids[x][data.start_positions[x] as usize..data.end_positions[x] as usize].to_owned(), false).unwrap();   
        let other_answer = data.answers[x].to_owned().unwrap().to_string().to_lowercase();
        
        //println!("AA {} {:?}", base_answer, data.answers[x].to_owned().unwrap().to_string().to_lowercase());

        let base_chars:Vec<char> = base_answer.chars().collect();
        let other_chars:Vec<char> = other_answer.chars().collect();
        //println!("Base {} {}", base_chars.len(), other_chars.len());

        let mut b = 0;
        let mut o = 0;

        while b < base_chars.len() && o < other_chars.len() {
            let bchar = base_chars[b];
            let ochar = other_chars[o];
            if bchar == ' ' {
                b += 1;
            }
            else if ochar == ' ' {
                o += 1;
            }
            else if bchar as u32 > 128 || ochar as u32 > 128 {
                o += 1;
                b += 1;
            }
            else if bchar != ochar {
                println!("Mismatch {}:{}", base_answer, other_answer);
                return false;
            }
            else {
                o += 1;
                b += 1;
            }
        }

        //assert!(&base_answer == &data.answers[x].to_owned().unwrap())
    }
    return true;
}

pub async fn receiver(config_in:&SquadConfig, 
    mut rx:tokio::sync::mpsc::Receiver<ZmqChannel<SquadData>>
) -> bool {
    let config = config_in.clone();
    let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());

    let data_full = rx.recv().await.unwrap();

    let _data:SquadData;
    match data_full {
        ZmqChannel::Complete => {
            println!("First Batch Required");
            _data = SquadData::new(1, 1);
        },
        ZmqChannel::Data(x) => {
            _data = x;
        },
    }
    
    // Wait for the rest of the inputs to flush out to exit
    loop {
        let result = rx.recv().await; //.unwrap();
        match result {
            Some(ZmqChannel::Complete) => {
                println!("Done Receiver");
                return true;
            },
            Some(ZmqChannel::Data(data)) => {
                return check_batch(data, &tokenizer);
                //println!("RX");    
            },
            None => {
                println!("RX ERROR");
                return true;
            }
        }
    }

}
