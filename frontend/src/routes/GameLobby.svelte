<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import { Client, Server } from "../types/proto/message";
  import { Game, Game_State } from "../types/proto/game";
  import type { OwnPlayer, Player } from "../types/proto/player";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./states/Settings.svelte";
  import WaitForTask from "./states/WaitForTask.svelte";
  import ActiveGame from "./states/ActiveGame.svelte";
  import { sendRequest } from "../utils/requests";

  export let params: { token?: string } = {};
  let currentGame: Game | null = null;
  let ownPlayer: OwnPlayer | null = null;
  let players: Record<string, Player> = {};
  let ws: WebSocket | null = null;
  let connectSuccessful = false;
  let connectClosed = false;

  const leaveGame = async () => {
    ws?.close();
    if (getToken()) {
      // TODO use WS message
      await sendRequest(`/api/games/${params.token}/leave`, "POST");
    }
    await push("/games");
  };

  function createWsConnection() {
    if (ws) {
      return;
    }

    ws = new WebSocket(
      `${window.location.protocol === "https:" ? "wss" : "ws"}://${
        window.location.host
      }/api/active_game`
    );
    ws.onopen = () => {
      connectClosed = false;
      connectSuccessful = true;
      ws?.send(
        Client.encode({
          message: {
            $case: "authConfirmed",
            authConfirmed: { token: getToken() || "" },
          },
        }).finish()
      );
    };
    ws.onclose = (ev) => {
      ws = null;
      connectClosed = true;
    };
    ws.onerror = (ev) => {
      console.error("Error", ev);
    };
    ws.onmessage = async (ev: MessageEvent<Blob>) => {
      try {
        const raw = await ev.data.arrayBuffer();
        const { message } = Server.decode(new Uint8Array(raw));
        console.info("Incoming message", message);
        if (message?.$case === "playerUpdated") {
          const { player } = message.playerUpdated;
          if (player?.id && players[player!.id]) {
            players[player!.id] = player!;
          }
        } else if (message?.$case === "selfUpdated") {
          const { player } = message.selfUpdated;
          ownPlayer = player!;
          players[player!.id] = {
            id: player!.id,
            name: player!.name,
            credits: player!.credits,
          };
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
        } else if (message?.$case === "gameDeclined") {
          currentGame = null;
          ownPlayer = null;
          connectClosed = true;
          ws?.close();
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
  {#if currentGame && ownPlayer && ws}
    {#if currentGame?.state === Game_State.INITIALIZED}
      <Settings {leaveGame} {currentGame} {players} {ownPlayer} {ws} />
    {:else if currentGame?.state === Game_State.STARTED && ownPlayer}
      <ActiveGame {leaveGame} {currentGame} {players} {ownPlayer} {ws} />
    {:else}
      <WaitForTask {leaveGame} />
    {/if}
  {:else}
    <DialogHeader>Lobby</DialogHeader>
    {#if connectClosed && connectSuccessful}
      <p>Game doesn't exist.</p>
      <div class="flex items-center">
        <div class="flex ml-auto">
          <InternalLink href="/games">Back to Games</InternalLink>
        </div>
      </div>
    {:else if connectClosed}
      <p>Connection lost</p>
    {:else}
      <p>Loading game</p>
    {/if}
  {/if}
</Dialog>
