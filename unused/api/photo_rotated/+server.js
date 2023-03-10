// @ts-nocheck
import fs from 'fs';
//import Jimp from 'jimp/es';
//const sharp = require('sharp');

/** @type {import('./$types').RequestHandler} */
export function GET({ url }) {
	const path = url.searchParams.get('path');
	const deg = url.searchParams.get('deg');
	console.log('Req. deg: ', deg);
	const imageBuffer = fs.readFileSync(path);
	return new Response(imageBuffer);
	/*
	if (deg === 90 || deg === 180 || deg === 270) {
		const temp_path = `_temp_${deg}.png`;
		fs.exists(temp_path, async function (exists) {
			if (exists) {
				fs.unlinkSync(temp_path);
			}
			const imageBuffer = sharp(path).rotate(deg).toBuffer();

			//const imageBuffer = fs.readFileSync(temp_path);
			return new Response(imageBuffer);
		});
	} else {
		const imageBuffer = fs.readFileSync(path);
		return new Response(imageBuffer);
	}*/
}
