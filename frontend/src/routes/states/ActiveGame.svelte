<script lang="typescript">
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import InactivePlayer from "../../components/game/InactivePlayer.svelte";
  import Credits from "../../components/game/Credits.svelte";
  import type { Game } from "../../types/proto/game";
  import type { Player, OwnPlayer } from "../../types/proto/player";
  import Divider from "../../components/layout/Divider.svelte";

  export let currentGame: Game;
  export let players: Record<string, { player: Player; active: boolean }>;
  export let ws: WebSocket;
  export let ownPlayer: OwnPlayer;
  export let leaveGame: () => Promise<void>;
</script>

<DialogHeader>Round</DialogHeader>
<div>
  <div class="grid grid-flow-col gap-4">
    {#each Object.values(players).sort((a, b) => b.player.position - a.player.position) as p}
      <div class="border border-gray-200 p-4 rounded-md">
        <div class="font-bold flex justify-between items-center mb-2">
          {p.player.name}
          <InactivePlayer isActive={p.active} />
          {#if currentGame.smallBlindId == p.player.id}
            <div
              class="rounded-full text-xs bg-green-700 text-white w-6 h-6 justify-center items-center inline-flex"
            >
              B
            </div>
          {/if}
          {#if currentGame.bigBlindId == p.player.id}
            <div
              class="rounded-full text-xs bg-red-800 text-white w-6 h-6 justify-center items-center inline-flex"
            >
              S
            </div>
          {/if}
        </div>
        <div><Credits amount={p.player.credits} /></div>
      </div>
    {/each}
  </div>
  <Divider />
  <div class="flex justify-center">
    <div>
      Pot: <Credits amount={currentGame.pot} /> | Blinds: <Credits
        amount={currentGame.blind}
      />
    </div>
  </div>
</div>
<ActionRow>
  <div />
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
