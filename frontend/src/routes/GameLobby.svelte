<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import type { GameDetails } from "../types/Game";
  import { Server } from "../types/proto/message";
  import { IncomingMessages, IncomingMessageType } from "../types/Messages";
  import { Task, TaskType } from "../types/Tasks";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./tasks/Settings.svelte";
  import WaitForTask from "./tasks/WaitForTask.svelte";
  import DiscloseRole from "./tasks/DiscloseRole.svelte";
  import Discuss from "./tasks/Discuss.svelte";
  import { sendRequest } from "../utils/requests";

  export let params: { token?: string } = {};
  let details: GameDetails | null = null;
  let currentTask: Task | null = null;
  let ws: WebSocket | null = null;

  const refreshGame = async () => {
    const res = await sendRequest<GameDetails>(
      `/api/games/${params.token}/details`,
      "GET"
    );
    if (!res) {
      details = null;
      return;
    }

    details = res;
  };

  const leaveGame = async () => {
    ws?.close();
    if (getToken()) {
      await sendRequest(`/api/games/${params.token}/leave`, "POST");
    }
    await push("/games");
  };

  const fetchGamePeriodically = async () => {
    try {
      await refreshGame();
    } catch (err) {
      console.error(err);
      await push("/errors/unexpected");
    }
    createWsConnection();
  };

  async function createWsConnection() {
    if (ws) {
      return;
    }

    ws = new WebSocket("ws://localhost:3333/api/active_game");
    ws.onopen = () => {
      ws?.send(JSON.stringify({ auth: { token: getToken() } }));
    };
    ws.onclose = () => {
      ws = null;
    };
    ws.onerror = (ev) => {
      console.error("Error", ev);
    };
    ws.onmessage = (ev: MessageEvent<Uint8Array>) => {
      try {
        const msg = Server.decode(ev.data);
        if (msg.newTask) {
          const { task } = msg.newTask;
          if (task?.discloseRole) {
            currentTask = {
              type: TaskType.DiscloseRole,
              role: {
                description: "",
                name: "",
                party: "bad",
              },
            };
          }
        } else if (msg.playerUpdated) {
          if (details) {
            const { player } = msg.playerUpdated;
            console.log(player);
            // TODOdetails.players[player.id] = player;
          }
        } else if (msg.gameUpdated) {
          if (details) {
            const { game } = msg.gameUpdated;
            console.log(game);
            // TODO details.game = game;
          }
        } else {
          console.warn("Unknown task type: " + Object.keys(msg));
        }
      } catch (err) {
        console.error("Parsing task has failed");
      }
    };
  }
</script>

<Dialog>
  {#await fetchGamePeriodically()}
    <DialogHeader>Lobby</DialogHeader>
    <p>Loading game</p>
  {:then _}
    {#if details !== null}
      {#if !currentTask}
        <WaitForTask {leaveGame} />
      {:else if currentTask.type === TaskType.Settings}
        <Settings {leaveGame} {refreshGame} {details} />
      {:else if currentTask.type === TaskType.DiscloseRole}
        <DiscloseRole {leaveGame} role={currentTask.role} />
      {:else if currentTask.type === TaskType.Discuss}
        <Discuss {leaveGame} />
      {:else}
        <p>Unsupported Game State.</p>
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
  {/await}
</Dialog>
