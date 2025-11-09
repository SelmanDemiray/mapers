#!/bin/bash
export DISPLAY=:0
Xvfb :0 -screen 0 1920x1080x24 &
x11vnc -display :0 -nopw -listen localhost -xkb -ncache 10 -ncache_cr -forever &
ppsspp --server --port 8086 &
wait

