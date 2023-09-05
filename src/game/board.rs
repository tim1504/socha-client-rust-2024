//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Board.kt

use crate::util::{Element, Error, Result};

use super::{CubeDir, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    segments: Vec<Segment>,
    next_direction: CubeDir,
}

impl TryFrom<&Element> for Board {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            segments: elem.childs_by_name("segment")
                .map(Segment::try_from)
                .collect::<Result<Vec<Segment>>>()?,
            next_direction: elem.attribute("nextDirection")?.parse()?,
        })
    }
}
