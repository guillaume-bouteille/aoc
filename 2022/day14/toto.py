
import json
from functools import cmp_to_key
import curses

def parse_inputs(file_name):
    rocks = [ l.strip().split(" -> ") for l in open(file_name) ]
    rocks = list(map(lambda x : [list(map(int, it.split(","))) for it in x], rocks))
    return rocks
    
def gen_path(pos_1, pos_2):
    if pos_1[0] == pos_2[0]:
        return [ [pos_1[0], i] for i in range(min(pos_1[1], pos_2[1]), max(pos_1[1], pos_2[1])+1) ]
    else:
        return [ [i, pos_1[1]] for i in range(min(pos_1[0], pos_2[0]), max(pos_1[0], pos_2[0])+1) ]
    
# Because flemme
min_x = 300
max_x = 700
min_y = 0
max_y = 175

def init_map(inputs):
    my_map = [ ["." for i in range(min_x, max_x+1)] for j in range(min_y, max_y+1)]
    for rock in inputs:
        for i in range(len(rock)-1):
            for x,y in gen_path(rock[i+1], rock[i]):
                my_map[y-min_y][x-min_x] = "#"
    return my_map
        
def wow_much_simulation(stdscr, inputs):
    lp = get_lowest_point(inputs)
    my_map = init_map(inputs+[[[min_x, lp+2], [max_x, lp+2]]])
    # Clear screen
    stdscr.clear()
    
    while True:
        x,y = drop_next(my_map, 500, 0)
        my_map[y-min_y][x-min_x] = "O"

        if (x,y) == (500, 0):
            break

        for i in range(len(my_map)):
            stdscr.addstr(i, 0, "".join(my_map[i][100:300]))

        stdscr.refresh()
        
def get_lowest_point(inputs):
    t = [it[1] for sub in inputs for it in sub] # Flatten 1 level
    return max(t)
    
    
def drop_next(my_map, x, y):
    stopped = False
    while not stopped:
        if y >= max_y:
            return (500,0)
        if my_map[y-min_y+1][x-min_x] == ".":
            y = y+1
        elif my_map[y-min_y+1][x-min_x-1] == ".":
            y = y+1
            x = x-1
        elif my_map[y-min_y+1][x-min_x+1] == ".":
            y = y+1
            x = x+1
        else:
            stopped = True
    return (x,y)
    
def compute(my_map):
    count = 0
    while True:
        x,y = drop_next(my_map, 500, 0)
        my_map[y-min_y][x-min_x] = "O"
        count += 1
        if (x,y) == (500, 0):
            break

    return count
        
def exo1(inputs):
    my_map = init_map(inputs)
    return compute(my_map)-1
    
def exo2(inputs):
    lp = get_lowest_point(inputs)
    my_map = init_map(inputs+[[[min_x, lp+2], [max_x, lp+2]]])

    return compute(my_map)

    
inputs = parse_inputs("input.txt")


answer1 = exo1(inputs)
print(f"The first answer is {answer1}")
answer2 = exo2(inputs)
print(f"The second answer is {answer2}")

# Caution! This takes a lot of time ! And requires your terminal to be set to display many characters!!!
curses.wrapper(wow_much_simulation, inputs)

