import React, { useEffect, useState } from 'react';
import { getConnectedUsers, ConnectedUser } from '../services/api';
import './ConnectedUsers.css';

const ConnectedUsers: React.FC = () => {
  const [users, setUsers] = useState<ConnectedUser[]>([]);
  const [isOpen, setIsOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await getConnectedUsers();
      setUsers(response.users);
    } catch (err) {
      console.error('Failed to fetch connected users:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Fetch immediately
    fetchUsers();

    // Then fetch every 5 seconds
    const interval = setInterval(fetchUsers, 5000);

    return () => clearInterval(interval);
  }, []);

  const formatTime = (timestamp: string) => {
    try {
      const date = new Date(timestamp);
      const now = new Date();
      const diff = Math.floor((now.getTime() - date.getTime()) / 1000);
      
      if (diff < 60) {
        return 'just now';
      } else if (diff < 3600) {
        const minutes = Math.floor(diff / 60);
        return `${minutes}m ago`;
      } else {
        const hours = Math.floor(diff / 3600);
        return `${hours}h ago`;
      }
    } catch {
      return 'unknown';
    }
  };

  return (
    <div className="connected-users-container">
      <button
        className="connected-users-toggle"
        onClick={() => setIsOpen(!isOpen)}
        title="View connected users"
      >
        <span className="users-icon">👥</span>
        <span className="users-count">{users.length}</span>
        <span className="users-label">Connected</span>
      </button>
      
      {isOpen && (
        <div className="connected-users-panel">
          <div className="connected-users-header">
            <h3>Connected Users</h3>
            <button
              className="close-button"
              onClick={() => setIsOpen(false)}
              aria-label="Close"
            >
              ×
            </button>
          </div>
          <div className="connected-users-list">
            {loading ? (
              <div className="loading">Loading...</div>
            ) : users.length === 0 ? (
              <div className="empty-state">No users connected</div>
            ) : (
              users.map((user, index) => (
                <div key={`${user.username}-${user.ip_address}-${index}`} className="user-item">
                  <div className="user-info">
                    <div className="user-name">{user.username}</div>
                    <div className="user-ip">{user.ip_address}</div>
                  </div>
                  <div className="user-time">{formatTime(user.last_seen)}</div>
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default ConnectedUsers;

