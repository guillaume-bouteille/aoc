#!/usr/bin/env python3
elves = [0]

# Read inputs
for l in open("input", "r"):
    if l.strip() == "":
        elves.append(0)
    else:
        elves[-1] += int(l.strip())
print(elves)

# Find the highest value
print(max(elves))

# Sum the top 3 values
print(sum(sorted(elves)[-3:]))
