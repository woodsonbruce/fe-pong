# fe-pong

This is Iron Pong.

- pong.rs encodes the game and is compiled to WASM
- game_server.rs is built using Rocket.rs


Requirements:

- Docker
- Any modern browser


At the prompt:

1. git clone https://github.com/woodsonbruce/fe-pong.git
2. cd fe-pong/
3. sudo docker build -t fe-pong . && sudo docker run -p 5000:5000 fe-pong

In the browser:  play pong at http://<host-ip>:5000 
