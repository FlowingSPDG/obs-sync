import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NetworkStatus, ConnectionState } from "../types/network";

interface NetworkConfig {
  host: string;
  port: number;
}

export const useNetworkStatus = () => {
  const [status, setStatus] = useState<NetworkStatus>({
    state: ConnectionState.Disconnected,
  });
  const [error, setError] = useState<string | null>(null);
  const pollingIntervalRef = useRef<number | null>(null);

  const updateClientCount = useCallback(async () => {
    try {
      const count = await invoke<number>("get_connected_clients_count");
      setStatus((prev) => {
        if (prev.state === ConnectionState.Connected) {
          return { ...prev, connectedClients: count };
        }
        return prev;
      });
    } catch (err) {
      console.error("Failed to get connected clients count:", err);
    }
  }, []);

  const startMasterServer = useCallback(async (port: number) => {
    try {
      setStatus({ state: ConnectionState.Connecting });
      await invoke("start_master_server", { port });
      setStatus({ state: ConnectionState.Connected, connectedClients: 0 });
      setError(null);
      
      // Start polling for client count
      updateClientCount();
      pollingIntervalRef.current = window.setInterval(updateClientCount, 1000);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setStatus({
        state: ConnectionState.Error,
        lastError: errorMessage,
      });
      throw err;
    }
  }, [updateClientCount]);

  const stopMasterServer = useCallback(async () => {
    try {
      // Stop polling
      if (pollingIntervalRef.current !== null) {
        clearInterval(pollingIntervalRef.current);
        pollingIntervalRef.current = null;
      }
      
      await invoke("stop_master_server");
      setStatus({ state: ConnectionState.Disconnected });
      setError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    }
  }, []);

  const connectToMaster = useCallback(async (config: NetworkConfig) => {
    try {
      setStatus({ state: ConnectionState.Connecting });
      await invoke("connect_to_master", { config });
      setStatus({ state: ConnectionState.Connected });
      setError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setStatus({
        state: ConnectionState.Error,
        lastError: errorMessage,
      });
      throw err;
    }
  }, []);

  const disconnectFromMaster = useCallback(async () => {
    try {
      await invoke("disconnect_from_master");
      setStatus({ state: ConnectionState.Disconnected });
      setError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    }
  }, []);

  // Cleanup polling on unmount
  useEffect(() => {
    return () => {
      if (pollingIntervalRef.current !== null) {
        clearInterval(pollingIntervalRef.current);
      }
    };
  }, []);

  return {
    status,
    error,
    startMasterServer,
    stopMasterServer,
    connectToMaster,
    disconnectFromMaster,
  };
};
