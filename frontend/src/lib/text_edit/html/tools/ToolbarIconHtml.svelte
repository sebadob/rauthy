<script lang="ts">
    import type { Snippet } from 'svelte';
    import { elementsInRangeNoBrHr } from '$lib/text_edit/html/utils';
    import Button from '$lib/button/Button.svelte';

    let {
        // ref of the contenteditable div
        ref,
        // title / tooltip
        title,
        tagName,
        // optional mandatory direct wrapping element, like e.g. <ul> for <li>
        tagNameParent,
        attributes,
        // withNewline,
        onClickCustom,
        children,
    }: {
        ref: undefined | HTMLDivElement;
        title: string;
        tagName?: string;
        tagNameParent?: string;
        attributes?: Map<string, string>;
        // withNewline?: boolean,
        onClickCustom?: (range: Range) => void;
        children: Snippet;
    } = $props();

    function onclick(ev: Event) {
        if (!ref) {
            return;
        }

        let sel = document.getSelection();
        // make sure we have a valid selection inside our editor container
        if (!sel || sel.rangeCount < 1 || !ref.contains(sel.getRangeAt(0).commonAncestorContainer)) {
            return;
        }

        let range = sel.getRangeAt(0);
        if (onClickCustom) {
            onClickCustom(range);
        } else if (tagNameParent) {
            wrapRangeWithParent(range);
        } else {
            wrapRange(range);
        }

        // if (withNewline) {
        //     let r = new Range();
        //     r.setStartAfter(node);
        //     r.setEndAfter(node);
        //     r.insertNode(document.createElement('br'));
        // }
        // newLineEnding();
    }

    // // Makes sure that after we formatted something, at the end of the
    // // container will always be an empty, unformatted newline.
    // function newLineEnding() {
    //     if (!ref) {
    //         return;
    //     }
    //
    //     let r = new Range();
    //     r.selectNodeContents(ref);
    //     r.setStart(r.endContainer, r.endOffset);
    //
    //     console.log('ancestor check', r.startContainer, ref, r.startContainer === ref);
    //     // r.commonAncestorContainer
    // }

    function surroundWithTag(range: Range) {
        if (range.collapsed || !tagName) {
            return;
        }

        let node = document.createElement(tagName);
        if (attributes) {
            for (let [k, v] of attributes) {
                node.setAttribute(k, v);
            }
        }
        range.surroundContents(node);
    }

    function surroundWithParentAndTag(range: Range) {
        if (range.collapsed || !tagNameParent || !tagName) {
            return;
        }

        let parent = document.createElement(tagNameParent);
        range.surroundContents(parent);

        range.selectNodeContents(parent);
        let child = document.createElement(tagName);
        if (attributes) {
            for (let [k, v] of attributes) {
                child.setAttribute(k, v);
            }
        }
        range.surroundContents(child);

        range.selectNodeContents(child);
    }

    function wrapRange(range: Range) {
        if (!tagName) {
            console.error('no onClickCustom and no tagName given');
            return;
        }

        const isCaret = range.collapsed;
        if (isCaret) {
            range.insertNode(document.createTextNode(' '));
            surroundWithTag(range);
            return;
        }

        let intersectingNodes = elementsInRangeNoBrHr(range);
        if (intersectingNodes.length === 0) {
            console.warn('no intersecting nodes, this should never happen');
            return;
        }
        console.log('intersectingNodes', intersectingNodes);

        // handle a possibly existing everything-wrapping, non-single ancestor
        if (intersectingNodes.length > 1) {
            let nf = new Range();
            nf.selectNode(intersectingNodes[0]);
            if (range.compareBoundaryPoints(Range.START_TO_START, nf) === 1) {
                let r = new Range();
                r.setStart(range.startContainer, range.startOffset);

                let cmpEnd = range.compareBoundaryPoints(Range.END_TO_END, nf);
                if (cmpEnd === -1) {
                    // fully-wrapping outer container
                    intersectingNodes = intersectingNodes.slice(1, intersectingNodes.length);
                    console.log('remove all-wrapping ancestor from intersections', intersectingNodes);

                    let allInner = true;

                    for (const node of intersectingNodes) {
                        r.selectNode(node);
                        const startsBefore = range.compareBoundaryPoints(Range.START_TO_START, r) === 1;
                        const endsAfter = range.compareBoundaryPoints(Range.END_TO_END, r) === -1;

                        if (startsBefore || endsAfter) {
                            allInner = false;
                            break;
                        }
                    }

                    if (allInner) {
                        console.log('all nodes are inner -> full wrap');
                        surroundWithTag(range);
                        return;
                    } else {
                        // fix the first piece until the next node
                        console.log('fix the first piece until the next node after ancestor cleanup');
                        r.selectNodeContents(intersectingNodes[0]);
                        r.setStart(range.startContainer, range.startOffset);
                        surroundWithTag(r);
                    }
                } else if (cmpEnd === 1) {
                    // container ends inside range
                    // -> fix only the left side, right will be taken care of later
                    console.log('fix the first piece until the next node');
                    r.selectNodeContents(intersectingNodes[0]);
                    r.setStart(range.startContainer, range.startOffset);
                    surroundWithTag(r);
                }
            }

            // if (range.compareBoundaryPoints(Range.START_TO_START, nf) === 1
            //     && range.compareBoundaryPoints(Range.END_TO_END, nf) === -1
            // ) {
            //     intersectingNodes = intersectingNodes.slice(1, intersectingNodes.length);
            //     console.log('remove all-wrapping ancestor from intersections', intersectingNodes);
            //
            //     let allInner = true;
            //     let r = new Range();
            //     for (const node of intersectingNodes) {
            //         r.selectNode(node);
            //         const startsBefore = range.compareBoundaryPoints(Range.START_TO_START, r) === 1;
            //         const endsAfter = range.compareBoundaryPoints(Range.END_TO_END, r) === -1;
            //
            //         if (startsBefore || endsAfter) {
            //             allInner = false;
            //             break;
            //         }
            //     }
            //
            //     if (allInner) {
            //         console.log('all nodes are inner -> full wrap');
            //         surroundWithTag(range);
            //         return;
            //     } else {
            //         // fix the first piece until the next node
            //         console.log('fix the first piece until the next node after ancestor cleanup');
            //         let node = intersectingNodes[0];
            //         r.setStart(range.startContainer, range.startOffset);
            //         r.setEndBefore(node);
            //         surroundWithTag(r);
            //     }
            // }
        }
        console.log('intersectingNodes after ancestor cleanup', intersectingNodes);

        tagName = tagName.toLowerCase();

        for (let i = 0; i < intersectingNodes.length; i++) {
            const node = intersectingNodes[i];
            const children = node.childNodes;
            const isLastNode = i === intersectingNodes.length - 1;
            const nodeNameLower = node.nodeName.toLowerCase();

            let nr = new Range();
            nr.selectNode(node);

            // 3 possible cases:
            // - node starts before range
            // - range fully includes / surrounds node
            // - node goes beyond range

            const startsBefore = range.compareBoundaryPoints(Range.START_TO_START, nr) === 1;
            const endsAfter = range.compareBoundaryPoints(Range.END_TO_END, nr) === -1;

            let textOrTagOnly = false;
            if (node.nodeName.toLowerCase() === tagName) {
                textOrTagOnly = true;
                for (let child of children) {
                    // TODO find a way to check for attributes
                    if (child.nodeType !== Node.TEXT_NODE && child.nodeName.toLowerCase() !== tagName) {
                        textOrTagOnly = false;
                        break;
                    }
                }
            }
            if (textOrTagOnly) {
                console.log('is textOrTagOnly', node);
                let r = new Range();
                if (startsBefore) {
                    r.setStart(nr.startContainer, nr.startOffset);
                } else {
                    r.setStart(range.startContainer, range.startOffset);
                }
                if (endsAfter) {
                    r.setEnd(nr.endContainer, nr.endOffset);
                } else {
                    const next = intersectingNodes[i + 1];
                    if (next) {
                        r.setEndBefore(next);
                    } else {
                        r.setEnd(range.endContainer, range.endOffset);
                    }
                }

                let textNode = document.createTextNode(r.toString());
                r.deleteContents();
                r.insertNode(textNode);
                surroundWithTag(r);
            } else if (startsBefore) {
                if (!isLastNode) {
                    console.log('node', nodeNameLower, 'starts before range and is NOT the last node');
                    const next = intersectingNodes[i + 1];
                    // if (i === 0) {
                    // }

                    nr.setStartAfter(node);
                    nr.setEndBefore(next);
                    surroundWithTag(nr);
                } else {
                    if (endsAfter) {
                        // -> full wrap
                        surroundWithTag(range);
                    } else {
                        console.log('node', nodeNameLower, 'starts before range and is the last node');
                        let left = new Range();
                        left.selectNodeContents(node);
                        left.setStart(range.startContainer, range.startOffset);
                        surroundWithTag(left);

                        let right = new Range();
                        right.setStartAfter(node);
                        right.setEnd(range.endContainer, range.endOffset);
                        surroundWithTag(right);
                    }
                }
            } else if (endsAfter) {
                console.log('node', nodeNameLower, 'ends after range', node);

                let right = new Range();
                right.selectNodeContents(node);
                right.setEnd(range.endContainer, range.endOffset);
                surroundWithTag(right);
            } else if (isLastNode && range.compareBoundaryPoints(Range.END_TO_END, nr) === 1) {
                // range ends after the last node
                console.log('isLastNode and range goes beyond');

                let left = new Range();
                left.selectNodeContents(node);
                surroundWithTag(left);

                let right = new Range();
                right.setStartAfter(node);
                right.setEnd(range.endContainer, range.endOffset);
                surroundWithTag(right);
            } else {
                // range includes node
                console.log('range includes node', nodeNameLower);
                surroundWithTag(nr);
            }
        }
    }

    function wrapRangeWithParent(range: Range) {
        if (!tagNameParent || !tagName) {
            console.error('must not call wrapRangeWithParent with no tagNameParent');
            return;
        }

        const isCaret = range.collapsed;
        if (isCaret) {
            range.insertNode(document.createTextNode(' '));
            surroundWithParentAndTag(range);
            return;
        }

        // If the range has intersections on the ends with a mandatory parent,
        // we need to expand the selection until everything fits in without any overlap.
        let intersectingNodes = elementsInRangeNoBrHr(range);
        if (intersectingNodes.length === 0) {
            console.warn('no intersecting nodes, this should never happen');
            return;
        }
        console.log('intersectingNodes', intersectingNodes);

        let first = intersectingNodes[0];
        let firstNodeName = first.nodeName.toLowerCase();
        let last = intersectingNodes[intersectingNodes.length - 1];

        let parentLower = tagNameParent.toLowerCase();
        let childLower = tagName.toLowerCase();

        // TODO if the parent is the requested child, we may want to build a nested structure instead
        if (firstNodeName === parentLower || firstNodeName === childLower) {
            // Make sure the selected range fits in without overlap.
            if (firstNodeName === parentLower && intersectingNodes.length > 1) {
                let filtered = [];
                // filter out all nodes that already meet the criteria
                for (let child of first.childNodes) {
                    if (child.nodeName.toLowerCase() !== childLower) {
                        filtered.push(child);
                    }
                }
                first = filtered[0];
                last = filtered[filtered.length - 1];
            } else {
                first = intersectingNodes[1];
                last = intersectingNodes[intersectingNodes.length - 1];
            }
        }

        console.log('intersectingNodes filtered', intersectingNodes);

        // set start
        let r = new Range();
        if (first) {
            r.selectNode(first);
            const startsBefore = range.compareBoundaryPoints(Range.START_TO_START, r) === 1;
            if (startsBefore) {
                range.setStart(r.startContainer, r.startOffset);
            }
        }

        // set end
        if (last) {
            r.selectNode(last);
            const endsAfter = range.compareBoundaryPoints(Range.END_TO_END, r) === -1;
            if (endsAfter) {
                range.setEnd(r.endContainer, r.endOffset);
            }
        }

        surroundWithParentAndTag(range);
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
