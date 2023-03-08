import { writable } from 'svelte/store';

export const selectedPhotoGrapher = writable('');
export const photo_paths = writable([]);
export const currentFolder = writable('');
export const mainDestPath = writable(
	'D:\\OneDrive - Save the Children International\\Kantor\\2023\\AFU\\Clean Photos\\'
);
export const selectedSchool = writable('');
