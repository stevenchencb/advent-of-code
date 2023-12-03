import { extractWithRegex, getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

let powerSum = 0;

for (const line of lines) {
	const sets = line.split(';');

	const blueCubes: number[] = [];
	const redCubes: number[] = [];
	const greenCubes: number[] = [];

	blueCubes.push(...getCubes(sets, 'blue'));
	redCubes.push(...getCubes(sets, 'red'));
	greenCubes.push(...getCubes(sets, 'green'));

	powerSum += getPower(blueCubes, redCubes, greenCubes);
}

console.log(powerSum);

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

function getPower(blueCubes: number[], redCubes: number[], greenCubes: number[]) {
	return Math.max(...blueCubes) * Math.max(...redCubes) * Math.max(...greenCubes);
}
