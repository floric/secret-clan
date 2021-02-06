<script lang="typescript">
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";

  export let leaveGame: () => Promise<void>;
</script>

<DialogHeader>Please wait</DialogHeader>
<div class="mb-4">Please wait for next steps...</div>
<ActionRow>
  <div />
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
