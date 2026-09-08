"""Run the prepared NITF parent and eight unchanged authored parents."""
import hashlib,json,os,subprocess,time,urllib.request
from pathlib import Path
root=Path('/nvme/development/emuella/worktrees/polyorama-proof/polyorama')
s=Path('/nvme/development/emuella/.build-targets/polyorama-proof');d=s/'nitf-final';b=d/'build';out=d/'composed';out.mkdir(exist_ok=False)
env=dict(os.environ,DISPLAY=':97',LD_LIBRARY_PATH=str(root/'.tools/sysroot/usr/lib')+':/home/rob/.cache/ms-playwright/chromium-1234/chrome-linux64',VK_DRIVER_FILES='/usr/share/vulkan/icd.d/nvidia_icd.json',WGPU_BACKEND='vulkan',XDG_RUNTIME_DIR='/run/user/3000')
roots=[d/'qualified-43008-warm-performance/representation']+[s/'final-public/representations'/f'scene-{i:02}' for i in range(1,9)]
def write(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for block in iter(lambda:f.read(1<<20),b''):h.update(block)
 return h.hexdigest()
def input_state():return {str(p):{'bytes':p.stat().st_size,'sha256':sha(p)} for r in roots for p in sorted(r.rglob('*')) if p.is_file()}
write(out/'inputs-before.json',input_state());write(out/'build-attestation.json',json.loads((d/'build-attestation.json').read_text()))
cmd=[str(b/'emuella-viewer-tools'),'serve','--listen','127.0.0.1:8128','--verify-payload','true','--web',str(b/'web')]
for r in roots:cmd+=['--representation',str(r)]
records=[];write(out/'server-command.json',cmd)
with (out/'service.stdout').open('w') as stdout,(out/'service.stderr').open('w') as stderr:server=subprocess.Popen(cmd,cwd=root,env=env,stdout=stdout,stderr=stderr,start_new_session=True)
try:
 for i in range(100):
  try:
   with urllib.request.urlopen('http://127.0.0.1:8128/catalogue',timeout=2) as f:cat=json.load(f)
   assert len(cat)==9 and cat[0]['identity']['profile']['width']==43008
   break
  except Exception:time.sleep(.1)
 else:raise RuntimeError('server startup failed')
 for mode in ['native','browser','recovery']:
  limits=root/'apps/emuella-viewer/qualification'/({'native':'native','browser':'browser-complete','recovery':'recovery'}[mode]+'-thresholds.json')
  cmd=['python3',str(root/'tools/viewer-composed-journey.py'),'--mode',mode,'--output',str(out/(mode+'-01')),'--url','http://127.0.0.1:8128','--native-bin',str(b/'emuella-viewer'),'--web-root',str(b/'web'),'--codec-repo','/nvme/development/emuella/emuella-j2k','--benchmark-repo','/nvme/development/emuella/emuella-benchmark','--server-cache-state','uncontrolled-first-observation' if mode=='native' else 'warm-server','--thresholds',str(limits)]
  start=time.time_ns()//1000000
  with (out/(mode+'.stdout')).open('w') as stdout,(out/(mode+'.stderr')).open('w') as stderr:r=subprocess.run(cmd,cwd=root,env=env,stdout=stdout,stderr=stderr)
  admission=[str(s/'benchmark/final-target/release/emuella-benchmark'),'journey',str(out/(mode+'-01')/'composed-trace.json'),str(limits)]
  with (out/(mode+'-admission.json')).open('w') as stdout,(out/(mode+'-admission.stderr')).open('w') as stderr:admitted=subprocess.run(admission,cwd=root,env=env,stdout=stdout,stderr=stderr)
  records.append({'admission_command':admission,'admission_returncode':admitted.returncode,'mode':mode,'command':cmd,'started_unix_ms':start,'completed_unix_ms':time.time_ns()//1000000,'returncode':r.returncode,'threshold_sha256':sha(limits)});write(out/'commands.json',records);print(mode,r.returncode,flush=True)
finally:
 server.terminate();server.wait(timeout=10);write(out/'inputs-after.json',input_state())
assert all(r['returncode']==0 and r['admission_returncode']==0 for r in records),records
assert json.loads((out/'inputs-before.json').read_text())==json.loads((out/'inputs-after.json').read_text())
