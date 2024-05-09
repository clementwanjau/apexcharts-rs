#![doc = include_str!("../README.md")]

mod options;
mod bindings;

#[cfg(feature = "yew")]
mod yew;
#[cfg(feature = "leptos")]
mod leptos;

pub mod prelude {
	pub use crate::bindings::ApexChart;
	pub use crate::options::{ChartType, ChartSeries, SeriesData, to_jsvalue};
	#[cfg(feature = "yew")]
	pub use crate::yew::{ApexChartComponent, ApexChartComponentProps};
	#[cfg(feature = "leptos")]
	pub use crate::leptos::ApexChartComponent;
}
