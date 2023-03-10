// @ts-nocheck
import { invoke } from '@tauri-apps/api/tauri';
const rotate = (elId, deg) => {
	const rotated = document.getElementById(elId);
	rotated.style.transform = `rotate(${deg}deg)`;
	set_curr_rot_deg(deg);
};
export const rotate90 = (elId) => {
	rotate(elId, 90);
};
export const rotate180 = (elId) => {
	rotate(elId, 180);
};
export const rotate270 = async (elId, identity_image) => {
	if (elId === 'active') {
		rotate(elId, 270);
	} else {
		identity_image.source = await invoke('get_rotated_image', {
			srcPath: identity_image.path,
			deg: 270
		});
	}
	return identity_image;
};
