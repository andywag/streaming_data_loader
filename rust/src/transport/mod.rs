
pub mod zmq_transmit;
pub mod zmq_receive;

pub enum ZmqChannel<T> {
    Complete,
    Data(T)
}

