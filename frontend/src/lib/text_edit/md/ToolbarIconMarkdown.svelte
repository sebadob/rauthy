<script lang="ts">
    import type { Snippet } from 'svelte';
    import Button from '$lib/button/Button.svelte';

    let {
        ref,
        title,
        content = $bindable(),
        addLeft,
        addRight,
        addRightLink,
        children,
    }: {
        ref: undefined | HTMLDivElement;
        title: string;
        content: string;
        addLeft: string;
        addRight?: string;
        addRightLink?: string;
        children: Snippet;
    } = $props();

    function onclick(ev: Event) {
        if (!ref) {
            return;
        }

        let sel = document.getSelection();
        // make sure we have a valid selection inside our editor container
        if (
            !sel ||
            sel.rangeCount < 1 ||
            !ref.contains(sel.getRangeAt(0).commonAncestorContainer)
        ) {
            return;
        }

        const range = sel.getRangeAt(0);
        let isCaret = range.collapsed;

        if (addRightLink) {
            let textBefore = range.toString();

            const node = document.createTextNode(addLeft + textBefore + '](' + addRightLink + ')');
            range.deleteContents();
            range.insertNode(node);

            range.setStart(node, node.length - 1 - addRightLink.length);
            range.setEnd(node, node.length - 1);
        } else if (!addRight) {
            // if we only have addRight, this will be considered as an
            // "add at the beginning of the line" -> search all brs before

            let brs = ref.getElementsByTagName('br');
            if (brs.length === 0) {
                // special case, there is only the first line
                content = addLeft + content;
                return;
            }

            let rc = range.cloneRange();
            // search through all brs and find the one before
            let brBefore;
            let brsIntersect = [];
            for (let i = 0; i < brs.length; i++) {
                if (!brBefore) {
                    if (rc.comparePoint(brs[i], 0) > -1) {
                        // in this case, the br from before is the one we need
                        if (i > 0) {
                            brBefore = brs[i - 1];
                        }
                    }
                    continue;
                }
                if (rc.comparePoint(brs[i], 0) === 0) {
                    // we want to catch multi-line selections too
                    if (i > 0) {
                        brsIntersect.push(brs[i - 1]);
                    }
                } else if (brsIntersect.length > 0 && rc.comparePoint(brs[i], 0) === 1) {
                    brsIntersect.push(brs[i - 1]);
                    break;
                }
            }
            if (brBefore) {
                rc.setStartAfter(brBefore);
                rc.setEndAfter(brBefore);
            } else {
                // the range is in the very first line
                let fc = ref.firstChild;
                if (fc) {
                    rc.setStartBefore(fc);
                    rc.setEndBefore(fc);
                } else {
                    console.error('no first child exists - this should never happen');
                }
            }
            rc.insertNode(document.createTextNode(addLeft));

            for (let br of brsIntersect) {
                // intersecting br's can never result in the very first line
                rc.setStartAfter(br);
                rc.setEndAfter(br);
                rc.insertNode(document.createTextNode(addLeft));
            }
        } else {
            let textBefore = range.toString();
            // console.log('textBefore', textBefore);
            const len = textBefore.length;

            let spaceBefore = 0;
            let spaceAfter = 0;
            if (!isCaret) {
                for (let i = 0; i < len; i++) {
                    if (textBefore[i] === ' ') {
                        spaceBefore += 1;
                    } else {
                        break;
                    }
                }

                for (let i = len - 1; i >= 0; i--) {
                    if (textBefore[i] === ' ') {
                        spaceAfter += 1;
                    } else {
                        break;
                    }
                }
            }
            // console.log('spaceBefore', spaceBefore);
            // console.log('spaceAfter', spaceAfter);
            textBefore = textBefore.slice(spaceBefore, len - spaceAfter);

            const node = document.createTextNode(
                ' '.repeat(spaceBefore) + addLeft + textBefore + addRight + ' '.repeat(spaceAfter),
            );
            range.deleteContents();
            range.insertNode(node);

            if (isCaret) {
                range.setStart(node, 2);
                range.setEnd(node, 2);
            } else {
                range.setStartBefore(node);
                range.setEndAfter(node);
            }
        }

        sel.removeAllRanges();
        sel.addRange(range);

        // TODO is there a nicer way of triggering a re-render?
        content = ref.innerText;
    }
</script>

<Button
    invisible
    {onclick}
>
    <div {title}>
        {@render children()}
    </div>
</Button>

<style>
</style>
