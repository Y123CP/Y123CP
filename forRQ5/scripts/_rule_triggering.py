#!/usr/bin/env python3
""                                                                                     
                                                                                                                        
   
import json,sys,subprocess,statistics as st
from collections import defaultdict
from pathlib import Path
ARTIFACT = Path(__file__).resolve().parents[2]
ROOT = ARTIFACT / 'forRQ4' / 'dataset_trans'
sys.path.insert(0,str(ARTIFACT / 'PerfTrans' / 'script_c2rust'))
from tools.facet_coverage import RULE_OF,_base
sys.path.insert(0,str(ROOT))
from _op_groups import GROUPS
from _commit_history import commits
RULES=["Residual Bounds-Check Elimination","Range-Proven Conversion Simplification","Vectorization Restoration","Hot-Callee Inlining Restoration","Invariant Dispatch Specialization","Owned Buffer Management","Typed Memory Operations","Data-Level Parallelism Exposure","Memory Property Recovery"]
fired_fn=defaultdict(set); fired_proj=defaultdict(set); land_proj=defaultdict(set); land_n=defaultdict(int); land_fn=defaultdict(set)
allfn=set(); hotfn=set(); proj_fired=defaultdict(set); proj_land=defaultdict(set)
for p in GROUPS:
    d=str(ROOT / p / '3_perf_opt')
    for h in json.load(open(d+'/hotspots.json'))['hot_functions']: hotfn.add((p,h['name']))
    sha2fn={}
    for l in open(d+'/rewrites.log'):
        if not l.strip(): continue
        r=json.loads(l); fn=r['fn_name']; allfn.add((p,fn))
        for c in r.get('fired_rules') or []:
            rule=RULE_OF.get(_base(c))
            if rule: fired_fn[rule].add((p,fn)); fired_proj[rule].add(p); proj_fired[p].add(rule)
        if r.get('commit_sha'): sha2fn[r['commit_sha'][:7]]=fn
    for sha,subj in commits(Path(d)):
        rules={RULE_OF[_base(c.strip())] for c in subj.split(':',1)[1].split(',') if c.strip() and _base(c.strip()) in RULE_OF}
        fn=sha2fn.get(sha[:7])
        for rule in rules:
            land_proj[rule].add(p); land_n[rule]+=1; proj_land[p].add(rule)
            if fn: land_fn[rule].add((p,fn))
print('hot fns (hotspots)',len(hotfn),'fns in log',len(allfn))
for r in RULES:
    print(f"{r:40s} fired fns {len(fired_fn[r]):3d} ({100*len(fired_fn[r])/len(hotfn):4.1f}%)  fired proj {len(fired_proj[r]):2d}  landed proj {len(land_proj[r]):2d}  rewrites {land_n[r]:2d}  landed fns {len(land_fn[r])}")
fp=[len(proj_fired[p]) for p in GROUPS]; lp=[len(proj_land[p]) for p in GROUPS]
print('rules fired per project',dict((p,len(proj_fired[p])) for p in GROUPS),'median',st.median(fp),'min',min(fp))
print('rules landed per project median',st.median(lp))
fnk=defaultdict(int)
for r in RULES:
    for x in fired_fn[r]: fnk[x]+=1
ks=[fnk.get(x,0) for x in hotfn]
print('hot fns with >=1 rule fired',sum(k>=1 for k in ks),'>=2',sum(k>=2 for k in ks),'median',st.median(ks))
print('fired fn not in hotspots',len(set().union(*fired_fn.values())-hotfn))
json.dump({r:{'fired_proj':sorted(fired_proj[r]),'land_proj':sorted(land_proj[r])} for r in RULES},open(sys.argv[1],'w'),indent=1)
