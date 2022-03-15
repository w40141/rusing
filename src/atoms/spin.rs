use getset::Getters;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Spin {
    symbol: Option<String>,
    coefficient: i64,
    value: State,
}

#[derive(Debug)]
pub enum State {
    UP = 1,
    Zero = 0,
    DOWN = -1,
}
