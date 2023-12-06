import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');
const time = [...lines[0].replaceAll(' ', '').matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));
const distance = [...lines[1].replaceAll(' ', '').matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));

const wins = getNumberOfWins();

console.log(wins);

function getNumberOfWins(): number {
	const maxTime = time[0];
	const distanceToBeat = distance[0];
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
