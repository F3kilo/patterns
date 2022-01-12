use patterns::factory::SalesFactory;
use patterns::factory::sales_lib::Analizer;

fn main() {
    let json_factory = patterns::factory::json_factory();
    let reporter = json_factory.create_reporter();
    let anilizer = json_factory.create_analizer();
    println!("JSON analize result: {}", anilizer.analize(reporter));

    let xml_factory = patterns::factory::xml_factory();
    let reporter = xml_factory.create_reporter();
    let anilizer = xml_factory.create_analizer();
    println!("XML analize result: {}", anilizer.analize(reporter));
}