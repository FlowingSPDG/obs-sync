import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { DesyncAlert } from "../types/sync";

export const useDesyncAlerts = () => {
  const [alerts, setAlerts] = useState<DesyncAlert[]>([]);

  useEffect(() => {
    // Listen for desync-alert events from Tauri backend
    let unlistenFn: (() => void) | null = null;

    const setupListener = async () => {
      const unlisten = await listen<DesyncAlert>("desync-alert", (event) => {
        console.log("Received desync alert:", event.payload);
        // Ensure severity is lowercase to match TypeScript type
        const alert: DesyncAlert = {
          ...event.payload,
          severity: (event.payload.severity as string).toLowerCase() as "warning" | "error",
        };
        setAlerts((prev) => [alert, ...prev].slice(0, 50)); // Keep last 50 alerts
      });
      unlistenFn = unlisten;
    };

    setupListener();

    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, []);

  const clearAlert = (id: string) => {
    setAlerts((prev) => prev.filter((alert) => alert.id !== id));
  };

  const clearAllAlerts = () => {
    setAlerts([]);
  };

  return {
    alerts,
    clearAlert,
    clearAllAlerts,
  };
};
