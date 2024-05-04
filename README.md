# Apexcharts-rs
[![Crates.io](https://img.shields.io/crates/v/apexcharts-rs)](https://crates.io/crates/apexcharts-rs)
[![Build](https://github.com/clementwanjau/apexcharts-rs/actions/workflows/build.yaml/badge.svg)](https://github.com/clementwanjau/apexcharts-rs/actions/workflows/build.yaml)

This is a Rust WASM bindings for generating charts using the [ApexCharts](https://apexcharts.com/) JavaScript library.

## Usage
```toml
[dependencies]
apexcharts-rs = "0.1.3"
```
and then in your code:

```rust
use apexcharts_rs::prelude::{ChartOptions};

fn main() {
    let options = r#"
        {
            chart: {
                type: 'bar'
            },
            series: [{
                name: 'sales',
                data: [30,40,35,50,49,60,70,91,125]
            }],
            xaxis: {
                categories: [1991,1992,1993,1994,1995,1996,1997,1998,1999]
            }
        }
    "#;
    let options = ChartOptions::from_string(String::from(options));
    //let chart = ApexChart::new(&options.into());
    //chart.render("chart_el_id");
}
```

## License

This project is licensed under the `Apache License 2.0` - see the [LICENSE](LICENSE) file for details

