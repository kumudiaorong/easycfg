use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, strum_macros::Display, Serialize, Deserialize, PartialEq)]
pub enum Distri {
    #[default]
    Unknown,
    Arch,
    OpenSUSE,
}
