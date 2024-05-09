//! # ApexCharts Rust
//! 
//! This is a Rust library for rendering ApexCharts in web applications. It provides a simple API for 
//! rendering different types of charts such as line, bar, pie, donut, and radial bar charts. The 
//! library is designed to be used with popular web frameworks such as Yew and Leptos.
//! 
//! ## Features
//! By default the library provides Wasm bindings for rendering ApexCharts in web applications. To actually render the charts in a web application, you need to enable one of the following features depending on the web framework you are using:
//! 
//! - **yew**: Enables support for rendering ApexCharts in Yew applications.
//! - **leptos**: Enables support for rendering ApexCharts in Leptos applications.
//! 
//! ## Usage
//! 
//! To use this library, add the following to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! apexcharts-rs = {version="0.1", features=["yew"]}
//! ```
//! 
//! To render a chart in a Yew application, you can use the `ApexChartComponent` component:
//! 
//! ```rust,ignore
//! use yew::prelude::*;
//! use apexcharts_rs::prelude::{ApexChartComponent, ApexChartComponentProps, ChartSeries, SeriesData, ChartType};
//! 
//! #[function_component]
//! pub fn App() -> Html {
//!   let series = vec![
//!     ChartSeries {
//!       name: "Series 1".to_string(),
//!       data: SeriesData::Single(vec![10, 20, 30, 40, 50]),
//!       color: "#008FFB".to_string(),
//!       r#type: None,
//!       z_index: None,
//!     }
//!   ];
//!   html! {
//!     <ApexChartComponent
//!       id={"my_area_chart".to_string()}
//!       r#type={ChartType::Area}
//!       series={series}
//!       width={"100%".to_string()}
//!       height={"300px".to_string()}
//!     />
//!   }
//! }
//! 
//! fn main() {
//!   yew::Renderer::<App>::new().render();
//! }
//! ```
//! 
//! The equivalent component for Leptos applications is `ApexChartComponent` with the `leptos` feature enabled.
//! 
//! ```rust,ignore
//! use leptos::*;
//! use apexcharts_rs::prelude::{ApexChartComponent, ChartSeries, ChartType, SeriesData};
//! 
//! #[component]
//! fn App() -> impl IntoView {
//!   let series = vec![
//!     ChartSeries {
//!       name: "Series 1".to_string(),
//!       data: SeriesData::Single(vec![10, 20, 30, 40, 50]),
//!       color: "#008FFB".to_string(),
//!       r#type: None,
//!       z_index: None,
//!     }
//!   ];
//!   let (series, _) = create_signal(series);
//!   view! {
//!     <ApexChartComponent
//!       id={"my_area_chart".to_string()}
//!       r#type={ChartType::Area}
//!       series={series}
//!       width={"100%".to_string()}
//!       height={"300px".to_string()}
//!     />
//!   }
//! }
//! 
//! fn main() {
//!   mount_to_body(App);
//! }
//! ```
//! 
//! To change the type of the chart, you can set the `r#type` property to the desired type. The `series` property
//! is used to set the data that will be rendered in the chart. The `width` and `height` properties are used to set
//! the width and height of the chart respectively.
//! 
//! To further customize the chart, you can set the `options` property to a JSON string containing the options for the chart.
//! Kindly refer to the [ApexCharts documentation](https://apexcharts.com/docs/options/) for more information on the available options.

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
