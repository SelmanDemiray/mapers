#!/bin/bash
export DISPLAY=:0
Xvfb :0 -screen 0 1920x1080x24 &
x11vnc -display :0 -nopw -listen localhost -xkb -ncache 10 -ncache_cr -forever &
mono /usr/local/BizHawk/EmuHawk.exe --server --port 8092 &
wait

