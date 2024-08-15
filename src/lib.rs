#[macro_export]
macro_rules! avec {
    // Create Vector With One Or Many Items
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $(v.push($element);)*
        v
    }};

    // Create Vector With Repeat Element
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut v = Vec::with_capacity(count);
        v.extend(std::iter::repeat($element).take(count));
        v
    }};
}

#[test]
fn empty_vec() {
    let v: Vec<i32> = avec![];
    assert_eq!(v, []);
    assert!(v.is_empty());
}

#[test]
fn single() {
    let v: Vec<i32> = avec![42];
    assert!(!v.is_empty());
    assert_eq!(v[0], 42);
    assert_eq!(v.len(), 1);
}

#[test]
fn double() {
    let v: Vec<i32> = avec![42, 45];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 45);
}

#[test]
fn many() {
    let v: Vec<i32> = avec![42, 45, 49, 55];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 4);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 45);
    assert_eq!(v[2], 49);
    assert_eq!(v[3], 55);
}

#[test]
fn fill() {
    let v: Vec<i32> = avec![42; 5];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 5);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
    assert_eq!(v[2], 42);
    assert_eq!(v[3], 42);
    assert_eq!(v[4], 42);
}

#[test]
fn fill_non_literal() {
    let mut y = Some(42);
    let v: Vec<i32> = avec![y.take().unwrap(); 5];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 5);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 42);
    assert_eq!(v[2], 42);
    assert_eq!(v[3], 42);
    assert_eq!(v[4], 42);
}

#[test]
fn trailing() {
    let v: Vec<i32> = avec![42,];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}
