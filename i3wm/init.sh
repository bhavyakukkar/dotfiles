#!/bin/bash

#echo "\""

#xdotool key Alt+Return Alt+r Left Left Escape Alt+v Alt+Return Alt+r Up Up Escape h t o p Return Alt+Left

#exec xterm -e f2

#xterm -e "cat ~/.neofetch && (neofetch > ~/.neofetch &) && $SHELL" &
#xterm -e "xdotool key Alt+r Left Left Escape && $SHELL" &
#xdotool key Alt+v
#xterm -e "xdotool key Alt+r Up Up Escape && htop" &

#exec xterm -e "$(xterm -e "$(xterm -e "xdotool key Alt+r Up Up Up Escape && htop ") && $SHELL") && cat ~/.neofetch && (neofetch > ~/.neofetch &) && $SHELL"
#echo "xterm -e \"xterm -e \"xdotool key Alt+r Up Up Up Escape && htop \" && $SHELL\" && cat ~/.neofetch && (neofetch > ~/.neofetch &) && $SHELL"
#echo "xterm -e \"xdotool key Alt+r Up Up Up Escape && htop \" && $SHELL"
#./test.sh 0

if [ $1 -eq "0" ]; then
	exec xterm -e "(./test.sh 1 &) && cat ~/.neofetch && (neofetch > ~/.neofetch &) && $SHELL"
elif [ $1 -eq "1" ]; then
	exec xterm -e "xdotool key Alt+v && (./test.sh 2 &) && xdotool key Alt+r Left Left Escape && $SHELL"
elif [ $1 -eq "2" ]; then
	exec xterm -e "xdotool key Alt+r Up Up Up Escape && htop"
fi
