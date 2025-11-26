import type { PamGroupResponse } from '$api/types/pam';

export interface PamGroupsSorted {
	host: PamGroupResponse[];
	generic: PamGroupResponse[];
	local: PamGroupResponse[];
	user: PamGroupResponse[];
}
