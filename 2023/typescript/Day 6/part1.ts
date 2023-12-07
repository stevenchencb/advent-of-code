import { getFileLines } from '../utils';

const lines = await getFileLines('./input.txt');
const times = [...lines[0].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));
const distances = [...lines[1].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));

const wins = times.map((_, i) => getNumberOfWins(i)).reduce((acc, curr) => (acc *= curr), 1);

console.log(wins);

function getNumberOfWins(race: number): number {
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
