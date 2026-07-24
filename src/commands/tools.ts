import type { ParsedArgs } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag, selfOnlyFlag } from "./flags";
import type { ResourceCommands } from "./types";

const runTool = async <T extends object>({
  args,
  wait,
  start,
  andWait,
}: {
  args: ParsedArgs;
  wait: boolean;
  start: (request: T) => Promise<unknown>;
  andWait: (request: T) => Promise<unknown>;
}): Promise<void> => {
  // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
  const request = (await resolveRequest({ flags: args.flags, requireBody: true })) as T;
  printJson(wait ? await andWait(request) : await start(request));
};

export const toolsCommands: ResourceCommands = {
  // @sdk-operation generateImage
  "generate-image": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateImage>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateImage(request),
      andWait: (request) => ctx.client.tools.generateImageAndWait(request),
    }),

  // @sdk-operation generateVideoClip
  "generate-video-clip": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateVideoClip>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateVideoClip(request),
      andWait: (request) => ctx.client.tools.generateVideoClipAndWait(request),
    }),

  // @sdk-operation generateMotionGraphic
  "generate-motion-graphic": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateMotionGraphic>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateMotionGraphic(request),
      andWait: (request) => ctx.client.tools.generateMotionGraphicAndWait(request),
    }),

  // @sdk-operation textToSpeech
  "text-to-speech": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.textToSpeech>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.textToSpeech(request),
      andWait: (request) => ctx.client.tools.textToSpeechAndWait(request),
    }),

  // @sdk-operation generateSoundEffect
  "generate-sound-effect": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateSoundEffect>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateSoundEffect(request),
      andWait: (request) => ctx.client.tools.generateSoundEffectAndWait(request),
    }),

  // @sdk-operation generateMusic
  "generate-music": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateMusic>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateMusic(request),
      andWait: (request) => ctx.client.tools.generateMusicAndWait(request),
    }),

  // @sdk-operation generateAvatar
  "generate-avatar": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.generateAvatar>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.generateAvatar(request),
      andWait: (request) => ctx.client.tools.generateAvatarAndWait(request),
    }),

  // @sdk-operation vectorizeImage
  "vectorize-image": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.vectorizeImage>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.vectorizeImage(request),
      andWait: (request) => ctx.client.tools.vectorizeImageAndWait(request),
    }),

  // @sdk-operation removeImageBackground
  "remove-image-background": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.removeImageBackground>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.removeImageBackground(request),
      andWait: (request) => ctx.client.tools.removeImageBackgroundAndWait(request),
    }),

  // @sdk-operation removeVideoBackground
  "remove-video-background": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.removeVideoBackground>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.removeVideoBackground(request),
      andWait: (request) => ctx.client.tools.removeVideoBackgroundAndWait(request),
    }),

  // @sdk-operation upscaleImage
  "upscale-image": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.upscaleImage>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.upscaleImage(request),
      andWait: (request) => ctx.client.tools.upscaleImageAndWait(request),
    }),

  // @sdk-operation upscaleVideo
  "upscale-video": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.upscaleVideo>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.upscaleVideo(request),
      andWait: (request) => ctx.client.tools.upscaleVideoAndWait(request),
    }),

  // @sdk-operation image3dEffect
  "image-3d-effect": async (ctx) =>
    runTool<Parameters<typeof ctx.client.tools.image3dEffect>[0]>({
      args: ctx.args,
      wait: ctx.wait,
      start: (request) => ctx.client.tools.image3dEffect(request),
      andWait: (request) => ctx.client.tools.image3dEffectAndWait(request),
    }),

  // @sdk-operation listToolExecutions
  "list-tool-executions": async ({ client, args }) => {
    printJson(
      await client.tools.listToolExecutions({
        ...paginationFlags(args.flags),
        ...selfOnlyFlag(args.flags),
      }),
    );
  },

  // @sdk-operation getToolExecutionInfo
  "get-tool-execution-info": async ({ client, args }) => {
    const toolExecutionId = requireStringFlag(args.flags, "toolExecutionId");
    printJson(await client.tools.getToolExecutionInfo({ toolExecutionId }));
  },

  // @sdk-operation cancelToolExecution
  "cancel-tool-execution": async ({ client, args }) => {
    const toolExecutionId = requireStringFlag(args.flags, "toolExecutionId");
    printJson(await client.tools.cancelToolExecution({ toolExecutionId }));
  },
};
