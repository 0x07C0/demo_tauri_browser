import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [urlOrSearch, setUrlOrSearch] = useState("");

  async function visitUrl() {
    await invoke("visit", { urlOrSearch });
  }

  return (
    <main className="container">
      <h1>Demo browser's default tab</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          visitUrl();
        }}
      >
        <input
          id="url-input"
          onChange={(e) => setUrlOrSearch(e.currentTarget.value)}
          placeholder="URL or a Google search"
        />
        <button onClick={visitUrl} type="submit">Go</button>
      </form>
    </main>
  );
}

export default App;
