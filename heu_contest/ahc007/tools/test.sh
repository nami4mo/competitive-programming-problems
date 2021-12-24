#!/bin/bash

EXE_FILE="../../../target/debug/ahc007"

cd ~/Desktop/pro_con/8_heuristics/contest/ahc007/
rm $EXE_FILE
cargo build --features dbg
# cargo build

date=`date "+%Y%m%d%H%M%S"`
outdir="./tools/out/$date"
score_file="./tools/out/$date/0_score.txt"

mkdir $outdir
for i in `seq $1 $2`
do
    x="000${i}"
    filename="${x: -4}".txt
    echo ${filename}
    $EXE_FILE < "./tools/in/${filename}" > "$outdir/${x: -4}.txt" 2>> $score_file
    ./tools/vis "./tools/in/${filename}" "$outdir/${x: -4}.txt"
    # cp out.svg $"$outdir/${x: -4}.svg"
    tail -n 1 $score_file
done

# python3 ./tools/ave.py $score_file
# head -n 12 "$score_file.ave.txt"
cp $score_file ~/Desktop/pro_con/8_heuristics/contest/ahc007/tools/0_out.txt