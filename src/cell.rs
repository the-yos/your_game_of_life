/// The square/pixel type used for [`Life`][`super::Life`].
/// 
/// The Cell type has three fields, representing an RGB structure. There is no added functionality with this structure on its own.
/// 
/// You can also treat a Cell as either alive or dead, if you want. In that case, use the respective methods.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

impl Cell {
    /// Creates a living Cell whose RGB values are 255.
    #[inline]
    pub const fn alive() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    /// Creates a dead Cell whose RGB values are 0.
    #[inline]
    pub const fn dead() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    /// Returns false if all of the RGB values are 0.
    #[inline]
    pub const fn is_alive(self) -> bool {
        self.r != 0 || self.g != 0 || self.b != 0
    }

    /// Creates a Cell whose RGB values are all set to `rgb`.
    #[inline]
    pub const fn all(rgb: u8) -> Self {
        Self {
            r: rgb,
            g: rgb,
            b: rgb,
        }
    }

    /// Creates a Cell whose RGB values are 255.
    /// 
    /// This is identical to [`Cell::alive`].
    #[inline]
    pub const fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    /// Creates a Cell whose RGB values are 0.
    /// 
    /// This is identical to [`Cell::dead`].
    #[inline]
    pub const fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
        }
    }

    /// Creates a Cell whose R value is 255, and G and B values are 0.
    #[inline]
    pub const fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
        }
    }

    /// Creates a Cell whose G value is 255, and R and B values are 0.
    #[inline]
    pub const fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
        }
    }

    /// Creates a Cell whose B value is 255, and R and G values are 0.
    #[inline]
    pub const fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
        }
    }
}

impl From<[u8; 3]> for Cell {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

impl From<(u8, u8, u8)> for Cell {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

impl From<u8> for Cell {
    /// Returns a Cell whose RGB values are all set to `rgb`.
    fn from(rgb: u8) -> Self {
        Self {
            r: rgb,
            g: rgb,
            b: rgb,
        }
    }
}

impl From<bool> for Cell {
    /// If true, returns [`Cell::alive()`]. Otherwise, returns [`Cell::dead()`].
    fn from(alive: bool) -> Self {
        match alive {
            true => Self::alive(),
            false => Self::dead(),
        }
    }
}

macro_rules! impl_cell_neighbors {
    ($($m:ident $i:literal)*) => {
        /// Convenience trait for `[Cell; 8]`.
        /// 
        /// This trait makes indexing a `[Cell; 8]` in specific directions more readable. Also adds the [`alive`][CellNeighbors::alive] method.
        /// 
        /// # Examples
        /// 
        /// ```
        /// use your_game_of_life::*;
        /// 
        /// let cells = [Cell::alive(); 8];
        /// 
        /// let bl = cells[5];
        /// // or with convenient method:
        /// let bl = cells.bottom_left();
        /// ```
        pub trait CellNeighbors {
            /// Returns the number of neighboring alive [Cells][Cell].
            /// 
            /// Whether a [Cell] is alive is determined by the [`Cell::alive`] method.
            fn alive(self) -> u8;

            $(
                /// Returns the [Cell] in the corresponding direction.
                fn $m(self) -> Cell;
            )*
        }

        impl CellNeighbors for [Cell; 8] {
            fn alive(self) -> u8 {
                self.into_iter().filter(|cell| cell.is_alive()).count() as u8
            }

            $(
                #[doc = "Indexes the array with `["]
                #[doc = stringify!($i)]
                #[doc = "]`."]
                fn $m(self) -> Cell {
                    unsafe {
                        *self.get_unchecked($i)
                    }
                }
            )*
        }
    };
}

impl_cell_neighbors! {
    top_left 0
    top 1
    top_right 2
    left 3
    right 4
    bottom_left 5
    bottom 6
    bottom_right 7
}