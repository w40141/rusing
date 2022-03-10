use getset::Getters;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Binary {
    symbol: Option<String>,
    indexes: Option<Vec<i64>>,
    coefficient: i64,
    value: B,
}

#[derive(Debug)]
pub enum B {
    One,
    Zero,
}
