import React, { useState, useEffect } from 'react';
import { getGames, getEmulators } from '../services/api';
import GameCard from './GameCard';
import './GameLibrary.css';

interface Game {
  id: number;
  title: string;
  system: string;
  file_path: string;
  emulator_id: string;
  emulator: {
    name: string;
    system: string;
    github_url: string;
  };
  launch_url: string;
}

interface Emulator {
  id: string;
  name: string;
  system: string;
}

const GameLibrary: React.FC<{ token: string; onSelectGame: (id: number) => void }> = ({ 
  token, 
  onSelectGame 
}) => {
  const [games, setGames] = useState<Game[]>([]);
  const [emulators, setEmulators] = useState<Emulator[]>([]);
  const [filteredGames, setFilteredGames] = useState<Game[]>([]);
  const [selectedSystem, setSelectedSystem] = useState<string>('all');
  const [selectedEmulator, setSelectedEmulator] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');

  useEffect(() => {
    loadData();
  }, []);

  useEffect(() => {
    filterGames();
  }, [games, selectedSystem, selectedEmulator, searchQuery]);

  const loadData = async () => {
    try {
      const [gamesData, emulatorsData] = await Promise.all([
        getGames(token),
        getEmulators(token)
      ]);
      setGames(gamesData);
      setEmulators(emulatorsData);
    } catch (error) {
      console.error('Failed to load data:', error);
    }
  };

  const filterGames = () => {
    let filtered = games;

    if (selectedSystem !== 'all') {
      filtered = filtered.filter(g => g.system === selectedSystem);
    }

    if (selectedEmulator !== 'all') {
      filtered = filtered.filter(g => g.emulator_id === selectedEmulator);
    }

    if (searchQuery) {
      filtered = filtered.filter(g => 
        g.title.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    setFilteredGames(filtered);
  };

  const systems = Array.from(new Set(games.map(g => g.system)));

  return (
    <div className="game-library">
      <div className="filters">
        <input
          type="text"
          placeholder="Search games..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="search-input"
        />
        
        <select 
          value={selectedSystem} 
          onChange={(e) => setSelectedSystem(e.target.value)}
          className="filter-select"
        >
          <option value="all">All Systems</option>
          {systems.map(system => (
            <option key={system} value={system}>{system}</option>
          ))}
        </select>

        <select 
          value={selectedEmulator} 
          onChange={(e) => setSelectedEmulator(e.target.value)}
          className="filter-select"
        >
          <option value="all">All Emulators</option>
          {emulators.map(emu => (
            <option key={emu.id} value={emu.id}>{emu.name}</option>
          ))}
        </select>
      </div>

      <div className="games-grid">
        {filteredGames.map(game => (
          <GameCard 
            key={game.id} 
            game={game} 
            onClick={() => onSelectGame(game.id)} 
          />
        ))}
      </div>

      {filteredGames.length === 0 && (
        <div className="empty-state">
          <p>No games found. Add ROMs to the /roms directory.</p>
        </div>
      )}
    </div>
  );
};

export default GameLibrary;

