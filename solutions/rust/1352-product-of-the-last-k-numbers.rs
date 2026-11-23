struct ProductOfNumbers {
    products: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { products: vec![] }
    }

    fn add(&mut self, num: i32) {
        for number in self.products.iter_mut().rev() {
            if *number == 0 {
                break;
            }

            *number *= num;
        }
        self.products.push(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.products.len();
        self.products[n - k as usize]
    }
}
