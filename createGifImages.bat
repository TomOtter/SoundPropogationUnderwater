
cd ./Documents/GroupProj/SoundProp

for /L %%N in (1,1,11) do gnuplot -e 'ARG1=value' %%N

pause