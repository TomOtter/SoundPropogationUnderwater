
cd ./Documents/GroupProj/

FOR /L %%N in (1,1,199) DO gnuplot -c imageScript1D.gp  %%N 



pause