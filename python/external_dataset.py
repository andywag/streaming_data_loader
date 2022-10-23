import torch
import zmq

import threading
import queue
import pickle

class ExternalDataset(torch.utils.data.Dataset):

    def __init__(self, address, maxsize = 4):
        self.ctx = zmq.Context()
        self.socket = self.ctx.socket(zmq.REQ)
        self.socket.connect(self.address)
        load_thread = threading.Thread(target=self.__load_data__)
        data_queue = queue.Queue(maxsize = maxsize)

    def __load_data__(self):
        socket.send_string("Hello")
        data = socket.recv()
        result = pickle.loads(data)
        data_queue.push(result)

    def __len__(self):
        return 1000000

    def __getitem__(self, idx):
        

