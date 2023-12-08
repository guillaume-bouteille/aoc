
import os
import sys

TEMPLATES = [
    ("data/%04d/day%02d/input.txt", ""),
    ("data/%04d/day%02d/ref.txt",
"""P1=TODO
P2=TODO
"""),
    ("%04d/day%02d/main.rs",
"""

use std::io;

type Inputs = i64;

fn parse_inputs() -> Inputs {
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
        }
    }
    0
}

fn main() {


    let p1 : i64 = 0;
    let p2 : i64 = 0;

    println!("P1={}", p1);
    println!("P2={}", p2);
}
""")
]

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage {sys.argv[0]} <YEAR> <DAY>")
        exit(1)
    year_str, day_str = sys.argv[1:]
    year = int(year_str)
    day = int(day_str)
    print(f"Creating files for year {year} day {day}")

    files_to_create = list(map(
        lambda x: (os.path.abspath(os.path.join(os.path.dirname(__file__), "..",
            x[0] %(year, day))), x[1]),
        TEMPLATES))
    for file_path, content in files_to_create:
        print(f"-> {file_path}")
        if not os.path.exists( os.path.dirname(file_path) ):
            os.makedirs( os.path.dirname(file_path) )
        assert not os.path.exists( file_path )
        with open(file_path, "w") as f:
            f.write(content)
