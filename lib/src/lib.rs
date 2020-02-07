use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Add;
#[derive(Debug)]
pub struct Grid<T: Debug> {
    grid: Vec<T>,
    grid_size: (usize, usize),
    offset: usize,
}

impl<T: Display + Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut output = String::from("");
        let (x, y) = self.grid_size;

        for col in (0..y).into_iter() {
            for row in (0..x).into_iter() {
                let item = self.grid.get(col * y + row).unwrap();
                output = output.add(format!("{}", item).as_str());
            }
            output = output.add("\n");
        }

        write!(f, "\n{}", output)
    }
}

impl<T: Default + Debug> Grid<T> {
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Grid<T> {
        let mut data = Vec::with_capacity(width * height);

        for _ in 0..width * height {
            data.insert(0, T::default())
        }

        Grid {
            grid: data,
            grid_size: (width, height),
            offset: y_offset * height + x_offset,
        }
    }
}

impl<T: Debug + Clone> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Result<&T, ()> {
        let (_, dim_y) = self.grid_size;
        let index = ((y * dim_y as isize + x) + self.offset as isize) as usize;
        let item = self.grid.get(index).unwrap();
        Ok(item)
    }

    pub fn set(&mut self, x: isize, y: isize, item: T) -> Result<(), ()> {
        let (_, dim_y) = self.grid_size;
        let index = ((y * dim_y as isize + x) + self.offset as isize) as usize;
        let old_item = self.grid.get_mut(index).unwrap();
        *old_item = item;
        Ok(())
    }

    pub fn grid(&self) -> Vec<T> {
        self.grid.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_simple_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 0, 0);
        assert_eq!(&false, grid.get(2, 3).unwrap());
        assert_eq!(&false, grid.get(0, 0).unwrap());
        assert_eq!(&false, grid.get(4, 4).unwrap());

        grid.set(2, 2, true).unwrap();
        assert_eq!(&true, grid.get(2, 2).unwrap());
    }

    #[test]
    fn test_offset_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 2, 2);
        assert_eq!(&false, grid.get(-1, -1).unwrap());
        assert_eq!(&false, grid.get(-2, -2).unwrap());

        grid.set(-2, -2, true).unwrap();
        assert_eq!(&true, grid.get(-2, -2).unwrap());
    }
}
