use std::default::Default;
use std::fmt;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut grid = Grid::new(6, 7);
    let mut player = Tile::X;

    grid.print();

    loop {
        println!("Player {}'s turn. Which column? ", player);
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let column = match usize::from_str(&input[..].trim()) {
                    Ok(column) => {
                        if column != 0 {
                            column - 1
                        } else {
                            println!("That isn't a valid column!");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("That isn't a valid column!");
                        continue;
                    }
                };

                if grid.place_tile(player, column).is_err() {
                    println!("That isn't a valid column!");
                    continue;
                }

                grid.print();

                match grid.winner() {
                    Some(winner) => {
                        println!("Player {} wins!", winner);
                        break;
                    },
                    None => {},
                }

                player = match player {
                    Tile::X => Tile::O,
                    _ => Tile::X,
                };
            },
            Err(_) => {
                println!("Input required.");
                continue;
            }
        }
    }
}

struct Grid {
    column_count: usize,
    row_count: usize,
    rows: Vec<Row>,
}

impl Grid {
    pub fn new(row_count: usize, column_count: usize) -> Grid {
        Grid {
            column_count: column_count,
            row_count: row_count,
            rows: (0..row_count).map(|_| Row::new(column_count)).collect(),
        }
    }

    pub fn place_tile(&mut self, player: Tile, column: usize) -> Result<(), ()> {
        let error = Err(());

        for row in self.rows.iter_mut().rev() {
            if row.place_tile(player, column).is_ok() {
                return Ok(());
            }
        }

        error
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn winner(&self) -> Option<Tile> {
        match self.horizontal_winner() {
            Some(tile) => Some(tile),
            None => match self.vertical_winner() {
                Some(tile) => Some(tile),
                None => match self.descending_diagonal_winner() {
                    Some(tile) => Some(tile),
                    None => match self.ascending_diagonal_winner() {
                        Some(tile) => Some(tile),
                        None => None,
                    }
                }
            }
        }
    }

    fn horizontal_winner(&self) -> Option<Tile> {
        let mut count = 1;
        let mut last_tile = Tile::Empty;

        for row in self.rows.iter() {
            for tile in row.tiles.iter() {
                if tile.is_player() && *tile == last_tile {
                    count += 1;

                    if count == 4 {
                        return Some(*tile)
                    }
                } else {
                    count = 1;
                    last_tile = *tile;
                }
            }
        }

        None
    }

    fn vertical_winner(&self) -> Option<Tile> {
        let mut count = 1;
        let mut last_tile = Tile::Empty;

        for col in (0..self.column_count) {
            for row in self.rows.iter() {
                let tile = row.tiles[col];

                if tile.is_player() && tile == last_tile {
                    count += 1;

                    if count == 4 {
                        return Some(tile)
                    }
                } else {
                    count = 1;
                    last_tile = tile;
                }
            }
        }

        None
    }

    fn descending_diagonal_winner(&self) -> Option<Tile> {
        for (col_num, _) in (0..(self.column_count - 3)).enumerate() {
            for (row_num, _) in (0..(self.row_count - 3)).enumerate() {
                let mut count = 1;
                let mut last_tile = self.rows[row_num].tiles[col_num];

                if !last_tile.is_player() {
                    continue;
                }

                for offset in (1..4) {
                    let tile = self.rows[row_num + offset].tiles[col_num + offset];

                    if tile.is_player() && tile == last_tile {
                        count += 1;

                        if count == 4 {
                            return Some(tile);
                        }
                    } else {
                        count = 1;
                        last_tile = tile;
                    }
                }
            }
        }

        None
    }

    fn ascending_diagonal_winner(&self) -> Option<Tile> {
        for (col_num, _) in (0..(self.column_count - 3)).enumerate() {
            for (index, _) in ((self.row_count - 3)..self.row_count).enumerate() {
                let row_num = self.row_count - 3 + index;
                let mut count = 1;
                let mut last_tile = self.rows[row_num].tiles[col_num];

                if !last_tile.is_player() {
                    continue;
                }

                for offset in (1..4) {
                    let tile = self.rows[row_num - offset].tiles[col_num + offset];

                    if tile.is_player() && tile == last_tile {
                        count += 1;

                        if count == 4 {
                            return Some(tile);
                        }
                    } else {
                        count = 1;
                        last_tile = tile;
                    }
                }
            }
        }

        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for n in 0..self.rows[0].len() {
            try!(write!(f, "{}  ", n + 1));
        }

        try!(writeln!(f, ""));

        let mut result = Ok(());

        for row in self.rows.iter() {
            result = writeln!(f, "{}", row);
        }

        result
    }
}

struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    pub fn new(column_count: usize) -> Row {
        Row {
            tiles: (0..column_count).map(|_| Tile::Empty).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.tiles.len() as usize
    }

    pub fn place_tile(&mut self, player: Tile, column: usize) -> Result<(), ()> {
        if column > self.tiles.len() - 1 {
            return Err(());
        }

        match self.tiles[column] {
            Tile::Empty => {
                self.tiles[column] = player;
                return Ok(());
            },
            _ => return Err(()),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut result = Ok(());

        for tile in self.tiles.iter() {
            result = write!(f, "{}  ", tile);
        }

        result
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    O,
    X,
}

impl Tile {
    pub fn is_player(&self) -> bool {
        match *self {
            Tile::Empty => false,
            _ => true,
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Empty
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let value = match *self {
            Tile::Empty => ".",
            Tile::O => "O",
            Tile::X => "X",
        };

        write!(f, "{}", value)
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}
