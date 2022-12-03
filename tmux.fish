#!/usr/bin/fish

set -l day $argv[1]

cd $day
tmux new -d -s $day
tmux send-keys -t $day 'vim' C-m
tmux new-window -t $day

tmux a
