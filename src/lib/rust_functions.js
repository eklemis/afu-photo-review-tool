// @ts-nocheck
import { invoke } from '@tauri-apps/api/tauri';
export const getPhoto = async (path) => {
	//console.log('get photo executed!');
	return await invoke('get_photo', { path: path });
};

export const get_higher_rotated_image = async (src_path, deg) => {
	//console.log('get higher rotated photo executed!');
	return await invoke('get_rotated_image', { srcPath: src_path, deg: deg });
};

export const rotate_and_copy = async (deg, src_path, dest_path) => {
	//console.log('rotate and copy photo executed!');
	return await invoke('rotate_and_copy', { deg: deg, srcPath: src_path, destPath: dest_path });
};

export const get_all_child_ids = async (path) => {
	return await invoke('get_jpg_chil_ids', { folderPath: path });
};

export const create_nonexist_folders = async (path) => {
	return await invoke('create_folder_paths', { folderPath: path });
};
