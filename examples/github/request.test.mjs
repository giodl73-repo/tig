import test from 'node:test';
import assert from 'node:assert/strict';
import { request } from './request.mjs';
const env = { TIG_ENDPOINT: 'https://broker.example.invalid/v1/reconcile', TIG_AUDIENCE: 'tig:fixture', ACTIONS_ID_TOKEN_REQUEST_URL: 'https://run-actions.actions.githubusercontent.com/oidc?api-version=2.0', ACTIONS_ID_TOKEN_REQUEST_TOKEN: 'synthetic-runtime-token', GITHUB_SHA: 'a'.repeat(40) };
const token = 'synthetic-signed-oidc';
function mock(result, status = 200) {
  const calls = [];
  return { calls, fetcher: async (url, options) => { calls.push([new URL(url), options]); return calls.length === 1 ? new Response(JSON.stringify({ value: token })) : new Response(JSON.stringify(result), { status }); } };
}
test('request sends only exact commit and separates issuer and storage credentials', async () => {
  const m = mock({ schema: 'tig.github-response.design-1', authenticated: true, ok: true, result: { schema: 'tig.github-placement-report.design-1', storage_converged: true } });
  const result = await request(env, m.fetcher);
  assert.equal(result.exitCode, 0); assert.equal(result.summary.storage_converged, true);
  assert.equal(m.calls[0][0].searchParams.get('audience'), env.TIG_AUDIENCE);
  assert.equal(m.calls[0][1].headers.Authorization, `Bearer ${env.ACTIONS_ID_TOKEN_REQUEST_TOKEN}`);
  assert.equal(m.calls[1][1].headers.Authorization, `Bearer ${token}`);
  assert.deepEqual(JSON.parse(m.calls[1][1].body), { schema: 'tig.github-request.design-1', commit: env.GITHUB_SHA });
  assert.ok(!JSON.stringify(result).includes(token));
  assert.equal(m.calls[0][1].redirect, 'error'); assert.equal(m.calls[1][1].redirect, 'error');
});
test('pending and resumed/unknown convergence never become complete', async () => {
  for (const convergence of [false, null]) {
    const m = mock({ schema: 'tig.github-response.design-1', authenticated: true, ok: true, result: { schema: 'tig.github-placement-report.design-1', storage_converged: convergence } });
    const result = await request(env, m.fetcher); assert.equal(result.exitCode, 2); assert.equal(result.summary.state, 'pending'); assert.equal(result.summary.storage_converged, convergence);
  }
  const m = mock({ schema: 'tig.github-response.design-1', authenticated: true, ok: false, code: 'unavailable' }, 503);
  assert.equal((await request(env, m.fetcher)).exitCode, 2);
});
test('stale authenticated event is explicitly skipped without asserting storage state', async () => {
  const m = mock({ schema: 'tig.github-response.design-1', authenticated: true, ok: false, code: 'stale' }, 409);
  const result = await request(env, m.fetcher); assert.equal(result.summary.state, 'stale'); assert.equal(result.summary.storage_converged, null); assert.equal(result.exitCode, 0);
});
test('invalid endpoints, unauthenticated responses and exceptions expose no tokens', async () => {
  for (const altered of [{ ...env, TIG_ENDPOINT: 'http://broker.example.invalid/v1/reconcile' }, { ...env, ACTIONS_ID_TOKEN_REQUEST_URL: 'https://untrusted.invalid/oidc' }, { ...env, GITHUB_SHA: 'not-a-commit' }]) {
    await assert.rejects(request(altered, () => { throw new Error(token); }), { message: 'Tig request failed' });
  }
  const m = mock({ schema: 'tig.github-response.design-1', authenticated: false, ok: false }, 401);
  await assert.rejects(request(env, m.fetcher), { message: 'Tig request failed' });
  await assert.rejects(request(env, async () => { throw new Error(env.ACTIONS_ID_TOKEN_REQUEST_TOKEN); }), { message: 'Tig request failed' });
});
