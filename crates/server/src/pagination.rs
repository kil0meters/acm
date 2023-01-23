use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination<const C: i64, const O: i64> {
    #[serde(default = "default_value::<O>")]
    pub count: i64,

    #[serde(default = "default_value::<C>")]
    pub offset: i64,
}

fn default_value<const T: i64>() -> i64 {
    T
}
