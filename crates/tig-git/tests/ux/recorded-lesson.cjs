'use strict';
// Consumes actual native producer/API output, never an invented placement DTO.
const {readFileSync}=require('node:fs'),assert=require('node:assert/strict');
const {catalogueFixture}=require('./catalogue-fixture.cjs');
(async()=>{
 const book=JSON.parse(readFileSync(0,'utf8')),f=catalogueFixture({teaching:true});
 await f.settle();
 f.state().projects.push({...f.state().projects[0],id:book.project,label:'Native sample'});
 f.override('/api/lessons',()=>book);await f.click(f.get('lessons'));
 assert.equal(f.state().mode,'lessons');assert.equal(f.get('notice').hidden,true);
 const row=name=>f.nodes(f.get('assets')).find(e=>e.tag==='tr'&&e.children[0]?.children[0]?.textContent===name);
 const selector=()=>f.nodes().find(e=>e.tag==='select'&&e.getAttribute('aria-label')==='Recorded lesson');
 assert.equal(selector().children.length,7);
 let identity;
 for(const [index,role] of [[3,'working'],[4,'history'],[5,'working']]){
  const select=selector();select.value=String(index);await select.onchange();await f.settle();
  const original=row('original'),expected=book.roles.find(r=>r.id===role).label;
  assert.ok(original.children[3].textContent.includes(expected));
  if(identity)assert.equal(original.children[0].title,identity);
  identity=original.children[0].title;
 }
 assert.equal(f.seen.filter(r=>r.path==='/api/lessons').length,1);
 assert.ok(f.seen.every(r=>!r.init?.method||r.init.method==='GET'));
 process.stdout.write('Actual native evidence renders job hold, demotion and revert.\n');
})().catch(error=>{process.stderr.write(String(error)+'\n');process.exitCode=1;});
