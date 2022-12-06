struct Section {
    lower: i32,
    upper: i32,
}

impl From<&str> for Section {
    fn from(input: &str) -> Self {
        let mut parts = input.split('-');

        Section {
            lower: parts
                .next()
                .expect("invalid input: mising lower section bound")
                .parse()
                .expect("invalid input: lower section bound not a number"),
            upper: parts
                .next()
                .expect("invalid input: mising upper section bound")
                .parse()
                .expect("invalid input: upper section bound not a number"),
        }
    }
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Section) -> bool {
        (self.lower <= other.lower && self.upper >= other.lower)
            || (self.lower <= other.upper && self.upper >= other.upper)
    }
}

fn solve(input: &str) -> (u32, u32) {
    let lines = input.lines();

    let mut contained_count = 0;
    let mut overlapped_count = 0;

    for line in lines {
        let pair: Vec<Section> = line
            .split(',')
            .take(2)
            .map(|part| Section::from(part))
            .collect();

        if pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) {
            contained_count += 1;
            overlapped_count += 1;
            continue;
        }

        if pair[0].overlaps(&pair[1]) {
            overlapped_count += 1;
        }
    }

    (contained_count, overlapped_count)
}

fn main() {
    let input = include_str!("../input.txt");
    let (contained, overlapped) = solve(&input);
    println!("fully contained: {}", contained);
    println!("overlapped: {}", overlapped);
}
