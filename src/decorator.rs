pub trait Price {
    fn calculate(&self, price: f32) -> f32;
}

pub struct BasePrice;

impl Price for BasePrice {
    fn calculate(&self, price: f32) -> f32 {
        price
    }
}

pub struct TaxDecorator {
    decorated: Box<dyn Price>,
}

impl TaxDecorator {
    pub fn new(decorated: Box<dyn Price>) -> Self {
        Self { decorated }
    }
}

impl Price for TaxDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorated.calculate(price) * 1.15
    }
}

pub struct DiscauntDecorator {
    decorated: Box<dyn Price>,
}

impl DiscauntDecorator {
    pub fn new(decorated: Box<dyn Price>) -> Self {
        Self { decorated }
    }
}

impl Price for DiscauntDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorated.calculate(price) * 0.75
    }
}