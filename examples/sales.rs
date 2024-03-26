use patterns::factory::sales_lib::{Analizer, Reporter};
use patterns::factory::SalesFactory;

fn main() {
    let json_factory = patterns::factory::json_factory();
    println!("JSON analize result: {}", analyze_report(json_factory));

    let xml_factory = patterns::factory::xml_factory();
    println!("XML analize result: {}", analyze_report(xml_factory));
}

fn analyze_report(factory: impl SalesFactory) -> usize {
    let reporter = factory.create_reporter();
    let anilizer = factory.create_analizer();
    anilizer.analize(reporter.report())
}
