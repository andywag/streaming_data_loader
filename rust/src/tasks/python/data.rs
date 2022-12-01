use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonData {
    pub input_ids:Vec<Vec<u32>>,
    pub attention_mask:Vec<Vec<u32>>,
    pub labels:Vec<Vec<i32>>,
    pub original:Option<Vec<Vec<u32>>>,
    pub index:usize,

    batch_size:usize,
    sequence_length:usize,
    attention_base:Vec<u32>,
    label_base:Vec<i32>,
}

impl PythonData {
    pub fn new(batch_size:usize, sequence_length:usize) -> Self{
        Self {
            input_ids: vec![vec![0;sequence_length as usize];batch_size as usize],
            attention_mask: vec![vec![1;sequence_length as usize];batch_size as usize],
            labels:vec![vec![-100;sequence_length as usize]; batch_size as usize],
            original:None,
            index:0,
            batch_size:batch_size,
            sequence_length:sequence_length,
            attention_base:vec![0;sequence_length as usize],
            label_base:vec![-100;sequence_length as usize]
        }
    }

    pub fn new_data(&mut self) -> Self {
        PythonData::new(self.batch_size, self.sequence_length)
    }

    pub fn put_data(&mut self, ids:&[u32]) -> bool{
        self.input_ids[self.index][0..ids.len() as usize].clone_from_slice(ids);
        let labels:Vec<i32> = self.input_ids[self.index].clone().into_iter().map(|e| e as i32).collect();
        self.labels[self.index][0..labels.len() as usize].clone_from_slice(&labels);
        if ids.len() < self.sequence_length {
            let s = self.sequence_length as usize;
            let l = ids.len() as usize;
            self.labels[self.index][(s-l)..s].copy_from_slice(&self.label_base[(s-l)..s]);
            self.attention_mask[self.index][(s-l)..s].copy_from_slice(&self.attention_base[(s-l)..s]);
        }
        self.index += 1;
        self.done()
        
    }

    pub fn done(&self) -> bool{
        self.index == self.input_ids.len()
    }

}