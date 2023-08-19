fn main() {
    let mut triangle = PascalTriangle::default();
    let size_of_square: usize = 20;
    triangle.build(size_of_square * 2 + 1);
    // triangle.draw();
    println!("{}", triangle.steps[size_of_square * 2][size_of_square]);
}

struct PascalTriangle {
    steps: Vec<Vec<u64>>,
}

impl Default for PascalTriangle {
    fn default() -> Self {
        Self { steps: vec![vec![1]] }
    }
}

impl PascalTriangle {
    fn add_step(&mut self) {
        let mut new_step = vec![1];
        let last_step = self.steps.last().unwrap();
        for j in 1..last_step.len() {
            new_step.push(last_step[j - 1] + last_step[j]);
        }
        new_step.push(1);
        self.steps.push(new_step);
    }

    fn build(&mut self, size: usize) {
        for _ in 1..size {
            self.add_step();
        }
    }

    fn draw(&self) {
        for step in &self.steps {
            for x in step {
                print!("{} ", x);
            }
            println!();
        }
    }
}
