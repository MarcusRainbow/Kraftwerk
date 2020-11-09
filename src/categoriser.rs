use std::collections::{HashMap, HashSet};
use football::validate_graph;

pub fn full_test() -> () {
    let graph = [
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

    // first find the shortest route to the goal
    let categories = categorise(&graph, 'A');
    // println!("graph: {:?}", graph);
    // println!("categories: {:?}", categories);
    let (distance, ref path) = categories[&'x'];
    println!("shortest route to goal is: {}, distance {}, number of players {}", path, distance, path.len() - 1);

    // who are the unused players
    let mut unused = HashSet::new();
    for player in (b'A' ..= b'O').map(char::from) {
        if !path.contains(player) {
            unused.insert(player);
        } 
    } 
    println!("unused players {:?}", unused);

    // now keep trying to add players until only four unused
    let (extra, ref path) = add_players(&graph, path, &mut unused, 4);
    if extra < 0 {
        println!("Unable to find a route with 11 players. Best effort is {}", path);
    } else {
        println!("shortest route using 11 players is: {}, distance {}", path, distance + extra);
    }
}

pub fn add_players(
        graph: &HashMap<char, HashMap<char, i64>>, 
        path: &str, 
        unused: &mut HashSet<char>, count: usize) -> (i64, String) {

    let mut min_cost = -1;
    let mut min_path = "".to_string();
    let mut current_path = path.to_string();
    while unused.len() > count {
        min_cost = -1;
        let mut min_player = '?';
        for player in unused.iter() {
            let (distance, path) = add_player(graph, *player, &current_path);
            if distance >= 0 && (min_cost < 0 || distance < min_cost) {
                min_cost = distance;
                min_path = path;
                min_player = *player;
            }
        }

        if min_cost < 0 {
            println!("Failed to find a player who can be inserted into {}", path);
            return (-1, current_path);
        }

        unused.remove(&min_player);
        current_path = min_path.clone();
    }

    (min_cost, min_path)
}

/// Find the best place to insert a player in a path, to reduce the distance to the goal.
/// If player cannot easily be inserted anywhere (e.g. not adjacent to any player on the path) return -1.
pub fn add_player(graph: &HashMap<char, HashMap<char, i64>>, player: char, path: &str) -> (i64, String) {

    // println!("add_player: player={}, path={}", player, path);
    // first try a single insert (e.g. replace GH with GXH for new player X)
    let player_moves = &graph[&player];
    let mut min_distance = -1;
    let mut min_path = "".to_string();
    let mut players = path.chars();
    if let Some(first) = players.next() {
        let mut from = first;
        for (i, to) in players.enumerate() {
            // we can only do an insert if there are edges both ways
            if let Some(step_to) = player_moves.get(&to) {
                let from_moves = &graph[&from];
                // println!("add_player: player can step to={}, from={}, from_moves={:?}", to, from, from_moves);
                if let Some(step_from) = from_moves.get(&player) {
                    let straight_line = graph[&from][&to];
                    let distance = step_from + step_to - straight_line;
                    // println!("add_player: success! distance={}", distance);
                    if min_distance < 0 || distance < min_distance {
                        min_distance = distance;
                        min_path = path.to_string();
                        min_path.insert(i, player);
                    } 
                }
            }
            from = to;
        }
    }

    // Also try a simple spur insert, (e.g. replace GH with GXGH for new player X)
    players = path.chars();
    for (i, from) in players.enumerate() {
        // we can only do an insert if there are edges both ways
        if let Some(step_to) = player_moves.get(&from) {
            let from_moves = &graph[&from];
            // println!("add_player: player can step from={}, from_moves={:?}", from, from_moves);
            let step_from = from_moves[&player];
            let distance = step_from + step_to;
            // println!("add_player: success! distance={}", distance);
            if min_distance < 0 || distance < min_distance {
                min_distance = distance;
                min_path = path.to_string();
                min_path.insert(i, player);
                min_path.insert(i, from);
            }
        }
    }

    // TODO try fancier ways of doing the insert here

    (min_distance, min_path)
}

pub fn evaluate_path(graph: &HashMap<char, HashMap<char, i64>>, path: &str) -> i64 {
    let mut distance = 0;
    let mut players = path.chars();
    if let Some(first) = players.next() {
        let mut from = first;
        for to in players {
            distance += graph[&from][&to];
            from = to;
        }
    }
    distance
}

/// Given a graph, assign a distance to each node, and the optimal path to get there
pub fn categorise(graph: &HashMap<char, HashMap<char, i64>>, start: char)
    -> HashMap<char, (i64, String)> {

    let mut categories : HashMap<char, (i64, String)> = HashMap::new();
    let mut path = "".to_string();
    path.push(start);
    update_categories(&graph, &mut categories, start, &path, 0);
    categories
}

fn update_categories(
        graph: &HashMap<char, HashMap<char, i64>>,
        categories: &mut HashMap<char, (i64, String)>,
        start: char, 
        path: &str, 
        distance: i64) -> () {

    categories.insert(start, (distance, path.to_string()));

    let moves = &graph[&start];
    for (player, step) in moves {
        let new_distance = distance + step;
        let mut want_this = true;
        if let Some((distance, _)) = categories.get(&player) {
            want_this = new_distance < *distance;
        }
        if want_this {
            let mut new_path = path.to_string();
            new_path.push(*player);
            update_categories(&graph, categories, *player, &new_path, new_distance);
        }
    }
}
