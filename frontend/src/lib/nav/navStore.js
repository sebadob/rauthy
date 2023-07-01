import { writable } from 'svelte/store';

export const navWidthExpanded = writable();
export const navWidthCollapsed = writable();
export const navSelected = writable('');
export const navIsExpanded = writable(true);
export const navContainerExpanded = writable('');
