import { NavLink, Outlet, Navigate } from 'react-router-dom'
import { useAuth } from './components/AuthContext'

function App() {
  const { isAuthenticated, logout } = useAuth();
  
  if (!isAuthenticated) {
      return <Navigate to="/login" replace />;
  }

  return (
    <main className="app-shell">
      <header>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <h1>Daily Tracker</h1>
            <button onClick={logout} style={{ padding: '8px 16px', background: '#ef4444', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Logout</button>
        </div>
        <nav className="nav">
          <NavLink
            to="/daily-tracks"
            className={({ isActive }) => (isActive ? 'active' : undefined)}
          >
            Daily Tracks
          </NavLink>
          <NavLink
            to="/topics"
            className={({ isActive }) => (isActive ? 'active' : undefined)}
          >
            Topics
          </NavLink>
          <NavLink
            to="/api-keys"
            className={({ isActive }) => (isActive ? 'active' : undefined)}
          >
            API Keys
          </NavLink>
        </nav>
      </header>
      <Outlet />
    </main>
  )
}

export default App
