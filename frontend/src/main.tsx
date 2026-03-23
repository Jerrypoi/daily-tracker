import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Navigate, RouterProvider, createBrowserRouter } from 'react-router-dom'

import { initializeApiConfig } from './api/config'
import './index.css'
import App from './App.tsx'
import { DailyTracksPage } from './pages/DailyTracksPage'
import { TopicsPage } from './pages/TopicsPage'

initializeApiConfig()

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      { path: 'daily-tracks', element: <DailyTracksPage /> },
      { path: 'topics', element: <TopicsPage /> },
      { index: true, element: <Navigate to="/daily-tracks" replace /> },
    ],
  },
])

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
)
