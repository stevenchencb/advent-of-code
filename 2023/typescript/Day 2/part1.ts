import { extractOne, getFileLines } from '../utils';

const lines = await getFileLines('./input.txt');
const maxRed = 12;
const maxGreen = 13;
const maxBlue = 14;

let idSum = 0;

for (const line of lines) {
	const id = Number.parseInt(extractOne(line, 'Game (\\d+)', 1));
	const sets = line.split(';');

	const blueCubes: number[] = [];
	const redCubes: number[] = [];
	const greenCubes: number[] = [];

	addCubes(sets, blueCubes, redCubes, greenCubes);

	if (isPossibleGame(blueCubes, redCubes, greenCubes)) {
		idSum += id;
	}
}

console.log(idSum);

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

function isPossibleGame(blueCubes: number[], redCubes: number[], greenCubes: number[]) {
	return Math.max(...blueCubes) <= maxBlue && Math.max(...redCubes) <= maxRed && Math.max(...greenCubes) <= maxGreen;
}
