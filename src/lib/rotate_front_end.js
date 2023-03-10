//@ts-nocheck
const rotate = (elId, deg) => {
	const rotated = document.getElementById(elId);
	rotated.style.transform = `rotate(${deg}deg)`;
	return deg;
};
export const rotate90 = (elId) => {
	return rotate(elId, 90);
};
export const rotate180 = (elId) => {
	return rotate(elId, 180);
};
export const rotate270 = (elId) => {
	return rotate(elId, 270);
};
