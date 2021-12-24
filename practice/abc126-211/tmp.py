if ymax_d=='R' or ymax_d=='L':
    if xyld['U']:
        dist=ymax-xyld['U'][-1][0]
        t_cands.append(dist)
elif ymax_d=='D':
    if xyld['U']:
        for i in range(len(xyld['U'])):
            x,y=xyld['U'][i]
            if y>ymax and i>0:
                dist=(ymax-xyld['U'][i-1][0])/2
                t_cands.append(dist)
                break
        else:
            dist=(ymax-xyld['U'][-1][0])/2
            t_cands.append(dist)
    if xyld['R']:
        for i in range(len(xyld['R'])):
            x,y=xyld['R'][i]
            if y>ymax and i>0:
                dist=(ymax-xyld['R'][i-1][0])
                t_cands.append(dist)
                break
        else:
            dist=(ymax-xyld['R'][-1][0])/2
            t_cands.append(dist)
    if xyld['L']:
        for i in range(len(xyld['L'])):
            x,y=xyld['L'][i]
            if y>ymax and i>0:
                dist=(ymax-xyld['L'][i-1][0])
                t_cands.append(dist)
                break
        else:
            dist=(ymax-xyld['L'][-1][0])/2
            t_cands.append(dist)


if ymin_d=='R' or ymin_d=='L':
    if xyld['D']:
        dist=xyld['D'][-1][0]-ymin
        t_cands.append(dist)
elif ymin_d=='U':
    if xyld['D']:
        for i in range(len(xyld['D'])):
            x,y=xyld['D'][i]
            if y>=ymin:
                dist=(xyld['D'][i][0]-ymin)/2
                t_cands.append(dist)
                break
    if xyld['R']:
        for i in range(len(xyld['R'])):
            x,y=xyld['D'][i]
            if y>=ymin:
                dist=(xyld['R'][i][0]-ymin)
                t_cands.append(dist)
                break
    if xyld['L']:
        for i in range(len(xyld['L'])):
            x,y=xyld['L'][i]
            if y>=ymin:
                dist=(xyld['L'][i][0]-ymin)
                t_cands.append(dist)
                break