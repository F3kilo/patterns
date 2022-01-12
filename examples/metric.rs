use patterns::visitor::{
    DecreaseVisitor, FloatMetric, IncreaseVisitor, IntMetric, Metric,
};

fn main() {
    let mut float_metric = get_float_metric();
    let mut int_metric = get_int_metric();

    println!(
        "int: {}, float: {}",
        int_metric.get_report(),
        float_metric.get_report()
    );

    int_metric.accept(&IncreaseVisitor);
    float_metric.accept(&DecreaseVisitor);

    println!(
        "int: {}, float: {}",
        int_metric.get_report(),
        float_metric.get_report()
    );
}

fn get_float_metric() -> impl Metric {
    FloatMetric::default()
}

fn get_int_metric() -> impl Metric {
    IntMetric::default()
}