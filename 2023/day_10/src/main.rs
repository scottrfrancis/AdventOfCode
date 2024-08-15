/*** Day 10
 * 
 * Dijkstra's algorithm -- in reverse find longest
 *  
 */



// enum for direction - NEWS
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

// struct for a node with connections to NEWS
struct Node {
    connections: Vec<Direction>
}

// impl for Node
impl Node {
    fn new() -> Node {
        Node {
            connections : Vec::new(),
        }
    }

    fn connect(&mut self, dir: Direction) {
        self.connections.push(dir);
    }

    fn connects_to(&self, dir: Direction) -> bool {
        self.connections.contains(&dir)
    }
}


fn parse_map(input: &str) -> (Vec<Vec<Node>>, Option<(usize, usize)>) {
    let mut map = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    
    let mut ri = 0;
    for line in input.lines() {
        let mut row = Vec::new();

        for (ci, c) in line.trim().chars().enumerate() {
            let mut node = Node::new();

            match c {
                '.' => (),
                'F' =>  {
                    node.connect(Direction::East);
                    node.connect(Direction::South);
                },
                '-' => { 
                    node.connect(Direction::East);
                    node.connect(Direction::West);
                },
                '7' => { 
                    node.connect(Direction::South);
                    node.connect(Direction::West);
                },
                '|' => {
                    node.connect(Direction::South);
                    node.connect(Direction::North);
                },
                'L' => {
                    node.connect(Direction::North);
                    node.connect(Direction::East);
                },
                'J' => {
                    node.connect(Direction::North);
                    node.connect(Direction::West);
                },
                'S' => {
                    assert!(start.is_none());
                    start = Some((ri, ci));
                }
                _ => panic!("Invalid character"),
            }

            row.push(node);
        }

        map.push(row);
        ri += 1;
    }

    // infer start
    if let Some((r, c)) = start {
        // check NESW for connections
        // North
        if r > 0 {
            let north = &map[r - 1][c];
            if north.connects_to(Direction::South) {
                let node = &mut map[r][c];
                node.connect(Direction::North);
            }
        }

        // East
        if c < map[r].len() - 1 {
            let east = &map[r][c + 1];
            if east.connects_to(Direction::West) {
                let node = &mut map[r][c];
                node.connect(Direction::East);
            }
        }

        // South
        if r < map.len() - 1 {
            let south = &map[r+1][c];
            if south.connects_to(Direction::North) {
                let node = &mut map[r][c];
                node.connect(Direction::South);
            }
        }

        // West
        if c > 0 {
            let west = &map[r][c-1];
            if west.connects_to(Direction::East) {
                let node = &mut map[r][c];
                node.connect(Direction::West);
            }
        }
    }

    (map, start)
}

fn find_farthest(map: &Vec<Vec<Node>>, start: (usize, usize)) -> (Option<(usize, usize)>, usize) {
    let mut farthest: Option<(usize, usize)> = None;
    let mut distance = 0;

    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut queue = Vec::new();
    queue.push((start, 0));

    while !queue.is_empty() {
        let (pos, dist) = queue.remove(0);
        if visited.contains(&pos) {
            continue;
        }
        println!("Visiting {:?} with distance {}", pos, dist);
        visited.push(pos);
        let (x, y) = pos;

        if dist > distance {
            distance = dist;
            farthest = Some(pos);
        }

        let node = &map[x][y];
        for dir in &node.connections {
            match dir {
                Direction::North => {
                    if x > 0 {
                        queue.push(((x - 1, y), dist + 1));
                    }
                },
                Direction::South => {
                    if x < map.len() - 1 {
                        queue.push(((x + 1, y), dist + 1));
                    }
                },
                Direction::East => {
                    if y < map[x].len() - 1 {
                        queue.push(((x, y + 1), dist + 1));
                    }
                },
                Direction::West => {
                    if y > 0 {
                        queue.push(((x, y - 1), dist + 1));
                    }
                },
            }
        }
    }

    (farthest, distance)
}

fn blocked(map: &Vec<Vec<Node>>, start: (usize, usize), dir: Direction) -> bool {
    let mut is_blocked: Option<bool> = None;

    let (dr, dc) = match dir {
        Direction::North => (-1i32, 0),
        Direction::South => (1, 0),
        Direction::East => (0, 1),
        Direction::West => (0, -1i32),
    };
    let comp_dir = match dir {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    };

    let cross_dir = match dir {
        Direction::North => (Direction::East, Direction::West),
        Direction::South => (Direction::West, Direction::East),
        Direction::East => (Direction::South, Direction::North),
        Direction::West => (Direction::South, Direction::North),
    };
    let mut pipe_connections = Vec::new();

    let mut r = start.0 as i32;
    let mut c = start.1 as i32;
    while r >= 0 && r < map.len() as i32 && c >= 0 && c < map[r as usize ].len() as i32 && is_blocked.is_none() {
        let node = &map[r as usize][c as usize];
        for conn in &node.connections {
            if *conn == comp_dir {
                is_blocked = Some(false);
                break;
            }

            pipe_connections.push(*conn);
        }
        if pipe_connections.contains(&cross_dir.0) && pipe_connections.contains(&cross_dir.1) {
            is_blocked = Some(true);
            break;
        }

        r += dr;
        c += dc;
    }

    is_blocked.unwrap_or(false)
}

fn find_enclosed(map: &Vec<Vec<Node>>) -> usize {
    let mut enclosed: Vec<(usize, usize)> = Vec::new();

    /***
     * build a collection of '.' nodes -- nodes with no connections
     * 
     * for each node in the collection, walk the cardinal directions (NESW)
     * until you hit a complete block in that direction.
     * a 'block' is a pipe node where the direction of traveral is not a connection
     * or where two pipe nodes complement the direction
     * 
     * e.g. walking NORTH from a node, must encounter a pipe node with no connection to the SOUTH or north (E-W only)
     * OR a pipe connection to the NORTH, followed by a pipe connection to the SOUTH where east and west are ALSO blocked
     * 
     * so... walking NORTH
     *  '-' blocks and you're done
     *  '7' or 'F' means your in a pipe -- they are open to the SOUTH 
     *  'L' or 'J' means you've hit a corner and need to hit a complementary corner otherwise it's open
     *      - each of these are open to the EAST OR WEST, but not both... so there needs to be a 
     *          later pipe that is open to the complementary direction (east or west)
     *      - for 'L' you need to hit a 'J' or '7' 
     *      - for 'J' you need to hit a 'L' or 'F'
     */

    // find non-pipe nodes
    let mut non_pipes = Vec::new();
    for (r, row) in map.iter().enumerate() {
        for (c, node) in row.iter().enumerate() {
            if !node.connects_to(Direction::North) &&
                !node.connects_to(Direction::South) &&
                !node.connects_to(Direction::East) &&
                !node.connects_to(Direction::West) {
                    non_pipes.push((r, c));
                }
        }
    }

    // walk each non-pipe node to N E S W 
    let mut enclosed = Vec::new();
    for n in non_pipes {
        if blocked(map, n, Direction::North) &&
            blocked(map, n, Direction::East) &&
            blocked(map, n, Direction::South) &&
            blocked(map, n, Direction::West) {
                enclosed.push(n);
        }
    }

    enclosed.len()
}

fn main() {
    println!("Part 1");
    let input = include_str!("../input.txt");
    let (map, start) = parse_map(input);
    let (farthest, distance) = find_farthest(&map, start.unwrap());
    println!("Farthest: {:?}, Distance: {}", farthest, distance);
    
    println!("Part 2");    
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test] 
    fn test_find_enclosed() {
        let (map, start) = parse_map(INPUT4);
        assert_eq!(start, Some((1, 1)));

        let num_enclosed = find_enclosed(&map);
        assert_eq!(num_enclosed, 4);
    }


    const INPUT4: &str = 
       "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        let (map, start) = parse_map(input);
        let (farthest, distance) = find_farthest(&map, start.unwrap());
        println!("Farthest: {:?}, Distance: {}", farthest, distance);

        assert_eq!(distance, 6942);
    }

    #[test]
    fn test_farthest_complex() {
        let (map, start) = parse_map(INPUT3);
        assert_eq!(start, Some((2, 0)));

        let (farthest, distance) = find_farthest(&map, start.unwrap());
        assert_eq!(farthest, Some((2, 4)));
        assert_eq!(distance, 8);
    }

    #[test]
    fn test_find_farthest() {
        let (map, start) = parse_map(INPUT2);
        assert_eq!(start, Some((1, 1)));

        let (farthest, distance) = find_farthest(&map, start.unwrap());
        assert_eq!(farthest, Some((3, 3)));
        assert_eq!(distance, 4);
    }

    #[test]
    fn test_infer_start() {
        let (map, _) = parse_map(INPUT2);

        let n = &map[1][1];
        let conns = &n.connections;
        assert!(conns.contains(&Direction::East));
        assert!(conns.contains(&Direction::South));
        assert!(!n.connects_to(Direction::North));
        assert!(!n.connects_to(Direction::West));
    }

    #[test]
    fn test_parse_map() {
        let (map, _) = parse_map(INPUT1);
        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 5);
        assert_eq!(map[4].len(), 5);

        let n = &map[1][1];
        let conns = &n.connections;
        assert!(conns.contains(&Direction::East));
        assert!(conns.contains(&Direction::South));

        let e = &map[1][2];
        assert!(e.connects_to(Direction::East));
        assert!(e.connects_to(Direction::West));

        let s = &map[2][1];
        assert!(s.connects_to(Direction::South));
        assert!(s.connects_to(Direction::North));
    }

    const INPUT3: &str =
    "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    const INPUT2: &str =
    ".....
     .S-7.
     .|.|.
     .L-J.
     .....";

    const INPUT1: &str =
        ".....
         .F-7.
         .|.|.
         .L-J.
         .....";
    // (1,1) "F" --> E: (1,2), S: (2,1)
    // (1,2) "-" --> E: (1,3)
    // (2,1) "|" --> S: (3,1)
        
}

