fn main() {
    let result = which_way(Direction::Down);
    let color_result = which_color(Colors::Purple);
    println!("{:?}", result);
    println!("{:?}", color_result)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

enum Colors {
    Blue,
    Yellow,
    Pink,
    Purple,
}

fn which_color(color: Colors) -> &'static str {
    match color {
        Colors::Blue => "blue",
        Colors::Yellow => "yellow",
        Colors::Pink => "pink",
        Colors::Purple => "purple",
    }
}
