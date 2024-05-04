use std::fmt::Display;
use serde_json::{to_string, to_string_pretty, Value};
use wasm_bindgen::JsValue;

/// Represents the options that can be passed to the ApexCharts constructor. This is a wrapper around
/// the JSON object that ApexCharts expects.
///
/// ## Usage
/// ```rust
/// let options = apexcharts_rs::ChartOptions::from_file("path/to/options.json");
///
/// // or
///
/// let options_str = r#"
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
/// }"#;
/// let options = apexcharts_rs::ChartOptions::from_string(String::from(options_str));
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
	/// let options = apexcharts_rs::ChartOptions::from_string(options_str);
	/// let chart = apexcharts_rs::ApexChart::new(&options.into());
	/// ```
	pub fn from_string(options: String) -> Self {
		Self {
			options
		}
	}
	/// Create a new instance of ChartOptions from a json file.
	/// ```rust
	/// let options = apexcharts_rs::ChartOptions::from_file("path/to/options.json");
	/// let chart = apexcharts_rs::ApexChart::new(&options.into());
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
		JsValue::from_str(&options.options)
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


#[cfg(test)]
mod tests {
	use serde_json::Value;
	use crate::{ChartOptions};

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
	
	#[test]
	pub fn test_parsing_chart_options() {
		let options = ChartOptions::from_string(String::from(OPTIONS_STR));
		assert_eq!(options.options, OPTIONS_STR);
	}
	
	#[test]
	pub fn test_setting_chart_type() {
		let options = ChartOptions::from_string(String::from(OPTIONS_STR))
			.set_chart_type(crate::ChartType::Bar);
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
	
	#[test]
	pub fn test_parsing_chart_options_from_file() {
		let file_path = "options.json";
		std::fs::write(file_path, OPTIONS_STR).unwrap();
		let options = ChartOptions::from_file(file_path);
		assert_eq!(options.options, OPTIONS_STR);
		std::fs::remove_file(file_path).unwrap();
	}
}