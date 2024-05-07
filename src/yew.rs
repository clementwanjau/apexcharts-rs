use wasm_bindgen::JsValue;
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use crate::prelude::{ApexChart, ChartSeries, ChartType};

pub struct ApexChartComponent {
	chart: ApexChart,
	draw_timeout: Timeout,
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
		let options = format!(
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
			serde_json::to_string(&props.series).unwrap(),
			if props.options.is_empty() {
				"".to_string()
			} else {
				format!(",{}", props.options)
			}
		);
		Self {
			chart: ApexChart::new(&JsValue::from_str(&options)),
			draw_timeout: stand_alone_timer,
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
