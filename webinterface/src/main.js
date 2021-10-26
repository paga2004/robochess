import { Chessground } from "chessground";
import init, { get_best_move } from "../chers_wrapper/pkg/chers_wrapper.js";

async function run() {
  await init();

  const socket = new WebSocket("ws://192.168.0.4:8080");

  // Listen for messages
  socket.addEventListener("message", function (event) {
    console.log("Message from server ", event.data);
    if (event.data != "OK") {
      console.error(event.data);
    }
  });
  socket.addEventListener("open", function (event) {
    console.log("Connected!");

    function afterMove(orig, dest, metadata) {
      let m1 = orig + dest;
      console.log(m1);
      socket.send(m1);

      let fen = cg.getFen();
      let m2 = get_best_move(fen, 3);
      socket.send(m2);

      let from = m2.substr(0, 2);
      let to = m2.substr(2, 2);

      console.log(from, to);
      cg.move(from, to);
      cg.set({ turnColor: "white" });
    }

    const config = {
      coordinates: false,
      movable: {
        color: "white", // only allow white pieces to be moved (it's white's turn to start)
        free: true, // don't allow movement anywhere ...
        events: {
          after: afterMove, // called after the move has been played
        },
      },
      draggable: {
        showGhost: true,
      },
    };

    const container = document.getElementById("board-container");
    const cg = Chessground(container, config);
  });
}
run();
