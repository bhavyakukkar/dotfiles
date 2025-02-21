# .bashrc

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# import aliases
if [ -e $HOME/.aliases ]; then
	source $HOME/.bash_aliases
fi

# variables
export "MICRO_TRUECOLOR"=1
export EDITOR="vim"

# prompts
#PS1='[\u@\h \W]\$ '
#PS1='\[\e[32;107;7m\] \u \[\e[0m\]\[\e[37;108;1;7m\] \w \[\e[0m\]\[\e[33;107;3;7m\] \@ \[\e[0m\] \[\e[2m\]::\[\e[0m\] '
#PS1='[\[\e[4m\]\u@\h \W]\$\[\e[0m\] '
#PS1='\[\e[38;108;1;7m\] \w \[\e[0m\]  \@ $ '
#PS1='\[\e[38;108;1;7m\] \w \[\e[0m\]  \A $ '
#PS1='\W $ '
#PS1='
#\e[32;107;7m\] \u \[\e[0m\]\e[35;107;1;7m\] \w \[\e[0m\]\e[36;107;7m\] \@ \[\e[0m\]\n$ '
# PS1='[\u@\h \W]\$ '

# fetches
#printf "\n" && uwufetch -d gentoo
#ncneofetch && printf "\n"
#/home/bhavya/programs/fetch/fastfetch/build/flashfetch

# start colors
#colorscript -e zwaves
#colorscript -e panes; printf "\n"
#colorscript -e tiefighter1
#colorscript -e tiefighter1-no-invo
source ~/programs/scripts/shell-mommy.sh; export PROMPT_COMMAND="mommy \\$\\(exit \$?\\); printf '\n'; $PROMPT_COMMAND"

# starship
eval "$(starship init bash)"

# cargo/rust
. "$HOME/.cargo/env"

# nvm
source /usr/share/nvm/init-nvm.sh
