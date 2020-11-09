from typing import Dict

def full_test() -> None:
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
    remove_player(graph, "B")
    remove_player(graph, "O")
    remove_player(graph, "K")
    remove_player(graph, "F")
    print(f"new team: {graph}")

    validate_graph(graph)

    distance, path = score(graph, "A", "goal", "CDEGHIJLMN", "A", 0)
    if distance < 0:
        print("failed")
    else:
        print(f"success: path={path}, distance={distance}")

def score(graph: Dict[str, Dict[str, int]], start: str, goal: str, untouched: str, path: str, distance: int) -> (int, str):
    # have we scored yet?
    if start == goal:
        if not untouched:
            return -1, ""
        else:
            return distance, path
    
    moves = graph[start]

    # have we used all the players? If so, try to score
    if not untouched and goal in moves:
        return score(graph, goal, goal, untouched, path + goal, distance + moves[goal])
    
    # otherwise, try to make a move that results in a goal and minimises the distance
    min_distance = -1
    min_path = ""
    for move in moves:
        # do not make a move we have made before
        edge = start + move  # e.g. "AB"
        if edge in path:
            continue

        new_path = path + move
        new_untouched = untouched
        if move in untouched:
            new_untouched = untouched.replace(move, "")
        
        size = moves[move]
        result, result_path = score(graph, move, goal, new_untouched, new_path, distance + size)
        if result < 0:
            continue

        if min_distance < 0 or result < min_distance:
            min_distance = result
            min_path = result_path
    
    return min_distance, min_path

def remove_player(graph: Dict[str, Dict[str, int]], player: str) -> None:
    del graph[player]
    for other in graph:
        moves = graph[other]
        if player in moves:
            del moves[player]

def validate_graph(graph: Dict[str, Dict[str, int]]) -> None:
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
