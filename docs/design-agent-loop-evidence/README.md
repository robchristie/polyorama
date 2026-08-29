# Design-agent loop evidence

This directory retains selected, reviewable campaign evidence. Generated test
runs remain outside the tracked tree unless a result is deliberately selected
and described here.

## Increment 2: token-driven application bar

`increment-2-token-application-bar.png`

- source candidate: `codex/design-tokens` before commit;
- captured: 29 August 2026;
- viewport: 1279×756 CSS pixels;
- application: release Wasm `analytical-workspace-lab`;
- browser: Chrome 151.0.7922.47 in a disposable Lantern session;
- backend: hardware WebGPU, `nvidia.com/gpu=0`, unsafe WebGPU explicitly enabled;
- result: nonblank scalar canvas, no Lantern layout finding, and the isolated
  application-bar recipe rendered without overlap or clipping;
- fresh follow-up navigation after the favicon correction: zero console
  messages, zero exceptions, zero failed requests and zero HTTP errors; and
- SHA-256:
  `2f96463ede2be59901c6f92439048b10fe7ffd336ded8209a3b9263af42f0d40`.

The screenshot is supporting visual evidence. Token correctness and variant
coverage are established by generated-code, contrast, preference and compiler
tests rather than by this single dark/comfortable capture.
