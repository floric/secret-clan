<script lang="typescript">
  import PrimaryButton from "../../components/buttons/Primary.svelte";
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import TextInput from "../../components/inputs/TextInput.svelte";
  import Label from "../../components/inputs/Label.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import InactivePlayer from "../../components/game/InactivePlayer.svelte";
  import { Client } from "../../types/proto/message";
  import type { Game } from "../../types/proto/game";
  import type { Player, OwnPlayer } from "../../types/proto/player";

  export let currentGame: Game;
  export let players: Record<string, { player: Player; active: boolean }>;
  export let ws: WebSocket;
  export let leaveGame: () => Promise<void>;
  export let ownPlayer: OwnPlayer;

  const startGame = async () => {
    ws?.send(
      Client.encode({
        message: {
          $case: "gameStarted",
          gameStarted: {},
        },
      }).finish()
    );
  };

  const onChangeName = async (ev: any) => {
    const name = ev?.target?.value;
    if (!name) {
      return;
    }
    ws?.send(
      Client.encode({
        message: {
          $case: "nameUpdated",
          nameUpdated: { name },
        },
      }).finish()
    );
  };
</script>

<DialogHeader>Lobby</DialogHeader>
<div class="mb-8 flex items-center">
  <div>Token</div>
  <div
    class="font-extrabold mx-4 px-3 py-2 rounded-md bg-gray-200 border-black"
  >
    {currentGame.token.toUpperCase()}
  </div>
</div>
<div class="grid md:grid-cols-3 grid-cols-1 gap-8 mb-8">
  <div class="md:col-span-2">
    <h4 class="font-bold mb-4">Settings</h4>
    <div class="max-w-xs">
      <Label target="name">Your Name</Label>
      <TextInput
        id="name"
        placeholder="Name"
        value={ownPlayer.name}
        on:change={onChangeName}
      />
    </div>
  </div>
  <div>
    <h4 class="font-bold mb-4">{Object.values(players).length} Players</h4>
    <ul>
      {#each Object.values(players).sort((a, b) => b.player.position - a.player.position) as p}
        <li>
          {p.player.name}
          {#if p.player.id === currentGame.adminId}
            <span class="font-bold">(Admin)</span>
          {/if}
          <InactivePlayer isActive={p.active} />
        </li>
      {/each}
    </ul>
  </div>
</div>
<ActionRow>
  {#if currentGame.adminId === ownPlayer.id}
    <PrimaryButton
      disabled={Object.values(players).length < 2}
      onClick={startGame}>Start</PrimaryButton
    >
  {:else}
    <p>Wait for the game to start.</p>
  {/if}
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
