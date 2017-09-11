type Row = Vec<bool>;
type Grid = Vec<Row>;

pub struct CA {
    grid: Grid,
    rule: u8,
}

impl CA {
    pub fn new(width: usize, height: usize, rule: u8) -> Result<CA, String> {
        if width < 2 || height < 2 {
            return Err(String::from("width and hight too small"));
        }

        Ok(CA {
            grid: vec![vec![false; width]; height],
            rule: rule,
        })
    }

    pub fn rule(&self) -> u8 {
        self.rule
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn width(&self) -> usize {
        match self.grid.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn fill(&mut self, x: usize, y: usize) {
        self.grid[y][x] = true;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self, x: usize, y: usize) {
        self.grid[y][x] = false;
    }

    pub fn update(&mut self) {
        let width = self.width();
        let height = self.height();

        for y in 0..height - 1 {
            self.grid.swap(y, y + 1);
        }

        let (last_row, rest_rows) = self.grid.split_last_mut().unwrap();
        let (second_last_row, _) = rest_rows.split_last().unwrap();

        for (i, cell) in last_row.iter_mut().enumerate().take(width - 1).skip(1) {
            let mut value = 0u8;
            for (n, old_cell) in second_last_row.iter().skip(i - 1).take(3).enumerate() {
                value |= (*old_cell as u8) << (2 - n);
            }
            *cell = self.rule & (1 << value) > 0;
        }
    }
}
