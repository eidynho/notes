import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import "./index.css";

export function App() {
  useEffect(() => {
    async function setAppOnRight() {
      try {
        await invoke("set_window_position_right");
      } catch (e) {
        console.error("Failed to set window position:", e);
      }
    }

    setAppOnRight();
  }, []);
  return (
    <>
      <div className="h-full text-sm bg-gray-300/90 backdrop-blur-md border border-white rounded-lg py-3 px-4">
        Notes app
      </div>
    </>
  );
}
