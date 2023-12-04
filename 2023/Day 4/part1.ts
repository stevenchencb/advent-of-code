import { getFileLines, intersection } from '../../utils';

const lines = await getFileLines('./input.txt');

let pointsSum = 0;

for (const line of lines) {
	const allNumbers = line.replace(/card\s+\d+: /gi, '').split('|');
	const winningNumbers = [...allNumbers[0].matchAll(/\d+/g)].map((m) => m[0]);
	const myNumbers = [...allNumbers[1].matchAll(/\d+/g)].map((m) => m[0]);

	pointsSum += calculateCardScore(winningNumbers, myNumbers);
}

console.log(pointsSum);

function calculateCardScore(winningNumbers: string[], myNumbers: string[]): number {
	const myWinningNumbers = intersection(winningNumbers, myNumbers);
	return myWinningNumbers.reduce((acc, curr) => {
		if (acc === 0) {
			acc += 1;
		} else {
			acc *= 2;
		}
		return acc;
	}, 0);
}
