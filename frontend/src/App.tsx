import React, { useState, useEffect } from 'react';
import GameLibrary from './components/GameLibrary';
import './App.css';

function App() {
  const [token] = useState<string>('demo-token'); // In production, get from auth
  const [selectedGameId, setSelectedGameId] = useState<number | null>(null);
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [totalGames, setTotalGames] = useState<number>(0);

  // Load theme from localStorage on mount
  useEffect(() => {
    const savedTheme = localStorage.getItem('theme') as 'dark' | 'light' | null;
    if (savedTheme) {
      setTheme(savedTheme);
    }
  }, []);

  // Apply theme to document
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme(prevTheme => prevTheme === 'dark' ? 'light' : 'dark');
  };

  const handleGamesCountUpdate = (count: number) => {
    setTotalGames(count);
  };

  return (
    <div className="App">
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
      <main>
        <GameLibrary 
          token={token} 
          onSelectGame={setSelectedGameId}
          onGamesCountUpdate={handleGamesCountUpdate}
        />
      </main>
    </div>
  );
}

export default React.memo(App);

