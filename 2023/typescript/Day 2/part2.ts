import { extractOne, getFileLines } from '../utils';

const lines = await getFileLines(2);

export async function solve() {
	let powerSum = 0;

	for (const line of lines) {
		const sets = line.split(';');

		const blueCubes: number[] = [];
		const redCubes: number[] = [];
		const greenCubes: number[] = [];

		addCubes(sets, blueCubes, redCubes, greenCubes);

		powerSum += getPower(blueCubes, redCubes, greenCubes);
	}

	return powerSum;
}

function addCubes(sets: string[], blueCubes: number[], redCubes: number[], greenCubes: number[]) {
	const blueCubesRegexString = '((?<blueCubes>\\d+) blue)';
	const redCubesRegexString = '((?<redCubes>\\d+) red)';
	const greenCubesRegexString = '((?<greenCubes>\\d+) green)';

	for (const set of sets) {
		const extractedBlueCubes = extractOne(set, blueCubesRegexString, 'blueCubes');
		const extractedRedCubes = extractOne(set, redCubesRegexString, 'redCubes');
		const extractedGreenCubes = extractOne(set, greenCubesRegexString, 'greenCubes');

		const blueCubesInSet = Number.parseInt(extractedBlueCubes);
		const redCubesInSet = Number.parseInt(extractedRedCubes);
		const greenCubesInSet = Number.parseInt(extractedGreenCubes);

		// only push if a cube color is available in current set
		!Number.isNaN(blueCubesInSet) && blueCubes.push(blueCubesInSet);
		!Number.isNaN(redCubesInSet) && redCubes.push(redCubesInSet);
		!Number.isNaN(greenCubesInSet) && greenCubes.push(greenCubesInSet);
	}
}

function getPower(blueCubes: number[], redCubes: number[], greenCubes: number[]) {
	return Math.max(...blueCubes) * Math.max(...redCubes) * Math.max(...greenCubes);
}
