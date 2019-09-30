const SIZE: usize = 1001;

enum Direction {
    Left,
    Down,
    Right,
    Up,
}

pub fn solve() -> String {
    let mut arr = vec![vec![0; SIZE]; SIZE];

    let mut curr = SIZE * SIZE;
    let mut i = 0;
    let mut j = SIZE - 1;
    let mut d = Direction::Left;
    while curr >= 1 {
        arr[i][j] = curr;

        match d {
            Direction::Left => {
                if j == 0 || arr[i][j - 1] != 0 {
                    d = Direction::Down;
                }
            }
            Direction::Down => {
                if i >= SIZE - 1 || arr[i + 1][j] != 0 {
                    d = Direction::Right;
                }
            }
            Direction::Right => {
                if j >= SIZE - 1 || arr[i][j + 1] != 0 {
                    d = Direction::Up;
                }
            }
            Direction::Up => {
                if i == 0 || arr[i - 1][j] != 0 {
                    d = Direction::Left;
                }
            }
        }

        match d {
            Direction::Left => {
                j -= 1;
            }
            Direction::Down => {
                i += 1;
            }
            Direction::Right => {
                j += 1;
            }
            Direction::Up => {
                i -= 1;
            }
        }

        curr -= 1;
    }

    let mut sum = 0;
    i = 0;
    j = 0;
    while i < SIZE {
        sum += arr[i][j];
        sum += arr[i][SIZE - j - 1];
        i += 1;
        j += 1;
    }

    (sum - 1).to_string()
}
