import { OBSConnectionStatus } from "../types/obs";

interface ConnectionStatusProps {
  status: OBSConnectionStatus;
}

export const ConnectionStatus = ({ status }: ConnectionStatusProps) => {
  return (
    <div className="connection-status">
      <div className="status-indicator">
        <span
          className={`status-dot ${status.connected ? "connected" : "disconnected"}`}
        ></span>
        <span className="status-text">
          {status.connected ? "接続済み" : "未接続"}
        </span>
      </div>
      {status.connected && (
        <div className="status-details">
          <div>OBS Version: {status.obsVersion || "N/A"}</div>
          <div>
            WebSocket Version: {status.obsWebSocketVersion || "N/A"}
          </div>
        </div>
      )}
    </div>
  );
};
