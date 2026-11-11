'use strict';
const lessonView={book:null,selection:'',active:false};
function resetLessonChrome(){if(lessonView.active){lessonView.active=false;document.querySelector('main footer').textContent='Counts describe declared records and selected objects, not videos produced or physical disk usage. No automatic eviction.';}}
function openLessons(){
 if(!state.teachingAvailable)return;
 state.mode='lessons';state.generation++;close();drawNavigation();load();
}
function drawLessonsNavigation(){
 lessonView.active=true;
 $('heading').textContent='Learn Tig';$('subheading').textContent='Follow a recorded fixture from Git changes to verified cache copies.';
 $('projects').replaceChildren(navButton('All projects',state.projects.length,false,()=>chooseProject('all'),'◫'));
 state.projects.forEach(p=>$('projects').append(navButton(p.label,p.work_count,false,()=>chooseProject(p.id))));
 $('categories').replaceChildren();$('technical').hidden=true;document.querySelector('.toolbar').hidden=true;
 $('work-controls').hidden=false;$('assets').className='lesson-view';$('more').hidden=true;
 document.querySelector('main footer').textContent='Recorded synthetic rehearsal. Current custody, hardware independence and storage permission are separate.';
}
function lessonRoles(book,values,empty,unknown=[]){
 if(values==null)return node('span','muted','Not recorded');
 const area=node('div','lesson-role-set');
 for(const role of book.roles){if(values.includes(role.id))area.append(node('span','copy-chip',role.label));else if(unknown.includes(role.id))area.append(node('span','copy-chip','Unknown · '+role.label));}
 if(!values.length&&!unknown.length)area.append(node('span','muted',empty));return area;
}
function selectLesson(book,index){
 if(state.mode!=='lessons'||lessonView.book!==book)return;
 const lesson=book.lessons[index];if(!lesson)return;lessonView.selection=lesson.id;
 const controls=$('work-controls');controls.replaceChildren();
 const select=node('select');select.setAttribute('aria-label','Recorded lesson');
 book.lessons.forEach((l,i)=>select.append(new Option((i+1)+'. '+l.title,String(i))));select.value=String(index);
 select.onchange=()=>selectLesson(book,Number(select.value));
 const previous=node('button','button secondary','Previous lesson'),next=node('button','button','Next lesson');
 previous.disabled=index===0;next.disabled=index===book.lessons.length-1;
 previous.onclick=()=>selectLesson(book,index-1);next.onclick=()=>selectLesson(book,index+1);
 const browse=node('button','button secondary','Inspect sample catalogue');browse.onclick=()=>chooseProject(book.project);
 controls.append(previous,select,next,browse);
 const before=book.lessons[index-1],all=new Map();
 for(const l of book.lessons)for(const o of l.objects)all.set(o.name,o.object);
 let knownCopies=0,copyBytes=0;for(const o of lesson.objects)if(Array.isArray(o.observed)){knownCopies+=o.observed.length;copyBytes+=o.object.bytes*o.observed.length;}
 const partial=lesson.objects.some(o=>o.unknown?.length),stats=document.querySelector('.stats');stats.replaceChildren();
 for(const [value,label] of [[(index+1)+' / '+book.lessons.length,'recorded checkpoint'],[all.size,'stable byte identities'],[knownCopies,partial?'known recorded copies':'recorded copies'],[bytes(lesson.stream_bytes),'payload streamed this step']]){
  const s=node('div');s.append(node('span','',value),node('small','',label));stats.append(s);
 }
 $('category-heading').textContent=lesson.title;$('result-count').textContent=bytes(copyBytes)+(partial?' known recorded copy bytes · partial coverage':' recorded copy bytes');
 const area=$('assets');area.replaceChildren();
 area.append(node('p','callout','Recorded rehearsal · synthetic admitted domains. These are saved verified observations, not a check of current hardware custody.'));
 area.append(node('p','lesson-event',lesson.event));
 const source=node('p','muted','Associated Git commit '+lesson.source_commit.slice(0,12)+(lesson.source_available?' · available in this catalogue':' · source metadata unavailable here')+'. Drafts and jobs may contribute separate admitted snapshots.');
 source.title=lesson.source_commit;area.append(source);
 if(lesson.final_loss)area.append(node('p','callout','Explicit final loss · '+lesson.final_loss.phase+'. New removal: '+bytes(lesson.final_loss.new_removed_bytes)+'. Historical deletion: '+bytes(lesson.final_loss.historical_removed_bytes)+'. Separate synthetic owner consent and simulated clock.'+(lesson.final_loss.required_age_millis===undefined?'':' Required continuous inactive age: '+lesson.final_loss.required_age_millis+' ms (simulated).')+' This recorded lesson grants no storage authority.'));
 if(lesson.final_loss?.file_allocation_before_new_removals!==undefined)area.append(node('p','muted','Allocated file data before new removals: '+bytes(lesson.final_loss.file_allocation_before_new_removals)+'. Free-space change: unmeasured.'));
 if(!lesson.final_loss&&lesson.rejection)area.append(node('p','callout','Rejected before storage effects: stale plan. The desired column belongs to the rejected plan. A fresh plan is required for current protection.'));
 else if(!lesson.final_loss&&(!lesson.converged||!lesson.basis_current||lesson.holds.length))area.append(node('p','callout','This recorded step remained pending or held: '+(lesson.holds.join(', ')||'the admitted basis changed')));
 const wrap=node('div','object-table-wrap'),table=node('table','object-manager-table lesson-object-table'),head=node('thead'),header=node('tr');
 for(const text of ['Same bytes','Before this step',lesson.final_loss?'Final selection':'Desired in this plan','Observed after'])header.append(node('th','',text));head.append(header);table.append(head);
 const body=node('tbody');
 for(const [name,object] of all){
  const current=lesson.objects.find(o=>o.name===name),old=before?.objects.find(o=>o.name===name),row=node('tr');
  const identity=node('td');identity.append(node('strong','',name),node('small','',bytes(object.bytes)),node('div','mono',object.sha256.slice(0,12)));identity.title=object.sha256;
  const left=node('td'),desired=node('td'),after=node('td');
  left.append(lessonRoles(book,current?.before??old?.observed,'No managed copies recorded',current?.before?[]:old?.unknown||[]));
  if(current?.desired==null)desired.append(node('span','muted',lesson.final_loss?(current.final_selection==='eligible'?'Eligible for separately authorized final removal':current.final_selection==='unselected'?'Not selected in this block':'Protected · excluded from final loss'):'Not included in this recorded plan'));
  else desired.append(lessonRoles(book,current.desired,'No placements required in this plan'));
  after.append(lessonRoles(book,current?.observed,'No managed copies recorded',current?.unknown||[]));
  row.append(identity,left,desired,after);body.append(row);
 }
 table.append(body);wrap.append(table);area.append(wrap);
 const evidence=node('details','lesson-evidence');evidence.append(node('summary','','Recorded evidence'));
 evidence.append(node('p','muted','Report '+lesson.report.sha256),node('p','muted','Plan '+lesson.plan));
 if(lesson.rejection)evidence.append(node('p','muted','Planned protection basis '+lesson.rejection.planning_basis),node('p','muted','Current observed basis '+lesson.rejection.observed_basis));
 const effects=node('ul');for(const effect of lesson.effects)effects.append(node('li','',effect.kind.replaceAll('_',' ')+' · '+(book.roles.find(r=>r.id===effect.role)?.label||effect.role)+' · '+effect.object.sha256.slice(0,12)+' · '+bytes(effect.logical_bytes)));
 if(!lesson.effects.length)evidence.append(node('p','muted','No new provider effects recorded.'));else evidence.append(effects);area.append(evidence);
 if(book.pending.length)area.append(node('p','muted','Remaining course work: '+book.pending.join(' · ')));
}
async function loadLessons(){
 const gen=++state.generation;notice('');lessonView.book=null;$('more').hidden=true;
 $('work-controls').replaceChildren();document.querySelector('.stats').replaceChildren();$('result-count').textContent='';
 $('assets').replaceChildren(node('div','empty','Reading hash-pinned lesson reports…'));
 try{
  const book=await api('/api/lessons');if(gen!==state.generation||state.mode!=='lessons')return;
  if(!['tig.catalogue-teaching.design-1','tig.catalogue-teaching.design-2','tig.catalogue-teaching.design-3','tig.catalogue-teaching.design-4'].includes(book.schema)||book.mode!=='recorded-rehearsal'||book.authorizes_storage!==false||book.synthetic_domains!==true||book.independent_hardware_backup_proven!==false
   ||!state.projects.some(p=>p.id===book.project)||!Array.isArray(book.lessons)||!book.lessons.length||book.lessons.length>16||!Array.isArray(book.roles)||book.roles.length<2||book.roles.length>4
   ||book.lessons.some(l=>!Array.isArray(l.objects)||l.objects.length>32||!Array.isArray(l.holds)||!Array.isArray(l.effects)))throw Error('Unrecognized teaching response');
  const roles=new Set(book.roles.map(r=>r.id));
  for(const l of book.lessons)for(const o of l.objects){
   if(book.schema==='tig.catalogue-teaching.design-1'){if(o.unknown!==undefined)throw Error('Unrecognized teaching coverage');}
   else if(!Array.isArray(o.unknown)||o.unknown.some((r,i)=>!roles.has(r)||(i&&o.unknown[i-1]>=r)||o.observed?.includes(r))||(o.unknown.length&&(l.converged||l.storage_converged===true)))throw Error('Unrecognized teaching coverage');
  }
  for(const l of book.lessons)if(l.rejection!==undefined){const r=l.rejection;
   if(!['tig.catalogue-teaching.design-3','tig.catalogue-teaching.design-4'].includes(book.schema)||!r||r.error!=='stale'||typeof r.planning_basis!=='string'||typeof r.observed_basis!=='string'||!r.planning_basis.match(/^[0-9a-f]{64}$/)||!r.observed_basis.match(/^[0-9a-f]{64}$/)||r.planning_basis===r.observed_basis||l.basis_current!==false||l.converged!==false||l.storage_converged!==false||l.stream_bytes!==0||l.effects.length||l.remaining_page)throw Error('Unrecognized rejected attempt');
  }
  for(const l of book.lessons)if(l.final_loss!==undefined){const r=l.final_loss;
   if(r?.file_allocation_before_new_removals!==undefined&&(!Number.isSafeInteger(r.file_allocation_before_new_removals)||r.file_allocation_before_new_removals<0))throw Error('Unrecognized allocation evidence');
   if(book.schema!=='tig.catalogue-teaching.design-4'||!r||!['completed','cancelled'].includes(r.phase)||r.synthetic_owner_consent!==true||r.simulated_clock!==true||!Number.isSafeInteger(r.new_removed_bytes)||r.new_removed_bytes<0||!Number.isSafeInteger(r.historical_removed_bytes)||r.historical_removed_bytes<r.new_removed_bytes||l.objects.some(o=>!Array.isArray(o.before)||o.before.some((id,i)=>!roles.has(id)||(i&&o.before[i-1]>=id))))throw Error('Unrecognized final loss');
  }
  lessonView.book=book;const selected=book.lessons.findIndex(l=>l.id===lessonView.selection);selectLesson(book,Math.max(0,selected));
 }catch(e){if(gen===state.generation&&state.mode==='lessons'){
  lessonView.book=null;notice(e.message);$('assets').replaceChildren(node('div','empty','Recorded lessons are unavailable. No storage changes were requested.'));
  const retry=node('button','button secondary','Retry lessons');retry.onclick=()=>{if(state.mode==='lessons')loadLessons();};$('work-controls').replaceChildren(retry);
 }}
}
