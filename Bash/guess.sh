echo "Enter a number"
read number 
if [$number == 12 ]
then 
   echo "a yooo bro you are right" 
   exit
else
while [ number != 12 ] 
do
echo "Enter a number"
read number  

if [ $number -lt 12 ]
then 
  echo "LOL it's greater, guess again" 
else 
    if [ $number -gt 12 ] 
    then 
       echo "LOL man, it's less; try again"
    else 
       echo "a yooo bro you are right" 
       exit
    fi
fi 
done
fi 
