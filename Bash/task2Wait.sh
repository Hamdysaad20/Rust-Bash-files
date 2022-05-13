#!/bin/bash
# create 2 var date,directory
# two echo strings wait between them 
date = $(date)
root = $(pwd)
me = $(whoami)
echo "What is your Name ?"
read name
echo "Hi $name , it's $date to be exact , and you are here as $me and you are now in $root"
sleep 1
echo "have a nice day "
sleep 2
echo "BTW , you look awesome today tho :)"
