<script lang="typescript">
  import { push } from "svelte-spa-router";
  import type { Game, GameStats } from "../types/Game";
  import ActionRow from "../components/buttons/ActionRow.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import SecondaryButton from "../components/buttons/Secondary.svelte";
  import Dialog from "../components/layout/Dialog.svelte";
  import Divider from "../components/layout/Divider.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { saveToken } from "../utils/auth";
  import TextInput from "../components/inputs/TextInput.svelte";
  import { sendRequest } from "../utils/requests";

  let inputToken = "";
  let inputName = "";

  type AttendGameResponse = {
    game: Game;
    token: string;
  };

  async function createGame() {
    try {
      const game = await sendRequest<AttendGameResponse>("/api/games", "PUT");
      if (!game) {
        throw new Error("Game creation failed");
      }
      saveToken(game.token);
      await push(`/games/${game.game.token}`);
    } catch (err) {
      // TODO Handle all API errors in a generic way
    }
  }

  async function attendGame() {
    try {
      const game = await sendRequest<AttendGameResponse>(
        `/api/games/${inputToken}/attend`,
        "POST"
      );
      if (!game) {
        // TODO Check name and if game exists, show helpful message
        return;
      }

      saveToken(game.token);

      await push(`/games/${inputToken?.trim()}`);
    } catch (err) {
      // TODO Handle all API errors in a generic way
    }
  }
  const loadStats = async () => {
    const stats = await sendRequest<GameStats>(`/api/games/`, "GET");
    if (!stats) {
      return { total: 0 };
    }

    return stats;
  };
</script>

<Dialog>
  <DialogHeader>Start Game</DialogHeader>
  <p class="mb-4">
    {#await loadStats()}
      Loading stats...
    {:then { total }}
      There are currently
      {total}
      active games.
    {/await}
  </p>
  <SecondaryButton on:click={createGame}>Create new game</SecondaryButton>

  <Divider><span slot="text">or</span></Divider>

  <form on:submit|preventDefault={attendGame}>
    <div class="grid grid-cols-1 md:grid-cols-2 mb-6 gap-4">
      <TextInput id="token" placeholder="Token" bind:value={inputToken} />
      <PrimaryButton>Attend</PrimaryButton>
    </div>
    <ActionRow>
      <div />
      <InternalLink href="/">Back to Start</InternalLink>
    </ActionRow>
  </form>
</Dialog>
