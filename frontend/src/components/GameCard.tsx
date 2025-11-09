import React from 'react';
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
}

const GameCard: React.FC<GameCardProps> = ({ game, onClick }) => {
  return (
    <div className="game-card" onClick={onClick}>
      <div className="game-card-header">
        <h3>{game.title}</h3>
        <span className="system-badge">{game.system}</span>
      </div>
      <div className="game-card-body">
        <p className="emulator-name">Emulator: {game.emulator.name}</p>
        <a 
          href={game.emulator.github_url} 
          target="_blank" 
          rel="noopener noreferrer"
          className="github-link"
          onClick={(e) => e.stopPropagation()}
        >
          View on GitHub
        </a>
      </div>
      <div className="game-card-footer">
        <button className="play-button" onClick={(e) => {
          e.stopPropagation();
          window.open(game.launch_url, '_blank');
        }}>
          Play
        </button>
      </div>
    </div>
  );
};

export default GameCard;

