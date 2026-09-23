import { useState, Suspense, lazy } from 'react'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom'
import { Layout, Menu, theme, Spin } from 'antd'
import {
  DashboardOutlined,
  DatabaseOutlined,
  LineChartOutlined,
  SettingOutlined,
  BranchesOutlined,
  RocketOutlined,
  SafetyCertificateOutlined,
  ExportOutlined,
  PictureOutlined,
  LockOutlined,
  MonitorOutlined
} from '@ant-design/icons'

const DataManagement = lazy(() => import('./pages/DataManagement'))
const Analysis = lazy(() => import('./pages/Analysis'))
const Versions = lazy(() => import('./pages/Versions'))
const Settings = lazy(() => import('./pages/Settings'))
const Augmentation = lazy(() => import('./pages/Augmentation'))
const Dashboard = lazy(() => import('./pages/Dashboard'))
const Quality = lazy(() => import('./pages/Quality'))
const Export = lazy(() => import('./pages/Export'))
const Multimodal = lazy(() => import('./pages/Multimodal'))
const Security = lazy(() => import('./pages/Security'))
const System = lazy(() => import('./pages/System'))

const { Header, Sider, Content } = Layout

const menuItems = [
  {
    key: '/dashboard',
    icon: <DashboardOutlined />,
    label: '仪表盘'
  },
  {
    key: '/',
    icon: <DatabaseOutlined />,
    label: '数据管理'
  },
  {
    key: '/augment',
    icon: <RocketOutlined />,
    label: '数据增强'
  },
  {
    key: '/quality',
    icon: <SafetyCertificateOutlined />,
    label: '质量中心'
  },
  {
    key: '/security',
    icon: <LockOutlined />,
    label: '数据安全'
  },
  {
    key: '/export',
    icon: <ExportOutlined />,
    label: '导出中心'
  },
  {
    key: '/analysis',
    icon: <LineChartOutlined />,
    label: '数据分析'
  },
  {
    key: '/multimodal',
    icon: <PictureOutlined />,
    label: '多模态'
  },
  {
    key: '/versions',
    icon: <BranchesOutlined />,
    label: '版本管理'
  },
  {
    key: '/settings',
    icon: <SettingOutlined />,
    label: '配置管理'
  },
  {
    key: '/system',
    icon: <MonitorOutlined />,
    label: '系统状态'
  }
]

function AppLayout() {
  const [collapsed, setCollapsed] = useState(false)
  const location = useLocation()
  const {
    token: { colorBgContainer, borderRadiusLG }
  } = theme.useToken()

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={(value) => setCollapsed(value)}
      >
        <div style={{ height: 32, margin: 16, background: 'rgba(255, 255, 255, 0.2)', borderRadius: 6, display: 'flex', alignItems: 'center', justifyContent: 'center', color: 'white', fontWeight: 'bold' }}>
          {collapsed ? 'AI' : 'AI 数据平台'}
        </div>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[location.pathname]}
          items={menuItems}
        />
      </Sider>
      <Layout>
        <Header style={{ padding: '0 16px', background: colorBgContainer, display: 'flex', alignItems: 'center' }}>
          <h2 style={{ margin: 0 }}>企业级 AI 训练数据平台</h2>
        </Header>
        <Content style={{ margin: 24 }}>
          <div style={{
            padding: 24,
            minHeight: 360,
            background: colorBgContainer,
            borderRadius: borderRadiusLG
          }}>
            <Suspense fallback={<Spin style={{ display: 'block', margin: '40px auto' }} />}>
              <Routes>
                <Route path="/dashboard" element={<Dashboard />} />
                <Route path="/" element={<DataManagement />} />
                <Route path="/augment" element={<Augmentation />} />
                <Route path="/quality" element={<Quality />} />
                <Route path="/security" element={<Security />} />
                <Route path="/export" element={<Export />} />
                <Route path="/analysis" element={<Analysis />} />
                <Route path="/multimodal" element={<Multimodal />} />
                <Route path="/versions" element={<Versions />} />
                <Route path="/settings" element={<Settings />} />
                <Route path="/system" element={<System />} />
              </Routes>
            </Suspense>
          </div>
        </Content>
      </Layout>
    </Layout>
  )
}

function App() {
  return (
    <Router>
      <AppLayout />
    </Router>
  )
}

export default App
