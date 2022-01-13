pub mod generic {
    use num::{CheckedAdd, CheckedSub, Signed, Unsigned};
    use std::cmp::PartialOrd;
    use std::fmt::Debug;
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
        pub fn index(&self, width: T) -> T {
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

    #[derive(Clone, Copy)]
    pub struct SignedDeltaPos<T: Clone + Copy + Signed> {
        row: T,
        column: T,
    }

    impl<T: Clone + Copy + Signed> SignedDeltaPos<T> {
        pub fn new(row: T, column: T) -> Self {
            SignedDeltaPos { row, column }
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
        T: Clone + Copy + Unsigned + CheckedSub + CheckedAdd + PartialOrd,
    {
        pub fn checked_apply<S>(
            &self,
            width: T,
            height: T,
            delta: &SignedDeltaPos<S>,
        ) -> Option<Self>
        where
            S: Clone + Copy + Signed,
            T: TryFrom<S>,
            <T as TryFrom<S>>::Error: Debug,
        {
            let checked_delta_add = |val: T, delta: S| -> Option<T> {
                let delta_abs = T::try_from(delta.abs()).unwrap();
                if delta.is_negative() {
                    val.checked_sub(&delta_abs)
                } else {
                    Some(val.checked_add(&delta_abs).unwrap())
                }
            };

            let row = checked_delta_add(self.row(), delta.row())?;
            let column = checked_delta_add(self.column(), delta.column())?;

            if (row >= height) || (column >= width) {
                None
            } else {
                Some(Position::new(row, column))
            }
        }
    }
}

pub type Position = generic::Position<usize>;
pub type DeltaPos = generic::DeltaPos<usize>;
pub type SignedDeltaPos = generic::SignedDeltaPos<i32>;
