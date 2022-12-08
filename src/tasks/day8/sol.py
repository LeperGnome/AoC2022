data = open("./data.txt", 'r').readlines()

known_coords = set()

columns = [[] for _ in range(len(data[0])-1)]

for ri, row in enumerate(data):
    row = row.strip()
    known_coords.add((ri, 0))
    known_coords.add((ri, len(row)-1))

    lmax = int(row[0])
    for i, t in enumerate(row):
        t = int(t)
        columns[i].append(t)
        if t > lmax:
            lmax = t
            known_coords.add((ri,i))

    row = list(reversed(row))
    rmax = int(row[0])
    for i, t in enumerate(row):
        t = int(t)
        if t > rmax:
            rmax = t
            known_coords.add((ri, (len(row) - 1 - i)))

for ci, col in enumerate(columns):
    lmax = col[0]
    known_coords.add((0, ci))
    known_coords.add((len(col)-1, ci))

    for i, t in enumerate(col):
        if t > lmax:
            known_coords.add((i, ci))
            lmax = t

    col = list(reversed(col))
    rmax = col[0]
    for i, t in enumerate(col):
        if t > rmax:
            rmax = t
            known_coords.add(((len(col) - 1 - i), ci))

print("Pt 1:", len(known_coords))

def calc_dir_score(coords, direct) -> int:
    r, c = coords
    cur_h = int(data[r][c])
    n = 0
    if direct == 'up':
        it = columns[c][r-1::-1]
    elif direct == 'down':
        it = columns[c][r+1:]
    elif direct == 'left':
        it = data[r][c-1::-1].strip()
    elif direct == 'right':
        it = data[r][c+1:].strip()
    else:
        raise RuntimeError()

    for t in it:
        n += 1
        if cur_h <= int(t):
            break
    return n

def calc_score(coords) -> int:
    total = 1
    for d in ['up', 'down', 'left', 'right']:
        total *= calc_dir_score(coords, d)
    return total

scores = set()
for ri, row in enumerate(data):
    for ci, _ in enumerate(row.strip()):
        if ri == 0 or ci == 0:
            continue
        sc = calc_score((int(ri), int(ci)))
        scores.add(sc)

print("Pt 2:", max(scores))
