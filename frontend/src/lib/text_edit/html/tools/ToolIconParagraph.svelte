<script lang="ts">

    import ToolbarIconHtml from "$lib/text_edit/html/tools/ToolbarIconHtml.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import Icon3BarsLeft from "$icons/editor/Icon3BarsLeft.svelte";

    let {
        ref,
        iconSize,
    }: {
        ref: undefined | HTMLDivElement,
        iconSize: string,
    } = $props();

    let ta = useI18nAdmin();

    function onClickCustom(range: Range) {
        if (!ref) {
            return;
        }

        range.selectNodeContents(ref);
        range.setStart(range.endContainer, range.endOffset);

        const p = document.createElement('p');
        range.insertNode(p)
        range.selectNodeContents(p);
        range.insertNode(document.createTextNode('...'));
    }
</script>

<ToolbarIconHtml {ref} title={ta.editor.paragraph} {onClickCustom}>
    <Icon3BarsLeft width={iconSize}/>
</ToolbarIconHtml>
