use wasm_bindgen::JsValue;
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use crate::prelude::ApexChart;

pub struct ApexChartComponent {
	chart: ApexChart,
	draw_timeout: Timeout,
}

pub enum ApexChartComponentMsg {
	DrawChart
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ApexChartComponentProps {
	pub options: JsValue,
	pub id: String,
	//pub data: Option<Vec<SeriesData>>
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
		Self {
			chart: ApexChart::new(&props.options),
			draw_timeout: stand_alone_timer,
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			ApexChartComponentMsg::DrawChart => {
				self.chart.render("chart");
				true
			}
		}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
            <div id="chart"></div>
        }
	}
}
