// Run on the hosted requester, outside the storage executor's OS authority.
import { pathToFileURL } from 'node:url';

async function boundedJSON(response, cap) {
  const chunks = [];
  let length = 0;
  for await (const chunk of response.body) {
    length += chunk.byteLength;
    if (length > cap) throw new Error('response budget');
    chunks.push(chunk);
  }
  return JSON.parse(new TextDecoder('utf-8', { fatal: true }).decode(Buffer.concat(chunks)));
}

export async function request(env, fetcher = globalThis.fetch) {
  try {
    const endpoint = new URL(env.TIG_ENDPOINT);
    const issuer = new URL(env.ACTIONS_ID_TOKEN_REQUEST_URL);
    if (endpoint.protocol !== 'https:' || endpoint.username || endpoint.password || endpoint.search || endpoint.hash || endpoint.pathname !== '/v1/reconcile') throw new Error('endpoint');
    if (issuer.protocol !== 'https:' || !issuer.hostname.endsWith('.actions.githubusercontent.com') || issuer.username || issuer.password || issuer.hash) throw new Error('issuer');
    if (!env.TIG_AUDIENCE || env.TIG_AUDIENCE.length > 512 || !env.ACTIONS_ID_TOKEN_REQUEST_TOKEN) throw new Error('enrollment');
    if (!/^(?:[0-9a-f]{40}|[0-9a-f]{64})$/.test(env.GITHUB_SHA) || /^0+$/.test(env.GITHUB_SHA)) throw new Error('commit');
    issuer.searchParams.set('audience', env.TIG_AUDIENCE);
    const oidc = await fetcher(issuer, { headers: { Authorization: `Bearer ${env.ACTIONS_ID_TOKEN_REQUEST_TOKEN}` }, redirect: 'error', signal: AbortSignal.timeout(15000) });
    if (!oidc.ok) throw new Error('issuer');
    const { value: token } = await boundedJSON(oidc, 24576);
    if (typeof token !== 'string' || !token || token.length > 16384) throw new Error('token');
    const response = await fetcher(endpoint, {
      method: 'POST', redirect: 'error', signal: AbortSignal.timeout(660000),
      headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({ schema: 'tig.github-request.design-1', commit: env.GITHUB_SHA }),
    });
    const body = await boundedJSON(response, 1024 * 1024);
    if (body.schema !== 'tig.github-response.design-1') throw new Error('protocol');
    if (response.status === 409 && body.authenticated === true && body.code === 'stale') {
      return { summary: { state: 'stale', authenticated: true, storage_converged: null }, exitCode: 0 };
    }
    if (body.authenticated !== true) throw new Error('authentication');
    if (!response.ok || body.ok !== true) return { summary: { state: 'pending', authenticated: true, storage_converged: false }, exitCode: 2 };
    const result = body.result;
    if (result?.schema !== 'tig.github-placement-report.design-1' || ![true, false, null].includes(result.storage_converged)) throw new Error('protocol');
    const complete = result.storage_converged === true;
    return { summary: { state: complete ? 'converged' : 'pending', authenticated: true, storage_converged: result.storage_converged }, exitCode: complete ? 0 : 2 };
  } catch {
    // Neither network exceptions nor remote bodies become credential-bearing logs.
    throw new Error('Tig request failed');
  }
}

if (process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url) {
  try {
    const result = await request(process.env);
    console.log(JSON.stringify(result.summary));
    process.exitCode = result.exitCode;
  } catch {
    console.error('Tig request failed');
    process.exitCode = 1;
  }
}
