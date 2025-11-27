<script lang="ts">
    import IconTrash from '$icons/IconTrash.svelte';
    import IconBackspace from '$icons/IconBackspace.svelte';
    import ToolbarIconHtml from '$lib/text_edit/html/tools/ToolbarIconHtml.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';

    let {
        ref,
        iconSize,
    }: {
        ref: undefined | HTMLDivElement;
        iconSize: string;
    } = $props();

    let ta = useI18nAdmin();

    function onClickCustom(range: Range) {
        // let r = range.cloneRange();
        // let s = range.toString();
        //
        // let ancestor = range.commonAncestorContainer;
        // let i = 0;
        // console.log('ref', ref);
        // while (i < 100) {
        //     r.setStart(range.startContainer, range.startOffset);
        //     r.setEnd(range.endContainer, range.endOffset);
        //
        //     let s = range.toString();
        //     r.deleteContents();
        //     r.insertNode(document.createTextNode(s));
        //
        //     if (ancestor.nodeName.toLowerCase() === 'p' || ancestor === ref) {
        //         console.log('found ancestor');
        //         break;
        //     }
        //
        //     console.log('ancestor', ancestor);
        //     r.selectNode(ancestor);
        //     ancestor = r.commonAncestorContainer;
        //
        //     i += 1;
        // }
        // console.log('ancestor after', i, 'iterations:', ancestor);
        // r.deleteContents();
        // r.insertNode(document.createTextNode(s));

        // TODO make it work for inner selections via ancestor split

        let node = document.createTextNode(range.toString());
        range.deleteContents();
        range.insertNode(node);
    }

    // function splitLeft(range: Range, nodes: Node[]) {
    //     console.log('split Left', nodes);
    //     let left = new Range();
    //     let right = new Range();
    //     let startsBefore: boolean;
    //     let endsInside: boolean;
    //
    //     for (const node of nodes) {
    //         if (node.hasChildNodes()) {
    //             splitLeft(range, [...node.childNodes.values()]);
    //         }
    //         left.selectNode(node);
    //
    //         startsBefore = range.compareBoundaryPoints(Range.START_TO_START, left) === 1;
    //         if (startsBefore) {
    //             endsInside = range.compareBoundaryPoints(Range.END_TO_START, left) === -1;
    //             if (endsInside) {
    //                 console.log('node starts before and ends inside', node);
    //                 left.selectNodeContents(node);
    //                 left.setEnd(range.startContainer, range.startOffset);
    //                 let strLeft = left.toString();
    //                 console.log('strLEft', strLeft);
    //
    //                 right.selectNodeContents(node);
    //                 right.setStart(range.startContainer, range.startOffset);
    //                 let strRight = right.toString();
    //                 console.log('strRight', strRight);
    //
    //                 left.deleteContents();
    //                 left.insertNode(document.createTextNode(strLeft));
    //
    //                 right.deleteContents();
    //                 right.insertNode(document.createTextNode(strRight));
    //             }
    //         }
    //     }
    // }
    //
    // function splitRight(range: Range, nodes: Node[]) {
    //     console.warn('todo split Right');
    // }
</script>

<ToolbarIconHtml
    {ref}
    title={ta.editor.removeFmt}
    {onClickCustom}
>
    <IconBackspace width={iconSize} />
</ToolbarIconHtml>
