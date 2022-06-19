def collatz(a:int) -> int :
    if (a % 2) == 0:
        return int(a/2)
    else:
        return 3*a + 1

how_long = dict()
for a in range(1, 1000_000):
    n = 0
    # print()
    # print(f"The collatz set of {a} is ... ")
    if not a in how_long:
        # print("... calculating ...")
        b = a
        memory = dict()
        while a != 1:
            b = collatz(b)
            # print(f"{b}")
            # incrementing step count of this number as well as in between number of sequence by 1
            n += 1
            for i in memory:
                memory[i] += 1

            if b in how_long:
                # print(f"found {b} goes {how_long[b]} steps long")
                # incrementing step count of this number as well as in between number of sequence by familiar number b
                n += how_long[b]
                for i in memory:
                    memory[i] += how_long[b]
                break
            else:
                memory[b] = 0          
        # saving this number and in between number in memory (how_long)
        how_long[a] = n
        for i in memory:
            # how_long[i] = memory[i]
            a
        
        # print(f" ... {n} steps long")

max = 0
num = 0    
for a in how_long:
    if how_long[a] > max:
        num = a;
        max = how_long[num]

print(f"{num} produces the longest chain of {max} steps")
    