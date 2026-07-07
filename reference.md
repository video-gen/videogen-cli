# VideoGen API CLI Reference

Full command reference for `videogen`.

## Commands

- [`videogen account`](#videogen-account)
- [`videogen entities`](#videogen-entities)
- [`videogen files`](#videogen-files)
- [`videogen projects`](#videogen-projects)
- [`videogen resources`](#videogen-resources)
- [`videogen text`](#videogen-text)
- [`videogen tools`](#videogen-tools)
- [`videogen webhooks`](#videogen-webhooks)
- [`videogen workflows`](#videogen-workflows)

---

### `videogen account`

#### `videogen account get-me`

Return the account and team behind the API key making the request. Takes no parameters. Call it as a connection test to confirm a key is valid and to discover the `teamId` and account `email` a key belongs to.

`GET /v1/me`

---

### `videogen entities`

#### `videogen entities add-entity-reference`

Attach an image file as a reference for the entity. Upload the image first via `POST /v1/files/upload`. Returns the updated entity.

`POST /v1/entities/{entityId}/references`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-id` | `string` | Yes | The entity id (e.g. `vg_enti_...`). |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen entities archive-entity`

Archive an entity. Archived entities no longer appear in `GET /v1/entities` and can't be attached to new workflows.

`POST /v1/entities/{entityId}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-id` | `string` | Yes | The entity id (e.g. `vg_enti_...`). |

#### `videogen entities create-entity`

Create a new actor or visual style. Attach reference images with `POST /v1/entities/{entityId}/references`.

`POST /v1/entities`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen entities get-entity`

Retrieve a single entity by its id, including its reference images.

`GET /v1/entities/{entityId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-id` | `string` | Yes | The entity id (e.g. `vg_enti_...`). |

#### `videogen entities list-entities`

List the actors and visual styles available to your team, most recently updated first. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/entities`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-type` | `ACTOR | PRODUCT | VISUAL_STYLE` | No | When provided, returns only entities of this type. Omit to return all entities. |
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |

#### `videogen entities remove-entity-reference`

Detach a reference image from the entity. Returns the updated entity.

`POST /v1/entities/{entityId}/references/remove`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-id` | `string` | Yes | The entity id (e.g. `vg_enti_...`). |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen entities update-entity`

Update an entity's name or description. Provide at least one field.

`POST /v1/entities/{entityId}/update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--entity-id` | `string` | Yes | The entity id (e.g. `vg_enti_...`). |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `videogen files`

#### `videogen files archive-file`

Archive a file by setting its archived timestamp. Archived files are excluded from list results. Returns the updated file object.

`POST /v1/files/{fileId}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The file id (e.g. `vg_file_...`). |

#### `videogen files create-file-upload`

Create a new file and receive a pre-signed upload URL. PUT the file bytes to the returned URL, then poll `GET /v1/files/{fileId}` until the file is ready.

`POST /v1/files/upload`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen files disable-public-preview`

Disable public preview for a file. Removes the permanent public URL copy and revokes unauthenticated embed streaming access. Authenticated signed URLs remain functional. Returns the updated file.

`POST /v1/files/{fileId}/disable-public-preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The file id (e.g. `vg_file_...`). |

#### `videogen files enable-public-preview`

Enable public preview for a file. Works for any file type. Copies the file to a permanent public URL (`staticPublicPreviewSource`) and, for video and audio, registers a public embed playback id (`publicPlaybackId`) for use with `@videogen/player`. If streaming playback is still processing, the endpoint polls briefly and background processing finishes creating the embed playback id. Returns the updated file.

`POST /v1/files/{fileId}/enable-public-preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The file id (e.g. `vg_file_...`). |

#### `videogen files get-file`

Retrieve metadata for a single file by its id.

`GET /v1/files/{fileId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The file id (e.g. `vg_file_...`). |

#### `videogen files get-files`

List files in your account, including generated assets and uploads. Files are returned most recently updated first. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |

#### `videogen files hydrate-file`

Generate fresh signed URLs for all available renditions of a file. Call this when source URLs are missing or expired. Returns the full file object with populated `thumbnailSource`, `previewSource`, and `downloadSource`.

`POST /v1/files/{fileId}/hydrate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes | The file id (e.g. `vg_file_...`). |

#### `videogen files search-files`

Semantic vector search over your files. Embeds the query text and returns the closest matching files ranked by cosine similarity. Only files with indexed descriptions are searchable.

`POST /v1/files/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `videogen projects`

#### `videogen projects export-project`

Starts an export of a project to MP4. Returns immediately with an export id; the file becomes available when the export task completes.

`POST /v1/projects/{projectId}/export`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project id (e.g. `vg_proj_...`). |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `videogen projects get-project`

Returns a simplified view of a project including its title, aspect ratio, status, and URL.

`GET /v1/projects/{projectId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project id (e.g. `vg_proj_...`). |

#### `videogen projects get-project-export`

Returns the current status of a project export started via `POST /v1/projects/{projectId}/export`.

`GET /v1/projects/{projectId}/exports/{exportId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project id (e.g. `vg_proj_...`). |
| `--export-id` | `string` | Yes | The export id returned by `POST /v1/projects/{projectId}/export`. |

#### `videogen projects list-project-remix-actions`

Returns every remix action applied to a project (via `POST /v1/projects/{projectId}/remix` or as a post-workflow step), most recent first, with each action's status and progress.

`GET /v1/projects/{projectId}/remix-actions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project id (e.g. `vg_proj_...`). |

#### `videogen projects list-projects`

Returns projects, most recently updated first. By default only API-created projects are included; pass `includeUiProjects=true` to also include dashboard-created projects. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all matching projects for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/projects`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |
| `--self-only` | `boolean` | No | When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team. |
| `--include-ui-projects` | `boolean` | No | When true, includes dashboard-created projects in addition to API-created projects. When false (default), returns only API-created projects. |

#### `videogen projects remix-project`

Applies an ordered list of edits (background music, logo overlay, caption visibility/style) to a project. Each action runs asynchronously as its own remix action; the response returns one remix action id per action in order. Set `saveAsNewProject` to apply the edits to a copy and leave the original untouched. Poll `GET /v1/projects/{projectId}/remix-actions` for status.

`POST /v1/projects/{projectId}/remix`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The project id (e.g. `vg_proj_...`). |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `videogen resources`

#### `videogen resources list-avatar-presenters`

List available avatar presenters. Pass an `avatarPresenterId` from the response to the avatar video endpoint or to a script/slideshow workflow. Pass a reference `voiceId` to return presenters sorted by best match for that voice. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/resources/avatar-presenters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |
| `--voice-id` | `string` | No | Optional reference voice id from `GET /v1/resources/tts-voices` (e.g. `vg_voic_...`). When provided, avatar presenters are returned sorted by best match for that voice (best first). Omit to return presenters in the default catalogue order. |

#### `videogen resources list-languages`

List the languages a project can be translated into. Pass a `languageCode` from the response to the `TRANSLATE_PROJECT` remix action. Returns the full catalogue in a single response (not paginated).

`GET /v1/resources/languages`

#### `videogen resources list-tts-voices`

List available text-to-speech voices. Pass a `voiceId` from the response to the text-to-speech endpoint. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/resources/tts-voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |
| `--include-deprecated-voices` | `boolean` | No | When true, includes voices that are deprecated but still callable. Defaults to false. |

---

### `videogen text`

#### `videogen text generate-text`

Generate text from a prompt using a general-purpose language model. Choose a quality tier with `quality` (`LOW`, `STANDARD`, `HIGH`, or `MAX`). Synchronous: the response includes the generated text. Useful for drafting scripts, titles, descriptions, and other short copy before generating a video.

`POST /v1/text/generate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `videogen tools`

#### `videogen tools cancel-tool-execution`

Request cancellation of a running tool execution. The execution transitions to `cancelled` if it has not already completed.

`POST /v1/tools/executions/{toolExecutionId}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-execution-id` | `string` | Yes | The tool execution id returned when the tool was started. |

#### `videogen tools generate-avatar`

Generate a talking-head avatar video by pairing a presenter with an audio file, typically from a prior text-to-speech result.

`POST /v1/tools/generate-avatar`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools generate-image`

Generate an image from a text prompt, optionally guided by one or more reference images. When reference images are provided, the prompt describes the desired transformation. VideoGen automatically routes each request to the most effective state-of-the-art image model for your prompt, reference images, and quality tier, so you don't pick a model.

`POST /v1/tools/generate-image`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools generate-music`

Generate an instrumental music track from a text description. The returned track is approximately 30 seconds long. VideoGen automatically routes each request to the most effective state-of-the-art music model for your prompt, so you don't pick a model.

`POST /v1/tools/generate-music`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools generate-sound-effect`

Generate a sound effect from a text description. Optionally control the duration and prompt influence. VideoGen automatically routes each request to the most effective state-of-the-art sound effect model for your prompt and settings, so you don't pick a model.

`POST /v1/tools/generate-sound-effect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools generate-video-clip`

Generate a single short video clip (up to 15 seconds) from a text prompt, optionally guided by reference images, videos, and audio. At least one of `prompt`, `imageFileIds`, `videoFileIds`, or `audioFileIds` must be provided. VideoGen automatically routes each request to the most effective state-of-the-art video model for your inputs and settings, so you don't pick a model. This endpoint returns one standalone clip. For longer, higher-quality, professionally edited videos with narration, captions, music, and multiple scenes, use a video workflow such as [Script to video](/workflows) (`POST /v1/workflows/script-to-video`) instead.

`POST /v1/tools/generate-video-clip`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools get-tool-execution-info`

Retrieve the current status and result of a tool execution. Poll this endpoint until `status` is `succeeded`, `failed`, or `cancelled`.

`GET /v1/tools/executions/{toolExecutionId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-execution-id` | `string` | Yes | The tool execution id returned when the tool was started. |

#### `videogen tools image3d-effect`

Turn a still image into a short video clip with a 3D parallax motion effect, simulating camera movement through the scene.

`POST /v1/tools/image-3d-effect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools list-tool-executions`

List tool executions started via the API, most recently created first. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all executions for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/tools/executions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |
| `--self-only` | `boolean` | No | When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team. |

#### `videogen tools remove-image-background`

Remove the background from an image, returning a transparent-background PNG of the foreground subject.

`POST /v1/tools/remove-image-background`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools remove-video-background`

Remove the background from a video, producing a transparent-background video of the foreground subject.

`POST /v1/tools/remove-video-background`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools text-to-speech`

Convert text into a spoken audio file. Only voices with `supportsDirectToolExecution` set to true can be used. Optionally choose a voice, language, speed, and pronunciation overrides.

`POST /v1/tools/text-to-speech`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools upscale-image`

Increase the resolution of an image while preserving detail and sharpness.

`POST /v1/tools/upscale-image`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools upscale-video`

Increase the resolution of a video while preserving detail and sharpness.

`POST /v1/tools/upscale-video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen tools vectorize-image`

Convert any raster image into a scalable vector graphic (SVG). The output traces the shapes and colors of the input image.

`POST /v1/tools/vectorize-image`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `videogen webhooks`

#### `videogen webhooks create-webhook-endpoint`

Register a new webhook endpoint to receive `tool_execution.*`, `workflow_run.*`, and `file.*` events. The signing secret is only returned in this response. Store it securely.

`POST /v1/webhooks/endpoints`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen webhooks delete-webhook-endpoint`

Remove a webhook endpoint. It will stop receiving events immediately.

`DELETE /v1/webhooks/endpoints/{endpointId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--endpoint-id` | `string` | Yes | The webhook endpoint id returned by `POST /v1/webhooks/endpoints`. |

#### `videogen webhooks list-webhook-endpoints`

List configured webhook endpoints for your account. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/webhooks/endpoints`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |

---

### `videogen workflows`

#### `videogen workflows add-narration-transitions-and-captions-to-slideshow` `[DEPRECATED]`

Legacy alias for `POST /v1/workflows/slideshow-to-video`. Use that endpoint instead.

`POST /v1/workflows/add-narration-transitions-and-captions-to-slideshow`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows add-visuals-and-captions-to-voiceover` `[DEPRECATED]`

Legacy alias for `POST /v1/workflows/voiceover-to-video`. Use that endpoint instead.

`POST /v1/workflows/add-visuals-and-captions-to-voiceover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows add-visuals-narrations-and-captions-to-script` `[DEPRECATED]`

Legacy alias for `POST /v1/workflows/script-to-video`. Use that endpoint instead.

`POST /v1/workflows/add-visuals-narrations-and-captions-to-script`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows cancel-workflow-run`

Cancel a workflow run

`POST /v1/workflows/runs/{workflowRunId}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-run-id` | `string` | Yes | The workflow run id returned when the workflow was started. |

#### `videogen workflows generate-scenes-from-storyboard` `[DEPRECATED]`

Legacy alias for `POST /v1/workflows/storyboard-to-video`. Use that endpoint instead.

`POST /v1/workflows/generate-scenes-from-storyboard`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows get-workflow-run`

Get workflow run status

`GET /v1/workflows/runs/{workflowRunId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workflow-run-id` | `string` | Yes | The workflow run id returned when the workflow was started. |

#### `videogen workflows list-workflow-runs`

List workflow runs started via the API, most recently created first. Use `selfOnly=true` to restrict results to the calling API key's user; otherwise all runs for the team are returned. Cursor-paginated; see the [Pagination](/pagination) guide.

`GET /v1/workflows/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of items to return in the page. Defaults to 50; capped at 200. See [Pagination](/pagination). |
| `--cursor` | `string` | No | Opaque pagination cursor returned as `nextCursor` by the previous page. Omit on the first request. Cursors are tied to the endpoint that produced them and must be passed unmodified. See [Pagination](/pagination). |
| `--self-only` | `boolean` | No | When true, returns only items created by the API key's owner. When false (default), returns all items accessible to the team. |

#### `videogen workflows script-to-video`

Creates a project and generates a narrated video from a prompt or script. Returns immediately with a workflow run id; poll or subscribe to webhooks for completion.

`POST /v1/workflows/script-to-video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows slideshow-to-video`

Creates a project from an uploaded PDF or PowerPoint file and generates an AI-narrated video walking through each slide. Upload the file via `POST /v1/files/upload` first.

`POST /v1/workflows/slideshow-to-video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows storyboard-to-video`

Creates a project from an ordered list of scenes and generates one section per scene. Each scene is generated from its prompt as either a still image or a video clip; the scenes are then assembled into a single video. Returns immediately with a workflow run id; poll or subscribe to webhooks for completion.

`POST /v1/workflows/storyboard-to-video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `videogen workflows voiceover-to-video`

Creates a project from an uploaded voiceover file and generates a video with matching b-roll. Upload the voiceover via the files API first.

`POST /v1/workflows/voiceover-to-video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

