
import torch

device = 'cuda';

data = torch.tensor([[4],[5],[6]])
d = data.expand(3,4)
pass
