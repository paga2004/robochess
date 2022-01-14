import { Chessground } from "chessground";
import init, { get_best_move } from "../chers_wrapper/pkg/chers_wrapper.js";

const config = {
  coordinates: false,
  movable: {
    color: "white", // only allow white pieces to be moved (it's white's turn to start)
    free: true, // don't allow movement anywhere ...
    events: {
      // called after the move has been played
      after: afterMove,
    },
  },
  draggable: {
    showGhost: true,
  },
};

const url = `ws://${location.hostname}:8080`;
const subprotocol = "robochess-websocket";
const container = document.getElementById("board-container");
const slider_container = document.getElementById("slider-container");
const slider = document.getElementById("depth-slider");
const depth_label = document.getElementById("depth-label");
const input_button = document.getElementById("input");
const calibrate_button = document.getElementById("calibrate");
const reset_button = document.getElementById("reset");

let cg, socket;
let depth;

function updateDepth() {
  depth = slider.value;
  depth_label.innerHTML = depth;
}

slider.oninput = updateDepth;

input_button.onclick = function () {
  let m = prompt("Zug:");
  socket.send(m);
};

calibrate_button.onclick = function () {
  socket.send("!calibrate");
};

reset_button.onclick = function () {
  let fen = prompt(
    "Fen:",
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  );
  socket.send("!fen " + fen);
};

function afterMove(orig, dest, metadata) {
  setTimeout(function () {
    let m1 = orig + dest;
    console.log("User move:", m1);
    socket.send(m1);

    let fen = cg.getFen();
    let m2 = get_best_move(fen, depth);
    console.log("Response: ", m2);
    socket.send(m2);

    let from = m2.substr(0, 2);
    let to = m2.substr(2, 2);

    cg.move(from, to);
    cg.set({ turnColor: "black" });
  });
  cg.redrawAll();
}

(async function () {
  // wasm stuff
  await init();
  updateDepth();

  socket = new WebSocket(url, subprotocol);

  // Listen for messages
  socket.addEventListener("message", function (event) {
    let msg = event.data;
    console.log("Message from server:", msg);
    if (msg.startsWith("!")) {
      let commands = msg.split(" ");
      if (commands[0] == "!set") {
        let fen = commands[1];
        cg.set({ fen, ...config });
        cg.redrawAll();
      } else if (commands[0] == "!checkmate") {
        alert("Schachmatt!");
        cg.stop();
      } else if (commands[0] == "!draw") {
        alert("Unentschieden!");
        cg.stop();
      } else if (commands[0] == "!white") {
        cg.set({ turnColor: "white" });
      } else if (commands[0] == "!black") {
        cg.set({ turnColor: "black" });
      }
    } else if (msg != "OK") {
      console.error(msg);
    }
  });

  socket.addEventListener("error", function (event) {
    console.error(event);
    container.innerHTML = "Error";
    slider_container.style.visibility = "hidden";
  });

  socket.addEventListener("open", function (event) {
    console.log("Connected!");
    slider_container.style.visibility = "visible";
    cg = Chessground(container, config);
  });
})();
