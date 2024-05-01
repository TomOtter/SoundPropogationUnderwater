

pwd

set term gif animate delay 4 size 600, 600

infile = sprintf("%s%s",".\\outputdata\\", "dataset")

outfile = sprintf("%s%s%s",".\\outputImages", "\\imageGif", ".gif")

set xrange [-1500 : 1500]
set yrange [-2000 : 1000]
set cbrange [0.996:1]  # Set the color range from 0.966 to 1


set out outfile 

limit = 100

do for [i = 0: limit-1] {plot infile.i.".txt" using 1:2:3 with points pt 7 ps 1 palette title sprintf("Time: %.2f", i * 0.01)}

# We should aim to allow the following variables to be passed in: xrange, yrange, number of files, intensity range, time between frames
# Currently alternatives have been hard-coded