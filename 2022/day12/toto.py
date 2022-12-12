
import copy

def parse_inputs(file_name):
    return [l.strip() for l in open(file_name)]

def find_c(inputs, c):
    for x, l in enumerate(inputs):
        for y, cc in enumerate(l):
            if c == cc:
                return (x, y)

def is_in_map(inputs, pos):
    return pos[0] >= 0 and pos[0] < len(inputs) and pos[1] >= 0 and pos[1] < len(inputs[0])

def can_go(inputs, next_pos, pos):
    return ord(inputs[pos[0]][pos[1]])-ord(inputs[next_pos[0]][next_pos[1]]) <= 1

def get_adjacents(inputs, visited, to_visit, pos):
    x,y = pos
    r = [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
    return list(filter(lambda p : (not (p in visited)) and (not (p in to_visit)) and is_in_map(inputs, p) and can_go(inputs, p, pos), r) )

def get_map(inputs):
    sx,sy = find_c(inputs, 'S')
    ex,ey = find_c(inputs, 'E')
    inputs = copy.deepcopy(inputs) # Since the next 2 intructions will modify input
    inputs[sx] = inputs[sx][0:sy] + 'a' + inputs[sx][sy+1:]
    inputs[ex] = inputs[ex][0:ey] + 'z' + inputs[ex][ey+1:]
    
    visited = [(ex,ey)]
    my_map = [[None for j in range(len(inputs[0]))] for j in range(len(inputs)) ]
    to_visit = get_adjacents(inputs, visited, [], (ex, ey))
    for x,y in to_visit:
        my_map[x][y] = (ex,ey)
    while len(to_visit) > 0:
        p = to_visit.pop(0)
        visited.append(p)
        adj = get_adjacents(inputs, visited, to_visit, p)
        for x,y in adj:
            my_map[x][y] = p

        to_visit += adj

    return my_map

def print_map(my_map):
    # Nice!
    for l in my_map:       
        print("".join([ "." if x == None else "X" for x in l]))


def get_nb_steps(my_map, sx,sy,ex,ey):
    x,y = sx,sy
    count = 0
    while (x,y) != (ex,ey):
        count += 1
        if my_map[x][y] == None: # This means there is no path
            return 1000000000000000000
        x,y = my_map[x][y]
    return count


def exo1(inputs):
    sx,sy = find_c(inputs, 'S')
    ex,ey = find_c(inputs, 'E')
    my_map = get_map(inputs)

    #print_map(my_map)

    return get_nb_steps(my_map, sx,sy,ex,ey)

def exo2(inputs):

    sx,sy = find_c(inputs, 'S')
    ex,ey = find_c(inputs, 'E')
    my_map = get_map(inputs)

    #print_map(my_map)

    best_nb_steps = 1000000000
    for x,l in enumerate(inputs):
        for y,c in enumerate(l):
            if c == 'a':
                nb_steps = get_nb_steps(my_map, x,y,ex,ey)
                if nb_steps < best_nb_steps:
                    best_nb_steps = nb_steps
    return best_nb_steps
    
    
    
inputs = parse_inputs("input.txt")
answ1 = exo1(inputs)
print(f"The first answer is {answ1}")
answ2 = exo2(inputs)
print(f"The second answer is {answ2}")

