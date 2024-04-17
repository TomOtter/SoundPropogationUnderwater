

pwd

set term gif animate delay 4 size 600, 600

infile = sprintf("%s%s",".\\outputdata\\", "dataset")

outfile = sprintf("%s%s%s",".\\outputImages", "\\imageGif", ".gif")

set xrange [-1500 : 1500]
set yrange [-2000 : 1000]


set out outfile 

do for [i = 1: 100] {plot infile.i.".txt"}