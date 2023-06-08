# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# import aliases
if [ -e $HOME/.aliases ]; then
	source $HOME/.aliases
fi

## prompts
# PS1='
# ╭─(\e[34m\]\u\[\e[0m\]) in [\e[95;1m\]\w\[\e[0m\]] at \e[96m\]\@\[\e[0m\]
# ╰─$ '

PS1='
\e[32;107;7m\] \u \[\e[0m\]\e[35;107;1;7m\] \w \[\e[0m\]\e[36;107;7m\] \@ \[\e[0m\]\n$ '
