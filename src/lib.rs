#[doc = include_str!("../README.md")]

mod options;
mod bindings;

pub mod prelude {
	pub use crate::bindings::ApexChart;
	pub use crate::options::{ChartOptions, ChartType, ChartSeries, SeriesData};
}
