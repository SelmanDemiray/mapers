const API_URL = process.env.REACT_APP_API_URL || 'http://localhost:37291';

export interface Game {
  id: number;
  title: string;
  system: string;
  file_path: string;
  emulator_id: string;
  emulator: {
    id: string;
    name: string;
    system: string;
    github_url: string;
  };
  launch_url: string;
}

export interface Emulator {
  id: string;
  name: string;
  system: string;
  core: string;
  supported_formats: string[];
  emulator_type: string;
  service_port?: number;
  github_url: string;
  license: string;
}

export async function getGames(token: string): Promise<Game[]> {
  const response = await fetch(`${API_URL}/api/games`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    throw new Error('Failed to fetch games');
  }
  return response.json();
}

export async function getEmulators(token: string): Promise<Emulator[]> {
  const response = await fetch(`${API_URL}/api/emulators`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    throw new Error('Failed to fetch emulators');
  }
  return response.json();
}

export async function getGame(token: string, id: number): Promise<Game> {
  const response = await fetch(`${API_URL}/api/games/${id}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    throw new Error('Failed to fetch game');
  }
  return response.json();
}

export async function addGame(token: string, game: {
  title: string;
  system: string;
  file_path: string;
  emulator_id: string;
}): Promise<Game> {
  const response = await fetch(`${API_URL}/api/games`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(game),
  });
  if (!response.ok) {
    throw new Error('Failed to add game');
  }
  return response.json();
}

