use std::collections::HashMap;

pub fn full_test() -> () {
    let mut graph = [
        ('A', [('B', 1), ('C', 4)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('B', [('A', 1), ('C', 2), ('D', 15), ('E', 21), ('G', 25)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('C', [('A', 4), ('B', 2), ('E', 19), ('F', 5), ('H', 13)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('D', [('B', 15), ('G', 10), ('J', 18)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('E', [('B', 21), ('C', 19), ('G', 4), ('H', 6)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('F', [('C', 5), ('H', 7), ('K', 20)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('G', [('B', 25), ('D', 10), ('E', 4), ('H', 11), ('I', 2), ('J', 8), ('L', 16)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('H', [('C', 13), ('E', 6), ('F', 7), ('G', 11), ('I', 12), ('K', 7), ('M', 31)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('I', [('G', 2), ('H', 12), ('L', 14), ('M', 19)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('J', [('D', 18), ('G', 8), ('L', 7), ('N', 4)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('K', [('F', 20), ('H', 7), ('M', 17)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('L', [('G', 16), ('I', 14), ('J', 7), ('M', 5), ('N', 7), ('O', 5)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('M', [('H', 31), ('I', 19), ('K', 17), ('L', 5), ('O', 3)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('N', [('J', 4), ('L', 7), ('O', 2), ('x', 5)]
            .iter().cloned().collect::<HashMap<_, _>>()),
        ('O', [('L', 5), ('M', 3), ('N', 2), ('x', 3),
            ].iter().cloned().collect::<HashMap<_, _>>()),
        ('x', [].iter().cloned().collect::<HashMap<_, _>>()),
        ].iter().cloned().collect::<HashMap<_, _>>();
    validate_graph(&graph);
    remove_player(&mut graph, 'B');
    remove_player(&mut graph, 'O');
    remove_player(&mut graph, 'K');
    remove_player(&mut graph, 'F');
    println!("new team: {:?}", graph);
    validate_graph(&graph);
    let (distance, path) = heuristic_score(&mut graph, 'A', 'x', "A", 0);
    //let (distance, path) = score(&graph, 'A', 'x', "CDEGHIJLMN", "A", 0);
    if distance < 0 {
        println!("failed: path={}, graph={:?}", path, graph);
    } else {
        println!("success: path={}, distance={}", path, distance);
    }
}

pub fn score(graph: &HashMap<char, HashMap<char, i64>>, 
    start: char, goal: char, untouched: &str, path: &str, distance: i64) -> (i64, String) {
    if start == goal {
        if untouched.is_empty() {
            return (-1, "".to_string());
        } else {
            return (distance, path.to_string());
        }
    }
    let moves = &graph[&start];
    if untouched.is_empty() {
        if let Some(step) = moves.get(&goal) {
            let mut new_path = path.to_string();
            new_path.push(goal);
            return score(graph, goal, goal, untouched, &new_path, distance + step);
        }
    }
    let mut min_distance = -1;
    let mut min_path = "".to_string();
    for (player, step) in moves {
        let mut edge = "".to_string();
        edge.push(start);
        edge.push(*player);
        if path.contains(&edge) {
            continue;
        }
        let mut new_path = path.to_string();
        new_path.push(*player);
        let mut new_untouched = untouched.to_string();
        new_untouched.retain(|c| c != *player);
        let (result, result_path) = score(graph, *player, goal, &new_untouched, &new_path, distance + step);
        if result < 0 {
            continue;
        }
        if min_distance < 0 || result < min_distance {
            min_distance = result;
            min_path = result_path;
        }
    }
    return (min_distance, min_path);
}

pub fn heuristic_score(graph: &mut HashMap<char, HashMap<char, i64>>, 
    start: char, goal: char, path: &str, distance: i64) -> (i64, String) {

    // Simple nearest-neighbour heuristic:
    // After we visit a node, remove that node from the graph. From each node, go to
    // the nearest neighbour.
    if start == goal {
        if graph.is_empty() {
            return (distance, path.to_string());
        } else {
            return (-1, path.to_string());
        }
    }

    // remove the node we are starting from, but remember its contents
    let moves = graph[&start].clone();
    let n_moves = moves.len();
    remove_player(graph, start);

    // if there is nowhere to go, fail
    if n_moves == 0 {
        return (-1, path.to_string()); 
    }

    // find the nearest neighbour, but do not shoot for goal unless it is the only one
    let mut min_distance = -1;
    let mut min_player = '?';
    for (player, step) in moves {
        if player == 'x' && n_moves != 1 {
            continue;
        }
        if min_distance < 0 || step < min_distance {
            min_distance = step;
            min_player = player;
        }
    }

    assert!(min_distance >= 0);

    let mut new_path = path.to_string();
    new_path.push(min_player);
    return heuristic_score(graph, min_player, goal, &new_path, distance + min_distance);
}

pub fn remove_player(graph: &mut HashMap<char, HashMap<char, i64>>, player: char) -> () {
    graph.remove(&player);
    for (_, moves) in graph {
        moves.remove(&player);
    }
}

pub fn validate_graph(graph: &HashMap<char, HashMap<char, i64>>) -> () {
    for (start, moves) in graph {
        for (player, _) in moves {
            assert!(graph.contains_key(&player));
            let next_moves = &graph[&player];
            if *player == 'x' {
                if !next_moves.is_empty() {
                    println!("Error: should not be possible to move from goal");
                }
            } else {
                if let Some(step) = next_moves.get(&start) {
                    if let Some(step_back) = moves.get(&player) {
                        if step_back != step {
                            println!("Error: move from {} to {} does not match reverse", start, player);
                        }
                    }
                } else {
                    println!("Error: cannot move to {} from {}", start, player);
                }
            }
        }
    }
}
