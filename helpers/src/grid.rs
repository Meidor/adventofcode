use std::fmt::Display;

use glam::{ivec2, IVec2};

static DIRECTIONS_4: [IVec2; 4] = [
    //LEFT
    ivec2(-1, 0),
    //RIGHT
    ivec2(1, 0),
    //DOWN
    ivec2(0, -1),
    //UP
    ivec2(0, 1),
];

static DIRECTIONS_8: [IVec2; 8] = [
    //LEFT
    ivec2(-1, 0),
    //RIGHT
    ivec2(1, 0),
    //DOWN
    ivec2(0, -1),
    //UP
    ivec2(0, 1),
    //UP LEFT
    ivec2(-1, 1),
    //UP RIGHT
    ivec2(1, 1),
    //DOWN LEFT
    ivec2(-1, -1),
    //DOWN RIGHT
    ivec2(1, -1),
];

pub struct Neighborhood {
    pub north: Option<IVec2>,
    pub east: Option<IVec2>,
    pub south: Option<IVec2>,
    pub west: Option<IVec2>,
    pub north_east: Option<IVec2>,
    pub north_west: Option<IVec2>,
    pub south_east: Option<IVec2>,
    pub south_west: Option<IVec2>,
}

impl Neighborhood {
    pub fn exising_neighbors(&self, include_diagonal: bool) -> Vec<IVec2> {
        let mut result = vec![];
        if let Some(north) = self.north {
            result.push(north);
        }
        if let Some(east) = self.east {
            result.push(east);
        }
        if let Some(south) = self.south {
            result.push(south);
        }
        if let Some(west) = self.west {
            result.push(west);
        }

        if !include_diagonal {
            return result;
        }

        if let Some(north_east) = self.north_east {
            result.push(north_east);
        }
        if let Some(north_west) = self.north_west {
            result.push(north_west);
        }
        if let Some(south_east) = self.south_east {
            result.push(south_east);
        }
        if let Some(south_west) = self.south_west {
            result.push(south_west);
        }
        result
    }
}

pub trait FilterGrid<T: std::marker::Copy = Self>: Grid<T> {
    fn filter_positions(&self, predicate: impl Fn(&T) -> bool) -> Vec<IVec2> {
        let mut result: Vec<IVec2> = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = ivec2(x as i32, y as i32);
                let item = self.get_position(pos);
                if predicate(item) {
                    result.push(pos);
                }
            }
        }
        result
    }
}

pub trait Grid<T: Copy> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn values(&self) -> &[T];
    fn values_mut(&mut self) -> &mut [T];

    fn get_index(&self, pos: IVec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * self.width()
    }

    fn try_get_index(&self, pos: IVec2) -> Option<usize> {
        if self.has_position(pos) {
            Some(self.get_index(pos))
        } else {
            None
        }
    }

    fn has_position(&self, pos: IVec2) -> bool {
        !(pos.x < 0 || pos.x >= self.width() as i32 || pos.y < 0 || pos.y >= self.height() as i32)
    }

    fn get_position(&self, pos: IVec2) -> &T {
        &self.values()[self.get_index(pos)]
    }

    fn set_position(&mut self, pos: IVec2, value: T) {
        let index = self.get_index(pos);
        self.values_mut()[index] = value;
    }

    fn get_row(&self, row: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let y = row;
        for x in 0..self.width() {
            result.push(*self.get_position(ivec2(x as i32, y as i32)));
        }
        result
    }

    fn get_column(&self, column: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        let x = column;
        for y in 0..self.height() {
            result.push(*self.get_position(ivec2(x as i32, y as i32)));
        }
        result
    }

    fn try_get_position(&self, pos: IVec2) -> Option<&T> {
        if self.has_position(pos) {
            Some(self.get_position(pos))
        } else {
            None
        }
    }

    fn get_neighbours(&self, pos: IVec2, include_diagonal: bool) -> Vec<IVec2> {
        match include_diagonal {
            true => DIRECTIONS_8
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
            false => DIRECTIONS_4
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
        }
    }

    fn get_neighborhood(&self, pos: IVec2) -> Neighborhood {
        Neighborhood {
            north: self
                .has_position(pos + ivec2(0, -1))
                .then(|| pos + ivec2(0, -1)),
            east: self
                .has_position(pos + ivec2(1, 0))
                .then(|| pos + ivec2(1, 0)),
            south: self
                .has_position(pos + ivec2(0, 1))
                .then(|| pos + ivec2(0, 1)),
            west: self
                .has_position(pos + ivec2(-1, 0))
                .then(|| pos + ivec2(-1, 0)),
            north_east: self
                .has_position(pos + ivec2(1, -1))
                .then(|| pos + ivec2(1, -1)),
            north_west: self
                .has_position(pos + ivec2(-1, -1))
                .then(|| pos + ivec2(-1, -1)),
            south_east: self
                .has_position(pos + ivec2(1, 1))
                .then(|| pos + ivec2(1, 1)),
            south_west: self
                .has_position(pos + ivec2(-1, 1))
                .then(|| pos + ivec2(-1, 1)),
        }
    }
}

impl<T: std::fmt::Display + std::marker::Copy> std::fmt::Debug for dyn Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = ivec2(x as i32, y as i32);
                let item = self.get_position(pos);
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Display + std::marker::Copy> Display for dyn Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = ivec2(x as i32, y as i32);
                let item = self.get_position(pos);
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
