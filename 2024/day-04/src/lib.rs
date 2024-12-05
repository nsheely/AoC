pub mod shared {
    #[derive(Debug)]
    pub struct Grid<'a> {
        pub data: Vec<&'a [u8]>,
        pub width: usize,
        pub height: usize,
    }

    impl<'a> Grid<'a> {
        pub fn new(input: &'a str) -> Option<Self> {
            let data: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
            if data.is_empty() || data[0].is_empty() {
                return None;
            }
            Some(Self {
                height: data.len(),
                width: data[0].len(),
                data,
            })
        }

        #[inline(always)]
        pub fn get_unchecked(&self, x: usize, y: usize) -> u8 {
            unsafe { *self.data.get_unchecked(y).get_unchecked(x) }
        }
    }
}

pub mod part1 {
    use super::shared::Grid;
    use rayon::prelude::*;

    pub fn count_xmas_occurrences(input: &str) -> usize {
        let grid = match Grid::new(input) {
            Some(g) => g,
            None => return 0,
        };

        let height = grid.height;
        let width = grid.width;

        (3..height-3).into_par_iter().map(|y| {
            (3..width-3).filter(|&x| {
                grid.get_unchecked(x, y) == b'X' && 
                check_all_directions(&grid, x, y)
            }).count()
        }).sum()
    }

    #[inline(always)]
    fn check_all_directions(grid: &Grid, x: usize, y: usize) -> bool {
        check_south(grid, x, y) || 
        check_southeast(grid, x, y) || 
        check_east(grid, x, y) || 
        check_northeast(grid, x, y) || 
        check_north(grid, x, y) || 
        check_northwest(grid, x, y) || 
        check_west(grid, x, y) || 
        check_southwest(grid, x, y)
    }

    #[inline(always)]
    fn check_south(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x, y+1) == b'M' &&
        grid.get_unchecked(x, y+2) == b'A' &&
        grid.get_unchecked(x, y+3) == b'S'
    }

    #[inline(always)]
    fn check_southeast(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x+1, y+1) == b'M' &&
        grid.get_unchecked(x+2, y+2) == b'A' &&
        grid.get_unchecked(x+3, y+3) == b'S'
    }

    #[inline(always)]
    fn check_east(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x+1, y) == b'M' &&
        grid.get_unchecked(x+2, y) == b'A' &&
        grid.get_unchecked(x+3, y) == b'S'
    }

    #[inline(always)]
    fn check_northeast(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x+1, y-1) == b'M' &&
        grid.get_unchecked(x+2, y-2) == b'A' &&
        grid.get_unchecked(x+3, y-3) == b'S'
    }

    #[inline(always)]
    fn check_north(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x, y-1) == b'M' &&
        grid.get_unchecked(x, y-2) == b'A' &&
        grid.get_unchecked(x, y-3) == b'S'
    }

    #[inline(always)]
    fn check_northwest(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y-1) == b'M' &&
        grid.get_unchecked(x-2, y-2) == b'A' &&
        grid.get_unchecked(x-3, y-3) == b'S'
    }

    #[inline(always)]
    fn check_west(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y) == b'M' &&
        grid.get_unchecked(x-2, y) == b'A' &&
        grid.get_unchecked(x-3, y) == b'S'
    }

    #[inline(always)]
    fn check_southwest(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y+1) == b'M' &&
        grid.get_unchecked(x-2, y+2) == b'A' &&
        grid.get_unchecked(x-3, y+3) == b'S'
    }
}

pub mod part2 {
    use super::shared::Grid;
    use rayon::prelude::*;

    pub fn count_xmas_occurrences(input: &str) -> usize {
        let grid = match Grid::new(input) {
            Some(g) => g,
            None => return 0,
        };

        let height = grid.height;
        let width = grid.width;

        (1..height-1).into_par_iter().map(|y| {
            (1..width-1).filter(|&x| {
                grid.get_unchecked(x, y) == b'A' &&
                check_x_pattern(&grid, x, y)
            }).count()
        }).sum()
    }

    #[inline(always)]
    fn check_x_pattern(grid: &Grid, x: usize, y: usize) -> bool {
        check_both_forward(grid, x, y) ||
        check_both_backward(grid, x, y) ||
        check_forward_backward(grid, x, y) ||
        check_backward_forward(grid, x, y)
    }

    #[inline(always)]
    fn check_both_forward(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y-1) == b'M' &&
        grid.get_unchecked(x+1, y+1) == b'S' &&
        grid.get_unchecked(x-1, y+1) == b'M' &&
        grid.get_unchecked(x+1, y-1) == b'S'
    }

    #[inline(always)]
    fn check_both_backward(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y-1) == b'S' &&
        grid.get_unchecked(x+1, y+1) == b'M' &&
        grid.get_unchecked(x-1, y+1) == b'S' &&
        grid.get_unchecked(x+1, y-1) == b'M'
    }

    #[inline(always)]
    fn check_forward_backward(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y-1) == b'M' &&
        grid.get_unchecked(x+1, y+1) == b'S' &&
        grid.get_unchecked(x-1, y+1) == b'S' &&
        grid.get_unchecked(x+1, y-1) == b'M'
    }

    #[inline(always)]
    fn check_backward_forward(grid: &Grid, x: usize, y: usize) -> bool {
        grid.get_unchecked(x-1, y-1) == b'S' &&
        grid.get_unchecked(x+1, y+1) == b'M' &&
        grid.get_unchecked(x-1, y+1) == b'M' &&
        grid.get_unchecked(x+1, y-1) == b'S'
    }
}