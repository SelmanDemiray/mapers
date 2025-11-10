import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { getGames, getEmulators } from '../services/api';
import GameCard from './GameCard';
import RomUpload from './RomUpload';
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

type SortOption = 'title-asc' | 'title-desc' | 'system-asc' | 'system-desc';
type ViewMode = 'grid' | 'list';

interface GameLibraryProps {
  token: string;
  onSelectGame: (id: number) => void;
  onGamesCountUpdate?: (count: number) => void;
}

const GameLibrary: React.FC<GameLibraryProps> = ({ 
  token, 
  onSelectGame,
  onGamesCountUpdate 
}) => {
  const [games, setGames] = useState<Game[]>([]);
  const [emulators, setEmulators] = useState<Emulator[]>([]);
  const [selectedSystem, setSelectedSystem] = useState<string>('all');
  const [selectedEmulator, setSelectedEmulator] = useState<string>('all');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [sortBy, setSortBy] = useState<SortOption>('title-asc');
  const [viewMode, setViewMode] = useState<ViewMode>('grid');
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  const [showUploadModal, setShowUploadModal] = useState<boolean>(false);

  useEffect(() => {
    loadData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [token]);

  const loadData = async () => {
    try {
      setLoading(true);
      setError('');
      const [gamesData, emulatorsData] = await Promise.all([
        getGames(token),
        getEmulators(token)
      ]);
      setGames(gamesData);
      setEmulators(emulatorsData);
      onGamesCountUpdate?.(gamesData.length);
    } catch (err) {
      console.error('Failed to load data:', err);
      setError('Failed to load games. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  // Memoized filtering and sorting
  const filteredAndSortedGames = useMemo(() => {
    let filtered = games;

    // Apply filters
    if (selectedSystem !== 'all') {
      filtered = filtered.filter(g => g.system === selectedSystem);
    }

    if (selectedEmulator !== 'all') {
      filtered = filtered.filter(g => g.emulator_id === selectedEmulator);
    }

    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(g => 
        g.title.toLowerCase().includes(query) ||
        g.system.toLowerCase().includes(query) ||
        g.emulator.name.toLowerCase().includes(query)
      );
    }

    // Apply sorting
    const sorted = [...filtered].sort((a, b) => {
      switch (sortBy) {
        case 'title-asc':
          return a.title.localeCompare(b.title);
        case 'title-desc':
          return b.title.localeCompare(a.title);
        case 'system-asc':
          return a.system.localeCompare(b.system);
        case 'system-desc':
          return b.system.localeCompare(a.system);
        default:
          return 0;
      }
    });

    return sorted;
  }, [games, selectedSystem, selectedEmulator, searchQuery, sortBy]);

  const systems = useMemo(() => 
    Array.from(new Set(games.map(g => g.system))),
    [games]
  );

  const handleClearFilters = useCallback(() => {
    setSelectedSystem('all');
    setSelectedEmulator('all');
    setSearchQuery('');
  }, []);

  const handleUploadComplete = useCallback(() => {
    loadData();
  }, []);

  if (loading) {
    return (
      <div className="game-library">
        <div className="loading-state">
          <div className="spinner"></div>
          <p>Loading your game library...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="game-library">
        <div className="error-state">
          <div className="error-icon">⚠️</div>
          <p>{error}</p>
          <button className="retry-button" onClick={loadData}>
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="game-library">
      <div className="library-controls">
        <div className="search-container">
          <span className="search-icon">🔍</span>
          <input
            type="text"
            placeholder="Search games, systems, or emulators..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="search-input"
          />
          {searchQuery && (
            <button 
              className="clear-search" 
              onClick={() => setSearchQuery('')}
              aria-label="Clear search"
            >
              ✕
            </button>
          )}
        </div>

        <button 
          className="upload-button"
          onClick={() => setShowUploadModal(!showUploadModal)}
          aria-label="Upload ROMs"
        >
          <span>📤</span>
          <span>{showUploadModal ? 'Hide Upload' : 'Upload ROMs'}</span>
        </button>
        
        <div className="filters">
          <select 
            value={selectedSystem} 
            onChange={(e) => setSelectedSystem(e.target.value)}
            className="filter-select"
          >
            <option value="all">All Systems ({games.length})</option>
            {systems.map(system => (
              <option key={system} value={system}>
                {system} ({games.filter(g => g.system === system).length})
              </option>
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

          <select 
            value={sortBy} 
            onChange={(e) => setSortBy(e.target.value as SortOption)}
            className="filter-select"
          >
            <option value="title-asc">Title (A-Z)</option>
            <option value="title-desc">Title (Z-A)</option>
            <option value="system-asc">System (A-Z)</option>
            <option value="system-desc">System (Z-A)</option>
          </select>

          {(selectedSystem !== 'all' || selectedEmulator !== 'all' || searchQuery) && (
            <button className="clear-filters" onClick={handleClearFilters}>
              Clear Filters
            </button>
          )}
        </div>

        <div className="view-controls">
          <span className="results-count">
            {filteredAndSortedGames.length} {filteredAndSortedGames.length === 1 ? 'game' : 'games'}
          </span>
          <div className="view-mode-toggle">
            <button
              className={`view-mode-btn ${viewMode === 'grid' ? 'active' : ''}`}
              onClick={() => setViewMode('grid')}
              aria-label="Grid view"
            >
              ⊞
            </button>
            <button
              className={`view-mode-btn ${viewMode === 'list' ? 'active' : ''}`}
              onClick={() => setViewMode('list')}
              aria-label="List view"
            >
              ☰
            </button>
          </div>
        </div>
      </div>

      {showUploadModal && (
        <div className="upload-modal">
          <RomUpload token={token} onUploadComplete={handleUploadComplete} />
        </div>
      )}

      {filteredAndSortedGames.length > 0 ? (
        <div className={`games-container ${viewMode}-view`}>
          {filteredAndSortedGames.map((game, index) => (
            <GameCard 
              key={game.id} 
              game={game} 
              onClick={() => onSelectGame(game.id)}
              viewMode={viewMode}
              animationDelay={index * 0.05}
            />
          ))}
        </div>
      ) : (
        <div className="empty-state">
          <div className="empty-icon">🎮</div>
          <h3>No games found</h3>
          <p>
            {games.length === 0 
              ? 'Add ROMs to the /roms directory to get started.'
              : 'Try adjusting your filters or search query.'}
          </p>
          {(selectedSystem !== 'all' || selectedEmulator !== 'all' || searchQuery) && (
            <button className="clear-filters-btn" onClick={handleClearFilters}>
              Clear All Filters
            </button>
          )}
        </div>
      )}
    </div>
  );
};

export default React.memo(GameLibrary);

