import yaml
from dataclasses import dataclass
from typing import List

@dataclass
class Epochs:
    epochs:int
    @staticmethod
    def constructor(loader: yaml.SafeLoader, node: yaml.nodes.MappingNode) -> 'Epochs':
        """Construct an employee."""
        return Epochs(**loader.construct_mapping(node))

@dataclass
class Iterations:
    iterations:int
    @staticmethod
    def constructor(loader: yaml.SafeLoader, node: yaml.nodes.MappingNode) -> 'Iterations':
        """Construct an employee."""
        return Iterations(**loader.construct_mapping(node))




class NoneHolder:
    def __init__(self):
        pass
    @staticmethod
    def constructor(loader: yaml.SafeLoader, node: yaml.nodes.MappingNode) -> 'Epochs':
        """Construct an employee."""
        return NoneHolder()

@dataclass
class Python:
    command:str
    cwd:str
    args:List[str]
    @staticmethod
    def constructor(loader: yaml.SafeLoader, node: yaml.nodes.MappingNode) -> 'Python':
        """Construct an employee."""
        return Python(**loader.construct_mapping(node))

@dataclass
class Zmq:
    address:str

    @staticmethod
    def constructor(loader: yaml.SafeLoader, node: yaml.nodes.MappingNode) -> 'Python':
        """Construct an employee."""
        return Zmq(**loader.construct_mapping(node))

loader = yaml.SafeLoader
loader.add_constructor("!epochs", Epochs.constructor)
loader.add_constructor("!iterations", Iterations.constructor)
loader.add_constructor("!none", NoneHolder.constructor)
loader.add_constructor("!python", Python.constructor)
loader.add_constructor("!wiki", NoneHolder.constructor)
loader.add_constructor("!pile", NoneHolder.constructor)
loader.add_constructor("!test",  NoneHolder.constructor)
loader.add_constructor("!zmq",  Zmq.constructor)
loader.add_constructor("!list",  NoneHolder.constructor)
loader.add_constructor("!total",  NoneHolder.constructor)
loader.add_constructor("!huggingface",  NoneHolder.constructor)
loader.add_constructor("!squad",  NoneHolder.constructor)

def load(path:str):
    with open(path, 'r') as fp:
        config = yaml.safe_load(fp)
        return config


if __name__ == '__main__':
    result = load('../rust/tests/masking.yaml')
    print(result)


