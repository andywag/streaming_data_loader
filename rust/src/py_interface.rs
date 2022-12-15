
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::sync::mpsc::{SyncSender, Receiver};
use std::{thread, sync::mpsc};
use std::time::Duration;

use crate::config::{TrainingConfig, TaskType};
use crate::datasets::dataset::DataSet;
use crate::provider::ProviderChannel;
use crate::py_conversions;
use crate::{logger, tasks::cases::BasicCases};



#[tokio::main]
async fn run_case(tx:SyncSender<ProviderChannel<DataSet>>, config:TrainingConfig) {
    log::info!("Running Case");
    //task::spawn(async move {
    //    let result = zmq_receive::python_node_transport(command);
    //    result.await
    //})
    let _result = crate::tasks::run(config, TaskType::Mlm, Some("../../../storage".to_string()), Some(tx)).await;
    thread::sleep(Duration::from_millis(20000));
    log::info!("Final Result ");

}

#[pyclass]
pub struct TrainingRun {
    config:TrainingConfig,
    rx:Option<Receiver<ProviderChannel<DataSet>>>
}

#[pymethods]
impl TrainingRun {
    #[new]
    fn py_new(task:String) -> PyResult<Self> {
        logger::create_logger();
        let config = match task.as_str() {
            "mlm" => Some(BasicCases::Bert.get_config(false)),
            "clm" => Some(BasicCases::Gpt.get_config(false)),
            "span" => Some(BasicCases::T5.get_config(false)),
            "python" => Some(BasicCases::Python.get_config(false)),
            "emot" => Some(BasicCases::Multi.get_config(false)),
            "imdb" => Some(BasicCases::Single.get_config(false)),
            _ => {log::error!("Configuration Not Found"); None}
        };
        log::info!("Creating Configuration");        
        
        match config {
            Some(x) => Ok(Self {config:x, rx:None}),
            None => Err(PyValueError::new_err("argument is wrong"))
        }
        /*thread::spawn(|| {
            run_case(tx, config_copy);
        });*/
        
        //Ok(Self {config, _rx:None})
    }

    fn start(&mut self) {
        let (tx, rx) = mpsc::sync_channel::<ProviderChannel<DataSet>>(8);
        self.rx = Some(rx);
        log::info!("Creating Training Run");
        let config = self.config.clone();
        thread::spawn(|| {
            run_case(tx, config);
        });
        //py.allow_threads(|| run_case(tx,self._config.to_owned()));
    }

    fn get_data(&mut self, py:Python) -> PyObject {
    
        let data = self.rx.as_ref().unwrap().recv();
        let result = match data {
            Ok(ProviderChannel::Complete) => None,
            Ok(ProviderChannel::Data(x)) => {
                Some(py_conversions::convert_data_set(x, py, self.config.dataset_config.clone()))
            }
            _ => None
        };
        result.into_py(py)
    }

    fn stop(&mut self) {
        log::info!("Killing Rust Process");
        std::process::exit(0);
    }
}

