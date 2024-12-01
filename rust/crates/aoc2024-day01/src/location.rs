#[derive(Debug)]
pub struct Location {
    id: i32,
}

impl Location {
    fn new(raw_id: &str) -> Self {
        let id = raw_id.parse().expect("input should parse to integer");
        Location { id }
    }
}

#[derive(Debug)]
pub struct Locations {
    left: Vec<Location>,
    right: Vec<Location>,
}

impl Locations {
    fn new() -> Self {
        Locations {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mut locations = Locations::new();
        let lines = input.trim().split("\n");
        for line in lines {
            locations.add_line(line)
        }
        locations.left.sort_by_key(|l| l.id);
        locations.right.sort_by_key(|l| l.id);
        locations
    }

    fn add_line(&mut self, line: &str) {
        let mut split = line.split_whitespace();
        let left = Location::new(split.next().expect("should have left value"));
        let right = Location::new(split.next().expect("should have right value"));
        self.left.push(left);
        self.right.push(right);
    }

    pub fn calculate_distance(self) -> i32 {
        let mut calc = 0;
        let location_pairs = self.left.iter().zip(self.right.iter());
        for pair in location_pairs {
            let distance = pair.0.id - pair.1.id;
            calc += distance.abs();
        }
        calc
    }

    pub fn calculate_similarity(self) -> i32 {
        let mut calc = 0;
        for location in self.left.iter() {
            let mut count = 0;
            for x in &self.right {
                if x.id == location.id {
                    count += 1
                }
            }
            calc += location.id * count
        }
        calc
    }
}
