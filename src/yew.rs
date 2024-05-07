use wasm_bindgen::JsValue;
use gloo::timers::callback::Timeout;
use serde_json::Value;
use yew::prelude::*;
use crate::prelude::{ApexChart, ChartSeries, ChartType};

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
		let options = if props.options.is_empty() {
			format!(
				r#"{{
				"chart": {{
					"type": "{}",
					"width": "{}",
					"height": "{}"
				}},
				"series": {}
			}}"#,
				props.r#type,
				props.width,
				props.height,
				serde_json::to_string(&props.series).unwrap()
			)
		} else {
			let mut options = serde_json::from_str::<serde_json::Value>(&props.options).unwrap();
			options["chart"]["type"] = Value::String(props.r#type.to_string());
			options["chart"]["width"] = Value::String(props.width.clone());
			options["chart"]["height"] = Value::String(props.height.clone());
			options["series"] = serde_json::to_value(&props.series).unwrap();
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
