
import re
from functools import cmp_to_key

def parse_inputs(file_name):
    inputs = []
    for l in open(file_name):
        m = re.match("Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)", l.strip())
        assert m != None
        inputs.append((int(m.group(1)), int(m.group(2)), int(m.group(3)), int(m.group(4))))
    return inputs
    
def get_x_edges(inputs):
    min_x = 10000000000
    max_x = 0
    for xs,ys,xb,yb in inputs:
        if xs < min_x:
            min_x = xs
        if xs > max_x:
            max_x = xb
        if xb < min_x:
            min_x = xb
        if xb > max_x:
            max_x = xb
    return (min_x,max_x)
    
    
def get_impossible_loc(xs,ys,xb,yb, y):
    d = get_dist(xs,ys,xb,yb)
    d0 = get_dist(xs,ys,xs,y)
    if d0 <= d:
        return [xs-(d-d0),xs+(d-d0)]
    else:
        return None

def my_compare(lhs, rhs):
    if lhs[0] > rhs[0]:
        return 1
    elif lhs[0] < rhs[0]:
        return -1
    else:
        return 0

def merge_and_sort(impossible_loc):
    impossible_loc = [i for i in impossible_loc if i != None]
    #print("Before sort - ", impossible_loc)
    # Sort lists
    impossible_loc = sorted(impossible_loc, key=cmp_to_key(my_compare))
    #print("After sort - ", impossible_loc)
    
    # Merge lists
    i = 0
    while i < len(impossible_loc)-1:
        #print(i, impossible_loc)
        if impossible_loc[i][1] + 1 > impossible_loc[i+1][0]:
            impossible_loc = impossible_loc[0:i] + [[impossible_loc[i][0], max(impossible_loc[i][1], impossible_loc[i+1][1])]] + impossible_loc[i+2:]
            i = 0
        else:
            i += 1
    #print("After merge - ", impossible_loc)    
    return impossible_loc

def idx_beacon_in_impossible_loc(impossible_loc, xb):
    for i,p in enumerate(impossible_loc):
        if p[0] <= xb <= p[1]:
            return i
    return None

def add_beacon(impossible_loc, inputs, y):
    for _,_,xb,yb in inputs:
        if yb == y:
            idx = idx_beacon_in_impossible_loc(impossible_loc, xb)
            if idx != None:
                #print("Adding beacon - ",idx,xb)
                #print(impossible_loc[0:idx])
                #print(impossible_loc[idx+1:])
                impossible_loc = impossible_loc[0:idx] + [[impossible_loc[idx][0], xb-1]] + [[xb+1, impossible_loc[idx][1]]] + impossible_loc[idx+1:]
            #print(impossible_loc)
    return impossible_loc
    
def count_impossible_location(impossible_loc):
    #print("Count - ",impossible_loc, sum(map(lambda p : p[1]-p[0]+1, impossible_loc)))
    return sum(map(lambda p : p[1]-p[0]+1, impossible_loc))



def get_dist(x1,y1, x2,y2):
    return abs(x1-x2)+abs(y1-y2)
    
    
def get_full_impossible_loc(inputs, y, min_x, max_x, do_add_beacon=True):
    
    impossible_loc = [get_impossible_loc(xs,ys,xb,yb, y) for xs,ys,xb,yb in inputs]
    
    impossible_loc = merge_and_sort(impossible_loc)
    
    if do_add_beacon:
        impossible_loc = add_beacon(impossible_loc, inputs, y)
    return impossible_loc



def exo1(inputs):
    y = 2000000
    min_x,max_x = get_x_edges(inputs)
    
    return count_impossible_location(get_full_impossible_loc(inputs, y, min_x, max_x))
    
def exo2(inputs):
    min_x,max_x = get_x_edges(inputs)
    
    # print("Beacons")
    # beacons=set()
    # for _,_,xb,yb in inputs:
        # if 0 < xb < 4000000 and 0 < yb < 4000000:
            # beacons.add((xb,yb))
    # for b in beacons:
        # print(b)

    for y in range(0, 4000000):
        if y % 100000 == 0:
            print(f"Processing {y} / 4000000")
        l = get_full_impossible_loc(inputs, y, min_x,max_x, do_add_beacon=False)
        if len(l) > 1 or (l[0][0] > 0) or l[0][1] < 4000000 :
            #print(y,l)
            res = (l[0][1]+1,y)
            #print(res)
            break

    return res[0]*4000000+res[1]

    
inputs = parse_inputs("input.txt")
answer1 = exo1(inputs)
print(f"The first answer is {answer1}")
answer2 = exo2(inputs)
print(f"The second answer is {answer2}")
