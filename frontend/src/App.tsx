import React, { useState, useEffect } from 'react';
import GameLibrary from './components/GameLibrary';
import LoginModal from './components/LoginModal';
import ConnectedUsers from './components/ConnectedUsers';
import { registerSession } from './services/api';
import './App.css';

function App() {
  const [token] = useState<string>('demo-token'); // In production, get from auth
  const [selectedGameId, setSelectedGameId] = useState<number | null>(null);
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [totalGames, setTotalGames] = useState<number>(0);
  const [username, setUsername] = useState<string | null>(null);
  const [showLogin, setShowLogin] = useState(true);

  // Load username and theme from localStorage on mount
  useEffect(() => {
    const savedTheme = localStorage.getItem('theme') as 'dark' | 'light' | null;
    if (savedTheme) {
      setTheme(savedTheme);
    }
    
    const savedUsername = localStorage.getItem('username');
    if (savedUsername) {
      setUsername(savedUsername);
      setShowLogin(false);
    }
  }, []);

  // Apply theme to document
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  // Register session periodically when logged in
  useEffect(() => {
    if (!username) return;

    // Register immediately
    registerSession(username).catch(err => {
      console.error('Failed to register session:', err);
    });

    // Then register every 30 seconds to keep session alive
    const interval = setInterval(() => {
      registerSession(username).catch(err => {
        console.error('Failed to register session:', err);
      });
    }, 30000);

    return () => clearInterval(interval);
  }, [username]);

  const handleLogin = (loggedInUsername: string) => {
    setUsername(loggedInUsername);
    setShowLogin(false);
    localStorage.setItem('username', loggedInUsername);
    // Register session immediately after login
    registerSession(loggedInUsername).catch(err => {
      console.error('Failed to register session:', err);
    });
  };

  const toggleTheme = () => {
    setTheme(prevTheme => prevTheme === 'dark' ? 'light' : 'dark');
  };

  const handleGamesCountUpdate = (count: number) => {
    setTotalGames(count);
  };

  return (
    <div className="App">
      {showLogin && <LoginModal onLogin={handleLogin} />}
      <header className="App-header">
        <div className="header-content">
          <div className="logo">
            <div className="logo-icon">🎮</div>
            <div className="header-text">
              <h1>Gamers Unite</h1>
              <p>Multi-system emulator collection</p>
            </div>
          </div>
        </div>
        <div className="header-controls">
          {username && <ConnectedUsers />}
          {totalGames > 0 && (
            <div className="stats-badge">
              <span>Total Games:</span>
              <span className="stats-number">{totalGames}</span>
            </div>
          )}
          <button 
            className="theme-toggle" 
            onClick={toggleTheme}
            aria-label="Toggle theme"
          >
            <span>{theme === 'dark' ? '☀️' : '🌙'}</span>
            <span>{theme === 'dark' ? 'Light' : 'Dark'}</span>
          </button>
        </div>
      </header>
      {!showLogin && (
        <main>
          <GameLibrary 
            token={token} 
            onSelectGame={setSelectedGameId}
            onGamesCountUpdate={handleGamesCountUpdate}
          />
        </main>
      )}
    </div>
  );
}

export default React.memo(App);

