#!/bin/bash

EXE_FILE="../target/debug/httf_2022"

cd ~/Desktop/pro_con/httf_2022/
rm $EXE_FILE
cargo build --features dbg

date=`date "+%Y%m%d%H%M%S"`
outdir="./tools/out/$date"
score_file="./tools/out/$date/0_score.txt"

lh="low"
multi=0
if [ $# -eq 3 ]; then
    if [ $3 = "h" ]; then
        lh="high"
    elif [ $3 = "lh" ]; then
        lh="multi" # TODO
    fi
fi

echo ${lh}
mkdir $outdir
for i in `seq $1 $2`
do
    x="000${i}"
    filename="${x: -4}".txt
    echo ${filename}
    # $EXE_FILE < "./tools/in/${filename}" > "$outdir/${filename}" 2>> $score_file
    $EXE_FILE < "./tools/in_${lh}/${filename}" > "$outdir/${x: -4}_${lh}.txt" 2>> $score_file
    tail -n 1 $score_file
done

python3 ./tools/ave.py $score_file
head -n 12 "$score_file.ave.txt"
cp $score_file ~/Desktop/pro_con/httf_2022/tools/0_out.txt