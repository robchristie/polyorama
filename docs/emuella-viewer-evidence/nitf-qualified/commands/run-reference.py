"""Compare retained application records with complete selected-tile file decodes."""
import hashlib,json,subprocess,time
from pathlib import Path
s=Path('/nvme/development/emuella/.build-targets/polyorama-proof');d=s/'nitf-final';journeys=d/'composed-display98';out=d/'reference';out.mkdir(exist_ok=False)
roots=[d/'qualified-43008-warm-performance/representation']+[s/'final-public/representations'/f'scene-{i:02}' for i in range(1,9)]
by_tid={json.loads((r/'manifest.json').read_text())['tid']:r for r in roots}
def write(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
groups=[];records_by_mode={}
for mode in ['native','browser']:
 p=journeys/(mode+'-01');snapshots=[json.loads((p/'app.json').read_text())]+json.loads((p/'app.json.stages.json').read_text())
 if mode=='browser':snapshots+=json.loads((p/'browser.json').read_text())['samples']
 unique={json.dumps(r,sort_keys=True):r for snap in snapshots for r in snap.get('worker',{}).get('decoded_evidence',[])}
 records_by_mode[mode]=unique
 for tid,root in by_tid.items():
  records=[r for r in unique.values() if r['tid']==tid];assert records,(mode,tid);assert len(records)<=1024
  name=mode+'-'+json.loads((root/'manifest.json').read_text())['target'];requests=out/(name+'.requests.json');write(requests,records)
  result=out/(name+'.reference.json');stderr=out/(name+'.stderr');cmd=[str(d/'build/emuella-viewer-tools'),'reference','--representation',str(root),'--requests',str(requests)]
  start=time.monotonic()
  with result.open('w') as stdout,stderr.open('w') as errors:r=subprocess.run(cmd,stdout=stdout,stderr=errors)
  row={'mode':mode,'tid':tid,'command':cmd,'records':len(records),'requests_sha256':sha(requests),'output_sha256':sha(result),'returncode':r.returncode,'wall_seconds':time.monotonic()-start}
  if r.returncode==0:
   report=json.loads(result.read_text());row.update({k:report[k] for k in ['compared_pixels','mismatched_records','metrics']})
  groups.append(row);write(out/'progress.json',groups);print(name,len(records),r.returncode,flush=True)
a=records_by_mode['native'];b=records_by_mode['browser'];key=lambda r:json.dumps({k:v for k,v in r.items() if k!='fnv1a64_u16le'},sort_keys=True)
na={key(r):r['fnv1a64_u16le'] for r in a.values()};nb={key(r):r['fnv1a64_u16le'] for r in b.values()};common=na.keys()&nb.keys();mismatch=[k for k in common if na[k]!=nb[k]]
summary={'groups':groups,'group_count':len(groups),'passed_groups':sum(g['returncode']==0 for g in groups),'record_count':sum(g['records'] for g in groups),'compared_pixels':sum(g.get('compared_pixels',0) for g in groups),'mismatched_records':sum(g.get('mismatched_records',0) for g in groups),'native_browser_shared_records':len(common),'native_browser_mismatches':mismatch,'binary_sha256':sha(d/'build/emuella-viewer-tools'),'binary_source_revision':'16296520e146df842555a8d763fcce704207adc8','codec_revision':'2568f1c40c83a40f527c7ee8f1600af511e046d0'}
write(out/'comparisons.json',summary);assert len(groups)==18 and all(g['returncode']==0 for g in groups) and not mismatch
