use std::fmt;

use itertools::Itertools;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    width: i32,
    height: i32,
    storage: Vec<T>,
}

impl From<&str> for Grid<char> {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let width = lines.iter().map(|l| l.len()).max().unwrap();
        let height = lines.len();

        let mut storage = Vec::new();
        storage.resize(width * height, ' ');

        for (y, l) in lines.iter().enumerate() {
            let chars: Vec<_> = l.chars().collect();
            storage[y * width..y * width + chars.len()].copy_from_slice(&chars);
        }

        Grid {
            width: width as i32,
            height: height as i32,
            storage,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: Clone,
{
    fn from(input: Vec<Vec<T>>) -> Self {
        let width = input[0].len();
        let height = input.len();
        let storage = input.concat();
        assert_eq!(storage.len(), width * height);

        Grid {
            width: width as i32,
            height: height as i32,
            storage,
        }
    }
}

impl<T> Grid<T>
{
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

impl<T> Grid<T>
where
    T: Sized + Clone,
{
    pub fn new_empty(width: usize, height: usize, initial: T) -> Self {
        let mut storage = Vec::new();
        storage.resize_with(width * height, || initial.clone());
        Grid {
            width: width as i32,
            height: height as i32,
            storage,
        }
    }
}

impl<T> Grid<T>
where
    T: Sized,
{
    pub fn get(&self, x: i32, y: i32) -> &T {
        assert!(0 <= x);
        assert!(x < self.width);
        assert!(0 <= y);
        assert!(y < self.height);
        self.storage.get((x + y * self.width) as usize).unwrap()
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        assert!(0 <= x);
        assert!(x < self.width);
        assert!(0 <= y);
        assert!(y < self.height);
        self.storage.get_mut((x + y * self.width) as usize).unwrap()
    }

    pub fn get_unchecked(&self, x: i32, y: i32) -> &T {
        self.storage.get((x + y * self.width) as usize).unwrap()
    }

    pub fn try_get(&self, x: i32, y: i32) -> Option<&T> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.storage.get((x + y * self.width) as usize)
        } else {
            None
        }
    }

    pub fn try_get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.storage.get_mut((x + y * self.width) as usize)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, c: T) {
        assert!(0 <= x);
        assert!(x < self.width);
        assert!(0 <= y);
        assert!(y < self.height);
        *self.storage.get_mut((x + y * self.width) as usize).unwrap() = c;
    }

    pub fn try_set(&mut self, x: i32, y: i32, c: T) -> Option<()> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            *self.storage.get_mut((x + y * self.width) as usize).unwrap() = c;
            Some(())
        } else {
            None
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(i32, i32, &T),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                f(x, y, self.get(x, y));
            }
        }
    }

    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(i32, i32, &mut T),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                f(x, y, self.get_mut(x, y));
            }
        }
    }
}

impl<T> Grid<T>
where
    T: Sized + Clone,
{
    pub fn flipv(&self) -> Self {
        let storage = (0..self.height).map(|y|
            (0..self.width).map(|x|
                self.get(x, self.height - 1 - y).clone()
        ).collect::<Vec<T>>()).concat();
        Self { width: self.width, height: self.height, storage }
    }

    pub fn fliph(&self) -> Self {
        let storage = (0..self.height).map(|y|
            (0..self.width).map(|x|
                self.get(self.width - 1 - x, y).clone()
        ).collect::<Vec<T>>()).concat();
        Self { width: self.width, height: self.height, storage }
    }

    pub fn rot90(&self) -> Self {
        let storage = (0..self.width).map(|y|
            (0..self.height).map(|x|
                self.get(y, self.height - 1 - x).clone()
        ).collect::<Vec<T>>()).concat();
        Self { width: self.height, height: self.width, storage }
    }
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn find(&self, c: &T) -> Option<(i32, i32)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_unchecked(x, y) == c {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn map<B, F>(&self, f: F) -> Grid<B>
    where
        F: FnMut(T) -> B,
    {
        Grid {
            width: self.width,
            height: self.height,
            storage: self.storage.iter().copied().map(f).collect(),
        }
    }

    pub fn map_coords<B, F>(&self, mut f: F) -> Grid<B>
    where
        F: FnMut(i32, i32, T) -> B,
    {
        Grid {
            width: self.width,
            height: self.height,
            storage: self
                .storage
                .iter()
                .enumerate()
                .map(|(i, c)| f(i as i32 % self.width, i as i32 / self.width, *c))
                .collect(),
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self.get(x, y).fmt(f)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                self.get(x, y).fmt(f)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
