import { getFileLines } from '../utils';

const lines = await getFileLines(6);

export async function solve() {
	const times = [...lines[0].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));
	const distances = [...lines[1].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));

	const wins = times.map((_, i) => getNumberOfWins(times, distances, i)).reduce((acc, curr) => (acc *= curr), 1);

	return wins;
}

function getNumberOfWins(times: number[], distances: number[], race: number): number {
	const maxTime = times[race];
	const distanceToBeat = distances[race];
	const minChargeTime = Math.ceil(distanceToBeat / maxTime);
	let waysToWin = 0;

	for (let i = minChargeTime; i < maxTime; i++) {
		const timeLeft = maxTime - i;
		if (i * timeLeft > distanceToBeat) {
			waysToWin++;
		}
	}

	return waysToWin;
}
