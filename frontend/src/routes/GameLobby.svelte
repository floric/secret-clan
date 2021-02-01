<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import { Client, Server } from "../types/proto/message";
  import type { Game } from "../types/proto/game";
  import type { Player } from "../types/proto/player";
  import type { Task } from "../types/proto/task";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./tasks/Settings.svelte";
  import WaitForTask from "./tasks/WaitForTask.svelte";
  import { sendRequest } from "../utils/requests";

  export let params: { token?: string } = {};
  let currentGame: Game | null = null;
  let players: Record<string, Player> = {};
  let currentTask: Task | null = null;
  let ws: WebSocket | null = null;
  let connectSuccessful = false;

  const leaveGame = async () => {
    ws?.close();
    if (getToken()) {
      await sendRequest(`/api/games/${params.token}/leave`, "POST");
    }
    await push("/games");
  };

  function createWsConnection() {
    if (ws) {
      return;
    }

    ws = new WebSocket("ws://localhost:3333/api/active_game");
    ws.onopen = () => {
      ws?.send(
        Client.encode({
          message: {
            $case: "authConfirmed",
            authConfirmed: { token: getToken() || "" },
          },
        }).finish()
      );
      connectSuccessful = true;
    };
    ws.onclose = () => {
      ws = null;
    };
    ws.onerror = (ev) => {
      console.error("Error", ev);
    };
    ws.onmessage = async (ev: MessageEvent<Blob>) => {
      try {
        const raw = await ev.data.arrayBuffer();
        const { message } = Server.decode(new Uint8Array(raw));
        console.info("Received new message", message);
        if (message?.$case === "playerUpdated") {
          const { player } = message.playerUpdated;
          if (player?.id && players[player!.id]) {
            players[player!.id] = player!;
          }
        } else if (message?.$case === "selfUpdated") {
          const { player } = message.selfUpdated;
          players[player!.id] = { id: player!.id, name: player!.name };
          currentTask = player?.openTasks[0] || null;
        } else if (message?.$case === "gameUpdated") {
          const { game } = message.gameUpdated;
          currentGame = game!;
        } else if (message?.$case === "playerEntered") {
          const { player } = message.playerEntered;
          players[player!.id] = player!;
        } else if (message?.$case === "playerLeft") {
          const { playerId } = message.playerLeft;
          delete players[playerId!];
          players = players;
        } else {
          console.warn("Unknown task type");
        }
      } catch (err) {
        console.error("Parsing task has failed", err);
      }
    };
  }

  createWsConnection();
</script>

<Dialog>
  {#if !ws}
    <DialogHeader>Lobby</DialogHeader>
    {#if !connectSuccessful}
      <p>Loading game</p>
    {:else}
      <p>Connection lost</p>
    {/if}
  {:else if currentGame !== null}
    {#if currentTask?.definition?.$case === "settings"}
      <Settings {leaveGame} {currentGame} {players} {ws} />
    {:else}
      <WaitForTask {leaveGame} />
    {/if}
  {:else}
    <DialogHeader>Lobby</DialogHeader>
    <p>Game doesn't exist.</p>
    <div class="flex items-center">
      <div class="flex ml-auto">
        <InternalLink href="/games">Back to Games</InternalLink>
      </div>
    </div>
  {/if}
</Dialog>
