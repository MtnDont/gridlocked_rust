use std::time::{Instant, SystemTime};

// Custom Random structure b/c **** using libraries outside of std
// Do it old school B)
struct Random {
    seed: u64
}
impl Random {
    fn new() -> Random {
        Random {
            seed: SystemTime::now() // SystemTime object
            .duration_since(SystemTime::UNIX_EPOCH) // Duration since UNIX time 0
            .expect("TIME HUH???") // SystemTime error, handle panic
            .as_secs()
        }
    }

    // Called like: rand_obj.rseed(seed_u64);
    fn rseed(&mut self, seed: u64) {
        if seed == 0 {
            self.seed = SystemTime::now() // SystemTime object
            .duration_since(SystemTime::UNIX_EPOCH) // Duration since UNIX time 0
            .expect("TIME HUH???") // SystemTime error, handle panic
            .as_secs();
        } else {
            self.seed = seed
        }
    }

    pub(self) fn rand(&mut self) -> u64 {
        // Algorithm used: Xorshift
        // Read more https://en.wikipedia.org/wiki/Xorshift
        let mut r: u64 = self.seed;
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;

        self.seed = r;

        // Expressions can be returned at the end of functions
        // without a semicolon, return is used for conditionals
        r
    }

    fn next(&mut self, min: u64, max: u64) -> u64 {
        self.rand() % ((max - min) + 1) + min
    }
}

fn main() {
    the_algorithm(8, 8);
}

fn visualize_array(grid: Vec<bool>, x_size: usize, y_size: usize) {
    let mut line = String::from("");
    
    for i in 0..y_size {
        for j in 0..x_size {
            if grid[i*x_size + j] {
                line.push_str("1");
            } else {
                line.push_str("0");
            }
            line.push_str(" ");
        }
        line.push_str("\n");
    }

    println!("{}", line);
}

fn the_algorithm(x: usize, y: usize) {
    // Initialize 2D Boolean array
    // multi-dimensional arrays in std are pain so
    // we're doing the single vector solution
    let mut grid = vec![false; (x+2) * (y+2)];

    // Read input from user for a seed
    println!("Please input a seed: ");
    let mut seed_line = String::new();
    // Returns number of bytes read from stdin
    std::io::stdin().read_line(&mut seed_line).unwrap();
    // Trim removes trailing newline and to_owned converts from &str to String
    let seed_int: u64 = seed_line.trim().parse().unwrap();

    let start = Instant::now();

    let mut random = Random::new();
    random.rseed(seed_int);
    println!("Seed: {}", random.seed);

    let empty_room_perc: f32 = random.next(30, 50) as f32;
    let num_rooms = (x * y) - (((x * y) as f32) * (empty_room_perc/100.0)).round() as usize;

    //Set center cell to always be a room
    let center_x = (x + 1) / 2 + (x % 2 == 0) as usize;
    let center_y = (y + 1) / 2 + (y % 2 == 0) as usize;
    grid[center_y*(x+2) + center_x] = true;

    for _ in 0..(num_rooms-1) {
        let mut change_x: usize;
        let mut change_y: usize;

        loop {
            let rand_x = random.next(1, (x-1) as u64) as usize;
            let rand_y = random.next(1, (y-1) as u64) as usize;
            let rand_dir = random.next(2, 5);

            change_x = rand_x;
            change_y = rand_y;

            match rand_dir {
                2 => change_x += 1, // Right
                3 => change_x -= 1, // Left
                4 => change_y += 1, // Down
                5 => change_y -= 1, // Up
                _ => change_x += 1 // Right
            }

            if change_x != 0 && change_y != 0 && change_x != x+1 && change_y != y+1 && // Make sure chosen block isnt at an edge
                grid[rand_y*(x+2) + rand_x] && // Make sure block exists
                !grid[change_y*(x+2) + change_x] // Make sure block is disabled
            {
                break;
            }
        }

        grid[change_y*(x+2) + change_x] = true;
    }
    visualize_array(grid, x+2, y+2);

    let duration = start.elapsed();
    println!("Time to Execute: {:?}", duration);
}