use leptos::*;
use leptos_meta::{provide_meta_context, Title, Meta, Stylesheet};

use apexcharts_rs::prelude::{ApexChartComponent, ChartSeries, ChartType, SeriesData};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let raw_options = r##"
{
  "chart": {
    "type": "area",
    "height": "100%",
    "maxWidth": "100%",
    "fontFamily": "Inter, sans-serif",
    "dropShadow": {
      "enabled": false
    },
    "toolbar": {
      "show": false
    }
  },
  "xaxis": {
    "categories": ["01 February", "02 February", "03 February", "04 February", "05 February", "06 February", "07 February"],
    "labels": {
      "show": false
    },
    "axisBorder": {
      "show": false
    },
    "axisTicks": {
      "show": false
    }
  },
    "yaxis": {
        "show": false
    },
    "legend": {
        "show": false
    },
  "stroke": {
    "width": 6,
    "curve": "smooth"
  },
  "grid": {
    "show": true,
    "strokeDashArray": 4,
    "padding": {
      "left": 2,
      "right": 2,
      "top": 0
    }
  },
  "dataLabels": {
    "enabled": false
  },
  "tooltip": {
    "enabled": true,
    "x": {
      "show": false
    }
  }
}
    "##;
    let series = vec![
        ChartSeries {
            name: "New users".to_string(),
            data: SeriesData::Single(vec![6500, 6418, 6456, 6526, 6356, 6456]),
            color: "#1A56DB".to_string(),
            r#type: None,
            z_index: None,
        }
    ];
    let multiple_series = vec![
        ChartSeries {
            name: "Clicks".to_string(),
            data: SeriesData::Single(vec![6500, 6418, 6456, 6526, 6356, 6456]),
            color: "#1A56DB".to_string(),
            r#type: None,
            z_index: None,
        },
        ChartSeries {
            name: "CPC".to_string(),
            data: SeriesData::Single(vec![6456, 6356, 6526, 6332, 6418, 6500]),
            color: "#7E3AF2".to_string(),
            r#type: None,
            z_index: None,
        }
    ];
    let radial_series = vec![
        ChartSeries {
            name: "Clicks".to_string(),
            data: SeriesData::Radial(vec![
                ("Direct".to_string(), 52.8),
                ("Organic".to_string(), 21.4),
                ("Referral".to_string(), 12.6),
                ("Social media".to_string(), 13.2)
            ]),
            color: "#1A56DB".to_string(),
            r#type: None,
            z_index: None,
        }
    ];
    let category_paired_series = vec![
        ChartSeries {
            name: "Organic".to_string(),
            data: SeriesData::CategoryPaired(vec![
                ("Mon".to_string(), 231),
                ("Tue".to_string(), 122),
                ("Wed".to_string(), 63),
                ("Thu".to_string(), 421),
                ("Fri".to_string(), 122),
                ("Sat".to_string(), 323),
                ("Sun".to_string(), 111)
            ]),
            color: "#1A56DB".to_string(),
            r#type: None,
            z_index: None,
        },
        ChartSeries {
            name: "Social media".to_string(),
            data: SeriesData::CategoryPaired(
                vec![("Mon".to_string(), 232),
                     ("Tue".to_string(), 113),
                     ("Wed".to_string(), 341),
                     ("Thu".to_string(), 224),
                     ("Fri".to_string(), 522),
                     ("Sat".to_string(), 411),
                     ("Sun".to_string(), 243)
                ]),
            color: "#FDBA8C".to_string(),
            r#type: None,
            z_index: None,
        }
    ];
    let (series_data, _) = create_signal(series);
    let (multiple_series, _) = create_signal(multiple_series);
    let (radial_series, _) = create_signal(radial_series);
    let (category_paired_series, _) = create_signal(category_paired_series);
    view! {
		<>
        <Stylesheet href="/assets/vendor/flowbite/2.3.0/flowbite.min.css" />
        <Stylesheet href="/assets/stylesheets/output.css" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="description" content="ApexCharts for Leptos SSR" />
        <Title text="ApexCharts for Leptos SSR" />
		<div class="p-4 text-center">
			<h1 class="text-3xl font-bold p-4">"ApexCharts for Leptos SSR"</h1>
		</div>
		<div class="flex flex-row p-4 flex-wrap">
                <div class="w-1/4 p-4">
                    <div class="flex flex-col flex-wrap">
                        <h1 class="text-xl font-bold mb-2">"Area chart."</h1>
                        <p class="text-gray-500 overflow-x-auto mb-2">"This is an example of an area chart. This is achieved by setting the " <code class="bg-gray-100 px-2 rounded">"type"</code> " of the chart to " <code class="bg-gray-100 px-2 rounded">"ChartType::Area"</code></p>
                        <ApexChartComponent
                            options=raw_options.to_string()
                            r#type=ChartType::Area
                            id="chart1".to_string()
                            series=series_data
                        />
                        <pre class="bg-gray-100 p-4">
                            <code class="text-sm text-gray-500 whitespace-pre">
r#"<ApexChartComponent
    options={options.to_string()}
    r#type={ChartType::Area}
    id={"chart_id".to_string()}
    series={series} />"#</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/4 p-4">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold mb-2">"Bar chart."</h1>
                        <p class="text-gray-500 mb-2">"This is an example of an Bar chart. This is achieved by setting the " <code class="bg-gray-100 px-2 rounded">"type"</code> " of the chart to " <code class="bg-gray-100 px-2 rounded">"ChartType::Bar"</code></p>
                        <ApexChartComponent
                            options=
r##"{
    "chart": {
        "sparkline": {
            "enabled": false
        },
        "toolbar": {
            "show": false
        }
    },
    "plotOptions": {
        "bar": {
            "horizontal": true,
            "columnWidth": "100%",
            "borderRadiusApplication": "end",
            "borderRadius": 6,
            "dataLabels": {
                "position": "top"
            }
        }
    },
    "tooltip": {
        "shared": true,
        "intersect": false,
        "style": {
            "fontFamily": "Inter, sans-serif"
        }
    },
    "states": {
        "hover": {
            "filter": {
                "type": "darken",
                "value": 1
            }
        }
    },
    "stroke": {
        "show": true,
        "width": 0,
        "colors": ["transparent"]
    },
    "dataLabels": {
        "enabled": false
    },
    "fill": {
        "opacity": 1
    },
    "legend": {
        "show": true,
        "position": "bottom"
    },
    "xaxis": {
        "categories": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
        "labels": {
            "show": true,
            "style": {
                "fontFamily": "Inter, sans-serif",
                "cssClass": "text-xs font-normal fill-gray-500"
            }
        },
        "axisBorder": {
            "show": false
        },
        "axisTicks": {
            "show": false
        }
    },
    "yaxis": {
        "labels": {
            "show": true,
            "style": {
                "fontFamily": "Inter, sans-serif",
                "cssClass": "text-xs font-normal fill-gray-500"
            }
        }
    },
    "grid": {
        "show": true,
        "strokeDashArray": 4,
        "padding": {
            "left": 2,
            "right": 2,
            "top": -20
        }
    }
}"##.to_string()
                            r#type=ChartType::Bar
                            height="400px".to_string()
                            id="chart2".to_string()
                            series=multiple_series
                        />
                        <pre class="bg-gray-100 p-4">
                            <code class="text-sm text-gray-500 whitespace-pre">
r#"<ApexChartComponent
    options={options.to_string()}
    r#type={ChartType::Bar}
    id={"chart_id".to_string()}
    series={series} />"#</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/4 p-4">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold mb-2">"Line chart."</h1>
                        <p class="text-gray-500 mb-2">"This is an example of an line chart. This is achieved by setting the " <code class="bg-gray-100 px-2 rounded">"type"</code> " of the chart to "<code class="bg-gray-100 px-2 rounded">"ChartType::Line"</code></p>
                        <ApexChartComponent
                            options=raw_options.to_string()
                            r#type=ChartType::Line
                            id="chart3".to_string()
                            series=multiple_series
                        />
                        <pre class="bg-gray-100 p-4">
                            <code class="text-sm text-gray-500 whitespace-pre">
r#"<ApexChartComponent
    options={options.to_string()}
    r#type={ChartType::Line}
    id={"chart_id".to_string()}
    series={series} />"#</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/4 p-4">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold mb-2">"Column chart."</h1>
                        <p class="text-gray-500 mb-2">"This is an example of an column chart. This is achieved by setting the " <code class="bg-gray-100 px-2 rounded">"type"</code> " of the chart to " <code class="bg-gray-100 px-2 rounded">"ChartType::Bar" </code>  "and set the " <code  class="bg-gray-100 px-2 rounded"> "plotOptions.bar.horizontal"</code>  " option to" <code class="bg-gray-100 px-2 rounded"> "false" </code></p>
                        <ApexChartComponent
                            options=
r##"{
    "plotOptions": {
        "bar": {
            "horizontal": false,
            "columnWidth": "70%",
            "borderRadiusApplication": "end",
            "borderRadius": 8
        }
    },
    "tooltip": {
        "shared": true,
        "intersect": false,
        "style": {
            "fontFamily": "Inter, sans-serif"
        }
    },
    "states": {
        "hover": {
            "filter": {
                "type": "darken",
                "value": 1
            }
        }
    },
    "stroke": {
        "show": true,
        "width": 0,
        "colors": ["transparent"]
    },
    "dataLabels": {
        "enabled": false
    },
    "fill": {
        "opacity": 1
    },
    "legend": {
        "show": false
    }
}"##.to_string()

                            height="320px".to_string()
                            r#type=ChartType::Bar
                            id="chart4".to_string()
                            series=category_paired_series
                        />
                        <pre class="bg-gray-100 p-4 h-64 overflow-y-scroll">
                            <code class="text-sm text-gray-500 whitespace-pre">
r###"<ApexChartComponent
    options={
        r##"{
            "plotOptions": {
                "bar": {
                    "horizontal": false,
                    "columnWidth": "70%",
                    "borderRadiusApplication": "end",
                    "borderRadius": 8
                }
            },
            "tooltip": {
                "shared": true,
                "intersect": false,
                "style": {
                    "fontFamily": "Inter, sans-serif"
                }
            },
            "states": {
                "hover": {
                    "filter": {
                        "type": "darken",
                        "value": 1
                    }
                }
            },
            "stroke": {
                "show": true,
                "width": 0,
                "colors": ["transparent"]
            },
            "dataLabels": {
                "enabled": false
            },
            "fill": {
                "opacity": 1
            },
            "legend": {
                "show": false
            }
        }"##.to_string()
    }
    r#type={ChartType::Bar}
    id={"chart_id".to_string()}
    series={series} />"###</code>
                        </pre>
                    </div>
                </div>
                <div class="w-1/4 p-4">
                    <div class="flex flex-col">
                        <h1 class="text-xl font-bold mb-2">"Pie chart."</h1>
                        <p class="text-gray-500 mb-2">"This is an example of an pie chart. This is achieved by setting the " <code>"type"</code> " of the chart to " <code>"ChartType::Pie"</code></p>
                        <ApexChartComponent
                            options=
r##"{
    "plotOptions": {
        "pie": {
            "labels": {
                "show": true
            },
            "size": "100%",
            "dataLabels": {
                "offset": -25
            }
        }
    },
    "colors": ["#1C64F2", "#16BDCA", "#9061F9"],
    "tooltip": {
        "shared": true,
        "intersect": false,
        "style": {
            "fontFamily": "Inter, sans-serif"
        }
    },
    "states": {
        "hover": {
            "filter": {
                "type": "darken",
                "value": 1
            }
        }
    },
    "stroke": {
        "lineCap": "",
        "colors": ["white"]
    },
    "dataLabels": {
        "enabled": true,
        "style": {
            "fontFamily": "Inter, sans-serif"
        }
    },
    "fill": {
        "opacity": 1
    },
    "legend": {
        "show": false
    }
}"##.to_string()

                            height="420px".to_string()
                            r#type=ChartType::Pie
                            id="chart5".to_string()
                            series=radial_series
                        />
                        <pre class="bg-gray-100 p-4 h-64 overflow-y-scroll">
                            <code class="text-sm text-gray-500 whitespace-pre">
r###"<ApexChartComponent
    options={
        r##"{
            "plotOptions": {
                "bar": {
                    "horizontal": false,
                    "columnWidth": "70%",
                    "borderRadiusApplication": "end",
                    "borderRadius": 8
                }
            },
            "tooltip": {
                "shared": true,
                "intersect": false,
                "style": {
                    "fontFamily": "Inter, sans-serif"
                }
            },
            "states": {
                "hover": {
                    "filter": {
                        "type": "darken",
                        "value": 1
                    }
                }
            },
            "stroke": {
                "show": true,
                "width": 0,
                "colors": ["transparent"]
            },
            "dataLabels": {
                "enabled": false
            },
            "fill": {
                "opacity": 1
            },
            "legend": {
                "show": false
            }
        }"##.to_string()
    }
    r#type={ChartType::Column}
    id={"chart_id".to_string()}
    series={series} />"###</code>
                        </pre>
                    </div>
                </div>
            </div>
		</>
	}
}
