import { getFileLines } from '../../utils';

type NumbersAndSymbolCoords = { numbers: number[]; potentialSymbolCoordinates: [number, number][][] };

const lines = await getFileLines('./input.txt');

// numbers and their potential symbol coordinates (in 2D matrix) per line
const numberAndPotentialSymbolCoords: NumbersAndSymbolCoords[] = [];

// coordinates of all symbols
const allSymbolCoordinates: [number, number][] = [];

let partNumbersSum = 0;

// get coordinates of all symbols, as well as all numbers and the coordinates for which a number is
// considered adjacent if a symbol is present in at least one of them ("potential symbol coordinates")
calculateNumbersAndSymbolCoords(lines);

for (const numbersAndPotentialCoords of numberAndPotentialSymbolCoords) {
	numbersAndPotentialCoords.numbers.forEach((n, i) => {
		const potentialSymbolCoordsOfNumber = numbersAndPotentialCoords.potentialSymbolCoordinates[i];
		if (intersects(allSymbolCoordinates, potentialSymbolCoordsOfNumber)) {
			partNumbersSum += n;
		}
	});
}

console.log(partNumbersSum);

function calculateNumbersAndSymbolCoords(lines: string[]) {
	for (let i = 0; i < lines.length; i++) {
		const currentLine = lines[i];

		const numbers: number[] = [];
		const potentialSymbolCoordinates: [number, number][][] = [];

		// get numbers and their potential symbol coordinates in current line
		const matches = [...currentLine.matchAll(/\d+/g)];
		matches.forEach((m) => {
			const matchedNumberString = m[0];
			const minX = m.index ?? -1;
			const maxX = minX + matchedNumberString.length - 1;
			numbers.push(Number.parseInt(matchedNumberString));
			potentialSymbolCoordinates.push(getPotentialSymbolCoordinates(minX, maxX, i));
		});
		numberAndPotentialSymbolCoords.push({ numbers, potentialSymbolCoordinates });

		// get coordinates of symbols in current line
		allSymbolCoordinates.push(...([...currentLine.matchAll(/[^0-9\.]/g)].map((m) => [m.index ?? -1, i]) as [number, number][]));
	}
}

function getPotentialSymbolCoordinates(minX: number, maxX: number, line: number): [number, number][] {
	const potentialCoordinates: [number, number][] = [];

	// potential coordinates above and below number
	for (let i = minX - 1; i <= maxX + 1; i++) {
		potentialCoordinates.push([i, line - 1]);
		potentialCoordinates.push([i, line + 1]);
	}

	// potential coordinates to the side of the number
	potentialCoordinates.push([minX - 1, line]);
	potentialCoordinates.push([maxX + 1, line]);

	return potentialCoordinates;
}

function intersects(a: [number, number][], b: [number, number][]) {
	return a.filter((x) => b.find((y) => isEqualTuple(x, y))).length > 0;
}

function isEqualTuple(a: [number, number], b: [number, number]) {
	return a[0] === b[0] && a[1] === b[1];
}
