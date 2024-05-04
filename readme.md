# Apexcharts-rs

This is a Rust WASM bindings for generating charts using the [ApexCharts](https://apexcharts.com/) JavaScript library.

## Usage
```toml
[dependencies]
apexcharts-rs = "0.1.0"
```
and then in your code:

```rust
use apexcharts_rs::ApexCharts;

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
    let options = ChartOptions(String::from(options));
    let chart = ApexCharts::new(&options.into());
    chart.render("chart_el_id");
}
```


License: MIT

