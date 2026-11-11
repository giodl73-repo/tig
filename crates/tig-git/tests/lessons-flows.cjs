'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict');
const {catalogueFixture,deferred}=require('./ux/catalogue-fixture.cjs');
const calls=f=>f.seen.filter(r=>r.path==='/api/lessons');
const row=(f,name)=>f.nodes(f.get('assets')).find(e=>e.tag==='tr'&&e.children[0]?.children[0]?.textContent===name);
async function pick(f,index){const select=f.nodes().find(e=>e.tag==='select'&&e.getAttribute('aria-label')==='Recorded lesson');select.value=String(index);await select.onchange();await f.settle();}

test('lessons stay hidden and do no evidence IO without explicit enrollment',async()=>{
 const f=catalogueFixture();await f.settle();assert.equal(f.get('lessons').hidden,true);assert.equal(calls(f).length,0);
});
test('real shipped lesson controls show old-job hold, demotion and revert with stable identities',async()=>{
 const f=catalogueFixture({teaching:true});await f.settle();await f.click(f.get('lessons'));
 assert.match(f.text('assets'),/Recorded rehearsal/);assert.match(row(f,'original').children[1].textContent,/Not recorded/);
 await pick(f,3);const hash=row(f,'original').children[0].title;
 assert.match(row(f,'original').children[3].textContent,/Working \/ F/);assert.doesNotMatch(row(f,'original').children[3].textContent,/History \/ D/);
 await f.click('Next lesson');assert.equal(row(f,'original').children[0].title,hash);
 assert.match(row(f,'original').children[1].textContent,/Working \/ F/);assert.match(row(f,'original').children[3].textContent,/History \/ D/);
 assert.doesNotMatch(row(f,'original').children[3].textContent,/Working \/ F/);
 await f.click('Next lesson');assert.equal(row(f,'original').children[0].title,hash);assert.match(row(f,'original').children[3].textContent,/Working \/ F/);
 assert.equal(calls(f).length,1);assert.ok(f.seen.every(r=>!r.init?.method||r.init.method==='GET'));
 assert.equal(f.seen.filter(r=>r.path.startsWith('/api/storage')).length,0);
});
test('visiting the sample catalogue and returning retains the chosen lesson',async()=>{
 const f=catalogueFixture({teaching:true});await f.settle();await f.click(f.get('lessons'));await pick(f,4);
 await f.click('Inspect sample catalogue');assert.equal(f.state().mode,'works');assert.equal(f.state().project,'atlas');
 await f.click(f.get('lessons'));assert.equal(f.get('category-heading').textContent,'job ended');
});
test('missing observations stay unknown rather than becoming zero copies',async()=>{
 const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.lessons[4].objects=b.lessons[4].objects.filter(o=>o.name!=='original');return b;});
 await f.settle();await f.click(f.get('lessons'));await pick(f,4);
 assert.match(row(f,'original').children[3].textContent,/Not recorded/);assert.doesNotMatch(row(f,'original').children[3].textContent,/No managed copies/);
});
test('late lesson successes and errors cannot replace a different project view',async()=>{
 for(const fail of [false,true]){
  const f=catalogueFixture({teaching:true}),pending=deferred();f.override('/api/lessons',()=>pending.promise);await f.settle();await f.click(f.get('lessons'));
  await f.click(f.get('projects').children.find(e=>e.textContent.includes('Boreal')));
  if(fail)pending.reject(Error('Old lesson failure'));else pending.resolve(f.reply('/api/lessons',{}));await f.settle();
  assert.equal(f.state().project,'boreal');assert.equal(f.state().mode,'works');assert.equal(f.get('notice').hidden,true);assert.doesNotMatch(f.text('assets'),/Recorded rehearsal|Old lesson/);
 }
});
test('unavailable or unrecognized evidence clears old lesson controls and supports explicit retry',async()=>{
 const f=catalogueFixture({teaching:true});await f.settle();await f.click(f.get('lessons'));await pick(f,3);
 f.override('/api/lessons',()=>({fixtureError:'Teaching metadata unavailable'}));await f.context.loadLessons();
 assert.match(f.text('assets'),/unavailable/);assert.equal(f.nodes().some(e=>e.textContent==='Next lesson'),false);
 f.override('/api/lessons',()=>({...f.reply('/api/lessons',{}),authorizes_storage:true}));await f.click('Retry lessons');assert.match(f.get('notice').textContent,/Unrecognized teaching/);
 f.override('/api/lessons',()=>f.reply('/api/lessons',{}));await f.click('Retry lessons');assert.equal(f.get('category-heading').textContent,'merge v2 job active');
});
test('enrolled lesson titles and events stay text, never executable markup',async()=>{
 const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.lessons[0].event='<img src=x onerror=alert(1)>';return b;});
 await f.settle();await f.click(f.get('lessons'));assert.match(f.text('assets'),/<img src=x/);assert.equal(f.nodes().some(e=>e.tag==='img'),false);
});

test('partial provider coverage shows known copies plus unknown, including the next before column',async()=>{
 const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-2';
  for(const l of b.lessons)for(const o of l.objects)o.unknown=[];
  const l=b.lessons[3];l.converged=false;l.storage_converged=false;l.holds=['unavailable'];
  for(const o of l.objects){o.observed=o.observed.filter(r=>r!=='backup');o.unknown=['backup'];}
  return b;});
 await f.settle();await f.click(f.get('lessons'));await pick(f,3);
 assert.match(row(f,'original').children[3].textContent,/Working \/ F/);assert.match(row(f,'original').children[3].textContent,/Unknown Â· Backup \/ M/);
 assert.match(f.get('result-count').textContent,/partial coverage/);await pick(f,4);assert.match(row(f,'original').children[1].textContent,/Unknown Â· Backup \/ M/);
});

test('unknown-only provider coverage cannot become zero recorded copies',async()=>{
 const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-2';
  for(const l of b.lessons)for(const o of l.objects)o.unknown=[];
  const l=b.lessons[0];l.converged=false;l.storage_converged=false;l.holds=['unavailable'];
  for(const o of l.objects){o.observed=[];o.unknown=['backup'];}return b;});
 await f.settle();await f.click(f.get('lessons'));assert.match(row(f,'original').children[3].textContent,/Unknown Â· Backup \/ M/);
 assert.doesNotMatch(row(f,'original').children[3].textContent,/No managed copies/);
});

test('conflicting, unversioned or falsely converged unknown coverage refuses',async()=>{
 for(const bad of ['overlap','foreign','version','converged','missing']){
  const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-2';
   for(const l of b.lessons)for(const o of l.objects)o.unknown=[];
   const l=b.lessons[1],o=l.objects[0];l.converged=false;l.storage_converged=false;o.unknown=['backup'];o.observed=o.observed.filter(r=>r!=='backup');
   if(bad==='overlap')o.observed.push('backup');if(bad==='foreign')o.unknown=['elsewhere'];if(bad==='version')b.schema='tig.catalogue-teaching.design-1';if(bad==='converged')l.converged=true;if(bad==='missing')delete o.unknown;
   return b;});await f.settle();await f.click(f.get('lessons'));assert.match(f.get('notice').textContent,/Unrecognized teaching coverage/);
 }
});

test('rejected stale attempts distinguish historical desire from unchanged protected copies',async()=>{
 const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-3';
  for(const l of b.lessons)for(const o of l.objects)o.unknown=[];
  const l=b.lessons[3];l.converged=false;l.basis_current=false;l.storage_converged=false;l.remaining_page=false;l.stream_bytes=0;l.effects=[];
  l.rejection={error:'stale',planning_basis:'a'.repeat(64),observed_basis:'b'.repeat(64)};l.objects.find(o=>o.name==='original').desired=['history','backup'];return b;});
 await f.settle();await f.click(f.get('lessons'));await pick(f,3);
 assert.match(f.text('assets'),/Rejected before storage effects/);assert.match(f.text('assets'),/Current observed basis/);
 assert.match(row(f,'original').children[2].textContent,/History \/ D/);assert.match(row(f,'original').children[3].textContent,/Working \/ F/);
 assert.ok(f.seen.every(r=>!r.init?.method||r.init.method==='GET'));
});

test('rejection evidence cannot hide effects, convergence, unchanged basis or older protocol versions',async()=>{
 for(const bad of ['version','effects','stream','converged','current','same','page','null']){
  const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-3';
   for(const l of b.lessons)for(const o of l.objects)o.unknown=[];
   const l=b.lessons[3];l.converged=false;l.basis_current=false;l.storage_converged=false;l.remaining_page=false;l.stream_bytes=0;l.effects=[];
   l.rejection={error:'stale',planning_basis:'a'.repeat(64),observed_basis:'b'.repeat(64)};
   if(bad==='version')b.schema='tig.catalogue-teaching.design-2';if(bad==='effects')l.effects=[{}];if(bad==='stream')l.stream_bytes=1;if(bad==='converged')l.converged=true;
   if(bad==='current')l.basis_current=true;if(bad==='same')l.rejection.observed_basis=l.rejection.planning_basis;if(bad==='page')l.remaining_page=true;if(bad==='null')l.rejection=null;
   return b;});await f.settle();await f.click(f.get('lessons'));assert.match(f.get('notice').textContent,/Unrecognized rejected attempt/);
 }
});

test('final allocation evidence refuses negative unsafe and nonnumeric values',async()=>{
 for(const allocation of [-1,Number.MAX_SAFE_INTEGER+1,'4096']){
  const f=catalogueFixture({teaching:true});f.override('/api/lessons',()=>{const b=f.reply('/api/lessons',{});b.schema='tig.catalogue-teaching.design-4';
   for(const lesson of b.lessons)for(const o of lesson.objects)o.unknown=[];
   const l=b.lessons[3];l.final_loss={phase:'completed',new_removed_bytes:0,historical_removed_bytes:0,synthetic_owner_consent:true,simulated_clock:true,file_allocation_before_new_removals:allocation};
   for(const o of l.objects)o.before=[...o.observed];return b;});
  await f.settle();await f.click(f.get('lessons'));assert.match(f.get('notice').textContent,/Unrecognized allocation evidence/);
 }
});
