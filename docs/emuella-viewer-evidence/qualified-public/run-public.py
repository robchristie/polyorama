import subprocess,os,json,time,hashlib,shutil,sys
from pathlib import Path
root=Path('/nvme/development/emuella/worktrees/polyorama-proof/polyorama')
p=Path(__file__).resolve().parent
b=p/'build'
env=dict(os.environ,DISPLAY=':97',LD_LIBRARY_PATH=str(root/'.tools/sysroot/usr/lib')+':/home/rob/.cache/ms-playwright/chromium-1234/chrome-linux64',VK_DRIVER_FILES='/usr/share/vulkan/icd.d/nvidia_icd.json',WGPU_BACKEND='vulkan',XDG_RUNTIME_DIR='/run/user/3000')
records=[]
def run(command,label):
 start=time.time_ns()//1000000
 with (p/(label+'.stdout')).open('w') as out,(p/(label+'.stderr')).open('w') as err:
  result=subprocess.run([str(x) for x in command],cwd=root,env=env,stdout=out,stderr=err)
 records.append(dict(label=label,command=[str(x) for x in command],started_unix_ms=start,completed_unix_ms=time.time_ns()//1000000,exit_code=result.returncode))
 (p/'commands.json').write_text(json.dumps(dict(environment={k:env[k] for k in ['DISPLAY','LD_LIBRARY_PATH','VK_DRIVER_FILES','WGPU_BACKEND','XDG_RUNTIME_DIR']},commands=records),indent=2)+'\n')
 print(label,result.returncode,flush=True)
 return result.returncode
if run(['python3',root/'tools/viewer-prepare-workload.py','--tool',b/'emuella-viewer-tools','--output',p/'representations'],'preparation'):sys.exit(4)
catalogue=json.loads((root/'docs/emuella-viewer-evidence/synthetic-input-catalogue.json').read_text())
command=[str(b/'emuella-viewer-tools'),'serve','--listen','127.0.0.1:8126','--verify-payload','true','--web',str(b/'viewer-web')]
for source in catalogue:command+=['--representation',str(p/'representations'/source['target'])]
with (p/'service.stdout').open('w') as out,(p/'service.stderr').open('w') as err:
 server=subprocess.Popen(command,cwd=root,env=env,stdout=out,stderr=err,start_new_session=True)
(p/'server.json').write_text(json.dumps(dict(pid=server.pid,command=command,started_unix_ms=time.time_ns()//1000000),indent=2)+'\n')
import urllib.request
for attempt in range(100):
 try:
  with urllib.request.urlopen('http://127.0.0.1:8126/catalogue',timeout=2) as r:json.load(r)
  break
 except Exception:time.sleep(.1)
else:raise RuntimeError('service failed startup')
for mode in ['native','browser','recovery']:
 for i in range(1,2 if mode=='recovery' else 6):
  label=f'{mode}-{i:02d}'
  thresholds=b/({'native':'native','browser':'browser-complete','recovery':'recovery'}[mode]+'-thresholds.json')
  run(['python3',root/'tools/viewer-composed-journey.py','--mode',mode,'--output',p/label,'--url','http://127.0.0.1:8126','--native-bin',b/'emuella-viewer','--web-root',b/'viewer-web','--codec-repo','/nvme/development/emuella/emuella-j2k','--benchmark-repo','/nvme/development/emuella/emuella-benchmark','--server-cache-state','uncontrolled-first-observation' if mode=='native' and i==1 else 'warm-server','--thresholds',thresholds],label)
  if (p/label/'composed-trace.json').exists():
   run([b/'emuella-benchmark','journey',p/label/'composed-trace.json',thresholds],label+'-admission')
