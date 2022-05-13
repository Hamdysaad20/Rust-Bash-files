#!/bin/bash
# tasks
#• Create a directory with name Direc1. D
#• Create a bash script inside Direc1 Folder does that.D
# o Read two values from the user.D
# o Find out the odd and even number between these two values.
#  o Run this bash script
   
   mkdir Direc1
   cd Direc1
nano bashFile.sh



   
   read -p "Enter the first Number : " num1
   read -p "Enter the sec Number : " num2

if [ `expr $num1 % 2` == 0 ]
then
	echo "$num1 is even"
else
	echo "$num1 is Odd"
fi

if [ `expr $num2 % 2` == 0 ]
then
	echo "$num1 is even"
else
	echo "$num1 is Odd"
fi```

ctrl-x y enter
   chmod +x bashFile.sh
   ./bashFile.sh
