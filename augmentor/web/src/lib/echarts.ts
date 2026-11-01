// ECharts 按需引入：只注册本项目用到的图表与组件，显著减小 vendor chunk。
// 用法：`import ReactEChartsCore from 'echarts-for-react/lib/core'` +
// `import echarts from '../lib/echarts'`，然后 `<ReactEChartsCore echarts={echarts} ... />`。
import * as echarts from 'echarts/core'
import { BarChart, PieChart, RadarChart } from 'echarts/charts'
import {
  GridComponent,
  LegendComponent,
  TitleComponent,
  TooltipComponent,
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'

echarts.use([
  BarChart,
  PieChart,
  RadarChart,
  GridComponent,
  LegendComponent,
  TitleComponent,
  TooltipComponent,
  CanvasRenderer,
])

export default echarts
