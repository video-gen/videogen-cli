import assert from "node:assert/strict";
import { test } from "node:test";
import { VideoGen } from "@videogen/sdk";
import { parseArgs } from "../src/args.ts";
import { RESOURCES, listUsage } from "../src/commands/index.ts";

const projectId = "vg_proj_cli";
const editableVideoId = "vg_art_cli";
const videoEditId = "vg_edit_cli";
const started = {
  projectId,
  projectUrl: `https://app.videogen.io/project/${projectId}`,
  videoEditIds: [videoEditId],
};
const completed = {
  videoEdits: [
    {
      videoEditId,
      editableVideoId,
      status: "succeeded",
      progressPercentage: 100,
    },
  ],
  hasMore: false,
  nextCursor: null,
};

const invoke = (name, flags = [], wait = false) =>
  RESOURCES.projects[name]({
    client: new VideoGen({
      apiKey: "test-api-key",
      baseUrl: "https://example.test",
      clientId: "cli",
    }),
    args: parseArgs(["projects", name, ...flags]),
    wait,
  });

const capture = (t, responses) => {
  const requests = [];
  const output = [];
  t.mock.method(globalThis, "fetch", async (url, init) => {
    requests.push({ url, init });
    const response = responses[requests.length - 1];
    assert.ok(response, "The command sent an unexpected request.");
    return new Response(JSON.stringify(response.body ?? response), {
      status: response.statusCode ?? 200,
      headers: { "Content-Type": "application/json" },
    });
  });
  t.mock.method(process.stdout, "write", (chunk) => {
    output.push(String(chunk));
    return true;
  });
  return { requests, output };
};

test("help advertises video edits while old commands remain executable", () => {
  const usage = listUsage();
  assert.match(usage, /start-video-edits/);
  assert.match(usage, /list-video-edits/);
  assert.doesNotMatch(usage, /remix/i);
  assert.equal(typeof RESOURCES.projects["remix-project"], "function");
  assert.equal(
    typeof RESOURCES.projects["list-project-remix-actions"],
    "function",
  );
});

for (const targetInFlag of [false, true]) {
  test(`start-video-edits forwards the complete server-validated body (target flag: ${targetInFlag})`, async (t) => {
    const { requests, output } = capture(t, [started]);
    const videoEdits = [
      { type: "ENABLE_CAPTIONS", styleId: "preset_supplied_by_caller" },
    ];
    const body = {
      videoEdits,
      saveAsNewProject: true,
      ...(targetInFlag ? {} : { editableVideoId }),
    };
    await invoke("start-video-edits", [
      "--project-id",
      projectId,
      ...(targetInFlag ? ["--editable-video-id", editableVideoId] : []),
      "--body",
      JSON.stringify(body),
    ]);
    assert.equal(requests.length, 1);
    assert.equal(
      requests[0].url,
      `https://example.test/v1/projects/${projectId}/video-edits`,
    );
    assert.equal(requests[0].init.method, "POST");
    assert.deepEqual(JSON.parse(requests[0].init.body), {
      ...body,
      editableVideoId,
    });
    assert.deepEqual(JSON.parse(output.join("")), started);
  });
}

test("start-video-edits --wait polls the returned project and only the submitted edits", async (t) => {
  const response = {
    ...completed,
    videoEdits: [
      ...completed.videoEdits,
      {
        videoEditId: "vg_edit_other",
        editableVideoId: "vg_art_other",
        status: "running",
        progressPercentage: 0,
      },
    ],
  };
  const { requests, output } = capture(t, [
    { ...started, projectId: "vg_proj_copy" },
    response,
  ]);
  await invoke(
    "start-video-edits",
    [
      "--project-id",
      projectId,
      "--editable-video-id",
      editableVideoId,
      "--body",
      '{"videoEdits":[{"type":"ENABLE_CAPTIONS"}]}',
    ],
    true,
  );
  assert.equal(requests.length, 2);
  const pollUrl = new URL(requests[1].url);
  assert.equal(pollUrl.pathname, "/v1/projects/vg_proj_copy/video-edits");
  const result = JSON.parse(output.join(""));
  assert.ok(result.videoEdits.some((edit) => edit.videoEditId === videoEditId));
});

test("list-video-edits sends pagination and editable-video filtering", async (t) => {
  const { requests, output } = capture(t, [completed]);
  await invoke("list-video-edits", [
    "--project-id",
    projectId,
    "--editable-video-id",
    editableVideoId,
    "--limit",
    "25",
    "--cursor",
    "next cursor",
  ]);
  assert.equal(requests.length, 1);
  const url = new URL(requests[0].url);
  assert.equal(url.pathname, `/v1/projects/${projectId}/video-edits`);
  assert.equal(url.searchParams.get("editableVideoId"), editableVideoId);
  assert.equal(url.searchParams.get("limit"), "25");
  assert.equal(url.searchParams.get("cursor"), "next cursor");
  assert.deepEqual(JSON.parse(output.join("")), completed);
});

test("legacy project commands retain their request and response contracts", async (t) => {
  const legacyStarted = {
    projectId,
    projectUrl: started.projectUrl,
    remixActionIds: ["vg_remx_cli"],
  };
  const legacyHistory = { remixActions: [], hasMore: false, nextCursor: null };
  const { requests, output } = capture(t, [legacyStarted, legacyHistory]);
  const body = {
    remixActions: [{ type: "ADD_TRANSITIONS" }],
    saveAsNewProject: true,
  };
  await invoke("remix-project", [
    "--project-id",
    projectId,
    "--body",
    JSON.stringify(body),
  ]);
  await invoke("list-project-remix-actions", [
    "--project-id",
    projectId,
    "--limit",
    "10",
  ]);
  assert.equal(
    requests[0].url,
    `https://example.test/v1/projects/${projectId}/remix`,
  );
  assert.deepEqual(JSON.parse(requests[0].init.body), body);
  const historyUrl = new URL(requests[1].url);
  assert.equal(historyUrl.pathname, `/v1/projects/${projectId}/remix-actions`);
  assert.equal(historyUrl.searchParams.get("limit"), "10");
  assert.deepEqual(JSON.parse(output[0]), legacyStarted);
  assert.deepEqual(JSON.parse(output[1]), legacyHistory);
});

test("canonical start preserves server validation failures without polling or success output", async (t) => {
  const { requests, output } = capture(t, [
    { statusCode: 400, body: { message: "Select an editable video." } },
  ]);
  await assert.rejects(
    invoke(
      "start-video-edits",
      ["--project-id", projectId, "--body", '{"videoEdits":[]}'],
      true,
    ),
    /Select an editable video/,
  );
  assert.equal(requests.length, 1);
  assert.equal(output.length, 0);
});
