import ApexCharts from "../../apexcharts/dist/apexcharts";

export class ApexChart {
    chart;

    constructor(options) {
        this.options = options;
    }

    update_series(series, animate) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.updateSeries(series, animate);
    }

    render(element_id) {
        const element = document.getElementById(element_id);
        this.chart = new ApexCharts(element, this.options);
        this.chart.render();
    }

    update_options(options, redraw_paths, animate, update_synced_charts) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.updateOptions(options, redraw_paths, animate, update_synced_charts);
    }

    append_series(series, animate) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.appendData(series, animate);
    }
    
    toggle_series(series_name) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.toggleSeries(series_name);
    }

    show_series(series_name) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.showSeries(series_name);
    }

    hide_series(series_name) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.hideSeries(series_name);
    }

    zoom_x(start, end) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.zoomX(start, end);
    }

    reset_series(should_update_chart, should_reset_zoom) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.resetSeries(should_update_chart, should_reset_zoom);
    }

    append_data(data) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.appendData(data);
    }

    data_uri(scale, width) {
        if (this.chart === undefined) {
            return;
        }
        return this.chart.dataURI({scale, width});
    }

    toggle_data_point_selection(series_index, data_point_index) {
        if (this.chart === undefined) {
            return;
        }
        this.chart.toggleDataPointSelection(series_index, data_point_index);
    }

    destroy() {
        if (this.chart === undefined) {
            return;
        }
        this.chart.destroy();
    }
}
