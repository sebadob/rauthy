/** Returns an array with intersecting nodes in the given range, as well as the ancestor. */
export function elementsInRange(range: Range): Node[] {
    let commonAncestor = range.commonAncestorContainer;
    let startNode = range.startContainer;
    let endNode = range.endContainer;

    const elements: (Node | HTMLElement)[] = [];
    let beforeStart = true;
    let afterEnd = false;

    function elOrNode(nodeOrEl: Node | Element) {
        if (nodeOrEl?.nodeType === Node.ELEMENT_NODE) {
            return nodeOrEl;
        } else {
            return nodeOrEl?.parentElement;
        }
    }

    // go left
    const ancestor = elOrNode(commonAncestor);
    let start = elOrNode(startNode);
    let end = elOrNode(endNode);
    let current: Node | HTMLElement | undefined | null = start;
    // while (current !== ancestor) {
    //     if (current) {
    //         elements.push(current);
    //     }
    //     current = current?.parentElement
    // }
    do {
        if (current) {
            elements.push(current);
        }
    } while (current !== ancestor && (current = current?.parentElement));

    if (end !== ancestor && start !== ancestor && end !== start) {
        elements.pop();
    }
    // make sure elements match DOM order
    elements.reverse();

    // go right
    function walk(branch: Node) {
        const branchNodes = branch.childNodes;
        for (let i = 0; !afterEnd && i < branchNodes.length; i++) {
            let current = branchNodes[i];
            if (current === startNode) {
                beforeStart = false;
            }
            if (!beforeStart && current.nodeType === Node.ELEMENT_NODE) {
                elements.push(current);
            }
            if (current === endNode) {
                afterEnd = true;
            } else {
                walk(current);
            }
        }
    }

    walk(commonAncestor);

    return elements;
}

export function elementsInRangeNoBrHr(range: Range): Node[] {
    return elementsInRange(range).filter(node => {
        let lower = node.nodeName.toLowerCase();
        return lower !== 'br' && lower !== 'hr';
    });
}

/**
 Filters all intersecting nodes that are either fully inside the given Range or fully wrap it.
 Returns all Nodes that are partly intersecting the Range.
 The first array item will contain Nodes that start earlier and end inside the Range,
 The second item will contain Nodes that start inside the Range and end afterward.
 `=> [left, right]`
 */
export function elementsInRangeFiltered(range: Range): [Node[], Node[]] {
    let left: Node[] = [];
    let right: Node[] = [];

    let r = new Range();
    for (const node of elementsInRange(range)) {
        r.selectNode(node);
        const startsBefore = range.compareBoundaryPoints(Range.START_TO_START, r) === 1;
        const endsAfter = range.compareBoundaryPoints(Range.END_TO_END, r) === -1;

        if (startsBefore && !endsAfter) {
            left.push(node);
        } else if (!startsBefore && endsAfter) {
            right.push(node);
        }
    }

    return [left, right];
}

/**
 Correctly parses the `innerText` from a given Node, if it
 has been edited via `contenteditable`. Browsers behave differently
 onEnter.
 */
export function htmlToInnerText(html: string): string {
    let node = document.createElement('div');
    node.setHTMLUnsafe(html);

    let s = '';
    let isOnNewLine = true;

    function parse(childNodes: NodeListOf<ChildNode>) {
        for (let child of childNodes) {
            // Chrome favors br, but switches between br and div for contenteditable
            if (child.nodeName === 'BR') {
                s += '\n';
                isOnNewLine = true;
                continue;
            }

            // we may need to create a new line after a div
            if (child.nodeName === 'DIV' && isOnNewLine === false) {
                // divs create new lines for themselves if they aren't already on one
                s += '\n';
            }

            // Whether we created a new line or not, we'll use it for this content so
            // the next loop will not be on a fresh line.
            isOnNewLine = false;

            if (child.nodeType === Node.TEXT_NODE && child.textContent) {
                s += child.textContent;
            }

            parse(child.childNodes);
        }
    }

    parse(node.childNodes);

    return s;
}

export function mergeOverlappingStrings(left: string, right: string): undefined | string {
    if (left.includes(right)) {
        return left;
    } else if (right.includes(left)) {
        return right;
    }

    function tryMerge(s1: string, s2: string): undefined | string {
        if (s2.length < 1) {
            return;
        }

        let matchIdx: undefined | number = undefined;
        for (let i = 0; i < s1.length; i++) {
            if (matchIdx !== undefined) {
                let idx: number = matchIdx + 1;
                if (s2[idx] === undefined) {
                    break;
                }

                if (s1[i] === s2[idx]) {
                    matchIdx = idx;
                } else {
                    matchIdx = undefined;
                }
            } else {
                if (s1[i] === s2[0]) {
                    matchIdx = 0;
                }
            }
        }

        if (matchIdx !== undefined) {
            // let overlap = s2.slice(0, matchIdx + 1);
            // console.log('overlap', overlap);
            // console.log('sliced', s2.slice(matchIdx + 1, s2.length));
            return s1 + s2.slice(matchIdx + 1, s2.length);
        }
    }

    let s = tryMerge(left, right);
    if (s) {
        // console.log('s1', s);
        return s;
    }

    s = tryMerge(right, left);
    if (s) {
        // console.log('s2', s);
        return s;
    }
}
