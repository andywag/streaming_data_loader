import torch
import zmq

import threading
import queue
import pickle


class ExternalDataset(torch.utils.data.IterableDataset):

    def __init__(self, address, maxsize=8):
        self.ctx = zmq.Context()
        self.socket = self.ctx.socket(zmq.REQ)
        self.socket.connect(address)

        # Get the Dataset info from the server
        self.socket.send_string("Info")
        data = self.socket.recv()
        self.info = pickle.loads(data)

        # Start the loading process
        load_thread = threading.Thread(target=self.__load_data__)
        self.data_queue = queue.Queue(maxsize = maxsize)
        load_thread.start()

        self.internal_iter = self.__internal_item__()

    def __load_data__(self):
        while True:
            self.socket.send_string("Data")
            data = self.socket.recv()
            if len(data) == 8:
                print("Done with Download")
                break
            result = pickle.loads(data)
            self.data_queue.put(result)

    def __iter__(self):
        return self

    def __len__(self):
        return self.info['length']

    def __next__(self):
        return next(self.internal_iter)

    def __internal_item__(self):
        while True:
            data = self.data_queue.get()

            for x in range(1024):
                result = dict()
                for k, v in data.items():
                    result[k] = v[x]
                yield result

    def __getitem__(self, idx):
        return next(self.internal_iter)



def main():
    dataset = ExternalDataset("ipc:///tmp/multi-label")


if __name__ == '__main__':
    main()