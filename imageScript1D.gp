

set term jpeg size 600, 600

infile = sprintf("%s%s%s%s","C:\\Users\\thoma\\Documents\\GroupProj\\SoundProp", "\\dataset", ARG1, ".txt")

outfile = sprintf("%s%s%s%s","C:\\Users\\thoma\\Documents\\GroupProj\\SoundProp\\outputImages", "\\image", ARG1, ".jpg")

set xrange [-1500 : 1500]
set yrange [-2000 : 1000]


print ARG1

set out outfile 

plot infile