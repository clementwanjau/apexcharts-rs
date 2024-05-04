use std::fmt::Display;
use serde_json::Value;
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
/// let options = apexcharts_rs::ChartOptions(String::from(options_str));
pub struct ChartOptions(String);

impl ChartOptions {
	/// Create a new instance of ChartOptions from a json file.
	/// ```rust
	/// let options = apexcharts_rs::ChartOptions::from_file("path/to/options.json");
	/// let chart = apexcharts_rs::ApexChart::new(&options.into());
	/// ```
	pub fn from_file(file_path: &str) -> Self {
		// Read the file and return the content as a string
		let content = std::fs::read_to_string(file_path).unwrap();
		Self(content)
	}

	/// Set the type of the chart. This overrides the type set in the options.
	pub fn set_chart_type(&self, chart_type: ChartType) -> Self {
		let chart_type = chart_type.to_string();
		let mut content: Value = serde_json::from_str(&self.0).unwrap();
		content["chart"]["type"] = Value::String(chart_type);
		Self(serde_json::to_string_pretty(&content).unwrap())
	}
}

impl From<ChartOptions> for JsValue {
	fn from(options: ChartOptions) -> JsValue {
		JsValue::from_str(&options.0)
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
