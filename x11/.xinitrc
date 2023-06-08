#!/bin/sh

userresources=$HOME/.Xresources
usermodmap=$HOME/.Xmodmap
sysresources=/etc/X11/xinit/.Xresources
sysmodmap=/etc/X11/xinit/.Xmodmap

# merge in defaults and keymaps

if [ -f $sysresources ]; then
    xrdb -merge $sysresources
fi

if [ -f $sysmodmap ]; then
    xmodmap $sysmodmap
fi

if [ -f "$userresources" ]; then
    xrdb -merge "$userresources"
fi

if [ -f "$usermodmap" ]; then
    xmodmap "$usermodmap"
fi


if [ -d /etc/X11/xinit/xinitrc.d ] ; then
 for f in /etc/X11/xinit/xinitrc.d/?*.sh ; do
  [ -x "$f" ] && . "$f"
 done
 unset f
fi

wal -R -n

# twm &
# xclock -geometry 50x50-1+1 &
# xterm -geometry 50x12+770-5 -e "/usr/local/bin/unimatrix -l k -c cyan -s 80; bash" &
# xterm -geometry 50x23+770+10 -e "/usr/bin/htop; bash" &
# exec xterm -geometry 75x35+10+10 -name login -e "/usr/bin/neofetch; bash"

xfwm4 &
exec startxfce4
