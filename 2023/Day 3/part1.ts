import { getFileLines } from '../../utils';

type NumbersAndSymbolCoords = { numbers: number[]; potentialSymbolCoordinates: [number, number][][] };

const lines = await getFileLines('./input.txt');

// numbers and their potential symbol coordinates (in 2D matrix) per line
const numberAndPotentialSymbolCoords: NumbersAndSymbolCoords[] = calculateNumbersAndPotentialSymbolCoords(lines);

let partNumbersSum = 0;

for (let i = 0; i < lines.length; i++) {
	const currentLine = lines[i];

	// get coordinates of symbols in line
	const symbolCoordinates: [number, number][] = [...currentLine.matchAll(/[^0-9\.]/g)].map((m) => [m.index ?? -1, i]);

	symbolCoordinates.forEach((coordinate) => {
		// check if symbol coordinates occur in lists of potential symbol coordinates
		// enough to only check the potential symbol coordinates of current line and the lines above and below
		for (let j = i - 1; j <= i + 1; j++) {
			const numbersOfLine = numberAndPotentialSymbolCoords[j].numbers;
			const potentialSymbolCoordsOfLine = numberAndPotentialSymbolCoords[j].potentialSymbolCoordinates;
			numbersOfLine.forEach((n, k) => {
				const potentialCoordsOfNumber = potentialSymbolCoordsOfLine[k];
				if (potentialCoordsOfNumber.find((pc) => isEqualTuple(pc, coordinate))) {
					partNumbersSum += n;
				}
			});
		}
	});
}

console.log(partNumbersSum);

function calculateNumbersAndPotentialSymbolCoords(lines: string[]): NumbersAndSymbolCoords[] {
	const result: NumbersAndSymbolCoords[] = [];

	for (let i = 0; i < lines.length; i++) {
		const currentLine = lines[i];

		const numbers: number[] = [];
		const potentialSymbolCoordinates: [number, number][][] = [];

		const matches = [...currentLine.matchAll(/\d+/g)];
		matches.forEach((m) => {
			const matchedNumberString = m[0];
			const minX = m.index ?? -1;
			const maxX = minX + matchedNumberString.length - 1;
			numbers.push(Number.parseInt(matchedNumberString));
			potentialSymbolCoordinates.push(getPotentialSymbolCoordinates(minX, maxX, i));
		});
		result.push({ numbers, potentialSymbolCoordinates });
	}

	return result;
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
