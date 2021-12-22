#[derive(Debug)]
pub struct Board {
  size: usize,
  nums: Vec<usize>,
  seen: Vec<bool>,
}

impl Board {
  pub fn from_nums(size: usize, nums: Vec<usize>) -> Board {
    Board {
      size,
      nums,
      seen: vec![false; size * size],
    }
  }

  pub fn see(&mut self, num: usize) {
    for i in 0..(self.size * self.size) {
      if self.nums[i] == num {
        self.seen[i] = true
      }
    }
  }

  pub fn is_complete(&self) -> bool {
    for row in 0..self.size {
      let mut complete = true;

      for column in 0..self.size {
        complete &= self.seen[row * self.size + column];
      }

      if complete {
        return true;
      }
    }

    for column in 0..self.size {
      let mut complete = true;

      for row in 0..self.size {
        complete &= self.seen[column + row * self.size];
      }

      if complete {
        return true;
      }
    }

    false
  }

  pub fn unmarked_sum(&self) -> usize {
    let mut res = 0;

    for i in 0..(self.size * self.size) {
      if !self.seen[i] {
        res += self.nums[i]
      }
    }

    res
  }
}
