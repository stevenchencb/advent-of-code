import { extractWithRegex, getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');
const maxRed = 12;
const maxGreen = 13;
const maxBlue = 14;

let idSum = 0;

for (const line of lines) {
	const id = Number.parseInt(extractWithRegex(line, 'Game (\\d+)', 1));
	const sets = line.split(';');

	const blueCubes: number[] = [];
	const redCubes: number[] = [];
	const greenCubes: number[] = [];

	blueCubes.push(...getCubes(sets, 'blue'));
	redCubes.push(...getCubes(sets, 'red'));
	greenCubes.push(...getCubes(sets, 'green'));

	if (isPossibleGame(blueCubes, redCubes, greenCubes)) {
		idSum += id;
	}
}

console.log(idSum);

function getCubes(sets: string[], color: 'blue' | 'green' | 'red'): number[] {
	const regexAsString = `(\\d+) ${color}`;
	const cubes: number[] = [];

	for (const set of sets) {
		const extractedNum = extractWithRegex(set, regexAsString, 1);
		const cubesInSet = Number.parseInt(extractedNum);
		if (!Number.isNaN(cubesInSet)) {
			cubes.push(cubesInSet);
		}
	}

	return cubes;
}

function isPossibleGame(blueCubes: number[], redCubes: number[], greenCubes: number[]) {
	return Math.max(...blueCubes) <= maxBlue && Math.max(...redCubes) <= maxRed && Math.max(...greenCubes) <= maxGreen;
}
