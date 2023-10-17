
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: 'シート1',
        axes: {
            left: {
                mapsTo: 'value'
            },
            bottom: {
                scaleType: ScaleTypes.LABELS,
                mapsTo: 'group',
                title: ' 最終更新時間 2023-10-17 11:52:34'
            },
        },
        bars: {
            spacingFactor: 0.6,
            maxWidth: 100
        },
        height: "400px",
        theme: ChartTheme.G90
    }
    