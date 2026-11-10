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
