import torch
import zmq

import threading
import queue
import pickle
from typing import List
import rust_loader

class InternalDataset(torch.utils.data.IterableDataset):

    def __init__(self, config, maxsize=8):

        self.loader = rust_loader.TrainingRun(config)
        self.loader.start()

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
            data = self.loader.get_data()
            self.data_queue.put(data)

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
                try :
                    for key in keys:
                        result[key] = data[key][x]
                    yield result
                except:
                    pass

    def __getitem__(self, idx):
        return next(self.internal_iter)

def main():
    dataset = InternalDataset("mlm")
    for i, data in enumerate(dataset):
        print("Rx Data")
        print("Exitting")
        dataset.loader.stop()
        break
        print("Done")

if __name__ == '__main__':
    main()


