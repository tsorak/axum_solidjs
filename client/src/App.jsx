import { Router, Route } from "@solidjs/router";

import Layout from "./components/Layout.jsx";

import Home from "./routes/Home.jsx";
import Motd from "./routes/Motd.jsx";
import ClientSocket from "./websocket/socket.js";
import * as appState from "./appState.js";

export default function App() {
  const socket = new ClientSocket("/api/ws");
  socket.doReloadPageOnReconnect(true);
  socket.devtools();

  const AppState = appState.create({ socket });
  const state = appState.use();

  return (
    <AppState.Provider value={state}>
      <Router root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/motd" component={Motd} />
      </Router>
    </AppState.Provider>
  );
}
