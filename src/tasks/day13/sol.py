"""
nested structure here, so not doing rust =(
"""

from functools import cmp_to_key

def indian_cmp(lhs, rhs):
    if lhs < rhs:
        return -10
    elif rhs < lhs:
        return 10
    return None

def compare(lhs, rhs):
    print(lhs,"---vs---", rhs)
    if type(lhs) == list and type(rhs) == list:
        if not lhs and not rhs:
            return None
        elif not lhs:
            return -10
        elif not rhs:
            return 10
        il = iter(lhs)
        ir = iter(rhs)
        for _ in range(max(len(lhs), len(rhs))):
            try:
                n_l = next(il)
            except StopIteration:
                return -10

            try:
                n_r = next(ir)
            except StopIteration:
                return 10

            res = compare(n_l, n_r)
            if res is not None:
                return res

    elif type(lhs) == list:
        rhs = [rhs]
        return compare(lhs, rhs)
    elif type(rhs) == list:
        lhs = [lhs]
        return compare(lhs, rhs)
    else:
        res = indian_cmp(lhs, rhs)
        if res is not None:
            return res

data = list(
    map(
        eval, 
        [
            l.strip() for l 
            in open("./data.txt", 'r').readlines() 
            if l.strip()
        ]
    )
) + [[[2]]] + [[[6]]]

corr_packets = sorted(data, key=cmp_to_key(compare))
res = 1
print("^^^^^^^^^^^^^^^^^^^^^^^^^")
for i, el in enumerate(corr_packets):
    print(el)
    idx = i + 1
    if el  == [[2]] or el == [[6]]:
        res *= idx

print(res)
