import { getFileLines } from '../../utils';

const inputLines = await getFileLines('./input.txt');
const numberRegex = /[0-9]/g;

const calibrationSum = inputLines.reduce((acc, curr) => {
	const numbersOfLine = curr.match(numberRegex) ?? [];
	const firstAndLastNumbers = numbersOfLine[0] + numbersOfLine[numbersOfLine.length - 1];
	const calibration = Number.parseInt(firstAndLastNumbers);
	return acc + calibration;
}, 0);

console.log(calibrationSum);
