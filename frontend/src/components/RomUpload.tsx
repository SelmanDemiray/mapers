import React, { useState, useEffect, useCallback } from 'react';
import { uploadRom, scanRoms, getConsoles, Console, UploadResult, ScanResult } from '../services/api';
import './RomUpload.css';

interface RomUploadProps {
  token: string;
  onUploadComplete?: () => void;
}

interface UploadFile {
  id: string;
  file: File;
  progress: number;
  status: 'pending' | 'uploading' | 'success' | 'error';
  message?: string;
  console?: string;
  title?: string;
}

const RomUpload: React.FC<RomUploadProps> = ({ token, onUploadComplete }) => {
  const [consoles, setConsoles] = useState<Console[]>([]);
  const [selectedConsole, setSelectedConsole] = useState<string>('auto');
  const [files, setFiles] = useState<UploadFile[]>([]);
  const [isDragging, setIsDragging] = useState(false);
  const [isScanning, setIsScanning] = useState(false);
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [showScanner, setShowScanner] = useState(false);

  useEffect(() => {
    loadConsoles();
  }, [token]);

  const loadConsoles = async () => {
    try {
      const data = await getConsoles(token);
      setConsoles(data);
    } catch (error) {
      console.error('Failed to load consoles:', error);
    }
  };

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const droppedFiles = Array.from(e.dataTransfer.files);
    addFiles(droppedFiles);
  }, []);

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      const selectedFiles = Array.from(e.target.files);
      addFiles(selectedFiles);
    }
  }, []);

  const addFiles = (newFiles: File[]) => {
    const uploadFiles: UploadFile[] = newFiles.map(file => ({
      id: `${Date.now()}-${Math.random()}`,
      file,
      progress: 0,
      status: 'pending',
      console: selectedConsole === 'auto' ? undefined : selectedConsole,
    }));

    setFiles(prev => [...prev, ...uploadFiles]);
  };

  const handleUploadFile = async (fileToUpload: UploadFile) => {
    setFiles(prev => prev.map(f => 
      f.id === fileToUpload.id ? { ...f, status: 'uploading', progress: 0 } : f
    ));

    try {
      const result = await uploadRom(
        token,
        fileToUpload.file,
        fileToUpload.console,
        fileToUpload.title,
        (progress) => {
          setFiles(prev => prev.map(f =>
            f.id === fileToUpload.id ? { ...f, progress } : f
          ));
        }
      );

      setFiles(prev => prev.map(f =>
        f.id === fileToUpload.id
          ? { ...f, status: 'success', progress: 100, message: result.message }
          : f
      ));

      onUploadComplete?.();
    } catch (error) {
      setFiles(prev => prev.map(f =>
        f.id === fileToUpload.id
          ? {
              ...f,
              status: 'error',
              message: error instanceof Error ? error.message : 'Upload failed',
            }
          : f
      ));
    }
  };

  const uploadAll = async () => {
    const pendingFiles = files.filter(f => f.status === 'pending' || f.status === 'error');
    
    for (const file of pendingFiles) {
      await handleUploadFile(file);
    }
  };

  const removeFile = (id: string) => {
    setFiles(prev => prev.filter(f => f.id !== id));
  };

  const clearCompleted = () => {
    setFiles(prev => prev.filter(f => f.status !== 'success'));
  };

  const handleScan = async () => {
    setIsScanning(true);
    setScanResult(null);
    
    try {
      const result = await scanRoms(token);
      setScanResult(result);
      onUploadComplete?.();
    } catch (error) {
      console.error('Scan failed:', error);
      setScanResult({
        total_found: 0,
        newly_added: 0,
        already_exists: 0,
        errors: [error instanceof Error ? error.message : 'Scan failed'],
      });
    } finally {
      setIsScanning(false);
    }
  };

  const getStatusIcon = (status: UploadFile['status']) => {
    switch (status) {
      case 'success': return '✓';
      case 'error': return '✕';
      case 'uploading': return '⟳';
      default: return '⏸';
    }
  };

  const getStatusColor = (status: UploadFile['status']) => {
    switch (status) {
      case 'success': return '#4caf50';
      case 'error': return '#f44336';
      case 'uploading': return '#2196f3';
      default: return '#666';
    }
  };

  return (
    <div className="rom-upload">
      <div className="upload-header">
        <div className="upload-tabs">
          <button
            className={`tab ${!showScanner ? 'active' : ''}`}
            onClick={() => setShowScanner(false)}
          >
            📤 Upload ROMs
          </button>
          <button
            className={`tab ${showScanner ? 'active' : ''}`}
            onClick={() => setShowScanner(true)}
          >
            🔍 Scan Directory
          </button>
        </div>
      </div>

      {!showScanner ? (
        <>
          <div className="console-selector">
            <label htmlFor="console-select">Target Console:</label>
            <select
              id="console-select"
              value={selectedConsole}
              onChange={(e) => setSelectedConsole(e.target.value)}
              className="console-select"
            >
              <option value="auto">Auto-detect from file extension</option>
              {consoles.map(console => (
                <option key={console.id} value={console.id}>
                  {console.name} ({console.supported_formats.join(', ')})
                </option>
              ))}
            </select>
          </div>

          <div
            className={`drop-zone ${isDragging ? 'dragging' : ''}`}
            onDragEnter={handleDragEnter}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
          >
            <div className="drop-zone-content">
              <div className="drop-icon">📁</div>
              <h3>Drag & Drop ROM Files Here</h3>
              <p>or</p>
              <label className="file-select-button">
                Browse Files
                <input
                  type="file"
                  multiple
                  onChange={handleFileSelect}
                  accept=".nes,.smc,.sfc,.n64,.z64,.iso,.bin,.cue,.gba,.nds,.3ds,.nsp,.xci,.gcm,.wbfs,.cdi,.gdi,.chd,.pbp,.vpk,.pkg"
                />
              </label>
              <p className="supported-formats">
                Supports: NES, SNES, N64, GameCube, Wii, PS1, PS2, PS3, PSP, GBA, DS, 3DS, Switch, and more
              </p>
            </div>
          </div>

          {files.length > 0 && (
            <div className="upload-queue">
              <div className="queue-header">
                <h3>Upload Queue ({files.length})</h3>
                <div className="queue-actions">
                  <button
                    onClick={uploadAll}
                    disabled={files.every(f => f.status === 'success' || f.status === 'uploading')}
                    className="upload-all-btn"
                  >
                    Upload All
                  </button>
                  <button onClick={clearCompleted} className="clear-btn">
                    Clear Completed
                  </button>
                </div>
              </div>

              <div className="file-list">
                {files.map(uploadFile => (
                  <div key={uploadFile.id} className={`file-item ${uploadFile.status}`}>
                    <div className="file-info">
                      <span
                        className="status-icon"
                        style={{ color: getStatusColor(uploadFile.status) }}
                      >
                        {getStatusIcon(uploadFile.status)}
                      </span>
                      <div className="file-details">
                        <div className="file-name">{uploadFile.file.name}</div>
                        <div className="file-meta">
                          {(uploadFile.file.size / 1024 / 1024).toFixed(2)} MB
                          {uploadFile.console && ` • ${uploadFile.console}`}
                        </div>
                        {uploadFile.message && (
                          <div className={`file-message ${uploadFile.status}`}>
                            {uploadFile.message}
                          </div>
                        )}
                      </div>
                    </div>

                    <div className="file-actions">
                      {uploadFile.status === 'uploading' && (
                        <div className="progress-bar">
                          <div
                            className="progress-fill"
                            style={{ width: `${uploadFile.progress}%` }}
                          />
                          <span className="progress-text">
                            {uploadFile.progress.toFixed(0)}%
                          </span>
                        </div>
                      )}
                      {uploadFile.status === 'pending' && (
                        <button
                          onClick={() => handleUploadFile(uploadFile)}
                          className="upload-btn"
                        >
                          Upload
                        </button>
                      )}
                      {uploadFile.status !== 'uploading' && (
                        <button
                          onClick={() => removeFile(uploadFile.id)}
                          className="remove-btn"
                        >
                          Remove
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </>
      ) : (
        <div className="scanner-section">
          <div className="scanner-description">
            <h3>📂 Scan ROMs Directory</h3>
            <p>
              Scan the ROMs directory for new files that haven't been added to your library yet.
              The scanner will automatically detect console types and match ROMs with compatible emulators.
            </p>
          </div>

          <button
            onClick={handleScan}
            disabled={isScanning}
            className="scan-button"
          >
            {isScanning ? (
              <>
                <span className="spinner">⟳</span> Scanning...
              </>
            ) : (
              <>
                🔍 Start Scan
              </>
            )}
          </button>

          {scanResult && (
            <div className="scan-results">
              <h4>Scan Results</h4>
              <div className="scan-stats">
                <div className="stat-item">
                  <div className="stat-value">{scanResult.total_found}</div>
                  <div className="stat-label">Total Files Found</div>
                </div>
                <div className="stat-item success">
                  <div className="stat-value">{scanResult.newly_added}</div>
                  <div className="stat-label">Newly Added</div>
                </div>
                <div className="stat-item info">
                  <div className="stat-value">{scanResult.already_exists}</div>
                  <div className="stat-label">Already in Library</div>
                </div>
                {scanResult.errors.length > 0 && (
                  <div className="stat-item error">
                    <div className="stat-value">{scanResult.errors.length}</div>
                    <div className="stat-label">Errors</div>
                  </div>
                )}
              </div>

              {scanResult.errors.length > 0 && (
                <div className="scan-errors">
                  <h5>Errors:</h5>
                  <ul>
                    {scanResult.errors.map((error, index) => (
                      <li key={index}>{error}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default React.memo(RomUpload);

