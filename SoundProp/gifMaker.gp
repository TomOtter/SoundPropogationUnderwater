

pwd

set term gif animate delay 6 size 650,600
set lmargin at screen 0.10
set rmargin at screen 0.75
set tmargin at screen 0.92
set bmargin at screen 0.10

# Define input and output paths
infile = "./outputdata/dataset"
outfile = "./outputImages/imageGif.gif"
boundary = "./outputdata/boundary"

set xrange [x_min:x_max]
set yrange [y_min:y_max]
set cbrange [0:max_intensity/10000]
set cbtics format "%.1e"
set cblabel "Intensity (W/m^2)" rotate by -90 offset 2,0


# Output to GIF file
set output outfile
set palette color positive
set pm3d map

do for [i = 0:frames-1] {
    current_time = i * duration / (frames * 1.0)
    time_label = sprintf("Time = %.2e seconds", current_time)
    
    set title time_label
    plot infile.sprintf("%d.txt", i) using 1:2:3 with points pt 7 ps 1 palette title sprintf("Sound Ray"), \
        for [j = 0:boundaries-1] boundary.sprintf("%d.txt", j) using 1:2 with lines lw 2 title sprintf("Boundary %d", j)
}

# Close the output file
set output