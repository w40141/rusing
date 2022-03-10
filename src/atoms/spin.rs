use getset::Getters;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Spin {
    symbol: Option<String>,
    indexes: Option<Vec<i64>>,
    coefficient: i64,
    value: S,
}

#[derive(Debug)]
pub enum S {
    UP,
    DOWN,
}
