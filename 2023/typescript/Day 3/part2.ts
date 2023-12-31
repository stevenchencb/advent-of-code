import { getFileLines, isEqualCoordinate } from '../utils';

type NumbersAndGearCoords = { numbers: number[]; potentialGearCoordinates: [number, number][][] };

const lines = await getFileLines(3);

export async function solve() {
	// numbers and their potential symbol coordinates (in 2D matrix) per line
	const numberAndPotentialGearCoords: NumbersAndGearCoords[] = calculateNumbersAndGearCoords(lines);

	let gearRatioSum = 0;

	for (let i = 0; i < lines.length; i++) {
		const currentLine = lines[i];

		// get coordinates of gears in line
		const gearCoordinates: [number, number][] = [...currentLine.matchAll(/\*/g)].map((m) => [m.index ?? -1, i]);

		const prevLine = i - 1 < 0 ? 0 : i - 1;
		const nextLine = i + 1 > numberAndPotentialGearCoords.length - 1 ? numberAndPotentialGearCoords.length - 1 : i + 1;

		gearCoordinates.forEach((coordinate) => {
			let adjacentTo: number[] = [];

			// check if gear coordinate occurs in lists of potential gear coordinates
			// enough to only check the potential symbol coordinates of current line and the lines above and below
			for (let j = prevLine; j <= nextLine; j++) {
				const numbersOfLine = numberAndPotentialGearCoords[j].numbers;
				const potentialGearCoordsOfLine = numberAndPotentialGearCoords[j].potentialGearCoordinates;
				numbersOfLine.forEach((n, k) => {
					const potentialCoordsOfNumber = potentialGearCoordsOfLine[k];
					if (potentialCoordsOfNumber.find((pc) => isEqualCoordinate(pc, coordinate))) {
						adjacentTo.push(n);
					}
				});
			}

			if (adjacentTo.length === 2) {
				const gearRatio = adjacentTo[0] * adjacentTo[1];
				gearRatioSum += gearRatio;
			}
		});
	}

	return gearRatioSum;
}

function calculateNumbersAndGearCoords(lines: string[]): NumbersAndGearCoords[] {
	const result: NumbersAndGearCoords[] = [];

	for (let i = 0; i < lines.length; i++) {
		const currentLine = lines[i];

		const numbers: number[] = [];
		const potentialGearCoordinates: [number, number][][] = [];

		const matches = [...currentLine.matchAll(/\d+/g)];
		matches.forEach((m) => {
			const matchedNumberString = m[0];
			const minX = m.index ?? -1;
			const maxX = minX + matchedNumberString.length - 1;
			numbers.push(Number.parseInt(matchedNumberString));
			potentialGearCoordinates.push(getPotentialGearCoordinates(minX, maxX, i));
		});
		result.push({ numbers, potentialGearCoordinates: potentialGearCoordinates });
	}

	return result;
}

function getPotentialGearCoordinates(minX: number, maxX: number, line: number): [number, number][] {
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
