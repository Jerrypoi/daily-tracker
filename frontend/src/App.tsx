import { NavLink, Outlet } from 'react-router-dom'

function App() {
  return (
    <main className="app-shell">
      <header>
        <h1>Daily Tracker</h1>
        <nav className="nav">
          <NavLink
            to="/topics"
            className={({ isActive }) => (isActive ? 'active' : undefined)}
          >
            Topics
          </NavLink>
          <NavLink
            to="/daily-tracks"
            className={({ isActive }) => (isActive ? 'active' : undefined)}
          >
            Daily Tracks
          </NavLink>
        </nav>
      </header>
      <Outlet />
    </main>
  )
}

export default App
