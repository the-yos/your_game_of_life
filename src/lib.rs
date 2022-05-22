//! Conway's Game of Life but customized.
//! 
//! This is a small and simple crate. The crate is meant to mimic Conway's Game of Life but with customization. You can edit the rules using the [`play`] or [`play_for`] by providing your own closure. Also, the [Cells][Cell] (squares/pixels) are 8-bit RGB values, which means you can make a colored game of life.
//! 
//! Note that this crate only provides functionality for 2D matrix manipulation designed for Game of Life. It does not provide functionality for drawing the game.
//! 
//! # Examples
//! 
//! This is how you would use the normal rules of Conway's Game of Life. First, we create a life defining how big it is and which cells are alive and/or which color they are. (You can define them as RGB instead by replacing `ALIVE` or `DEAD` with `Cell {r: 255, g: 255, b: 255 }`.) Then we use the [`play_for`] method to apply our closure 2 times.
//! ```
//! const ALIVE: Cell = Cell::alive();
//! const DEAD: Cell = Cell::dead();
//! 
//! fn main() {
//!     let mut life = Life::from([
//!         [ALIVE, DEAD,  ALIVE],
//!         [DEAD,  ALIVE, DEAD],
//!         [ALIVE, DEAD,  ALIVE],
//!     ]);
//!
//!     life.play_for(2, |same, others, _, _| {
//!         let alive = others.alive();
//!
//!         if alive < 2 {
//!             DEAD
//!         }
//!         else if alive == 2 {
//!             same
//!         }
//!         else if alive == 3 {
//!             ALIVE
//!         }
//!         else {
//!             DEAD
//!         }
//!     });
//! }
//! ```
//! 
//! [`play`]: Life::play
//! [`play_for`]: Life::play_for

mod cell;
pub use cell::*;

/// 2D array of [Cells].
/// 
/// This is the base of the game where you manage the [Cells] and provide the closures for running it.
/// 
/// [Cells]: Cell
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Life<const HEIGHT: usize, const WIDTH: usize> {
    /// The [Cells][Cell] that live in this Life.
    pub cells: [[Cell; HEIGHT]; WIDTH],
    /// The [Cell] that is added in the `[Cell; 8]` array on invoking the closure for [`play`][Life::play] and [`play_for`][Life::play_for] when the neighboring [Cell] would have been out of bounds.
    pub out_of_bounds: Cell,
}

impl<const HEIGHT: usize, const WIDTH: usize> Default for Life<HEIGHT, WIDTH> {
    fn default() -> Self {
        Self {
            cells: [[Cell::default(); HEIGHT]; WIDTH],
            out_of_bounds: Cell::default(),
        }
    }
}

/// Converts any 2D container of [Cells][Cell] into a Life, for example a [`Vec<Vec<bool>>`].
/// 
/// # Examples
/// 
/// ```
/// # use your_game_of_life::Life;
/// let life = Life::<2, 2>::from(vec![vec![true, false], vec![false, true]]);
/// ```
impl<T, const HEIGHT: usize, const WIDTH: usize> From<T> for Life<HEIGHT, WIDTH>
where T: IntoIterator,
Self: FromIterator<T::Item> {
    fn from(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

/// Converts any 2D container of [Cells][Cell] into a Life, for example a [`Vec<Vec<bool>>`].
impl<A, const HEIGHT: usize, const WIDTH: usize> FromIterator<A> for Life<HEIGHT, WIDTH>
where A: IntoIterator,
A::Item: Into<Cell> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut cells = [[Cell::default(); HEIGHT]; WIDTH];

        for (y, row) in iter.into_iter().enumerate().take(WIDTH) {
            for (x, cell) in row.into_iter().enumerate().take(HEIGHT) {
                unsafe {
                    *cells.get_unchecked_mut(y).get_unchecked_mut(x) = cell.into();
                }
            }
        }

        Self {
            cells,
            out_of_bounds: Cell::default(),
        }
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> Life<HEIGHT, WIDTH> {
    /// Returns the [Cell] at the given index.
    /// 
    /// # Panics
    /// 
    /// Panics if either the `x` or `y` index is out of bounds.
    #[track_caller]
    #[inline]
    pub fn get(self, x: usize, y: usize) -> Cell {
        self.cells[y][x]
    }
    
    /// Returns an immutable reference to the [Cell] at the given index.
    /// 
    /// # Panics
    /// 
    /// Panics if either the `x` or `y` index is out of bounds.
    #[track_caller]
    #[inline]
    pub fn get_ref(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    /// Returns a mutable reference to the [Cell] at the given index.
    /// 
    /// # Panics
    /// 
    /// Panics if either the `x` or `y` index is out of bounds.
    #[track_caller]
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    /// Sets the [Cell] at the given index.
    /// 
    /// # Panics
    /// 
    /// Panics if either the `x` or `y` index is out of bounds.
    #[track_caller]
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    unsafe fn get_surrounding(self, x: usize, y: usize) -> [Cell; 8] {
        let (mut tl, mut t, mut tr, mut l, mut r, mut bl, mut b, mut br) = (Cell::default(), Cell::default(), Cell::default(), Cell::default(), Cell::default(), Cell::default(), Cell::default(), Cell::default());

        macro_rules! insert {
            ($var:ident, $xop:tt $xoff:literal, $yop:tt $yoff:literal) => {
                $var = *self.cells.get_unchecked(x $xop $xoff).get_unchecked(y $yop $yoff)
            }
        }

        if x != 0 {
            insert!(l, -1, +0);

            if y != 0 {
                insert!(tl, -1, -1);
            }
            if y != HEIGHT - 1 {
                insert!(bl, -1, +1);
            }
        }
        if y != 0 {
            insert!(t, +0, -1);

            if x != WIDTH - 1 {
                insert!(tr, +1, -1);
            }
        }
        if x != WIDTH - 1 {
            insert!(r, +1, +0);

            if y != HEIGHT - 1 {
                insert!(br, +1, +1);
            }
        }
        if y != WIDTH - 1 {
            insert!(b, +0, +1);
        }

        [tl, t, tr, l, r, bl, b, br]
    }

    /// Invokes the given closure on each [Cell] in the Life.
    /// 
    /// The parameters for the closure are, in order:
    /// * The Cell itself
    /// * The Cells surrounding the Cell itself
    /// * The x-position
    /// * The y-position
    /// 
    /// If a neighboring [Cell] would have been out of bounds, it's instead replaced by the [`out_of_bounds`] [Cell] in the `[Cell; 8]` array.
    /// 
    /// You can make use of the [CellNeighbors] trait for indexing the surrounding Cells with readability.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut life = Life::<3, 2>::from([
    ///     [1, 0]
    ///     [0, 1]
    ///     [1, 1]
    /// ]);
    /// 
    /// life.play(|this, others, x, y| {
    ///     println!("The cell at position ({x}, {y}) has {} living neighbors", others.alive());
    ///     
    ///     // return the same cell to keep it unchanged
    ///     this
    /// });
    /// ```
    /// 
    /// [`out_of_bounds`]: struct.Life.html#structfield.out_of_bounds
    pub fn play(&mut self, mut f: impl FnMut(Cell, [Cell; 8], usize, usize) -> Cell) {
        let mut proto = self.cells;

        for (x, column) in self.cells.into_iter().enumerate() {
            for (y, cell) in column.into_iter().enumerate() {
                unsafe {
                    *proto.get_unchecked_mut(x).get_unchecked_mut(y) = f(cell, self.get_surrounding(x, y), x, y);
                }
            }
        }

        self.cells = proto;
    }

    /// Invokes the given closure `n` times on each [Cell] in the Life.
    /// 
    /// The parameters for the closure are, in order:
    /// * The Cell itself
    /// * The Cells surrounding the Cell itself
    /// * The x-position
    /// * The y-position
    /// 
    /// If a neighboring [Cell] would have been out of bounds, it's instead replaced by the [`out_of_bounds`] [Cell] in the `[Cell; 8]` array.
    /// 
    /// You can make use of the [CellNeighbors] trait for indexing the surrounding Cells with readability.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut life = Life::<3, 2>::from([
    ///     [1, 0]
    ///     [0, 1]
    ///     [1, 1]
    /// ]);
    /// 
    /// life.play_for(2, |_this, others, x, y| {
    ///     println!("The cell at position ({x}, {y}) has {} living neighbors", others.alive());
    ///     
    ///     // return a living cell to make the cell alive regardless of its previous state
    ///     Cell::alive()
    /// });
    /// ```
    /// 
    /// [`out_of_bounds`]: struct.Life.html#structfield.out_of_bounds
    #[inline]
    pub fn play_for(&mut self, n: u32, mut f: impl FnMut(Cell, [Cell; 8], usize, usize) -> Cell) {
        for _ in 0..n {
            self.play(&mut f);
        }
    }
}