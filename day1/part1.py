try:
    highest = -1
    current = 0
    while True:
        inp = input()
        if inp == "":
            if current > highest:
                highest = current
            current = 0
            continue
        current += int(inp)
except EOFError:
    print(highest)