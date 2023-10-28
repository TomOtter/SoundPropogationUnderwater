set term jpeg size 600, 600

infile = sprintf("%s%s%s%s","C:\Users\thoma\Documents\GroupProj\SoundProp\dataSets", "output", ARG1, ".txt")

outfile = sprintf("%s%s%s%s","/Users/coto/OneDrive\ -\ Loughborough\ University/Comp\ Data\ Sem\ 2/outputImages", "image", ARG1, ".jpg")

set out outfile 

plot infile