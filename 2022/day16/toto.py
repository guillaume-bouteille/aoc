
import re
def parse_inputs(file_name):
    inputs = {}
    for l in open(file_name):
        m = re.match("Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.+)$", l.strip())
        assert(m != None)
        inputs[m.group(1)] = (int(m.group(2)), m.group(3).replace(" ", "").split(","))
    return inputs

def compute_distance(inputs, a, b):
    def rec(inputs, visited, a, b, di):
        if a == b:
            return di
        else:
            p = [rec(inputs, visited+[it], it, b, di+1) for it in inputs[a][1] if not it in visited]
            if len(p) == 0:
                return 100000000
            return min(p)

    return rec(inputs, [], a, b, 0)

pi_dist_ = None
def compute_pi_distances(inputs, pi):
    global pi_dist_
    if pi_dist_ != None:
        return pi_dist_
        
    # Optimize?
    distances = {}
    for a in pi:
        for b in pi:
            if a!=b:
                d = compute_distance(inputs, a, b)
                distances[(a,b)] = d + 1
                distances[(b,a)] = d + 1
        distances[("AA",a)] = compute_distance(inputs, "AA", a) + 1
 
    pi_dist_=distances
    return distances

def get_path_length(distances, path):
    l = 0
    for i in range(len(path)-1):
        l += distances[(path[i], path[i+1])]
    return l
    
def get_best_pressure_release(pi, distances):

    def rec(pi, distances, curr_path, curr_d, curr_pv, curr_p):
    
        p_def = curr_p + curr_pv*(30-curr_d)   
        
        for p in filter(lambda x : not x in curr_path, pi):
            d = distances[(curr_path[-1], p)]
            if curr_d+d < 30:
                p_def = max(p_def, rec(pi, distances, curr_path+[p], curr_d+d, curr_pv+inputs[p][0], curr_p+curr_pv*d))
    
        return p_def
        
    return rec(pi, distances, ["AA"], 0, 0, 0)


# WIP - clairement, c'est pas la bonne approche, ca fait 15min que l'algo tourne
def get_best_pressure_release_with_elephant(pi, distances):
    def rec(pi, distances, curr_path_1, curr_path_2, curr_d, curr_pv, curr_p, next_d, next_pv):
    
        p_def = curr_p + curr_pv*(26-curr_d) + next_pv*(26-curr_d-next_d)        
        remaining_pi = filter(lambda x : not x in curr_path_1+curr_path_2, pi)
        for p in remaining_pi:
            d = distances[(curr_path_1[-1], p)]
            if curr_d+next_d < 26 and next_d < d: # curr_path_2 comes next
                n_def = rec(pi, distances, curr_path_2, curr_path_1+[p], curr_d+next_d, curr_pv+next_pv, curr_p+curr_pv*next_d, d-next_d, inputs[p][0])
                p_def = max(n_def, p_def)
            elif curr_d+d < 26: # curr_path_1 comes next
                n_def = rec(pi, distances, curr_path_1+[p], curr_path_2, curr_d+d, curr_pv+inputs[p][0], curr_p+curr_pv*d, next_d-d, next_pv)
                p_def = max(n_def, p_def)
    
        return p_def
        
    return rec(pi, distances, ["AA"], ["AA"], 0, 0, 0, 0, 0)

def exo1(inputs):
    # PI = points of interest
    pi = [k for k,v in inputs.items() if v[0] > 0]
    distances = compute_pi_distances(inputs, pi)
    best_pressure = get_best_pressure_release(pi, distances)
    return best_pressure
    
def exo2(inputs):
    # PI = points of interest
    pi = [k for k,v in inputs.items() if v[0] > 0]
    distances = compute_pi_distances(inputs, pi)
    best_pressure = get_best_pressure_release_with_elephant(pi, distances)
    return best_pressure

    
inputs = parse_inputs("input.txt")
answer1 = exo1(inputs)
print(f"The first answer is {answer1}")
answer2 = exo2(inputs)
print(f"The second answer is {answer2}")
