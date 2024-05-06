use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys;

/// Represents the options that can be passed to the ApexCharts constructor. This is a wrapper around
/// the JSON object that ApexCharts expects.
///
/// ## Usage
/// ```rust
/// use apexcharts_rs::prelude::ChartOptions;
///
/// let options_str = r#""
/// {
///     "chart": {
///         "type": "line"
///     },
///     "series": [
///         {
///             "name": "Series 1",
///             "data": [30, 40, 35, 50, 49, 125]
///         }
///     ]
/// }""#;
/// let options = ChartOptions::from_string(String::from(options_str));
/// ```
#[derive(Clone, Debug)]
pub struct ChartOptions {
	options: String
}

impl ChartOptions {
	/// Create a new instance of ChartOptions from a string.
	/// ```rust
	/// let options_str = r#"
	/// {
	///    "chart": {
	///       "type": "line"
	///   },
	///  "series": [
	///    {
	///       "name": "Series 1",
	///       "data": [30, 40, 35, 50, 49, 125]
	///   }
	/// ]
	/// }"#;
	/// let options = apexcharts_rs::prelude::ChartOptions::from_string(String::from(options_str));
	/// //let chart = apexcharts_rs::prelude::ApexChart::new(&options.into());
	/// ```
	pub fn from_string(options: String) -> Self {
		Self {
			options
		}
	}
	/// Create a new instance of ChartOptions from a json file.
	/// ```rust
	/// let options_str = r#"
	/// {
	///    "chart": {
	///       "type": "line"
	///   },
	///  "series": [
	///    {
	///       "name": "Series 1",
	///       "data": [30, 40, 35, 50, 49, 125]
	///   }
	/// ]
	/// }"#;
	/// let filename = "options.json";
	/// std::fs::write(filename, options_str).unwrap();
	/// let options = apexcharts_rs::prelude::ChartOptions::from_file(filename);
	/// //let chart = apexcharts_rs::prelude::ApexChart::new(&options.into());
	/// std::fs::remove_file(filename).unwrap();
	/// ```
	pub fn from_file(file_path: &str) -> Self {
		// Read the file and return the content as a string
		let options = std::fs::read_to_string(file_path).unwrap();
		Self {
			options
		}
	}

	/// Set the type of the chart. This overrides the type set in the options.
	pub fn set_chart_type(&self, chart_type: ChartType) -> Self {
		let chart_type = chart_type.to_string();
		let mut content: Value = serde_json::from_str(&self.options).unwrap();
		content["chart"]["type"] = Value::String(chart_type);
		Self {
			options: to_string_pretty(&content).unwrap()
		}
	}
}

impl From<ChartOptions> for JsValue {
	fn from(options: ChartOptions) -> JsValue {
		JsValue::from_str(options.options.as_str())
	}
}

impl Display for ChartOptions {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.options)
	}
}

/// Represents the type of the chart that will be rendered.
pub enum ChartType {
	Area,
	Bar,
	Line,
	Column,
	BoxPlot,
	CandleStick,
	RangeBar,
	RangeArea,
	HeatMap,
	Treemap,
	Funnel,
	MultiAxis,
	Pie,
	Donut,
	Radar,
	RadialBar,
	CircularGauge,
}

impl Display for ChartType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ChartType::Area => write!(f, "area"),
			ChartType::Bar => write!(f, "bar"),
			ChartType::Line => write!(f, "line"),
			ChartType::Column => write!(f, "column"),
			ChartType::BoxPlot => write!(f, "boxPlot"),
			ChartType::CandleStick => write!(f, "candlestick"),
			ChartType::RangeBar => write!(f, "rangeBar"),
			ChartType::RangeArea => write!(f, "rangeArea"),
			ChartType::HeatMap => write!(f, "heatmap"),
			ChartType::Treemap => write!(f, "treemap"),
			ChartType::Funnel => write!(f, "funnel"),
			ChartType::MultiAxis => write!(f, "multiAxis"),
			ChartType::Pie => write!(f, "pie"),
			ChartType::Donut => write!(f, "donut"),
			ChartType::Radar => write!(f, "radar"),
			ChartType::RadialBar => write!(f, "radialBar"),
			ChartType::CircularGauge => write!(f, "radialBar"),
		}
	}
}

/// Represents the data that will be rendered in the chart.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SeriesData {
	/// Represents a single array of data points. eg `[10, 20, 30]`
	Single(Vec<f64>),
	/// Represents a double array of data points. eg `[(10, 20), (20, 30)]`
	NumericPaired(Vec<(f64, f64)>),
	/// Represents a double array of data points with a category. eg `[("Apple", 30), ("Banana", 40)]`
	CategoryPaired(Vec<(String, f64)>),
	/// Represents a double array of data points with a timestamp. eg `[(1619683200, 30), (1619769600, 40)]`
	Timestamped(Vec<(u64, f64)>),
	/// Represents a double array of data points with a date. eg `[("2021-04-29", 30), ("2021-04-30", 40)]`
	Dated(Vec<(String, f64)>),
}

/// Represents a series in the chart.
/// 
/// This type is used to represent a series in the chart. It contains the name of the series, the data
/// that will be rendered, the color of the series, the type of the series, and the z-index of the series.
/// It is mostly used when you want to update the series in the chart dynamically.
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartSeries {
	pub name: String,
	pub data: SeriesData,
	pub color: String,
	pub r#type: Option<String>,
	pub z_index: Option<i32>,
}

impl From<ChartSeries> for JsValue {
	fn from(chart_series: ChartSeries) -> JsValue {
		let series = js_sys::Object::new();
		js_sys::Reflect::set(&series, &JsValue::from_str("name"), &JsValue::from_str(&chart_series.name)).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("data"), &serde_wasm_bindgen::to_value(&chart_series.data).unwrap()).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("color"), &JsValue::from_str(&chart_series.color)).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("type"), &JsValue::from_str(&chart_series.r#type.unwrap_or("".to_string()))).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("zIndex"), &JsValue::from_f64(chart_series.z_index.unwrap_or(0) as f64)).unwrap();
		series.into()
	}
}

pub fn to_jsvalue<T: Into<JsValue>>(vec: Vec<T>) -> JsValue {
	let array = js_sys::Array::new();
	for item in vec {
		array.push(&item.into());
	}
	array.into()
}

#[cfg(test)]
mod tests {
	use serde_json::Value;
	use wasm_bindgen_test::wasm_bindgen_test;
	use crate::prelude::{ChartOptions, ChartType};

	const OPTIONS_STR: &str = r#"
	{
		"chart": {
			"type": "line"
		},
		"series": [
			{
				"data": [30, 40, 35, 50, 49, 125],
				"name": "Series 1"
			}
		]
	}"#;
	
	#[wasm_bindgen_test]
	pub fn test_parsing_chart_options() {
		let options = ChartOptions::from_string(String::from(OPTIONS_STR));
		assert_eq!(options.options, OPTIONS_STR);
	}

	#[wasm_bindgen_test]
	pub fn test_setting_chart_type() {
		let options = ChartOptions::from_string(String::from(OPTIONS_STR))
			.set_chart_type(ChartType::Bar);
		let new_options = serde_json::from_str::<Value>(r#"
		{
			"chart": {
				"type": "bar"
			},
			"series": [
				{
					"data": [30, 40, 35, 50, 49, 125],
					"name": "Series 1"
				}
			]
		}"#).unwrap();
		assert_eq!(serde_json::from_str::<Value>(options.options.as_str()).unwrap(), new_options);
	}

	#[wasm_bindgen_test]
	pub fn test_parsing_chart_options_from_file() {
		let file_path = "options.json";
		std::fs::write(file_path, OPTIONS_STR).unwrap();
		let options = ChartOptions::from_file(file_path);
		assert_eq!(options.options, OPTIONS_STR);
		std::fs::remove_file(file_path).unwrap();
	}
}
