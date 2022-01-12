use patterns::decorator::{BasePrice, DiscauntDecorator, Price, TaxDecorator};

fn main() {
    let with_tax = TaxDecorator::new(Box::new(BasePrice));
    let with_tax_and_discaunt = DiscauntDecorator::new(Box::new(with_tax));

    let start_price = 100.0;
    let finish_price = with_tax_and_discaunt.calculate(start_price);
    println!("Start price: {}, finish price: {}", start_price, finish_price)
}
