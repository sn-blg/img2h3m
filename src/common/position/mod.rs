pub mod generic;

pub type Position = generic::Position<usize>;
pub type DeltaPos = generic::DeltaPos<usize>;
pub type SignedDeltaPos = generic::SignedDeltaPos<isize>;

impl Position {
    pub fn sub_position(&self, position: &Position) -> SignedDeltaPos {
        let sub = |a: usize, b: usize| -> isize {
            if a >= b {
                (a - b).try_into().unwrap()
            } else {
                let tmp: isize = (b - a).try_into().unwrap();
                -tmp
            }
        };

        let row = sub(self.row(), position.row());
        let column = sub(self.column(), position.column());

        SignedDeltaPos::new(row, column)
    }
}
