<script lang="ts">
    import IconPencil from '#icons/IconPencil.svelte';

    let {
        files = $bindable(),
        ariaLabel,
        accept,
        disabled,
    }: {
        files?: File[];
        ariaLabel: string;
        accept: string;
        disabled?: boolean;
    } = $props();
</script>

<label class="image-edit" title={ariaLabel}>
    <input
        type="file"
        aria-label={ariaLabel}
        {accept}
        {disabled}
        onchange={event => {
            files = Array.from(event.currentTarget.files ?? []);
            event.currentTarget.value = '';
        }}
    />
    <span class="edit-picture"><IconPencil width="0.875rem" /></span>
</label>

<style>
    .image-edit {
        position: absolute;
        inset: 0;
        z-index: 1;
        display: grid;
        place-items: end;
        padding: 3px;
        cursor: pointer;
    }

    .image-edit:has(input:disabled) {
        pointer-events: none;
        opacity: 0.6;
    }

    input {
        position: absolute;
        inline-size: 1px;
        block-size: 1px;
        clip-path: inset(50%);
    }

    .edit-picture {
        display: grid;
        place-items: center;
        inline-size: 1.375rem;
        block-size: 1.375rem;
        border: 1px solid var(--ui-outline, hsl(var(--bg-high)));
        border-radius: 50%;
        background: var(--ui-surface, hsl(var(--bg)));
        color: var(--ui-muted, hsl(var(--text)));
        opacity: 0;
        transition: opacity 140ms ease;
    }

    .image-edit:hover .edit-picture,
    .image-edit:focus-within .edit-picture {
        opacity: 1;
    }

    .image-edit:hover .edit-picture,
    input:focus-visible + .edit-picture {
        border-color: var(--ui-primary, hsl(var(--action)));
        color: var(--ui-primary, hsl(var(--action)));
    }

    @media (hover: none) {
        .image-edit:not(:focus-within) .edit-picture {
            opacity: 0.4;
        }
    }

    input:focus-visible + .edit-picture {
        outline: 2px solid var(--ui-primary, hsl(var(--action)));
        outline-offset: 1px;
    }
</style>
