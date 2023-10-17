
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: 'シート4',
        axes: {
            left: {
                mapsTo: 'group',
                scaleType: ScaleTypes.LABELS,
            },
            bottom: {
                mapsTo: 'value',
                extendLinearDomainBy: 'marker',
                title :' 最終更新時間 2023-10-17 11:52:36'
            },
        },
        bars: {
            spacingFactor: 0.6,
            maxWidth: 300
        },
        height: "600px",
        theme: ChartTheme.G90
    }
    