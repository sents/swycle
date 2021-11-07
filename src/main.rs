use swayipc::{Connection, Fallible};
use clap::Parser;

#[derive(Debug)]
enum Action {
    Go,
    Move,
    GoMove
}

impl std::str::FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let result = match s {
	    "go" => Action::Go,
	    "move" => Action::Move,
	    "gomove" => Action::GoMove,
	    _ => return Err("Invalid Action")
	};
	Ok(result)
    }
}


#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl std::str::FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let result = match s {
	    "left" => Direction::Left,
	    "right" => Direction::Right,
	    "up" => Direction::Up,
	    "down" => Direction::Down,
	    _ => return Err("Invalid Direction")
	};
	Ok(result)
    }
}


#[derive(Parser, Debug)]
#[clap(name = "swaycycle")]
struct Opts {
    #[clap(short, long, default_value = "3")]
    rows: u8,
    #[clap(short, long, default_value = "3")]
    columns: u8,
    #[clap()]
    action: Action,
    #[clap()]
    direction: Direction
}

fn next_workspace(num: i32, rows: i32, columns: i32 ,direction: Direction) -> i32 {
    if num == -1 || num > rows * columns {return num}
    let (dx, dy) = match direction {
	Direction::Left => (-1,0),
	Direction::Right => (1,0),
	Direction::Up => (0,-1),
	Direction::Down => (0,1)
    };
    let num = num - 1;
    let mut row = num / columns;
    let mut column = num % columns;
    row = (row + dy + rows) % rows;
    column = (column + dx + columns) % columns;
    row * columns + column + 1
}

fn main() -> Fallible<()> {
    let opts: Opts = Opts::parse();
    let rows: i32 = opts.rows.into();
    let columns: i32 = opts.columns.into();
    let mut connection = Connection::new()?;
    let workspaces = connection.get_workspaces()?;
    let num = workspaces.iter().find(|w| w.focused).unwrap().num;
    if num == -1 || num > rows*columns {return Ok(())}
    let nnum = next_workspace(num, rows, columns, opts.direction);
    match opts.action {
	Action::Go => connection.run_command(format!("workspace number {}", nnum))?,
	Action::Move => connection.run_command(format!("move container to workspace number {}", nnum))?,
	Action::GoMove => {
	    connection.run_command(format!("move container to workspace number {}", nnum))?;
	    connection.run_command(format!("workspace number {}", nnum))?
	}
    };
    Ok(())
}
