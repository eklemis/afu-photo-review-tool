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
	let child_ids = await invoke('get_jpg_chil_ids', { folderPath: path });
	let uniq_ids = [];
	child_ids.forEach((id) => {
		if (!uniq_ids.includes(id)) {
			uniq_ids.push(id);
		}
	});
	return uniq_ids;
};

export const create_nonexist_folders = async (path) => {
	return await invoke('create_folder_paths', { folderPath: path });
};

export const import_excel = async (path) => {
	return await invoke('import_excel', { excelPath: path });
};

export const get_afu_of = async (chil_id) => {
	return await invoke('get_afu_of', { childId: Number(chil_id) });
};

export const get_pg_stat_byschool = async (pg_id, school) => {
	return await invoke('get_pg_stat_byschool', { pgId: Number(pg_id), school: school.trim() });
};

export const get_pg_stats_all = async (pg_id) => {
	return await invoke('get_pg_stats_all', { pgId: Number(pg_id) });
};

export const get_all_photographers = async () => {
	return await invoke('get_all_photographers', {});
};

export const get_all_schools = async () => {
	return await invoke('get_all_schools', {});
};

export const get_photographer_of = async (school) => {
	return await invoke('get_photographer_of', { school: school });
};
