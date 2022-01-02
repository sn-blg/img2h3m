use num::{Unsigned, CheckedSub};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy)]
pub struct Position<T: Clone + Copy + Unsigned> {
    row: T,
    column: T,
}

impl<T: Clone + Copy + Unsigned> Position<T> {
    pub fn new(row: T, column: T) -> Self {
        Position { row, column }
    }

    pub fn row(&self) -> T {
        self.row
    }

    pub fn column(&self) -> T {
        self.column
    }
}

impl<T> Position<T>
where
    T: Clone + Copy + Unsigned + Div<Output = T> + Rem<Output = T>,
{
    pub fn from_index(width: T, index: T) -> Self {
        Position::new(index / width, index % width)
    }
}

impl<T> Position<T>
where
    T: Clone + Copy + Unsigned + Mul<Output = T> + Add<Output = T>,
{
    pub fn to_index(&self, width: T) -> T {
        self.row * width + self.column
    }
}

#[derive(Clone, Copy)]
pub struct DeltaPos<T: Clone + Copy + Unsigned> {
    row: T,
    column: T,
}

impl<T: Clone + Copy + Unsigned> DeltaPos<T> {
    pub fn new(row: T, column: T) -> Self {
        DeltaPos { row, column }
    }

    pub fn row(&self) -> T {
        self.row
    }

    pub fn column(&self) -> T {
        self.column
    }
}

impl<T: Clone + Copy + Unsigned + CheckedSub> Position<T> {
    pub fn checked_sub(&self, delta: &DeltaPos<T>) -> Option<Self> {
        let row = self.row().checked_sub(&delta.row());
        let column = self.column().checked_sub(&delta.column());
        match (row, column) {
            (Some(row), Some(column)) => Some(Position::new(row, column)),
            _ => None,
        }
    }
}

impl<T> Position<T>
where
    T: Clone + Copy + Unsigned + Sub<Output = T>,
{
    pub fn sub(&self, delta: &DeltaPos<T>) -> Self {
        let row = self.row() - delta.row();
        let column = self.column() - delta.column();
        Position::new(row, column)
    }
}
