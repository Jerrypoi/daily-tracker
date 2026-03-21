<<<<<<< Current (Your changes)
=======
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
      { index: true, element: <Navigate to="/topics" replace /> },
      { path: 'topics', element: <TopicsPage /> },
      { path: 'daily-tracks', element: <DailyTracksPage /> },
    ],
  },
])

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
)
>>>>>>> Incoming (Background Agent changes)
