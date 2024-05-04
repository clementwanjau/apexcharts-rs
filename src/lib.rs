#[doc = include_str!("../readme.md")]

mod options;
mod bindings;

pub use crate::bindings::ApexChart;
pub use crate::options::{ChartOptions, ChartType};
