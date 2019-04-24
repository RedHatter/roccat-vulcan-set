#!/bin/bash
#
# Set colors on keypad based on cpu load, cpu temps, and ram usage.
#
# Setup:
#   A couple packages and a little setup are needed to obtain system
#   information.
#
#     $ sudo apt install lm-sensors sysstat
#     $ sudo sensors-detect
#     $ reboot
#
##

# Color green-yellow-red for 0-100
color_percent() {
  # Scale to 0-510
  color=`bc <<< "($1*5.1)/1"`

  if [ $color -lt 256 ]; then
    red=$color
    green=255
  else
    red=255
    green=$((510 - color))
  fi

  echo $red $green 0
}

# Extract core temperature and calculate color
temp_color() {
  if [ -z "$3" ]; then
    echo 0 0 0
    return
  fi

  # temperature is in the third column
  temp=${3%°C}
  temp=${temp:1}
  color_percent $temp
}

# Extract total core load and calculate color
core_color() {
  if [ -z "${13}" ]; then
    echo 0 0 0
    return
  fi

  # The 13th column shows idle percentage
  load=`bc <<< "100 - ${13}"`
  color_percent $load
}

get_values () {
  IFS=$'\n'
  cores=(`mpstat -P all,0-3`)
  temps=(`sensors|grep Core`)
  ram=(`free -b`)
  IFS=$' '

  ram=(${ram[1]})
  ram=`bc <<< "scale=3;${ram[2]}/${ram[1]}*100"`

  echo \
  KP0 `core_color ${cores[2]}` \
  NUMLOCK `core_color ${cores[3]}` \
  KP7 `core_color ${cores[4]}` \
  KP4 `core_color ${cores[5]}` \
  KP1 `core_color ${cores[6]}` \
  KPASTERISK `color_percent $ram` \
  KPSLASH `temp_color ${temps[0]}` \
  KP8 `temp_color ${temps[1]}` \
  KP5 `temp_color ${temps[2]}` \
  KP2 `temp_color ${temps[3]}`
}

run () {
  while true; do
    get_values
    sleep 1
  done
}

run|roccat-vulcan-set
