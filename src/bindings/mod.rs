use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bindings/chart.js")]
extern "C" {
	/// The `ApexChart` type is a wrapper around the [ApexCharts library](https://apexcharts.com).
	/// Once created using the `new()` method, the chart should be rendered on the page using the 
	/// `render()` method before performing any other operations. Failure to do so will result in
	/// the subsequent operations not doing anything.
	///
	/// ## Usage
	///
	/// ```rust
	/// use apexcharts_rs::prelude::{ApexChart, ChartOptions};
	/// 
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
	/// let options = ChartOptions::from_file(filename);
	/// //let chart = ApexChart::new(&options.into());
	/// // chart.render("chart-id");
	/// std::fs::remove_file(filename).unwrap();
	/// ```
	pub type ApexChart;

	/// Create a new instance of the `ApexChart` type.
	#[wasm_bindgen(constructor)]
	pub fn new(options: &JsValue) -> ApexChart;

	/// The `render()` method is responsible for drawing the bindings on the page.
	/// It is the primary method that has to be called after configuring the options.
	///
	/// The `element_id` is the id of the element in which the bindings will be drawn.
	#[wasm_bindgen(method)]
	pub fn render(this: &ApexChart, element_id: &str);

	/// This method allows you to update the configuration object by passing the options as the first parameter. The new config object is merged with the existing config object preserving the existing configuration.
	///
	/// The `redraw` parameter is a boolean value that allows you to redraw the bindings after updating the configuration. Default value is `false`.
	/// The `animate` parameter is a boolean value that allows you to animate the bindings after updating the configuration. Default value is `true`.
	/// The `update_synced_charts` parameter is a boolean value that allows you to update all the charts that are in sync with the current bindings. Default value is `true`.
	#[wasm_bindgen(method)]
	pub fn update_options(this: &ApexChart, options: &JsValue, redraw: Option<bool>, animate: Option<bool>, update_synced_charts: Option<bool>);

	/// Allows you to update the series array overriding the existing one.
	/// If you want to append series to existing series, use the `append_series()` method
	#[wasm_bindgen(method)]
	pub fn update_series(this: &ApexChart, series: &JsValue, animate: Option<bool>);

	/// This method allows you to append a new series to the existing one.
	#[wasm_bindgen(method)]
	pub fn append_series(this: &ApexChart, series: &JsValue, animate: Option<bool>);

	/// This method allows you to toggle the visibility of series programmatically. Useful when you have a custom legend.
	#[wasm_bindgen(method)]
	pub fn toggle_series(this: &ApexChart, series_name: &str);

	/// This method allows you to show the hidden series. If the series is already visible, this doesn’t affect it.
	#[wasm_bindgen(method)]
	pub fn show_series(this: &ApexChart, series_name: &str);

	/// This method allows you to hide the visible series. If the series is already hidden, this method doesn’t affect it.
	#[wasm_bindgen(method)]
	pub fn hide_series(this: &ApexChart, series_name: &str);

	/// Resets all toggled series and bring back the bindings to its original state.
	#[wasm_bindgen(method)]
	pub fn reset_series(this: &ApexChart, should_update_chart: Option<bool>, should_reset_zoom: Option<bool>);

	/// Manually zoom into the bindings with the start and end X values.
	#[wasm_bindgen(method)]
	pub fn zoom_x(this: &ApexChart, min: u64, max: u64);

	/// This method allows you to append new data to the series array. If you have existing multiple series, provide the new array in the same indexed order.
	#[wasm_bindgen(method)]
	pub fn append_data(this: &ApexChart, data: &JsValue);

	/// The dataURI() method is used to get base64 dataURI. Then this dataURI can be used to generate PDF using jsPDF or can be downloaded as an image.
	#[wasm_bindgen(method)]
	pub async fn data_uri(this: &ApexChart, scale: Option<u64>, width: Option<u64>);

	/// Removes the SVG element that belongs to the bindings instance also removing all events handlers attached to it.
	#[wasm_bindgen(method)]
	pub fn destroy(this: &ApexChart);

}
