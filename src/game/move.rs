//! Ported from https://github.com/software-challenge/backend/blob/be88340f619892fe70c4cbd45e131d5445e883c7/plugin/src/main/kotlin/sc/plugin2024/Field.kt

use crate::util::{Error, Element, Result, Perform};

use super::Action;

/// A game move.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub actions: Vec<Action>,
}

impl Default for Move {
    fn default() -> Self {
        Self::new()
    }
}

impl Move {
    /// Creates an empty move.
    pub fn new() -> Self {
        Move { actions: Vec::new() }
    }

    /// The last action within the move.
    pub fn last(&self) -> Option<&Action> {
        self.actions.last()
    }

    /// An iterator over the actions within this move.
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }

    /// A mutable iterator over the actions within this move.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Action> {
        self.actions.iter_mut()
    }

    /// Whether the move contains no actions.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}

impl<T> Perform<Move> for T where T: Perform<Action> {
    type Output = ();

    fn perform(&mut self, m: Move) {
        for action in m.actions {
            self.perform(action);
        }
    }
}

impl From<Action> for Move {
    fn from(action: Action) -> Self {
        Self { actions: vec![action] }
    }
}

impl Perform<Action> for Move {
    type Output = ();

    fn perform(&mut self, action: Action) {
        self.actions.push(action);
    }
}

impl IntoIterator for Move {
    type Item = Action;
    type IntoIter = <Vec<Action> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

impl TryFrom<&Element> for Move {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(Self {
            actions: elem.child_by_name("actions")?
                .childs()
                .map(Action::try_from)
                .collect::<Result<Vec<_>>>()?
        })
    }
}

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        Element::new("data")
            .attribute("class", "move")
            .child(Element::new("actions")
                .childs(m.actions.into_iter().map(Element::from)))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{util::{assert_xml_format, assert_xml_roundtrip}, game::{Move, CubeDir, Action}};

    #[test]
    fn test_xml_formats() {
        assert_xml_format!(
            Move {
                actions: vec![
                    Action::accelerate(-1),
                    Action::turn(CubeDir::DownRight),
                    Action::advance(2),
                ]
            },
            indoc! {r#"
                <data class="move">
                    <actions>
                        <acceleration acc="-1"/>
                        <turn direction="DOWN_RIGHT"/>
                        <advance distance="2"/>
                    </actions>
                </data>
            "#}
        );
    }

    #[test]
    fn test_xml_roundtrips() {
        assert_xml_roundtrip!(Move {
            actions: vec![
                Action::advance(1),
                Action::turn(CubeDir::DownRight),
                Action::turn(CubeDir::Left),
            ]
        });
    }
}
