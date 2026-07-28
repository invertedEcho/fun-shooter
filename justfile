debug-client:
  RUST_LOG="game_core=debug,client=debug,info" cargo run -p client

debug-server:
  RUST_LOG="game_core=debug,server=debug,info" cargo run -p server
