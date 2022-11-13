
from datasets import load_dataset
import argparse
import pickle
from dataclasses import dataclass
from typing import List, Dict

parser = argparse.ArgumentParser(description='Download Dataset')
parser.add_argument('--dataset', type=str, default="squad")
parser.add_argument('--ar', type=str, default=None)
parser.add_argument('--store', type=str, default="cache.pkl")

@dataclass
class Descriptor:
    name: str
    paths: List[str]
    num_rows: int

@dataclass
class Transfer:
    children: List[Descriptor]


def main():
    print("BASE")
    args = parser.parse_args()

    if args.ar is None:
        d = load_dataset(args.dataset)
    else:
        d = load_dataset(args.dataset, args.ar)

    descriptors = []
    for k, v in d.cache_files.items():
        paths = []
        for x in v:
            paths.append(x['filename'])
        descriptor = Descriptor(k, paths, d.num_rows[k])
        descriptors.append(descriptor)
    transfer = Transfer(descriptors)

    store = {"store": transfer}
    with open(args.store, 'wb') as fp:
        pickle.dump(transfer, fp)


if __name__ == '__main__':
    main()