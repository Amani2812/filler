FROM rust:1.63-buster

# Reference game engine, opponent bots, and test maps
COPY ./maps               /filler/maps
COPY ./linux_robots       /filler/linux_robots
COPY ./m1_robots          /filler/m1_robots
COPY ./linux_game_engine  /filler/linux_game_engine
COPY ./m1_game_engine     /filler/m1_game_engine

RUN chmod +x /filler/linux_game_engine /filler/linux_robots/* /filler/m1_game_engine /filler/m1_robots/*

WORKDIR /filler

ENTRYPOINT ["/bin/bash"]