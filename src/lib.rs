#[doc = include_str!("../README.md")]

mod options;
mod bindings;

#[cfg(feature = "yew-component")]
mod yew;

pub mod prelude {
	pub use crate::bindings::ApexChart;
	pub use crate::options::{ChartType, ChartSeries, SeriesData, to_jsvalue};
	#[cfg(feature = "yew-component")]
	pub use crate::yew::{ApexChartComponent, ApexChartComponentProps};
}
