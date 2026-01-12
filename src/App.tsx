import { useState, useEffect } from "react";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import "./App.css";

import { useOBSConnection } from "./hooks/useOBSConnection";
import { useSyncState } from "./hooks/useSyncState";
import { ConnectionStatus } from "./components/ConnectionStatus";
import { MasterControl } from "./components/MasterControl";
import { SlaveMonitor } from "./components/SlaveMonitor";
import { SyncTargetSelector } from "./components/SyncTargetSelector";
import { AlertPanel } from "./components/AlertPanel";
import { OBSSourceList } from "./components/OBSSourceList";
import { AppMode } from "./types/sync";
import { OBSSource } from "./types/obs";
import { DesyncAlert } from "./types/sync";

function App() {
  const [appMode, setAppMode] = useState<AppMode | null>(null);
  const [obsHost, setObsHost] = useState("localhost");
  const [obsPort, setObsPort] = useState(4455);
  const [obsPassword, setObsPassword] = useState("");
  const [sources, setSources] = useState<OBSSource[]>([]);
  const [alerts, setAlerts] = useState<DesyncAlert[]>([]);

  const { status: obsStatus, connect, disconnect, error: obsError } = useOBSConnection();
  const { syncState, setMode, error: syncError } = useSyncState();

  useEffect(() => {
    if (obsError) {
      toast.error(`OBS接続エラー: ${obsError}`);
    }
  }, [obsError]);

  useEffect(() => {
    if (syncError) {
      toast.error(`同期エラー: ${syncError}`);
    }
  }, [syncError]);

  const handleConnectOBS = async () => {
    try {
      await connect({
        host: obsHost,
        port: obsPort,
        password: obsPassword || undefined,
      });
      toast.success("OBSに接続しました");
    } catch (error) {
      console.error("Failed to connect to OBS:", error);
    }
  };

  const handleDisconnectOBS = async () => {
    try {
      await disconnect();
      toast.info("OBSから切断しました");
    } catch (error) {
      console.error("Failed to disconnect from OBS:", error);
    }
  };

  const handleSetMode = async (mode: AppMode) => {
    try {
      await setMode(mode);
      setAppMode(mode);
      toast.success(`${mode === AppMode.Master ? "Master" : "Slave"}モードに設定しました`);
    } catch (error) {
      console.error("Failed to set mode:", error);
    }
  };

  const handleClearAlert = (id: string) => {
    setAlerts((prev) => prev.filter((alert) => alert.id !== id));
  };

  return (
    <div className="app">
      <ToastContainer position="top-right" autoClose={3000} />
      
      <header className="app-header">
        <h1>OBS Sync</h1>
        <p className="subtitle">LAN内のOBS同期システム</p>
      </header>

      <main className="app-main">
        {/* Mode Selection */}
        {!appMode && (
          <div className="mode-selection">
            <h2>モードを選択してください</h2>
            <div className="mode-buttons">
              <button
                className="mode-button master"
                onClick={() => handleSetMode(AppMode.Master)}
              >
                <div className="mode-icon">🎛️</div>
                <div className="mode-title">Masterモード</div>
                <div className="mode-description">
                  OBSの変更を監視し、Slaveに配信
                </div>
              </button>
              <button
                className="mode-button slave"
                onClick={() => handleSetMode(AppMode.Slave)}
              >
                <div className="mode-icon">📺</div>
                <div className="mode-title">Slaveモード</div>
                <div className="mode-description">
                  Masterからの変更を受信し、OBSに適用
                </div>
              </button>
            </div>
          </div>
        )}

        {/* OBS Connection Section */}
        {appMode && (
          <>
            <section className="section obs-connection-section">
              <h2>OBS接続</h2>
              <ConnectionStatus status={obsStatus} />
              
              {!obsStatus.connected ? (
                <div className="connection-form">
                  <div className="form-row">
                    <label>
                      ホスト:
                      <input
                        type="text"
                        value={obsHost}
                        onChange={(e) => setObsHost(e.target.value)}
                        placeholder="localhost"
                      />
                    </label>
                    <label>
                      ポート:
                      <input
                        type="number"
                        value={obsPort}
                        onChange={(e) => setObsPort(Number(e.target.value))}
                        min={1024}
                        max={65535}
                      />
                    </label>
                  </div>
                  <div className="form-row">
                    <label>
                      パスワード (オプション):
                      <input
                        type="password"
                        value={obsPassword}
                        onChange={(e) => setObsPassword(e.target.value)}
                        placeholder="パスワードなしの場合は空欄"
                      />
                    </label>
                  </div>
                  <button onClick={handleConnectOBS} className="btn-primary">
                    OBSに接続
                  </button>
                </div>
              ) : (
                <div className="connection-actions">
                  <button onClick={handleDisconnectOBS} className="btn-danger">
                    OBSから切断
                  </button>
                </div>
              )}
            </section>

            {/* Sync Target Selection */}
            {obsStatus.connected && (
              <section className="section">
                <SyncTargetSelector />
              </section>
            )}

            {/* Mode-specific Controls */}
            {obsStatus.connected && (
              <section className="section">
                {appMode === AppMode.Master ? (
                  <MasterControl />
                ) : (
                  <SlaveMonitor />
                )}
              </section>
            )}

            {/* Sources and Alerts */}
            {obsStatus.connected && syncState.isActive && (
              <div className="info-panels">
                <section className="section">
                  <OBSSourceList sources={sources} />
                </section>
                
                {appMode === AppMode.Slave && (
                  <section className="section">
                    <AlertPanel alerts={alerts} onClearAlert={handleClearAlert} />
                  </section>
                )}
              </div>
            )}

            {/* Reset Mode */}
            <div className="mode-reset">
              <button
                onClick={() => {
                  setAppMode(null);
                  setMode(null as any).catch(console.error);
                }}
                className="btn-secondary"
              >
                モードを変更
              </button>
            </div>
          </>
        )}
      </main>

      <footer className="app-footer">
        <p>OBS Sync - イベント向けOBS同期システム</p>
      </footer>
    </div>
  );
}

export default App;
