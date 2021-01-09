<script lang="typescript">
    import type { Role } from "../../types/Role";
    import SecondaryButton from "../../components/buttons/Secondary.svelte";
    import PrimaryButton from "../../components/buttons/Primary.svelte";
    import ActionRow from "../../components/buttons/ActionRow.svelte";
    import DialogHeader from "../../components/headers/DialogHeader.svelte";
    import { sendRequest } from "../../utils/requests";

    export let leaveGame: () => Promise<void>;
    export let role: Role;

    const acknowledge = async () => {
        await sendRequest(`/api/tasks/disclose-role`, "POST", {
            acknowledge: true,
        });
    };
    const decline = async () => {
        await sendRequest(`/api/tasks/disclose-role`, "POST", {
            acknowledge: false,
        });
    };
</script>

<DialogHeader>Your role</DialogHeader>
<div class="flex flex-col md:items-center">
    <div
        class="mb-8 md:m-8 p-4 md:p-8 rounded-xl  {role.party === 'good' ? 'border-green-600' : 'border-red-600'} border-4 flex items-center flex-col">
        <div class="mb-4">
            <span class="font-bold">{role.name}</span>
            ({role.party})
        </div>
        <div>{role.description}</div>
    </div>
    <div class="max-w-full md:max-w-md lg:max-w-lg grid gap-4 md:grid-cols-2">
        <PrimaryButton onClick={acknowledge}>Everything OK</PrimaryButton>
        <SecondaryButton onClick={decline}>Role unclear</SecondaryButton>
    </div>
</div>
<ActionRow>
    <div />
    <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
