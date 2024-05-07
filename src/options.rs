use indexmap::IndexMap;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde::ser::{SerializeSeq};
use serde_json::{Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys;

/// Represents the type of the chart that will be rendered.
#[derive(Clone, Debug, PartialEq, Deserialize)]
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

impl Serialize for ChartType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

/// Represents the data that will be rendered in the chart.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum SeriesData {
	/// Represents a single array of data points. eg `[10, 20, 30]`
	Single(Vec<i64>),
	/// Represents a double array of data points. eg `[(10, 20), (20, 30)]`
	NumericPaired(Vec<(i64, i64)>),
	/// Represents a double array of data points with a category. eg `[("Apple", 30), ("Banana", 40)]`
	CategoryPaired(Vec<(String, i64)>),
	/// Represents a double array of data points with a timestamp. eg `[(1619683200, 30), (1619769600, 40)]`
	Timestamped(Vec<(i64, i64)>),
	/// Represents a double array of data points with a date. eg `[("2021-04-29", 30), ("2021-04-30", 40)]`
	Dated(Vec<(String, i64)>),
	/// Represents a double array of data points which is a percentage adding up to 100. eg `[("Apple", 30.0), ("Banana", 40.0), ("Orange", 30.0)]`. It is used primarily for the `Pie`, `Donut`, and `Radial` chart types.
	Radial(Vec<(String, f64)>),
}

impl Serialize for SeriesData {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
	{
		match self {
			SeriesData::Single(data) => {
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				for item in data {
					seq.serialize_element(item)?;
				}
				seq.end()
			}
			SeriesData::NumericPaired(data) => {
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				let data = data.iter().map(|(x, y)| vec![*x, *y]).collect::<Vec<_>>();
				for item in data {
					seq.serialize_element(&item)?;
				}
				seq.end()
			}
			SeriesData::CategoryPaired(data) => {
				// Serialize the data into a sequence of an object with two properties `x` and `y`. eg `[{x: "Apple", y: 30}, {x: "Banana", y: 40}]`
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				let data: Vec<IndexMap<String, serde_json::Value>> = data.iter().map(|(x, y)| {
					IndexMap::from_iter(
						vec![
							("x".to_string(), Value::String(x.to_string())),
							("y".to_string(), Value::Number(serde_json::Number::from(*y)))
						]
					)
				}).collect::<Vec<_>>();
				for item in data {
					seq.serialize_element(&item)?;
				}
				seq.end()
			}
			SeriesData::Timestamped(data) => {
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				let data = data.iter().map(|(x, y)| vec![*x, *y]).collect::<Vec<_>>();
				for item in data {
					seq.serialize_element(&item)?;
				}
				seq.end()
			}
			SeriesData::Dated(data) => {
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				let data: Vec<IndexMap<String, serde_json::Value>> = data.iter().map(|(x, y)| {
					IndexMap::from_iter(
						vec![
							("x".to_string(), Value::String(x.to_string())),
							("y".to_string(), Value::Number(serde_json::Number::from(*y)))
						]
					)
				}).collect::<Vec<_>>();
				for item in data {
					seq.serialize_element(&item)?;
				}
				seq.end()
			},
			SeriesData::Radial(data) => {
				let mut seq = serializer.serialize_seq(Some(data.len()))?;
				let data: Vec<IndexMap<String, serde_json::Value>> = data.iter().map(|(x, y)| {
					IndexMap::from_iter(
						vec![
							("x".to_string(), Value::String(x.to_string())),
							("y".to_string(), Value::Number(serde_json::Number::from_f64(*y).unwrap()))
						]
					)
				}).collect::<Vec<_>>();
				for item in data {
					seq.serialize_element(&item)?;
				}
				seq.end()
			}
		}
	}

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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ChartType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub z_index: Option<i32>,
}

impl From<ChartSeries> for JsValue {
	fn from(chart_series: ChartSeries) -> JsValue {
		let series = js_sys::Object::new();
		js_sys::Reflect::set(&series, &JsValue::from_str("name"), &JsValue::from_str(&chart_series.name)).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("data"), &serde_wasm_bindgen::to_value(&chart_series.data).unwrap()).unwrap();
		js_sys::Reflect::set(&series, &JsValue::from_str("color"), &JsValue::from_str(&chart_series.color)).unwrap();

		if let Some(series_type) = chart_series.r#type {
			js_sys::Reflect::set(&series, &JsValue::from_str("type"), &JsValue::from_str(&series_type.to_string())).unwrap();
		}
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
	use crate::prelude::SeriesData;

	#[test]
	pub fn test_series_data_serialization() {
		let single_data = serde_json::to_string(&SeriesData::Single(vec![10, 20, 30])).unwrap();
		assert_eq!(single_data, "[10,20,30]");

		let numeric_paired_data = serde_json::to_string(&SeriesData::NumericPaired(vec![(10, 20), (20, 30)])).unwrap();
		assert_eq!(numeric_paired_data, "[[10,20],[20,30]]");

		let category_paired_data = serde_json::to_string(&SeriesData::CategoryPaired(vec![("Apple".to_string(), 30), ("Banana".to_string(), 40)])).unwrap();
		assert_eq!(category_paired_data, r#"[{"x":"Apple","y":30},{"x":"Banana","y":40}]"#);

		let timestamped_data = serde_json::to_string(&SeriesData::Timestamped(vec![(1619683200, 30), (1619769600, 40)])).unwrap();
		assert_eq!(timestamped_data, "[[1619683200,30],[1619769600,40]]");

		let dated_data = serde_json::to_string(&SeriesData::Dated(vec![("2021-04-29".to_string(), 30), ("2021-04-30".to_string(), 40)])).unwrap();
		assert_eq!(dated_data, r#"[{"x":"2021-04-29","y":30},{"x":"2021-04-30","y":40}]"#);
	}

}
