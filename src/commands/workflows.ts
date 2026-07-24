import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag, selfOnlyFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const workflowsCommands: ResourceCommands = {
  // @sdk-operation scriptToVideo
  "script-to-video": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.workflows.scriptToVideo>[0];
    printJson(
      wait
        ? await client.workflows.scriptToVideoAndWait(request)
        : await client.workflows.scriptToVideo(request),
    );
  },

  // @sdk-operation voiceoverToVideo
  "voiceover-to-video": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.workflows.voiceoverToVideo>[0];
    printJson(
      wait
        ? await client.workflows.voiceoverToVideoAndWait(request)
        : await client.workflows.voiceoverToVideo(request),
    );
  },

  // @sdk-operation slideshowToVideo
  "slideshow-to-video": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.workflows.slideshowToVideo>[0];
    printJson(
      wait
        ? await client.workflows.slideshowToVideoAndWait(request)
        : await client.workflows.slideshowToVideo(request),
    );
  },

  // @sdk-operation promptToVideoClip
  "prompt-to-video-clip": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.workflows.promptToVideoClip>[0];
    printJson(
      wait
        ? await client.workflows.promptToVideoClipAndWait(request)
        : await client.workflows.promptToVideoClip(request),
    );
  },

  // @sdk-operation contentOutlineToVideo
  "content-outline-to-video": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.workflows.contentOutlineToVideo>[0];
    printJson(
      wait
        ? await client.workflows.contentOutlineToVideoAndWait(request)
        : await client.workflows.contentOutlineToVideo(request),
    );
  },

  // @sdk-operation listWorkflowRuns
  "list-workflow-runs": async ({ client, args }) => {
    printJson(
      await client.workflows.listWorkflowRuns({
        ...paginationFlags(args.flags),
        ...selfOnlyFlag(args.flags),
      }),
    );
  },

  // @sdk-operation getWorkflowRun
  "get-workflow-run": async ({ client, args }) => {
    const workflowRunId = requireStringFlag(args.flags, "workflowRunId");
    printJson(await client.workflows.getWorkflowRun({ workflowRunId }));
  },

  // @sdk-operation cancelWorkflowRun
  "cancel-workflow-run": async ({ client, args }) => {
    const workflowRunId = requireStringFlag(args.flags, "workflowRunId");
    printJson(await client.workflows.cancelWorkflowRun({ workflowRunId }));
  },
};
