from pathlib import Path
import hashlib,json,shutil
p=Path(__file__).resolve().parent;root=Path('/nvme/development/emuella/worktrees/polyorama-proof/polyorama');out=root/'docs/emuella-viewer-evidence/nitf-qualified';out.mkdir(exist_ok=False);j=p/'composed-display98'
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as s:
  for b in iter(lambda:s.read(1<<20),b''):h.update(b)
 return h.hexdigest()
def write(f,d):f.parent.mkdir(parents=True,exist_ok=True);f.write_text(json.dumps(d,indent=2)+'\n')
def copy(f,name):dest=out/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(f,dest);return {'path':name,'sha256':sha(dest)}
copy(p/'build-attestation.json','build-attestation.json')
for name in ['qualified-43008-cold-command.json','qualified-43008-warm-command.json','run-composed.py','run-composed-display98.py','run-reference.py','package-evidence.py']:copy(p/name,'commands/'+name)
for f in (root/'apps/emuella-viewer/qualification').glob('*thresholds.json'):copy(f,'thresholds/'+f.name)
preparations=[];raw=[]
for mode,dirname in [('observer','qualified-43008-cold-observer'),('performance','qualified-43008-warm-performance')]:
 directory=p/dirname;report=json.loads((directory/'preparation-result.json').read_text());assert report['qualified'] and not report['failures']
 entry={'mode':mode,'report':copy(directory/'preparation-result.json','preparation/'+mode+'.json'),'wall_ms':report['wall_ms'],'metrics':report['representation']['metrics'],'encoded_bytes':report['representation']['encoded_bytes'],'source_syscalls':report['source_syscalls']};preparations.append(entry)
 for name in ['result.json','stderr.txt']:copy(directory/name,'preparation/'+mode+'-'+name)
 for f in directory.glob('source.strace.*'):raw.append({'scope':'preparation/'+mode,'name':f.name,'bytes':f.stat().st_size,'sha256':sha(f),'retained_exact':False,'reason':'syscall aggregate retained; full trace generated reproducibly but not included in source repository'})
assert json.loads((p/'qualified-43008-cold-observer/preparation-result.json').read_text())['representation']['manifest']['sha256']==json.loads((p/'qualified-43008-warm-performance/preparation-result.json').read_text())['representation']['manifest']['sha256']
assert json.loads((j/'inputs-before.json').read_text())==json.loads((j/'inputs-after.json').read_text())
for name in ['commands.json','server-command.json','inputs-before.json','inputs-after.json']:copy(j/name,'composed/'+name)
copy(j/'native-01/catalogue.json','inputs/catalogue.json')
runs=[]
for mode in ['native','browser','recovery']:
 label=mode+'-01';directory=j/label;trace=json.loads((directory/'composed-trace.json').read_text());admission=json.loads((j/(mode+'-admission.json')).read_text());assert trace['completed'] and not trace['failures'] and admission['qualified'] and not admission['reasons']
 runs.append({'mode':mode,'trace':copy(directory/'composed-trace.json','runs/'+label+'/composed-trace.json'),'admission':copy(j/(mode+'-admission.json'),'runs/'+label+'/admission.json'),'environment':trace['environment'],'cache_state':trace['cache_state'],'observations':trace['observations']})
 for name in ['app.json','run-identity.json','process-memory.json','service-before.json','service-after.json','wire.json','process.log','browser-final.png','browser-recovery.json']:
  if (directory/name).exists():copy(directory/name,'runs/'+label+'/'+name)
 stages=directory/'app.json.stages.json'
 if stages.exists():
  summaries=json.loads(stages.read_text())
  for snap in summaries:snap.pop('events',None);snap['worker'].pop('decoded_evidence',None)
  write(out/'runs'/label/'stages-summary.json',{'source_sha256':sha(stages),'omitted_fields':['events','worker.decoded_evidence'],'stages':summaries})
 for f in directory.iterdir():
  if f.is_file():raw.append({'scope':label,'name':f.name,'bytes':f.stat().st_size,'sha256':sha(f)})
for f in (p/'reference').iterdir():
 if f.is_file() and f.name!='progress.json':copy(f,'reference/'+f.name)
for f in (p/'visuals').iterdir():
 if f.is_file():copy(f,'visuals/'+f.name)
for mode in ['native','browser','recovery']:
 directory=p/'composed'/(mode+'-01')
 for name in ['process.log','run-identity.json','composed-trace.json','browser-failure.json']:
  if (directory/name).exists():copy(directory/name,'failed-display/'+mode+'/'+name)
 for name in [mode+'-admission.json',mode+'.stdout',mode+'.stderr']:
  if (p/'composed'/name).exists():copy(p/'composed'/name,'failed-display/'+name)
write(out/'failed-display/assessment.json',{'qualified':False,'cause':'The supplied DISPLAY=:97 no longer had a live X server (Xvfb session exit137); native XOpenDisplay failed and browser could not create a GPU surface. Recovery also did not start.','disposition':'Retained all three failed admissions/logs; restored a fresh Xvfb display:98 and reran in new output with identical application/native/static binaries, representations and frozen limits. No cause for the Xvfb exit is inferred.'})
write(out/'visuals/assessment.json',{'opened':['full-detail-small-objects.png','full-detail-aggressive-stretch.png','reduced-centre-boundaries.png','coarse-fallback-detail-pending.png'],'observations':['Cached coarse primary imagery remains visible while an actual detail response is held; linked overview and detection gallery remain populated.','Full detail shows authored bright circles and small rectangles over the ramp. Vertical lines are authored x%251 edges; diagonal discontinuities are the modular source ramp.','Aggressive stretch visibly clips ramp ranges and emphasises small objects; the captured counters show no additional compressed request or decode for stretch.','No blank regional strip is apparent in opened detail/reduced captures; numerical complete-selected-tile comparisons independently check region boundaries.'],'limitations':['Appearance only for arithmetic inputs, not satellite object or sensor quality.','One-code faint perturbation fidelity is not established visually at this lossy target; no exact source preservation is claimed.','Display screenshots do not measure GPU execution time or scanout.']})
ref=json.loads((p/'reference/comparisons.json').read_text());assert ref['passed_groups']==18 and ref['mismatched_records']==0 and not ref['native_browser_mismatches']
inputs=json.loads((j/'inputs-before.json').read_text());aux={k:v for k,v in inputs.items() if '/final-public/representations/' in k}
report={'schema':'viewer-independent-nitf-qualification/1','qualified':True,'scope':'Independent project-authored NITF C8 U11 large-source preparation and one fresh native/browser/recovery composed confirmation; original vendor and real-scene visual qualification deferred.','build_source_revision':'16296520e146df842555a8d763fcce704207adc8','composed_harness_revision':'3ad17777371e5069db902049e799008c5e8713e7','build_attestation':'build-attestation.json','preparations':preparations,'preparation_identity_equal':True,'runs':runs,'reference':{'path':'reference/comparisons.json','groups':ref['group_count'],'records':ref['record_count'],'pixels':ref['compared_pixels'],'mismatched_records':0,'native_browser_shared':ref['native_browser_shared_records'],'native_browser_mismatches':0},'reuse':{'representations':9,'unchanged_all_files':len(inputs),'unchanged_all_bytes':sum(v['bytes'] for v in inputs.values()),'previously_prepared_auxiliary_parents':8,'reused_auxiliary_files':len(aux),'reused_auxiliary_bytes':sum(v['bytes'] for v in aux.values()),'original_source_opened_during_viewing':False},'visuals':'visuals/assessment.json','failed_environment_runs':'failed-display/assessment.json','limitations':['One run per mode confirms this integration; no new population latency or five-run statistical claim.','Source-file OS-cache state is proved at preparation launch only. Tool hashing warms input before decoder reads; syscall bytes include rereads/hash and are not physical-device/NFS traffic.','The authored source is more compressible than the HT target: no original satellite-storage benefit is established.','Viewer service source-storage OS/NFS state is uncontrolled; warm server, compressed and GPU stages remain separate.','Native RTX3090 Vulkan and browser NVIDIA/Ampere are observed separately; browser device model is withheld.','Logical GPU texture bytes and CPU residency times do not measure physicalGPU overhead, execution completion or scanout.','Browser memory sums sampled descendants and observed high-water marks; shared pages may count repeatedly and short-lived processes may be missed.','Bounded event/checksum rings and samples are incomplete; full raw traces are hash-indexed and are not all retained in this repository.','Complete reference bypasses JPP/cache but shares codec algorithms; separate owner tests provide algorithm and independently encoded source evidence.']}
write(out/'qualification.json',report)
byhash={}
for f in out.rglob('*'):
 if f.is_file():byhash.setdefault(sha(f),[]).append(str(f.relative_to(out)))
for r in raw:r['retained_paths']=byhash.get(r['sha256'],[]);r['retained_exact']=bool(r['retained_paths'])
write(out/'raw-artefact-index.json',raw)
files={str(f.relative_to(out)):{'sha256':sha(f),'bytes':f.stat().st_size} for f in sorted(out.rglob('*')) if f.is_file()};write(out/'manifest.json',{'schema':'viewer-qualified-evidence-manifest/1','files':files,'bytes':sum(f['bytes'] for f in files.values())});print(len(files),sum(f['bytes'] for f in files.values()))
