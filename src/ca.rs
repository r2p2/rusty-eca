type Row  = Vec<bool>;
type Grid = Vec<Row>;

pub struct CA {
    grid: Grid,
    rule: u8,
}

impl CA {
    pub fn new(width: usize, height: usize, rule: u8) -> Result<CA, String> {
        if width < 2 || height < 2 {
            return Err(String::from("width and hight too small"))
        }

        Ok(CA {
            grid: vec!(vec!(false; width); height),
            rule: rule,
        })
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn width(&self) -> usize {
        match self.grid.first() {
            Some(row) => row.len(),
            None => 0
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
        for y in 0..self.grid.len()-1 {
            self.grid[y] = self.grid[y+1].clone();
        }

        let last_row = self.grid.last_mut().expect("");
        let mut new_row = last_row.clone();

        for x in 1..last_row.len()-1 {
            let l = last_row[x-1] as u8;
            let m = last_row[x] as u8;
            let r = last_row[x+1] as u8;
            let sig = (l << 2) | (m << 1) | r ;
            new_row[x] = (self.rule & (1 << sig)) > 0;
        }

        *last_row = new_row;
    }
}
