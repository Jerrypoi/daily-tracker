import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Navigate, RouterProvider, createBrowserRouter } from 'react-router-dom'

import { initializeApiConfig } from './api/config'
import './index.css'
import App from './App.tsx'
import { DailyTracksPage } from './pages/DailyTracksPage'
import { TopicsPage } from './pages/TopicsPage'
import { ApiKeysPage } from './pages/ApiKeysPage'
import { Login } from './pages/Login'
import { Register } from './pages/Register'
import { VerifyEmail } from './pages/VerifyEmail'
import { AuthProvider } from './components/AuthContext'

initializeApiConfig()

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      { path: 'daily-tracks', element: <DailyTracksPage /> },
      { path: 'topics', element: <TopicsPage /> },
      { path: 'api-keys', element: <ApiKeysPage /> },
      { index: true, element: <Navigate to="/daily-tracks" replace /> },
    ],
  },
  { path: '/login', element: <Login /> },
  { path: '/register', element: <Register /> },
  { path: '/verify-email', element: <VerifyEmail /> },
])

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthProvider>
      <RouterProvider router={router} />
    </AuthProvider>
  </StrictMode>,
)
