import { invoke } from '@tauri-apps/api/tauri';
export const getPhoto = async (/** @type {string} */ path) => {
	return await invoke('get_photo', { path: path });
};
