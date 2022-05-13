# tasks
#• Create a directory with name FizFuzProject. 
#• Create a bash script inside FizFuzProject Folder does that.
# o Read two values from the user.
# o Find out the odd and even number between these two values.
#  o Run this bash script

cd Home
mkdir FizFuzBuzProject
cd FizFuzBuzProject
touch BashScript.sh
chmod +x BashScript.sh
clear
vi BashScript.sh
read -p "Put A Var : " MyVar 

if[$MyVar ]
if[`expr $MyVar % 2 `== 0 ]
then
   echo "Fiz"
else
   echo "Fuz"
fi
