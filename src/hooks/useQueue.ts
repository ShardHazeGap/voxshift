import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export function useQueue() {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [running, setRunning] = useState(false);

  const refresh = useCallback(async () => {
    try {
      const snapshot = await invoke<Job[]>("queue_snapshot");
      setJobs(snapshot);
    } catch (e) {
      console.error("queue snapshot failed", e);
    }
  }, []);

  useEffect(() => {
    refresh();
    const t = setInterval(refresh, 1000);
    return () => clearInterval(t);
  }, [refresh]);

  return { jobs, running, refresh };
}

export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);

  useEffect(() => {
    invoke<Settings>("load_settings")
      .then(setSettings)
      .catch(console.error);
  }, []);

  const save = useCallback(async (patch: Partial<Settings>) => {
    const next = { ...(settings ?? defaultSettings), ...patch };
    setSettings(next);
    await invoke("save_settings", { settings: next });
  }, [settings]);

  return { settings, save };
}

const defaultSettings: Settings = {
  theme: "dark",
  outputDir: null,
  copyToClipboard: true,
  parallelJobs: 2,
  keepHistory: 50,
};

export interface Job {
  id: string;
  source: string;
  state: "queued" | "analyzing" | "encoding" | "done" | "failed";
  progress?: number;
  preset: string;
}

export interface Settings {
  theme: string;
  outputDir: string | null;
  copyToClipboard: boolean;
  parallelJobs: number;
  keepHistory: number;
}
