const SIZE: usize = 20;
const GRID: [[u32; SIZE]; SIZE] = [
    [
        8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92,
    ],
    [
        16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40,
    ],
    [
        4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54,
    ],
    [
        1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48,
    ],
];

struct Horizontal {
    width: usize,
    height: usize,
    items: usize,
    x: usize,
    y: usize,
}

impl Horizontal {
    fn new(width: usize, height: usize, items: usize) -> Horizontal {
        Horizontal {
            width,
            height,
            x: 0,
            y: 0,
            items,
        }
    }
}

impl Iterator for Horizontal {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let Horizontal {
            width,
            height,
            x,
            y,
            items,
        } = &self;
        let done = y >= height;
        if !done {
            let done_row = x + items > *width;

            if done_row {
                self.y += 1;
                self.x = 0;

                self.next()
            } else {
                let result = Some((0..*items).map(|i| (*y, x + i)).collect());
                self.x += 1;
                result
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod horizontal_tests {
    use super::*;

    #[test]
    fn works() {
        let iter = Horizontal::new(5, 2, 4);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 1), (0, 2), (0, 3), (0, 4)],
                vec![(1, 0), (1, 1), (1, 2), (1, 3)],
                vec![(1, 1), (1, 2), (1, 3), (1, 4)],
            ]
        );

        let iter = Horizontal::new(2, 2, 1);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![vec![(0, 0)], vec![(0, 1)], vec![(1, 0)], vec![(1, 1)]]
        );
    }

    #[test]
    fn grid() {
        let iter = Horizontal::new(SIZE, SIZE, 4);
        let last = iter.last().unwrap();
        let from_grid: Vec<_> = last.into_iter().map(|(i, j)| GRID[i][j]).collect();

        assert_eq!(from_grid, vec![89, 19, 67, 48]);
    }
}

struct Vertical {
    width: usize,
    height: usize,
    items: usize,
    x: usize,
    y: usize,
}

impl Vertical {
    fn new(width: usize, height: usize, items: usize) -> Vertical {
        Vertical {
            width,
            height,
            x: 0,
            y: 0,
            items,
        }
    }
}

impl Iterator for Vertical {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let Vertical {
            width,
            height,
            x,
            y,
            items,
        } = &self;
        let done = x >= width;
        if !done {
            let done_col = y + items > *height;

            if done_col {
                self.x += 1;
                self.y = 0;

                self.next()
            } else {
                let result = Some((0..*items).map(|i| (y + i, *x)).collect());
                self.y += 1;
                result
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod vertical_tests {
    use super::*;

    #[test]
    fn works() {
        let iter = Vertical::new(2, 5, 4);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(1, 0), (2, 0), (3, 0), (4, 0)],
                vec![(0, 1), (1, 1), (2, 1), (3, 1)],
                vec![(1, 1), (2, 1), (3, 1), (4, 1)],
            ]
        );

        let iter = Vertical::new(2, 2, 1);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![vec![(0, 0)], vec![(1, 0)], vec![(0, 1)], vec![(1, 1)]]
        );
    }

    #[test]
    fn grid() {
        let iter = Vertical::new(SIZE, SIZE, 4);
        let last = iter.last().unwrap();
        let from_grid: Vec<_> = last.into_iter().map(|(i, j)| GRID[i][j]).collect();

        assert_eq!(from_grid, vec![36, 16, 54, 48]);
    }
}

struct DiagonalNorth {
    width: usize,
    height: usize,
    items: usize,
    x: usize,
    y: usize,
}

impl DiagonalNorth {
    fn new(width: usize, height: usize, items: usize) -> DiagonalNorth {
        DiagonalNorth {
            width,
            height,
            x: 0,
            y: 0,
            items,
        }
    }
}

impl Iterator for DiagonalNorth {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let DiagonalNorth {
            width,
            height,
            x,
            y,
            items,
        } = &self;
        let done = y + items > *height;
        if !done {
            let done_row = x + items > *width;

            if done_row {
                self.y += 1;
                self.x = 0;

                self.next()
            } else {
                let result = Some((0..*items).map(|i| (y + i, x + i)).collect());
                self.x += 1;
                result
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod diagonal_north_tests {
    use super::*;

    #[test]
    fn works() {
        let iter = DiagonalNorth::new(5, 5, 4);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![
                vec![(0, 0), (1, 1), (2, 2), (3, 3)],
                vec![(0, 1), (1, 2), (2, 3), (3, 4)],
                vec![(1, 0), (2, 1), (3, 2), (4, 3)],
                vec![(1, 1), (2, 2), (3, 3), (4, 4)],
            ]
        );
    }

    #[test]
    fn grid() {
        let iter = DiagonalNorth::new(SIZE, SIZE, 4);
        let last = iter.last().unwrap();
        let from_grid: Vec<_> = last.into_iter().map(|(i, j)| GRID[i][j]).collect();

        assert_eq!(from_grid, vec![40, 4, 5, 48]);
    }
}

struct DiagonalSouth {
    width: usize,
    height: usize,
    items: usize,
    x: usize,
    y: usize,
}

impl DiagonalSouth {
    fn new(width: usize, height: usize, items: usize) -> DiagonalSouth {
        DiagonalSouth {
            width,
            height,
            x: 0,
            y: 0,
            items,
        }
    }
}

impl Iterator for DiagonalSouth {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let DiagonalSouth {
            width,
            height,
            x,
            y,
            items,
        } = &self;
        let done = y + items > *height;
        if !done {
            let done_row = x + items > *width;

            if done_row {
                self.y += 1;
                self.x = 0;

                self.next()
            } else {
                let result = Some(
                    (0..*items)
                        .rev()
                        .map(|i| (y + (items - i - 1), x + i))
                        .collect(),
                );
                self.x += 1;
                result
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod diagonal_south_tests {
    use super::*;

    #[test]
    fn works() {
        let iter = DiagonalSouth::new(5, 5, 4);
        let items: Vec<_> = iter.collect();
        assert_eq!(
            items,
            vec![
                vec![(0, 3), (1, 2), (2, 1), (3, 0)],
                vec![(0, 4), (1, 3), (2, 2), (3, 1)],
                vec![(1, 3), (2, 2), (3, 1), (4, 0)],
                vec![(1, 4), (2, 3), (3, 2), (4, 1)],
            ]
        );
    }

    #[test]
    fn grid() {
        let iter = DiagonalSouth::new(SIZE, SIZE, 4);
        let last = iter.last().unwrap();
        let from_grid: Vec<_> = last.into_iter().map(|(i, j)| GRID[i][j]).collect();

        assert_eq!(from_grid, vec![36, 36, 57, 89]);
    }
}

pub fn solve() -> String {
    Horizontal::new(SIZE, SIZE, 4)
        .chain(Vertical::new(SIZE, SIZE, 4))
        .chain(DiagonalNorth::new(SIZE, SIZE, 4))
        .chain(DiagonalSouth::new(SIZE, SIZE, 4))
        .map(|indices| {
            indices
                .into_iter()
                .map(|(i, j)| GRID[i][j])
                .product::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}
