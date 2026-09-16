import { useState } from 'react'
import { BrowserRouter as Router, Routes, Route, Link, useLocation } from 'react-router-dom'
import { Layout, Menu, theme } from 'antd'
import {
  DatabaseOutlined,
  LineChartOutlined,
  SettingOutlined,
  BranchesOutlined,
  RocketOutlined
} from '@ant-design/icons'
import DataManagement from './pages/DataManagement'
import Analysis from './pages/Analysis'
import Versions from './pages/Versions'
import Settings from './pages/Settings'
import Augmentation from './pages/Augmentation'

const { Header, Sider, Content } = Layout

const menuItems = [
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
    key: '/analysis',
    icon: <LineChartOutlined />,
    label: '数据分析'
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
          {collapsed ? 'AI' : 'AI 数据增强'}
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
          <h2 style={{ margin: 0 }}>AI 训练数据增强工具</h2>
        </Header>
        <Content style={{ margin: 24 }}>
          <div style={{
            padding: 24,
            minHeight: 360,
            background: colorBgContainer,
            borderRadius: borderRadiusLG
          }}>
            <Routes>
              <Route path="/" element={<DataManagement />} />
              <Route path="/augment" element={<Augmentation />} />
              <Route path="/analysis" element={<Analysis />} />
              <Route path="/versions" element={<Versions />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
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
