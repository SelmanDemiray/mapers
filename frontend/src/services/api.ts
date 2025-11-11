// Use relative URL when accessed through Cloudflare Tunnel, otherwise use env var or localhost
const getApiUrl = () => {
  // If we're accessing through a tunnel (HTTPS) or from a remote host, use relative URL
  // The React dev server proxy will handle /api requests in development
  // For production/remote access, Cloudflare Tunnel will route /api to backend
  if (window.location.protocol === 'https:' || 
      (window.location.hostname !== 'localhost' && window.location.hostname !== '127.0.0.1')) {
    // Return empty base so callers can use absolute path starting with /api
    return '';
  }
  // For local development, use the configured API URL
  let base = process.env.REACT_APP_API_URL || 'http://localhost:37291';
  // Ensure single /api suffix for local absolute base
  if (!base.endsWith('/api')) {
    base = `${base.replace(/\/+$/, '')}/api`;
  }
  return base;
};

const API_URL = getApiUrl();

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

export interface Console {
  id: string;
  name: string;
  supported_formats: string[];
}

export interface ScanResult {
  total_found: number;
  newly_added: number;
  already_exists: number;
  errors: string[];
}

export interface UploadResult {
  success: boolean;
  message: string;
  game_id?: number;
  file_path?: string;
}

export async function getConsoles(token: string): Promise<Console[]> {
  const response = await fetch(`${API_URL}/api/roms/consoles`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    throw new Error('Failed to fetch consoles');
  }
  return response.json();
}

export async function scanRoms(token: string): Promise<ScanResult> {
  const response = await fetch(`${API_URL}/api/roms/scan`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    throw new Error('Failed to scan ROMs');
  }
  return response.json();
}

export async function uploadRom(
  token: string,
  file: File,
  console?: string,
  title?: string,
  onProgress?: (progress: number) => void
): Promise<UploadResult> {
  const formData = new FormData();
  formData.append('file', file);
  if (console) {
    formData.append('console', console);
  }
  if (title) {
    formData.append('title', title);
  }

  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    xhr.upload.addEventListener('progress', (e) => {
      if (e.lengthComputable && onProgress) {
        const progress = (e.loaded / e.total) * 100;
        onProgress(progress);
      }
    });

    xhr.addEventListener('load', () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        try {
          const result = JSON.parse(xhr.responseText);
          resolve(result);
        } catch (e) {
          reject(new Error('Invalid response from server'));
        }
      } else {
        reject(new Error(`Upload failed with status ${xhr.status}`));
      }
    });

    xhr.addEventListener('error', () => {
      reject(new Error('Upload failed'));
    });

    xhr.open('POST', `${API_URL}/api/roms/upload`);
    xhr.setRequestHeader('Authorization', `Bearer ${token}`);
    xhr.send(formData);
  });
}

