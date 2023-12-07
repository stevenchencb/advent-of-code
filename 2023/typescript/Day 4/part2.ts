import { getFileLines, intersection } from '../utils';

const lines = await getFileLines(4);

export async function solve() {
	const scratchCards: Record<number, number> = lines.reduce<Record<number, number>>((acc, curr, i) => {
		acc[i + 1] = 1;
		return acc;
	}, {});

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];

		const allNumbers = line.replace(/card\s+\d+: /gi, '').split('|');
		const winningNumbers = [...allNumbers[0].matchAll(/\d+/g)].map((m) => m[0]);
		const myNumbers = [...allNumbers[1].matchAll(/\d+/g)].map((m) => m[0]);

		addCopies(scratchCards, winningNumbers, myNumbers, i + 1);
	}

	return Object.values(scratchCards).reduce((acc, curr) => {
		acc += curr;
		return acc;
	}, 0);
}

function addCopies(scratchCards: Record<number, number>, winningNumbers: string[], myNumbers: string[], currentCardNumber: number) {
	const myWinningNumbers = intersection(winningNumbers, myNumbers);

	let cardNumberToCopy = currentCardNumber + 1;
	myWinningNumbers.forEach((_) => {
		scratchCards[cardNumberToCopy] += scratchCards[currentCardNumber];
		cardNumberToCopy += 1;
	});
}
