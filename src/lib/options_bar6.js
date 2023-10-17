
import { ScaleTypes } from "@carbon/charts-svelte";
import { ChartTheme } from "@carbon/charts-svelte";
export default {
    title: 'シート6',
    axes: {
        left: {
          mapsTo: 'value1',
          scaleType: 'linear',
          title: '左縦軸タイトル'
        },
        right: {
          mapsTo: 'value2',
          scaleType: 'linear',
          title: '右縦軸タイトル',
          correspondingDatasets: [
            'グラフB'
          ]
        },
        bottom: {
          mapsTo: 'date',
          scaleType: 'labels',
          title: ' 最終更新時間 2023-10-17 15:30:57'
        },
    },
    comboChartTypes: [
    {
      type: 'simple-bar',
      correspondingDatasets: [
        'グラフA'
      ]
    },
    {
      type: 'line',
      options: {
        points: {
          radius: 5
        }
      },
      correspondingDatasets: [
        'グラフB'
      ]
    }
  ],
    bars: {
        spacingFactor: 0.15,
        maxWidth: 300
    },
    height: "700px",
    theme: ChartTheme.G90
}
