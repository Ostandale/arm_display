
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: 'シート2',
        axes: {
            left: {
                mapsTo: 'group',
                scaleType: ScaleTypes.LABELS,
            },
            bottom: {
                mapsTo: 'value',
                title: ' 最終更新時間 2023-10-17 15:30:55'
            },
        },
        bars: {
            spacingFactor: 0.15,
            maxWidth: 300
        },
        height: "600px",
        theme: ChartTheme.G90
    }
    