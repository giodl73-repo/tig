# Security and private data

Tig is experimental; no stable supported release or security-support window has
been declared. Use exact source pins and review the relevant provider capability
and protection assumptions before trusting important bytes.

Do not put credentials, private assets, complete resolver profiles, receipts or
unredacted logs in public issues, PRs or screenshots. Use GitHub's private
vulnerability reporting through the repository Security tab when enabled. If it
is unavailable, open only a nonsensitive request for a private reporting channel;
do not include vulnerability details or secrets in that request.

Useful private reports identify the source commit/platform, affected boundary,
synthetic reproduction, expected versus observed behavior and potential impact.
Avoid demonstrations against live stores or destructive workloads.

The catalogue binds loopback and is intended for local use. Do not expose its
port/session token through a public proxy. Enrollment grants explicit metadata
and byte-read access to configured stores. Unknown dependencies or copy custody
must not be turned into deletion authority. See docs/SAFETY-PROMISE.md.
