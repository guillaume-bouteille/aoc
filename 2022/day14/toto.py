
import json
from functools import cmp_to_key

def parse_inputs(file_name):
    rocks = [ l.strip().split(" -> ") for l in open(file_name) ]
    rocks = list(map(lambda x : [list(map(int, it.split(","))) for it in x], rocks))
    return rocks
    
def gen_path(pos_1, pos_2):
    if pos_1[0] == pos_2[0]:
        return [ [pos_1[0], i] for i in range(min(pos_1[1], pos_2[1]), max(pos_1[1], pos_2[1])+1) ]
    else:
        return [ [i, pos_1[1]] for i in range(min(pos_1[0], pos_2[0]), max(pos_1[0], pos_2[0])+1) ]
    

def init_map(inputs):
    min_x = 400
    max_x = 600
    min_y = 0
    max_y = 200
    my_map = [ ["." for i in range(min_x, max_x)] for j in range(min_y, max_y)]
    for rock in inputs:
        for i in range(len(rock)-1):
            for x,y in gen_path(rock[i+1], rock[i]):
                print(x,y)
                my_map[y-min_y][x-min_x] = "#"
    return my_map
    
def draw_map(my_map):
    for i in range(len(my_map)):
        print("".join(my_map[i]))
        
def exo1(inputs):
    my_map = init_map(inputs)
    draw_map(my_map)
    
    # TODO drop the sand!
    return 1
    
def exo2(inputs):
    
    return 2
    
inputs = parse_inputs("input.txt")
print(inputs)


answer1 = exo1(inputs)
print(f"The first answer is {answer1}")
answer2 = exo2(inputs)
print(f"The second answer is {answer2}")
