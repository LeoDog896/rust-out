use array2d::Array2D;
use anyhow::Result;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rusty,
    Clean,
}

impl Tile {
    fn flip(&self) -> Tile {
        match self {
            Tile::Rusty => Tile::Clean,
            Tile::Clean => Tile::Rusty,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Rusty => write!(f, "R"),
            Tile::Clean => write!(f, "C"),
        }
    }
}

type Grid = Array2D<Tile>;

fn new_game(num_rows: usize, num_cols: usize) -> Grid {
    let mut grid = Array2D::filled_with(Tile::Rusty, num_rows, num_cols);

    let mut rng = rand::thread_rng();

    for row in 0..num_rows {
        for col in 0..num_cols {
            // randomize the grid by "clicking" on each tile
            let tile = grid.get(row, col).unwrap().clone();

            if rng.gen_bool(0.5) {
                // click the tile by flipping it and its neighbors
                grid.set(row, col, tile.flip()).unwrap();
                if row > 0 {
                    grid.set(row - 1, col, tile.flip()).unwrap();
                }
                if row < num_rows - 1 {
                    grid.set(row + 1, col, tile.flip()).unwrap();
                }
                if col > 0 {
                    grid.set(row, col - 1, tile.flip()).unwrap();
                }
                if col < num_cols - 1 {
                    grid.set(row, col + 1, tile.flip()).unwrap();
                }

            }

        }
    }

    return grid;
}

fn print_grid(grid: &Grid) {
    for row in 0..grid.num_rows() {
        for col in 0..grid.num_columns() {
            print!("{}", grid.get(row, col).unwrap());
        }
        println!();
    }
}

fn main() -> Result<()> {
    let grid = new_game(5, 5);

    print_grid(&grid);

    Ok(())
}
