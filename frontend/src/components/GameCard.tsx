import React, { useState } from 'react';
import './GameCard.css';

interface Game {
  id: number;
  title: string;
  system: string;
  emulator: {
    name: string;
    github_url: string;
  };
  launch_url: string;
}

interface GameCardProps {
  game: Game;
  onClick: () => void;
  viewMode?: 'grid' | 'list';
  animationDelay?: number;
}

const systemIcons: Record<string, string> = {
  'Nintendo Switch': '🎮',
  'PS3': '🎮',
  'PS2': '🎮',
  'PSP': '🎮',
  'PS Vita': '🎮',
  'GameCube': '🎮',
  'Wii': '🎮',
  '3DS': '🎮',
  'Dreamcast': '🎮',
  'default': '🎯'
};

const GameCard: React.FC<GameCardProps> = ({ 
  game, 
  onClick, 
  viewMode = 'grid',
  animationDelay = 0 
}) => {
  const [isHovered, setIsHovered] = useState(false);
  const systemIcon = systemIcons[game.system] || systemIcons.default;

  const handlePlayClick = async (e: React.MouseEvent) => {
    e.stopPropagation();
    e.preventDefault();
    
    if (!game.launch_url) {
      console.error('Launch URL is missing for game:', game.title);
      alert(`Unable to launch ${game.title}. Launch URL is not available.`);
      return;
    }
    
    // Extract port from URL to provide better error messages
    const urlMatch = game.launch_url.match(/localhost:(\d+)/);
    const port = urlMatch ? urlMatch[1] : 'unknown';
    
    // Determine service name based on port
    const getServiceInfo = (port: string) => {
      switch (port) {
        case '8081':
          return { name: 'RetroArch', command: 'docker-compose up -d retroarch' };
        case '8082':
          return { name: 'EmulatorJS', command: 'docker-compose up -d emulatorjs' };
        case '8083':
          return { name: 'Dolphin', command: 'docker-compose up -d dolphin' };
        case '8084':
          return { name: 'PCSX2', command: 'docker-compose up -d pcsx2' };
        case '8085':
          return { name: 'RPCS3', command: 'docker-compose up -d rpcs3' };
        case '8086':
          return { name: 'PPSSPP', command: 'docker-compose up -d ppsspp' };
        case '8087':
          return { name: 'Citra', command: 'docker-compose up -d citra' };
        case '8088':
          return { name: 'Yuzu', command: 'docker-compose up -d yuzu' };
        case '8089':
          return { name: 'Ryujinx', command: 'docker-compose up -d ryujinx' };
        case '8090':
          return { name: 'Flycast', command: 'docker-compose up -d flycast' };
        case '8091':
          return { name: 'Vita3K', command: 'docker-compose up -d vita3k' };
        case '8092':
          return { name: 'BizHawk', command: 'docker-compose up -d bizhawk' };
        default:
          return { name: `Emulator service on port ${port}`, command: `docker-compose up -d <service-name>` };
      }
    };
    
    const serviceInfo = getServiceInfo(port);
    
    // Try to check if service is available (quick check)
    let serviceAvailable = false;
    try {
      const url = new URL(game.launch_url);
      const testUrl = `${url.protocol}//${url.host}`;
      
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 1500);
      
      try {
        await fetch(testUrl, { 
          method: 'HEAD', 
          signal: controller.signal,
          mode: 'no-cors',
          cache: 'no-cache'
        });
        // If fetch succeeds (doesn't throw), service might be available
        // Note: with no-cors mode, we can't read the response, but no error means connection was attempted
        serviceAvailable = true;
      } catch (fetchError) {
        // Service not available
        serviceAvailable = false;
      } finally {
        clearTimeout(timeoutId);
      }
    } catch (urlError) {
      // Invalid URL format
      console.warn('Could not parse launch URL:', urlError);
    }
    
    // If service is not available, show helpful message before opening
    if (!serviceAvailable) {
      const userConfirmed = window.confirm(
        `⚠️ ${serviceInfo.name} service appears to be offline (port ${port})\n\n` +
        `If you see "ERR_CONNECTION_REFUSED", the emulator service is not running.\n\n` +
        `To start it:\n` +
        `1. Open docker-compose.yml\n` +
        `2. Uncomment the ${serviceInfo.name} service section\n` +
        `3. Run: ${serviceInfo.command}\n\n` +
        `Would you like to try opening the game anyway?`
      );
      
      if (!userConfirmed) {
        return;
      }
    }
    
    try {
      const newWindow = window.open(game.launch_url, '_blank');
      if (!newWindow) {
        alert('Please allow pop-ups for this site to launch games.');
        return;
      }
    } catch (error) {
      console.error('Error launching game:', error);
      alert(
        `❌ Failed to launch ${game.title}\n\n` +
        `Error: ${error instanceof Error ? error.message : 'Unknown error'}\n\n` +
        `The ${serviceInfo.name} service may not be running.\n` +
        `Please check your docker-compose.yml and ensure the emulator service is started.\n\n` +
        `Command: ${serviceInfo.command}`
      );
    }
  };

  const handleGithubClick = (e: React.MouseEvent) => {
    e.stopPropagation();
  };

  return (
    <div 
      className={`game-card ${viewMode}-mode ${isHovered ? 'hovered' : ''}`}
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      style={{ animationDelay: `${animationDelay}s` }}
    >
      <div className="game-card-background"></div>
      
      <div className="game-card-content">
        <div className="game-card-header">
          <div className="game-info">
            <div className="game-icon">{systemIcon}</div>
            <div className="game-title-wrapper">
              <h3 className="game-title">{game.title}</h3>
              <p className="emulator-name">{game.emulator.name}</p>
            </div>
          </div>
          <div className="system-badge">
            <span>{game.system}</span>
          </div>
        </div>

        <div className="game-card-body">
          <div className="game-details">
            <div className="detail-item">
              <span className="detail-label">System</span>
              <span className="detail-value">{game.system}</span>
            </div>
            <div className="detail-item">
              <span className="detail-label">Emulator</span>
              <span className="detail-value">{game.emulator.name}</span>
            </div>
          </div>
        </div>

        <div className="game-card-footer">
          <a 
            href={game.emulator.github_url} 
            target="_blank" 
            rel="noopener noreferrer"
            className="github-link"
            onClick={handleGithubClick}
          >
            <span className="github-icon">⚙️</span>
            <span>GitHub</span>
          </a>
          
          <button 
            className="play-button" 
            onClick={handlePlayClick}
          >
            <span className="play-icon">▶</span>
            <span>Play Now</span>
          </button>
        </div>
      </div>

      <div className="card-shine"></div>
    </div>
  );
};

export default React.memo(GameCard);

