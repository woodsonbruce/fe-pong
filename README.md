# fe-pong

This is Iron Pong.  Iron Pong is single-player and does not keep score.

- src/bin/pong.rs encodes the game and is compiled to WASM
- src/bin/game_server.rs is built using Rocket.rs


Requirements:

- git
- Docker
- browser


At the prompt:

1. git clone https://github.com/woodsonbruce/fe-pong.git
2. cd fe-pong/
3. sudo docker build -t fe-pong . && sudo docker run -p 5000:5000 fe-pong


Then use up and down arrows to play at http://\<host-ip\>:5000

![pong](assets/pong.png)
