import { useState } from "react";
import { SyncTargetType } from "../types/obs";
import { useSyncState } from "../hooks/useSyncState";

export const SyncTargetSelector = () => {
  const [selectedTargets, setSelectedTargets] = useState<SyncTargetType[]>([
    SyncTargetType.Program,
    SyncTargetType.Source,
  ]);
  const { setSyncTargets } = useSyncState();

  const handleToggleTarget = (target: SyncTargetType) => {
    setSelectedTargets((prev) => {
      const newTargets = prev.includes(target)
        ? prev.filter((t) => t !== target)
        : [...prev, target];
      
      // Apply changes
      setSyncTargets(newTargets).catch(console.error);
      
      return newTargets;
    });
  };

  return (
    <div className="sync-target-selector">
      <h3>同期対象</h3>
      
      <div className="target-options">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={selectedTargets.includes(SyncTargetType.Source)}
            onChange={() => handleToggleTarget(SyncTargetType.Source)}
          />
          <span>ソース (Source)</span>
        </label>

        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={selectedTargets.includes(SyncTargetType.Preview)}
            onChange={() => handleToggleTarget(SyncTargetType.Preview)}
          />
          <span>プレビュー (Preview)</span>
        </label>

        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={selectedTargets.includes(SyncTargetType.Program)}
            onChange={() => handleToggleTarget(SyncTargetType.Program)}
          />
          <span>プログラム (Program)</span>
        </label>
      </div>
    </div>
  );
};
