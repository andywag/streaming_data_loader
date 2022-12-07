import torch
import zmq

import threading
import queue
import pickle
from typing import List

class ExternalDataset(torch.utils.data.IterableDataset):

    def __init__(self, address, maxsize=8):

        self.ctx = zmq.Context()
        self.socket = self.ctx.socket(zmq.REQ)
        self.socket.connect(address)

        self.socket.send_string("Config")
        data = self.socket.recv()
        self.context = pickle.loads(data)
        #print("Received Configuration", self.context)
        self.batch_size = self.context['batch']['batch_size']


        # Get the Dataset info from the server
        self.socket.send_string("Info")
        data = self.socket.recv()
        self.info = pickle.loads(data)

        # Start the loading process
        load_thread = threading.Thread(target=self.__load_data__)
        self.data_queue = queue.Queue(maxsize = maxsize)
        load_thread.start()

        self.internal_iter = self.__internal_item__()

    def flatten(self, data):
        keys = list(data.keys())
        l = len(data[keys[0]])

        for x in range(l):
            result = dict()
            for key in keys:
                result[key] = data[key][x]
                self.data_queue.put(result)
    def __load_data__(self):
        while True:
            self.socket.send_string("Data")
            data = self.socket.recv()
            if len(data) == 8:
                print("Done with Download")
                break
            result = pickle.loads(data)
            self.data_queue.put(result)
            #self.flatten(result)

    def __iter__(self):
        return self.__internal_item__()

    def __len__(self):
        return self.info['length']

    def __next__(self):
        return self.__internal_item__()

        #return self.__internal_item__() #//next(self.internal_iter)

    def __internal_item__(self):
        while True:
            data = self.data_queue.get()
            keys = list(data.keys())
            for x in range(len(data[keys[0]])):
                result = dict()
                for key in keys:
                    result[key] = data[key][x]
                yield result

    def __getitem__(self, idx):
        return next(self.internal_iter)



def main():
    dataset = ExternalDataset("ipc:///tmp/multi-label")


if __name__ == '__main__':
    main()