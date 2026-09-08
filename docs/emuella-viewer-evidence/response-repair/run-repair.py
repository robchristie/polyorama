import subprocess,os,json,time,hashlib,shutil,sys,urllib.request,signal
from pathlib import Path
root=Path('/nvme/development/emuella/worktrees/polyorama-proof/polyorama');p=Path(__file__).resolve().parent;base=p.parent;b=p/'build';reps=base/'final-public/representations'
env=dict(os.environ,DISPLAY=':97',LD_LIBRARY_PATH=str(root/'.tools/sysroot/usr/lib')+':/home/rob/.cache/ms-playwright/chromium-1234/chrome-linux64',VK_DRIVER_FILES='/usr/share/vulkan/icd.d/nvidia_icd.json',WGPU_BACKEND='vulkan',XDG_RUNTIME_DIR='/run/user/3000')
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for block in iter(lambda:f.read(1<<20),b''):h.update(block)
 return h.hexdigest()
def write(path,d):path.write_text(json.dumps(d,indent=2)+'\n')
def git(*args):return subprocess.check_output(['git','-C',str(root),*args],text=True).strip()
def state():return dict(revision=git('rev-parse','HEAD'),tree=git('rev-parse','HEAD^{tree}'),status=git('status','--porcelain'),builds={str(f.relative_to(b)):sha(f) for f in sorted(b.rglob('*')) if f.is_file()})
def input_state():
 return dict(root=str(reps),source_preparation_report_sha256=sha(reps/'preparation-result.json'),files={str(f.relative_to(reps)):dict(sha256=sha(f),bytes=f.stat().st_size) for folder in sorted(reps.iterdir()) if folder.is_dir() for f in sorted(folder.rglob('*')) if f.is_file()})
records=[]
def run(command,label):
 start=time.time_ns()//1000000
 with (p/(label+'.stdout')).open('w') as out,(p/(label+'.stderr')).open('w') as err:r=subprocess.run([str(x) for x in command],cwd=root,env=env,stdout=out,stderr=err)
 records.append(dict(label=label,command=[str(x) for x in command],started_unix_ms=start,completed_unix_ms=time.time_ns()//1000000,exit_code=r.returncode))
 write(p/'commands.json',dict(environment={k:env[k] for k in ['DISPLAY','LD_LIBRARY_PATH','VK_DRIVER_FILES','WGPU_BACKEND','XDG_RUNTIME_DIR']},commands=records));print(label,r.returncode,flush=True);return r.returncode
assert git('rev-parse','HEAD')=='abf1b0aa0b0ee2b8b85e94696267b061692dd8f0' and git('status','--porcelain')==''
b.mkdir(exist_ok=False)
for name in ['emuella-viewer','emuella-viewer-tools']:shutil.copy2(base/'viewer-app/target/release'/name,b/name)
shutil.copy2(base/'benchmark/final-target/release/emuella-benchmark',b/'emuella-benchmark')
shutil.copytree(base/'viewer-canonical/runtime/viewer-web',b/'viewer-web')
for name in ['native-thresholds.json','browser-complete-thresholds.json','recovery-thresholds.json']:shutil.copy2(root/'apps/emuella-viewer/qualification'/name,b/name)
write(p/'before.json',state());write(p/'inputs-before.json',input_state())
if run(['node',root/'tools/viewer-response-headers.mjs',b/'viewer-web'],'wasm-response-headers'):sys.exit(4)
cat=json.loads((base/'final-public/native-01/catalogue.json').read_text())
command=[str(b/'emuella-viewer-tools'),'serve','--listen','127.0.0.1:8127','--verify-payload','true','--web',str(b/'viewer-web')]
for source in cat:command+=['--representation',str(reps/source['target'])]
with (p/'service.stdout').open('w') as out,(p/'service.stderr').open('w') as err:server=subprocess.Popen(command,cwd=root,env=env,stdout=out,stderr=err,start_new_session=True)
server_record=dict(pid=server.pid,command=command,started_unix_ms=time.time_ns()//1000000);write(p/'server.json',server_record)
try:
 for attempt in range(100):
  try:
   with urllib.request.urlopen('http://127.0.0.1:8127/catalogue',timeout=2) as r:observed=json.load(r)
   assert observed==cat
   break
  except Exception:time.sleep(.1)
 else:raise RuntimeError('service failed startup or catalogue changed')
 for mode in ['native','browser','recovery']:
  label=mode+'-01';thresholds=b/({'native':'native','browser':'browser-complete','recovery':'recovery'}[mode]+'-thresholds.json')
  run(['python3',root/'tools/viewer-composed-journey.py','--mode',mode,'--output',p/label,'--url','http://127.0.0.1:8127','--native-bin',b/'emuella-viewer','--web-root',b/'viewer-web','--codec-repo','/nvme/development/emuella/emuella-j2k','--benchmark-repo','/nvme/development/emuella/emuella-benchmark','--server-cache-state','uncontrolled-first-observation' if mode=='native' else 'warm-server','--thresholds',thresholds],label)
  if (p/label/'composed-trace.json').exists():run([b/'emuella-benchmark','journey',p/label/'composed-trace.json',thresholds],label+'-admission')
finally:
 server.terminate();server.wait(timeout=10);server_record['stopped_unix_ms']=time.time_ns()//1000000;server_record['exit_code']=server.returncode;write(p/'server-lifecycle.json',server_record)
 write(p/'after.json',state());write(p/'inputs-after.json',input_state())
print('execution complete; service stopped',flush=True)
