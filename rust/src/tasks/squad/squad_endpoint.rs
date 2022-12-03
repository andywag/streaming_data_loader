use tokenizers::Tokenizer;

use crate::{transport::test_endpoint::EndPoint, config::TrainingConfig};

use super::{squad_data::{SquadData}};

fn _check_batch(data:SquadData, tokenizer:&Tokenizer) -> bool{

    for x in 0..data.input_ids.len() {
        let base_answer = tokenizer.decode(data.input_ids[x][data.start_positions[x] as usize..data.end_positions[x] as usize].to_owned(), false).unwrap();   
        log::info!("Answer {:?} ", data.input_ids);
        let other_answer = data.answers[x].to_owned().unwrap().to_string().to_lowercase();
        

        let base_chars:Vec<char> = base_answer.chars().collect();
        let other_chars:Vec<char> = other_answer.chars().collect();

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

    }
    return true;
}

pub struct SquadEnpoint {
    //_tokenizer:Tokenizer
}

impl SquadEnpoint {
    pub fn new(_config:TrainingConfig) -> Self {
        //let tokenizer = utils::get_tokenizer(config.tokenizer_name.to_owned());
        Self {
        //    _tokenizer:tokenizer
        }
    }
}

impl EndPoint<SquadData> for SquadEnpoint {
    fn receive(&mut self, _data:SquadData) -> bool {
        //return check_batch(data, &self.tokenizer);
        //log::info!("Data {:?}", data);
        // TODO : Squad is broken
        return true;
    }
}

