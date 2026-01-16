import { useState, useMemo } from "react";
import { DesyncAlert } from "../types/sync";

interface AlertPanelProps {
  alerts: DesyncAlert[];
  onClearAlert: (id: string) => void;
  onClearAllAlerts?: () => void;
}

export const AlertPanel = ({ alerts, onClearAlert, onClearAllAlerts }: AlertPanelProps) => {
  const [filterSeverity, setFilterSeverity] = useState<"all" | "warning" | "error">("all");
  const [sortOrder, setSortOrder] = useState<"newest" | "oldest">("newest");

  const filteredAndSortedAlerts = useMemo(() => {
    let filtered = alerts;
    
    if (filterSeverity !== "all") {
      filtered = filtered.filter((alert) => alert.severity === filterSeverity);
    }
    
    const sorted = [...filtered].sort((a, b) => {
      if (sortOrder === "newest") {
        return b.timestamp - a.timestamp;
      } else {
        return a.timestamp - b.timestamp;
      }
    });
    
    return sorted;
  }, [alerts, filterSeverity, sortOrder]);

  const warningCount = alerts.filter((a) => a.severity === "warning").length;
  const errorCount = alerts.filter((a) => a.severity === "error").length;

  if (alerts.length === 0) {
    return (
      <div className="alert-panel empty">
        <div className="empty-icon">✅</div>
        <p>アラートはありません</p>
        <p className="empty-subtitle">すべての同期が正常に動作しています</p>
      </div>
    );
  }

  return (
    <div className="alert-panel">
      <div className="alert-panel-header">
        <div className="header-title-section">
          <h3>非同期アラート</h3>
          <div className="alert-stats">
            <span className={`stat-badge stat-warning ${filterSeverity === "warning" ? "active" : ""}`}>
              ⚠️ {warningCount}
            </span>
            <span className={`stat-badge stat-error ${filterSeverity === "error" ? "active" : ""}`}>
              ❌ {errorCount}
            </span>
            <span className="stat-badge stat-total">
              合計: {alerts.length}
            </span>
          </div>
        </div>
        <div className="header-controls">
          <select
            className="filter-select"
            value={filterSeverity}
            onChange={(e) => setFilterSeverity(e.target.value as "all" | "warning" | "error")}
          >
            <option value="all">すべて</option>
            <option value="warning">警告のみ</option>
            <option value="error">エラーのみ</option>
          </select>
          <select
            className="sort-select"
            value={sortOrder}
            onChange={(e) => setSortOrder(e.target.value as "newest" | "oldest")}
          >
            <option value="newest">新しい順</option>
            <option value="oldest">古い順</option>
          </select>
          {onClearAllAlerts && (
            <button
              className="btn-clear-all"
              onClick={onClearAllAlerts}
              title="すべてのアラートをクリア"
            >
              すべてクリア
            </button>
          )}
        </div>
      </div>
      
      <div className="alerts-list">
        {filteredAndSortedAlerts.length === 0 ? (
          <div className="no-alerts-filtered">
            <p>フィルター条件に一致するアラートがありません</p>
          </div>
        ) : (
          filteredAndSortedAlerts.map((alert) => {
            const date = new Date(alert.timestamp);
            const timeString = date.toLocaleTimeString("ja-JP", {
              hour: "2-digit",
              minute: "2-digit",
              second: "2-digit",
            });
            const dateString = date.toLocaleDateString("ja-JP", {
              month: "short",
              day: "numeric",
            });
            
            return (
              <div
                key={alert.id}
                className={`alert-item alert-item-${alert.severity}`}
              >
                <div className="alert-severity-indicator" />
                <div className="alert-content-wrapper">
                  <div className="alert-header">
                    <div className="alert-time-group">
                      <span className="alert-date">{dateString}</span>
                      <span className="alert-time">{timeString}</span>
                    </div>
                    <button
                      className="alert-close"
                      onClick={() => onClearAlert(alert.id)}
                      title="このアラートを削除"
                    >
                      ×
                    </button>
                  </div>
                  <div className="alert-content">
                    {(alert.sceneName || alert.sourceName) && (
                      <div className="alert-location">
                        {alert.sceneName && (
                          <span className="location-tag location-scene">
                            📺 {alert.sceneName}
                          </span>
                        )}
                        {alert.sourceName && (
                          <span className="location-tag location-source">
                            📦 {alert.sourceName}
                          </span>
                        )}
                      </div>
                    )}
                    <div className="alert-message">
                      <span className="message-icon">
                        {alert.severity === "error" ? "❌" : "⚠️"}
                      </span>
                      {alert.message}
                    </div>
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>

      <style>{`
        .alert-panel {
          display: flex;
          flex-direction: column;
          gap: 1rem;
        }

        .alert-panel.empty {
          text-align: center;
          padding: 3rem 2rem;
          background: rgba(16, 185, 129, 0.1);
          border: 2px dashed var(--success-color);
          border-radius: 0.75rem;
        }

        .empty-icon {
          font-size: 3rem;
          margin-bottom: 1rem;
        }

        .empty-subtitle {
          color: var(--text-secondary);
          font-size: 0.875rem;
          margin-top: 0.5rem;
        }

        .alert-panel-header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
          gap: 1rem;
          padding-bottom: 1rem;
          border-bottom: 2px solid var(--border-color);
          flex-wrap: wrap;
        }

        .header-title-section {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
        }

        .alert-panel-header h3 {
          font-size: 1.5rem;
          font-weight: 700;
          color: var(--text-primary);
          margin: 0;
        }

        .alert-stats {
          display: flex;
          gap: 0.5rem;
          flex-wrap: wrap;
        }

        .stat-badge {
          padding: 0.375rem 0.75rem;
          border-radius: 0.5rem;
          font-size: 0.875rem;
          font-weight: 600;
          background: var(--surface-color);
          border: 1px solid var(--border-color);
          color: var(--text-secondary);
          transition: all 0.2s ease;
        }

        .stat-badge.stat-warning {
          border-color: var(--warning-color);
          color: var(--warning-color);
        }

        .stat-badge.stat-warning.active {
          background: rgba(245, 158, 11, 0.1);
        }

        .stat-badge.stat-error {
          border-color: var(--danger-color);
          color: var(--danger-color);
        }

        .stat-badge.stat-error.active {
          background: rgba(239, 68, 68, 0.1);
        }

        .stat-badge.stat-total {
          border-color: var(--primary-color);
          color: var(--primary-color);
        }

        .header-controls {
          display: flex;
          gap: 0.5rem;
          align-items: center;
          flex-wrap: wrap;
        }

        .filter-select,
        .sort-select {
          padding: 0.5rem 0.75rem;
          border: 1px solid var(--border-color);
          border-radius: 0.5rem;
          background: var(--bg-color);
          color: var(--text-primary);
          font-size: 0.875rem;
          cursor: pointer;
          transition: all 0.2s ease;
        }

        .filter-select:focus,
        .sort-select:focus {
          outline: none;
          border-color: var(--primary-color);
          box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
        }

        .btn-clear-all {
          padding: 0.5rem 1rem;
          background: var(--danger-color);
          color: white;
          border: none;
          border-radius: 0.5rem;
          font-size: 0.875rem;
          font-weight: 600;
          cursor: pointer;
          transition: all 0.2s ease;
        }

        .btn-clear-all:hover {
          background: #dc2626;
          transform: translateY(-1px);
        }

        .alerts-list {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
          max-height: 600px;
          overflow-y: auto;
          padding-right: 0.5rem;
        }

        .alerts-list::-webkit-scrollbar {
          width: 8px;
        }

        .alerts-list::-webkit-scrollbar-track {
          background: var(--bg-color);
          border-radius: 4px;
        }

        .alerts-list::-webkit-scrollbar-thumb {
          background: var(--border-color);
          border-radius: 4px;
        }

        .alerts-list::-webkit-scrollbar-thumb:hover {
          background: var(--text-muted);
        }

        .no-alerts-filtered {
          text-align: center;
          padding: 2rem;
          color: var(--text-secondary);
        }

        .alert-item {
          display: flex;
          gap: 1rem;
          padding: 1rem;
          background: var(--bg-color);
          border: 2px solid var(--border-color);
          border-radius: 0.75rem;
          transition: all 0.2s ease;
          position: relative;
        }

        .alert-item:hover {
          border-color: var(--primary-color);
          box-shadow: var(--shadow);
        }

        .alert-item-warning {
          border-left: 4px solid var(--warning-color);
        }

        .alert-item-error {
          border-left: 4px solid var(--danger-color);
        }

        .alert-severity-indicator {
          width: 4px;
          background: var(--border-color);
          border-radius: 2px;
          flex-shrink: 0;
        }

        .alert-item-warning .alert-severity-indicator {
          background: var(--warning-color);
        }

        .alert-item-error .alert-severity-indicator {
          background: var(--danger-color);
        }

        .alert-content-wrapper {
          flex: 1;
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
        }

        .alert-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .alert-time-group {
          display: flex;
          gap: 0.5rem;
          align-items: center;
        }

        .alert-date {
          font-size: 0.75rem;
          color: var(--text-muted);
          font-weight: 500;
        }

        .alert-time {
          font-size: 0.875rem;
          color: var(--text-secondary);
          font-weight: 600;
          font-family: 'Monaco', 'Courier New', monospace;
        }

        .alert-close {
          width: 24px;
          height: 24px;
          display: flex;
          align-items: center;
          justify-content: center;
          background: transparent;
          border: none;
          color: var(--text-muted);
          font-size: 1.5rem;
          line-height: 1;
          cursor: pointer;
          border-radius: 0.25rem;
          transition: all 0.2s ease;
          padding: 0;
        }

        .alert-close:hover {
          background: var(--surface-hover);
          color: var(--danger-color);
        }

        .alert-content {
          display: flex;
          flex-direction: column;
          gap: 0.5rem;
        }

        .alert-location {
          display: flex;
          gap: 0.5rem;
          flex-wrap: wrap;
        }

        .location-tag {
          padding: 0.25rem 0.5rem;
          border-radius: 0.375rem;
          font-size: 0.75rem;
          font-weight: 600;
          background: var(--surface-color);
          border: 1px solid var(--border-color);
        }

        .location-tag.location-scene {
          color: var(--primary-color);
          border-color: var(--primary-color);
        }

        .location-tag.location-source {
          color: var(--warning-color);
          border-color: var(--warning-color);
        }

        .alert-message {
          display: flex;
          align-items: flex-start;
          gap: 0.5rem;
          color: var(--text-primary);
          font-size: 0.875rem;
          line-height: 1.5;
        }

        .message-icon {
          font-size: 1rem;
          flex-shrink: 0;
          margin-top: 0.125rem;
        }

        @media (max-width: 768px) {
          .alert-panel-header {
            flex-direction: column;
          }

          .header-controls {
            width: 100%;
          }

          .filter-select,
          .sort-select,
          .btn-clear-all {
            flex: 1;
          }

          .alert-item {
            flex-direction: column;
          }

          .alert-severity-indicator {
            width: 100%;
            height: 4px;
          }
        }
      `}</style>
    </div>
  );
};
