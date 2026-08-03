# `@videogen/cli`

Command-line interface for the [VideoGen API](https://docs.videogen.io). Built on [`@videogen/sdk`](https://www.npmjs.com/package/@videogen/sdk).

Every public API resource is a subcommand: `videogen <resource> <kebab-command>`. Commands print JSON to stdout (pipe into `jq`).

## Install

```bash
npm install -g @videogen/cli
```

Or run once without installing:

```bash
npx @videogen/cli --help
```

Requires Node.js 20+.

## Auth

```bash
export VIDEOGEN_API_KEY=sk_videogen_live_...
```

Override per command with `--api-key`. Optional `VIDEOGEN_BASE_URL` / `--base-url` (default `https://api.videogen.io`).

## Quick start

```bash
videogen account get-me
```

Example response:

```json
{
  "apiKeyId": "vg_key_...",
  "apiKeyNickname": "CI",
  "email": "you@example.com",
  "displayName": "Ada",
  "teamId": "vg_team_..."
}
```

List every resource and command:

```bash
videogen --help
videogen tools --help
```

## Workflows

Start a script-to-video run and wait until it finishes (`--wait` polls for you):

```bash
videogen workflows script-to-video --wait --body '{
  "script": "Stay hydrated for better focus and energy.",
  "visualStyle": {
    "type": "AI_IMAGE",
    "aiStyle": "loose watercolor illustration with visible brushstrokes"
  },
  "quality": "HIGH",
  "remixActions": [
    { "type": "ENABLE_CAPTIONS" },
    {
      "type": "CONVERT_IMAGES_TO_VIDEOS",
      "motionPrompt": "slow cinematic push-in",
      "muteOutputVideos": true,
      "quality": "HIGH"
    }
  ]
}'
```

Example response (after `--wait`):

```json
{
  "workflowRunId": "vg_work_...",
  "status": "succeeded",
  "workflowType": "SCRIPT_TO_VIDEO",
  "projectId": "vg_proj_...",
  "projectUrl": "https://app.videogen.io/project/vg_proj_...",
  "progressPercentage": 100,
  "attemptIndex": 0,
  "error": null
}
```

Without `--wait`, the command returns immediately with `workflowRunId` / `projectId`. Poll yourself:

```bash
videogen workflows get-workflow-run --workflow-run-id vg_work_...
```

Other workflow starters: `voiceover-to-video`, `slideshow-to-video`, `prompt-to-video-clip`.

## Tools

```bash
videogen tools generate-image --wait --body '{
  "prompt": "A sunset over a calm ocean, cinematic lighting",
  "quality": "HIGH"
}'
```

Example response (after `--wait`):

```json
{
  "toolExecutionId": "vg_tool_...",
  "status": "succeeded",
  "toolType": "GENERATE_IMAGE",
  "progressPercentage": 100,
  "attemptIndex": 0,
  "results": [
    {
      "fileId": "vg_file_...",
      "type": "IMAGE",
      "downloadUrl": "https://...",
      "downloadUrlExpiresAt": 1769900000,
      "thumbnailUrl": "https://...",
      "thumbnailUrlExpiresAt": 1769900000,
      "file": { "fileId": "vg_file_...", "scope": "TEAM", "displayName": "..." }
    }
  ],
  "error": null
}
```

Start without waiting, then poll:

```bash
videogen tools generate-image --body '{"prompt":"a cat","quality":"HIGH"}'
videogen tools get-tool-execution-info --tool-execution-id vg_tool_...
```

## Files

Upload a local file (convenience wrapper around create → PUT → poll). `--type` is a VideoGen file type, not a MIME type:

```bash
videogen files upload ./clip.mp4 --type VIDEO --display-name clip.mp4
```

Example response:

```json
{
  "fileId": "vg_file_...",
  "type": "VIDEO",
  "scope": "TEAM",
  "displayName": "clip.mp4",
  "downloadUrl": "https://...",
  "downloadUrlExpiresAt": 1769900000,
  "thumbnailUrl": "https://...",
  "thumbnailUrlExpiresAt": 1769900000
}
```

Fetch or hydrate later:

```bash
videogen files get-file --file-id vg_file_...
videogen files hydrate-file --file-id vg_file_...
```

## Projects

Export a finished workflow project:

```bash
videogen projects export-project --project-id vg_proj_... --wait --body '{"quality":"HIGH"}'
```

Apply remix actions:

```bash
videogen projects remix-project --project-id vg_proj_... --wait --body '{
  "remixActions": [
    { "type": "ENABLE_CAPTIONS" },
    { "type": "ADD_TRANSITIONS" }
  ]
}'
```

## Assistant

```bash
videogen assistant start-assistant-chat --wait --body '{
  "message": "Draft a 20-second script about morning hydration."
}'
```

Continue on the returned `assistantId`:

```bash
videogen assistant send-assistant-message --assistant-id vg_asst_... --wait --body '{
  "message": "Make it punchier."
}'
```

## Entities

```bash
videogen entities create-entity --body '{
  "entityType": "ACTOR",
  "name": "Alex",
  "description": "Friendly narrator in casual clothes"
}'
```

## Text

```bash
videogen text generate-text --body '{
  "prompt": "Write a one-sentence hook for a hydration tip video."
}'
```

Example response:

```json
{
  "text": "Your brain runs on water; keep the glass within reach."
}
```

## Webhooks

```bash
videogen webhooks create-webhook-endpoint --body '{
  "url": "https://example.com/videogen-webhooks",
  "events": ["workflow_run.succeeded", "tool_execution.succeeded"]
}'
```

List or delete:

```bash
videogen webhooks list-webhook-endpoints
videogen webhooks delete-webhook-endpoint --webhook-endpoint-id vg_wh_...
```

## Common flags

| Flag | Description |
| --- | --- |
| `--api-key <key>` | API key (or `VIDEOGEN_API_KEY`) |
| `--base-url <url>` | Override API base URL |
| `--body '<json>'` | JSON request body (`--body @file.json`, or pipe JSON on stdin) |
| `--wait` | Poll until complete (tools, workflows, export, remix, assistant) |
| `--json` / `--no-json` | Force JSON stdout (on by default). **Not** a body flag. |
| Path/query params | Kebab flags, e.g. `--project-id`, `--workflow-run-id`, `--file-id`, `--limit`, `--self-only` |

Important: pass request JSON with `--body`. `--json` only controls stdout formatting — it is not a request-body flag.

## Resources

`account` · `workflows` · `tools` · `files` · `projects` · `assistant` · `entities` · `text` · `resources` · `webhooks`

## Develop

```bash
cd cli
npm install
npm run build
npm run typecheck
```

`npm install` pulls `@videogen/sdk` from npm (or the local `file:../sdk-typescript` link in this monorepo).

## Docs

- [API documentation](https://docs.videogen.io)
- [npm: @videogen/cli](https://www.npmjs.com/package/@videogen/cli)
- [GitHub](https://github.com/video-gen/videogen-cli)
