import sys
import glob
import shutil

low=0
high=0
files = glob.glob("./in/*")
for file in files:
    filename=str(file)
    with open(filename, 'r') as f:
        vals = f.read().rstrip().split('\n')
        r=int(vals[0].split()[3])
        if r<2000:
            shutil.copyfile(filename, "./in_low/"+str(low).zfill(4)+".txt")
            low+=1
        else:
            shutil.copyfile(filename, "./in_high/"+str(high).zfill(4)+".txt")
            high+=1