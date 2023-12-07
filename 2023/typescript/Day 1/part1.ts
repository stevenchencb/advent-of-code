import { getFileLines } from '../utils';

const inputLines = await getFileLines(1);

export async function solve() {
	const numberRegex = /[0-9]/g;

	return inputLines.reduce((acc, curr) => {
		const numbersOfLine = curr.match(numberRegex) ?? [];
		const firstAndLastNumbers = numbersOfLine[0] + numbersOfLine[numbersOfLine.length - 1];
		const calibration = Number.parseInt(firstAndLastNumbers);
		return acc + calibration;
	}, 0);
}
