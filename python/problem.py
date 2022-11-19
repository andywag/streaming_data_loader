
from typing import List, Set


# The basic idea of this approach is to exploit the calculated distance between items which have a similar amount of keys to
# prune the search to improve performance. The complexity is still worst case O(N^2*M) but I'm guessing the pruning will
# be substantial. I'm curious but have other side projects.

# N number of items, Maximum Number of Keys
# Sort the data based on number of keys (smallest to largest)
# Create Variable to Store Distance Between Elements [N^2] Actually Block Diagonal
#
# 1. Group by Length of Keys (Not Required but Easier for Discussion)
# 2. Iterate over all length of Keys (min to max)
# 3. Calculate all distances between x and x-1
# 4. Backtrack to x-k using the following conditions
#    a. Distance between I(x,n) and I(y,n-1) are small and I(x,n) and I(z, n-2) is small : Search Likely and Prune Impossible
#    b. Distance between I(x,n) and I(y,n-1) is large and I(x,n) and I(z, n-2) is small : Search Likely and Prune Impossible
#    c. Distance between I(x,n), I(y,n) and I(z,n+1) can be used to derive distance between I(x,n) and I(y,n) another search
# 5. Fill non-computed gaps for exact solution or Ignore for Approximate
# 6.


def check_item(xx, yy):

    for x in xx:
        if not x in yy:
            return False
    return True

def intersect_basic(data:List[Set[int]]):
    removed = set()
    data.sort(key=lambda x: -len(x))
    for i in range(0, len(data)):
        for j in range(i+1, len(data)):
            if len(data[i]) != len(data[j]) and j not in removed:
                if check_item(data[j], data[i]):
                    removed.add(j)

    results = []
    for i in range(0, len(data)):
        if i not in removed:
            results.append(data[i])
    return results

def intersect_store(data:List[Set[int]]):
    store = [[0 for x in range(len(data))] for y in range(len(data))]
    pass

def intersect_lists(data:List[Set[int]], levels):
    def create_map(item):
        c_map = [0]*levels
        for x in item:
            c_map[x % levels] += 1
        return c_map


    data.sort(key=lambda x : len(x))
    lengths = [len(x) for x in data]

    min_length = min(lengths)
    store = []
    for x in range(max(lengths) - min(lengths) + 1):
        store.append(dict())


    for d in data:
        l = create_map(d)
        pass

    print(data)

data = [[0, 1, 2], [1],[0]]
data = [set(x) for x in data]

results = interset_basic(data)
results = intersect_store(data)

results = intersect_lists(data, 2)

print(results)

