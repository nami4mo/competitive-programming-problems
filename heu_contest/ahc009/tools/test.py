from distutils.log import error
from re import X
import subprocess
import os
from datetime import datetime
import argparse
import pyperclip

parser = argparse.ArgumentParser()
parser.add_argument('left', type=int)
parser.add_argument('right', type=int)
args = parser.parse_args()

EXE_FILE = '../../../target/debug/ahc009'
os.chdir('/Users/shimoo/Desktop/competitive-programming/Solutions/heu_contest/ahc009/')
subprocess.run(f'rm {EXE_FILE}', shell=True)
subprocess.run(f'cargo build', shell=True)

date = datetime.now().strftime('%Y%m%d%H%M%S')
OUT_DIR = f'./tools/out/{date}'
TMP_SCORE_OUT_DIR = f'./tools/out/{date}/tmp'
SCORE_FILE = f'./tools/out/{date}/0_score.txt'

subprocess.run(f'mkdir -p {OUT_DIR}', shell=True)
subprocess.run(f'mkdir -p {TMP_SCORE_OUT_DIR}', shell=True)


procs_list = []
MAX_PROC = 8
l, r = args.left, args.right
n = r-l+1

result = {}
for ei, i in enumerate(range(l, r+1)):
    filename = f'{i:04}.txt'
    tmp_score_filename = f'{i:04}.txt'
    if ei % MAX_PROC == 0:
        print(filename)
    # cmd = f'./tools/tester {EXE_FILE} < ./tools/in/{filename} > {OUT_DIR}/{i:04}.txt 2>> {TMP_SCORE_OUT_DIR}/{i:04}.txt'
    cmd = f'{EXE_FILE} < ./tools/in/{filename} > {OUT_DIR}/{i:04}.txt 2>> {TMP_SCORE_OUT_DIR}/{i:04}.txt'
    # print(cmd)
    proc = subprocess.Popen(cmd, shell=True)
    procs_list.append((i, proc))
    if (ei + 1) % MAX_PROC == 0 or i == r:
        for pi, subproc in procs_list:
            subproc.wait()
        procs_list = []

res = []
res_vi = []
for i in range(l, r+1):
    with open(f'{TMP_SCORE_OUT_DIR}/{i:04}.txt', 'r') as f:
        rows = f.readlines()
        for v in rows:
            if 'Score = ' in v:
                v = v.replace('Score = ', '')
                try:
                    v = float(v)
                    res.append(v)
                    res_vi.append((v, i))
                except:
                    print('error', i)
        # if v >= 35000000:


with open(SCORE_FILE, 'w') as f:
    for v in res:
        f.write(str(v)+'\n')

ans = ''
with open(f'{OUT_DIR}/{r:04}.txt', 'r') as f:
    for row in f.readlines():
        ans += row.rstrip()
        ans += '\n'
    pyperclip.copy(ans)

if res:
    print(f'[ ave ] {sum(res)//len(res)}')
    print('[worst]')
    res_vi.sort()
    for v, i in res_vi[:min(n, 10)]:
        print(f'{i:02} :', v)
