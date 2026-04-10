import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function App() {
  const [version, setVersion] = useState("");
  const [ready, setReady] = useState(false);

  useEffect(() => {
    invoke<string>("app_version").then(setVersion).catch(console.error);
    invoke("load_config").then(() => setReady(true)).catch(console.error);
  }, []);

  return (
    <div className="app">
      <header>
        <h1>VoxShift</h1>
        <span>{ready ? `v${version}` : "loading…"}</span>
      </header>
    </div>
  );
}
