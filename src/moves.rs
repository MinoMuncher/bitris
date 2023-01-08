use std::rc::Rc;

use crate::{BlPlacement, Board64, RotationSystem};
use crate::internal_moves::moves64;
use crate::srs::SrsKickTable;

/// A collection of piece drop types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Drop {
    #[default] Softdrop,
    Harddrop,
}

/// Rules to be applied during move generation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MoveRules<T> where T: RotationSystem {
    pub rotation_system: Rc<T>,
    pub drop: Drop,
}

impl MoveRules<SrsKickTable> {
    #[inline]
    pub fn default() -> Self {
        Self::srs(Drop::Softdrop)
    }

    #[inline]
    pub fn srs(drop: Drop) -> Self {
        Self { rotation_system: Rc::new(SrsKickTable), drop }
    }
}

impl<T> MoveRules<T> where T: RotationSystem {
    #[inline]
    pub fn new(rotation_system: Rc<T>, drop: Drop) -> Self {
        Self { rotation_system, drop }
    }

    /// Collect all the places that can be placed in the rotation system.
    /// If the placements have the same block positions, but the orientations are different, each will be collected.
    /// `P` allows you to specify the type of placement to output.
    pub fn generate_all_moves(&self, board: impl Into<Board64>, spawn: impl Into<BlPlacement>) -> Vec<BlPlacement> {
        match self.drop {
            Drop::Softdrop => {
                let result = moves64::all_moves_softdrop(self.rotation_system.as_ref(), &board.into(), spawn.into());
                result.vec()
            }
            Drop::Harddrop => {
                let result = moves64::all_moves_harddrop(self.rotation_system.as_ref(), &board.into(), spawn.into());
                result.vec()
            }
        }
    }

    /// Collect all the places that can be placed in the rotation system.
    /// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
    /// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
    /// `P` allows you to specify the type of placement to output.
    pub fn generate_minimized_moves(&self, board: impl Into<Board64>, spawn: impl Into<BlPlacement>) -> Vec<BlPlacement> {
        match self.drop {
            Drop::Softdrop => {
                let result = moves64::minimized_moves_softdrop(self.rotation_system.as_ref(), &board.into(), spawn.into());
                result.vec()
            }
            Drop::Harddrop => {
                let result = moves64::minimized_moves_harddrop(self.rotation_system.as_ref(), &board.into(), spawn.into());
                result.vec()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;
    use crate::moves::MoveRules;

    #[test]
    fn generate_all_moves() {
        let board = Board64::from_str(" \
            ..XXXXXX..\
            ..........\
            ..........\
            ..........\
        ").unwrap();
        let rules = MoveRules::srs(Drop::Harddrop);
        let placement = piece!(SN).with(bl(4, 20));
        let moves = rules.generate_all_moves(board, placement);
        assert_eq!(moves.len(), 34);
        assert_eq!(moves.iter().filter(|it| it.position.by == 0).count(), 4);
    }

    #[test]
    fn generate_minimized_moves() {
        let board = Board64::from_str(" \
            ..XXXXXX..\
            ..........\
            ..........\
            ..........\
        ").unwrap();
        let rules = MoveRules::srs(Drop::Harddrop);
        let placement = piece!(SN).with(bl(4, 20));
        let moves = rules.generate_minimized_moves(board, placement);
        assert_eq!(moves.len(), 17);
        assert_eq!(moves.iter().filter(|it| it.position.by == 0).count(), 2);
    }
}
