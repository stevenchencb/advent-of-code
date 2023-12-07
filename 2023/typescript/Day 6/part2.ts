import { getFileLines } from '../utils';

const lines = await getFileLines(6);

export async function solve() {
	const time = [...lines[0].replaceAll(' ', '').matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]))[0];
	const distance = [...lines[1].replaceAll(' ', '').matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]))[0];

	return getNumberOfWins(time, distance);
}

function getNumberOfWins(time: number, distance: number): number {
	const minChargeTime = Math.ceil(distance / time);
	let waysToWin = 0;

	for (let i = minChargeTime; i < time; i++) {
		const timeLeft = time - i;
		if (i * timeLeft > distance) {
			waysToWin++;
		}
	}

	return waysToWin;
}
