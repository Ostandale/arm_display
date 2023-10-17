
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: 'シート3',
        axes: {
            left: {
                mapsTo: 'group',
                scaleType: ScaleTypes.LABELS,
            },
            bottom: {
                mapsTo: 'value',
                title: ' 最終更新時間 2023-10-17 11:52:35'
            },
        },
        bars: {
            spacingFactor: 0.15,
            maxWidth: 300
        },
        height: "700px",
        theme: ChartTheme.G90
    }
    