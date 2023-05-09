#!/usr/bin/env bash

# Terminate already running bar instances
# If all your bars have ipc enabled, you can use 
polybar-msg cmd quit
# Otherwise you can use the nuclear option:
# killall -q polybar

# Iterate over monitors and enable polybar for each.
for m in $(xrandr --listmonitors | tail -n +2 | rev | cut -d ' ' -f1 | rev); do
  MONITOR=$m polybar & disown
done
