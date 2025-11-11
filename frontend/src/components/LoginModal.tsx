import React, { useState, useEffect } from 'react';
import { login, getPreviousUsernames, deleteAccount } from '../services/api';
import './LoginModal.css';

interface LoginModalProps {
  onLogin: (username: string) => void;
}

const LoginModal: React.FC<LoginModalProps> = ({ onLogin }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [previousUsernames, setPreviousUsernames] = useState<string[]>([]);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [deleteLoading, setDeleteLoading] = useState(false);

  // Load previous usernames on mount
  useEffect(() => {
    const loadPreviousUsernames = async () => {
      try {
        const response = await getPreviousUsernames();
        setPreviousUsernames(response.usernames);
      } catch (err) {
        console.error('Failed to load previous usernames:', err);
      }
    };
    loadPreviousUsernames();
  }, []);

  const handleUsernameClick = (selectedUsername: string) => {
    setUsername(selectedUsername);
    // Focus password field after selecting username
    setTimeout(() => {
      const passwordInput = document.getElementById('password') as HTMLInputElement;
      if (passwordInput) {
        passwordInput.focus();
      }
    }, 100);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    if (!username.trim()) {
      setError('Please enter a username');
      setLoading(false);
      return;
    }

    if (!password) {
      setError('Please enter a password');
      setLoading(false);
      return;
    }

    try {
      const response = await login({ username: username.trim(), password });
      if (response.success && response.username) {
        onLogin(response.username);
      } else {
        setError(response.message || 'Login failed');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteAccount = async () => {
    if (!username.trim() || !password) {
      setError('Please enter username and password to delete account');
      return;
    }

    setDeleteLoading(true);
    setError('');

    try {
      const response = await deleteAccount({ 
        username: username.trim(), 
        password 
      });
      
      if (response.success) {
        // Remove username from previous usernames list
        setPreviousUsernames(prev => prev.filter(u => u !== username.trim()));
        setUsername('');
        setPassword('');
        setError('Account deleted successfully');
        setShowDeleteConfirm(false);
        // Reload previous usernames
        try {
          const usernamesResponse = await getPreviousUsernames();
          setPreviousUsernames(usernamesResponse.usernames);
        } catch (err) {
          console.error('Failed to reload usernames:', err);
        }
      } else {
        setError(response.message || 'Failed to delete account');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete account');
    } finally {
      setDeleteLoading(false);
    }
  };

  return (
    <div className="login-modal-overlay">
      <div className="login-modal">
        <div className="login-modal-header">
          <h2>Welcome to Gamers Unite</h2>
          <p>Please enter your username and password to continue</p>
        </div>
        {previousUsernames.length > 0 && (
          <div className="previous-usernames">
            <p className="previous-usernames-label">Quick Login:</p>
            <div className="previous-usernames-list">
              {previousUsernames.map((prevUsername) => (
                <button
                  key={prevUsername}
                  type="button"
                  className={`previous-username-btn ${username === prevUsername ? 'selected' : ''}`}
                  onClick={() => handleUsernameClick(prevUsername)}
                  disabled={loading || deleteLoading}
                >
                  {prevUsername}
                </button>
              ))}
            </div>
          </div>
        )}
        <form onSubmit={handleSubmit} className="login-form">
          <div className="form-group">
            <label htmlFor="username">Username</label>
            <input
              id="username"
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Enter your username"
              maxLength={50}
              autoFocus
              disabled={loading || deleteLoading}
            />
          </div>
          <div className="form-group">
            <label htmlFor="password">Password</label>
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter the password"
              disabled={loading || deleteLoading}
            />
          </div>
          {error && <div className="error-message">{error}</div>}
          <div className="login-actions">
            <button type="submit" className="login-button" disabled={loading || deleteLoading}>
            {loading ? 'Logging in...' : 'Login'}
          </button>
            {username.trim() && (
              <button
                type="button"
                className="delete-account-button"
                onClick={() => setShowDeleteConfirm(true)}
                disabled={loading || deleteLoading}
              >
                Delete Account
              </button>
            )}
          </div>
        </form>
        {showDeleteConfirm && (
          <div className="delete-confirm-overlay" onClick={() => setShowDeleteConfirm(false)}>
            <div className="delete-confirm-modal" onClick={(e) => e.stopPropagation()}>
              <h3>Delete Account</h3>
              <p>Are you sure you want to delete your account? This action cannot be undone.</p>
              <p className="delete-warning">
                This will permanently delete your account, all your games, save states, and play history.
              </p>
              <div className="delete-confirm-actions">
                <button
                  type="button"
                  className="delete-confirm-button"
                  onClick={handleDeleteAccount}
                  disabled={deleteLoading}
                >
                  {deleteLoading ? 'Deleting...' : 'Yes, Delete Account'}
                </button>
                <button
                  type="button"
                  className="delete-cancel-button"
                  onClick={() => setShowDeleteConfirm(false)}
                  disabled={deleteLoading}
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default LoginModal;

