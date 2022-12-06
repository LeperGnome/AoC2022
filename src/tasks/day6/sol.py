from collections import deque

data = open("./data.txt", 'r').read()

win_size = 14

deck = deque()
for idx, c in enumerate(data):
    deck.append(c)
    if len(deck) == win_size:
        if len(set(deck)) == win_size:
            print(idx+1)
            break
        deck.popleft()
