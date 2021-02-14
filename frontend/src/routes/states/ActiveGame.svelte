<script lang="typescript">
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import InactivePlayer from "../../components/game/InactivePlayer.svelte";
  import type { Game } from "../../types/proto/game";
  import type { Player, OwnPlayer } from "../../types/proto/player";
  import Divider from "../../components/layout/Divider.svelte";

  export let currentGame: Game;
  export let players: Record<string, { player: Player; active: boolean }>;
  export let ws: WebSocket;
  export let ownPlayer: OwnPlayer;
  export let leaveGame: () => Promise<void>;
</script>

<DialogHeader>Game</DialogHeader>
<div>
  <div class="grid grid-flow-col gap-4">
    {#each Object.values(players) as p}
      <div class="border border-gray-200 p-4 rounded-md">
        <div class="font-bold">
          {p.player.name}
          <InactivePlayer isActive={p.active} />
        </div>
        <div>{p.player.credits} Credits</div>
      </div>
    {/each}
  </div>
  <Divider />
  <div class="flex justify-center">
    <div>Pot: {currentGame.pot} Credits</div>
  </div>
</div>
<ActionRow>
  <div />
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
