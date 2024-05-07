use wasm_bindgen::JsValue;
use gloo::timers::callback::Timeout;
use serde_json::Value;
use yew::prelude::*;
use crate::prelude::{ApexChart, ChartSeries, ChartType, SeriesData};

pub struct ApexChartComponent {
	chart: ApexChart,
	_draw_timeout: Timeout,
}

pub enum ApexChartComponentMsg {
	DrawChart
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ApexChartComponentProps {
	#[prop_or(ChartType::Line)]
	pub r#type: ChartType,
	#[prop_or("".into())]
	pub options: String,
	pub id: String,
	pub series: Vec<ChartSeries>,
	#[prop_or("100%".to_string())]
	pub width: String,
	#[prop_or("auto".to_string())]
	pub height: String,
}

impl Component for ApexChartComponent {
	type Message = ApexChartComponentMsg;
	type Properties = ApexChartComponentProps;

	fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link();
		let stand_alone_timer = {
			let link = link.clone();
			Timeout::new(10, move||{
				link.send_message(ApexChartComponentMsg::DrawChart);
			})
		};
		let props = ctx.props().clone();
		let mut labels_data = None;
		let series_data = match props.r#type {
			ChartType::Pie | ChartType::Donut | ChartType::RadialBar => {
				let chart_serie = props.series.first().unwrap();
				match chart_serie.data {
					SeriesData::Radial(ref data) => {
						let data_values = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();
						labels_data = Some(data.iter().map(|(x, _)| x.clone()).collect::<Vec<_>>());
						serde_json::to_value(data_values).unwrap()
					},
					_=> {
						serde_json::to_value(&props.series).unwrap()
					}
				}
			},
			_=> {
				serde_json::to_value(&props.series).unwrap()
			}
		};
		let options = if props.options.is_empty() {
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
				props.r#type,
				props.width,
				props.height,
				series_data,
				if let Some(labels) = labels_data {
					format!(r#","labels": {}"#, serde_json::to_string(&labels).unwrap())
				} else {
					"".to_string()
				}
			)
		} else {
			let mut options = serde_json::from_str::<serde_json::Value>(&props.options).unwrap();
			options["chart"]["type"] = Value::String(props.r#type.to_string());
			options["chart"]["width"] = Value::String(props.width.clone());
			options["chart"]["height"] = Value::String(props.height.clone());
			options["series"] = series_data;
			if let Some(labels) = labels_data {
				options["labels"] = Value::Array(labels.iter().map(|label| Value::String(label.clone())).collect());
			}
			serde_json::to_string(&options).unwrap()
		};
		Self {
			chart: ApexChart::new(&JsValue::from_str(&options)),
			_draw_timeout: stand_alone_timer,
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			ApexChartComponentMsg::DrawChart => {
				self.chart.render(&ctx.props().id.clone());
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
            <div id={ctx.props().id.clone()}></div>
        }
	}
}
