// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl From<bool> for Direction {
    fn from(i: bool) -> Self {
        if i {
            Self::Left
        } else {
            Self::Right
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<bool> for Direction {
    fn into(self) -> bool {
        match self {
            Direction::Left => false,
            Direction::Right => true,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u8> for Direction {
    fn into(self) -> u8 {
        self.map(0, 1)
    }
}

impl TryFrom<u8> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Left),
            1 => Ok(Direction::Right),
            _ => Err(anyhow!("Cannot convert integer {} into a direction", value)),
        }
    }
}

impl Direction {
    pub fn map<T>(&self, left: T, right: T) -> T {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }

    pub fn flip(&self) -> Self {
        self.map(Self::Right, Self::Left)
    }

    pub fn of_bool(other: bool) -> Self {
        Self::from(other)
    }

    pub fn to_bool(&self) -> bool {
        (*self).into()
    }

    pub fn to_int(&self) -> u8 {
        (*self).into()
    }

    pub fn of_int(other: u8) -> Option<Direction> {
        Direction::try_from(other).ok()
    }

    // pub fn of_int_exn
    // pub fn gen
    // pub fn gen_var_length_list
    // pub fn gen_list
    // pub fn shrinker
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
