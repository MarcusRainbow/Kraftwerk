import Prelude hiding (lookup)
import qualified Data.Map.Strict as Map

-- A Player (P) is represented by their name followed by a list of players
-- they can pass to, with the costs of the passes as an integer.
data Player = P Char [(Player, Int)]

-- Define the problem, which is a graph of football players (A..O) and a goal
-- with bidirectional passes between them. Note that in Haskell it is easy
-- to create these as circular references. In other languages such as Python
-- or Rust, we have to do things indirectly, using a dict.
-- For simplicity we also represent the goal as a player, but one with
-- no outgoing passes. The name of the goal is the character '.'
a = P 'A' [(b, 1), (c, 4)]
b = P 'B' [(a, 1), (c, 2), (d, 15), (e, 21), (g, 25)]
c = P 'C' [(a, 4), (b, 2), (e, 19), (f, 5), (h, 13)]
d = P 'D' [(b, 15), (g, 10), (j, 18)]
e = P 'E' [(b, 21), (c, 19), (g, 4), (h, 6)]
f = P 'F' [(c, 5), (h, 7), (k, 20)]
g = P 'G' [(b, 25), (d, 10), (e, 4), (h, 11), (i, 2), (j, 8), (l, 16)]
h = P 'H' [(c, 13), (e, 6), (f, 7), (g, 11), (i, 12), (k, 7), (m, 31)]
i = P 'I' [(g, 2), (h, 12), (l, 14), (m, 19)]
j = P 'J' [(d, 18), (g, 8), (l, 7), (n, 4)]
k = P 'K' [(f, 20), (h, 7), (m, 17)]
l = P 'L' [(g, 16), (i, 14), (j, 7), (m, 5), (n, 7), (o, 5)]
m = P 'M' [(h, 31), (i, 19), (k, 17), (l, 5), (o, 3)]
n = P 'N' [(j, 4), (l, 7), (o, 2), (goal, 5)]
o = P 'O' [(l, 5), (m, 3), (n, 2), (goal, 3)]
goal = P '.' []

-- Given a player, return the name of the player (a single character)
pid :: Player -> Char
pid (P x ys) = x

-- A sorted-tree map from player name to the cost of getting the ball
-- to that player (an integer) and the path to get there. Note that
-- for efficiency the path is stored starting with the destination player
-- and ending with the starting player (e.g. 'A')
type Costs = Map.Map Char (Int, String)

-- Returns a minimum cost map of paths, from a given starting point
min_paths :: Player -> Costs
min_paths player = walk_min_paths init_Costs player 0 init_path
    where
        name = pid player
        init_Costs = Map.fromList [(name, (0, ""))]
        init_path = [name]

-- Updates a map of player to minimum distance and minimum path. Note that
-- we do not have to supply the starting point, as that is implicit in the
-- costs map already supplied. This function merely builds on that map.
walk_min_paths :: Costs -> Player -> Int -> String -> Costs
walk_min_paths map (P _ ys) dist path = foldr (extend_min_paths dist path) map ys

-- Extends the map if appropriate along the given edge (recursively)
-- First looks in the map to see whether the player at the end of this edge
-- can be reached more quickly some other route. Otherwise goes that way. 
extend_min_paths :: Int -> String -> (Player, Int) -> Costs -> Costs
extend_min_paths dist path (next, len) map =
    case Map.lookup name map of
        Just (min_dist, min_path) -> if min_dist < updated_dist
            then map  -- do not insert or continue walking if the existing path is shorter
            else keep_walking 
        Nothing -> keep_walking
    where
        name = pid next
        updated_dist = len + dist
        updated_path = name : path
        updated_map = Map.insert name (updated_dist, updated_path) map
        keep_walking = walk_min_paths updated_map next updated_dist updated_path
