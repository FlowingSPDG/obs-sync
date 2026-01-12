import { useState } from "react";
import { useNetworkStatus } from "../hooks/useNetworkStatus";
import { ConnectionState } from "../types/network";

export const SlaveMonitor = () => {
  const [host, setHost] = useState("localhost");
  const [port, setPort] = useState(8080);
  const { status, connectToMaster, disconnectFromMaster } = useNetworkStatus();

  const handleConnect = async () => {
    try {
      await connectToMaster({ host, port });
    } catch (error) {
      console.error("Failed to connect to master:", error);
    }
  };

  const handleDisconnect = async () => {
    try {
      await disconnectFromMaster();
    } catch (error) {
      console.error("Failed to disconnect from master:", error);
    }
  };

  const isConnected = status.state === ConnectionState.Connected;
  const isConnecting = status.state === ConnectionState.Connecting;

  return (
    <div className="slave-monitor">
      <h2>Slaveモード</h2>

      <div className="control-section">
        <label>
          Masterホスト:
          <input
            type="text"
            value={host}
            onChange={(e) => setHost(e.target.value)}
            disabled={isConnected || isConnecting}
            placeholder="localhost"
          />
        </label>
      </div>

      <div className="control-section">
        <label>
          ポート番号:
          <input
            type="number"
            value={port}
            onChange={(e) => setPort(Number(e.target.value))}
            disabled={isConnected || isConnecting}
            min={1024}
            max={65535}
          />
        </label>
      </div>

      <div className="control-section">
        {!isConnected ? (
          <button
            onClick={handleConnect}
            className="btn-primary"
            disabled={isConnecting}
          >
            {isConnecting ? "接続中..." : "Masterに接続"}
          </button>
        ) : (
          <button onClick={handleDisconnect} className="btn-danger">
            切断
          </button>
        )}
      </div>

      {status.state === ConnectionState.Connected && (
        <div className="status-info">
          <p>
            Masterに接続中 ({host}:{port})
          </p>
        </div>
      )}

      {status.lastError && (
        <div className="error-message">{status.lastError}</div>
      )}
    </div>
  );
};
