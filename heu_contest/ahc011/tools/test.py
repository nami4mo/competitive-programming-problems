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

EXE_FILE = '../../../target/release/ahc011'
os.chdir('/Users/shimoo/Desktop/competitive-programming/Solutions/heu_contest/ahc011/')
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

if not 6 <= pattern <= 10:
    ra = list(range(l, r+1))
else:
    mod_rem = pattern-6
    ls = (l//5)*5 + mod_rem
    ra = list(range(ls, r+1, 5))


result = {}
for ei, i in enumerate(tqdm(ra)):
    # print(ei, i)
    filename = f'{i:04}.txt'
    tmp_score_filename = f'{i:04}.txt'
    # if ei % MAX_PROC == 0:
    # print(filename)
    # cmd = f'./tools/tester {EXE_FILE} < ./tools/in/{filename} > {OUT_DIR}/{i:04}.txt 2>> {TMP_SCORE_OUT_DIR}/{i:04}.txt'
    cmd = f'{EXE_FILE} < ./tools/in/{filename} > {OUT_DIR}/{i:04}.txt 2>> {TMP_SCORE_OUT_DIR}/{i:04}.txt'
    # print(cmd)
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
        # if v >= 35000000:


with open(SCORE_FILE, 'w') as f:
    for v in res:
        f.write(str(v)+'\n')

ans = ''
with open(f'{OUT_DIR}/{ra[-1]:04}.txt', 'r') as f:
    for row in f.readlines():
        ans += row.rstrip()
        ans += '\n'
    pyperclip.copy(ans)

if res:
    res_tree = []
    for v in res:
        if v >= 500000:
            res_tree.append(v)

    res5 = [[] for _ in range(5)]
    no_tree5 = [0]*5
    for v, i in res_vi:
        if v >= 500000:
            res5[i % 5].append(v)
        else:
            no_tree5[i % 5] += 1

    print('len: ', len(res))
    print('[res5]')
    for i in range(5):
        if res5[i]:
            print(i, sum(res5[i])/len(res5[i]))
    print('no tree', no_tree5)

    print(f'[ ave ] {sum(res)//len(res)}')
    print(f'[ tree] {sum(res_tree)//len(res_tree)}')
    print('[worst]')
    res_vi.sort()
    for v, i in res_vi[:min(n, 15)]:
        print(f'{i:02} {(i%5+6)} :', v)
    notree = 0
    for v, i in res_vi:
        if v < 500000.0:
            notree += 1
    print('notree:', notree)
