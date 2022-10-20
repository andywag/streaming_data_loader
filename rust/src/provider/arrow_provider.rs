

pub async fn load_data(location:&String, iterations:u64, tx:Sender<ProviderChannel<String>>) {
    //let location = "/home/andy/.cache/huggingface/datasets/squad/plain_text/1.0.0/d6ec3ceb99ca480ce37cdd35555d6cb2511d223b9150cce08a837ef62ffea453/squad-validation.arrow";
    let f = File::open(location);
    let ar = StreamReader::try_new(f.unwrap(), None);
    let mut rr = ar.unwrap();
    println!("Base {:?} ", rr.schema());

    let data = rr.next().unwrap().unwrap();
    println!("Data {:?}", data.slice(0, 1));

}