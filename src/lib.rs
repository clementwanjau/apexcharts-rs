#[doc = include_str!("../README.md")]

mod options;
mod bindings;

#[cfg(feature = "yew-component")]
mod yew;
#[cfg(feature = "leptos-component")]
mod leptos;

pub mod prelude {
	pub use crate::bindings::ApexChart;
	pub use crate::options::{ChartType, ChartSeries, SeriesData, to_jsvalue};
	#[cfg(feature = "yew-component")]
	pub use crate::yew::{ApexChartComponent, ApexChartComponentProps};
	#[cfg(feature = "leptos-component")]
	pub use crate::leptos::ApexChartComponent;
}
