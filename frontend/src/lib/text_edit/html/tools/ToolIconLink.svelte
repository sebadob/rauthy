<script lang="ts">
    import Input from '$lib/form/Input.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { elementsInRangeFiltered } from '$lib/text_edit/html/utils';
    import ToolbarIconHtml from '$lib/text_edit/html/tools/ToolbarIconHtml.svelte';
    import IconLink from '$icons/editor/IconLink.svelte';
    import Modal from '$lib/Modal.svelte';
    import Form from '$lib/form/Form.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';

    let {
        ref,
        iconSize,
    }: {
        ref: undefined | HTMLDivElement;
        iconSize: string;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let refName: undefined | HTMLInputElement = $state();
    let refHref: undefined | HTMLInputElement = $state();

    let range: undefined | Range;
    let name = $state('');
    let href = $state('');

    function addLink(r: Range) {
        showModal = true;

        let [left, right] = elementsInRangeFiltered(r);
        if (left.length === 0 && right.length === 0) {
            name = r.toString();
        } else {
            console.log('got intersecting link creation range', left, right);
            // if we have intersections, just append the link to the end for now
            r.setStart(r.endContainer, r.endOffset);
        }

        requestAnimationFrame(() => {
            if (name) {
                refHref?.focus();
            } else {
                refName?.focus();
            }
        });

        range = r;
    }

    function onSubmit() {
        if (!range) {
            console.error('range is undefined when it should not be');
            return;
        }

        let text = document.createTextNode(name || 'Link');
        range.deleteContents();
        range.insertNode(text);

        let link = document.createElement('a');
        link.setAttribute('href', href);
        link.setAttribute('target', '_blank');
        range.surroundContents(link);

        range.selectNode(text);
        closeModal?.();
    }

    function onClose() {
        range = undefined;
        name = '';
        href = '';
    }
</script>

<ToolbarIconHtml
    {ref}
    title={ta.editor.link}
    onClickCustom={addLink}
>
    <IconLink width={iconSize} />
</ToolbarIconHtml>
<Modal
    bind:showModal
    bind:closeModal
    {onClose}
>
    <div>
        <h3>{ta.editor.link}</h3>
        <Form
            action=""
            {onSubmit}
        >
            <Input
                bind:ref={refName}
                bind:value={name}
                label={ta.common.name}
                placeholder={ta.common.name}
            />
            <Input
                bind:ref={refHref}
                typ="url"
                bind:value={href}
                label={ta.editor.link}
                placeholder={ta.editor.link}
            />
            <Button type="submit">{t.common.save}</Button>
        </Form>
    </div>
</Modal>

<style>
    div {
        text-align: left;
    }
</style>
