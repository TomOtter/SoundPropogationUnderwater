

pwd

set term gif animate delay 4 size 600, 600

infile = sprintf("%s%s",".\\outputdata\\", "dataset")

outfile = sprintf("%s%s%s",".\\outputImages", "\\imageGif", ".gif")

set xrange [-1500 : 1500]
set yrange [-2000 : 1000]
set cbrange [0:0.00004]  # Set the color range from 0.966 to 1


set out outfile 

limit = 100

do for [i = 0: limit-1] {
    plot infile.i.".txt" using 1:2:3 with points pt 7 ps 1 palette title sprintf("Frame: %.2f", i)}



#Heat map code - Does need fixing (currently just plots 1 large box around all data and leaves the rest blank).
# pwd

# set term gif animate delay 4 size 600, 600

# set view map
# set dgrid3d
# set pm3d interpolate 10,10
# set palette defined (0 'white', 1 'blue')

# infile = sprintf("%s%s",".\\outputdata\\", "dataset")

# outfile = sprintf("%s%s%s",".\\outputImages", "\\imageGif", ".gif")

# set xrange [-1500 : 1500]
# set yrange [-2000 : 1000]
# set cbrange [0:0.00004]  # Set the color range from 0.966 to 1


# set out outfile 

# limit = 100

# do for [i = 0: limit-1] {
#     splot infile.i.".txt" using 1:2:3 with pm3d title sprintf("Frame: %.2f", i)}