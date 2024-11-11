import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  function back() {
    invoke("back");
  }
  function forward() {
    invoke("forward");
  }

  return (
    <main className="navbar">
        <button className="nav" onClick={back}>Back</button>
        <button className="nav" onClick={forward}>Forward</button>
    </main>
  );
}

export default App;
