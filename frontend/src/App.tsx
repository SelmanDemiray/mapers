import React, { useState } from 'react';
import GameLibrary from './components/GameLibrary';
import './App.css';

function App() {
  const [token] = useState<string>('demo-token'); // In production, get from auth
  const [selectedGameId, setSelectedGameId] = useState<number | null>(null);

  return (
    <div className="App">
      <header className="App-header">
        <h1>Emulator Platform</h1>
        <p>Multi-system emulator collection</p>
      </header>
      <main>
        <GameLibrary 
          token={token} 
          onSelectGame={setSelectedGameId}
        />
      </main>
    </div>
  );
}

export default App;

