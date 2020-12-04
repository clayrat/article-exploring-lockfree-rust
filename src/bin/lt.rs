// single-threaded

pub struct LazyTransform<T, S, FN> {
    transform_fn: FN,
    source: Option<S>,
    value: Option<T>,
}

impl<T: Clone, S, FN: Fn(S) -> Option<T>> LazyTransform<T, S, FN> {
    pub fn new(transform_fn: FN) -> LazyTransform<T, S, FN> {
        LazyTransform {
            transform_fn: transform_fn,
            source: None,
            value: None,
        }
    }

    pub fn set_source(&mut self, source: S) {
        self.source = Some(source);
    }

    pub fn get_transformed(&mut self) -> Option<T> {
        if let Some(source) = self.source.take() {
            let new_value = (self.transform_fn)(source);
            if new_value.is_some() {
                self.value = new_value;
            }
        }
        self.value.clone()
    }
}

fn main() {
    let mut lt = LazyTransform::new(|x: u64| Some(x + 1));

    for i in 0..10_000 {
        lt.set_source(i);
    }

    let val = lt.get_transformed().unwrap();

    println!("{}", val);

    /*
    std::thread::spawn(move || {
        for i in 0..10_000 {
            lt.set_source(i);
        }
    });
    while lt.get_transformed().is_none() {
    }
    let val = lt.get_transformed().unwrap();
    assert!(val >=0 && val < 10_000);
    */
}
