import argparse
import pandas as pd
import pyarrow as pa
import plotly.graph_objects as go
from plotly.subplots import make_subplots
import panel as pn
import json

class MmssVisualizer:
    def __init__(self, file_path):
        self.file_path = file_path
        self.df = self._load_data()
        # Parse JSON strings in payload
        self.df['payload'] = self.df['payload'].apply(lambda x: json.loads(x) if isinstance(x, str) else x)
    
    def _load_data(self):
        with pa.OSFile(self.file_path, "rb") as f:
            reader = pa.ipc.open_file(f)
            table = reader.read_all()
            return table.to_pandas()
    
    def plot_timeseries(self, title="MMSS Metrics"):
        fig = make_subplots(specs=[[{"secondary_y": True}]])
        for kind in self.df["kind"].unique():
            df_kind = self.df[self.df["kind"] == kind]
            fig.add_trace(go.Scatter(
                x=pd.to_datetime(df_kind["timestamp"], unit="s"),
                y=df_kind["payload"].apply(lambda x: x["value"]),
                name=kind,
                mode="lines+markers"
            ))
        fig.update_layout(
            title=title,
            xaxis_title="Time",
            yaxis_title="Value",
            legend_title="Metrics",
            hovermode="x unified"
        )
        return fig
    
    def interactive_dashboard(self, port=5006):
        pn.extension("plotly")
        metric_selector = pn.widgets.MultiSelect(
            name="Metrics",
            options=list(self.df["kind"].unique()),
            value=list(self.df["kind"].unique())[:2]
        )
        
        @pn.depends(metric_selector.param.value)
        def update_plot(metrics):
            fig = self.plot_timeseries()
            fig.data = [trace for trace in fig.data if trace.name in metrics]
            return fig
        
        dashboard = pn.Column(
            pn.Row(
                pn.Column("### MMSS Dashboard", metric_selector),
                pn.Spacer(width=20)
            ),
            pn.Row(pn.panel(update_plot, sizing_mode="stretch_both")),
            sizing_mode="stretch_both"
        )
        dashboard.servable()
        pn.serve(dashboard, port=port, show=False)

def main():
    parser = argparse.ArgumentParser(description="MMSS Data Visualizer")
    parser.add_argument("input", help="Path to Arrow file")
    parser.add_argument("--dashboard", action="store_true", help="Launch interactive dashboard")
    parser.add_argument("--port", type=int, default=5006, help="Port for the dashboard")
    args = parser.parse_args()
    
    viz = MmssVisualizer(args.input)
    if args.dashboard:
        viz.interactive_dashboard(port=args.port)
    else:
        fig = viz.plot_timeseries()
        fig.show()

if __name__ == "__main__":
    main()
