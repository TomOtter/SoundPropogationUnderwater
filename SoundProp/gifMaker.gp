

pwd

set term gif animate delay 4 size 600,600

# Define input and output paths
infile = "./outputdata/dataset"
outfile = "./outputImages/imageGif.gif"
boundary = "./outputdata/boundary"

set xrange [x_min:x_max]
set yrange [y_min:y_max]
set cbrange [0:0.0000004]

# Output to GIF file
set output outfile
set palette color positive
set pm3d map

do for [i = 0:frames-1] {
    plot infile.sprintf("%d.txt", i) using 1:2:3 with points pt 7 ps 1 palette title sprintf("Ray at frame %d", i), \
        for [j = 0:boundaries-1] boundary.sprintf("%d.txt", j) using 1:2 with lines lw 2 title sprintf("Boundary %d", j)
}

# Close the output file
set output