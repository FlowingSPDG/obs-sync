import { useState } from "react";
import { useNetworkStatus } from "../hooks/useNetworkStatus";
import { ConnectionState } from "../types/network";

export const MasterControl = () => {
  const [port, setPort] = useState(8080);
  const { status, startMasterServer, stopMasterServer } = useNetworkStatus();

  const handleStart = async () => {
    try {
      await startMasterServer(port);
    } catch (error) {
      console.error("Failed to start master server:", error);
    }
  };

  const handleStop = async () => {
    try {
      await stopMasterServer();
    } catch (error) {
      console.error("Failed to stop master server:", error);
    }
  };

  const isConnected = status.state === ConnectionState.Connected;

  return (
    <div className="master-control">
      <h2>Masterモード</h2>
      
      <div className="control-section">
        <label>
          ポート番号:
          <input
            type="number"
            value={port}
            onChange={(e) => setPort(Number(e.target.value))}
            disabled={isConnected}
            min={1024}
            max={65535}
          />
        </label>
      </div>

      <div className="control-section">
        {!isConnected ? (
          <button onClick={handleStart} className="btn-primary">
            サーバーを起動
          </button>
        ) : (
          <button onClick={handleStop} className="btn-danger">
            サーバーを停止
          </button>
        )}
      </div>

      {status.state === ConnectionState.Connected && (
        <div className="status-info">
          <p>サーバー起動中 (ポート: {port})</p>
          <p>接続中のクライアント: {status.connectedClients || 0}</p>
        </div>
      )}

      {status.lastError && (
        <div className="error-message">{status.lastError}</div>
      )}
    </div>
  );
};
