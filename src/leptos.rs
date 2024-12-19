//! This module contains the Leptos component for rendering ApexCharts in a Leptos application.
//!
use leptos::prelude::*;
use wasm_bindgen::JsValue;
use crate::prelude::{ChartSeries, ChartType};

/// An ApexCharts component for Leptos. 
///
/// This component is used to render an ApexCharts chart in a Leptos application. It is used to render different types of charts
/// such as line, bar, pie, donut, and radial bar charts. To use this component, you need to enable the `leptos` feature
/// in the `apexcharts-rs` crate. 
///
///
/// # Example
///
/// ```rust,ignore
/// use leptos::*;
/// use apexcharts_rs::prelude::{ApexChartComponent, ChartSeries, ChartType, SeriesData};
/// 
/// #[component]
/// fn App() -> impl IntoView {
///   let series = vec![
///     ChartSeries {
///       name: "Series 1".to_string(),
///       data: SeriesData::Single(vec![10, 20, 30, 40, 50]),
///       color: "#008FFB".to_string(),
///       r#type: None,
///       z_index: None,
/// 	 }
///    ];
///   let (series, _) = create_signal(series);
///   view! {
///    <ApexChartComponent
/// 	  id={"my_area_chart".to_string()}
/// 	  r#type={ChartType::Area}
/// 	  series={series}
/// 	  width={"100%".to_string()}
/// 	  height={"300px".to_string()}
///   />
///  }
/// }
/// 
/// fn main() {
///   mount_to_body(App);
/// }
/// 
/// 
#[component]
pub fn ApexChartComponent(
	/// The type of the chart.
	#[prop(default = ChartType::Line)]
	r#type: ChartType,
	/// The options for the chart in json.
	#[prop(default = String::default())]
	options: String,
	/// The unique id of the chart.
	id: String,
	/// The series to be rendered in the chart.
	series: ReadSignal<Vec<ChartSeries>>,
	/// The width of the chart.
	#[prop(default = "100%".to_string())]
	width: String,
	/// The height of the chart.
	#[prop(default = "auto".to_string())]
	height: String
) -> impl IntoView {
	let id_clone = id.clone();
	Effect::new(move |_| {
		use serde_json::Value;
		use crate::prelude::{SeriesData, ApexChart};

		let mut labels_data = None;
		let series_data = match r#type {
			ChartType::Pie | ChartType::Donut | ChartType::RadialBar => {
				let chart_series = series.get();
				match chart_series.first() {
					Some(chart_serie) => {
						match chart_serie.data {
							SeriesData::Radial(ref data) => {
								let data_values = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();
								labels_data = Some(data.iter().map(|(x, _)| x.clone()).collect::<Vec<_>>());
								serde_json::to_value(data_values).unwrap_or(Value::Array(vec![]))
							},
							_ => {
								serde_json::to_value(series.get()).unwrap_or(Value::Array(vec![]))
							}
						}
					},
					_=> {
						Value::Array(vec![])
					}
				}
			},
			_=> {
				serde_json::to_value(series.get()).unwrap_or(Value::Array(vec![]))
			}
		};
		let options = if options.is_empty() {
			format!(
				r#"{{
					"chart": {{
						"type": "{}",
						"width": "{}",
						"height": "{}"
					}},
					"series": {}
					{}
				}}"#,
				r#type,
				width,
				height,
				series_data,
				if let Some(labels) = labels_data {
					format!(r#","labels": {}"#, serde_json::to_string(&labels).unwrap_or("[]".to_string()))
				} else {
					"".to_string()
				}
			)
		} else {
			let mut options = serde_json::from_str::<Value>(&options).unwrap_or_else(|_| panic!("Invalid JSON: {}", options));
			options["chart"]["type"] = Value::String(r#type.to_string());
			options["chart"]["width"] = Value::String(width.clone());
			options["chart"]["height"] = Value::String(height.clone());
			options["series"] = series_data;
			if let Some(labels) = labels_data {
				options["labels"] = Value::Array(
					labels
						.iter()
						.map(|label| Value::String(label.clone()))
						.collect()
				);
			}
			serde_json::to_string(&options).unwrap_or_default()
		};
		let chart = ApexChart::new(&JsValue::from_str(&options));
		chart.render(&id_clone);
	});
	view! {
		<div id={id.clone()}></div>
	}
}
