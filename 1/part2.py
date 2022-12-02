try:
    elves = []
    current = 0
    while True:
        inp = input()
        if inp == "":
            elves.append(current)
            current = 0
            continue
        current += int(inp)
except EOFError:
    elves.sort(reverse=True)
    print(elves[0]+elves[1]+elves[2])
