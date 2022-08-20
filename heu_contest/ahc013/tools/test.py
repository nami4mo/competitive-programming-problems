import subprocess
import os
from datetime import datetime
import argparse
import pyperclip
from tqdm import tqdm

parser = argparse.ArgumentParser()
parser.add_argument('left', type=int)
parser.add_argument('right', type=int)
parser.add_argument('-p', '--pattern', type=int, default=-1)
args = parser.parse_args()

EXE_FILE = '../../../target/release/ahc013'
os.chdir('/Users/shimoo/Desktop/competitive-programming/Solutions/heu_contest/ahc013/')
subprocess.run(f'rm {EXE_FILE}', shell=True)
subprocess.run(f'cargo build --release --features mylocal', shell=True)

date = datetime.now().strftime('%Y%m%d%H%M%S')
OUT_DIR = f'./tools/out/{date}'
TMP_SCORE_OUT_DIR = f'./tools/out/{date}/tmp'
SCORE_FILE = f'./tools/out/{date}/0_score.txt'

subprocess.run(f'mkdir -p {OUT_DIR}', shell=True)
subprocess.run(f'mkdir -p {TMP_SCORE_OUT_DIR}', shell=True)


procs_list = []
MAX_PROC = 8
l, r = args.left, args.right
pattern = args.pattern
n = r-l+1

if not 2 <= pattern <= 5:
    ra = list(range(l, r+1))
else:
    mod_rem = pattern-2
    ls = (l//4)*4 + mod_rem
    ra = list(range(ls, r+1, 4))

print(ra)

result = {}
for ei, i in enumerate(tqdm(ra)):
    filename = f'{i:04}.txt'
    tmp_score_filename = f'{i:04}.txt'
    cmd = f'{EXE_FILE} < ./tools/in/{filename} > {OUT_DIR}/{i:04}.txt 2>> {TMP_SCORE_OUT_DIR}/{i:04}.txt'
    proc = subprocess.Popen(cmd, shell=True)
    procs_list.append((i, proc))
    if (ei + 1) % MAX_PROC == 0 or ei == len(ra)-1:
        for pi, subproc in procs_list:
            subproc.wait()
        procs_list = []

res = []
res_vi = []
for i in ra:
    with open(f'{TMP_SCORE_OUT_DIR}/{i:04}.txt', 'r') as f:
        rows = f.readlines()
        # print(rows)
        for v in rows:
            if 'Score = ' in v:
                v = v.replace('Score = ', '')
                try:
                    v = float(v)
                    res.append(v)
                    res_vi.append((v, i))
                except:
                    print('error', i)


with open(SCORE_FILE, 'w') as f:
    for v in res:
        f.write(str(v)+'\n')

ans = ''
with open(f'{OUT_DIR}/{ra[-1]:04}.txt', 'r') as f:
    for row in f.readlines():
        ans += row.rstrip()
        ans += '\n'
    pyperclip.copy(ans)

for v, i in res_vi:
    print(f'{i:04}: ', v)
