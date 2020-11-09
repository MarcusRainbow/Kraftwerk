import copy

def test():
    graph = {
        "A" : {"B": 1, "C": 4},
        "B" : {"A": 1, "C": 2, "D": 15, "E": 21, "G": 25},
        "C" : {"A": 4, "B": 2, "E": 19}, # also F, H
        "D" : {"B": 15, "G": 10}, # also J
        "E" : {"B": 21, "C": 19, "G": 4}, # also H
        "G" : {}
    }

    validate_graph(graph)
    path, distance = find_route(graph, "A", "G", set(), 2)

    if distance < 0:
        print("failed")
    else:
        print(f"success: path={path}, distance={distance}")

def full_test():
    graph = {
        "A" : {"B": 1, "C": 4},
        "B" : {"A": 1, "C": 2, "D": 15, "E": 21, "G": 25},
        "C" : {"A": 4, "B": 2, "E": 19, "F": 5, "H": 13},
        "D" : {"B": 15, "G": 10, "J": 18},
        "E" : {"B": 21, "C": 19, "G": 4, "H": 6},
        "F" : {"C": 5, "H": 7, "K": 20},
        "G" : {"B": 25, "D": 10, "E": 4, "H": 11, "I": 2, "J": 8, "L": 16},
        "H" : {"C": 13, "E": 6, "F": 7, "G": 11, "I": 12, "K": 7, "M": 31},
        "I" : {"G": 2, "H": 12, "L": 14, "M": 19},
        "J" : {"D": 18, "G": 8, "L": 7, "N": 4},
        "K" : {"F": 20, "H": 7, "M": 17},
        "L" : {"G": 16, "I": 14, "J": 7, "M": 5, "N": 7, "O": 5},
        "M" : {"H": 31, "I": 19, "K": 17, "L": 5, "O": 3},
        "N" : {"J": 4, "L": 7, "O": 2, "goal": 5},
        "O" : {"L": 5, "M": 3, "N": 2, "goal": 3},
        "goal": {}
    }

    validate_graph(graph)
    path, distance = find_route(graph, "A", "G", set(), 4)

    if distance < 0:
        print("failed")
    else:
        print(f"success: path={path}, distance={distance}")

def partial_test():
    graph = {
        "A" : {"B": 1, "C": 4},
        "B" : {"A": 1, "C": 2, "D": 15, "E": 21, "G": 25},
        "C" : {"A": 4, "B": 2, "E": 19, "F": 5, "H": 13},
        "D" : {"B": 15, "G": 10, "J": 18},
        "E" : {"B": 21, "C": 19, "G": 4, "H": 6},
        "F" : {"C": 5, "H": 7, "K": 20},
        "G" : {"B": 25, "D": 10, "E": 4, "H": 11, "I": 2, "J": 8, "L": 16},
        "H" : {"C": 13, "E": 6, "F": 7, "G": 11, "I": 12, "K": 7, "M": 31},
        "I" : {"G": 2, "H": 12, "L": 14, "M": 19},
        "J" : {"D": 18, "G": 8, "L": 7, "N": 4},
        "K" : {"F": 20, "H": 7, "M": 17},
        "L" : {"G": 16, "I": 14, "J": 7, "M": 5, "N": 7, "O": 5},
        "M" : {"H": 31, "I": 19, "K": 17, "L": 5, "O": 3},
        "N" : {"J": 4, "L": 7, "O": 2, "goal": 5},
        "O" : {"L": 5, "M": 3, "N": 2, "goal": 3},
        "goal": {}
    }

    validate_graph(graph)
    path, distance = find_route(graph, "A", "G", set(), 2)

    if distance < 0:
        print("failed")
    else:
        print(f"success: path={path}, distance={distance}")

def find_route(graph, start, end, touched, skips):
    """
    Given a graph and a number of nodes we must skip, find a route that
    traverses the graph from start to end.

    If not found, return -1 as the distance.

    Returns a tuple of the route, distance.
    """
    assert start in graph
    assert end in graph
    assert skips >= 0
    
    n_touched = len(touched)
    n_graph = len(graph)
    still_to_touch = n_graph - skips - n_touched
    assert still_to_touch >= 0
    
    # are we at the goal?
    if start == end:
        if still_to_touch > 0:
            return [], -1   # fail if we have not touched enough nodes
        return [start], 0   # goal!

    if still_to_touch == 0:
        return [], -1   # fail if we have touched too many nodes and are not at the goal

    # otherwise, we must move somewhere
    moves_from_here = graph[start]
    if not moves_from_here:
        return [], -1  # deadend

    # if one node left to touch, it must be the goal. Try to go there straight if we can.
    if still_to_touch == 1:
        assert end not in touched
        if end in moves_from_here:
            return [start, end], moves_from_here[end]

    # otherwise try all possible moves from here
    min_distance = -1
    min_path = []
    for move in moves_from_here:
        step = moves_from_here[move]

        # Add new start to the points touched.
        if move not in touched:
            if still_to_touch == 1:
                continue    # we cannot go here
            new_touched = touched.copy()
            new_touched.add(move)
        else:
            new_touched = touched

        # Assumption 1: No player never passes the ball twice to the same player. We need some assumption
        # like this to avoid loops, and this feels like the mildest such assumption we can make.
        new_graph = remove_move(graph, start, move)
        path, distance = find_route(new_graph, move, end, new_touched, skips)
        if distance < 0:
            continue    # no route using this move

        # Assumption 2: Given a set of players we have already used and a player in possession, we can always take
        # the shortest route to the goal that touches the number of remaining players that we need to.
        distance += step
        if min_distance >= 0 or distance < min_distance:
            min_distance = distance
            min_path = [start] + path

    return min_path, min_distance

def remove_move(graph, start, move):
    assert start in graph
    assert move in graph[start]

    new_graph = copy.deepcopy(graph)
    moves = new_graph[start]
    del moves[move]

    assert move in graph[start] # make sure the deepcopy worked
    return new_graph

def validate_graph(graph):
    for start in graph:
        moves = graph[start]
        for move in moves:
            # check that the reverse move exists, and that it is the same length
            assert move in graph
            next_moves = graph[move]
            if move == "goal":
                if next_moves:
                    print("Error: should not be possible to move from goal")
            elif start not in next_moves:
                print(f"Error: cannot move to {start} from {move}")
            elif next_moves[start] != moves[move]:
                print(f"Error: move from {start} to {move} is {moves[move]} but reverse is {next_moves[start]}")

if __name__ == "__main__":
    full_test()
