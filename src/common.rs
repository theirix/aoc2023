pub struct Answer {
    pub a: &'static str,
    pub b: &'static str,
    pub path: &'static str,
}

//impl Answer {
//pub const fn new(a: usize, b: usize) -> Answer {
//Answer { a, b, path: std::module_path!() }
//}
//}

#[macro_export]
macro_rules! answer {
    ($a: expr, $b: expr) => {
        Answer {
            a: $a,
            b: $b,
            path: std::module_path!(),
        }
    };
}
