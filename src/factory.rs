use crate::factory::sales_lib::{
    Analizer, JsonAnalizer, JsonReporter, Reporter, XmlAnalizer, XmlReporter,
};

pub fn xml_factory() -> impl SalesFactory {
    XmlSalesFactory
}

pub fn json_factory() -> impl SalesFactory {
    JsonSalesFactory
}

pub trait SalesFactory {
    type Report: Reporter;
    type Anilize: Analizer;

    fn create_reporter(&self) -> Self::Report;
    fn create_analizer(&self) -> Self::Anilize;
}

struct JsonSalesFactory;

impl SalesFactory for JsonSalesFactory {
    type Report = JsonReporter;
    type Anilize = JsonAnalizer;

    fn create_reporter(&self) -> Self::Report {
        JsonReporter
    }

    fn create_analizer(&self) -> Self::Anilize {
        JsonAnalizer
    }
}

struct XmlSalesFactory;

impl SalesFactory for XmlSalesFactory {
    type Report = XmlReporter;
    type Anilize = XmlAnalizer;

    fn create_reporter(&self) -> Self::Report {
        XmlReporter
    }

    fn create_analizer(&self) -> Self::Anilize {
        XmlAnalizer
    }
}

pub mod sales_lib {
    pub trait Reporter {
        fn report(&self) -> String;
    }

    pub struct XmlReporter;

    impl Reporter for XmlReporter {
        fn report(&self) -> String {
            "<sold>42</sold>".into()
        }
    }

    pub struct JsonReporter;

    impl Reporter for JsonReporter {
        fn report(&self) -> String {
            "{\"sold\":42}".into()
        }
    }

    pub trait Analizer {
        fn analize(&self, sales: String) -> usize;
    }

    pub struct JsonAnalizer;

    impl Analizer for JsonAnalizer {
        fn analize(&self, report: String) -> usize {
            if let Some(ch) = report.chars().next() {
                if ch != '{' {
                    panic!("bad json")
                }
            }
            42
        }
    }

    pub struct XmlAnalizer;

    impl Analizer for XmlAnalizer {
        fn analize(&self, report: String) -> usize {
            if let Some(ch) = report.chars().next() {
                if ch != '<' {
                    panic!("bad xml")
                }
            }
            42
        }
    }
}
