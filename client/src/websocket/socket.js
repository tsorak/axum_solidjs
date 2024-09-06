import handle_message from "./message_handler.js";

class ClientSocket {
  socket;
  path;
  reloadPageOnReconnect;
  connection;

  constructor(p) {
    const path = socketPathFromEndpoint(p);

    let didConnect;
    this.connection = new Promise((res, _) => {
      didConnect = res;
    });

    this.path = path;
    this.connect(path, didConnect);
  }

  async connect(p, didConnect) {
    const s = await connect(p);
    console.log("[CS] Connected");
    didConnect();

    const [socket, didClose] = attachHandlers(s);

    this.autoReconnect(didClose);

    this.socket = socket;
  }

  async autoReconnect(didClose) {
    await didClose;

    let didConnect;
    this.connection = new Promise((res, _) => {
      didConnect = res;
    })

    console.log("[CS] Reconnecting...");

    if (this.reloadPageOnReconnect) {
      await this.connect(this.path, didConnect);
      window.location.reload();
    } else {
      this.connect(this.path, didConnect);
    }
  }

  /**
   * @param {boolean} b
   */
  doReloadPageOnReconnect(b) {
    this.reloadPageOnReconnect = b;
  }

  async send(data) {
    await this.connection;

    this.socket.send(data);
  }

  devtools() {
    if (import.meta.env.DEV == true && import.meta.env.MODE == "development") {
      globalThis.rebuild = () => this.rebuild();
    }
  }

  rebuild() {
    this.socket.send("rebuild");
  }
}

function socketPathFromEndpoint(p) {
  const url = new URL(window.location.origin + p);
  let ws_path = "";

  if (url.protocol === "http:") {
    ws_path = url.href.replace("http", "ws");
  } else {
    ws_path = url.href.replace("https", "wss");
  }

  return ws_path;
}

function connect(path) {
  const p = new Promise((res, _rej) => {
    connectLoop(path, res);
  });

  return p;
}

function connectLoop(path, res, retry_count = 0) {
  console.log("[CS] Connecting... (" + retry_count + ")");
  const socket = new WebSocket(path);
  socket.onclose,
    socket.onerror = () => {
      return connectLoop(path, res, retry_count + 1);
    };
  socket.onopen = (_this, _ev) => {
    socket.onopen, socket.onclose, socket.onerror = null;
    return res(socket);
  };
}

/**
 * @param {WebSocket} socket
 */
function attachHandlers(socket) {
  socket.onmessage = (ev) => handle_message(ev);

  const didClose = new Promise((res, _rej) => {
    const closer = () => {
      console.log("[CS] Connection closed");
      socket.onclose = null;
      socket.onerror = null;
      setTimeout(() => res(), 1000);
    };

    socket.onclose = (_ev) => closer();
    socket.onerror = (_ev) => closer();
  });

  return [socket, didClose];
}

export default ClientSocket;
