<script lang="typescript">
  import PrimaryButton from "../../components/buttons/Primary.svelte";
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import TextInput from "../../components/inputs/TextInput.svelte";
  import Label from "../../components/inputs/Label.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import { getClaims, getToken } from "../../utils/auth";
  import { sendRequest } from "../../utils/requests";
  import { Client } from "../../types/proto/message";
  import type { Game } from "../../types/proto/game";
  import type { Player } from "../../types/proto/player";

  export let currentGame: Game;
  export let players: Record<string, Player>;
  export let ws: WebSocket;
  export let leaveGame: () => Promise<void>;
  const claims = getClaims();
  const currentName = players[claims.sub]?.name;

  const startGame = async () => {
    await sendRequest(`/api/games/${currentGame.token}/start`, "POST");
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
        value={currentName || ""}
        on:change={onChangeName}
      />
    </div>
  </div>
  <div>
    <h4 class="font-bold mb-4">Players</h4>
    <ul>
      {#each Object.values(players).sort((a, b) =>
        a.name.localeCompare(b.name)
      ) as p}
        <li>
          {p.name}
          {#if p.id === currentGame.adminId}
            <span class="font-bold">(Admin)</span>
          {/if}
        </li>
      {/each}
    </ul>
  </div>
</div>
<ActionRow>
  {#if currentGame.adminId === claims.sub}
    <PrimaryButton onClick={startGame}>Start</PrimaryButton>
  {:else}
    <p>Wait for the game to start.</p>
  {/if}
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
