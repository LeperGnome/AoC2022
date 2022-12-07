from collections import defaultdict


data = open("./data.txt", 'r').readlines()

sizes = defaultdict(int)
dirs = {'/': {}}
accepted = []

def size_of_dir(d, th, cur = ''):
    if not d:
        cur_size = sizes[cur]
        if cur_size >= th:
            accepted.append((cur, cur_size))

        return cur_size
    sub_sizes = 0
    for k, v in d.items():
        if cur == '/': 
            next_dir = cur + k
        elif cur == '':
            next_dir = k 
        else:
            next_dir = cur + '/' + k
        sub_sizes += size_of_dir(v,th, next_dir)
    cur_size = sizes[cur] + sub_sizes
    if cur_size >= th:
        accepted.append((cur, cur_size))
    return cur_size
        

def set_dir(new):
    # new -> [/ a b c]
    cur = dirs.get('/')
    for el in new:
        if el == '/':
            continue
        if cur.get(el) is None:
            cur[el] = {}
        else:
            cur = cur.get(el)

cur_dir = ["/"]

for line in data[1:]:
    line = line.rstrip('\n')
    if line[0] == '$':
        args = line[2:].split(' ')
        
        if args[0] == 'cd':
            d = args[1]
            if d == '..':
                cur_dir.pop()
            else:
                cur_dir.append(d)
    elif line[:3] == 'dir':
        _, args = line.split(' ', 1)
        set_dir(cur_dir + [args])
    else:
        size, fname = line.split(' ', 1)
        sizes['/'+'/'.join(cur_dir[1:])] += int(size)



print('-------------')

th = 30000000
total_used = size_of_dir(dirs, th)
total_free = 70000000 - total_used
print(f"{total_used=} {total_free=}")

print('-------------')

need_to_free = th - total_free
accepted.clear()
print(f"{need_to_free=}")
total_used = size_of_dir(dirs, th=need_to_free)

print('-------------')

accepted.sort(key=lambda x: x[1])
print(f"{accepted=}")
print(f"result: {accepted[0][1]}")
