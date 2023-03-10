//@ts-nocheck
import { writable } from 'svelte/store';
//import settings from './config.json';

export const selectedPhotoGrapher = writable('');
export const photo_paths = writable([]);
export const currentFolder = writable('');
export const mainDestPath = writable('D:\\BY_SCHOOL_AFU_PHOTOS\\');
export const selectedSchool = writable('');

export const saveConfig = (objectV) => {
	const localSettings = { ...settings };
	const { key, value } = objectV;
	settings[key] = value;
};
