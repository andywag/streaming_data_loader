import numpy as np

y = np.asarray([1,1,2,2,3,3])
z = np.broadcast_to(y,shape=(len(y),len(y)))
p = (z == z.transpose())
pass
