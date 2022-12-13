
import json
from functools import cmp_to_key

def parse_inputs(file_name):
    with open(file_name) as f:
        ls = f.readlines()
    pairs = []
    for i in range(len(ls)//3+1):
        pairs.append([json.loads(ls[3*i]), json.loads(ls[3*i+1])])
    return pairs
    
    
# Return 1 if lhs > rhs, 0 if lhs == 0, -1 if lhs < rhs
def my_compare(lhs, rhs):
    if isinstance(lhs, list) and isinstance(rhs, list):
        for i,lit in enumerate(lhs):
            if i >= len(rhs):
                return 1
            cmp = my_compare(lhs[i], rhs[i])
            if cmp != 0:
                return cmp
        if len(rhs) > len(lhs):
            return -1
        else:
            return 0
    elif isinstance(lhs, int) and isinstance(rhs, int):
        if lhs > rhs:
            return 1
        elif lhs < rhs:
            return -1
        else:
            return 0
    else:
        if isinstance(lhs, int):
            lhs = [lhs]
        if isinstance(rhs, int):
            rhs = [rhs]
        return my_compare(lhs, rhs)
        
        
def exo1(inputs):
    answer = 0
    for i, (lhs, rhs) in enumerate(inputs):
        if my_compare(lhs, rhs) == -1:
            answer += i+1
    return answer
    
def exo2(inputs):
    inputs = [it for sub in inputs for it in sub] # Flatten 1 level
    inputs += [[[2]], [[6]]]
    inputs = sorted(inputs, key=cmp_to_key(my_compare))
    
    for i,it in enumerate(inputs):
        if it == [[2]]:
            a = i+1
        if it == [[6]]:
            b = i+1
    return a*b
    
inputs = parse_inputs("input.txt")

answer1 = exo1(inputs)
print(f"The first answer is {answer1}")
answer2 = exo2(inputs)
print(f"The second answer is {answer2}")
