# `@videogen/cli`

Command-line interface for the [VideoGen API](https://docs.videogen.io). Built on [`@videogen/sdk`](https://www.npmjs.com/package/@videogen/sdk).

## Install

```bash
npm install -g @videogen/cli
# or build from a clone (installs @videogen/sdk from npm):
npm install && npm run build
```

## Auth

Set `VIDEOGEN_API_KEY`, or pass `--api-key`. Optional `VIDEOGEN_BASE_URL` / `--base-url` (default `https://api.videogen.io`).

## Usage

```bash
videogen account get-me

videogen workflows script-to-video --body '{"script":"...","visualStyle":{"type":"AI_IMAGE","aiStyle":"watercolor"},"quality":"HIGH","remixActions":[{"type":"ENABLE_CAPTIONS"},{"type":"CONVERT_IMAGES_TO_VIDEOS","motionPrompt":"slow cinematic push-in","muteOutputVideos":true,"quality":"HIGH"}]}' --wait

videogen workflows get-workflow-run --workflow-run-id vg_work_...

videogen tools generate-image --body '{"prompt":"a cat"}' --wait

videogen files upload ./clip.mp4 --type video/mp4
```

- Body POSTs: `--body '<json>'`, `--body @file.json`, or pipe JSON on stdin.
- Path/query params: kebab flags (e.g. `--project-id`, `--limit`, `--self-only`).
- `--wait` on tools/workflows (and project export/remix) calls the SDK `*AndWait` helpers.
- Output is JSON on stdout by default.

## Develop

`npm install` pulls `@videogen/sdk` from npm; no other setup is required.

```bash
npm install
npm run build
npm run typecheck
```
